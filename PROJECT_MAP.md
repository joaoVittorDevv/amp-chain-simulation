# PROJECT_MAP.md — Distortion Guitar Amp Plugin

> **Zero-context onboarding document.** Read this first if you are new to the
> repository. It maps the whole codebase: what it is, how the three languages
> interoperate, every source file's role, the real-time audio pipeline (with
> `file:line` anchors), the build orchestration, the key design patterns, the
> dependencies, the day-to-day commands, a fast reference, and the known rough
> edges.
>
> Combined from three independent agent-generated mappings and verified against
> the source tree on **2026-07-03**. Where prose documentation (e.g. `README.md`)
> disagrees with the code, **the code is the source of truth.**

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
9. [Fast Reference](#9-fast-reference)
10. [Known Issues & Rough Edges](#10-known-issues--rough-edges)

---

## 1. Project Overview

**Distortion** is a real-time **guitar distortion / amp-simulation audio plugin**.
It builds both as a native plugin (**VST3** and **CLAP** for DAW hosts) and as a
**standalone desktop application** with its own audio I/O and GUI.

**Purpose:** process live audio through a full guitar amplifier signal chain —
linear DSP (EQ, convolution) plus neural saturation (a Mojo-powered `tanh`
non-linearity) — with zero added latency and zero allocations on the audio thread.

The modeled signal chain:

- A **3-band parametric equalizer** (tone shaping).
- A **pre-EQ impulse-response convolution** (tone-stack / input coloration).
- A **neural / non-linear drive stage** (`tanh` saturation — the "distortion").
- A **cabinet impulse-response convolution** (speaker-cabinet simulation).
- **Master gain** and output conditioning (NaN/Inf sanitization).

It is deliberately **polyglot**, using each language for what it is best at:

| Language | Role | Why |
|---|---|---|
| **Rust** (2021) | Host / orchestrator, UI, audio routing, plugin lifecycle | Memory-safe systems language; `nih-plug` yields VST3/CLAP/standalone from one codebase |
| **Faust** | Linear DSP (parametric EQ) | Domain-specific DSP language; compiles to tight, correct C++ filter code |
| **Mojo** (Modular MAX SDK) | Non-linear neural drive (saturation) | High-performance, Python-like; used here for zero-copy in-place saturation |
| **C/C++** | Thin FFI wrapper around Faust-generated code | Exposes the Faust class as a stable C ABI for Rust `bindgen` |

### Key technologies

- **[nih-plug](https://github.com/robbert-vdh/nih-plug)** — Rust plugin framework (VST3, CLAP, standalone). Provides the `Plugin` trait, parameter system, and `nih_plug_egui` editor integration.
- **[Faust](https://faust.grame.fr/)** — functional DSP language, transpiled to C++ (`FaustModule.hpp`, Faust `2.85.1`) at build time.
- **[Mojo](https://www.modular.com/mojo)** — compiled to a shared library `libneural.so` and linked into the Rust binary.
- **[egui](https://github.com/emilk/egui)** / **eframe** — immediate-mode GUI (spectrum analyzer, knobs, signal-chain view); knobs via `egui_knob`.
- **[CPAL](https://github.com/RustAudio/cpal)** — cross-platform audio I/O for the standalone build.
- **[fft-convolver](https://crates.io/crates/fft-convolver)** — partitioned FFT convolution for the impulse responses.
- **[realfft](https://crates.io/crates/realfft)** — the 2048-point spectrum-analyzer FFT.

### At a glance

| | |
|---|---|
| **Package name** | `distortion` (`Cargo.toml`), version `0.1.0` |
| **Display name** | **"Distortion"** (`bundler.toml`) |
| **Exported plugin type** | `BaseIO` (`src/lib.rs`) |
| **Crate types** | `cdylib` (plugin) + `lib` (shared by the `standalone` binary) |
| **License** | GPL-3.0-or-later |
| **Author** | jao \<joaovittorh1@gmail.com\> |
| **VST3 ID** | `A9ED13810DBC4A4A98540F0F671E9E1A` |
| **CLAP ID** | `.distortion` |
| **Editor size** | 800×450 (plugin editor) · 1000×500 window (standalone) |
| **Tests** | none — no `#[test]` functions exist; validation is manual via the standalone host |

> **Onboarding note — doc drift.** Previous `tract-onnx` background inference has
> been **replaced by synchronous Mojo FFI**. The current dependency list contains
> no `tract-onnx`, and the current Mojo FFI signature is
> `mojo_process_block(address, size, drive, output_gain)`. ONNX assets are still
> bundled but are **not** on the live audio path. See §10.

---

## 2. Architecture

### 2.1 The three-language stack

Rust acts as the orchestrator, Faust generates linear DSP C++ code, and Mojo
implements neural non-linear processing — all connected via C-ABI FFI.

```
┌──────────────────────────────────────────────────────────────────┐
│                     HOST (DAW or Standalone)                      │
│  Audio buffers arrive as *mut f32 slices (2-channel stereo)       │
└─────────────────────────────┬────────────────────────────────────┘
                              │ nih_plug::Plugin trait
┌─────────────────────────────▼────────────────────────────────────┐
│                  RUST ORCHESTRATOR (src/lib.rs)                   │
│  BaseIO struct — owns all processors, parameters, analyzers       │
│  BaseIO::process() — single-pass, in-place, per-channel pipeline  │
│                                                                   │
│  ┌───────────────┐  ┌─────────────┐  ┌───────────────────────────┐│
│  │ FaustProcessor│  │MojoProcessor│  │ FFTConvolver (pre_eq, cab)││
│  │ (bridge/faust)│  │(bridge/mojo)│  │ (from fft-convolver crate)││
│  └──────┬────────┘  └──────┬──────┘  └───────────────────────────┘│
│         │                  │                                      │
│   ExternalProcessor trait (bridge/mod.rs) — uniform interface     │
└─────────┼──────────────────┼──────────────────────────────────────┘
          │ C ABI (bindgen)  │ C ABI (manual extern "C", zero-copy)
┌─────────▼───────┐  ┌────────▼──────────────┐
│  FAUST ENGINE   │  │   MOJO ENGINE         │
│  (C++ static    │  │   (shared library)    │
│   lib)          │  │                       │
│  dsp/wrapper.cpp│  │  neural/main.mojo     │
│  dsp/wrapper.h  │  │  neural/libneural.so  │
│  ┌────────────┐ │  │                       │
│  │FaustModule │ │  │  Tanh polynomial      │
│  │   .hpp     │ │  │  approximation        │
│  │ (mydsp)    │ │  │  x*(27+x²)/(27+9x²)   │
│  └────────────┘ │  │                       │
│  3-band SVF EQ  │  │  Zero-copy: pointer   │
│  low_shelf      │  │  passed as usize      │
│  peak_eq        │  │  address              │
│  high_shelf     │  │                       │
│  ma.tanh clip   │  │                       │
└─────────────────┘  └───────────────────────┘

Plus two Rust-side FFTConvolver stages (pre-EQ IR + cabinet IR) and the
WAV impulse responses in neural/drive/.
```

### 2.2 How the languages interoperate (FFI)

There are two distinct FFI bridges, both crossing the boundary **in-place, without
copying audio data**:

1. **Rust ↔ Faust (via C++):**
   - `dsp/main.dsp` is transpiled by the Faust compiler into `dsp/FaustModule.hpp` (a C++ class `mydsp`).
   - `dsp/wrapper.cpp` / `dsp/wrapper.h` wrap that class in a plain **C ABI** (`faust_create`, `faust_init`, `faust_process`, `faust_set_eq_*`, `faust_destroy`) behind an opaque `FaustHandle` pointer.
   - `build.rs` compiles the wrapper into a static library `libfaust_dsp.a` with `cc::Build`, and runs `bindgen` on `wrapper.h` to auto-generate Rust bindings (`bindings_faust.rs`).
   - `src/bridge/faust.rs` includes those bindings and exposes a safe-ish `FaustProcessor`.

2. **Rust ↔ Mojo (zero-copy address bypass):**
   - `neural/main.mojo` exports `mojo_init` and `mojo_process_block` as C symbols, compiled to `neural/libneural.so`.
   - Rust declares them in an `extern "C"` block (`src/bridge/mojo.rs`).
   - The audio buffer pointer is cast to a `usize` **address** and passed by value. Mojo reconstructs an `UnsafePointer` from the address and mutates the samples in place.

Both backends implement the shared `ExternalProcessor` trait, giving the
orchestrator one uniform interface (see §6.3).

### 2.3 Dual-target architecture

The same DSP building blocks run in two very different hosts:

| | **Plugin mode** (`src/lib.rs`) | **Standalone mode** (`src/bin/standalone.rs`) |
|---|---|---|
| Entry | `BaseIO` struct implementing nih-plug `Plugin` | `StandaloneApp` implementing `eframe::App` |
| Audio | Host DAW calls `process()` | CPAL streams on a dedicated `audio_worker` thread |
| Params | nih-plug `#[derive(Params)]` (`BaseIOParams`) — automatable, persisted | Plain `StandaloneState` behind `Arc<Mutex<…>>` |
| UI | `nih_plug_egui::create_egui_editor` | `eframe::run_native` |
| Routing | Fixed 2-in/2-out layout | User-selectable host/device/channel/buffer routing |
| Shared code | `core::dsp`, `core::ui`, `bridge::*` | same |

> **⚠️ Important nuance — two independent pipeline implementations.** The two
> targets **do not share a single `process` loop.** `core/` provides the shared
> DSP (analyzer) and UI widgets, and `bridge/` provides the shared FFI adapters,
> but each target implements its own audio pipeline. When you change DSP behavior,
> **keep both in sync manually.** They also differ in their neural controls (see §4).

---

## 3. Source Layout

Full annotated tree (`target/` and other pure build output omitted):

```
meu-novo-plugin/
│
├── PROJECT_MAP.md              ← This file
├── README.md                   ← Portuguese "zero-context" dev manual (some snippets stale — see §10)
├── CLAUDE.md                   ← Guidance for Claude Code working in this repo
│
├── Cargo.toml                  ← Crate manifest: deps, [lib] cdylib+lib, [[bin]] standalone, workspace, profiles
├── Cargo.lock                  ← Locked dependency graph
├── build.rs                    ← Build orchestrator (Faust transpile → Mojo build → cc → bindgen → link)
├── Makefile                    ← Developer command interface (run/build/bundle/clean/pre-build/check-env)
├── bundler.toml                ← nih-plug xtask bundler metadata ([baseIO] → "Distortion")
├── pyproject.toml              ← Minimal Python stub (`quickstart`); NOT used by the Rust build
├── wrapper.o                   ← ⚠ Tracked stray ELF object (Cargo rebuilds native artifacts anyway — see §10)
├── .gitignore
│
├── src/                        ← ─── Rust Source Root ───
│   ├── lib.rs                  │ PLUGIN entry point. BaseIO struct, Plugin/ClapPlugin/Vst3Plugin impls,
│   │                           │   editor(), initialize() (loads IRs), process() (staged pipeline),
│   │                           │   nih_export_clap!/nih_export_vst3! macros.
│   │
│   ├── bin/
│   │   └── standalone.rs       │ STANDALONE app (~1050 lines): CPAL host/device/channel routing,
│   │                           │   audio_worker thread, per-block DSP, eframe UI, buffer-size UX,
│   │                           │   error popups, beautify_linux_name() device-name cleaner.
│   │
│   ├── bridge/                 │ FFI adapters — the Rust ↔ external-language boundary
│   │   ├── mod.rs              │   ExternalProcessor trait (init + process_block(*mut f32, len)).
│   │   ├── faust.rs            │   FaustProcessor: includes bindgen bindings, wraps FaustHandle,
│   │   │                       │     set_eq_params() (9 setters), Drop→faust_destroy, unsafe impl Send.
│   │   ├── mojo.rs             │   MojoProcessor: extern "C" decls, drive/output_gain fields,
│   │   │                       │     zero-copy pointer→usize call to mojo_process_block.
│   │   └── wavenet.rs          │   (untracked in working tree) WaveNet-related bridge scaffold.
│   │
│   ├── core/
│   │   ├── mod.rs              │ Re-exports: state, dsp, ui.
│   │   ├── state/
│   │   │   ├── mod.rs          │   Re-exports plugin_params.
│   │   │   └── plugin_params.rs│   InputSelect enum, BaseIOParams (#[derive(Params)]), EditorState;
│   │   │                       │     all FloatParam ranges/smoothers/units/formatters. EguiState 800×450.
│   │   ├── dsp/
│   │   │   ├── mod.rs          │   Re-exports AnalyzerDsp, FFT_SIZE (= 2048).
│   │   │   └── analyzer.rs     │   2048-pt real FFT analyzer: Blackman-Harris window, +3 dB/oct tilt,
│   │   │                       │     EMA smoothing (prev*0.85 + db*0.15); fixed 48 kHz display rate.
│   │   └── ui/
│   │       ├── mod.rs          │   Re-exports draw_spectrum, draw_signal_chain, ActivePanel, render_shared_panels.
│   │       ├── main_view.rs    │   render_shared_panels() layout + draw_eq_band() 3-knob band widget.
│   │       ├── signal_chain.rs │   ActivePanel enum + draw_signal_chain() node graph with power toggles.
│   │       └── spectrum.rs     │   draw_spectrum() log-frequency / dB spectrum plot with grid + labels.
│   │
│   └── models/                 │ Legacy/aux ONNX models (NOT on the live audio path)
│       ├── modelo_amp.onnx
│       └── modelo_amp_lstm_2026_03_10_18_09_28.onnx.data
│
├── dsp/                        ← ─── Faust DSP + C wrapper (linear processing) ───
│   ├── main.dsp                │ Faust source: 3-band parametric EQ (low-shelf/peak/high-shelf) + ma.tanh.
│   ├── FaustModule.hpp         │ GENERATED by Faust from main.dsp (class `mydsp`) — ⚠ tracked in git.
│   ├── wrapper.cpp             │ C++ wrapper: FaustInstance, ParamMapUI (label→float*), extern "C" faust_*.
│   └── wrapper.h               │ C ABI header (input to bindgen; the Faust↔Rust contract).
│
├── faust-ddsp/                 ← ─── Faust libraries on the compiler -I search path ───
│   ├── filters.lib             │ Stable SVF-based filters (low_shelf, peak_eq, high_shelf) used by main.dsp.
│   └── diff.lib                │ Differentiable DSP (automatic differentiation) library; imported by main.dsp.
│
├── neural/                     ← ─── Mojo neural drive + IR assets ───
│   ├── main.mojo               │ Mojo source: mojo_init (stub) + mojo_process_block (tanh polynomial).
│   ├── libneural.so            │ GENERATED by Mojo (shared lib, linked dylib=neural) — ⚠ tracked, platform-specific.
│   └── drive/
│       ├── pre_eq_ir.wav       │   Pre-EQ / tone-stack impulse response (Stage 2).
│       ├── cabinet_ir.wav      │   Speaker cabinet impulse response — mono, 44.1 kHz, 16-bit PCM (Stage 4).
│       ├── wavenet_drive.onnx  │   WaveNet drive model asset (present; NOT executed at runtime).
│       └── wavenet_drive.onnx.data  │  WaveNet external-weights blob.
│
├── scripts/
│   ├── check_env.sh            │ Validates Faust, Mojo, Modular CLI, and dsp/neural write permissions.
│   └── run_standalone.sh       │ `cargo run --release --bin standalone`.
│
├── xtask/                      ← ─── nih-plug bundler task crate ───
│   ├── Cargo.toml              │ Workspace member; depends on nih_plug_xtask (git).
│   └── src/main.rs             │ `nih_plug_xtask::main()` — powers `cargo xtask bundle`.
│
└── .claude/
    └── settings.json           ← Claude Code project settings.
```

### 3.1 File sizes (approximate)

| File | Lines | Language | Role |
|---|---|---|---|
| `src/bin/standalone.rs` | ~1,050 | Rust | Standalone app (CPAL + eframe) |
| `src/lib.rs` | ~434 | Rust | Plugin entry point + processing pipeline |
| `src/core/state/plugin_params.rs` | ~170 | Rust | Parameter definitions |
| `src/core/ui/signal_chain.rs` | ~166 | Rust | Signal-chain UI nodes |
| `build.rs` | ~129 | Rust | Build orchestration |
| `dsp/wrapper.cpp` | ~129 | C++ | Faust C-ABI wrapper |
| `src/core/ui/main_view.rs` | ~91 | Rust | UI layout orchestrator |
| `src/core/ui/spectrum.rs` | ~86 | Rust | Spectrum visualization |
| `src/core/dsp/analyzer.rs` | ~73 | Rust | FFT analyzer |
| `src/bridge/faust.rs` | ~69 | Rust | Faust FFI adapter |
| `src/bridge/mojo.rs` | ~65 | Rust | Mojo FFI adapter |
| `Makefile` | ~76 | Make | Build commands |
| `dsp/wrapper.h` | ~43 | C | Faust ABI contract |
| `neural/main.mojo` | ~35 | Mojo | Neural saturation |
| `dsp/main.dsp` | ~29 | Faust | EQ + soft clipping DSP |
| `src/bridge/mod.rs` | ~13 | Rust | ExternalProcessor trait |

### 3.2 The files that matter most

**`src/lib.rs`** — the plugin. Defines `BaseIO`, which owns:
- `params: Arc<BaseIOParams>` — the automatable parameter set.
- `analyzer_producer` / `analyzer_consumer` — an `rtrb` ring buffer (`1024 * 64` = 65,536) feeding the spectrum analyzer from the audio thread.
- `faust: [Option<FaustProcessor>; 2]` — one Faust EQ instance per channel (L/R).
- `mojo: [Option<MojoProcessor>; 2]` — one Mojo drive per channel.
- `pre_eq_convolver` / `cabinet_convolver: [Option<FFTConvolver<f32>>; 2]` — the two IR convolution stages, per channel.
- `temp_buffer: Vec<f32>` — scratch for convolution (convolvers cannot process in place).

`initialize()` sets sample rate on Faust + Mojo, then loads `pre_eq_ir.wav` and `cabinet_ir.wav` via `hound` (handling both i16 and f32 WAVs) and initializes the convolvers with `max_buffer_size`. `process()` runs the pipeline (§4). `editor()` builds the egui UI (header with input-source combo + bypass, shared knob helper bound to nih-plug's parameter setter, shared EQ/Neural panels).

**`src/bin/standalone.rs`** — the standalone host. Highlights:
- `StandaloneState` — a plain parameter snapshot shared with the audio thread via `Arc<Mutex<…>>`; the UI writes it, the audio callback locks a copy each block. Note it exposes a **single `neural_vol`** control rather than the plugin's separate drive/makeup/volume.
- `AudioCommand` / `AudioEvent` — message enums between the UI thread and the `audio_worker` thread (`RefreshDevices`, `ApplyRouting`, `Stop` / `DevicesRefreshed`, `StreamStarted`).
- `audio_worker()` — owns CPAL input/output streams. On `ApplyRouting` it builds Faust + Mojo + two convolvers, loads the IRs, and runs the per-block DSP inside the input callback, pushing processed samples through a ring buffer to the output callback and the analyzer.
- `StandaloneApp` (`eframe::App`) — device/channel selectors, a buffer-size (latency) slider with adaptive range, sample-rate-mismatch warnings, an error-explanation popup, and the shared panels. `main()` opens a `1000×500` window titled "BaseIO Standalone".

**`src/bridge/mod.rs`** — the `ExternalProcessor` trait: `init(&mut self, sample_rate: f32)` and `process_block(&mut self, buffer: *mut f32, length: usize)`. The single seam every native backend conforms to.

**`src/bridge/faust.rs`** — `FaustProcessor` holds a `FaustHandle`. `new()` calls `faust_create()` (returns `None` on null). `set_eq_params(...)` forwards nine EQ values to the nine `faust_set_eq_*` FFI setters. `Drop` calls `faust_destroy`. `unsafe impl Send` lets the opaque pointer move to the audio thread.

**`src/bridge/mojo.rs`** — `MojoProcessor` holds `is_ready`, `drive`, `output_gain` (defaults `false / 1.0 / 1.0`). `set_drive`/`set_output_gain` are `#[inline(always)]` and real-time safe (field writes only). `process_block` casts the buffer pointer to `usize` and calls `mojo_process_block(ptr, len, drive, output_gain)`.

**`src/core/state/plugin_params.rs`** — the nih-plug parameter definitions:
- `InputSelect` (`Stereo`, `Input1` = Mic, `Input2` = Guitar).
- `BaseIOParams`: `editor_state`, `input_select`, `gain`, `bypass`, the neural block (`neural_amp_volume`, `neural_drive`, `neural_output_gain`, `neural_amp_active`), the 9 EQ params (low/mid/high × freq/gain/Q), and `eq_active`. Frequency params use a **`FloatRange::Skewed`** range; each `FloatParam` has a smoother, unit, and value↔string formatters.
- `EditorState` bundles params + analyzer + ring-buffer consumer + `ActivePanel` for the editor closure.

**`src/core/dsp/analyzer.rs`** — `AnalyzerDsp`: keeps a rolling history of `FFT_SIZE = 2048` samples, applies a 4-term Blackman-Harris window, runs a `realfft` forward FFT, converts magnitude to dB with a `+3 dB/octave` perceptual tilt, and exponentially smooths the spectrum (`prev*0.85 + db*0.15`). Uses a **fixed 48 kHz** display sample rate for the frequency axis.

**`src/core/ui/*`** — pure egui drawing: `spectrum.rs` (log-frequency spectrum with grid + labels), `signal_chain.rs` (clickable EQ/Neural nodes with per-module power toggles and bypass-aware colors), `main_view.rs` (`render_shared_panels` composes the central + bottom panels; `draw_eq_band` renders a labeled 3-knob band).

**`dsp/main.dsp`** — the Faust EQ. Imports `stdfaust.lib`, `diff.lib`, `filters.lib`. Three `si.smoo`-smoothed bands: low-shelf, mid peak-EQ, high-shelf, cascaded and soft-clipped with `ma.tanh`, applied to both stereo channels: `process = _,_ : (eq_chain, eq_chain);` where `eq_chain = eq_low : eq_mid : eq_high : ma.tanh`.

**`dsp/wrapper.cpp` / `wrapper.h`** — declare the minimal Faust `UI`/`Meta`/`dsp` base interfaces, include `FaustModule.hpp`, and define `ParamMapUI`, which captures each slider's `float*` zone into a `std::map<std::string, float*>`. The `SET_PARAM` macro looks up a label and writes the value straight into the Faust DSP's parameter memory — no per-sample lookup on the audio thread.

**`neural/main.mojo`** — two `@export` functions. `mojo_init(sample_rate: Float64)` is a stub. `mojo_process_block(address, size, drive, output_gain)` reconstructs an `UnsafePointer[Float32, MutAnyOrigin]` from the address and, per sample: applies `drive`, clamps to ±4, applies the tanh polynomial approximation `x*(27+x²)/(27+9x²)`, then multiplies by `output_gain`.

---

## 4. Processing Pipeline

The plugin processes audio **single-pass, in-place, per-channel** in
`BaseIO::process()` (`src/lib.rs:296`). Processing is synchronous and zero-latency
(`_context.set_latency_samples(0)` at `src/lib.rs:311`). The inline comments in
`lib.rs` number stages 1–7; the conceptual six-stage grouping below is what the
architecture describes (master gain + neural volume + sanitization collapse into
"output conditioning").

```
        ┌──────────────┐
 IN ───▶│ 0. Input     │  Stereo / Input1(mic) / Input2(guitar) selection
        │    routing   │  (runs BEFORE the bypass check)
        └──────┬───────┘
               ▼
        ┌──────────────┐
        │ 1. Faust EQ  │  3-band parametric EQ (if eq_active)        [FFI → C++]
        └──────┬───────┘
               ▼
        ┌──────────────┐
        │ 2. Pre-EQ    │  FFTConvolver × pre_eq_ir.wav              [Rust]
        │    convolve  │
        └──────┬───────┘
               ▼
        ┌──────────────┐
        │ 3. Mojo      │  tanh saturation drive (if neural_active)  [FFI → Mojo]
        │    neural    │
        └──────┬───────┘
               ▼
        ┌──────────────┐
        │ 4. Cabinet   │  FFTConvolver × cabinet_ir.wav            [Rust]
        │    IR        │
        └──────┬───────┘
               ▼
        ┌──────────────┐
        │ 5/6. Master  │  × gain, then × neural_amp_volume (if      [Rust]
        │  + neural vol│  neural), then NaN/Inf → 0.0 sanitization
        │  + sanitize  │  (sanitization ALWAYS runs, even in bypass)
        └──────┬───────┘
               ▼  ──▶ ring buffer ──▶ spectrum analyzer
  OUT ◀────────┘
```

Note the structure of the loop in `process()`: **input routing runs first** over
`buffer.iter_samples()` (`src/lib.rs:314`), *before* the `if !bypass` guard. The DSP
then runs over `buffer.as_slice()` (`src/lib.rs:335`), and a final loop feeds the
analyzer (`src/lib.rs:408`).

### Stage 0 — Input routing
**File:** `src/lib.rs:314-331`.
Reads L (channel 0) and R (channel 1, falling back to L if absent). Based on
`input_select`:

| Mode | Left out | Right out |
|---|---|---|
| `Stereo` | `l_in` | `r_source` |
| `Input1` (Mic) | `l_in` | `l_in` |
| `Input2` (Guitar) | `r_source` | `r_source` |

The selection is written back into the buffer before any DSP. This exists because
guitar recording often uses a single physical input, yet the plugin declares a
stereo layout.

### Stage 1 — Faust 3-band parametric EQ
**Rust:** `src/lib.rs:342-357` → **Bridge:** `src/bridge/faust.rs` → **Wrapper:** `dsp/wrapper.cpp` → **DSP:** `dsp/main.dsp` → **Filters:** `faust-ddsp/filters.lib`.
Only if `eq_active`. Per channel, the smoothed EQ parameters are pushed via
`set_eq_params(...)`, then `faust.process_block(ptr, len)` runs the cascaded
low-shelf → peak → high-shelf → `ma.tanh` filter in-place over the raw buffer
pointer. `faust_process` calls `mydsp::compute`.

The three bands (all SVF-based for stability under real-time modulation):

| Band | Filter | Default Freq | Freq Range | Gain | Q |
|---|---|---|---|---|---|
| Low | `low_shelf` | 100 Hz | 20–1000 Hz | ±12 dB | 0.707–10.0 |
| Mid | `peak_eq` | 1000 Hz | 100–10,000 Hz | ±12 dB | 0.707–10.0 |
| High | `high_shelf` | 5000 Hz | 1000–20,000 Hz | ±12 dB | 0.707–10.0 |

> **Parameter-label coupling.** The `SET_PARAM` macro keys on exact string labels
> (`"EQ Low Freq"`, `"EQ Low Gain"`, `"EQ Low Q"`, … 9 total). If a Faust slider
> label in `main.dsp` changes, the C++ wrapper must be updated or parameter
> updates silently break.

### Stage 2 — Pre-EQ convolution (LTI)
**File:** `src/lib.rs:359-365`, using `fft_convolver::FFTConvolver` + `neural/drive/pre_eq_ir.wav`.
A linear time-invariant convolution that colors the tone *before* the drive
(tone-stack / input coloration). Because the convolver cannot process in place, the
channel is copied to `temp_buffer`, convolved back into the channel, and on error
the dry copy is restored. One convolver instance per channel.

### Stage 3 — Mojo neural drive (the distortion)
**Rust:** `src/lib.rs:367-374` → **Bridge:** `src/bridge/mojo.rs:50-63` → **Mojo:** `neural/main.mojo`.
Only if `neural_amp_active`. `neural_drive` (pre-gain) and `neural_output_gain`
(makeup) are set on the processor, then `mojo.process_block(ptr, len)` applies the
clamped tanh polynomial `x*(27+x²)/(27+9x²)` sample-by-sample, in place, zero-copy
across the FFI boundary. The buffer pointer is cast to `usize` in Rust and
reconstructed as `UnsafePointer[Float32, MutAnyOrigin]` in Mojo — no allocation on
the audio thread.

### Stage 4 — Cabinet IR convolution
**File:** `src/lib.rs:377-383`, `FFTConvolver` + `neural/drive/cabinet_ir.wav`.
Speaker-cabinet simulation — the same copy-to-temp-then-convolve-with-fallback
pattern as Stage 2, placed after the drive to match real amp-chain ordering.

### Stage 5/6 — Master gain + neural master volume + NaN sanitization
**File:** `src/lib.rs:385-405`.
- Multiply by the smoothed **`gain`** master parameter (`src/lib.rs:385-388`).
- If neural is active, multiply by **`neural_amp_volume`** (`src/lib.rs:392-396`).
- **NaN/Inf sanitization runs unconditionally** — even under global `bypass` — zeroing any non-finite sample to protect downstream hardware/hosts (`src/lib.rs:399-405`).

After processing, a final loop pushes a mono mix `(L+R)*0.5` into the analyzer ring
buffer for the spectrum display (`src/lib.rs:408-415`).

### Standalone pipeline differences
`audio_worker` in `standalone.rs` runs the analogous chain
(Faust EQ → pre-EQ conv → Mojo drive → cabinet conv → sanitize) inside the CPAL
input callback, but:
- It processes explicit `buf_l`/`buf_r` vectors sized to the CPAL buffer.
- The neural stage uses `StandaloneState::neural_vol` for **both** the Mojo drive and the final gain multiply — a simpler control surface than the plugin's separate drive/makeup/volume. (Init hardcodes `set_drive(2.0)` / `set_output_gain(0.5)`.)
- The **i16 input path only sanitizes** — it does *not* run Faust/Mojo/IR processing (that lives on the f32 path).

### Parameter smoothing
All continuous `FloatParam`s carry a smoother (`SmoothingStyle::Logarithmic(50.0)`
for gains/frequencies, `Linear(50.0)` for Q). On the audio thread you **must** pull
`.smoothed.next()` (as `process()` does) and **never** read `.value()` during
processing, or you get zipper noise / clicks. Faust additionally smooths its own
slider inputs with `si.smoo`.

---

## 5. Build System

### 5.1 `build.rs` orchestration

`build.rs` (~129 lines) runs before the Rust crate compiles and performs, in order:

```
┌──────────────────────────────────────────────────────────────────┐
│                        cargo build                                │
│                             │                                     │
│                      build.rs executes:                           │
│                             ▼                                     │
│  1. pre_build_check()                                             │
│     • faust --version must exist (else panic! with install hint)  │
│     • find_mojo_path(): $PATH → ./.venv/bin → ~/.modular/.../bin  │
│     • emit rustc-link-search for <mojo>/../lib                    │
│                             ▼                                     │
│  2. rerun-if-changed: wrapper.cpp, wrapper.h, main.dsp, main.mojo │
│                             ▼                                     │
│  3. Faust transpile (incremental — only if main.dsp is newer):    │
│     faust -lang cpp -cn mydsp -vec -I faust-ddsp \                │
│           -i dsp/main.dsp -o dsp/FaustModule.hpp                  │
│                             ▼                                     │
│  4. Mojo build (incremental — only if main.mojo is newer):        │
│     mojo build --emit shared-lib neural/main.mojo \              │
│                -o neural/libneural.so                             │
│                             ▼                                     │
│  5. cc::Build — compile dsp/wrapper.cpp (-O3) → libfaust_dsp.a    │
│     (pulls in FaustModule.hpp)                                    │
│                             ▼                                     │
│  6. bindgen — dsp/wrapper.h → $OUT_DIR/bindings_faust.rs          │
│     (allow-list functions matching `faust_.*`)                   │
│                             ▼                                     │
│  7. Linker directives:                                            │
│     cargo:rustc-link-search=native=<manifest>/neural             │
│     cargo:rustc-link-lib=dylib=neural                            │
└──────────────────────────────────────────────────────────────────┘
```

### 5.2 Dependency chain

```
main.dsp ──faust──▶ FaustModule.hpp ──┐
                                       ├─(#include)─ wrapper.cpp ──cc::Build──▶ libfaust_dsp.a ──┐
wrapper.h ──bindgen──▶ bindings_faust.rs ──────────────────────(include!)──────────────────────┤
                                                                                                ├──▶ Rust crate
main.mojo ──mojo build──▶ libneural.so ──(rustc-link-lib=dylib=neural)──────────────────────────┘   (distortion
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

> **Note — the Makefile Faust command differs from `build.rs`:** the Makefile omits
> the `-vec` flag (`faust -lang cpp -cn mydsp -I faust-ddsp -i dsp/main.dsp -o
> dsp/FaustModule.hpp`), while `build.rs` includes `-vec` (vectorized codegen).

**Environment variables set by the Makefile:**
```
LIBTORCH                 ?= $(HOME)/libtorch/libtorch      # legacy — current code does not use LibTorch
LIBTORCH_BYPASS_VERSION_CHECK := 1
MOJO_HOME                ?= $(HOME)/.modular/pkg/packages.modular.com_mojo
LD_LIBRARY_PATH          := $(MOJO_HOME)/lib:$(PWD)/neural:$(LD_LIBRARY_PATH)
```

### 5.4 `cargo xtask` bundler

The `xtask` crate is a thin wrapper (`nih_plug_xtask::main()`). `cargo xtask bundle
distortion --release` reads `bundler.toml` (display name "Distortion") and produces
`.vst3` and `.clap` bundles for the host OS.

### 5.5 Release profile

```toml
[profile.release]
lto = "thin"       # thin Link-Time Optimization
strip = "symbols"  # strip debug symbols
# a separate `profiling` profile keeps debug info
```

### 5.6 Build-output locations

| Target | Output path |
|---|---|
| Standalone binary | `target/release/standalone` |
| Plugin library | `target/release/libdistortion.so` |
| VST3 bundle | `target/bundled/distortion.vst3/` |
| CLAP bundle | `target/bundled/distortion.clap/` |
| Generated Faust C++ | `dsp/FaustModule.hpp` (tracked) |
| Generated Mojo binary | `neural/libneural.so` (tracked) |
| bindgen output | `$OUT_DIR/bindings_faust.rs` (build cache) |

---

## 6. Key Design Patterns

### 6.1 Zero-copy FFI (Rust ↔ Mojo — the "Address Bypass" pattern)

No audio memory is allocated or copied at the language boundary. The Rust side
casts the raw buffer pointer to an integer address:

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

Type flow: `*mut f32 → usize → Int → UnsafePointer[Float32, MutAnyOrigin]`.

**Contract:** the Rust buffer must outlive the call; Mojo must not store the
pointer, must process only `0..size`, and must treat data as `Float32`.

### 6.2 The Mojo `@export` address-bypass constraint

Mojo's `@export` functions **cannot accept `UnsafePointer` parameters directly**
(parametricity restriction). The project's rules:

- ✅ Pass the pointer as an `Int` address; reconstruct `UnsafePointer(unsafe_from_address=…)` inside.
- ✅ Always use `@export` (**not** `@extern_c`) to emit FFI-visible symbols.
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
slider's `float*` **zone** into a `std::map<label, float*>`. Setters
(`faust_set_eq_low_freq`, …) write straight into those zones via the `SET_PARAM`
macro — no per-sample string lookup on the audio thread. The wrapper passes the same
channel buffer as **both** Faust input channels, so each per-channel slice runs the
stereo Faust graph against duplicated mono input (dual-mono processing).

### 6.5 Parameter smoothing (nih-plug)

See §4. Logarithmic smoothing for gains/frequencies, Linear for Q; always pull
`.smoothed.next()` on the audio thread, never `.value()`.

### 6.6 Lock-free audio→UI handoff (ring buffers)

The audio thread never blocks the UI. Processed samples are pushed into an `rtrb`
single-producer/single-consumer ring buffer (64K). The UI thread `try_lock`s the
consumer and drains it into `AnalyzerDsp` for the FFT/spectrum. The standalone uses
a second ring buffer to pass processed audio from the input callback to the output
callback. The analyzer is visualization-only — it never feeds audio back.

### 6.7 Convolver fallback

Each convolution stage copies the block to `temp_buffer`, processes temp→channel,
and if `FFTConvolver::process` errors, restores the dry copy — so a convolution
failure never leaves silence or partially modified output.

### 6.8 Per-channel processor instances

Faust and Mojo processors are stored as `[Option<_>; 2]` (one per stereo channel) so
left/right filter state stays independent. `channel_idx.min(1)` guards mono/×N
layouts against out-of-bounds processor access.

### 6.9 Dual UI composition

`render_shared_panels()` (plus `draw_signal_chain`, `draw_spectrum`, `draw_eq_band`)
is used by both the plugin (`nih_plug_egui`) and the standalone (`eframe`) shells.
It receives closures for the controls, so the spectrum and signal-chain UI are not
duplicated across targets.

---

## 7. Dependencies

### 7.1 Rust runtime crates (`Cargo.toml`)

| Crate | Version | Purpose |
|---|---|---|
| `nih_plug` | git (robbert-vdh) | Plugin framework; features `assert_process_allocs`, `standalone` |
| `nih_plug_egui` | git | egui editor integration for the plugin UI |
| `nih_plug_vizia` | git | (declared) alternative Vizia UI backend — not on active path |
| `eframe` | 0.31.0 | Standalone native egui app shell |
| `egui_knob` | 0.2.0 | Rotary knob widget used across both UIs |
| `cpal` | 0.15.2 | Cross-platform audio I/O (standalone) |
| `fft-convolver` | 0.3.0 | Partitioned FFT convolution (pre-EQ + cabinet IRs) |
| `realfft` | 3.5.0 | Real-input FFT for the spectrum analyzer |
| `num-complex` | 0.4.6 | Complex numbers for FFT output |
| `hound` | 3.5.1 | WAV reading for impulse responses |
| `rtrb` | 0.3.3 | Lock-free SPSC ring buffer (audio → analyzer) |
| `ringbuf` | 0.4.8 | Additional ring-buffer utilities (standalone pass-through) |
| `arc-swap` | 1.7.1 | (declared) atomic Arc swapping — not on active path |
| `biquad` | 0.5.0 | (declared) biquad filters — not on active path |
| `rubato` | 1.0.1 | (declared) sample-rate conversion — not on active path |
| `nalgebra` | 0.34.1 | (declared) linear algebra — not on active path |
| `fast-math` | 0.1.1 | (declared) fast approximate math — not on active path |
| `rfd` | 0.14.1 | (declared) native file dialogs — not on active path |

> Several declared crates (`nih_plug_vizia`, `biquad`, `rubato`, `nalgebra`,
> `arc-swap`, `fast-math`, `rfd`) are template residue and are not obviously used on
> the current audio path.

### 7.2 Rust build dependencies

| Crate | Version | Purpose |
|---|---|---|
| `cc` | 1.0 | Compile `dsp/wrapper.cpp` → `libfaust_dsp.a` |
| `bindgen` | 0.71.1 | Generate Rust FFI from `dsp/wrapper.h` → `bindings_faust.rs` |

Workspace member `xtask` depends on `nih_plug_xtask` (git).

### 7.3 System dependencies

| Tool | Needed for | Install |
|---|---|---|
| Rust + Cargo | Build everything | `rustup` |
| **Faust** | Transpile `main.dsp` → C++ | `sudo apt install faust` / `brew install faust` |
| **Mojo** (Modular MAX SDK) | Build `libneural.so` | `modular install mojo` |
| **Clang / LLVM** | `bindgen` needs libclang | `sudo apt install clang libclang-dev` |
| **ALSA dev libs** | CPAL backend (Linux) | `sudo apt install libasound2-dev` |
| Python ≥3.10 | Mojo venv support | system / pyenv |
| (optional) LibTorch | Legacy neural-amp path (unused) | env `LIBTORCH` in Makefile |

### 7.4 Bundled assets

| Asset | Role |
|---|---|
| `neural/drive/pre_eq_ir.wav` | Pre-EQ / tone-stack impulse response (Stage 2) |
| `neural/drive/cabinet_ir.wav` | Speaker cabinet IR — mono, 44.1 kHz, 16-bit PCM (Stage 4) |
| `neural/drive/wavenet_drive.onnx` (+ `.data`) | WaveNet drive model asset — present but **not executed** |
| `src/models/modelo_amp.onnx`, `…_lstm_…onnx.data` | Legacy/auxiliary amp models — not on the live Mojo path |
| `faust-ddsp/filters.lib`, `diff.lib` | Faust libraries on the compiler `-I` include path |

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

**Golden rule (from the README):** validate every audio-processing change in
**Standalone** (`make run`) before generating the DAW bundle.

**Runtime note:** `libneural.so` must be on the library path at runtime. The
Makefile sets `LD_LIBRARY_PATH` to include `$(PWD)/neural` and the Mojo `lib`
directory; if you run binaries directly, replicate that.

**Mojo in a local venv:** if Mojo lives in `./.venv`, activate it
(`source .venv/bin/activate`) before building, or rely on the automatic `.venv`
detection in `build.rs` and the Makefile.

**Expected new-developer path:** `make check-env` → `make pre-build` → `make run` →
`make bundle`.

---

## 9. Fast Reference

### 9.1 Parameter IDs

| ID | Type | Default | Range | Unit |
|---|---|---|---|---|
| `input` | EnumParam | Stereo | Stereo / Input1 / Input2 | — |
| `gain` | FloatParam | 0 dB | -30 to +30 dB | dB |
| `bypass` | BoolParam | false | — | — |
| `neural_amp_volume` | FloatParam | 0 dB | -24 to +12 dB | dB |
| `neural_drive` | FloatParam | 0 dB | 0 to +30 dB | dB |
| `neural_output_gain` | FloatParam | 0 dB | -24 to +12 dB | dB |
| `neural_amp_active` | BoolParam | true | — | — |
| `eq_active` | BoolParam | true | — | — |
| `eq_low_freq` | FloatParam (Skewed) | 100 Hz | 20–1000 Hz | Hz |
| `eq_low_gain` | FloatParam | 0 dB | -12 to +12 dB | dB |
| `eq_low_q` | FloatParam | 0.707 | 0.707–10.0 | — |
| `eq_mid_freq` | FloatParam (Skewed) | 1000 Hz | 100–10,000 Hz | Hz |
| `eq_mid_gain` | FloatParam | 0 dB | -12 to +12 dB | dB |
| `eq_mid_q` | FloatParam | 0.707 | 0.707–10.0 | — |
| `eq_high_freq` | FloatParam (Skewed) | 5000 Hz | 1000–20,000 Hz | Hz |
| `eq_high_gain` | FloatParam | 0 dB | -12 to +12 dB | dB |
| `eq_high_q` | FloatParam | 0.707 | 0.707–10.0 | — |

### 9.2 Key constants

| Constant | Value | Location |
|---|---|---|
| `APP_NAME` | `"Distortion"` | `src/lib.rs:18` |
| `APP_ID` | `"distortion"` | `src/lib.rs:19` |
| `VENDOR` | `""` (empty) | `src/lib.rs:20` |
| `CLAP_ID` | `".distortion"` | `src/lib.rs:22` |
| `VST3_ID` | `A9ED13810DBC4A4A98540F0F671E9E1A` | `src/lib.rs:23` |
| `FFT_SIZE` | `2048` | `src/core/dsp/analyzer.rs:6` |
| Ring-buffer capacity | `1024 * 64` = 65,536 | `src/lib.rs` (`BaseIO::default`) |
| Plugin editor size | `800 × 450` | `src/core/state/plugin_params.rs:80` |
| Standalone window | `1000 × 500` | `src/bin/standalone.rs` (`main`) |
| Smoothing time | `50.0` ms | all `FloatParam`s |
| EQ chain ordering | `eq_low : eq_mid : eq_high : ma.tanh` | `dsp/main.dsp` |
| Tanh clamp | `[-4.0, 4.0]` | `neural/main.mojo` |
| Spectrum EMA | `prev*0.85 + db*0.15` | `src/core/dsp/analyzer.rs:69` |
| Analyzer display rate | `48000.0` Hz (fixed) | `src/core/dsp/analyzer.rs:55` |
| Latency | `0` samples | `src/lib.rs:311` |

### 9.3 Quick file finder

| If you need to… | Look at… |
|---|---|
| Change the plugin audio pipeline | `src/lib.rs` → `BaseIO::process()` |
| Change the standalone audio pipeline | `src/bin/standalone.rs` → `audio_worker()` |
| Add / rename a parameter | `src/core/state/plugin_params.rs` → `BaseIOParams` |
| Modify the EQ DSP | `dsp/main.dsp` (then `make build-faust`) |
| Change the neural saturation algorithm | `neural/main.mojo` (then `make build-mojo`) |
| Expose new Faust params to Rust | `dsp/wrapper.h` + `dsp/wrapper.cpp` + `src/bridge/faust.rs` |
| Change the Faust ↔ Mojo bridge | `src/bridge/faust.rs` / `src/bridge/mojo.rs` |
| Change the UI layout | `src/core/ui/main_view.rs` |
| Modify the spectrum analyzer | `src/core/dsp/analyzer.rs` + `src/core/ui/spectrum.rs` |
| Add a new build step | `build.rs` |
| Change build commands | `Makefile` |
| Change bundle metadata | `bundler.toml` |

### 9.4 Onboarding read order

1. `src/lib.rs` — plugin processing & lifecycle.
2. `src/core/state/plugin_params.rs` — the parameters.
3. `src/bridge/{mod,faust,mojo}.rs` — the FFI seams.
4. `dsp/main.dsp` + `faust-ddsp/filters.lib` — the EQ math.
5. `neural/main.mojo` — the drive math.
6. `src/bin/standalone.rs` — the standalone host.
7. `build.rs` + `Makefile` — the build.

### 9.5 Architecture decision record

| Decision | Rationale |
|---|---|
| Mojo replaces `tract-onnx` for the drive | Synchronous zero-copy ≈ zero latency; no background inference thread |
| Faust SVF filters over biquad | State-variable filters stay stable under real-time modulation (no coefficient-recalc artifacts) |
| Dual-mono (not true stereo) processing | Simpler; each channel runs an identical independent chain |
| `FFTConvolver` over direct convolution | O(n log n), partitioned, low-latency for long IRs |
| egui/eframe over vizia for standalone | Simpler API; shares UI code with the `nih_plug_egui` plugin |
| Address-Bypass pattern for Mojo FFI | Required by Mojo `@export` parametricity constraints |

---

## 10. Known Issues & Rough Edges

Worth knowing before you ship or refactor:

1. **Hardcoded absolute IR paths.** Both `src/lib.rs::initialize()` (`src/lib.rs:254`)
   and `src/bin/standalone.rs` load IRs from
   `"/home/jao/VSCode/distortion/meu-novo-plugin/neural/drive/"`. This breaks on any
   other machine, in sibling worktrees, and in installed plugin bundles. Make it
   relative (`CARGO_MANIFEST_DIR`) or embed the assets (`include_bytes!`). **This is
   the single most important portability issue.**

2. **Two independent pipelines.** Plugin (`lib.rs`) and standalone (`standalone.rs`)
   each implement the DSP chain separately, with different neural controls (the
   standalone collapses drive/makeup/volume into one `neural_vol`). Keep them in sync
   when changing DSP.

3. **`mojo_init` signature mismatch.** Rust declares `mojo_init(sample_rate: f32)`
   (`src/bridge/mojo.rs`), but the Mojo definition is
   `mojo_init(sample_rate: Float64)` (`neural/main.mojo`). It is currently harmless
   because `mojo_init` is a **stub** (sample rate is accepted but unused; the
   saturation is sample-rate-independent), but the ABI types disagree and should be
   reconciled before `mojo_init` does real work.

4. **Standalone i16 input path** only sanitizes — it skips Faust/Mojo/IR processing
   (those live on the f32 path).

5. **Faust wrapper is dual-mono.** Each per-channel slice is fed as both Faust input
   channels, so "stereo" processing is really two independent mono chains.

6. **Parameter-label coupling.** The C++ `SET_PARAM` macro keys on exact Faust
   slider labels (`"EQ Low Freq"`, …). Renaming a slider in `main.dsp` silently
   breaks parameter updates unless `wrapper.cpp` is updated too.

7. **Generated/build artifacts tracked in git.** `dsp/FaustModule.hpp` and
   `neural/libneural.so` are checked in (useful when Faust/Mojo are hard to install,
   but `libneural.so` is platform-specific). A stray **`wrapper.o`** is also tracked
   even though Cargo compiles the wrapper itself — it looks like an accidental
   commit. Decide and document the artifact-tracking policy.

8. **ONNX asset drift.** `neural/drive/wavenet_drive.onnx(.data)` and
   `src/models/*.onnx` are bundled but **not executed**; the live drive is the Mojo
   polynomial. `README.md` still references older `tract-onnx`/ONNX behavior and an
   older Mojo FFI signature. Treat the code as truth.

9. **Template residue.** `nih_plug_vizia`, `biquad`, `rubato`, `nalgebra`,
   `arc-swap`, `fast-math`, and `rfd` are declared but not on the active path. The
   `LIBTORCH*` env vars remain in the `Makefile` although no Rust code uses LibTorch.
   `pyproject.toml` (`quickstart`) is unrelated to the Rust build.

10. **Fixed 48 kHz analyzer display rate.** `AnalyzerDsp` / `draw_spectrum` assume a
    48 kHz display sample rate for the frequency axis regardless of the actual host
    rate — spectrum frequency labels can be off at other sample rates.

11. **Metadata polish.** `VENDOR` is empty; `CLAP_ID` is `.distortion` (may not be
    globally unique); `bundler.toml` uses `[baseIO]` while the package name is
    `distortion`; the `Cargo.toml` description is generic; some UI labels still say
    `BaseIO`/`PyTorch` while the product is `Distortion` on a Mojo drive.

12. **No test suite.** There are no `#[test]` functions; validation is manual via the
    standalone host and the spectrum analyzer.

---

*Definitive combined map of the Distortion plugin codebase, merged from three
independent agent mappings (pi, claude_code, codex) and verified against the source
tree. For the build/FFI/troubleshooting narrative (Portuguese), see `README.md`; for
repo-working guidance, see `CLAUDE.md`.*
