# Project Map

**Repository:** `meu-novo-plugin`

**Analyzed:** 2026-07-03

**Primary purpose:** Real-time guitar distortion / amp-chain audio plugin.

**Main deliverables:** VST3 plugin, CLAP plugin, and standalone desktop test host.

**Primary languages:** Rust, Faust, Mojo, C/C++ FFI glue.

**Primary plugin framework:** `nih-plug`.

---

## 1. Project Overview

This project is a real-time audio effect for guitar distortion and amp-chain processing.

The codebase combines host/plugin infrastructure, DSP filters, neural-style nonlinear drive, convolution-based impulse responses, and a spectrum-analyzer UI.

The runtime is centered around a Rust plugin named `distortion`.

The exported plugin type is `BaseIO`.

The crate builds as both:

- `cdylib` for plugin formats.
- `lib` for reuse by the standalone binary.

The project also includes a standalone binary named `standalone`.

The standalone binary is useful for development because it avoids needing a DAW to test audio routing, GUI behavior, and DSP behavior.

The sound-processing concept is an amp-chain style path:

- Select or duplicate input channels.
- Apply a Faust parametric EQ.
- Apply a pre-EQ impulse response.
- Apply a Mojo nonlinear neural-drive stage.
- Apply a cabinet impulse response.
- Apply output gain and sanitize invalid samples.

The implementation currently uses a Mojo soft-saturation processor.

The repository also contains ONNX neural model assets, but the current Rust processing path does not run ONNX inference through a Rust ONNX runtime.

The Rust source explicitly notes that previous `tract-onnx` background inference has been replaced by synchronous Mojo FFI.

Key technologies:

- Rust 2021 for plugin orchestration, state, UI, CPAL standalone audio, build orchestration, and FFI wrappers.
- `nih-plug` for VST3/CLAP plugin lifecycle, parameters, sample-accurate automation, and bundle tooling.
- `nih_plug_egui` and `eframe`/`egui` for plugin and standalone UI.
- Faust for declarative DSP and generated C++ EQ code.
- Local `faust-ddsp` libraries for stable state-variable filters and differentiable DSP primitives.
- Mojo for exported native neural/nonlinear processing functions.
- C/C++ wrapper code for exposing Faust-generated C++ to Rust through a C ABI.
- `bindgen` for generating Rust declarations from the C header.
- `cc` for compiling the Faust wrapper during Cargo builds.
- `fft-convolver` for pre-EQ and cabinet impulse-response convolution.
- `hound` for loading WAV impulse responses.
- `rtrb` / `ringbuf` for real-time-safe analyzer sample transfer.
- `realfft` and `num-complex` for spectrum analysis.
- `cpal` for standalone audio I/O.

The plugin metadata currently says:

- Crate name: `distortion`.
- Crate version: `0.1.0`.
- Plugin name: `Distortion`.
- App ID: `distortion`.
- License: `GPL-3.0-or-later`.
- Author: `jao <joaovittorh1@gmail.com>`.

Important onboarding note:

Some README snippets are older than the current code.

The current Mojo FFI function has the signature:

```text
mojo_process_block(address, size, drive, output_gain)
```

The current Rust dependency list does not include `tract-onnx`.

Treat source files and manifests as the source of truth when this map disagrees with older prose documentation.

---

## 2. Architecture

### High-Level Architecture

The architecture is a three-language audio engine with a Rust host/orchestrator.

Rust owns the plugin lifecycle.

Rust owns the standalone host lifecycle.

Rust owns plugin parameters.

Rust owns UI state.

Rust owns analyzer data transfer.

Rust owns convolution stages.

Rust calls Faust for the parametric EQ through generated C++ and a C wrapper.

Rust calls Mojo for the nonlinear drive through a dynamic shared library.

Faust owns the mathematical definition of the 3-band EQ.

Mojo owns the current zero-copy saturation implementation.

C/C++ owns the ABI boundary for Faust.

`build.rs` coordinates all generated/native pieces during Cargo builds.

### Architecture Diagram

```text
                         DAW Host
                            |
                            | VST3 / CLAP
                            v
                 +------------------------+
                 | Rust nih-plug BaseIO   |
                 | src/lib.rs             |
                 +------------------------+
                            |
                            | shared Rust modules
                            v
          +------------------------------------------+
          | state + UI + analyzer + bridge traits    |
          | src/core/* and src/bridge/mod.rs         |
          +------------------------------------------+
              |                    |              |
              | C ABI              | C ABI        | Rust APIs
              v                    v              v
   +---------------------+  +----------------+  +----------------------+
   | Faust C++ wrapper   |  | Mojo .so       |  | Convolution/analyzer |
   | dsp/wrapper.cpp     |  | neural/main.mojo| | fft-convolver/realfft|
   +---------------------+  +----------------+  +----------------------+
              |
              | includes generated C++
              v
   +---------------------+
   | dsp/FaustModule.hpp |
   | generated from DSP  |
   +---------------------+
```

Standalone mode uses the same Rust library and bridge modules but replaces the DAW host with `cpal` and `eframe`.

```text
                    OS audio devices
                          |
                          | CPAL input/output streams
                          v
            +------------------------------+
            | standalone binary            |
            | src/bin/standalone.rs        |
            +------------------------------+
                          |
                          | uses distortion crate
                          v
            +------------------------------+
            | Rust bridge + Faust + Mojo   |
            | same core processing pieces  |
            +------------------------------+
                          |
                          v
                    eframe/egui UI
```

### Three-Language Stack

| Layer | Language | Main files | Runtime role |
|---|---|---|---|
| Host/orchestrator | Rust | `src/lib.rs`, `src/bin/standalone.rs` | Plugin lifecycle, standalone lifecycle, parameters, UI, buffer routing, analyzer, convolution, FFI calls |
| DSP/EQ | Faust plus generated C++ | `dsp/main.dsp`, `dsp/FaustModule.hpp`, `dsp/wrapper.cpp`, `dsp/wrapper.h` | 3-band parametric EQ with stable filter implementations |
| Neural/nonlinear | Mojo | `neural/main.mojo`, `neural/libneural.so` | Zero-copy in-place soft saturation with drive and output gain |

### Rust Host Layer

The Rust host layer is implemented in `src/lib.rs`.

The primary struct is `BaseIO`.

`BaseIO` owns the shared plugin parameters.

`BaseIO` owns analyzer ring-buffer endpoints.

`BaseIO` owns per-channel Faust processors.

`BaseIO` owns per-channel Mojo processors.

`BaseIO` owns per-channel pre-EQ convolvers.

`BaseIO` owns per-channel cabinet convolvers.

`BaseIO` owns a reusable temporary audio buffer.

`BaseIO` implements `nih_plug::prelude::Plugin`.

`BaseIO` implements `ClapPlugin`.

`BaseIO` implements `Vst3Plugin`.

`nih_export_clap!(BaseIO)` exports the CLAP entry point.

`nih_export_vst3!(BaseIO)` exports the VST3 entry point.

### Faust DSP Layer

The Faust source is `dsp/main.dsp`.

The Faust DSP defines:

- Low band frequency, gain, and Q sliders.
- Mid band frequency, gain, and Q sliders.
- High band frequency, gain, and Q sliders.
- A low-shelf filter.
- A peak EQ filter.
- A high-shelf filter.
- A final `ma.tanh` soft clip in the Faust EQ chain.
- Stereo orchestration with `process = _,_ : (eq_chain, eq_chain);`.

The Faust file imports:

- `stdfaust.lib`.
- Local `diff.lib` through `library("diff.lib")`.
- Local `filters.lib` through `library("filters.lib")`.

The generated C++ file is `dsp/FaustModule.hpp`.

That generated file declares the generated DSP class as `mydsp`.

The generated file currently reports Faust `2.85.1` in its header.

The wrapper includes this generated header and adapts it to a C ABI.

### Mojo Neural Layer

The Mojo source is `neural/main.mojo`.

It exports two symbols:

- `mojo_init(sample_rate: Float64)`.
- `mojo_process_block(address: Int, size: Int, drive: Float32, output_gain: Float32)`.

`mojo_init` is currently a placeholder.

`mojo_process_block` reconstructs an unsafe mutable pointer from an integer address.

It processes the Rust buffer in-place.

It applies drive before the nonlinearity.

It clamps the driven signal to `[-4.0, 4.0]`.

It applies a polynomial approximation of `tanh`.

It applies output gain after saturation.

### FFI Interoperation

Rust talks to Faust through `src/bridge/faust.rs`.

`src/bridge/faust.rs` includes generated `bindgen` output from Cargo `OUT_DIR`.

