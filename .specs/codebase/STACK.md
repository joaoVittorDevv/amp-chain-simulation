# Tech Stack

**Analyzed:** 2026-07-06

## Core

- **Language:** Rust (edition 2021)
- **Package:** `distortion` v0.1.0
- **Build system:** Cargo + Makefile
- **Targets:** cdylib (VST3/CLAP) + lib + standalone binary

## DSP Backends

| Backend | Language | Compiler | Output | Bridge |
|---------|----------|----------|--------|--------|
| Faust EQ | Faust DSP → C++ | `faust` CLI | `FaustModule.hpp` | `cc::Build` + `bindgen` → Rust |
| Mojo Neural | Mojo | `mojo` CLI | `libneural.so` | Zero-copy FFI (`*mut f32` → `usize`) |

## Rust Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `nih_plug` | git (robbert-vdh) | VST3/CLAP framework, params, exports |
| `nih_plug_egui` | git | Plugin egui editor integration |
| `cpal` | 0.15.2 | Standalone audio I/O |
| `eframe` | 0.31.0 | Standalone GUI shell |
| `fft-convolver` | 0.3.0 | Pre-EQ + Cabinet IR convolution |
| `realfft` | 3.5.0 | FFT spectrum analyzer |
| `num-complex` | 0.4.6 | Complex number math (analyzer) |
| `rtrb` | 0.3.3 | Audio ↔ UI ring buffers |
| `hound` | 3.5.1 | WAV decoding (IRs) |
| `rfd` | 0.14.1 | Native file dialogs (IR import/export) |
| `dirs` | 5 | User data directories (cabinet library) |
| `rusqlite` | 0.31 (bundled) | Cabinet IR library database |
| `blake3` | 1 | IR content hashing |
| `rubato` | 1.0.1 | IR resampling |
| `egui_knob` | 0.2.0 | Knob UI widgets |
| `serde` / `serde_json` | 1 | Config serialization |
| `thiserror` | 1 | Error derive macros |
| `nih_plug_vizia` | git | **Unused** — compiled but no source usage |
| `ringbuf` | 0.4.8 | **Unused** — compiled but no source usage |
| `arc-swap` | 1.7.1 | Lock-free cabinet runtime handoff |
| `audioadapter-buffers` | 2.0.0 | Rubato resampling buffer adapter |

## Build Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `cc` | 1.0 | Compile `dsp/wrapper.cpp` |
| `bindgen` | 0.71.1 | Generate Rust FFI bindings for Faust C ABI |

## External Toolchain

| Tool | Required | Purpose |
|------|----------|---------|
| `faust` | Mandatory | Compile `.dsp` to C++ |
| `mojo` | Mandatory | Compile `.mojo` to shared library |

## Development Tools

- **Building:** `cargo build --release`
- **Running:** `make run` (standalone dev mode)
- **Bundling:** `make bundle` (VST3/CLAP distribution)
- **Pre-build:** `make pre-build` (Faust + Mojo compilation)
- **Cleaning:** `make clean` (removes target/ + generated artifacts)
