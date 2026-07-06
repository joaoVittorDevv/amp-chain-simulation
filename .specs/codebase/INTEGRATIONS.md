# External Integrations

**Analyzed:** 2026-07-06

## Faust DSP Compiler

**Purpose:** Compile `.dsp` signal processing graphs to C++ for real-time audio

**Location:**
- Source: `dsp/main.dsp`
- Build: `build.rs:31-77`
- Wrapper: `dsp/wrapper.cpp`, `dsp/wrapper.h`
- Bridge: `src/bridge/faust.rs`
- Includes path: `faust-ddsp/` (diff.lib, filters.lib)

**Build command:**
```bash
faust -lang cpp -cn mydsp -vec -I faust-ddsp -i dsp/main.dsp -o dsp/FaustModule.hpp
```

**Configuration:** `build.rs` checks `faust --version` and panics if missing. Rebuilds when `dsp/main.dsp` is newer than output.

## Mojo Compiler

**Purpose:** Compile `.mojo` neural processing to native shared library

**Location:**
- Source: `neural/main.mojo`
- Build: `build.rs:83-101`
- Bridge: `src/bridge/mojo.rs`
- Output: `neural/libneural.so`

**Build command:**
```bash
mojo build --emit shared-lib neural/main.mojo -o neural/libneural.so
```

**Configuration:** `build.rs` locates `mojo` in PATH, `.venv/bin/mojo`, or Modular install paths. Links `dylib=neural`.

## FFTConvolver

**Purpose:** Fast convolution for pre-EQ and cabinet impulse responses

**Usage:**
- Plugin pre-EQ: Two `FFTConvolver<f32>` initialized from embedded `DEFAULT_PRE_EQ_IR` [src/lib.rs:487]
- Standalone pre-EQ: Local `pre_eq_l`/`pre_eq_r` instances [src/bin/standalone.rs:406]
- Cabinet runtime: Left/right convolution engines from WAV IR data [src/core/cabinet/runtime.rs:47]

**Error handling:** Falls back to dry passthrough if `process()` returns error.

## nih_plug Framework

**Purpose:** VST3/CLAP audio plugin framework

**Integration points:**
- `Plugin` trait impl [src/lib.rs:118]
- `ClapPlugin` + `Vst3Plugin` impl [src/lib.rs:706, 714]
- `Params` derive macro on `BaseIOParams`
- `FloatParam`, `EnumParam`, `BoolParam` types
- `nih_plug_egui` for plugin editor
- Sample-accurate automation enabled [src/lib.rs:135]
- Stereo I/O enforced [src/lib.rs:125]

## CPAL Audio I/O

**Purpose:** Cross-platform audio I/O for standalone target

**Usage:**
- Host/device enumeration [src/bin/standalone.rs:300]
- Input stream: F32 only (rejects non-F32 with error)
- Output stream: F32 + I16 paths
- Channel selection UI with ComboBox widgets
- Ring buffer bridge between input and output streams

## egui / eframe UI

**Purpose:** Immediate-mode GUI for plugin editor and standalone app

**Plugin editor:** `nih_plug_egui::create_egui_editor` [src/lib.rs:147]
**Standalone:** `eframe::run_native` [src/bin/standalone.rs:1418]

**Shared panels:** Closure-injected so plugin and standalone provide their own parameter mutation logic. Panels: `main_view`, `signal_chain`, `spectrum`, `cabinet_panel`.

**Widget:** `egui_knob` crate for knob controls.

## bindgen FFI Generation

**Purpose:** Auto-generate Rust FFI bindings from C headers

**Usage:** `build.rs:112-120`
```rust
bindgen::Builder::default()
    .header("dsp/wrapper.h")
    .allowlist_function("faust_.*")
    .generate()
    .write_to_file(out_path.join("bindings_faust.rs"))
```

**Inclusion:** `include!(concat!(env!("OUT_DIR"), "/bindings_faust.rs"))` [src/bridge/faust.rs:6]

## Cabinet IR System

**Purpose:** SQLite-backed impulse response library with FFT convolution

**Components:**
- **Library** (`src/core/cabinet/library.rs`): SQLite storage, import/export, content hashing (blake3), WAV decoding (hound)
- **Runtime** (`src/core/cabinet/runtime.rs`): `FFTConvolver` instances, IR resampling (rubato)
- **Engine** (`src/core/cabinet/engine.rs`): Bypass, level, mix, mute ramping, fade

## WaveNet ONNX (Legacy/Unused)

**Purpose:** Neural amp modeling via ONNX inference

**Location:** `src/bridge/wavenet.rs`, `neural/drive/wavenet_drive.onnx`

**Status:** Not wired into `BaseIO::process()`. `tract-onnx` dependency remains but active neural path uses Mojo tanh.
