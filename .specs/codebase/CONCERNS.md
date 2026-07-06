# Concerns

**Analyzed:** 2026-07-06

## Critical

### Core DSP Path Untested
- **Risk:** `BaseIO::process()` has zero automated test coverage
- **Impact:** Regressions in the audio pipeline may go undetected
- **Fix:** Add integration tests for the DSP chain using known input/output buffers
- **Priority:** Add before modifying the pipeline for the new amp feature

### Standalone ↔ Plugin Parity Drift
- **Risk:** Every new parameter must be added in two places:
  - `BaseIOParams` (plugin) with NIH smoothing
  - `StandaloneState` + `AudioSnapshot` (standalone) with plain fields
- **Impact:** The two targets will diverge if params are added inconsistently
- **Fix:** Document the dual-add pattern explicitly in implementation tasks
- **Priority:** Enforcement via code review checklist

## High

### FFI Fragility
- **Risk:** Raw pointer contracts (`*mut f32`, `usize` casts) have no runtime bounds checking
- **Impact:** Buffer overrun in Mojo or Faust FFI = undefined behavior (crash/silence/corruption)
- **Location:** `src/bridge/faust.rs:56`, `src/bridge/mojo.rs:55`, `dsp/wrapper.cpp:89`
- **Fix:** Add debug-mode assertions on buffer length; consider safe wrapper types

### Disk Space Pressure
- **Risk:** `target/` is ~1.9 GB, `.venv` is ~1.8 GB, build artifacts accumulate
- **Impact:** CI failures, local dev friction
- **Fix:** Follow disk management rules in CLAUDE.md; clean worktree targets aggressively

### Stale Neural Architecture References
- **Risk:** Comments and UI labels reference PyTorch/ONNX while active path is Mojo
- **Location:** `src/core/ui/main_view.rs:64` — "Neural Amp (PyTorch)"
- **Impact:** Confusion when adding new amp model options
- **Fix:** Rename UI labels to match actual implementation before adding new amp

## Medium

### Build Complexity
- **Risk:** 3-language stack (Rust + Faust + Mojo) with hard build-time requirements
- **Impact:** High setup friction; `build.rs` panics if Faust or Mojo are missing
- **Fix:** Consider feature flags to allow Rust-only builds for non-DSP work

### Hardcoded Analyzer Sample Rate
- **Risk:** Analyzer uses `48000.0` hardcoded, not host sample rate [src/core/dsp/analyzer.rs:55]
- **Impact:** Incorrect frequency display at other sample rates
- **Fix:** Accept sample rate as parameter to analyzer

### No Automated Test Commands in Makefile
- **Risk:** Developers may not know how to run tests
- **Impact:** Tests are not run during development
- **Fix:** Add `make test` target invoking `cargo test`

## Low

### Unused Dependencies
- **Risk:** `tract-onnx`, `biquad`, `nalgebra`, `fast-math`, `ringbuf`, `nih_plug_vizia` are unused; `tract-onnx` via legacy `wavenet.rs`
- **Note:** Resolved in `cleanup/dead-code` branch — all removed
- **Impact:** Larger binary, longer compile times
- **Fix:** Audit dependencies during next major refactor

### Mixed Language Comments
- **Risk:** Portuguese/English bilingual comments may confuse non-Portuguese contributors
- **Impact:** Minor readability issue
- **Fix:** Standardize on English for code comments
