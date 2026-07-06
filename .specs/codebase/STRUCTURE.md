# Project Structure

**Root:** `/home/jao/VSCode/distortion/meu-novo-plugin`

## Directory Tree (3 Levels)

```
.
├── Cargo.toml              # Rust package manifest
├── Cargo.lock
├── Makefile                # Build/run/bundle/clean targets
├── build.rs                # Faust/Mojo compilation + bindgen
├── bundler.toml            # nih_plug bundler config
├── dsp/                    # Faust DSP sources
│   ├── main.dsp            # 3-band parametric EQ source
│   ├── wrapper.cpp          # C ABI wrapper for Faust
│   ├── wrapper.h            # C ABI header
│   └── FaustModule.hpp      # Generated Faust C++ output
├── neural/                 # Mojo neural sources
│   ├── main.mojo            # Tanh saturation processor
│   ├── libneural.so         # Built Mojo shared library
│   └── drive/               # Neural model assets
│       ├── cabinet_ir.wav
│       ├── pre_eq_ir.wav
│       ├── wavenet_drive.onnx
│       └── wavenet_drive.onnx.data
├── faust-ddsp/             # Faust DSP library includes
│   ├── diff.lib
│   └── filters.lib
├── src/
│   ├── lib.rs              # Plugin entry: BaseIO, DSP chain, exports
│   ├── bin/
│   │   └── standalone.rs   # Standalone app: CPAL + eframee
│   ├── bridge/             # External processor bridge layer
│   │   ├── mod.rs           # ExternalProcessor trait
│   │   ├── faust.rs         # Faust C ABI wrapper
│   │   ├── mojo.rs          # Mojo FFI wrapper
│   │   └── wavenet.rs       # ONNX WaveNet (legacy/unused)
│   └── core/               # Shared core modules
│       ├── cabinet/         # Cabinet IR system (engine, library, runtime)
│       ├── dsp/             # DSP utilities (analyzer)
│       ├── state/           # Plugin params + editor state
│       └── ui/              # Shared egui panels
├── scripts/                # Build/run helper scripts
│   ├── check_env.sh
│   └── run_standalone.sh
├── xtask/                  # nih_plug bundling xtask
│   ├── Cargo.toml
│   └── src/
├── docs/                   # Architecture plans + specs
└── target/                 # Build artifacts (~1.9 GB)
```

## Module Organization

### `src/lib.rs` — Plugin Target
- **Purpose:** Plugin entry point for DAW hosts
- **Key items:** `BaseIO` struct, `Plugin` trait impl, DSP chain, CLAP/VST3 exports

### `src/bin/standalone.rs` — Standalone Target
- **Purpose:** Self-contained app with CPAL audio I/O
- **Key items:** `StandaloneApp`, `StandaloneState`, `AudioSnapshot`, audio worker thread

### `src/bridge/` — External Processor Bridge
- **Purpose:** Common trait and external FFI wrappers
- **Key items:** `ExternalProcessor` trait, `FaustProcessor`, `MojoProcessor`, `WavenetProcessor`

### `src/core/state/` — Parameter Management
- **Purpose:** NIH parameter definitions and UI state
- **Key files:** `plugin_params.rs`

### `src/core/dsp/` — DSP Utilities
- **Purpose:** Shared DSP processing (analyzer)
- **Key files:** `analyzer.rs`

### `src/core/ui/` — Shared UI Panelss
- **Purpose:** egui panels used by both plugin and standalone
- **Key files:** `main_view.rs`, `signal_chain.rs`, `spectrum.rs`, `cabinet_panel.rs`

### `src/core/cabinet/` — Cabinet IR System
- **Purpose:** IR library, runtime convolution, engine management
- **Key files:** `engine.rs`, `library.rs`, `runtime.rs`

## Where Things Live

| Capability | Location |
|------------|----------|
| Plugin DSP chain | `src/lib.rs` |
| Standalone DSP chain | `src/bin/standalone.rs` |
| Parameter definitions | `src/core/state/plugin_params.rs` |
| Faust DSP source | `dsp/main.dsp` |
| Faust C ABI | `dsp/wrapper.cpp`, `dsp/wrapper.h` |
| Faust Rust bridge | `src/bridge/faust.rs` |
| Mojo DSP source | `neural/main.mojo` |
| Mojo Rust bridge | `src/bridge/mojo.rs` |
| FFT Analyzer | `src/core/dsp/analyzer.rs` |
| UI panels | `src/core/ui/*.rs` |
| Cabinet IR | `src/core/cabinet/*.rs` |
| Build orchestration | `build.rs`, `Makefile` |

## Special Directories

**`dsp/`:**
- **Purpose:** Faust DSP sources and C ABI wrapper
- **Examples:** `main.dsp`, `wrapper.cpp`, `wrapper.h`

**`neural/`:**
- **Purpose:** Mojo neural processing sources and model assets
- **Examples:** `main.mojo`, `libneural.so`, `drive/*.wav`, `drive/*.onnx`

**`faust-ddsp/`:**
- **Purpose:** Faust DSP library includes on the `-I` path
- **Examples:** `diff.lib`, `filters.lib`
