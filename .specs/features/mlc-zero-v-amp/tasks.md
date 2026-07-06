# MLC ZERO V Signature Amp — Tasks

**Feature ID:** `MLCZERO`
**Status:** Planning | **✅ Cross-vendor review COMPLETED (pi, 2026-07-06)**

---

## Task Breakdown

### Phase 0: Prerequisites

| ID | Task | Depends On | Files | Reuses | Gate |
|----|------|-----------|-------|--------|------|
| **T0** | Add `MlcZeroV` variant to the `ActivePanel` enum (prerequisite for UI panel routing in T14/T15) | — | `src/core/ui/main_view.rs` | Existing `ActivePanel::Neural` variant pattern | `cargo check` |

### Phase 1: Foundation — Amp Selector

| ID | Task | Depends On | Files | Reuses | Gate |
|----|------|-----------|-------|--------|------|
| **T1** | Add `AmpModel` enum + `amp_model` EnumParam to `BaseIOParams` | — | `src/core/state/plugin_params.rs` | `InputSelect` enum pattern | `cargo check` |
| **T2** | Add `mlc_zero_v: [Option<MlcZeroVProcessor>; 2]` to `BaseIO` struct (placeholder until T8), + crossfade state fields (`previous_amp_model`, `crossfade_sample`, `crossfade_buf`) | T1 | `src/lib.rs` | `faust/mojo` field patterns | `cargo check` |
| **T3** | Wire amp selector in `BaseIO::process()` — match on `amp_model`, route to Neural (unchanged) or MLC (noop placeholder) | T2 | `src/lib.rs` | Existing match on `InputSelect` | `cargo check` |
| **T4** | Add amp selector ComboBox to plugin UI | T1 | `src/lib.rs` (editor) | `InputSelect` ComboBox pattern | Manual: UI shows selector |

### Phase 2: Faust DSP Model

| ID | Task | Depends On | Files | Reuses | Gate |
|----|------|-----------|-------|--------|------|
| **T5** | Create `dsp/mlc_zero_v.dsp` — Faust model of Drive II channel with gain stages, tone stack, gate (Pre/Post selectable), power amp EQ | — | `dsp/mlc_zero_v.dsp` (CREATE) | `dsp/main.dsp` structure, `faust-ddsp/` libraries | `faust dsp/mlc_zero_v.dsp` |
| **T6** | Create `dsp/mlc_zero_v_wrapper.cpp` + `.h` — C ABI for MLC Faust module | T5 | `dsp/mlc_zero_v_wrapper.cpp` (CREATE), `dsp/mlc_zero_v_wrapper.h` (CREATE) | `dsp/wrapper.cpp/h` pattern | `cargo build` |
| **T7** | Add MLC Faust compilation + bindgen to `build.rs` — use Faust class name `-cn mlczerov` to prevent symbol collision with existing `mydsp` class | T6 | `build.rs` | Existing `faust` + `bindgen` blocks | `cargo build` succeeds |
| **T8** | Create `src/bridge/mlc_zero_v.rs` — Rust wrapper implementing `ExternalProcessor` | T7 | `src/bridge/mlc_zero_v.rs` (CREATE), `src/bridge/mod.rs` | `FaustProcessor` pattern in `faust.rs` | `cargo build` |

### Phase 3: MLC Parameters