`build.rs` generates this file from `dsp/wrapper.h`.

The wrapper functions exposed to Rust are allowlisted with the prefix `faust_`.

Rust talks to Mojo through `src/bridge/mojo.rs`.

`src/bridge/mojo.rs` declares an `extern "C"` block for the Mojo symbols.

The Mojo library is linked dynamically as `neural`.

`build.rs` emits a native link search path for the repository `neural` directory.

`build.rs` emits `cargo:rustc-link-lib=dylib=neural`.

### Bridge Module Pattern

The bridge pattern is centered on `src/bridge/mod.rs`.

That module defines:

```rust
pub trait ExternalProcessor {
    fn init(&mut self, sample_rate: f32);
    fn process_block(&mut self, buffer: *mut f32, length: usize);
}
```

Both external processors implement this trait.

`FaustProcessor` implements `ExternalProcessor`.

`MojoProcessor` implements `ExternalProcessor`.

The trait keeps the audio path generic at the call site.

The trait also makes a strict zero-copy contract explicit:

- The caller passes a raw mutable pointer.
- The caller passes the number of samples.
- The callee mutates the same memory in-place.
- No owned audio buffer crosses the FFI boundary.

### Dual-Target Architecture

The project has two runtime targets.

Plugin mode:

- Entry file: `src/lib.rs`.
- Host: DAW through VST3 or CLAP.
- Plugin framework: `nih-plug`.
- Audio callback: `Plugin::process`.
- Parameters: `BaseIOParams` through NIH `Params`.
- Editor: `nih_plug_egui::create_egui_editor`.
- Export macros: `nih_export_clap!` and `nih_export_vst3!`.
- Bundle metadata: `bundler.toml`.

Standalone mode:

- Entry file: `src/bin/standalone.rs`.
- Host: native executable.
- Audio backend: `cpal`.
- UI shell: `eframe`.
- UI toolkit: `egui`.
- Device routing: implemented directly in the standalone app.
- Audio worker thread: receives routing commands through `std::sync::mpsc`.
- Analyzer transfer: `rtrb` ring buffer.

Shared code between both modes:

- `src/bridge/*`.
- `src/core/dsp/*`.
- `src/core/ui/*`.
- `neural/main.mojo` and `neural/libneural.so`.
- `dsp/main.dsp`, `dsp/wrapper.*`, and `dsp/FaustModule.hpp`.
- IR assets under `neural/drive/`.

---

## 3. Source Layout

This section annotates every tracked file in the repository.

The tree is intentionally full-depth because this project is small enough for a complete onboarding map.

```text
.
├── .claude/
│   └── settings.json
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── Makefile
├── PROJECT_MAP.md
├── README.md
├── build.rs
├── bundler.toml
├── dsp/
│   ├── FaustModule.hpp
│   ├── main.dsp
│   ├── wrapper.cpp
│   └── wrapper.h
├── faust-ddsp/
│   ├── diff.lib
│   └── filters.lib
├── neural/
│   ├── drive/
│   │   ├── cabinet_ir.wav
│   │   ├── pre_eq_ir.wav
│   │   ├── wavenet_drive.onnx
│   │   └── wavenet_drive.onnx.data
│   ├── libneural.so
│   └── main.mojo
├── pyproject.toml
├── scripts/
│   ├── check_env.sh
│   └── run_standalone.sh
├── src/
│   ├── bin/
│   │   └── standalone.rs
│   ├── bridge/
│   │   ├── faust.rs
│   │   ├── mod.rs
│   │   └── mojo.rs
│   ├── core/
│   │   ├── dsp/
│   │   │   ├── analyzer.rs
│   │   │   └── mod.rs
│   │   ├── state/
│   │   │   ├── mod.rs
│   │   │   └── plugin_params.rs
│   │   ├── ui/
│   │   │   ├── main_view.rs
│   │   │   ├── mod.rs
│   │   │   ├── signal_chain.rs
│   │   │   └── spectrum.rs
│   │   └── mod.rs
│   ├── lib.rs
│   └── models/
│       ├── modelo_amp.onnx
│       └── modelo_amp_lstm_2026_03_10_18_09_28.onnx.data
├── wrapper.o
└── xtask/
    ├── Cargo.toml
    └── src/
        └── main.rs
```

### Root Files

| File | Role |
|---|---|
| `.claude/settings.json` | Local assistant/tool settings. Enables `context-mode@context-mode`. Not part of runtime or build logic. |
| `.gitignore` | Ignore rules for Cargo targets, Python caches, virtual environments, editor folders, logs, local docs, and scratch scripts. |
| `Cargo.lock` | Locked Rust dependency graph for reproducible Cargo builds. About 7,866 lines in the inspected worktree. |
| `Cargo.toml` | Main Rust manifest. Defines package metadata, workspace membership, library crate types, standalone binary, runtime dependencies, build dependencies, and release/profiling profiles. |
| `Makefile` | Developer command interface for environment checks, Faust builds, Mojo builds, standalone runs, release builds, bundles, and cleaning. |
| `PROJECT_MAP.md` | This onboarding map. Added as the single project-mapping deliverable. |
| `README.md` | Existing project guide in Portuguese. Useful background, but some snippets are stale compared to current code. |
| `build.rs` | Cargo build script. Validates Faust/Mojo availability, rebuilds generated Faust and Mojo artifacts when needed, compiles C++ wrapper, runs bindgen, and emits linker directives. |
| `bundler.toml` | Metadata for `nih-plug`'s `cargo xtask bundle` flow. Maps package entry `baseIO` to human name `Distortion`. |
| `pyproject.toml` | Minimal Python project metadata named `quickstart`. No Python dependencies are declared. It is not used by the Rust build path. |
| `wrapper.o` | Tracked ELF relocatable object file, likely a previously compiled C/C++ wrapper artifact. Cargo normally regenerates native artifacts under `target`, so this is an unusual tracked build artifact. |

### `src/`

`src/` contains the Rust crate code.

It is organized by runtime entry points, external bridges, and core functionality.

```text
src/
├── lib.rs
├── bin/
│   └── standalone.rs
├── bridge/
│   ├── faust.rs
│   ├── mod.rs
│   └── mojo.rs
├── core/
│   ├── mod.rs
│   ├── dsp/
│   │   ├── analyzer.rs
│   │   └── mod.rs
│   ├── state/
│   │   ├── mod.rs
│   │   └── plugin_params.rs
│   └── ui/
│       ├── main_view.rs
│       ├── mod.rs
│       ├── signal_chain.rs
│       └── spectrum.rs
└── models/
    ├── modelo_amp.onnx
    └── modelo_amp_lstm_2026_03_10_18_09_28.onnx.data
```

#### `src/lib.rs`

`src/lib.rs` is the plugin-mode entry point and main library module.

It declares `pub mod core`.

It declares `pub mod bridge`.

It defines plugin metadata constants:

- `APP_NAME`.
- `APP_ID`.
- `VENDOR`.
- `APP_EMAIL`.
- `CLAP_ID`.
- `VST3_ID`.

It defines `BaseIO`.

`BaseIO` stores:

- `params: Arc<BaseIOParams>`.
- `analyzer_consumer: Arc<Mutex<Option<Consumer<f32>>>>`.
- `analyzer_producer: Producer<f32>`.
- `faust: [Option<FaustProcessor>; 2]`.
- `mojo: [Option<MojoProcessor>; 2]`.
- `pre_eq_convolver: [Option<FFTConvolver<f32>>; 2]`.
- `cabinet_convolver: [Option<FFTConvolver<f32>>; 2]`.
- `temp_buffer: Vec<f32>`.

`Default` for `BaseIO` initializes:

- A 64K-sample `rtrb` ring buffer for analyzer samples.
- Default plugin parameters.
- Two Faust processors.
- Two Mojo processors.
- Empty convolution slots.
- Empty temporary buffer.

`Plugin::editor` builds the plugin editor with `nih_plug_egui`.

The editor:

- Requests repaint each frame.
- Pulls analyzer samples from the consumer.
- Shows a top header panel.
- Shows input-source selection.
- Shows global bypass.
- Renders the shared spectrum/signal-chain UI.
- Uses `egui_knob` for EQ and neural controls.

`Plugin::initialize`:

- Initializes Faust processors with the host sample rate.
- Initializes Mojo processors with the host sample rate.
- Loads `pre_eq_ir.wav`.
- Loads `cabinet_ir.wav`.
- Initializes convolution engines for left and right channels.
- Resizes `temp_buffer` to max host buffer size.

`Plugin::process`:

