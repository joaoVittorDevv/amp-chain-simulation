# Component Lab — Snapshot & Export System Specification

**Status:** Draft → Revision 2 (post cross-review)
**Created:** 2026-07-07
**Updated:** 2026-07-07 (addressed B1-B8 from codex + claude_code cross-review)
**Scope:** Complex (new domain, multi-component, database, AI-readable export)

---

## Problem Statement

O projeto atual atua como um "laboratório" de experimentação DSP onde componentes (nós como EQ, Amp, IR, Cab) são desenvolvidos, ajustados e testados. Quando um componente atinge maturidade, o usuário precisa exportá-lo com todas as suas configurações, dependências e assets para ser replicado em outro plugin com a mesma stack (Rust + Faust + Mojo + nih_plug). Hoje não existe nenhum mecanismo para salvar, versionar, restaurar ou exportar componentes — cada ajuste é efêmero e a transferência de um componente maduro para um plugin de produção é um processo manual e error-prone.

## Goals

- [ ] Salvar snapshots versionados de qualquer nó DSP individual com todas as configurações de valores
- [ ] Carregar snapshots restaurando os valores dos parâmetros para o estado salvo
- [ ] Suportar múltiplas variantes por nó (ex: MLC ZERO V e JCM900 dentro do mesmo nó Amp Modeler), todas pré-compiladas no binário
- [ ] Troca de variante ativa com estado de loading, sem travar a UI
- [ ] Formato de snapshot AI-readable para replicação por LLM em outros projetos
- [ ] Exportar componente maduro como bundle autocontido (fontes + metadados + templates)
- [ ] Checklist de verificação automática para cada snapshot (parâmetros, áudio, integridade)
- [ ] Pipeline de categorias com no máximo 1 nó ativo por categoria (enforced em runtime)
- [ ] Banco de dados local SQLite para persistência de snapshots, variantes e pipelines
- [ ] UI integrada no egui para gerenciar snapshots, variantes e exportações

## Out of Scope

| Feature | Reason |
|---|---|
| Compartilhamento remoto automático (cloud sync) | V1 usa git + arquivos locais; cloud é P3 |
| Edição de snapshots via UI (ajuste de ranges após snapshot) | Snapshots são imutáveis; ajustes geram nova versão |
| Exportação como VST3/CLAP pré-compilado | Export é bundle de fontes + metadados; compilação é no destino |
| Compilação Faust/Mojo em runtime (hot-reload DSP) | Todos os DSP são pré-compilados no binário via build.rs; snapshot seleciona qual DSP registrado usar |
| Snapshots de pipeline completo (múltiplos nós) | V1 é nó individual; pipeline completo é P2 |
| Substituição dinâmica de parâmetros NIH (Params derive) | Snapshot restaura **valores** nos parâmetros fixos; não redefine ranges/defaults em runtime |

---

## User Stories

### P1: Save Component Snapshot ⭐ MVP

**User Story**: As a DSP developer, I want to save a versioned snapshot of a component's current parameter values and DSP reference so that I can track changes and restore previous states.

**Why P1**: This is the foundation — without saving, nothing else exists.

**Acceptance Criteria**:

1. WHEN user clicks "Save Snapshot" on any component node THEN system SHALL persist all current parameter values (id + value pairs), the active DSP variant reference (which pre-compiled DSP implementation is selected), category, and metadata (version, timestamp, notes) to the SQLite database
2. WHEN snapshot is saved THEN system SHALL auto-increment the semver PATCH version (or prompt for version if first save)
3. WHEN snapshot save fails (disk full, DB locked) THEN system SHALL display a non-blocking error in the UI and rollback the partial save
4. WHEN a snapshot is created THEN system SHALL generate a verification checklist with automated checks (param count match, null-input silence, source hash integrity)

**Independent Test**: Open the plugin, adjust EQ parameters, click "Save Snapshot", close and reopen — the snapshot appears in the lab panel with correct version and can be loaded back.

**Architecture note:** Snapshots store parameter **values** (not definitions). The plugin has a fixed parameter layout per node (defined by `#[derive(Params)]` at compile time). A snapshot is a configuration of those fixed slots. All DSP implementations referenced by snapshots are pre-compiled into the binary via `build.rs`.

---

### P2: Load & Restore Snapshot

**User Story**: As a DSP developer, I want to load a previously saved snapshot so that the component's parameter values are restored exactly as they were when saved.

**Why P2**: Saving is useless without loading; this is the second half of the core loop.

**Acceptance Criteria**:

1. WHEN user selects a snapshot and clicks "Load" THEN system SHALL apply the snapshot's stored parameter values to the current node's parameter slots (matching by param ID)
2. WHEN snapshot is loaded THEN system SHALL set all parameter values to the saved values (overwriting current tweaks)
3. WHEN a snapshot references a DSP variant different from the currently active one THEN system SHALL switch to the snapshot's DSP variant (selection, not compilation — the target DSP must already be pre-compiled in the binary)
4. WHEN snapshot loading fails (variant not found in binary, corrupted snapshot data) THEN system SHALL keep the previous state active and display the error reason
5. WHEN a snapshot is loaded THEN system SHALL mark the current variant as matching that snapshot version in the UI

**Independent Test**: Save snapshot of MLC ZERO V with gain at 50%, bass at -2dB. Change gain to 80%. Load the snapshot — gain returns to 50%, bass returns to -2dB.

**Architecture note:** V1 restores **values only** into fixed parameter slots. The plugin's `BaseIOParams` struct is static; snapshots are configurations that fill those slots. If the snapshot has param IDs not present in the current node, those values are ignored. If the current node has params not in the snapshot, they keep their current values.

---

### P3: Multiple Variants Per Node

**User Story**: As a DSP developer, I want to have multiple amp models (e.g., MLC ZERO V and JCM900) within the same Amp Modeler node, and switch between them, so I can A/B test different models without leaving the plugin.

**Why P3**: Critical for the laboratory workflow — comparing models, testing interactions with EQ/cab, fine-tuning.

**Acceptance Criteria**:

1. WHEN user creates a new variant within a node THEN system SHALL create a new variant entry linked to that node, referencing a pre-compiled DSP implementation available in the binary
2. WHEN user switches from variant A to variant B THEN system SHALL: (a) keep variant A active and processing audio, (b) build variant B's DSP runtime in a background thread, (c) atomically swap via ArcSwap only after B is fully ready, (d) park variant A's runtime for deferred drop on the UI thread
3. WHEN a node is switching variants THEN the audio thread SHALL continue processing with the current variant until the swap completes (no silence gap)
4. WHEN variant switch fails (B fails to initialize) THEN system SHALL keep variant A active and display the error — no destructive unload
5. WHEN a node has multiple variants THEN the UI SHALL display all variants with the active one highlighted, and allow switching via click

**Independent Test**: Create "MLC ZERO V" and "JCM900" variants in Amp Modeler node. Switch between them — UI shows loading indicator while JCM900 initializes, audio continues through MLC until JCM900 is ready, then seamless swap.

**Architecture note:** All variants' DSP implementations are pre-compiled into the binary at build time. Switching is selection + initialization of the chosen implementation, not compilation. The swap protocol mirrors `CabinetEngine`'s mailbox+trash pattern: audio thread owns the live runtime uniquely; old runtimes are parked for UI-thread deferred drop.

---

### P4: AI-Readable Snapshot Export

**User Story**: As a DSP developer, I want to export a ready component as a self-contained bundle with AI-readable metadata so that an LLM in another project can replicate the exact component.

**Why P4**: This is the end goal of the laboratory — producing exportable, replicable components.

**Acceptance Criteria**:

1. WHEN user clicks "Export Component" on a ready snapshot THEN system SHALL generate a `.tar.gz` bundle containing: `variant.json` (full AI-readable snapshot), all DSP source files, wrapper files, `Cargo.toml.template`, `build.rs.template`, `VERIFICATION.md`, `README.md`, and `MANIFEST.json`
2. WHEN export is generated THEN `variant.json` SHALL include as top-level fields: `engineering_notes` (prose explaining design intent, gain staging, saturation character), `signal_flow_description` (textual diagram of the DSP chain), `integration_guide` (structured object with `summary`, `steps` array, and `rust_integration_example` string), and per-parameter `description` fields
3. WHEN MANIFEST.json is generated THEN it SHALL contain SHA256 hashes of every file in the bundle for integrity verification
4. WHEN an exported component is loaded in another project THEN an LLM reading `variant.json` SHALL have sufficient context to generate the Rust integration code, build.rs, and parameter definitions without ambiguity

**Independent Test**: Export MLC ZERO V snapshot. Extract the bundle. Read `variant.json` — it contains human-readable engineering notes, signal flow description, and a complete integration guide with Rust code snippets. All SHA256 hashes in MANIFEST.json match the actual files.

**Export status gating:**
- Snapshots with status `ready` can be exported directly via "Export" button
- Snapshots with status `draft` or `testing` can be exported via "Export Draft" button with a warning dialog; the bundle is marked `status: "draft"` in variant.json

---

### P5: Category Slot Enforcement

**User Story**: As a DSP developer, I want the system to enforce that each category (eq, amp-modeler, cab-sim, etc.) has exactly one node slot in the pipeline, so the signal chain remains well-defined.

**Why P5**: Without enforcement, the pipeline could have zero or two amps, breaking the DSP chain.

**Acceptance Criteria**:

1. WHEN plugin initializes THEN system SHALL create one node slot per registered category (following the defined category order)
2. WHEN a node has no loaded variant THEN the node SHALL pass audio through unchanged (unity gain bypass)
3. WHEN system validates the pipeline THEN each category SHALL have exactly one node slot
4. WHEN a new category is added to the system (via code registration) THEN a corresponding node slot SHALL be created automatically on next initialization

**Independent Test**: Start the plugin. Verify every category in the signal chain has exactly one node slot. Bypass all nodes — audio passes clean.

**Amp exclusivity:** The categories `amp-modeler` and `amp-capture` share a mutual exclusion group: at most one of the two amp categories may have an active variant at any time. The other amp category's node passes audio through if both are loaded.

---

### P6: Verification Checklist Engine

**User Story**: As a DSP developer, I want automated and manual verification checks to run against a snapshot so I know the component is faithful to what was saved.

**Why P6**: Quality gate before marking a component as "ready" for export.

**Acceptance Criteria**:

1. WHEN a snapshot is marked for verification THEN system SHALL run automated checks: parameter count matches snapshot metadata, null input produces silence (RMS < -96dBFS), sine sweep produces no NaN/Inf, all referenced DSP source files exist with matching content hashes
2. WHEN automated checks pass THEN system SHALL present manual checks: frequency response matches design, distortion character is expected, latency is acceptable
3. WHEN all checks pass THEN system SHALL allow marking the snapshot status as "ready"
4. WHEN any check fails THEN system SHALL block the "ready" status transition and display which checks failed

**Independent Test**: Run verification on an MLC ZERO V snapshot. All automated checks pass. Manual checks are displayed in the UI. Mark all as passed — snapshot status becomes "ready".

---

## Edge Cases

- WHEN a snapshot references a DSP source file that was deleted THEN loading SHALL fail with a clear error message naming the missing file
- WHEN two snapshots have the same version number THEN system SHALL reject the duplicate (enforced by UNIQUE(variant_id, version) in SQLite)
- WHEN the SQLite database is corrupted THEN system SHALL recreate it from scratch and log a warning (snapshots in git remain as backup)
- WHEN a variant switch is requested while already switching THEN system SHALL queue the second request and execute it after the first completes
- WHEN an exported bundle is larger than 100MB THEN system SHALL warn the user before generating (size estimated from source files before archive creation)
- WHEN a DSP source file has a syntax error detected at snapshot time THEN system SHALL flag the snapshot as "draft" (not "testing") until the DSP compiles at build time
- WHEN the user exports a snapshot with status "draft" THEN system SHALL show a warning dialog and mark the exported bundle as `status: "draft"`
- WHEN the plugin has two DAW instances in one project THEN each instance shares the same global lab state from `~/.config/distortion/lab.db` (documented limitation; the active variant selection is NOT per-instance in V1)

---

## Requirement Traceability

| Requirement ID | Story | Owning Task(s) | Status |
|---|---|---|---|
| LAB-01 | P1: Save Snapshot | T2, T4, T5, T11 | Pending |
| LAB-02 | P1: Verification checklist gen on save | T5, T9 | Pending |
| LAB-03 | P1: Save error handling + rollback | T5 | Pending |
| LAB-04 | P2: Load & restore values | T5, T11 | Pending |
| LAB-05 | P2: DSP variant switch on load | T5, T6, T11 | Pending |
| LAB-06 | P2: Keep previous state on failure | T5, T6, T11 | Pending |
| LAB-07 | P3: Multiple variants per node | T6, T10, T11, T14 | Pending |
| LAB-08 | P3: Variant switch with loading state | T6, T14 | Pending |
| LAB-09 | P3: Audio continues during switch | T6, T14 | Pending |
| LAB-10 | P3: No destructive unload on failure | T6 | Pending |
| LAB-11 | P4: AI-readable export | T8 | Pending |
| LAB-12 | P4: MANIFEST with SHA256 | T8 | Pending |
| LAB-13 | P4: Integration guide in export | T8 | Pending |
| LAB-14 | P5: Category slot enforcement | T7, T14 | Pending |
| LAB-15 | P6: Automated verification checks | T9 | Pending |
| LAB-16 | P6: Status gating for ready/export | T9 | Pending |

**Coverage:** 16 total, 16 mapped to tasks, 0 unmapped ✅

---

## Success Criteria

- [ ] User can save a snapshot of any DSP node in under 2 seconds
- [ ] User can load a snapshot restoring all parameter values to exact saved state
- [ ] User can switch between 2 amp model variants within 2 seconds (audio continues through active variant during switch)
- [ ] Exported `variant.json` contains sufficient context for an LLM to generate integration code
- [ ] All 16 requirements pass verification before merge
- [ ] Zero panics or allocations on audio thread during variant switching
- [ ] SQLite database operations never block the audio thread