| ID | Task | Depends On | Files | Reuses | Gate |
|----|------|-----------|-------|--------|------|
| **T9** | Add all MLC FloatParams to `BaseIOParams` (gain, bass, middle, treble, presence, depth, master, gate) | T1 | `src/core/state/plugin_params.rs` | Neural drive param patterns | `cargo check` |
| **T10** | Add all MLC switches to `BaseIOParams` (bright EnumParam, m45 BoolParam, warclaw BoolParam, feedback EnumParam, mlc_gate_pos EnumParam {Pre, Post}, eq_tanh_bypass BoolParam) | T1 | `src/core/state/plugin_params.rs` | `InputSelect` enum + `bypass` bool patterns | `cargo check` |
| **T11** | Write `set_*` methods on `MlcZeroVProcessor` for all parameters | T8 | `src/bridge/mlc_zero_v.rs` | `FaustProcessor::set_eq_params` pattern | `cargo build` |
| **T12** | Wire all MLC smoothed params into `process()` — read `.smoothed.next()`, call `mlc.set_*()`, then `mlc.process_block()` | T3, T11 | `src/lib.rs` | Existing Mojo param wiring in `process()` | `cargo build` |
| **T13** | Implement amp model crossfade on transition — use `crossfade_buf` + `crossfade_sample` fields (added in T2), 10ms linear blend (480 samples @ 48kHz), abort-and-restart on rapid toggling | T12 | `src/lib.rs` | — | Manual: no clicks on switch |

### Phase 4: UI Panel

| ID | Task | Depends On | Files | Reuses | Gate |
|----|------|-----------|-------|--------|------|
| **T14** | Create `src/core/ui/mlc_zero_v_panel.rs` — egui panel with all MLC knobs and switches | T0, T9, T10 | `src/core/ui/mlc_zero_v_panel.rs` (CREATE), `src/core/ui/mod.rs` | Existing panel patterns (`cabinet_panel.rs`, `main_view.rs` knobs) | `cargo check` |
| **T15** | Route amp panel in `main_view.rs` — show MLC panel when `ActivePanel::MlcZeroV`, hide Neural controls | T0, T14 | `src/core/ui/main_view.rs` | `ActivePanel` routing pattern | `cargo build` |
| **T16** | Add MLC node to signal chain UI | T1 | `src/core/ui/signal_chain.rs` | Existing node definitions | `cargo build` |

### Phase 5: Standalone Parity

| ID | Task | Depends On | Files | Reuses | Gate |
|----|------|-----------|-------|--------|------|
| **T17** | Add all MLC params + `amp_model` to `StandaloneState` and `AudioSnapshot` | T1, T9, T10 | `src/bin/standalone.rs` | Existing state field patterns | `cargo check` |
| **T18** | Add `MlcZeroVProcessor` instances to standalone audio callback | T8 | `src/bin/standalone.rs` | Faust/Mojo instance patterns in standalone | `cargo build` |
| **T19** | Wire MLC DSP in standalone input callback — match on `amp_model`, read snapshot params, call mlc process | T12, T18 | `src/bin/standalone.rs` | Existing DSP wiring in standalone | `cargo build` |
| **T20** | Add MLC UI controls to standalone panel | T14 | `src/bin/standalone.rs` | Existing UI wiring in standalone | `cargo build` |

### Phase 6: Build & Verify

| ID | Task | Depends On | Files | Reuses | Gate |
|----|------|-----------|-------|--------|------|
| **T21** | Add `make build-faust-mlc` target to Makefile | T7 | `Makefile` | `make build-faust` pattern | `make build-faust-mlc` works |
| **T22** | Full build gate: `cargo build --release` | T1-T20 | — | — | Build passes with 0 errors, no NEW warnings |
| **T23** | Test gate: `cargo test` (all existing cabinet tests must still pass) | T22 | — | — | 12/12 tests pass |
| **T24** | Manual smoke test: standalone `make run`, toggle amp, verify audio | T22 | — | — | Audio plays, no crashes |

---

## Dependency Graph

```
Phase 1 (Foundation)
T1 ──┬── T2 ── T3 ── T12 ── T13
     │                    │
     ├── T9 ──────────────┤
     ├── T10 ─────────────┤
     └── T16 ─────────────┘

Phase 2 (Faust DSP) — parallel with Phase 1
T5 ── T6 ── T7 ── T8 ── T11 ──┐
                              ├── T12
                              │

Phase 4 (UI) — after Phase 1 params
T9 ── T14 ── T15
T10 ──┘

Phase 5 (Standalone) — after Phase 3 DSP wired
T12 ── T17 ── T18 ── T19 ── T20

Phase 6 (Build & Verify) — after everything
T1-T20 ── T21 ── T22 ── T23 ── T24
```

