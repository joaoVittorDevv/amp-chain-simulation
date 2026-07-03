# PROJECT_MAP.md — Distortion Guitar Amp Plugin

> **Zero-context initiation document.**
> Read this file first if you are new to the project. It maps the entire codebase:
> what it is, how the three languages interoperate, every source file's role, the
> real-time audio pipeline, the build orchestration, the key design patterns, the
> dependencies, and the day-to-day commands.

---

## Table of Contents

1. [Project Overview](#1-project-overview)
2. [Architecture](#2-architecture)
3. [Source Layout](#3-source-layout)
4. [Processing Pipeline](#4-processing-pipeline)
5. [Build System](#5-build-system)
6. [Key Design Patterns](#6-key-design-patterns)
7. [Dependencies](#7-dependencies)
8. [Development Commands](#8-development-commands)
9. [Appendix — Known Rough Edges](#9-appendix--known-rough-edges)

---

## 1. Project Overview

**Distortion** is a real-time **guitar distortion / amp-simulation audio plugin**. It
builds both as a native plugin (**VST3** and **CLAP** for DAW hosts) and as a
**standalone desktop application** with its own audio I/O and GUI.

The plugin models a full guitar amplifier signal chain:

- A **3-band parametric equalizer** (tone shaping).
- A **pre-EQ impulse-response convolution** (tone-stack / input coloration).
- A **neural / non-linear drive stage** (tanh saturation — the "distortion").
- A **cabinet impulse-response convolution** (speaker cabinet simulation).
- **Master gain** and output conditioning.

It is a deliberately polyglot project, using each language for what it is best at:

| Language | Role | Why |
|---|---|---|
| **Rust** | Host / orchestrator, UI, audio routing, plugin lifecycle | Memory-safe systems language; `nih-plug` gives VST3/CLAP/standalone from one codebase |
| **Faust** | Linear DSP (parametric EQ) | Domain-specific DSP language; compiles to tight, correct C++ filter code |
| **Mojo** | Non-linear neural drive (saturation) | High-performance, Python-like; used here for zero-copy in-place saturation over the audio buffer |
| **C/C++** | Thin FFI wrapper around Faust-generated code | Exposes the Faust class as a stable C ABI for Rust `bindgen` |

### Key technologies

- **[nih-plug](https://github.com/robbert-vdh/nih-plug)** — Rust plugin framework (VST3, CLAP, standalone). Provides the `Plugin` trait, parameter system, and `nih_plug_egui` editor integration.
- **[Faust](https://faust.grame.fr/)** — Functional DSP language, transpiled to C++ at build time.
- **[Mojo](https://www.modular.com/mojo)** (Modular MAX SDK) — compiled to a shared library `libneural.so` and linked into the Rust binary.
- **[egui](https://github.com/emilk/egui)** / **eframe** — immediate-mode GUI (spectrum analyzer, knobs, signal-chain view).
- **[CPAL](https://github.com/RustAudio/cpal)** — cross-platform audio I/O for the standalone build.
- **[fft-convolver](https://crates.io/crates/fft-convolver)** — partitioned FFT convolution for the impulse responses.
- **[realfft](https://crates.io/crates/realfft)** — the 2048-point spectrum analyzer FFT.

### At a glance

- **Package name:** `distortion` (`Cargo.toml`), display name **"Distortion"** (`bundler.toml`).
- **Crate types:** `cdylib` (the plugin) + `lib` (shared by the `standalone` binary).
- **License:** GPL-3.0-or-later.
- **Editor size:** 800×450 (plugin), 1000×500 window (standalone).
- **No automated test suite currently exists** (no `#[test]` functions).

---

## 2. Architecture

### 2.1 The three-language stack

```
┌──────────────────────────────────────────────────────────────┐
│                    HOST  (DAW  or  Standalone)                │
│      supplies an audio buffer  &mut Buffer<f32>  per block    │
└───────────────────────────────┬──────────────────────────────┘
                                │  nih-plug (Rust)
              ┌─────────────────▼───────────────────┐
              │   Rust Orchestrator                  │
              │   src/lib.rs  (plugin)               │
              │   src/bin/standalone.rs (standalone) │
              │                                      │
              │   Trait: ExternalProcessor           │
              │   (src/bridge/mod.rs)                │
              └───────┬───────────────────┬──────────┘
                      │ FFI (C ABI)        │ FFI (C ABI, zero-copy)
        ┌─────────────▼──────────┐  ┌──────▼──────────────────────┐
        │  Faust  (C++)          │  │  Mojo  (MAX SDK)            │
        │  dsp/main.dsp          │  │  neural/main.mojo           │
        │  → dsp/FaustModule.hpp │  │  → neural/libneural.so      │
        │  → dsp/wrapper.cpp     │  │                             │
        │  → libfaust_dsp.a      │  │  tanh saturation, in-place  │
        │  3-band parametric EQ  │  │  (the "drive" / distortion) │
        └────────────────────────┘  └─────────────────────────────┘

        Plus two Rust-side FFTConvolver stages (pre-EQ IR + cabinet IR)
        and WAV impulse responses in neural/drive/.
```

### 2.2 How the languages interoperate (FFI)

There are two distinct FFI bridges, both crossing the boundary **in-place, without
copying audio data**:

1. **Rust ↔ Faust (via C++):**
   - `dsp/main.dsp` is transpiled by the Faust compiler into `dsp/FaustModule.hpp` (a C++ class `mydsp`).
   - `dsp/wrapper.cpp` / `dsp/wrapper.h` wrap that class in a plain **C ABI** (`faust_create`, `faust_init`, `faust_process`, `faust_set_eq_*`, `faust_destroy`) using an opaque `FaustHandle` pointer.
   - `build.rs` compiles the wrapper into a static library `libfaust_dsp.a` with `cc::Build`, and runs `bindgen` on `wrapper.h` to auto-generate Rust bindings (`bindings_faust.rs`).
   - `src/bridge/faust.rs` includes those bindings and exposes a safe-ish `FaustProcessor`.

2. **Rust ↔ Mojo (zero-copy address bypass):**
   - `neural/main.mojo` exports `mojo_init` and `mojo_process_block` as C symbols and is compiled to `neural/libneural.so`.
   - Rust declares them in an `extern "C"` block (`src/bridge/mojo.rs`).
   - The audio buffer pointer is cast to a `usize` **address** and passed by value. Mojo reconstructs an `UnsafePointer` from the address and mutates the samples in place.

Both backends implement (or are wrapped by an adapter that implements) the shared
`ExternalProcessor` trait, giving the orchestrator one uniform interface.

### 2.3 Dual-target architecture

The same DSP building blocks run in two very different hosts:

| | **Plugin mode** (`src/lib.rs`) | **Standalone mode** (`src/bin/standalone.rs`) |
|---|---|---|
| Entry | `BaseIO` struct implementing nih-plug `Plugin` | `StandaloneApp` implementing `eframe::App` |
| Audio | Host DAW calls `process()` | CPAL streams on a dedicated `audio_worker` thread |
| Params | nih-plug `#[derive(Params)]` (`BaseIOParams`) — automatable, persisted | Plain `StandaloneState` struct behind an `Arc<Mutex<…>>` |
| UI | `nih_plug_egui::create_egui_editor` | `eframe::run_native` |
| Routing | Fixed 2-in/2-out layout | User-selectable host/device/channel/buffer routing |
| Shared code | `core::dsp`, `core::ui`, `bridge::*` | same |

**Important nuance:** the two targets **do not share a single pipeline
implementation** — each has its own `process` loop. `core/` provides the shared
DSP (analyzer) and UI widgets, while `bridge/` provides the shared FFI adapters.
Keep them in sync manually when changing DSP behavior.

---

## 3. Source Layout

Full annotated tree (build artifacts and `target/` omitted):

```
meu-novo-plugin/
├── Cargo.toml                     # Crate manifest: deps, [lib] cdylib+lib, [[bin]] standalone, workspace
├── Cargo.lock                     # Locked dependency graph
├── build.rs                       # Build orchestrator (Faust transpile → Mojo build → cc → bindgen → link)
├── Makefile                       # Developer command interface (run/build/bundle/clean/pre-build/check-env)
├── bundler.toml                   # nih-plug xtask bundler metadata (human-readable plugin name)
├── pyproject.toml                 # Minimal Python project stub (env scaffolding for Mojo/tooling)
├── README.md                      # Portuguese "zero-context" dev manual (stack, build flows, FFI, troubleshooting)
├── CLAUDE.md                      # Guidance for Claude Code working in this repo
│
├── src/
│   ├── lib.rs                     # PLUGIN entry point: BaseIO struct, Plugin/ClapPlugin/Vst3Plugin impls,
│   │                              #   editor(), initialize() (loads IRs), process() (6-stage pipeline),
│   │                              #   nih_export_clap!/nih_export_vst3! macros
│   │
│   ├── bin/
│   │   └── standalone.rs          # STANDALONE app (~1050 lines): CPAL host/device/channel routing,
│   │                              #   audio_worker thread, per-block DSP, eframe UI, buffer-size UX,
│   │                              #   error popups, Linux device-name beautifier
│   │
│   ├── bridge/                    # FFI adapters — the Rust ↔ external-language boundary
│   │   ├── mod.rs                 # `ExternalProcessor` trait (init + process_block(*mut f32, len))
│   │   ├── faust.rs               # `FaustProcessor`: includes bindgen bindings, wraps FaustHandle,
│   │   │                          #   set_eq_params(), Drop→faust_destroy, unsafe Send
│   │   ├── mojo.rs                # `MojoProcessor`: extern "C" decls, drive/output_gain fields,
│   │   │                          #   zero-copy pointer→usize call to mojo_process_block
│   │   └── wavenet.rs             # (present in working tree, untracked) WaveNet-related bridge scaffold
│   │
│   ├── core/
│   │   ├── mod.rs                 # Re-exports: state, dsp, ui
│   │   ├── state/
│   │   │   ├── mod.rs             # Re-exports plugin_params
│   │   │   └── plugin_params.rs   # `InputSelect` enum, `BaseIOParams` (#[derive(Params)]),
│   │   │                          #   `EditorState`, and all FloatParam ranges/smoothers/formatters
│   │   ├── dsp/
│   │   │   ├── mod.rs             # Re-exports AnalyzerDsp, FFT_SIZE
│   │   │   └── analyzer.rs        # 2048-pt real FFT spectrum analyzer, Blackman-Harris window,
│   │   │                          #   log-tilt, exponential smoothing; consumes an rtrb ring buffer
│   │   └── ui/
│   │       ├── mod.rs             # Re-exports draw_spectrum, draw_signal_chain, ActivePanel, render_shared_panels
│   │       ├── main_view.rs       # `render_shared_panels()` layout + `draw_eq_band()` 3-knob band widget
│   │       ├── signal_chain.rs    # `ActivePanel` enum + `draw_signal_chain()` node graph with power toggles
│   │       └── spectrum.rs        # `draw_spectrum()` log-freq / dB spectrum plot
│   │
│   └── models/                    # Legacy/aux ONNX models (not on the live audio path)
│       ├── modelo_amp.onnx
│       └── modelo_amp_lstm_2026_03_10_18_09_28.onnx.data
│
├── dsp/                           # Faust DSP + C wrapper (linear processing)
│   ├── main.dsp                   # Faust source: 3-band parametric EQ (low-shelf/peak/high-shelf) + ma.tanh
│   ├── FaustModule.hpp            # GENERATED by Faust from main.dsp (class `mydsp`) — checked in
│   ├── wrapper.cpp                # C++ wrapper: FaustInstance, ParamMapUI, extern "C" faust_* functions
│   └── wrapper.h                  # C ABI header (input to bindgen; contract between Faust and Rust)
│
├── faust-ddsp/                    # Faust libraries on the compiler -I search path
│   ├── diff.lib                   # Differentiable DSP (automatic differentiation) library
│   └── filters.lib                # Stable SVF-based filters (low_shelf, peak_eq, high_shelf) used by main.dsp
│
├── neural/                        # Mojo neural drive + impulse-response assets
│   ├── main.mojo                  # Mojo source: mojo_init, mojo_process_block (tanh polynomial saturation)
│   ├── libneural.so               # GENERATED by Mojo (shared lib, linked as dylib=neural) — not versioned
│   └── drive/
│       ├── pre_eq_ir.wav          # Pre-EQ / tone-stack impulse response (~4 KB)
│       ├── cabinet_ir.wav         # Speaker cabinet impulse response (~20 KB)
│       ├── wavenet_drive.onnx     # WaveNet drive model (asset; ONNX graph)
│       └── wavenet_drive.onnx.data# WaveNet external weights blob
│
├── scripts/
│   ├── check_env.sh               # Validates Faust, Mojo, Modular CLI, and dsp/neural write permissions
│   └── run_standalone.sh          # `cargo run --release --bin standalone`
│
└── xtask/                         # nih-plug bundler task crate
    ├── Cargo.toml                 # depends on nih_plug_xtask (git)
    └── src/main.rs                # `nih_plug_xtask::main()` — powers `cargo xtask bundle`
```

### 3.1 File-by-file walkthrough (the parts that matter)

**`src/lib.rs`** — the plugin. Defines `BaseIO`, which owns:
- `params: Arc<BaseIOParams>` — the automatable parameter set.
- `analyzer_producer` / `analyzer_consumer` — an `rtrb` ring buffer (64K) feeding the spectrum analyzer from the audio thread.
- `faust: [Option<FaustProcessor>; 2]` — one Faust EQ instance per channel (L/R).
- `mojo: [Option<MojoProcessor>; 2]` — one Mojo drive per channel.
- `pre_eq_convolver` / `cabinet_convolver: [Option<FFTConvolver<f32>>; 2]` — the two IR convolution stages, per channel.
- `temp_buffer: Vec<f32>` — scratch for convolution (convolvers can't process in place).

`initialize()` sets sample rate on Faust+Mojo, then loads `pre_eq_ir.wav` and `cabinet_ir.wav` via `hound` (handling both i16 and f32 WAVs) and initializes the convolvers with `max_buffer_size`. `process()` runs the pipeline (Section 4). `editor()` builds the egui UI: header (input source combo, bypass), a shared knob helper bound to nih-plug's parameter setter, and the shared EQ/Neural panels.

> ⚠️ **Hardcoded path:** `initialize()` (and the standalone) load IRs from an
> absolute path `"/home/jao/VSCode/distortion/meu-novo-plugin/neural/drive/"`.
> This is machine-specific and should be made relative before distribution.

**`src/bin/standalone.rs`** — the standalone host. Highlights:
- `StandaloneState` — the plain, `Copy` parameter snapshot shared with the audio thread via `Arc<Mutex<…>>`; the UI writes it, the audio callback `try_lock`s a copy each block.
- `AudioCommand` / `AudioEvent` — message enums between the UI thread and the `audio_worker` thread (refresh devices, apply routing, stop).
- `audio_worker()` — owns CPAL input/output streams. On `ApplyRouting` it builds Faust + Mojo + two convolvers, loads the IRs, and runs the per-block DSP inside the input callback, pushing processed samples through a ring buffer to the output callback and the analyzer.
- `StandaloneApp` (`eframe::App`) — device/channel selectors, a buffer-size (latency) slider with adaptive range, sample-rate-mismatch warnings, an error explanation popup, and the shared panels.
- `beautify_linux_name()` — turns raw ALSA device names into friendly labels.

**`src/bridge/mod.rs`** — the `ExternalProcessor` trait: `init(&mut self, sample_rate: f32)` and `process_block(&mut self, buffer: *mut f32, length: usize)`. This is the single seam every external backend conforms to.

**`src/bridge/faust.rs`** — `FaustProcessor` holds a `FaustHandle`. `new()` calls `faust_create()`. `set_eq_params(...)` forwards nine EQ values to the nine `faust_set_eq_*` FFI setters. `Drop` calls `faust_destroy`. `unsafe impl Send` lets the opaque pointer move to the audio thread.

**`src/bridge/mojo.rs`** — `MojoProcessor` holds `is_ready`, `drive`, `output_gain`. `set_drive`/`set_output_gain` are `#[inline(always)]` and real-time safe (field writes only). `process_block` casts the buffer pointer to `usize` and calls `mojo_process_block(ptr, len, drive, output_gain)`.

**`src/core/state/plugin_params.rs`** — the nih-plug parameter definitions:
- `InputSelect` (`Stereo`, `Input1` Mic, `Input2` Guitar).
- `BaseIOParams`: `input_select`, `gain`, `bypass`, neural block (`neural_amp_volume`, `neural_drive`, `neural_output_gain`, `neural_amp_active`), and the 9 EQ params (low/mid/high × freq/gain/Q) plus `eq_active`. Each `FloatParam` has explicit range (Linear/Skewed), a smoother, unit, and value↔string formatters.
- `EditorState` bundles params + analyzer + ring-buffer consumer + `ActivePanel` for the editor closure.

**`src/core/dsp/analyzer.rs`** — `AnalyzerDsp`: keeps a rolling `audio_history` of `FFT_SIZE = 2048` samples, applies a Blackman-Harris window, runs a `realfft` forward FFT, converts magnitude to dB with a `+3 dB/octave` perceptual tilt, and exponentially smooths the spectrum (`prev*0.85 + db*0.15`).

**`src/core/ui/*`** — pure egui drawing: `spectrum.rs` (log-frequency spectrum with grid + labels), `signal_chain.rs` (clickable EQ/Neural nodes with per-module power toggles and bypass-aware colors), `main_view.rs` (`render_shared_panels` composes the header-independent central + bottom panels and `draw_eq_band` renders a labeled 3-knob band).

**`dsp/main.dsp`** — the Faust EQ. Imports `stdfaust.lib`, `diff.lib`, `filters.lib`. Three bands with `si.smoo`-smoothed sliders: low-shelf, mid peak-EQ, high-shelf, cascaded then soft-clipped with `ma.tanh`, applied to both stereo channels (`process = _,_ : (eq_chain, eq_chain)`).

**`dsp/wrapper.cpp` / `wrapper.h`** — declares the minimal Faust `UI`/`Meta`/`dsp` base interfaces, includes `FaustModule.hpp`, and defines `ParamMapUI` which captures each slider's `float*` zone into a `std::map<std::string, float*>`. The `SET_PARAM` macro looks up a label and writes the value directly into the Faust DSP's parameter memory — no per-sample lookup overhead.

**`neural/main.mojo`** — two `@export` functions. `mojo_init` is a stub. `mojo_process_block(address, size, drive, output_gain)` reconstructs an `UnsafePointer[Float32, MutAnyOrigin]` from the address, and per sample: applies `drive`, clamps to ±4, applies the tanh polynomial approximation `x*(27+x²)/(27+9x²)`, then multiplies by `output_gain`.

---

## 4. Processing Pipeline

The plugin processes audio in a **single pass per channel**. Below are the six
conceptual stages the task/architecture describes, mapped to the actual code in
`BaseIO::process` (`src/lib.rs`). (The inline comments in `lib.rs` number some
sub-steps 1–7; the conceptual grouping here is what matters.)

```
        ┌──────────────┐
 IN ───▶│ 0. Input     │  stereo / Input1(mic) / Input2(guitar) selection
        │    routing   │
        └──────┬───────┘
               ▼
        ┌──────────────┐
        │ 1. Faust EQ  │  3-band parametric EQ (if eq_active)   [FFI → C++]
        └──────┬───────┘
               ▼
        ┌──────────────┐
        │ 2. Pre-EQ    │  FFTConvolver × pre_eq_ir.wav          [Rust]
        │    convolve  │
        └──────┬───────┘
               ▼
        ┌──────────────┐
        │ 3. Mojo      │  tanh saturation drive (if neural)     [FFI → Mojo]
        │    neural    │
        └──────┬───────┘
               ▼
        ┌──────────────┐
        │ 4. Cabinet   │  FFTConvolver × cabinet_ir.wav         [Rust]
        │    IR        │
        └──────┬───────┘
               ▼
        ┌──────────────┐
        │ 5. Master    │  gain × (neural master vol) + NaN/Inf  [Rust]
        │    + sanitize│  sanitization (always, even in bypass)
        └──────┬───────┘
               ▼  ──▶ ring buffer ──▶ spectrum analyzer
  OUT ◀────────┘
```

### Stage 0 — Input routing
**File:** `src/lib.rs` (first `buffer.iter_samples()` loop).
Reads L (channel 0) and R (channel 1). Based on `input_select`:
- `Stereo` → (L, R) unchanged.
- `Input1` (Mic) → (L, L) — mono from the left/mic input.
- `Input2` (Guitar) → (R, R) — mono from the right/guitar input.
The selection is written back into the buffer before any DSP runs.

### Stage 1 — Faust 3-band parametric EQ
**Files:** `src/lib.rs` → `src/bridge/faust.rs` → `dsp/wrapper.cpp` → `dsp/main.dsp`.
Only if `eq_active`. Per channel, the smoothed EQ parameters are pushed via
`set_eq_params(...)`, then `faust.process_block(ptr, len)` runs the cascaded
low-shelf → peak → high-shelf → `tanh` filter in-place over the raw buffer pointer.

### Stage 2 — Pre-EQ convolution (LTI)
**File:** `src/lib.rs`, using `fft_convolver::FFTConvolver` + `neural/drive/pre_eq_ir.wav`.
A linear time-invariant convolution that colors the tone before the drive
(tone-stack / input coloration). Because the convolver cannot process in place,
the channel is copied to `temp_buffer` and convolved back into the channel; on
error it restores the dry copy.

### Stage 3 — Mojo neural drive (the distortion)
**Files:** `src/lib.rs` → `src/bridge/mojo.rs` → `neural/main.mojo`.
Only if `neural_amp_active`. `neural_drive` (pre-gain) and `neural_output_gain`
(makeup) are set, then `mojo.process_block(ptr, len)` applies the clamped tanh
polynomial saturation sample-by-sample, in place, zero-copy over the FFI boundary.

### Stage 4 — Cabinet IR convolution
**File:** `src/lib.rs`, `FFTConvolver` + `neural/drive/cabinet_ir.wav`.
Speaker-cabinet simulation — same copy-to-temp-then-convolve pattern as Stage 2.

### Stage 5 — Master gain + neural master volume + NaN sanitization
**File:** `src/lib.rs`.
- Multiply by the smoothed **`gain`** master parameter.
- If neural is active, multiply by **`neural_amp_volume`** (post master).
- **NaN/Inf sanitization runs unconditionally** (even under global `bypass`): any
  non-finite sample is zeroed, protecting downstream hardware/hosts.

After processing, a final loop pushes a mono mix `(L+R)*0.5` into the analyzer ring
buffer for the spectrum display. `set_latency_samples(0)` is reported because the
Mojo stage is synchronous and adds no latency (the convolvers are used in
zero-latency block mode).

### Standalone pipeline differences
`audio_worker` in `standalone.rs` runs the analogous chain
(Faust EQ → pre-EQ conv → Mojo drive → cabinet conv → sanitize) inside the CPAL
input callback, but:
- It processes explicit `buf_l`/`buf_r` vectors sized to the CPAL buffer.
- The neural stage uses `StandaloneState::neural_vol` for both drive and the final
  gain multiply (a simpler control surface than the plugin's separate drive/makeup/volume).
- The i16 input path currently only sanitizes (Faust/Mojo are on the f32 path).

---

## 5. Build System

### 5.1 `build.rs` orchestration

`build.rs` runs before compiling the Rust crate and performs, in order:

1. **`pre_build_check()`**
   - Verifies the **Faust** compiler exists (`faust --version`); `panic!`s with an install hint if missing.
   - Locates **Mojo** via `find_mojo_path()` — checks `$PATH` (`which mojo`), then `./.venv/bin/mojo`, then `~/.modular/pkg/.../bin/mojo`, then `~/.modular/bin/mojo`.
   - Emits `cargo:rustc-link-search=native=<mojo>/../lib` so the Mojo runtime libraries are on the linker path.

2. **`rerun-if-changed`** triggers for `dsp/wrapper.cpp`, `dsp/wrapper.h`, `dsp/main.dsp`, `neural/main.mojo`.

3. **Faust transpile (incremental):** if `dsp/main.dsp` is newer than `dsp/FaustModule.hpp` (or the `.hpp` is missing), run:
   ```
   faust -lang cpp -cn mydsp -vec -I faust-ddsp -i dsp/main.dsp -o dsp/FaustModule.hpp
   ```
   (`-cn mydsp` names the class, `-vec` enables vectorized code, `-I faust-ddsp` adds the local libs, `-i` inlines imports.)

4. **Mojo build (incremental):** if `neural/main.mojo` is newer than `neural/libneural.so` (or missing), run:
   ```
   mojo build --emit shared-lib neural/main.mojo -o neural/libneural.so
   ```

5. **`cc::Build`:** compile `dsp/wrapper.cpp` (C++, `-O3`) into a static library `faust_dsp` — this pulls in `FaustModule.hpp`.

6. **`bindgen`:** generate Rust bindings from `dsp/wrapper.h`, allow-listing `faust_.*` functions, into `$OUT_DIR/bindings_faust.rs` (included by `src/bridge/faust.rs`).

7. **Mojo linking:** emit `cargo:rustc-link-search=native=<manifest>/neural` and `cargo:rustc-link-lib=dylib=neural` so the final binary links `libneural.so`.

### 5.2 Dependency chain

```
main.dsp ──faust──▶ FaustModule.hpp ──┐
                                       ├─(#include)─ wrapper.cpp ──cc::Build──▶ libfaust_dsp.a ──┐
wrapper.h ──bindgen──▶ bindings_faust.rs ───────────────────────(include!)────────────────────┤
                                                                                                ├──▶ Rust crate
main.mojo ──mojo build──▶ libneural.so ──(rustc-link-lib=dylib=neural)─────────────────────────┘   (distortion
                                                                                                     + standalone)
```

At **runtime**, `libneural.so` must be discoverable via `LD_LIBRARY_PATH` (the
Makefile adds `$(PWD)/neural` and the Mojo `lib` dir).

### 5.3 Makefile targets

| Target | What it does |
|---|---|
| `make help` | Prints the command menu |
| `make check-env` | Runs `scripts/check_env.sh` (Faust/Mojo/Modular + dir permissions) |
| `make build-faust` | Transpiles `dsp/main.dsp` → `dsp/FaustModule.hpp` (skips gracefully if faust or the `.dsp` is missing) |
| `make build-mojo` | Compiles `neural/main.mojo` → `neural/libneural.so` (prefers `./.venv/bin/mojo`) |
| `make pre-build` | `check-env` → `build-faust` → `build-mojo` |
| `make run` | `pre-build` then `scripts/run_standalone.sh` (`cargo run --release --bin standalone`) |
| `make build` | `pre-build` then `cargo build --release` |
| `make bundle` | `pre-build` then `cargo xtask bundle distortion --release` (VST3 + CLAP) |
| `make clean` | `cargo clean` + removes generated `dsp/*.hpp`, `dsp/*.cpp`, `neural/*.so|dylib|dll` |
| `make init` | Runs `init_plugin.py` if present (template scaffolding; no-op once initialized) |

**Environment variables set by the Makefile:**
```
LIBTORCH                 ?= $(HOME)/libtorch/libtorch      # legacy neural-amp support
LIBTORCH_BYPASS_VERSION_CHECK := 1
MOJO_HOME                ?= $(HOME)/.modular/pkg/packages.modular.com_mojo
LD_LIBRARY_PATH          := $(MOJO_HOME)/lib:$(PWD)/neural:$(LD_LIBRARY_PATH)
```

### 5.4 `cargo xtask` bundler

The `xtask` crate is a thin wrapper (`nih_plug_xtask::main()`). `cargo xtask bundle
distortion --release` reads `bundler.toml` (display name "Distortion") and produces
`.vst3` and `.clap` bundles for the host OS.

---

## 6. Key Design Patterns

### 6.1 Zero-copy FFI (Rust ↔ Mojo — the "Address Bypass" pattern)

No audio memory is allocated or copied at the language boundary. The Rust side casts
the raw buffer pointer to an integer address:

```rust
// src/bridge/mojo.rs
extern "C" {
    fn mojo_init(sample_rate: f32);
    fn mojo_process_block(address: usize, size: usize, drive: f32, output_gain: f32);
}

fn process_block(&mut self, buffer: *mut f32, length: usize) {
    let ptr = buffer as usize;                 // pointer → usize, zero copy
    unsafe { mojo_process_block(ptr, length, self.drive, self.output_gain); }
}
```

Mojo reconstructs a pointer from the raw address and mutates in place:

```mojo
# neural/main.mojo
@export
fn mojo_process_block(address: Int, size: Int, drive: Float32, output_gain: Float32):
    var data = UnsafePointer[Float32, MutAnyOrigin](unsafe_from_address=address)
    for var i in range(size):
        var x = data[i] * drive
        if x > 4.0:  x = 4.0
        elif x < -4.0: x = -4.0
        var x2 = x * x
        data[i] = (x * (27.0 + x2) / (27.0 + 9.0 * x2)) * output_gain
```

**Why `Int`/address instead of a pointer type?** — see the Mojo `@export`
constraint below.

### 6.2 The Mojo `@export` address-bypass constraint

Mojo's `@export` functions **cannot accept `UnsafePointer` parameters directly**
(parametricity restriction). The project's rule:

- ✅ Pass the pointer as an `Int` address; reconstruct `UnsafePointer(unsafe_from_address=…)` inside.
- ✅ Always use `@export` (not `@extern_c`) to emit FFI-visible symbols in the `.so`.
- ✅ **No `fn main()`** in a shared library.
- ✅ Qualified import: `from std.memory import UnsafePointer`.

### 6.3 The `ExternalProcessor` trait (backend abstraction)

```rust
// src/bridge/mod.rs
pub trait ExternalProcessor {
    fn init(&mut self, sample_rate: f32);
    fn process_block(&mut self, buffer: *mut f32, length: usize);
}
```

Both `FaustProcessor` and `MojoProcessor` implement it, so the orchestrator treats
every external DSP backend uniformly: initialize with a sample rate, then process a
raw buffer in place. Adding a new native backend means implementing this one trait.

### 6.4 Faust C wrapper (opaque handle + captured parameter map)

`dsp/wrapper.cpp` exposes the Faust `mydsp` C++ class as a C ABI keyed by an opaque
`FaustHandle`. On `faust_create()` it builds a `ParamMapUI` that captures each
Faust slider's `float*` **zone** into a `std::map<label, float*>`. Parameter setters
(`faust_set_eq_low_freq`, …) write straight into those zones via the `SET_PARAM`
macro — no per-sample string lookup on the audio thread. `bindgen` turns
`wrapper.h` into Rust bindings; `cc::Build` compiles the wrapper as a static lib.

### 6.5 Parameter smoothing (nih-plug)

All continuous `FloatParam`s carry a smoother
(`SmoothingStyle::Logarithmic(50.0)` for gains/frequencies, `Linear(50.0)` for
gains in dB and Q). On the audio thread you **must** pull `.smoothed.next()` per
block (as `process()` does) and **never** read `.value()` during processing, so
parameter changes don't cause zipper noise or clicks. Faust also smooths its own
slider inputs internally with `si.smoo`.

### 6.6 Lock-free audio→UI handoff (ring buffers)

The audio thread never blocks the UI. Processed samples are pushed into an
`rtrb` single-producer/single-consumer ring buffer (64K). The UI thread `try_lock`s
the consumer and drains it into `AnalyzerDsp` for the FFT/spectrum. The standalone
uses a second ring buffer to pass processed audio from the input callback to the
output callback.

### 6.7 Per-channel processor instances

Faust and Mojo processors are stored as `[Option<_>; 2]` (one per stereo channel)
so left/right filter state stays independent. `channel_idx.min(1)` guards mono/×N
layouts.

---

## 7. Dependencies

### 7.1 Rust crates (`Cargo.toml`)

| Crate | Version | Purpose |
|---|---|---|
| `nih_plug` | git (robbert-vdh) | Plugin framework; features `assert_process_allocs`, `standalone` |
| `nih_plug_egui` | git | egui editor integration for the plugin UI |
| `nih_plug_vizia` | git | (declared) alternative Vizia UI backend |
| `eframe` | 0.31 | Standalone native egui app shell |
| `egui_knob` | 0.2 | Rotary knob widget used across both UIs |
| `cpal` | 0.15 | Cross-platform audio I/O (standalone) |
| `fft-convolver` | 0.3 | Partitioned FFT convolution (pre-EQ + cabinet IRs) |
| `realfft` | 3.5 | Real-input FFT for the spectrum analyzer |
| `num-complex` | 0.4 | Complex numbers for FFT output |
| `hound` | 3.5 | WAV reading for impulse responses |
| `rtrb` | 0.3 | Lock-free SPSC ring buffer (audio → analyzer) |
| `ringbuf` | 0.4 | (declared) additional ring-buffer utilities |
| `biquad` | 0.5 | (declared) biquad filters |
| `rubato` | 1.0 | (declared) sample-rate conversion |
| `nalgebra` | 0.34 | (declared) linear algebra |
| `arc-swap` | 1.7 | (declared) atomic Arc swapping |
| `fast-math` | 0.1 | (declared) fast approximate math |
| `num-complex` / `rfd` | — | complex math / native file dialogs |

Build-dependencies: **`cc` 1.0** (compile the C++ wrapper) and **`bindgen` 0.71**
(generate Faust bindings). Release profile uses `lto = "thin"` and
`strip = "symbols"`; a `profiling` profile keeps debug info.

> Some declared crates (`biquad`, `rubato`, `nalgebra`, `ringbuf`, `arc-swap`,
> `fast-math`, `nih_plug_vizia`) are dependencies of the broader template and are
> not all on the current active audio path.

### 7.2 System dependencies

| Tool | Needed for | Install |
|---|---|---|
| Rust + Cargo | Build everything | `rustup` |
| **Faust** | Transpile `main.dsp` → C++ | `sudo apt install faust` / `brew install faust` |
| **Mojo** (Modular MAX SDK) | Build `libneural.so` | `modular install mojo` |
| **Clang / LLVM** | `bindgen` needs libclang | `sudo apt install clang` |
| **ALSA dev libs** | CPAL backend (Linux) | `sudo apt install libasound2-dev` |
| (optional) LibTorch | Legacy neural-amp path | env `LIBTORCH` in Makefile |

### 7.3 Bundled assets

| Asset | Role |
|---|---|
| `neural/drive/pre_eq_ir.wav` | Pre-EQ / tone-stack impulse response (Stage 2) |
| `neural/drive/cabinet_ir.wav` | Speaker cabinet impulse response (Stage 4) |
| `neural/drive/wavenet_drive.onnx` (+ `.data`) | WaveNet drive model asset (present; ONNX graph + weights) |
| `src/models/modelo_amp.onnx`, `…_lstm_…onnx.data` | Legacy/auxiliary amp models (not on the live Mojo path) |
| `faust-ddsp/diff.lib`, `filters.lib` | Faust libraries on the compiler include path |

---

## 8. Development Commands

```bash
# Validate the toolchain (Faust, Mojo, Modular, write permissions)
make check-env

# Compile just the generated sources (Faust .dsp → .hpp, Mojo .mojo → .so)
make pre-build

# Standalone dev mode (CPAL audio + eframe UI) — the fast iteration loop
make run
#   equivalently: cargo run --release --bin standalone

# Release build of the whole crate
make build
#   equivalently: cargo build --release

# Bundle VST3 + CLAP for DAWs
make bundle
#   equivalently: cargo xtask bundle distortion --release

# Full clean (target/, generated dsp/*.hpp|cpp, neural/*.so)
make clean

# Debug build (slower; nih-plug is a git dependency)
cargo build
```

**Golden rule (from the README):** every change to the audio processing should be
validated in **Standalone** (`make run`) before generating the DAW bundle.

**Runtime note:** `libneural.so` must be on the library path at runtime. The
Makefile sets `LD_LIBRARY_PATH` to include `$(PWD)/neural` and the Mojo `lib`
directory; if you run binaries directly, replicate that.

**Interactive login / environment:** if Mojo lives in a local `.venv`, activate it
(`source .venv/bin/activate`) before building, or rely on the automatic `.venv`
detection in `build.rs` and the Makefile.

---

## 9. Appendix — Known Rough Edges

These are worth knowing before you ship or refactor:

- **Hardcoded absolute IR paths.** Both `src/lib.rs::initialize()` and
  `src/bin/standalone.rs` load IRs from
  `"/home/jao/VSCode/distortion/meu-novo-plugin/neural/drive/"`. This breaks on any
  other machine and in installed plugin bundles — should become a relative/embedded
  path (e.g. `include_bytes!` or a bundle-resource lookup).
- **`mojo_init` is a stub.** Sample rate is accepted but unused by the Mojo drive
  today; the saturation is sample-rate-independent.
- **Two independent pipelines.** Plugin (`lib.rs`) and standalone (`standalone.rs`)
  each implement the chain separately, with slightly different neural controls. Keep
  them in sync when changing DSP.
- **Standalone i16 input path** only sanitizes — it does not run Faust/Mojo/IR
  processing (that lives on the f32 path).
- **No test suite.** There are no `#[test]` functions; validation is manual via the
  standalone host and the spectrum analyzer.
- **`nih_plug_vizia`** and several math crates are declared but not on the active
  path — template residue.

---

*Generated as a zero-context initiation map of the Distortion plugin codebase.
For deeper build/FFI/troubleshooting narrative (in Portuguese), see `README.md`;
for repo-working guidance, see `CLAUDE.md`.*