- Reads current input, bypass, neural, drive, makeup, and volume parameters.
- Sets plugin latency to `0` samples.
- Performs input routing on interleaved sample iteration.
- Converts the NIH buffer to channel slices.
- Processes each channel through the staged DSP pipeline.
- Pushes post-processed analyzer samples.
- Returns `ProcessStatus::Normal`.

`ClapPlugin` implementation:

- Uses `CLAP_ID`.
- Describes the plugin as `BaseIO Audio Engine Template`.
- Advertises audio effect and stereo CLAP features.

`Vst3Plugin` implementation:

- Uses static `VST3_ID`.
- Advertises analyzer VST3 subcategory.

#### `src/bin/standalone.rs`

`src/bin/standalone.rs` is a native standalone host.

It uses `cpal` for audio device enumeration and streams.

It uses `eframe` for the desktop window.

It uses `egui` for UI.

It imports the library crate as `distortion`.

It reuses:

- `AnalyzerDsp`.
- `FFT_SIZE`.
- `MojoProcessor`.
- `ExternalProcessor`.
- `render_shared_panels`.
- `ActivePanel`.
- `FaustProcessor` through fully qualified paths.

It defines `StandaloneState`.

`StandaloneState` mirrors the main runtime controls without NIH parameter wrappers:

- EQ active flag.
- EQ low/mid/high frequency, gain, and Q values.
- Neural active flag.
- Neural volume.
- Bypass flag.

It defines `DeviceContext`.

`DeviceContext` stores:

- Display name.
- Raw CPAL device name.
- Channel count.
- Sample rate.

It defines `AudioCommand`.

`AudioCommand` variants are:

- `RefreshDevices`.
- `ApplyRouting`.
- `Stop`.

It defines `AudioEvent`.

`AudioEvent` variants are:

- `DevicesRefreshed`.
- `StreamStarted`.

It defines `StandaloneApp`.

`StandaloneApp` owns:

- Analyzer DSP and consumer.
- Available host list.
- Selected host.
- Input/output device lists.
- Selected input/output indices.
- Channel routing indices.
- Buffer-size exponent and bounds.
- Sample-rate warnings.
- Last audio errors.
- Settings panel state.
- Loading state.
- Active shared panel.
- Shared `StandaloneState`.
- Command/event channels for the audio worker.

`beautify_linux_name` turns raw Linux device names into friendlier UI labels.

`audio_worker` runs the CPAL stream management loop.

The F32 input stream path:

- Initializes local Faust processors.
- Initializes Mojo processors.
- Loads IR WAV files.
- Initializes FFT convolvers.
- Allocates left/right and temp buffers.
- Extracts selected input channels into block buffers.
- Applies Faust EQ.
- Applies pre-EQ convolution.
- Applies Mojo processing.
- Applies cabinet convolution.
- Sanitizes NaN/infinite samples.
- Pushes analyzer samples.
- Pushes processed samples to a ring buffer for output.

The I16 input stream path:

- Converts input samples to `f32`.
- Skips the full neural path.
- Sanitizes invalid values.
- Pushes analyzer/output samples.

The F32 output stream path:

- Pops left/right samples from the pass-through ring buffer.
- Writes them to selected output channels.

The I16 output stream path:

- Pops a sample.
- Converts it to PCM.
- Writes it to selected output channels.

`StandaloneApp::new`:

- Creates analyzer ring buffer.
- Enumerates CPAL hosts.
- Creates worker channels.
- Spawns the audio worker.
- Sets default routing and buffer-size UI state.
- Calls `refresh_devices`.

`StandaloneApp::update`:

- Polls worker events.
- Pulls analyzer samples.
- Draws top bar.
- Draws bypass checkbox.
- Draws settings panel when open.
- Draws routing controls.
- Draws buffer-size controls.
- Draws error popup.
- Draws shared spectrum/signal-chain panels.

`main`:

- Creates an `eframe::NativeOptions` window of `1000x500`.
- Runs `BaseIO Standalone`.

#### `src/bridge/mod.rs`

`src/bridge/mod.rs` exports the bridge modules.

It declares `pub mod faust`.

It declares `pub mod mojo`.

It defines `ExternalProcessor`.

The trait is the shared abstraction for native processors that can be initialized and can process an in-place block.

#### `src/bridge/faust.rs`

`src/bridge/faust.rs` is the Rust adapter for the Faust-generated C++ DSP.

It suppresses common bindgen naming warnings.

It includes generated bindings:

```rust
include!(concat!(env!("OUT_DIR"), "/bindings_faust.rs"));
```

It defines `FaustProcessor`.

`FaustProcessor` stores a `FaustHandle`.

`FaustProcessor::new` calls `faust_create`.

`FaustProcessor::new` returns `None` if the handle is null.

`set_eq_params` calls all nine Faust parameter setters:

- `faust_set_eq_low_freq`.
- `faust_set_eq_low_gain`.
- `faust_set_eq_low_q`.
- `faust_set_eq_mid_freq`.
- `faust_set_eq_mid_gain`.
- `faust_set_eq_mid_q`.
- `faust_set_eq_high_freq`.
- `faust_set_eq_high_gain`.
- `faust_set_eq_high_q`.

`Drop` calls `faust_destroy`.

`FaustProcessor` is marked `Send` with an unsafe impl.

`ExternalProcessor::init` calls `faust_init`.

`ExternalProcessor::process_block` calls `faust_process`.

#### `src/bridge/mojo.rs`

`src/bridge/mojo.rs` is the Rust adapter for the Mojo dynamic library.

It declares two foreign functions:

- `mojo_init(sample_rate: f32)`.
- `mojo_process_block(address: usize, size: usize, drive: f32, output_gain: f32)`.

It defines `MojoProcessor`.

`MojoProcessor` stores:

- `is_ready`.
- `drive`.
- `output_gain`.

`MojoProcessor::new` defaults to:

- `is_ready = false`.
- `drive = 1.0`.
- `output_gain = 1.0`.

`set_drive` updates a local scalar.

`set_output_gain` updates a local scalar.

`is_ready` returns readiness.

`ExternalProcessor::init` calls `mojo_init` and marks the processor ready.

`ExternalProcessor::process_block`:

- Returns early if not ready.
- Casts `*mut f32` to `usize`.
- Passes address, length, drive, and output gain to Mojo.

#### `src/core/mod.rs`

`src/core/mod.rs` reexports core areas:

- `state`.
- `dsp`.
- `ui`.

#### `src/core/state/mod.rs`

`src/core/state/mod.rs` declares `plugin_params`.

#### `src/core/state/plugin_params.rs`

`src/core/state/plugin_params.rs` defines plugin parameters and editor state.

It defines `InputSelect`.

`InputSelect` variants:

- `Stereo`.
- `Input1`.
- `Input2`.

It defines `BaseIOParams`.

`BaseIOParams` derives `Params`.

Persistent/UI fields:

- `editor_state`.

Input/output/global fields:

- `input_select`.
- `gain`.
- `bypass`.

Neural fields:

- `neural_amp_volume`.
- `neural_drive`.
- `neural_output_gain`.
- `neural_amp_active`.

EQ fields:

- `eq_active`.
- `eq_low_freq`.
- `eq_low_gain`.
- `eq_low_q`.
- `eq_mid_freq`.
- `eq_mid_gain`.
- `eq_mid_q`.
- `eq_high_freq`.
- `eq_high_gain`.
- `eq_high_q`.

It defines `EditorState`.

`EditorState` stores:

- Shared params.
- Analyzer DSP.
- Analyzer consumer.
- Active panel.

`Default for BaseIOParams` defines all initial values, ranges, units, formatters, and smoothing.

Smoothing examples:

- Master gain uses logarithmic smoothing over `50.0`.
- Neural volume uses logarithmic smoothing over `50.0`.
- Neural drive uses logarithmic smoothing over `50.0`.
- Neural makeup uses logarithmic smoothing over `50.0`.
- EQ frequencies use logarithmic smoothing over `50.0`.
- EQ gains use linear smoothing over `50.0`.
- EQ Q values use linear smoothing over `50.0`.

#### `src/core/dsp/mod.rs`

`src/core/dsp/mod.rs` declares and reexports the analyzer:

- `pub mod analyzer`.
- `pub use analyzer::{AnalyzerDsp, FFT_SIZE}`.

#### `src/core/dsp/analyzer.rs`

`src/core/dsp/analyzer.rs` implements the spectrum analyzer DSP.

It defines `FFT_SIZE` as `2048`.

It defines `AnalyzerDsp`.

`AnalyzerDsp` stores:

- Real FFT plan.
- Input buffer.
- Complex output buffer.
- Spectrum values.
- Audio history.