## Parallel Execution Plan

| Wave | Tasks | Parallel? | Rationale |
|------|-------|-----------|-----------|
| 1 | T1, T5 | ✅ Parallel | Different files, no dependency |
| 2 | T2, T6, T9, T10 | ✅ Parallel | T1→T2, T5→T6, T1→T9/T10 |
| 3 | T3, T7, T14, T16 | ✅ Parallel | T2→T3, T6→T7, T0+T9/T10→T14 |
| 4 | T8, T11, T15 | ✅ Parallel | T7→T8, T8→T11, T0+T14→T15 |
| 5 | T12, T17 | ✅ Parallel | T3+T11→T12, T1+T9/T10→T17 |
| 6 | T13, T18 | ✅ Parallel | Independent |
| 7 | T19, T20 | ✅ Parallel | T18→T19, T17→T20 |
| 8 | T21 | Sequential | After all files in place |
| 9 | T22, T23, T24 | Sequential | Build → Test → Smoke |

**Total waves:** 9 | **Estimated tasks per wave:** 1-4 | **Parallelizable waves:** 6/9

## Verification Checklist

### Per-Task Verification

| Task | Verify |
|------|--------|
| T1 | `cargo check` passes, enum compiles |
| T2 | `cargo check` passes, struct compiles |
| T3 | `cargo check` passes, match arms complete |
| T4 | UI renders ComboBox with "Neural" and "MLC ZERO V" |
| T5 | Faust compiles independently: `faust dsp/mlc_zero_v.dsp` |
| T6 | C ABI compiles with `cc::Build` |
| T7 | `cargo build` succeeds, bindgen generates MLC bindings |
| T8 | `cargo build` succeeds, MlcZeroVProcessor created |
| T9 | `cargo check` passes, all FloatParams defined |
| T10 | `cargo check` passes, all switches defined |
| T11 | `cargo build` passes, all setters compile |
| T12 | `cargo build` passes, DSP wiring compiles |
| T13 | Audio test: switch models, verify no clicks |
| T14 | UI compiles, renders knobs and switches |
| T15 | UI shows correct panel per model selection |
| T16 | Signal chain shows MLC node |
| T17 | `cargo check` passes for standalone |
| T18 | `cargo build` passes for standalone |
| T19 | `cargo build` passes, DSP wired |
| T20 | `cargo build` passes, UI wired |
| T21 | `make build-faust-mlc` succeeds |
| T22 | `cargo build --release` 0 errors |
| T23 | `cargo test` 12/12 pass |
| T24 | `make run` launches, amp toggle works |

### Traceability Matrix

| Requirement | Tasks |
|-------------|-------|
| MLCZERO-01 (Amp Selector) | T1, T2, T3, T4, T13, T16 |
| MLCZERO-02 (Gain + Tone Stack) | T5, T9, T11, T12 |
| MLCZERO-03 (Presence + Depth + Master) | T5, T9, T11, T12 |
| MLCZERO-04 (Voicing Switches) | T5, T10, T11, T12 |
| MLCZERO-05 (Noise Gate) | T5, T9, T11, T12 |
| MLCZERO-06 (UI Panel) | T14, T15, T16 |
| MLCZERO-07 (Standalone Parity) | T17, T18, T19, T20 |

### Definition of Done

- [ ] All 24 tasks completed
- [ ] `cargo build --release` passes with 0 warnings
- [ ] `cargo test` passes (12/12 existing + any new)
- [ ] Standalone launches and produces audio
- [ ] Toggle between Neural and MLC ZERO V works with crossfade
- [ ] All MLC knobs and switches respond in real time
- [ ] Gate reduces noise without cutting sustained notes
- [ ] Cross-vendor review approved all documents before implementation
- [ ] Cross-vendor review approved implementation diff after completion
