# Component Lab — Tasks

**Design:** `.specs/features/component-lab/design.md` (Revision 2)
**Status:** Draft → Revision 2 (post cross-review)

---

## Execution Plan

### Phase 1: Foundation (Sequential)

```
T1 → T2 → T3 → T4
```

Database schema, core data types, Lab facade, and variant registry. Must be solid before anything else.

### Phase 2: Core Logic (Mostly Sequential — DB Serialization)

```
T4 complete, then:
  T5  (Snapshot capture/serialize/load — extends database.rs)
    ↓
  T6  (Variant runtime + Node with mailbox+trash pattern)
    ↓
  T7  (Pipeline + categories + amp exclusivity)
```

T5 extends the database schema from T2. T6 depends on T5's snapshot types. T7 depends on T6's Node types. **Phase 2 is sequential** because T5/T6/T9 all touch `database.rs`.

### Phase 3: Features + Integration (Parallel OK after T7)

```
T7 complete, then:
     ┌→ T8  [P]  (Export engine + manifest)
     ├→ T9  [P]  (Verification engine)
     └→ T10 [P]  (DspVariant trait impls on existing processors)
          ↓
     T11 (Plugin integration: init lab, wire pipeline stages)
          ↓
     T12 (Lab panel UI)
          ↓
     T13 (Standalone parity)
          ↓
     T14 (Wire audio pipeline through node slots)
```

### Phase 4: Testing + Docs (Parallel OK after T14)

```
T14 complete, then:
     ┌→ T15 (Integration tests)
     └→ T16 [P] (Docs update)
```

---

## Task Breakdown

### T1: Add New Dependencies to Cargo.toml

**What:** Add `sha2`, `semver`, `chrono`, `uuid`, `flate2`, `tar` to `[dependencies]` in Cargo.toml
**Where:** `Cargo.toml`
**Depends on:** None
**Reuses:** Existing dependency formatting in Cargo.toml
**Requirement:** Foundation

**Done when:**
- [ ] 6 new crates added with correct versions
- [ ] `cargo check` passes (no compilation errors from new deps)
- [ ] `cargo build --release` completes successfully
- [ ] Disk check: `df -h /` shows > 5 GB free before build

**Tests:** none (dependency addition only)
**Gate:** build

**Disk note:** Prefer `cargo check` during development. Full `--release` build only for gate check.

---

### T2: Create Lab Module Structure + Database Schema

**What:** Create `src/lab/` directory with `mod.rs`, `database.rs` implementing full SQLite schema (categories, nodes, variants with `impl_id`/`active_snapshot_id`, snapshots with `status`/`ready_at` + `UNIQUE(variant_id, version)`, dependencies, pipelines, verification_checks tables + indexes) with WAL journal and migration support
**Where:** `src/lab/mod.rs`, `src/lab/database.rs`
**Depends on:** T1
**Reuses:** `rusqlite::Connection` with `bundled` feature; `CabinetLibrary::open_or_create()` migration pattern; `dirs::config_dir()` for DB path
**Requirement:** LAB-01, LAB-14

**Done when:**
- [ ] `Database::open(path)` creates or opens DB at `~/.config/distortion/lab.db`, runs migrations
- [ ] All 7 CREATE TABLE statements execute without errors (including `UNIQUE(variant_id, version)`, `status`/`ready_at` columns, `active_snapshot_id` FK, `impl_id`)
- [ ] 3 CREATE INDEX statements execute
- [ ] PRAGMA foreign_keys = ON and journal_mode = WAL set
- [ ] All CRUD methods for categories, nodes, variants, snapshots, dependencies, pipelines, verification_checks
- [ ] `Database::insert_snapshot()` enforces UNIQUE(variant_id, version) at DB level
- [ ] `Database::set_active_snapshot(variant_id, snapshot_id)` updates the pointer
- [ ] `cargo test` passes for database unit tests (test DB in temp dir)
- [ ] Test count: ≥6 tests pass (no silent deletions)

