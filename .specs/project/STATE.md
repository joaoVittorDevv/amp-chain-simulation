# State — Memory & Decisions

**Created:** 2026-07-06
**Last updated:** 2026-07-06

## Active Decisions

### D-001: Cross-Vendor Review Strategy
**Date:** 2026-07-06
**Decision:** Planning phase executed with codex only. Cross-vendor review by an independent agent was **COMPLETED** on 2026-07-06 by pi.
**Rationale:** Pi was initially intermittently unavailable (DeepSeek provider 404), but later recovered. All 13 `.specs/` documents reviewed. 16 blocking issues found and resolved.
**Action:** Blockers addressed — docs ready for implementation.

### D-002: Amp Model Architecture
**Date:** 2026-07-06
**Decision:** MLC ZERO V amp will be implemented as a Faust DSP module (`.dsp` file), following the same pattern as the existing EQ. This leverages the proven Faust → C++ → bindgen → Rust pipeline.
**Rationale:**
- Faust excels at analog circuit modeling (tube stages, tone stacks, clipping)
- Project already has the complete Faust toolchain integrated
- `ExternalProcessor` trait provides the exact interface needed
- Mojo remains as the alternative Neural path
**Alternatives considered:** Mojo-only (rejected — Faust is better for analog modeling); pure Rust (rejected — no existing analog modeling libraries in stack).

### D-003: MVP Channel Scope
**Date:** 2026-07-06
**Decision:** MVP implements only **Drive II** (high-gain metal channel). Clean and Drive I are deferred to future versions.
**Rationale:**
- User's primary use case is "high gain para metal moderno"
- Drive II is the highest-gain channel, most relevant to the target audience
- Reduces initial implementation complexity
- Clean/Drive I can reuse the same Faust architecture later

### D-004: Amp Selector UI Pattern
**Date:** 2026-07-06
**Decision:** Use `EnumParam<AmpModel>` with variants `Neural` and `MlcZeroV`, following the existing `EnumParam<InputSelect>` pattern.
**Rationale:**
- Proven pattern already in codebase [src/core/state/plugin_params.rs:7]
- NIH `EnumParam` supports host automation and DAW parameter display
- ComboBox UI already implemented for InputSelect [src/lib.rs:176]

### D-005: Dead Code Removal
**Date:** 2026-07-06
**Decision:** Remove all unused code before adding MLC ZERO V feature.
**Scope:** `src/bridge/wavenet.rs` (legacy ONNX), `tract-onnx`, `nih_plug_vizia`, `biquad`, `nalgebra`, `fast-math`, `ringbuf` from Cargo.toml. Fix stale "PyTorch" UI label → "Mojo".
**Rationale:** Cleaner codebase, faster builds, no confusion between ONNX WaveNet and Mojo neural paths. User explicitly approved.

### D-006: Faust Wrapper Architecture
**Date:** 2026-07-06
**Decision:** MLC ZERO V gets its own separate Faust C ABI wrapper (`dsp/mlc_zero_v_wrapper.cpp/h`) rather than extending the existing `dsp/wrapper.cpp`.
**Rationale:** The amps have fundamentally different DSP topologies, parameter sets, and Faust class names (`mydsp` vs `mlczerov`). Isolating them prevents coupling and makes each wrapper independently maintainable. User explicitly chose this option.

### D-007: Noise Gate Position
**Date:** 2026-07-06
**Decision:** The MLC ZERO V noise gate shall be user-selectable: pre-gain (traditional input gate) or post-tone-stack (modern style). Both positions implemented; never active simultaneously.
**Rationale:** User wants flexibility. Different playing styles benefit from different gate positions. A simple enum switch provides this.

### D-008: Parameter Range Convention
**Date:** 2026-07-06
**Decision:** All MLC ZERO V parameters use real-world units (dB for gain, Hz for frequency) with `FloatRange::Skewed` and display formatters (`v2s_f32_gain_to_db`, etc.), consistent with the existing EQ parameters.
**Rationale:** Professional DAW automation display, consistent UX across all plugin panels. User explicitly chose this over normalized 0-1 ranges.

