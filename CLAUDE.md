# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Disk Space Management (MANDATORY — polly + all sub-agents)

This project's build chain (Faust → C++, Mojo → .so, Rust → native) produces large
artifacts. Worktrees multiply this. **Every agent must treat disk space as a
first-class constraint.**

### Before any build or `cargo` invocation that fetches dependencies

```bash
# Check available space — alert if < 5 GB free
df -h / | tail -1

# Check worktree disk usage
du -sh .worktrees/*/target 2>/dev/null || true
du -sh .worktrees/* 2>/dev/null || true
```

### After any worktree is no longer needed

```bash
# Remove the worktree's target/ immediately — the rust-analyzer cache and
# incremental build artifacts are the #1 disk consumer (~5 GB each).
rm -rf .worktrees/<name>/target

# Once all work on the worktree is done (commits merged/pushed), remove it:
git worktree remove .worktrees/<name> --force
```

### Routine cleanup (run before every dispatch that may build)

```bash
# Remove target/ from all worktrees that are done (no active sub-agent)
for wt in .worktrees/*/target; do
  echo "$wt: $(du -sh "$(dirname "$wt")" 2>/dev/null | cut -f1)"
done

# Cargo registry cache — keep it (avoids re-downloads), but if truly desperate:
# rm -rf ~/.cargo/registry/cache   # DANGER: forces re-download of ALL crates

# Git garbage collection
git gc --auto
```

### Rules for sub-agents

1. **Never fetch/build without checking disk first.** Alert polly if < 5 GB free.
2. **Remove `target/` from your worktree** when your task is complete and tests
   have passed — the source code and commit history are what matter, not the
   binaries.
3. **Do NOT delete `~/.cargo/registry/`** — that cache prevents re-downloading
   crates on every build. Only `target/` directories are safe to purge.
4. **Prefer `cargo check` over `cargo build`** when you only need type-checking.
5. **Run `cargo clean` inside a worktree** before abandoning it (if
   `rm -rf target/` is not possible).

### polly orchestration duty

polly must:
- Check disk space in the same preflight that checks worker availability.
- Remove worktree `target/` directories immediately after an implementer's PR
  passes tests and the diff has been extracted for review.
- Remove the entire worktree after squash-merge to main is confirmed.
- Warn the human if free space drops below 10 GB at any point.

## Build & Dev Commands

`cargo xtask` is the canonical entry point. `make <target>` delegates to it on Unix.

```bash
cargo xtask check-env   # Validate Faust, Mojo, and directory permissions
cargo xtask pre-build   # Compile Faust .dsp + Mojo .mojo sources manually
cargo xtask build       # pre-build + cargo build --release
cargo xtask run         # pre-build + launch standalone (sets LD_LIBRARY_PATH)
cargo xtask bundle distortion --release  # VST3/CLAP distribution
cargo xtask clean       # Remove dsp/*.hpp, neural/libneural.*, target/
```

Unix shortcuts (delegate to xtask):
```bash
make run / make build / make bundle / make clean / make check-env
```

- Standalone binary: `cargo run --release --bin standalone`
- Test suite: `cargo test` runs unit tests plus `tests/lab_integration.rs`
- Debug build: `cargo build` (slow due to nih-plug git dep)

## Architecture Overview

Audio plugin (VST3/CLAP + standalone) for guitar distortion. Three-language stack:

### Processing Pipeline (6 stages, single-pass in `BaseIO::process`)
1. **Input routing** — stereo/mono/mic selection
2. **Faust 3-band parametric EQ** — `dsp/main.dsp` transpiled to C++, linked via `cc::Build`
3. **Pre-EQ convolution** — LTI filter via `FFTConvolver` + WAV impulse response
4. **Mojo neural drive** — tanh saturation via zero-copy FFI (`neural/libneural.so`)
5. **Cabinet IR convolution** — speaker cabinet simulation via `FFTConvolver` + WAV IR
6. **Master gain + NaN sanitization**

### Source Layout