**Tests:** unit
**Gate:** quick

---

### T3: Create Core Data Types

**What:** Define all data model structs with serde derives: `ParamValue`, `SnapshotMeta`, `SnapshotData`, `SnapshotFull`, `ComponentMeta`, `DspConfig`, `DspEngine`, `FfiInterface`, `SourceFile`, `FaustLib`, `RoutingConfig`, `DependencyList`, `IntegrationGuide`, `SnapshotDiff`, `VariantMeta`, `PipelineConfig`, `SlotConfig`, `VerificationCheckDef`, `CheckResult`, `CheckCategory`, `CheckStatus`, `Category`. Define `DspVariant` trait (with in-place `process_block(*mut f32, length)` signature).
**Where:** `src/lab/snapshot.rs`, `src/lab/component.rs`
**Depends on:** T2
**Reuses:** `serde::Serialize`, `serde::Deserialize`; `sha2::Sha256` for hashing
**Requirement:** LAB-01, LAB-04, LAB-07, LAB-11

**Done when:**
- [ ] All structs defined with correct field types and serde derives
- [ ] `IntegrationGuide` has fields: `summary`, `host_framework`, `target_platforms`, `steps: Vec<String>`, `rust_integration_example: String`
- [ ] `SnapshotData` has top-level fields: `engineering_notes: String`, `signal_flow_description: String`, `integration_guide: IntegrationGuide`
- [ ] `DspVariant` trait defined: `process_block(&mut self, buffer: *mut f32, length: usize)`, `param_count() -> usize`, `param_ids() -> &[&str]`, `latency() -> usize` — all `Send`
- [ ] `ParamValue` round-trips through JSON without data loss
- [ ] `cargo check` passes
- [ ] Documentation comments on all public types

**Tests:** unit
**Gate:** quick

---

### T4: Create Lab Facade + VariantRegistry

**What:** Implement `Lab` struct — initialization, DB reference, pipeline ownership. Create `VariantRegistry` (static map of variant IDs to factory functions). Wire `mod.rs` with pub mod declarations and re-exports.
**Where:** `src/lab/mod.rs`, `src/lab/registry.rs`
**Depends on:** T2, T3
**Reuses:** `EditorState` pattern for shared state; `CabinetEngine::init()` initialization pattern
**Requirement:** LAB-01, LAB-04, LAB-07

**Done when:**
- [ ] `Lab::init()` opens database, registers default categories, seeds category rows, loads or creates pipeline
- [ ] `VariantRegistry` stores `HashMap<String, VariantFactory>` where `VariantFactory = fn(f32) -> Box<dyn DspVariant>`
- [ ] `Lab` public API: `save_snapshot()`, `load_snapshot()`, `switch_variant()`, `export_component()`, `pipeline()`, `categories()`, `variant_registry()`
- [ ] Default categories seeded: `input-routing`, `eq`, `pre-eq-conv`, `amp-modeler`, `amp-capture`, `cab-sim`, `ir-loader`, `output-stage` with correct `sort_order`
- [ ] `Lab.db: Mutex<Database>` — DB access via `lab.db().lock().unwrap()` only
- [ ] Module re-exports all public types
- [ ] `cargo test` passes
- [ ] Test count: ≥4 tests pass (no silent deletions)

**Tests:** unit
**Gate:** quick

---

### T5: Implement Snapshot Capture, Serialize, Load

**What:** Implement `Snapshot::capture()`, `Snapshot::to_json()`, `Snapshot::from_json()`, `Snapshot::apply_to()` (writes values into NIH FloatParam slots by param ID), `Snapshot::diff()`, snapshot insertion into database with auto-incremented semver, and snapshot load from database
**Where:** `src/lab/snapshot.rs`, extend `src/lab/database.rs`
**Depends on:** T4 (needs Lab + Database + DspVariant types)
**Reuses:** `sha2::Sha256` for content hashing; `semver::Version` for version increment; `chrono::Utc` for timestamps
**Requirement:** LAB-01, LAB-02, LAB-03, LAB-04, LAB-05, LAB-06