### D-009: EQ Saturation Interaction
**Date:** 2026-07-06
**Decision:** The existing EQ Faust module's `ma.tanh` remains active even when MLC ZERO V is selected. A separate `eq_tanh_bypass` BoolParam allows the user to disable the EQ's tanh saturation independently of amp selection.
**Rationale:** The EQ tanh gives the existing EQ its character; some tones benefit from the cascaded saturation, others don't. Giving the user control respects both preferences. User chose "option B with bypass."

### D-010: Cross-Vendor Review Results
**Date:** 2026-07-06
**Decision:** All 16 blocking issues from cross-review resolved via doc corrections and design decisions D-005 through D-009. 22 warnings addressed where applicable.
**Reviewer:** pi (DeepSeek) — independent vendor from codex (planner).
**Outcome:** All 13 docs approved for implementation with corrected content. No remaining blockers.

## Blockers

None currently.

## Lessons Learned

### L-001: Double-Saturation Hazard
**Date:** 2026-07-06
**Finding:** Cross-review revealed the existing EQ Faust module already applies `ma.tanh` after the tone stack. Adding an MLC amp model with its own 3-stage tanh saturation would create unintentional cascaded distortion.
**Impact:** Design must account for this interaction. Solution: `eq_tanh_bypass` BoolParam (see D-009).

### L-002: Standalone ↔ Plugin Parity Requires Mirror Changes
**Date:** 2026-07-06
**Finding:** Every parameter addition must touch 6 locations: BaseIOParams, BaseIO::process(), plugin UI, StandaloneState, standalone audio callback, standalone UI.
**Impact:** Task breakdown must explicitly include standalone parity tasks for every parameter group. Pattern documented in tasks.md.

### L-003: Faust Class Name Isolation Prevents Linker Conflicts
**Date:** 2026-07-06
**Finding:** Using distinct `-cn` flags (`mydsp` for EQ, `mlczerov` for MLC) prevents symbol collisions when compiling multiple Faust modules in the same build.rs.
**Impact:** Design must specify Faust class names explicitly; tasks must verify no duplicate symbols.

## Operational Notes

### OP-001: Pi Provider Instability (Resolved)
**Date:** 2026-07-06
**Finding:** Pi's DeepSeek provider was intermittently unavailable (HTTP 404) during planning, causing task failures. Later recovered and successfully completed cross-vendor review.
**Impact:** Cross-review executed with pi after provider recovered. Future: have fallback review path (claude_code when available).

### D-011: Category Hierarchy for Component Lab
**Date:** 2026-07-07
**Decision:** Each DSP node belongs to exactly one category (e.g., `amp-modeler`, `eq`, `cab-sim`). The pipeline enforces at most one active node per category. Categories are ordered deterministically in the signal chain.
**Rationale:** Prevents ambiguous pipeline states (e.g., two amps in series not intended). Matches the user's mental model of "one slot per effect type." Categories are extensible — new categories added by registering them in the Lab.

### D-012: Snapshot Format — AI-Readable JSON
**Date:** 2026-07-07
**Decision:** Snapshots use a verbose, self-describing JSON format with engineering prose fields (`engineering_notes`, `signal_flow_description`, per-parameter `description`). The format is designed to be consumable by an LLM for code generation in another project.
**Rationale:** The primary export use case is replicating a component in a different plugin project via LLM. The format must contain enough semantic context for an LLM to generate correct Rust/nih_plug integration code without ambiguity.

### D-013: Variant Switching via ArcSwap + Background Thread
**Date:** 2026-07-07
**Decision:** Variant switching uses `ArcSwap<Option<Arc<Variant>>>` for lock-free audio thread access and `std::thread::spawn` for background DSP loading/compilation. Audio thread outputs silence during loading.
**Rationale:** Faust compilation can take 1-2 seconds and must never block the audio thread. `ArcSwap` is proven in the existing `CabinetEngine`. Silence during loading is acceptable for the laboratory use case (not a live performance tool).