`Default`:

- Creates a `RealFftPlanner`.
- Plans a forward FFT.
- Allocates input/output buffers.
- Initializes spectrum to `-100.0` dB.
- Reserves history capacity for `FFT_SIZE * 2`.

`process_consumer`:

- Pops all available samples from an `rtrb::Consumer`.
- Maintains only the latest `FFT_SIZE` samples.
- Applies a Blackman-Harris-style window.
- Runs the forward FFT.
- Converts magnitudes to dB.
- Uses a fixed `48000.0` Hz display sample-rate assumption.
- Adds a frequency tilt.
- Clamps display dB to `[-100.0, 20.0]`.
- Smooths spectrum bins with `0.85` old and `0.15` new.

#### `src/core/ui/mod.rs`

`src/core/ui/mod.rs` declares:

- `spectrum`.
- `signal_chain`.
- `main_view`.

It reexports:

- `draw_spectrum`.
- `draw_signal_chain`.
- `ActivePanel`.
- `render_shared_panels`.

#### `src/core/ui/main_view.rs`

`src/core/ui/main_view.rs` provides shared UI composition.

`draw_eq_band`:

- Draws a vertical grouped EQ band.
- Shows title.
- Shows frequency, gain, and Q controls provided as closures.

`render_shared_panels`:

- Draws an optional bottom plugin control panel when a module is active.
- Draws EQ controls when `ActivePanel::EQ`.
- Draws neural controls when `ActivePanel::NeuralAmp`.
- Draws the always-visible signal-chain bottom panel.
- Draws the central spectrum analyzer.

The function accepts closures for:

- EQ module toggle.
- Neural module toggle.
- EQ control rendering.
- Neural control rendering.

That closure pattern lets plugin mode and standalone mode share layout while using different parameter plumbing.

#### `src/core/ui/signal_chain.rs`

`src/core/ui/signal_chain.rs` draws the signal-chain strip.

It defines `ActivePanel`.

`ActivePanel` variants:

- `None`.
- `EQ`.
- `NeuralAmp`.

`draw_signal_chain`:

- Allocates a fixed-height panel.
- Draws a dark background.
- Draws a horizontal signal line.
- Draws two nodes: `PARAM EQ` and `NEURAL AMP`.
- Draws a small power icon on each node.
- Uses module state plus global bypass to determine visual active state.
- Opens/closes the active control panel when a node is clicked.
- Toggles module active state when a power icon is clicked.

#### `src/core/ui/spectrum.rs`

`src/core/ui/spectrum.rs` draws the analyzer.

`draw_spectrum`:

- Allocates a full available central region.
- Paints a dark background.
- Draws horizontal dB grid lines.
- Draws logarithmic frequency grid lines.
- Converts FFT bins to x/y points.
- Uses a fixed display sample rate of `48000.0`.
- Draws a cyan line for the spectrum.

#### `src/models/`

`src/models/` contains ONNX model assets.

`src/models/modelo_amp.onnx`:

- Bundled ONNX model asset.
- Not referenced by the current Rust processing path.

`src/models/modelo_amp_lstm_2026_03_10_18_09_28.onnx.data`:

- External data file associated with an ONNX LSTM model.
- Not referenced by the current Rust processing path.

### `dsp/`

`dsp/` contains Faust and C/C++ integration files.

```text
dsp/
├── FaustModule.hpp
├── main.dsp
├── wrapper.cpp
└── wrapper.h
```

#### `dsp/main.dsp`

`dsp/main.dsp` is the source of truth for the Faust EQ.

It imports `stdfaust.lib`.

It loads local `diff.lib`.

It loads local `filters.lib`.

It defines low-band sliders:

- `EQ Low Freq`.
- `EQ Low Gain`.
- `EQ Low Q`.

It defines mid-band sliders:

- `EQ Mid Freq`.
- `EQ Mid Gain`.
- `EQ Mid Q`.

It defines high-band sliders:

- `EQ High Freq`.
- `EQ High Gain`.
- `EQ High Q`.

It composes:

- `filters.low_shelf`.
- `filters.peak_eq`.
- `filters.high_shelf`.
- `ma.tanh`.

It sets stereo processing with the same EQ chain on both channels.

#### `dsp/FaustModule.hpp`

`dsp/FaustModule.hpp` is generated C++ output from Faust.

It currently contains the generated class `mydsp`.

It is generated from `dsp/main.dsp`.

The build script can regenerate it.

The generated class reports two inputs and two outputs.

The generated `buildUserInterface` exposes the nine EQ slider labels used by `dsp/wrapper.cpp`.

The generated `compute` method performs the per-sample EQ and `tanhf` processing.

Because it is generated, direct manual edits should be avoided.

#### `dsp/wrapper.h`

`dsp/wrapper.h` is the C ABI contract for Rust bindgen.

It defines `f_size_t` as `unsigned long`.

It defines an opaque `FaustHandle`.

It declares lifecycle functions:

- `faust_create`.
- `faust_init`.
- `faust_process`.
- `faust_destroy`.

It declares EQ parameter setters:

- `faust_set_eq_low_freq`.
- `faust_set_eq_low_gain`.
- `faust_set_eq_low_q`.
- `faust_set_eq_mid_freq`.
- `faust_set_eq_mid_gain`.
- `faust_set_eq_mid_q`.
- `faust_set_eq_high_freq`.
- `faust_set_eq_high_gain`.
- `faust_set_eq_high_q`.

#### `dsp/wrapper.cpp`

`dsp/wrapper.cpp` implements the C ABI over generated Faust C++.

It declares minimal Faust interface types needed by `FaustModule.hpp`.

It includes `dsp/FaustModule.hpp`.

It defines `ParamMapUI`.

`ParamMapUI` captures Faust slider label pointers in a `std::map<std::string, float*>`.

It ignores Faust UI rendering callbacks because the Rust UI owns all visual controls.

It defines `FaustInstance`.

`FaustInstance` stores:

- `mydsp* dsp`.
- `ParamMapUI* ui`.

`faust_create`:

- Allocates a `FaustInstance`.
- Allocates `mydsp`.
- Allocates `ParamMapUI`.
- Builds the Faust UI to capture parameter zones.
- Returns the opaque handle.

`faust_init`:

- Calls `mydsp::init(sample_rate)`.

`faust_process`:

- Treats a single buffer pointer as both input and output.
- Builds two channel pointer arrays using the same buffer pointer.
- Calls `compute(length, in_channels, out_channels)`.

`faust_destroy`:

- Deletes UI.
- Deletes DSP.
- Deletes wrapper instance.

The `SET_PARAM` macro:

- Looks up a Faust label.
- Writes the new value into the captured parameter zone.

### `faust-ddsp/`

`faust-ddsp/` contains local Faust library files.

```text
faust-ddsp/
├── diff.lib
└── filters.lib
```

#### `faust-ddsp/filters.lib`

`filters.lib` provides stable filters used by `dsp/main.dsp`.

It imports `stdfaust.lib`.

It implements a stable state-variable filter `svf(f, q)`.

It derives:

- `peak_eq`.
- `lp`.
- `hp`.
- `low_shelf`.
- `high_shelf`.

The EQ in `dsp/main.dsp` depends on:

- `low_shelf`.
- `peak_eq`.
- `high_shelf`.

#### `faust-ddsp/diff.lib`

`diff.lib` is a larger differentiable DSP helper library.

It declares name `Faust Automatic Differentiation Library`.

It includes helper aliases for standard Faust libraries.

It defines differentiable variables and forward-mode automatic differentiation utilities.

The current `dsp/main.dsp` imports it but does not appear to use its differentiable operations in the active EQ chain.

### `neural/`

`neural/` contains Mojo source, the compiled Mojo shared library, and drive-chain assets.

```text
neural/
├── drive/
│   ├── cabinet_ir.wav
│   ├── pre_eq_ir.wav
│   ├── wavenet_drive.onnx
│   └── wavenet_drive.onnx.data
├── libneural.so
└── main.mojo
```

#### `neural/main.mojo`

`neural/main.mojo` exports C-callable Mojo functions.

It imports `UnsafePointer` from `std.memory`.

It uses `@export`.

It implements the address bypass pattern.

It reconstructs `UnsafePointer[Float32, MutAnyOrigin]` from an integer address.

It processes the same buffer memory that Rust passed.

It currently implements soft saturation directly.

It does not currently load or execute the ONNX files in `neural/drive`.

#### `neural/libneural.so`

`neural/libneural.so` is the compiled Mojo shared library.

It is linked dynamically by Rust.