**Done when:**
- [ ] `Snapshot::capture(node)` collects current param values + variant_impl_id + metadata + parameter_metadata
- [ ] `Snapshot::to_json()` produces AI-readable JSON with `format`, `schema_version`, `engineering_notes`, `signal_flow_description`, `integration_guide`, `parameter_metadata` (all top-level fields)
- [ ] `Snapshot::from_json()` reconstructs full Snapshot from JSON with validation
- [ ] `Snapshot::apply_to(params: &mut BaseIOParams)` writes saved values into matching param IDs; missing IDs keep current values; extra IDs are ignored
- [ ] `Database::insert_snapshot()` persists both `values_json` (runtime restore) and `data_json` (full SnapshotData for export reconstruction); auto-incremented semver; rejects duplicate (variant_id, version)
- [ ] `Database::create_snapshot_with_checks()` — transactional: insert snapshot (with data_json) + insert verification check rows
- [ ] `Lab::load_snapshot(node_id, snapshot_id)` loads from DB, switches variant if different impl_id, applies values, marks active_snapshot_id
- [ ] `Lab::load_snapshot()` on failure keeps node in previous state (no partial apply)
- [ ] `Snapshot::diff()` reports changed values, variant changes
- [ ] Verification checklist auto-generated on snapshot creation (from DspConfig metadata)
- [ ] Export after fresh Lab/DB reload uses `data_json` from database (reconstruction test)
- [ ] `cargo test` passes
- [ ] Test count: ≥9 tests pass (no silent deletions)

**Tests:** unit
**Gate:** quick

---

### T6: Implement Variant Runtime + Node (ArcSwapOption Mailbox)

**What:** Implement `VariantMailbox` (ArcSwapOption inbox + trash, AtomicBool clear_flag — exactly mirrors `CabinetMailbox`), `VariantSlot` (audio thread owns `Option<Arc<VariantRuntime>>` uniquely, `&mut self` for process, lock-free mailbox swap, single-slot trash for UI deferred drop), `Node` (variant management, async switching via background thread posting to inbox)
**Where:** `src/lab/variant_runtime.rs`, `src/lab/node.rs`
**Depends on:** T5 (needs Snapshot, DspVariant types)
**Reuses:** `CabinetMailbox` pattern precisely (engine.rs:23–27): `ArcSwapOption` inbox + `ArcSwapOption` trash + `AtomicBool` clear_flag. `CabinetEngine::process()` pattern (engine.rs:173–220): audio thread processes through `Option<Arc<...>>` via `&mut self`, never drops DSP on audio thread.
**Requirement:** LAB-05, LAB-06, LAB-07, LAB-08, LAB-09, LAB-10

**Done when:**
- [ ] `VariantSlot::process(&mut self, buffer, length)` — `Option<Arc>` match, `Arc::get_mut` for safe mutation; returns early (passthrough) when no variant loaded; `debug_assert!` on shared Arc
- [ ] `VariantSlot::install(&self, runtime)` — posts to `mailbox.inbox.store()` (lock-free, called from background thread); drains trash first
- [ ] `VariantSlot::collect_mailbox(&mut self)` — `mailbox.inbox.swap(None)`, swaps into `current`, parks old in `mailbox.trash.store()` (lock-free, alloc-free, called from audio thread at safe point)
- [ ] `VariantSlot::collect_garbage(&self)` — `mailbox.trash.swap(None)` on UI thread (drops old DSP runtime here, NEVER audio)
- [ ] Zero `Mutex`, zero `Vec::push`, zero allocation on audio thread path (verified in tests)
- [ ] `Node::request_switch(variant_id, registry)` — looks up factory, spawns background thread, calls factory, posts to inbox via `install()`
- [ ] Background thread failure: error sent to UI, current variant stays active (no inbox post, no destructive unload)
- [ ] Second switch request while switching: queued, executed after first completes
- [ ] `cargo test` passes
- [ ] Test count: ≥7 tests pass (no silent deletions)