### D-014: Export as Source Bundle, Not Binary
**Date:** 2026-07-07
**Decision:** Component export produces a `.tar.gz` of source files + metadata + templates, not a pre-compiled VST3/CLAP binary.
**Rationale:** The destination project may have a different build configuration, target platform, or nih_plug version. Source bundles give maximum flexibility. The LLM integration guide provides step-by-step compilation instructions.

### D-015: SQLite as Single Database for All Lab State
**Date:** 2026-07-07
**Decision:** All lab state (categories, nodes, variants, snapshots, pipelines, verification checks) lives in a single SQLite database at `~/.config/distortion/lab.db`. Snapshots are also committable as JSON files in `snapshots/` for git sharing.
**Rationale:** Single-file simplicity, already have `rusqlite` in the stack. JSON exports provide the git-shareable artifact. WAL journal mode allows concurrent reads from UI while writes happen on background threads.

### D-016: Pre-Compiled DSP Model — No Runtime Compilation
**Date:** 2026-07-07
**Decision:** All DSP implementations (Faust, Mojo) are pre-compiled into the binary at build time via `build.rs`. A `VariantRegistry` maps variant IDs to factory functions. Snapshots store a `variant_impl_id` that selects which compiled-in DSP to use. Switching variants is selection + initialization, never compilation.
**Rationale:** Cross-review revealed runtime Faust/Mojo compilation is architecturally infeasible — `build.rs` runs at compile time, shipped VST3/CLAP has no Faust binary/C++ compiler/bindgen. Pre-compiled model matches the existing `build.rs → cc::Build → link` pipeline.

### D-017: SHA256 Exclusively for Content Hashing
**Date:** 2026-07-07
**Decision:** All snapshot and MANIFEST content hashing uses SHA256 via the `sha2` crate. The existing `blake3` usage in CabinetLibrary is NOT reused for the lab module.
**Rationale:** SHA256 is the standard for manifest integrity verification and interop. Using two different hash algorithms in the same feature creates confusion about which hash is authoritative. This is a new dependency (`sha2`) but adds only ~2 MB to the build.

### D-018: Mailbox+Trash Audio Thread Pattern (Mirrors CabinetEngine)
**Date:** 2026-07-07
**Decision:** Variant switching uses the exact same mailbox+trash hand-off pattern as `CabinetEngine`: audio thread owns the live `Arc<VariantRuntime>` uniquely, background thread posts new runtimes to a mailbox, audio thread swaps at safe points, old runtimes go to a trash slot for UI-thread deferred drop.
**Rationale:** Cross-review revealed the original `ArcSwap::load_full()` + drop-per-block design would deallocate DSP objects on the audio thread (violating CLAUDE.md no-alloc rule) and couldn't provide `&mut self` for `process_block()`. The proven CabinetEngine pattern solves both problems.

### D-019: Snapshots Store Parameter Values, Not Definitions
**Date:** 2026-07-07
**Decision:** Snapshots store `Vec<ParamValue>` (id + f32 value pairs), not NIH parameter definitions (ranges, smoothing, labels). The plugin's `BaseIOParams` layout is fixed at compile time. Loading a snapshot applies saved values into matching param IDs; extra IDs are ignored; missing params keep current values.
**Rationale:** `BaseIOParams` is a static `#[derive(Params)]` struct. DAW hosts expect a stable parameter list. Replacing ranges/defaults/smoothing at runtime is incompatible with NIH plugin parameter registration and host automation.

## Deferred Ideas

- MIDI control mapping for MLC amp parameters (matching real amp's MIDI capability)
- FX Loop simulation (modeling the real amp's series FX loop)
- Multi-mic cabinet IR blending
- Oversampling for reduced aliasing in distortion stages

## Preferences

None recorded yet.