| Path | Role |
|---|---|
| `src/lib.rs` | Plugin entry point (`BaseIO` struct, `Plugin` trait impl, nih_plug exports) |
| `src/bin/standalone.rs` | Standalone app (CPAL audio I/O + eframe UI, ~1428 lines) |
| `src/bridge/mod.rs` | `ExternalProcessor` trait — common interface for Faust/Mojo backends |
| `src/bridge/faust.rs` | Rust → Faust FFI adapter (bindgen-generated bindings) |
| `src/bridge/mojo.rs` | Rust → Mojo FFI adapter (zero-copy `*mut f32` → `usize`) |
| `src/core/state/plugin_params.rs` | nih_plug `#[derive(Params)]` struct + `EditorState` |
| `src/core/dsp/analyzer.rs` | FFT spectrum analyzer (2048-point, Blackman-Harris window) |
| `src/core/ui/` | egui panels: spectrum graph, signal chain, EQ/Neural controls |
| `src/lab/` | Component Lab database, snapshots, variant registry, pipeline slots, export/verification |
| `tests/lab_integration.rs` | Component Lab persistence and pipeline integration coverage |
| `dsp/main.dsp` | Faust DSP source (truth for linear processing) |
| `dsp/wrapper.cpp` / `wrapper.h` | C ABI wrapper exposing Faust as `FaustHandle` API |
| `neural/main.mojo` | Mojo neural saturation (tanh polynomial approximation) |
| `build.rs` | Orchestrates: Faust transpile → Mojo build → cc::Build → bindgen → linking |
| `faust-ddsp/` | Differentiable DSP library for Faust (on -I path) |
| `neural/drive/` | WAV impulse responses + WaveNet ONNX model for neural processing |

### Key Patterns

**Zero-Copy FFI (Rust ↔ Mojo):** Buffer pointer is cast to `usize` and passed by value. Mojo reconstructs `UnsafePointer` from the address. No copying, no allocation on audio thread.

```rust
// Rust side (bridge/mojo.rs)
let ptr = buffer as usize;
mojo_process_block(ptr, length, drive, output_gain);
```

```mojo
// Mojo side (neural/main.mojo)
var data = UnsafePointer[Float32, MutAnyOrigin](unsafe_from_address=address)
```

**Faust C Wrapper:** `wrapper.cpp` exposes Faust DSP as opaque `FaustHandle` C API. `bindgen` generates Rust bindings. `cc::Build` compiles wrapper as static library `faust_dsp`.

**Mojo Address Bypass Pattern:** `@export` functions cannot accept `UnsafePointer` params directly (parametricity restriction). Always pass address as `Int` and reconstruct internally. Never use `@extern_c` — use `@export`.

**Dual-Target Architecture:**
- **Plugin mode** (`src/lib.rs`): nih_plug `Plugin` trait, DAW hosts, nih_plug_egui UI
- **Standalone mode** (`src/bin/standalone.rs`): CPAL audio I/O, eframe/egui UI, audio routing config
- Both share `core/` modules (dsp, ui) but have independent DSP pipeline implementations

**Component Lab (`feature = "lab"`):**
- Enabled by default through Cargo features and gated at plugin/standalone integration points with `#[cfg(feature = "lab")]`.
- `Lab::init(data_dir)` opens `lab.db`, seeds default categories, registers DSP variant factories, and loads the default pipeline config.
- `VariantRegistry` maps implementation ids to infallible factories: `faust-eq`, `mojo-neural`, and `mlc-zero-v`.
- Existing bridge processors implement `DspVariant` additively; their `ExternalProcessor` behavior remains the runtime source of truth.
- `PipelineManager` owns node slots and is processed from the audio thread without locks or allocation. UI-side mailbox garbage collection uses cloned `VariantMailbox` handles.
- The lab panel is a simple egui window available in plugin and standalone via the `Lab` header button.

### Parameter Smoothing

All `FloatParam` use `SmoothingStyle::Logarithmic(50ms)` or `Linear(50ms)`. Call `.smoothed.next()` in the audio thread — never read `.value()` during `process()`.