**Tests:** unit
**Gate:** quick

---

### T7: Implement Pipeline + Category Registry

**What:** Implement `Pipeline` (category slot management, 1-node-per-category enforcement, amp exclusivity group), `CategoryRegistry` (seed + query)
**Where:** `src/lab/pipeline.rs`, `src/lab/category.rs`
**Depends on:** T6 (needs Node type)
**Reuses:** `BTreeMap` for deterministic category ordering; `serde` for PipelineConfig serialization
**Requirement:** LAB-14

**Done when:**
- [ ] `Pipeline::register_category()` creates a new empty Node slot
- [ ] `Pipeline::get_node(category_id)` returns `Option<&Node>`
- [ ] `Pipeline::validate()`: checks exactly 1 slot per category; checks amp exclusivity (amp-modeler + amp-capture at most 1 active)
- [ ] `PipelineConfig` serializes/deserializes to/from JSON
- [ ] `Pipeline::to_config()` captures current slot state (which variant_id is active per category)
- [ ] `Pipeline::from_config()` restores pipeline state, loads active variants
- [ ] Default categories seeded with correct `sort_order`
- [ ] `cargo test` passes
- [ ] Test count: ≥5 tests pass (no silent deletions)

**Tests:** unit
**Gate:** quick

---

### T8: Implement Export Engine [P]

**What:** Implement `ExportEngine` — generates `.tar.gz` bundles from snapshots with MANIFEST.json (SHA256 per file), variant.json (AI-readable), DSP sources, templates
**Where:** `src/lab/export.rs`, `src/lab/manifest.rs`
**Depends on:** T7 (needs SnapshotFull + pipeline context)
**Reuses:** `flate2::GzEncoder`, `tar::Builder`; `sha2::Sha256` for MANIFEST hashing; `rfd::FileDialog` for save path
**Requirement:** LAB-11, LAB-12, LAB-13

**Done when:**
- [ ] `ExportEngine::estimate_size()` sums source file sizes before copy
- [ ] `ExportEngine::export()` creates temp dir, writes all files, creates `.tar.gz`
- [ ] `variant.json` written with AI-readable format (engineering_notes, signal_flow_description, integration_guide with steps + code snippets)
- [ ] `MANIFEST.json` contains SHA256 for every file in bundle
- [ ] DSP source files copied from original locations (validated against stored hashes)
- [ ] Template files generated: `Cargo.toml.template`, `build.rs.template`, `plugin.rs.template`
- [ ] `README.md` + `VERIFICATION.md` generated from snapshot data
- [ ] Export returns path to generated `.tar.gz`
- [ ] Temp dir cleaned up after archive creation
- [ ] Golden `variant.json` fixture tested against schema (all required fields present)
- [ ] `cargo test` passes
- [ ] Test count: ≥6 tests pass (no silent deletions)

**Tests:** unit
**Gate:** quick

---

### T9: Implement Verification Engine [P]

**What:** Implement `Verifier` — runs automated checks (param count match, null input silence RMS < -96dBFS at 48kHz/1024 samples, sine sweep no NaN/Inf, source file hash match) and manages manual check state
**Where:** `src/lab/verification.rs`
**Depends on:** T7 (needs SnapshotFull + DspVariant types)
**Reuses:** `Database` for persistence
**Requirement:** LAB-02, LAB-15, LAB-16

