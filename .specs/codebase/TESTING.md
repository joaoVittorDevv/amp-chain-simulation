# Testing Infrastructure

**Analyzed:** 2026-07-06

## Test Frameworks

- **Unit/Integration:** Rust built-in `#[test]` (no external test framework)
- **Coverage:** No coverage tool configured
- **E2E:** None

## Test Organization

- **Location:** Tests co-located in source files as `#[cfg(test)] mod tests`
- **Naming:** Descriptive English function names (`passthrough_when_no_runtime`, `rejects_non_mono_stereo`)
- **Structure:** Each test function tests one behavior pattern

## Existing Tests

### Cabinet Engine Tests (`src/core/cabinet/engine.rs`)
5 tests covering:
- `passthrough_when_no_runtime` — dry signal passes unmodified when no IR loaded
- `installs_runtime_and_produces_finite_output` — IR processing produces valid audio
- `full_bypass_returns_dry` — bypass returns original signal
- `clear_removes_active_runtime` — cleanup works correctly
- `ir_switch_stays_finite` — IR switching is stable

### Cabinet Library Tests (`src/core/cabinet/library.rs`)
7 tests covering:
- `rejects_non_mono_stereo` — validation of WAV format
- `rejects_truncated_wav` — corrupt file handling
- `seed_lists_and_selects_default` — initial seeding behavior
- `integrity_and_dedup_and_rename` — content-based deduplication
- `delete_clears_selection` — removal behavior
- `size_guard_rejects_oversized` — size limits enforced
- `runtime_builds_from_stored_bytes` — IR → runtime conversion

## Test Gaps (No Coverage)

| Layer | Status | Risk |
|-------|--------|------|
| `BaseIO::process()` DSP chain | ❌ Untested | High — core audio path |
| Faust FFI parameter mapping | ❌ Untested | High — label mismatch = silent bug |
| Mojo FFI safety + DSP output | ❌ Untested | High — unsafe FFI |
| Parameter automation/smoothing | ❌ Untested | Medium |
| Plugin ↔ Standalone parity | ❌ Untested | Medium — drift risk |
| Standalone CPAL routing | ❌ Untested | Medium |
| UI rendering | ❌ Untested | Low — visual only |
| FFT Analyzer | ❌ Untested | Low — display only |

## Test Execution

**Commands:**
```bash
cargo test                          # Run all tests (cabinet only currently)
cargo test --lib                    # Library tests only
cargo test --test '*'               # Integration tests (none exist)
```

**Makefile:** No explicit `cargo test` target exists. `make run` launches standalone for manual testing.

## Gate Check Commands

| Gate Level | When to Use | Command |
|------------|-------------|---------|
| Quick | After small changes | `cargo test` |
| Build | After phase completion | `cargo build --release` |
| Full | After feature completion | `cargo build --release && cargo test` |

## Parallelism Assessment

| Test Type | Parallel-Safe? | Evidence |
|-----------|---------------|----------|
| Cabinet engine tests | Yes | Each test creates its own `CabinetEngine`, no shared global state |
| Cabinet library tests | Unknown | Tests create temp files — verify if they use unique paths |

**Note:** Rust runs tests in parallel by default. Cabinet tests appear safe (isolated state per test), but this has not been verified under `cargo test` concurrency.