It is produced by:

```text
mojo build --emit shared-lib neural/main.mojo -o neural/libneural.so
```

The inspected file is an ELF 64-bit x86-64 shared object.

Because it is checked in, new developers can potentially link without rebuilding Mojo, but the build script may regenerate it when `neural/main.mojo` is newer.

#### `neural/drive/cabinet_ir.wav`

`cabinet_ir.wav` is a cabinet impulse response.

The inspected file is a mono 44.1 kHz 16-bit PCM WAV.

It is loaded by plugin initialization and standalone F32 stream setup.

It feeds the cabinet convolution stage.

#### `neural/drive/pre_eq_ir.wav`

`pre_eq_ir.wav` is a pre-EQ impulse response.

The inspected file is a mono 44.1 kHz 16-bit PCM WAV.

It is loaded by plugin initialization and standalone F32 stream setup.

It feeds the pre-EQ convolution stage.

#### `neural/drive/wavenet_drive.onnx`

`wavenet_drive.onnx` is a bundled ONNX model asset.

It is not referenced by the current Rust/Mojo processing path.

It likely represents a future or previous neural drive model.

#### `neural/drive/wavenet_drive.onnx.data`

`wavenet_drive.onnx.data` is a sidecar data asset for the `wavenet_drive.onnx` model.

It is not referenced by the current Rust/Mojo processing path.

### `scripts/`

```text
scripts/
├── check_env.sh
└── run_standalone.sh
```

#### `scripts/check_env.sh`

`check_env.sh` validates the development environment.

It checks:

- `faust` in `PATH`.
- `mojo` in `PATH`.
- `./.venv/bin/mojo`.
- `~/.modular/pkg/packages.modular.com_mojo/bin/mojo`.
- `modular` CLI presence.
- Write permissions for `dsp`.
- Write permissions for `neural`.

It exits nonzero when required Faust or Mojo tools are missing.

#### `scripts/run_standalone.sh`

`run_standalone.sh` runs:

```text
cargo run --release --bin standalone
```

It is called by `make run` after `pre-build`.

### `xtask/`

```text
xtask/
├── Cargo.toml
└── src/
    └── main.rs
```

#### `xtask/Cargo.toml`

The `xtask` manifest defines a helper crate named `xtask`.

It depends on `nih_plug_xtask` from the `nih-plug` git repository.

This is used for `cargo xtask bundle`.

#### `xtask/src/main.rs`

The `xtask` binary delegates directly to `nih_plug_xtask::main()`.

This supplies the bundle command used by the Makefile.

---

## 4. Processing Pipeline

The plugin processing pipeline lives in `BaseIO::process` in `src/lib.rs`.

The requested six-stage view is:

1. Input routing.
2. Faust 3-band parametric EQ.
3. Pre-EQ convolution.
4. Mojo neural drive.
5. Cabinet IR convolution.
6. Master gain plus NaN/infinite sanitization.

The source code internally comments gain and sanitization as separate later steps.

This map groups them into stage 6 because they are both final safety/output conditioning.

### Stage 1: Input Routing

Implemented in:

- `src/lib.rs`.
- `src/core/state/plugin_params.rs`.

Controls:

- `InputSelect::Stereo`.
- `InputSelect::Input1`.
- `InputSelect::Input2`.

Behavior:

- `Stereo` keeps left and right as separate sources.
- `Input1` duplicates channel 1 to left and right.
- `Input2` duplicates channel 2 to left and right.
- If a second channel is missing, the right source falls back to the left input.

Why it exists:

- Guitar recording often uses a single physical input.
- The plugin still declares a stereo layout.
- Routing lets users pick a mic/guitar channel and process it as stereo.

Implementation detail:

- The code iterates through `buffer.iter_samples()`.
- It reads mutable sample references per frame.
- It writes selected left/right values back into the host buffer before any DSP.

Standalone equivalent:

- `src/bin/standalone.rs` exposes physical device/channel routing.
- It can run mono input by assigning the same physical channel to both left and right buffers.

### Stage 2: Faust 3-Band Parametric EQ

Implemented in:

- `src/lib.rs`.
- `src/bridge/faust.rs`.
- `dsp/main.dsp`.
- `dsp/wrapper.cpp`.
- `dsp/wrapper.h`.
- `dsp/FaustModule.hpp`.
- `faust-ddsp/filters.lib`.

Controls:

- `eq_active`.
- `eq_low_freq`.
- `eq_low_gain`.
- `eq_low_q`.
- `eq_mid_freq`.
- `eq_mid_gain`.
- `eq_mid_q`.
- `eq_high_freq`.
- `eq_high_gain`.
- `eq_high_q`.

Behavior:

- Runs only when global bypass is off.
- Runs only when `eq_active` is true.
- Each channel uses a per-channel `FaustProcessor`.
- Rust sends smoothed parameter values to Faust before processing the block.
- Faust processes the block in-place.

Faust DSP structure:

- Low shelf.
- Mid peak.
- High shelf.
- Final `ma.tanh`.

Bridge details:

- Rust calls `FaustProcessor::set_eq_params`.
- Rust calls `FaustProcessor::process_block`.
- `process_block` calls `faust_process`.
- `faust_process` calls `mydsp::compute`.

Parameter label dependency:

- The wrapper's `SET_PARAM` macro uses string labels like `EQ Low Freq`.
- Those labels must match labels emitted by Faust.
- If a Faust slider label changes, the C++ wrapper must be updated.

### Stage 3: Pre-EQ Convolution

Implemented in:

- `src/lib.rs`.
- `src/bin/standalone.rs`.
- `neural/drive/pre_eq_ir.wav`.

Dependency:

- `fft-convolver`.
- `hound`.

Behavior:

- Loads `pre_eq_ir.wav` during initialization.
- Creates left and right `FFTConvolver<f32>` instances.
- Copies the current channel block into `temp_buffer`.
- Calls `pre_eq.process(input, output)`.
- If processing fails, restores the original samples from the temp buffer.

Purpose:

- Represents a linear time-invariant pre-EQ or tone-stack response.
- Lets a measured or designed IR shape the signal before the nonlinear drive.

Real-time note:

- The convolver instances are created before the audio callback.
- The reusable `temp_buffer` avoids per-block vector allocation in plugin processing.

Path note:

- The plugin code currently uses an absolute base path for IR files.
- That path points to `/home/jao/VSCode/distortion/meu-novo-plugin/neural/drive/`.
- This is a portability concern for other machines and for checked-out worktrees.

### Stage 4: Mojo Neural Drive

Implemented in:

- `src/lib.rs`.
- `src/bridge/mojo.rs`.
- `neural/main.mojo`.
- `neural/libneural.so`.

Controls:

- `neural_amp_active`.
- `neural_drive`.
- `neural_output_gain`.

Behavior:

- Runs only when global bypass is off.
- Runs only when `neural_amp_active` is true.
- Rust updates the Mojo processor's drive scalar.
- Rust updates the Mojo processor's output gain scalar.
- Rust passes the channel buffer pointer to Mojo.
- Mojo mutates that buffer in-place.

Current Mojo algorithm:

- Multiply input by drive.
- Clamp to `[-4.0, 4.0]`.
- Apply polynomial tanh approximation.
- Multiply by output gain.

Zero-copy FFI detail:

- Rust casts `*mut f32` to `usize`.
- Mojo receives `address: Int`.
- Mojo reconstructs `UnsafePointer[Float32, MutAnyOrigin]`.
- Mojo writes samples directly to the original Rust/host buffer.

Latency detail:

- The plugin sets latency to zero with `_context.set_latency_samples(0)`.
- The current Mojo stage is synchronous and in-place.

### Stage 5: Cabinet IR Convolution

Implemented in:

- `src/lib.rs`.
- `src/bin/standalone.rs`.
- `neural/drive/cabinet_ir.wav`.

Dependency:

- `fft-convolver`.
- `hound`.

Behavior:

- Loads `cabinet_ir.wav` during initialization.
- Creates left and right `FFTConvolver<f32>` instances.
- Copies the current channel block into `temp_buffer`.
- Calls `cabinet.process(input, output)`.
- If processing fails, restores the original samples from the temp buffer.

Purpose:

- Simulates the cabinet/speaker response after the nonlinear drive.
- This matches typical guitar amp-chain ordering.

Asset details:

- The inspected WAV is mono, 44.1 kHz, 16-bit PCM.
- The code loads either integer PCM or float WAV samples.

### Stage 6: Master Gain + NaN Sanitization

Implemented in:

- `src/lib.rs`.
- `src/core/state/plugin_params.rs`.

Controls:

- `gain`.
- `neural_amp_volume`.
- `bypass`.

Behavior:

- Runs only when global bypass is off for gain multiplication.
- Applies `gain.smoothed.next()` to each sample.
- If neural stage is active, applies `neural_amp_volume.smoothed.next()` after master gain.
- Sanitization always runs, including bypass mode.
- Any `NaN` sample is replaced with `0.0`.
- Any infinite sample is replaced with `0.0`.

Why sanitization matters:

- External DSP and nonlinear math can produce invalid samples after extreme parameter changes.
- DAW audio engines should not receive NaN or infinite values.
- Sanitization prevents invalid values from propagating into hosts, meters, or later effects.

Analyzer step after the six stages:

- After processing and sanitization, the plugin averages the output channels for visualization.
- It pushes the visual sample into the analyzer producer.
- The UI drains this ring buffer and updates FFT display state.

---

## 5. Build System

### Build System Overview

The build system has three layers:

- Makefile orchestration for humans.
- Cargo build script orchestration for native/generator steps.
- `xtask` bundling through `nih_plug_xtask`.

The important chain is:

```text
make run / make build / make bundle
        |
        v
make pre-build
        |
        +--> scripts/check_env.sh
        +--> make build-faust
        +--> make build-mojo
        |
        v
cargo run/build/xtask bundle
        |
        v
build.rs
        |
        +--> validate Faust and Mojo
        +--> regenerate dsp/FaustModule.hpp when needed
        +--> regenerate neural/libneural.so when needed
        +--> compile dsp/wrapper.cpp with cc::Build
        +--> generate bindings_faust.rs with bindgen
        +--> link libneural.so
```

### `build.rs` Orchestration

`build.rs` starts with `pre_build_check`.

`pre_build_check` validates Faust:

- It tries `faust --version`.
- If Faust is unavailable, it panics with an installation message.

`pre_build_check` validates Mojo:

- It calls `find_mojo_path`.
- It accepts `mojo` from `PATH`.
- It accepts `./.venv/bin/mojo`.
- It accepts `~/.modular/pkg/packages.modular.com_mojo/bin/mojo`.
- It accepts `~/.modular/bin/mojo`.
- If Mojo is unavailable, it panics with an installation message.

`pre_build_check` also derives a Mojo library path:

- It takes the Mojo binary parent directory.
- It checks the sibling `lib` directory.
- It emits `cargo:rustc-link-search=native=...` when that lib directory exists.

`main` emits Cargo rerun triggers for:

- `dsp/wrapper.cpp`.
- `dsp/wrapper.h`.
- `dsp/main.dsp`.
- `neural/main.mojo`.

Faust rebuild step:

- Checks whether `dsp/main.dsp` exists.
- Compares modification time with `dsp/FaustModule.hpp`.
- Runs Faust when the header is missing or older than the DSP source.
- Command shape:

```text
faust -lang cpp -cn mydsp -vec -I faust-ddsp -i dsp/main.dsp -o dsp/FaustModule.hpp
```

Mojo rebuild step:

- Checks whether `neural/main.mojo` exists.
- Compares modification time with `neural/libneural.so`.
- Runs Mojo when the shared object is missing or older than the Mojo source.
- Command shape:

```text
mojo build --emit shared-lib neural/main.mojo -o neural/libneural.so
```

C++ wrapper compilation:

- Uses `cc::Build::new()`.
- Enables C++ mode with `.cpp(true)`.
- Compiles `dsp/wrapper.cpp`.
- Uses optimization level `3`.
- Produces a static library named `faust_dsp`.

Bindgen step:

- Reads `dsp/wrapper.h`.
- Allowlists functions matching `faust_.*`.
- Uses `bindgen::CargoCallbacks`.
- Writes `bindings_faust.rs` into Cargo `OUT_DIR`.

Mojo linking step:

- Adds the repository `neural` directory to the native link search path.
- Links a dynamic library named `neural`.

### Makefile Targets

| Target | Role |
|---|---|
| `make help` | Prints command help. |
| `make check-env` | Runs `scripts/check_env.sh`. |
| `make init` | Runs `init_plugin.py` if present; otherwise reports the project is already initialized. |
| `make build-faust` | Runs Faust manually if available and `dsp/main.dsp` exists. |
| `make build-mojo` | Runs Mojo manually if available and `neural/main.mojo` exists. |
| `make pre-build` | Runs `check-env`, `build-faust`, and `build-mojo`. |
| `make run` | Runs `pre-build`, then starts standalone through `scripts/run_standalone.sh`. |
| `make build` | Runs `pre-build`, then `cargo build --release`. |
| `make bundle` | Runs `pre-build`, then `cargo xtask bundle distortion --release`. |
| `make clean` | Runs `cargo clean`, removes generated Faust C++ files, and removes neural shared library artifacts. |

Makefile environment variables:

- `LIBTORCH` defaults to `$(HOME)/libtorch/libtorch`.
- `LIBTORCH_BYPASS_VERSION_CHECK` is set to `1`.
- `MOJO_HOME` defaults to `$(HOME)/.modular/pkg/packages.modular.com_mojo`.
- `LD_LIBRARY_PATH` includes the LibTorch library path.
- `LD_LIBRARY_PATH` includes `$(MOJO_HOME)/lib`.
- `LD_LIBRARY_PATH` includes `$(PWD)/neural`.

Current code note:

- The Rust dependency list does not include a Torch/ONNX runtime.
- The LibTorch environment setup appears to be leftover or future-facing.

### Bundle System

Bundling uses:

- `xtask/Cargo.toml`.
- `xtask/src/main.rs`.
- `bundler.toml`.
- `nih_plug_xtask`.

The command is:

```text
cargo xtask bundle distortion --release
```

The Makefile wraps this as:

```text
make bundle
```

The bundle metadata file maps `baseIO` to display name `Distortion`.

Potential naming concern:

- The crate/package is named `distortion`.
- `bundler.toml` uses a `[baseIO]` table.
- If bundling behaves unexpectedly, confirm the expected package/plugin key for the current `nih-plug` bundler.

### Full Dependency Chain

Clean build from source depends on:

- Rust toolchain.
- Cargo.
- C/C++ compiler.
- Clang/libclang for bindgen.
- Faust compiler.
- Mojo compiler.
- Mojo runtime libraries.
- `dsp/main.dsp`.
- `faust-ddsp/filters.lib`.
- `faust-ddsp/diff.lib`.
- `dsp/wrapper.h`.
- `dsp/wrapper.cpp`.
- `neural/main.mojo`.
- `neural/drive/pre_eq_ir.wav`.
- `neural/drive/cabinet_ir.wav`.
- Rust dependencies from Cargo.
- `nih-plug` git dependencies.

Generated or native artifacts:

- `dsp/FaustModule.hpp`.
- `neural/libneural.so`.
- `target/**/bindings_faust.rs`.
- `target/**/libfaust_dsp.a`.
- VST3/CLAP bundles generated by `xtask`.

---

## 6. Key Design Patterns

### Zero-Copy FFI

The audio callback avoids copying the main channel buffer when crossing into Faust or Mojo.

Rust passes raw `*mut f32` pointers to external processors.

Faust receives the pointer through the C ABI.

Mojo receives the pointer address as an integer.

Both external processors mutate samples in-place.

The shared abstraction is `ExternalProcessor`.

Benefits:

- Avoids per-block allocation.
- Avoids copying audio samples across language boundaries.
- Keeps latency low.
- Keeps the host buffer as the canonical audio storage.

Risks:

- Requires correct pointer length.
- Requires the external processor to honor the buffer element type.
- Requires external code to avoid out-of-bounds writes.
- Requires real-time-safe behavior inside native calls.

### Rust to Mojo Pointer Casting

Implemented in `src/bridge/mojo.rs`.

Pattern:

```text
*mut f32 -> usize -> Int -> UnsafePointer[Float32, MutAnyOrigin]
```

Rust side:

- Casts the pointer to `usize`.
- Calls `mojo_process_block`.

Mojo side:

- Receives `address: Int`.
- Uses `UnsafePointer(... unsafe_from_address=address)`.
- Reads/writes `data[i]`.

This exists because exported Mojo functions cannot directly receive `UnsafePointer` in the way this project needs.

### Mojo Address Bypass Pattern

The exported Mojo function uses `address: Int` instead of pointer parameters.

This avoids `@export` constraints around pointer types.

This also makes the ABI simple for Rust:

- Integer address.
- Integer size.
- Scalar drive.
- Scalar output gain.

Rules for this pattern:

- The Rust buffer must outlive the Mojo call.
- Mojo must not store the pointer.
- Mojo must process only `0..size`.
- Mojo must treat data as `Float32`.
- Rust and Mojo must agree on sample layout.

### Parameter Smoothing

Implemented in `src/core/state/plugin_params.rs`.

The project uses `nih_plug::prelude::FloatParam`.

Smoothing is set with `with_smoother(SmoothingStyle::...)`.

Logarithmic smoothing is used for gain-like parameters.

Linear smoothing is used for gain-in-dB and Q controls where the parameter itself is a linear user value.

The audio callback reads smoothed values through `.smoothed.next()`.

Important behavior:

- Each call to `.smoothed.next()` advances the smoother.
- In `BaseIO::process`, some smoothed values are read once before channel processing.
- EQ smoothed values are read while setting Faust params.
- Master gain is read once per channel before sample loop.

Benefits:

- Reduces zipper noise.
- Supports sample-accurate automation expectations.
- Keeps the GUI and DAW automation responsive without abrupt changes.

### ExternalProcessor Trait

The `ExternalProcessor` trait is a small adapter contract.

It hides whether the backend is Faust, Mojo, or another native processor.

The trait currently has only two methods:

- `init`.
- `process_block`.

This keeps audio code simple:

- Initialize processors in `initialize`.
- Process channel slices through a common method.

Potential future extension:

- Add reset methods for processor state.
- Add latency reporting.
- Add error/status reporting.
- Add parameter update traits per backend.

### Faust C Wrapper Pattern

Faust generates C++ classes, not a stable C ABI.

Rust FFI works best against a C ABI.

The wrapper pattern solves that.

Steps:

- `dsp/main.dsp` defines DSP.
- Faust generates `dsp/FaustModule.hpp`.
- `dsp/wrapper.cpp` includes generated C++.
- `dsp/wrapper.cpp` exposes `extern "C"` functions.
- `dsp/wrapper.h` declares those functions.
- `bindgen` generates Rust declarations from the header.
- `src/bridge/faust.rs` uses those declarations.

The wrapper also captures Faust parameters.

`ParamMapUI` stores slider labels to parameter zones.

The Rust bridge sets parameters by calling C functions.

The C functions set underlying Faust zones by label.

### Dual UI Composition Pattern

The plugin and standalone mode use different shells but share the core UI panels.

Plugin shell:

- `nih_plug_egui::create_egui_editor`.
- NIH parameter setter callbacks.

Standalone shell:

- `eframe::run_native`.
- Plain Rust state under `Arc<Mutex<StandaloneState>>`.

Shared UI:

- `render_shared_panels`.
- `draw_signal_chain`.
- `draw_spectrum`.
- `draw_eq_band`.

The shared renderer receives closures for controls and toggles.

This avoids duplicating spectrum and signal-chain UI across plugin and standalone.

### Analyzer Producer/Consumer Pattern

Audio callback code must avoid locking the UI.

The project uses ring buffers to decouple audio and UI.

Plugin mode:

- `BaseIO` owns an analyzer producer.
- Editor state owns the analyzer consumer behind `Arc<Mutex<Option<Consumer<f32>>>>`.
- Audio pushes visualization samples.
- UI tries to lock and drain samples.

Standalone mode:

- Audio worker creates analyzer producer/consumer pairs when routing is applied.
- UI drains the consumer.

The analyzer consumes only visualization samples.

It does not feed audio back into processing.

### Convolver Fallback Pattern

Convolution stages copy the current block into a temp buffer.

They attempt to process from temp input into the main output channel slice.

If the convolver returns an error, the code restores the original temp buffer.

This protects against silence or partially modified output when convolution fails.

---

## 7. Dependencies

### Rust Runtime Dependencies

Declared in `Cargo.toml`:

| Crate | Version/source | Role observed in project |
|---|---|---|
| `arc-swap` | `1.7.1` | Declared, not obviously used in current source. |
| `biquad` | `0.5.0` | Declared, not obviously used in current source. |
| `cpal` | `0.15.2` | Standalone audio host device and stream handling. |
| `eframe` | `0.31.0` | Standalone desktop UI shell. |
| `fast-math` | `0.1.1` | Declared, not obviously used in current source. |
| `hound` | `3.5.1` | WAV IR loading. |
| `nalgebra` | `0.34.1` | Declared, not obviously used in current source. |
| `nih_plug` | git `robbert-vdh/nih-plug` | Plugin framework, params, VST3/CLAP exports, standalone feature. |
| `nih_plug_vizia` | git `robbert-vdh/nih-plug` | Declared, not obviously used in current source. |
| `nih_plug_egui` | git `robbert-vdh/nih-plug` | Plugin editor integration with egui. |
| `num-complex` | `0.4.6` | Complex FFT output values in analyzer. |
| `realfft` | `3.5.0` | Real FFT planning and execution for analyzer. |
| `rfd` | `0.14.1` | Declared, not obviously used in current source. |
| `ringbuf` | `0.4.8` | Declared and used in standalone imports for ring buffer creation. |
| `rtrb` | `0.3.3` | Analyzer and standalone pass-through ring buffers. |
| `rubato` | `1.0.1` | Declared, not obviously used in current source. |
| `egui_knob` | `0.2.0` | Knob controls in plugin and standalone UI. |
| `fft-convolver` | `0.3.0` | Pre-EQ and cabinet convolution. |

### Rust Build Dependencies

| Crate | Version/source | Role |
|---|---|---|
| `cc` | `1.0` | Compiles `dsp/wrapper.cpp` as C++. |
| `bindgen` | `0.71.1` | Generates Rust FFI bindings from `dsp/wrapper.h`. |

### Workspace Dependencies

The workspace has one member:

- `xtask`.

`xtask` depends on:

- `nih_plug_xtask` from the `nih-plug` git repository.

### System Dependencies

Required or practically required:

| Dependency | Required for | Where enforced |
|---|---|---|
| Rust + Cargo | All Rust builds and bundling | Cargo/Makefile |
| C/C++ compiler | Compiling `dsp/wrapper.cpp` | `cc` crate |
| Clang/libclang | `bindgen` header parsing | `bindgen` |
| Faust compiler | Generating `dsp/FaustModule.hpp` | `build.rs`, `scripts/check_env.sh`, `make build-faust` |
| Mojo compiler | Generating `neural/libneural.so` | `build.rs`, `scripts/check_env.sh`, `make build-mojo` |
| Mojo runtime libs | Linking/running `libneural.so` | `build.rs`, `Makefile LD_LIBRARY_PATH` |
| Modular CLI | Mojo installation/auth support | Warned by `scripts/check_env.sh` |
| Audio backend libraries | Standalone CPAL streams | Platform dependent |
| ALSA dev/runtime packages | Linux standalone audio | Platform dependent |

### Bundled Assets

| Asset | Role |
|---|---|
| `neural/drive/pre_eq_ir.wav` | Pre-drive impulse response for convolution. |
| `neural/drive/cabinet_ir.wav` | Post-drive cabinet impulse response for convolution. |
| `neural/drive/wavenet_drive.onnx` | Bundled ONNX neural drive model, not currently used by runtime code. |
| `neural/drive/wavenet_drive.onnx.data` | Sidecar data for the bundled ONNX model, not currently used by runtime code. |
| `src/models/modelo_amp.onnx` | Additional ONNX model asset, not currently used by runtime code. |
| `src/models/modelo_amp_lstm_2026_03_10_18_09_28.onnx.data` | Sidecar data for additional ONNX model asset, not currently used by runtime code. |
| `neural/libneural.so` | Compiled Mojo shared library used by Rust at link/runtime. |
| `wrapper.o` | Tracked native object artifact, not part of normal Cargo source flow. |

---

## 8. Development Commands

Run environment checks:

```bash
make check-env
```

Run all prebuild generators:

```bash
make pre-build
```

Build Faust only:

```bash
make build-faust
```

Build Mojo only:

```bash
make build-mojo
```

Run standalone host:

```bash
make run
```

Equivalent standalone command after prebuild:

```bash
cargo run --release --bin standalone
```

Build release artifacts:

```bash
make build
```

Equivalent release build after prebuild:

```bash
cargo build --release
```

Bundle VST3/CLAP:

```bash
make bundle
```

Equivalent bundle command after prebuild:

```bash
cargo xtask bundle distortion --release
```

Clean Cargo and generated native artifacts:

```bash
make clean
```

Run Cargo metadata inspection:

```bash
cargo metadata --no-deps --format-version 1
```

Check Git state:

```bash
git status --short --branch
```

List tracked files:

```bash
git ls-files
```

Manual Faust generation command used by `build.rs`:

```bash
faust -lang cpp -cn mydsp -vec -I faust-ddsp -i dsp/main.dsp -o dsp/FaustModule.hpp
```

Manual Faust generation command used by `Makefile`:

```bash
faust -lang cpp -cn mydsp -I faust-ddsp -i dsp/main.dsp -o dsp/FaustModule.hpp
```

Manual Mojo generation command:

```bash
mojo build --emit shared-lib neural/main.mojo -o neural/libneural.so
```

Expected new-developer path:

```bash
make check-env
make pre-build
make run
make bundle
```

---

## 9. Important Runtime Notes

### Audio Thread Expectations

The plugin should avoid allocations in `process`.

`nih_plug` is built with `assert_process_allocs`, which helps catch allocations in the process callback.

The current plugin process loop uses preallocated processors, convolvers, and temp buffer.

The analyzer push uses a lock-free ring buffer style.

External processors are called synchronously.

### Channel Handling

The plugin declares a stereo layout.

`BaseIO::process` processes channel slices independently.

For channel indices above 1, it clamps the DSP processor index to 1.

This protects against extra host channels using out-of-bounds processor arrays.

The Faust wrapper itself passes the same channel buffer as both Faust input channels and output channels.

Because `BaseIO::process` calls Faust per channel slice, the wrapper effectively runs the stereo Faust code against duplicated mono input for that slice.

### Parameter Label Coupling

Faust parameter setters depend on exact label strings.

Changing labels in `dsp/main.dsp` can silently break parameter updates unless `dsp/wrapper.cpp` is updated.

The labels currently used by Rust/C++ are:

- `EQ Low Freq`.
- `EQ Low Gain`.
- `EQ Low Q`.
- `EQ Mid Freq`.
- `EQ Mid Gain`.
- `EQ Mid Q`.
- `EQ High Freq`.
- `EQ High Gain`.
- `EQ High Q`.

### Absolute Asset Path

Both `src/lib.rs` and `src/bin/standalone.rs` use an absolute base path for IR loading.

The path points to the original project directory, not necessarily the current worktree.

This can fail on other machines or in sibling worktrees.

Current path pattern:

```text
/home/jao/VSCode/distortion/meu-novo-plugin/neural/drive/
```

A more portable future approach would derive paths from `CARGO_MANIFEST_DIR` or bundle assets.

### ONNX Asset Drift

The repository contains several ONNX assets.

The active runtime does not use them.

The README still references older ONNX/tract behavior in places.

The current Mojo implementation is direct polynomial saturation.

Any future work that reintroduces model inference should update:

- `Cargo.toml`.
- `src/lib.rs`.
- `src/bridge/mojo.rs`.
- `neural/main.mojo`.
- Build scripts.
- This project map.
- README.

### Generated/Build Artifacts in Git

The repository tracks:

- `dsp/FaustModule.hpp`.
- `neural/libneural.so`.
- `wrapper.o`.

Tracking `dsp/FaustModule.hpp` can be useful when Faust is hard to install.

Tracking `neural/libneural.so` can be useful when Mojo is hard to install.

Tracking `wrapper.o` is less clearly useful because Cargo compiles the wrapper itself.

If build artifacts are intentionally tracked, the project should document the policy.

---

## 10. Quick Navigation

Start with these files when onboarding:

| Goal | Read first |
|---|---|
| Understand plugin processing | `src/lib.rs` |
| Understand standalone mode | `src/bin/standalone.rs` |
| Understand parameters | `src/core/state/plugin_params.rs` |
| Understand Faust bridge | `src/bridge/faust.rs`, `dsp/wrapper.cpp`, `dsp/wrapper.h` |
| Understand Mojo bridge | `src/bridge/mojo.rs`, `neural/main.mojo` |
| Understand EQ DSP | `dsp/main.dsp`, `faust-ddsp/filters.lib` |
| Understand analyzer | `src/core/dsp/analyzer.rs`, `src/core/ui/spectrum.rs` |
| Understand UI layout | `src/core/ui/main_view.rs`, `src/core/ui/signal_chain.rs` |
| Understand build orchestration | `build.rs`, `Makefile` |
| Understand bundling | `xtask/src/main.rs`, `xtask/Cargo.toml`, `bundler.toml` |

Common implementation entry points:

- Add or rename plugin parameters in `src/core/state/plugin_params.rs`.
- Use parameters in audio in `src/lib.rs`.
- Add plugin UI controls in `src/lib.rs` and shared UI modules.
- Add standalone controls in `src/bin/standalone.rs`.
- Change EQ math in `dsp/main.dsp`.
- Regenerate Faust output through `make build-faust`.
- Change Faust C ABI in `dsp/wrapper.h` and `dsp/wrapper.cpp`.
- Change Mojo processing in `neural/main.mojo`.
- Regenerate Mojo library through `make build-mojo`.
- Change bundle metadata in `bundler.toml`.

---

## 11. Maintenance Checklist

When changing Faust:

- Update `dsp/main.dsp`.
- Regenerate `dsp/FaustModule.hpp`.
- Confirm labels still match `dsp/wrapper.cpp`.
- Confirm `dsp/wrapper.h` exposes any new functions needed by Rust.
- Confirm bindgen allowlist still catches new functions.
- Run `make pre-build`.
- Run `make run`.

When changing Mojo:

- Update `neural/main.mojo`.
- Keep exported function signatures synchronized with `src/bridge/mojo.rs`.
- Rebuild `neural/libneural.so`.
- Ensure `LD_LIBRARY_PATH` can find Mojo/runtime libraries and repository `neural`.
- Run `make run`.

When changing plugin parameters:

- Update `BaseIOParams`.
- Add IDs that are stable and host-automation friendly.
- Add smoothing where audio-rate changes matter.
- Update plugin editor controls.
- Update standalone controls if the same behavior should be testable outside a DAW.
- Update Faust or Mojo parameter forwarding if needed.

When changing assets:

- Confirm WAV sample format is supported by `hound` loading code.
- Confirm IR length works with `FFTConvolver::init`.
- Avoid absolute paths where possible.
- Confirm standalone and plugin modes load the same assets.

When changing build logic:

- Update `build.rs`.
- Update `Makefile`.
- Update `scripts/check_env.sh` if new tools are required.
- Update README and this project map.
- Test `make clean`.
- Test `make pre-build`.
- Test `make build`.
- Test `make bundle`.

---

## 12. Known Concerns And Follow-Up Areas

These are not necessarily bugs in the requested documentation task, but they matter for future maintainers.

1. Absolute IR paths reduce portability.

2. README contains stale references to `tract-onnx` and an older Mojo FFI signature.

3. Several dependencies in `Cargo.toml` are declared but not obviously used by current source.

4. `LIBTORCH` environment variables remain in `Makefile`, but current Rust code does not use LibTorch.

5. ONNX assets are bundled but not currently executed.

6. `wrapper.o` is tracked even though Cargo builds native wrapper artifacts.

7. `neural/libneural.so` is tracked and platform-specific.

8. `dsp/FaustModule.hpp` is generated but tracked.

9. `AnalyzerDsp` and `draw_spectrum` use a fixed 48 kHz display sample rate.

10. Standalone F32 path has the richest DSP path; I16 input path skips full neural processing.

11. Faust wrapper processes each channel slice as duplicated pseudo-stereo.

12. The standalone path uses `neural_vol` as drive in one place, while plugin mode has distinct drive, makeup, and volume controls.

13. `bundler.toml` uses `[baseIO]` while the package name is `distortion`; verify bundler key expectations if bundle output is missing.

14. The `VENDOR` constant is empty.

15. `CLAP_ID` is `.distortion`, which may not be globally unique.

16. `Cargo.toml` package description is still generic.

17. `pyproject.toml` is minimal and appears unrelated to the current build path.

18. Some UI labels still reference `BaseIO` or `PyTorch`, while the current product identity is `Distortion` and active neural path is Mojo.

---

## 13. Summary

This repository is a Rust-centered real-time guitar distortion plugin.

It uses `nih-plug` to expose VST3 and CLAP plugin targets.

It uses `cpal` and `eframe` for a standalone development host.

It uses Faust for the stable 3-band EQ stage.

It uses Mojo for in-place nonlinear drive processing.

It uses FFT convolution for pre-EQ and cabinet IR stages.

The central runtime path is `BaseIO::process`.

The central build path is `build.rs`.

The central FFI abstraction is `ExternalProcessor`.

The most important onboarding habit is to keep Rust, Faust, Mojo, and wrapper signatures synchronized.

The most important portability issue is the current use of absolute IR asset paths.