**Done when:**
- [ ] `Verifier::run_automated(snapshot, variant)` returns `Vec<CheckResult>` for all 4 automated check types
- [ ] Null-input test: feed 1024 zero samples, measure RMS, assert < -96dBFS
- [ ] Sine sweep test: 20Hz-20kHz sweep at -6dBFS, assert all output samples finite
- [ ] Param count test: `variant.param_count() == snapshot.parameter_values.len()`
- [ ] Source hash test: SHA256 of all referenced source files matches stored hashes
- [ ] `Verifier::checklist(snapshot_id)` loads checks from DB
- [ ] `Verifier::mark_manual(check_id, passed, note)` updates status + checked_at
- [ ] `Verifier::all_passed(snapshot_id)` returns true only when ALL checks (auto + manual) pass
- [ ] Status gating: snapshot.status can only be set to "ready" when `all_passed() == true`; `ready_at` timestamp set on transition
- [ ] `cargo test` passes
- [ ] Test count: ≥5 tests pass (no silent deletions)

**Tests:** unit
**Gate:** quick

---

### T10: Implement DspVariant Trait on Existing Processors [P]

**What:** Implement `DspVariant` trait on `FaustProcessor` (EQ), `MojoProcessor` (Neural drive), and `MlcZeroVProcessor` (amp model). Register factory functions in VariantRegistry.
**Where:** `src/bridge/faust.rs`, `src/bridge/mojo.rs`, `src/bridge/mlc_zero_v.rs`
**Depends on:** T7 (needs DspVariant trait + pipeline context)
**Reuses:** Existing `FaustProcessor`, `MojoProcessor`, `MlcZeroVProcessor` structs; in-place zero-copy FFI pattern
**Requirement:** LAB-01, LAB-04, LAB-07

**Done when:**
- [ ] `FaustProcessor` implements `DspVariant` with `process_block(*mut f32, len)`, `param_count() = 6`, `param_ids()` returning EQ param IDs
- [ ] `MojoProcessor` implements `DspVariant` with `process_block(*mut f32, len)`, `param_count() = 2`
- [ ] `MlcZeroVProcessor` implements `DspVariant` (stub for now; real implementation in v0.2)
- [ ] Factory functions registered: `faust_eq_factory`, `mojo_neural_factory`, `mlc_zero_v_factory`
- [ ] `VariantRegistry` populated with all factory entries at Lab::init() time
- [ ] `cargo test` passes
- [ ] Test count: ≥3 tests pass (no silent deletions)

**Tests:** unit
**Gate:** quick

---

### T11: Integrate Lab into Plugin (BaseIO)

**What:** Wire `Lab` initialization into `BaseIO::initialize()`, add `Arc<Lab>` to `BaseIO` struct, pass lab reference to UI. Existing pipeline continues working unchanged; lab is add-only.
**Where:** `src/lib.rs`
**Depends on:** T4, T6, T7, T10
**Reuses:** `Arc` pattern from `EditorState`; `BaseIO::initialize()` lifecycle
**Requirement:** LAB-01, LAB-04, LAB-05, LAB-06, LAB-07, LAB-14

**Done when:**
- [ ] `Lab::init()` called in `BaseIO::initialize()`, stored as `Arc<Lab>` in `BaseIO`
- [ ] `Lab` instance shared with `EditorState` via `Arc::clone()`
- [ ] `lib.rs` declares `pub mod lab;`
- [ ] **Existing DSP pipeline is NOT removed, modified, or replaced.** Lab is add-only.
- [ ] `cargo check` passes for plugin target
- [ ] `cargo build --release` passes for plugin target
- [ ] Disk check: `df -h /` shows > 5 GB free before release build
- [ ] Test count: existing tests still pass (no silent deletions)

**Tests:** unit
**Gate:** build

---

### T12: Create Lab Panel UI (egui)

**What:** Create `lab_panel.rs` with variant switcher dropdown, snapshot version list, save/load/export buttons, verification checklist display, loading progress indicator. Create `LabUiState` struct for explicit async state management.
**Where:** `src/core/ui/lab_panel.rs`
**Depends on:** T11
**Reuses:** `egui` layout patterns from `cabinet_panel.rs`; `rfd::FileDialog` for export save path
**Requirement:** LAB-01, LAB-02, LAB-04, LAB-07, LAB-08, LAB-11, LAB-15

**Done when:**
- [ ] Lab panel accessible from signal chain UI (click on node opens lab panel)
- [ ] `LabUiState` struct: `export_progress: Option<ExportProgress>`, `last_error: Option<String>`, `switch_state: Option<SwitchProgress>`
- [ ] Variant switcher: dropdown listing all variants, active highlighted, "Switch" button with loading spinner during switch
- [ ] Snapshot list: version history with dates, notes, status badges (draft/testing/ready/deprecated)
- [ ] Save Snapshot button: opens dialog for notes, saves and refreshes list
- [ ] Load Snapshot button: applies saved values to node, refreshes UI
- [ ] "Export" button for ready snapshots; "Export Draft" button (with warning dialog) for non-ready
- [ ] Verification panel: checklist with pass/fail/pending indicators
- [ ] Error toasts via `LabUiState.last_error` (cleared on user dismiss)
- [ ] `cargo check` passes

**Tests:** unit (for LabUiState transitions only)
**Gate:** build

---

### T13: Standalone Parity for Lab

**What:** Mirror lab initialization and UI integration in standalone app
**Where:** `src/bin/standalone.rs`
**Depends on:** T11, T12
**Reuses:** `StandaloneState` struct; `eframe::App::update()`; existing standalone audio thread pattern
**Requirement:** LAB-01, LAB-04, LAB-07

**Done when:**
- [ ] `Lab::init()` called in `StandaloneApp::new()` or equivalent entry point
- [ ] `Arc<Lab>` stored in `StandaloneState`
- [ ] Lab panel rendered in standalone egui UI (same `lab_panel` function)
- [ ] Standalone DSP pipeline references lab nodes for audio processing
- [ ] Saving/loading snapshots works identically to plugin mode
- [ ] Variant switching works in standalone mode
- [ ] `cargo build --release` passes for standalone binary
- [ ] Test count: existing tests still pass (no silent deletions)

**Tests:** unit
**Gate:** full

---

### T14: Wire Audio Pipeline through Node Slots

**What:** Modify `BaseIO::process()` and standalone audio thread to query lab nodes for active variant at each pipeline stage. Existing hardcoded pipeline stays as fallback. When a lab node has an active variant, use it; otherwise, use existing hardcoded DSP.
**Where:** `src/lib.rs`, `src/bin/standalone.rs`
**Depends on:** T11, T13, T10
**Reuses:** Existing 7-stage pipeline structure; `AudioSnapshot` for parameter transport in standalone
**Requirement:** LAB-07, LAB-09, LAB-14

**Done when:**
- [ ] Each pipeline stage: `if let Some(variant) = lab.get_active_variant("eq") { variant.process_block(...) } else { /* existing EQ code */ }`
- [ ] Audio thread calls `node.variant_slot_mut().process(buf, len)` — single-writer `&mut self`, lock-free, alloc-free
- [ ] `collect_mailbox()` called at safe point in audio thread (beginning of each block)
- [ ] `collect_garbage()` called periodically from UI thread
- [ ] When node has no variant, existing hardcoded DSP runs (backward compatible)
- [ ] Category ordering in pipeline matches existing DSP chain order
- [ ] No regressions: existing EQ, Neural, Cabinet processing works as before
- [ ] `cargo build --release` passes for both plugin and standalone
- [ ] Manual smoke test: plugin loads, audio passes through all stages
- [ ] Test count: existing tests still pass (no silent deletions)

**Tests:** integration
**Gate:** full

---

### T15: End-to-End Integration Tests

**What:** Write integration tests for snapshot save/load cycle, variant switching, export roundtrip, verification checks
**Where:** `tests/lab_integration.rs`
**Depends on:** T14
**Reuses:** Existing test patterns from `src/core/cabinet/`
**Requirement:** LAB-01 through LAB-16

**Done when:**
- [ ] Test: save snapshot, restart (simulated via fresh Lab), load snapshot, verify param values match
- [ ] Test: create 2 variants, switch between them, verify active variant changes, audio continues through old variant during switch
- [ ] Test: export component, extract .tar.gz, read MANIFEST.json, verify all SHA256 hashes match
- [ ] Test: save snapshot, reload Lab from scratch (simulated DB close/reopen), export must produce identical variant.json (data_json reconstruction)
- [ ] Test: snapshot with mismatched param count fails verification
- [ ] Test: double-switch queues correctly (second request not dropped)
- [ ] Test: save/load pipeline config preserves active variant per slot
- [ ] Test: insert duplicate (variant_id, version) fails at DB level (UNIQUE constraint)
- [ ] Test: switch to non-existent variant ID fails gracefully, current stays active
- [ ] Test: load snapshot with extra param IDs — only matching IDs applied
- [ ] `cargo test` passes all tests
- [ ] Test count: ≥10 integration tests pass (no silent deletions)

**Tests:** integration
**Gate:** full

---

### T16: Update Project Configuration Files [P]

**What:** Update `.specs/project/PROJECT.md`, `ROADMAP.md`, `STATE.md`; update `.specs/codebase/ARCHITECTURE.md` to reflect lab module; finalize traceability
**Where:** `.specs/project/*.md`, `.specs/codebase/ARCHITECTURE.md`
**Depends on:** T14
**Reuses:** Existing document structure
**Requirement:** Documentation completeness

**Done when:**
- [ ] PROJECT.md: Component Lab in Goals, new crates in Tech Stack
- [ ] ROADMAP.md: v0.3 section with LAB-xx features
- [ ] STATE.md: Decisions D-011 (Category hierarchy), D-012 (Snapshot format), D-013 (Mailbox+trash switching), D-014 (Export as source bundle), D-015 (SQLite single DB), D-016 (Pre-compiled DSP model), D-017 (Sha256 exclusively)
- [ ] ARCHITECTURE.md: `src/lab/` module + mailbox+trash pattern + VariantRegistry + DspVariant trait documented
- [ ] All documents internally consistent; no stale claims about runtime compilation

**Tests:** none (docs only)
**Gate:** build

---

## Parallel Execution Map

```
Phase 1 (Sequential):
  T1 ──→ T2 ──→ T3 ──→ T4

Phase 2 (Sequential — database.rs serialization):
  T4 ──→ T5 ──→ T6 ──→ T7

Phase 3 (Parallel where safe):
  T7 complete, then:
    ├── T8  [P]  (Export — independent files)
    ├── T9  [P]  (Verification — independent file)
    └── T10 [P]  (DspVariant impls — existing bridge files)
  T8/T9/T10 complete, then:
    T11 ──→ T12 ──→ T13 ──→ T14

Phase 4 (Parallel):
  T14 complete, then:
    ├── T15 (Integration tests)
    └── T16 [P] (Docs update)
```

**Why Phase 2 is sequential:** T5 extends `database.rs` (T2). T6 needs T5's snapshot types. T7 needs T6's Node types. All three touch `database.rs` extensions. Parallel worktrees on the same DB module would conflict.

**Phase 3 parallelism is safe:** T8 (export.rs, manifest.rs), T9 (verification.rs), and T10 (bridge/*.rs) modify completely independent files with no shared mutable state.

---

## Task Granularity Check

| Task | Scope | Status |
|---|---|---|
| T1: Add dependencies | 1 file | ✅ Granular |
| T2: Database schema | 1 module | ✅ Granular |
| T3: Core data types | 2 files | ✅ Granular |
| T4: Lab facade + registry | 2 files | ✅ Granular |
| T5: Snapshot capture/load | 1 module + DB extension | ✅ Granular |
| T6: Variant runtime + node | 2 files | ✅ Granular |
| T7: Pipeline + categories | 2 files | ✅ Granular |
| T8: Export engine | 2 files | ✅ Granular |
| T9: Verification engine | 1 file | ✅ Granular |
| T10: DspVariant impls | 3 existing files | ✅ Granular |
| T11: Plugin integration | 1 file | ✅ Granular |
| T12: Lab panel UI | 1 file | ✅ Granular |
| T13: Standalone parity | 1 file | ✅ Granular |
| T14: Audio pipeline wiring | 2 files | ✅ Granular |
| T15: Integration tests | 1 file + test modules | ✅ Granular |
| T16: Docs update | 4 files | ✅ Granular |

---

## Test Co-location Validation

| Task | Code Layer | Matrix Requires | Task Says | Status |
|---|---|---|---|---|
| T1 | Cargo.toml | none | none | ✅ OK |
| T2 | Database | unit | unit | ✅ OK |
| T3 | Data models | unit | unit | ✅ OK |
| T4 | Lab facade | unit | unit | ✅ OK |
| T5 | Snapshot logic | unit | unit | ✅ OK |
| T6 | Variant runtime | unit | unit | ✅ OK |
| T7 | Pipeline logic | unit | unit | ✅ OK |
| T8 | Export engine | unit | unit | ✅ OK |
| T9 | Verification | unit | unit | ✅ OK |
| T10 | DSP bridge | unit | unit | ✅ OK |
| T11 | Plugin core | unit | unit | ✅ OK |
| T12 | Lab UI state | unit | unit | ✅ OK |
| T13 | Standalone | unit | unit | ✅ OK |
| T14 | Audio pipeline | integration | integration | ✅ OK |
| T15 | Integration tests | integration | integration | ✅ OK |
| T16 | Docs | none | none | ✅ OK |

---

## Diagram-Definition Cross-Check

| Task | Depends On (body) | Diagram Shows | Status |
|---|---|---|---|
| T1 | None | Phase 1 start | ✅ |
| T2 | T1 | T1 → T2 | ✅ |
| T3 | T2 | T2 → T3 | ✅ |
| T4 | T2, T3 | T3 → T4 | ✅ |
| T5 | T4 | T4 → T5 | ✅ |
| T6 | T5 | T5 → T6 | ✅ |
| T7 | T6 | T6 → T7 | ✅ |
| T8 | T7 | T7 → T8 [P] | ✅ |
| T9 | T7 | T7 → T9 [P] | ✅ |
| T10 | T7 | T7 → T10 [P] | ✅ |
| T11 | T4, T6, T7, T10 | T8/T9/T10 → T11 | ✅ |
| T12 | T11 | T11 → T12 | ✅ |
| T13 | T11, T12 | T12 → T13 | ✅ |
| T14 | T11, T13, T10 | T13 → T14 | ✅ |
| T15 | T14 | T14 → T15 | ✅ |
| T16 | T14 | T14 → T16 [P] | ✅ |

All cross-checks pass. ✅

---

## Requirement→Task Traceability

| Req | Task(s) |
|---|---|
| LAB-01 | T2, T4, T5, T11 |
| LAB-02 | T5, T9 |
| LAB-03 | T5 |
| LAB-04 | T5, T11 |
| LAB-05 | T5, T6, T11 |
| LAB-06 | T5, T6, T11 |
| LAB-07 | T6, T10, T11, T14 |
| LAB-08 | T6, T14 |
| LAB-09 | T6, T14 |
| LAB-10 | T6 |
| LAB-11 | T8 |
| LAB-12 | T8 |
| LAB-13 | T8 |
| LAB-14 | T7, T14 |
| LAB-15 | T9 |
| LAB-16 | T9 |

All 16 requirements mapped. ✅
