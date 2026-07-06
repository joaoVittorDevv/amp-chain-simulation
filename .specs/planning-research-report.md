# BROWNFIELD MAPPING REPORT

Analyzed repository: `/home/jao/VSCode/distortion/meu-novo-plugin`

Analyzed date: 2026-07-06

## 1. STACK

### Rust Package

- Package: `distortion`
- Version: `0.1.0`
- Edition: `2021`
- Authors/license/homepage/description: [Cargo.toml](/home/jao/VSCode/distortion/meu-novo-plugin/Cargo.toml:1)
- Workspace member: `xtask`: [Cargo.toml](/home/jao/VSCode/distortion/meu-novo-plugin/Cargo.toml:10)
- Library crate types: `cdylib`, `lib`: [Cargo.toml](/home/jao/VSCode/distortion/meu-novo-plugin/Cargo.toml:13)
- Standalone binary: `standalone`, path `src/bin/standalone.rs`: [Cargo.toml](/home/jao/VSCode/distortion/meu-novo-plugin/Cargo.toml:16)

### Rust Dependencies

All dependencies declared in [Cargo.toml](/home/jao/VSCode/distortion/meu-novo-plugin/Cargo.toml:20):

| Crate | Version/source | Purpose observed |
| --- | --- | --- |
| `arc-swap` | `1.7.1` | Lock-free cabinet runtime handoff via `ArcSwapOption`. |
| `biquad` | `0.5.0` | Present dependency; not used in the requested files. |
| `cpal` | `0.15.2` | Standalone audio device I/O. |
| `eframe` | `0.31.0` | Standalone GUI app shell. |
| `fast-math` | `0.1.1` | Present dependency; not used in requested files. |
| `hound` | `3.5.1` | WAV decoding for embedded pre-EQ and cabinet IRs. |
| `nalgebra` | `0.34.1` | Present dependency; not used in requested files. |
| `nih_plug` | git `https://github.com/robbert-vdh/nih-plug.git`, features `assert_process_allocs`, `standalone` | VST3/CLAP framework, params, audio callback, plugin exports. |
| `nih_plug_vizia` | same git repo | Present dependency; not used in requested files. |
| `nih_plug_egui` | same git repo | Plugin Egui editor integration. |
| `num-complex` | `0.4.6` | FFT analyzer complex spectrum values. |
| `realfft` | `3.5.0` | FFT spectrum analyzer. |
| `rfd` | `0.14.1` | Native file dialogs for cabinet IR import/export. |
| `ringbuf` | `0.4.8` | Present dependency; requested files use `rtrb` instead. |
| `rtrb` | `0.3.3` | Audio-to-UI analyzer ring buffer and standalone input-to-output ring buffer. |
| `rubato` | `1.0.1` | Cabinet IR resampling. |
| `audioadapter-buffers` | `=2.0.0` | Adapter buffers for `rubato` resampling. |
| `egui_knob` | `0.2.0` | Knob widgets in plugin and standalone UI. |
| `fft-convolver` | `0.3.0` | Pre-EQ and cabinet IR convolution. |
| `tract-onnx` | `0.23` | Unused by active `BaseIO`, but used in `bridge/wavenet.rs`. |
| `rusqlite` | `0.31`, feature `bundled` | Cabinet IR SQLite library. |
| `blake3` | `1` | Cabinet IR content hashing. |
| `dirs` | `5` | User data/config paths. |
| `serde` | `1`, feature `derive` | Config/data serialization support. |
| `serde_json` | `1` | Standalone cabinet selection persistence. |
| `thiserror` | `1` | Cabinet error type support. |

### Build Dependencies

- `cc = "1.0"`: compiles `dsp/wrapper.cpp` as C++: [Cargo.toml](/home/jao/VSCode/distortion/meu-novo-plugin/Cargo.toml:57), [build.rs](/home/jao/VSCode/distortion/meu-novo-plugin/build.rs:105)
- `bindgen = "0.71.1"`: generates Rust FFI bindings for functions matching `faust_.*`: [Cargo.toml](/home/jao/VSCode/distortion/meu-novo-plugin/Cargo.toml:57), [build.rs](/home/jao/VSCode/distortion/meu-novo-plugin/build.rs:112)

### Build Toolchain

- Faust compiler is mandatory during build. `build.rs` checks `faust --version` and panics if missing: [build.rs](/home/jao/VSCode/distortion/meu-novo-plugin/build.rs:31)
- Mojo compiler is mandatory during build. `build.rs` locates it in PATH, `./.venv/bin/mojo`, or Modular install locations: [build.rs](/home/jao/VSCode/distortion/meu-novo-plugin/build.rs:5), [build.rs](/home/jao/VSCode/distortion/meu-novo-plugin/build.rs:38)
- Faust compilation command: `faust -lang cpp -cn mydsp -vec -I faust-ddsp -i dsp/main.dsp -o dsp/FaustModule.hpp`: [build.rs](/home/jao/VSCode/distortion/meu-novo-plugin/build.rs:72)
- Mojo compilation command: `mojo build --emit shared-lib neural/main.mojo -o neural/libneural.so`: [build.rs](/home/jao/VSCode/distortion/meu-novo-plugin/build.rs:94)
- Link path includes local `neural` directory and links `dylib=neural`: [build.rs](/home/jao/VSCode/distortion/meu-novo-plugin/build.rs:125)

### Makefile Commands

- Environment setup: LibTorch and Mojo `LD_LIBRARY_PATH` exports: [Makefile](/home/jao/VSCode/distortion/meu-novo-plugin/Makefile:6)
- `make check-env`: runs `scripts/check_env.sh`: [Makefile](/home/jao/VSCode/distortion/meu-novo-plugin/Makefile:3)
- `make build-faust`: standalone Faust generation path: [Makefile](/home/jao/VSCode/distortion/meu-novo-plugin/Makefile:32)
- `make build-mojo`: standalone Mojo shared library generation path: [Makefile](/home/jao/VSCode/distortion/meu-novo-plugin/Makefile:45)
- `make pre-build`: `check-env build-faust build-mojo`: [Makefile](/home/jao/VSCode/distortion/meu-novo-plugin/Makefile:62)
- `make run`: runs standalone host through `scripts/run_standalone.sh`: [Makefile](/home/jao/VSCode/distortion/meu-novo-plugin/Makefile:64)
- `make build`: `cargo build --release`: [Makefile](/home/jao/VSCode/distortion/meu-novo-plugin/Makefile:67)
- `make bundle`: `cargo xtask bundle distortion --release`: [Makefile](/home/jao/VSCode/distortion/meu-novo-plugin/Makefile:70)
- `make clean`: `cargo clean`, removes generated DSP headers/C++ and neural shared libraries: [Makefile](/home/jao/VSCode/distortion/meu-novo-plugin/Makefile:73)

## 2. ARCHITECTURE

### High-Level Shape

This is a multi-target audio project:

- Plugin target: `BaseIO` implements `nih_plug::Plugin`, `ClapPlugin`, and `Vst3Plugin`: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:118), [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:706), [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:714)
- Standalone target: `StandaloneApp` implements `eframe::App` and owns CPAL device/routing UI plus an audio worker thread: [src/bin/standalone.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bin/standalone.rs:221), [src/bin/standalone.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bin/standalone.rs:285), [src/bin/standalone.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bin/standalone.rs:845)
- Shared DSP bridges live under `src/bridge/`: [src/bridge/mod.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bridge/mod.rs:1)
- Shared state, DSP analyzer, cabinet system, and UI live under `src/core/`.

### `BaseIO` Ownership

`BaseIO` owns:

- `Arc<BaseIOParams>` for plugin parameters: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:71)
- Analyzer `rtrb` producer/consumer: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:73)
- Per-channel Faust processors: `[Option<FaustProcessor>; 2]`: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:76)
- Per-channel Mojo processors: `[Option<MojoProcessor>; 2]`: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:77)
- Per-channel pre-EQ convolvers: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:80)
- Cabinet engine/library/scratch/error/sample-rate state: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:83)

`Default` creates a 64k analyzer ring buffer, initializes Faust/Mojo handles, opens/seeds the cabinet library, and initializes cabinet shared state: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:94)

### Full 6-Stage Plugin DSP Pipeline

The requested six stages are implemented in `BaseIO::process()` at [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:544). The code comments call the internal stages 1-7, but functionally the requested six-stage chain maps as follows:

1. **Input routing**
   - Reads `input_select` and `bypass`: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:550)
   - Iterates `buffer.iter_samples()`: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:575)
   - Selects stereo, Input 1 duplicated to both sides, or Input 2 duplicated to both sides: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:583)
   - Writes selected values back into the host buffer before any DSP: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:589)

2. **Faust 3-band parametric EQ**
   - `buffer.as_slice()` obtains mutable channel slices for raw-pointer FFI: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:594)
   - Per-channel loop begins when not bypassed: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:597)
   - `eq_active` gates EQ processing: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:603)
   - `FaustProcessor::set_eq_params(...)` receives low/mid/high frequency, gain, and Q values: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:606)
   - `faust.process_block(channel_samples.as_mut_ptr(), len)` runs in place: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:617)

3. **Pre-EQ convolution**
   - `pre_eq_convolver` is initialized from embedded `DEFAULT_PRE_EQ_IR` in `initialize()`: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:487)
   - Per-channel convolver runs after Faust EQ: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:621)
   - Uses `temp_buffer` as dry input copy; if `FFTConvolver::process()` errors, the dry temp buffer is restored: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:623)

4. **Mojo neural drive / tanh saturation**
   - `neural_active` gates the stage: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:629)
   - Per-channel Mojo receives smoothed `neural_drive` and `neural_output_gain`: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:631)
   - In-place zero-copy call: `mojo.process_block(channel_samples.as_mut_ptr(), len)`: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:634)

5. **Cabinet IR convolution**
   - Stereo cabinet stage runs after per-channel EQ/pre-EQ/neural stages: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:639)
   - Stereo path splits mutable channel slices and calls `CabinetEngine::process(...)`: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:643)
   - Mono fallback mirrors single channel through a temporary right side: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:654)
   - Cabinet `process()` preserves dry signal, convolves wet paths, applies `level`, `mix`, mute ramp, and bypass fade: [src/core/cabinet/engine.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/cabinet/engine.rs:230), [src/core/cabinet/engine.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/cabinet/engine.rs:256), [src/core/cabinet/engine.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/cabinet/engine.rs:294)

6. **Master gain + NaN sanitization**
   - Per-channel master gain applies `gain`: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:670)
   - If neural stage is active, applies `neural_amp_volume`: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:675)
   - NaN/Inf sanitization always runs, even in global bypass: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:683)

After processing, analyzer samples are pushed as post-processing L/R average: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:692)

### Dual-Target Architecture

Plugin path:

- Parameters come from `BaseIOParams`, including host automation and smoothing: [src/core/state/plugin_params.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/state/plugin_params.rs:17)
- Editor is created with `create_egui_editor`: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:144)
- Audio is processed directly in the NIH host callback through `Buffer`: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:544)
- CLAP/VST3 exports happen at the end of `src/lib.rs`: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:719)

Standalone path:

- `StandaloneState` duplicates the plugin parameter surface as plain fields: [src/bin/standalone.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bin/standalone.rs:40)
- `AudioSnapshot` gives the audio callback a copy-only subset without cloning `cab_active_hash`: [src/bin/standalone.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bin/standalone.rs:92)
- `audio_worker()` handles CPAL host/device enumeration and stream creation: [src/bin/standalone.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bin/standalone.rs:285)
- Input callback extracts selected hardware channels into `buf_l` and `buf_r`: [src/bin/standalone.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bin/standalone.rs:453)
- Standalone runs the same logical DSP order, then pushes stereo samples through an `rtrb` ring buffer to the output stream: [src/bin/standalone.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bin/standalone.rs:468), [src/bin/standalone.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bin/standalone.rs:541)
- `eframe::run_native()` launches `"BaseIO Standalone"`: [src/bin/standalone.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bin/standalone.rs:1418)

### `ExternalProcessor` Trait

Definition:

```rust
pub trait ExternalProcessor {
    fn init(&mut self, sample_rate: f32);
    fn process_block(&mut self, buffer: *mut f32, length: usize);
}
```

Location: [src/bridge/mod.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bridge/mod.rs:5)

Implementations:

- `FaustProcessor`: [src/bridge/faust.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bridge/faust.rs:56)
- `MojoProcessor`: [src/bridge/mojo.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bridge/mojo.rs:47)
- `WavenetProcessor`: [src/bridge/wavenet.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bridge/wavenet.rs:116)

`WavenetProcessor` is exported from `bridge/mod.rs` but not used by the current `BaseIO` chain: [src/bridge/mod.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bridge/mod.rs:3), [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:14)

### Zero-Copy Rust <-> Mojo FFI Pattern

Rust declares:

```rust
extern "C" {
    fn mojo_init(sample_rate: f32);
    fn mojo_process_block(address: usize, size: usize, drive: f32, output_gain: f32);
}
```

Location: [src/bridge/mojo.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bridge/mojo.rs:3)

Rust converts `*mut f32` to `usize` and passes drive/output gain by value: [src/bridge/mojo.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bridge/mojo.rs:55)

Mojo exports:

```mojo
@export
fn mojo_process_block(address: Int, size: Int, drive: Float32, output_gain: Float32):
```

Location: [neural/main.mojo](/home/jao/VSCode/distortion/meu-novo-plugin/neural/main.mojo:16)

Mojo reconstructs a mutable pointer with `UnsafePointer[Float32, MutAnyOrigin](unsafe_from_address=address)` and writes back to the same buffer: [neural/main.mojo](/home/jao/VSCode/distortion/meu-novo-plugin/neural/main.mojo:30)

### Faust C Wrapper Pattern

Faust source:

- Imports `stdfaust.lib`, local `diff.lib`, local `filters.lib`: [dsp/main.dsp](/home/jao/VSCode/distortion/meu-novo-plugin/dsp/main.dsp:1)
- Defines low, mid, high controls via `hslider`: [dsp/main.dsp](/home/jao/VSCode/distortion/meu-novo-plugin/dsp/main.dsp:8)
- `eq_chain = eq_low : eq_mid : eq_high : ma.tanh`: [dsp/main.dsp](/home/jao/VSCode/distortion/meu-novo-plugin/dsp/main.dsp:26)
- Stereo orchestration: `process = _,_ : (eq_chain, eq_chain)`: [dsp/main.dsp](/home/jao/VSCode/distortion/meu-novo-plugin/dsp/main.dsp:29)

C++ wrapper:

- Provides minimal Faust `Meta`, `UI`, and `dsp` interfaces expected by generated `FaustModule.hpp`: [dsp/wrapper.cpp](/home/jao/VSCode/distortion/meu-novo-plugin/dsp/wrapper.cpp:8)
- Includes generated `FaustModule.hpp`: [dsp/wrapper.cpp](/home/jao/VSCode/distortion/meu-novo-plugin/dsp/wrapper.cpp:44)
- `ParamMapUI` stores slider labels to `float*` zones: [dsp/wrapper.cpp](/home/jao/VSCode/distortion/meu-novo-plugin/dsp/wrapper.cpp:49)
- `FaustInstance` owns `mydsp*` and `ParamMapUI*`: [dsp/wrapper.cpp](/home/jao/VSCode/distortion/meu-novo-plugin/dsp/wrapper.cpp:69)
- C ABI functions: `faust_create`, `faust_init`, `faust_process`, `faust_destroy`: [dsp/wrapper.cpp](/home/jao/VSCode/distortion/meu-novo-plugin/dsp/wrapper.cpp:74)
- Param setters use label lookup through the `SET_PARAM` macro: [dsp/wrapper.cpp](/home/jao/VSCode/distortion/meu-novo-plugin/dsp/wrapper.cpp:110)
- Header declares opaque `FaustHandle` and exported functions: [dsp/wrapper.h](/home/jao/VSCode/distortion/meu-novo-plugin/dsp/wrapper.h:11)

Rust bridge:

- Includes bindgen-generated bindings from `$OUT_DIR/bindings_faust.rs`: [src/bridge/faust.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bridge/faust.rs:6)
- Wraps `FaustHandle` in `FaustProcessor`: [src/bridge/faust.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bridge/faust.rs:11)
- Calls the C ABI setters in `set_eq_params()`: [src/bridge/faust.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bridge/faust.rs:25)

### Parameter Management

`InputSelect` is a NIH enum param with display names: [src/core/state/plugin_params.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/state/plugin_params.rs:7)

`BaseIOParams` derives `Params` and contains:

- Persisted editor state: [src/core/state/plugin_params.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/state/plugin_params.rs:19)
- `EnumParam<InputSelect>`: [src/core/state/plugin_params.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/state/plugin_params.rs:22)
- Global gain and bypass: [src/core/state/plugin_params.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/state/plugin_params.rs:25)
- Neural volume/drive/output gain/active: [src/core/state/plugin_params.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/state/plugin_params.rs:31)
- Cabinet bypass/level/mix and persisted active hash: [src/core/state/plugin_params.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/state/plugin_params.rs:44)
- EQ active and 9 EQ float params: [src/core/state/plugin_params.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/state/plugin_params.rs:59)

`EditorState` is not a plugin param set; it is UI state passed to Egui and contains params, analyzer DSP, consumer, active panel, cabinet library/mailbox/sample-rate/max-block/error handles: [src/core/state/plugin_params.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/state/plugin_params.rs:85)

`Default for BaseIOParams` builds each `FloatParam` with:

- `FloatParam::new("Name", default, FloatRange::...)`
- `.with_smoother(SmoothingStyle::...)`
- optional `.with_unit(...)`
- optional display/input formatters

Examples:

- Gain: [src/core/state/plugin_params.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/state/plugin_params.rs:109)
- Neural drive: [src/core/state/plugin_params.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/state/plugin_params.rs:140)
- Cabinet mix: [src/core/state/plugin_params.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/state/plugin_params.rs:187)
- EQ params: [src/core/state/plugin_params.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/state/plugin_params.rs:202)

### FFT Spectrum Analyzer

`AnalyzerDsp` owns:

- `fft_plan: Arc<dyn realfft::RealToComplex<f32>>`
- input/output buffers
- smoothed `spectrum`
- rolling `audio_history`

Location: [src/core/dsp/analyzer.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/dsp/analyzer.rs:8)

Architecture:

- `FFT_SIZE = 2048`: [src/core/dsp/analyzer.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/dsp/analyzer.rs:6)
- Audio thread pushes post-DSP mono mix through `analyzer_producer`: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:692)
- UI drains the `Consumer<f32>`: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:164), [src/bin/standalone.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bin/standalone.rs:850)
- Analyzer keeps the newest `FFT_SIZE` samples: [src/core/dsp/analyzer.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/dsp/analyzer.rs:39)
- Applies a Blackman-Harris-style window: [src/core/dsp/analyzer.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/dsp/analyzer.rs:47)
- Uses fixed `sample_rate = 48000.0` for bin frequency display and tilt: [src/core/dsp/analyzer.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/dsp/analyzer.rs:55)
- Smooths spectrum bins with `prev * 0.85 + db * 0.15`: [src/core/dsp/analyzer.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/dsp/analyzer.rs:68)

## 3. CONVENTIONS

### Naming

- Rust files/modules use `snake_case`: `plugin_params.rs`, `signal_chain.rs`, `cabinet_panel.rs`.
- Types use `PascalCase`: `BaseIO`, `BaseIOParams`, `EditorState`, `InputSelect`, `AnalyzerDsp`, `FaustProcessor`, `MojoProcessor`, `CabinetEngine`.
- Functions and fields use `snake_case`: `decode_wav_flat`, `open_cabinet_library`, `process_block`, `eq_low_freq`, `neural_output_gain`.
- Constants use `SCREAMING_SNAKE_CASE`: `APP_NAME`, `DEFAULT_CABINET_IR`, `DEFAULT_PRE_EQ_IR`, `FFT_SIZE`.
- Faust labels use title-style UI strings: `EQ Low Freq`, `EQ Mid Gain`, `EQ High Q`: [dsp/main.dsp](/home/jao/VSCode/distortion/meu-novo-plugin/dsp/main.dsp:9)

### Import Organization

Imports are grouped loosely:

- External crates first, then std, then local modules in `src/lib.rs`: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:1)
- `src/bin/standalone.rs` imports CPAL/eframe/rtrb/std/fft/local modules in a pragmatic order: [src/bin/standalone.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bin/standalone.rs:1)
- No enforced alphabetical sorting is visible.

### Error Handling

Patterns:

- Build-time hard failures with `panic!`/`expect()` for missing Faust/Mojo and compile failures: [build.rs](/home/jao/VSCode/distortion/meu-novo-plugin/build.rs:31), [build.rs](/home/jao/VSCode/distortion/meu-novo-plugin/build.rs:77)
- Runtime initialization often degrades to fallback: cabinet library falls back to in-memory DB if data dir open fails: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:45)
- Audio DSP avoids panics and uses pass-through fallback on convolution error: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:624), [src/core/cabinet/engine.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/cabinet/engine.rs:262)
- UI/cabinet operations capture errors in `cabinet_error` for display rather than failing hard: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:336)
- Standalone CPAL setup reports errors through `AudioEvent::StreamStarted(Result<(), String>)`: [src/bin/standalone.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bin/standalone.rs:216)

### Parameter Smoothing

- Gain parameters generally use `SmoothingStyle::Logarithmic(50.0)`: [src/core/state/plugin_params.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/state/plugin_params.rs:118), [src/core/state/plugin_params.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/state/plugin_params.rs:149)
- Linear or percentage style parameters use `SmoothingStyle::Linear(50.0)`: [src/core/state/plugin_params.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/state/plugin_params.rs:192), [src/core/state/plugin_params.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/state/plugin_params.rs:204)
- `BaseIO::process()` samples smoothed values once per process block, not per sample: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:553)
- Standalone does not use NIH smoothing; it reads copied floats from `StandaloneState`/`AudioSnapshot`: [src/bin/standalone.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bin/standalone.rs:456)

### Comments and Documentation Style

- Comments are bilingual Portuguese/English.
- Audio pipeline comments are direct and stage-labeled: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:575), [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:603)
- Cabinet subsystem has stronger Rustdoc-style contracts around audio-thread safety: [src/core/cabinet/engine.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/cabinet/engine.rs:10), [src/core/cabinet/runtime.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/cabinet/runtime.rs:7)
- There are stale comments/labels: active neural path is Mojo tanh, but UI says `Neural Amp (PyTorch)`: [src/core/ui/main_view.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/ui/main_view.rs:64)

## 4. STRUCTURE

### Directory Tree, 3 Levels Deep

```text
/home/jao/VSCode/distortion/meu-novo-plugin
├── Cargo.toml
├── Cargo.lock
├── Makefile
├── build.rs
├── bundler.toml
├── docs
│   ├── ARCHITECTURE_OVERVIEW.md
│   ├── PLAN-bypass-system.md
│   ├── PLAN-cabinet-simulator.md
│   ├── PLAN-core-architecture.md
│   ├── PLAN-ir-loader.md
│   ├── PLAN-libtorch-integration.md
│   ├── PLAN-signal-chain-ui.md
│   ├── VERIFICATION.md
│   └── spec-cabinet-ir.md
├── dsp
│   ├── FaustModule.hpp
│   ├── main.dsp
│   ├── wrapper.cpp
│   └── wrapper.h
├── faust-ddsp
│   ├── diff.lib
│   └── filters.lib
├── neural
│   ├── drive
│   │   ├── cabinet_ir.wav
│   │   ├── pre_eq_ir.wav
│   │   ├── wavenet_drive.onnx
│   │   └── wavenet_drive.onnx.data
│   ├── libneural.so
│   └── main.mojo
├── scripts
│   ├── check_env.sh
│   └── run_standalone.sh
├── src
│   ├── bin
│   │   └── standalone.rs
│   ├── bridge
│   │   ├── faust.rs
│   │   ├── mod.rs
│   │   ├── mojo.rs
│   │   └── wavenet.rs
│   ├── core
│   │   ├── cabinet
│   │   ├── dsp
│   │   ├── state
│   │   └── ui
│   ├── lib.rs
│   └── models
├── target
└── xtask
    ├── Cargo.toml
    └── src
```

### Capability to File Map

| Capability | Location |
| --- | --- |
| Plugin entry, CLAP/VST3 export, plugin DSP chain | [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:1) |
| Standalone app, CPAL routing, standalone DSP | [src/bin/standalone.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bin/standalone.rs:1) |
| External DSP trait | [src/bridge/mod.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bridge/mod.rs:5) |
| Faust Rust wrapper | [src/bridge/faust.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bridge/faust.rs:1) |
| Mojo Rust wrapper | [src/bridge/mojo.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bridge/mojo.rs:1) |
| Legacy/optional WaveNet ONNX bridge | [src/bridge/wavenet.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bridge/wavenet.rs:1) |
| Plugin params and editor state | [src/core/state/plugin_params.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/state/plugin_params.rs:1) |
| FFT analyzer DSP | [src/core/dsp/analyzer.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/dsp/analyzer.rs:1) |
| Shared UI module exports | [src/core/ui/mod.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/ui/mod.rs:1) |
| Shared panel layout | [src/core/ui/main_view.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/ui/main_view.rs:1) |
| Signal-chain node UI and `ActivePanel` | [src/core/ui/signal_chain.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/ui/signal_chain.rs:1) |
| Spectrum drawing | [src/core/ui/spectrum.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/ui/spectrum.rs:1) |
| Cabinet panel UI | [src/core/ui/cabinet_panel.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/ui/cabinet_panel.rs:1) |
| Faust DSP source | [dsp/main.dsp](/home/jao/VSCode/distortion/meu-novo-plugin/dsp/main.dsp:1) |
| Faust C ABI wrapper | [dsp/wrapper.cpp](/home/jao/VSCode/distortion/meu-novo-plugin/dsp/wrapper.cpp:1), [dsp/wrapper.h](/home/jao/VSCode/distortion/meu-novo-plugin/dsp/wrapper.h:1) |
| Mojo neural source | [neural/main.mojo](/home/jao/VSCode/distortion/meu-novo-plugin/neural/main.mojo:1) |
| Build orchestration | [build.rs](/home/jao/VSCode/distortion/meu-novo-plugin/build.rs:1), [Makefile](/home/jao/VSCode/distortion/meu-novo-plugin/Makefile:1) |

### Key File Purposes

- `src/lib.rs`: plugin target, editor construction, initialization, DSP chain, plugin exports.
- `src/bin/standalone.rs`: eframe app, CPAL host/device routing, standalone DSP callback, shared UI integration.
- `src/bridge/mod.rs`: common zero-copy processor trait.
- `src/bridge/faust.rs`: safe-ish Rust wrapper around generated Faust C ABI.
- `src/bridge/mojo.rs`: Rust wrapper around Mojo shared library.
- `src/bridge/wavenet.rs`: ONNX WaveNet processor, currently not wired into `BaseIO`.
- `src/core/state/plugin_params.rs`: NIH parameter definitions and UI state.
- `src/core/dsp/analyzer.rs`: FFT analyzer state and processing.
- `src/core/ui/*`: shared Egui UI components used by plugin and standalone.
- `dsp/main.dsp`: Faust EQ/tanh source.
- `dsp/wrapper.cpp/h`: C ABI surface for Faust processor.
- `neural/main.mojo`: exported Mojo tanh processor.

## 5. TESTING

### Tests Found

There are no tests in the requested core plugin/bridge/UI/standalone files, but the cabinet subsystem has tests.

Tests in [src/core/cabinet/engine.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/cabinet/engine.rs:327):

- `passthrough_when_no_runtime`: [src/core/cabinet/engine.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/cabinet/engine.rs:338)
- `installs_runtime_and_produces_finite_output`: [src/core/cabinet/engine.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/cabinet/engine.rs:351)
- `full_bypass_returns_dry`: [src/core/cabinet/engine.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/cabinet/engine.rs:369)
- `clear_removes_active_runtime`: [src/core/cabinet/engine.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/cabinet/engine.rs:388)
- `ir_switch_stays_finite`: [src/core/cabinet/engine.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/cabinet/engine.rs:423)

Tests in [src/core/cabinet/library.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/cabinet/library.rs:296):

- `rejects_non_mono_stereo`: [src/core/cabinet/library.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/cabinet/library.rs:327)
- `rejects_truncated_wav`: [src/core/cabinet/library.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/cabinet/library.rs:335)
- `seed_lists_and_selects_default`: [src/core/cabinet/library.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/cabinet/library.rs:344)
- `integrity_and_dedup_and_rename`: [src/core/cabinet/library.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/cabinet/library.rs:357)
- `delete_clears_selection`: [src/core/cabinet/library.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/cabinet/library.rs:377)
- `size_guard_rejects_oversized`: [src/core/cabinet/library.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/cabinet/library.rs:386)
- `runtime_builds_from_stored_bytes`: [src/core/cabinet/library.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/cabinet/library.rs:393)

### Test Commands in Makefile

No explicit `cargo test` target exists. `make run` is described as development/testing but launches the standalone host: [Makefile](/home/jao/VSCode/distortion/meu-novo-plugin/Makefile:18), [Makefile](/home/jao/VSCode/distortion/meu-novo-plugin/Makefile:64)

### Coverage Assessment

Current automated coverage is narrow:

- Cabinet library/runtime behavior has unit coverage.
- Core plugin audio chain has no automated test harness.
- Faust FFI and Mojo FFI have no direct tests.
- Standalone CPAL routing has no automated tests.
- UI behavior has no automated tests.
- Parameter automation/smoothing behavior is not tested.

I did not run tests during this research task; this report is based on static repository reading.

## 6. INTEGRATIONS

### Faust Compiler Integration

`build.rs` checks Faust availability and recompiles the Faust module if `dsp/main.dsp` is newer than `dsp/FaustModule.hpp`: [build.rs](/home/jao/VSCode/distortion/meu-novo-plugin/build.rs:31), [build.rs](/home/jao/VSCode/distortion/meu-novo-plugin/build.rs:63)

The Makefile also provides `make build-faust`: [Makefile](/home/jao/VSCode/distortion/meu-novo-plugin/Makefile:32)

### Mojo Compiler Integration

`build.rs` locates Mojo, checks `neural/main.mojo` against `neural/libneural.so`, and builds a shared library if needed: [build.rs](/home/jao/VSCode/distortion/meu-novo-plugin/build.rs:83)

The Makefile also provides `make build-mojo`, preferring `./.venv/bin/mojo` if present: [Makefile](/home/jao/VSCode/distortion/meu-novo-plugin/Makefile:45)

### FFTConvolver Usage

- Pre-EQ IR: plugin initializes two `FFTConvolver<f32>` instances from embedded `DEFAULT_PRE_EQ_IR`: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:487)
- Standalone mirrors that setup with local `pre_eq_l`/`pre_eq_r`: [src/bin/standalone.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bin/standalone.rs:406)
- Cabinet runtime builds left/right convolution engines from WAV IR data: [src/core/cabinet/runtime.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/cabinet/runtime.rs:47)

### `nih_plug` Framework Integration

- `Plugin` implementation defines metadata, I/O layout, MIDI config, params, editor, initialize, reset, process: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:118)
- Stereo audio layout is enforced: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:125)
- Sample-accurate automation enabled: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:135)
- CLAP features: audio effect and stereo: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:706)
- VST3 subcategory: Analyzer: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:714)

### CPAL Audio I/O

Standalone enumerates hosts/devices, builds input and output streams, and supports selectable physical channels:

- Device discovery: [src/bin/standalone.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bin/standalone.rs:300)
- Apply routing command includes host, input/output names/channels, and buffer size: [src/bin/standalone.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bin/standalone.rs:205)
- Input stream only supports F32 for DSP parity; non-F32 input is rejected: [src/bin/standalone.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bin/standalone.rs:555)
- Output stream supports F32 and I16 paths: [src/bin/standalone.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bin/standalone.rs:597)

### egui/eframe UI

- Plugin editor uses `nih_plug_egui::create_egui_editor`: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:147)
- Standalone uses `eframe::run_native`: [src/bin/standalone.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bin/standalone.rs:1418)
- Shared panels are closure-injected so plugin and standalone provide their own parameter mutation logic: [src/core/ui/main_view.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/ui/main_view.rs:34)
- Signal-chain UI owns active panel selection and per-module power toggles: [src/core/ui/signal_chain.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/ui/signal_chain.rs:12)

### Bindgen FFI Generation

`bindgen::Builder` uses `dsp/wrapper.h`, allowlists `faust_.*`, and writes `bindings_faust.rs` to `OUT_DIR`: [build.rs](/home/jao/VSCode/distortion/meu-novo-plugin/build.rs:112)

Rust includes that generated file with:

```rust
include!(concat!(env!("OUT_DIR"), "/bindings_faust.rs"));
```

Location: [src/bridge/faust.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bridge/faust.rs:6)

## 7. CONCERNS

### Disk Space and Large Artifacts

Observed disk usage during research:

- Repository total: about `3.6G`
- `target`: about `1.9G`
- `.venv`: about `1.8G`
- Generated/local artifacts include `neural/libneural.so`, `wrapper.o`, generated `dsp/FaustModule.hpp`, ONNX files, and large build products.

Concern: the project can consume several GB locally, and generated artifacts make clean research/build states harder to reason about.

### Build Complexity

This is a 3-language DSP stack:

- Rust for plugin/standalone/control/UI.
- Faust -> generated C++ -> C ABI -> bindgen -> Rust.
- Mojo -> shared library -> Rust FFI.

`build.rs` hard-requires Faust and Mojo even if a developer only wants to inspect or run Rust tests. That increases setup friction: [build.rs](/home/jao/VSCode/distortion/meu-novo-plugin/build.rs:31), [build.rs](/home/jao/VSCode/distortion/meu-novo-plugin/build.rs:38)

### Test Gaps

Cabinet has tests, but the core audio path does not. The riskiest untested surfaces are:

- `BaseIO::process()` ordering and bypass behavior.
- Faust FFI parameter label mapping.
- Mojo FFI safety and DSP output.
- Plugin/standalone parity.
- Standalone CPAL routing.

### Stale or Conflicting Neural Architecture

Active comments say Mojo replaces ONNX/tract inference: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:58). But `tract-onnx` and `bridge/wavenet.rs` remain, and UI still says `Neural Amp (PyTorch)`: [src/core/ui/main_view.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/ui/main_view.rs:64)

Concern: future amp-model work could accidentally build on the wrong neural path.

### FFI Fragility

- Raw pointer contracts rely on contiguous buffers and correct `length`: [src/bridge/mod.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bridge/mod.rs:10)
- Faust process passes the same mono buffer as both inputs and outputs even though generated DSP is stereo-shaped: [dsp/wrapper.cpp](/home/jao/VSCode/distortion/meu-novo-plugin/dsp/wrapper.cpp:89)
- `unsafe impl Send for FaustProcessor` assumes host scheduling is safe for opaque Faust handles: [src/bridge/faust.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bridge/faust.rs:52)

### Standalone/Plugin Parity Drift

The plugin uses NIH params and smoothing; standalone uses immediate values from `StandaloneState`. Any new amp parameter must be added in both places or the two targets drift.

Plugin param source: [src/core/state/plugin_params.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/state/plugin_params.rs:17)

Standalone state source: [src/bin/standalone.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bin/standalone.rs:40)

### Hardcoded Analyzer Sample Rate

Analyzer uses `48000.0` internally for frequency bin display/tilt rather than host sample rate: [src/core/dsp/analyzer.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/dsp/analyzer.rs:55)

## Critical Questions

### Q1: How does the Mojo neural drive work? What parameters does it accept? Show the FFI signature.

Rust FFI signature:

```rust
extern "C" {
    fn mojo_init(sample_rate: f32);
    fn mojo_process_block(address: usize, size: usize, drive: f32, output_gain: f32);
}
```

Source: [src/bridge/mojo.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bridge/mojo.rs:3)

Mojo exported signature:

```mojo
@export
fn mojo_process_block(address: Int, size: Int, drive: Float32, output_gain: Float32):
```

Source: [neural/main.mojo](/home/jao/VSCode/distortion/meu-novo-plugin/neural/main.mojo:16)

Parameters:

- `address`: memory address of the Rust `*mut f32` buffer.
- `size`: sample count.
- `drive`: pre-gain.
- `output_gain`: post-saturation gain.

Processing:

1. Reconstructs `UnsafePointer[Float32, MutAnyOrigin]` from `address`: [neural/main.mojo](/home/jao/VSCode/distortion/meu-novo-plugin/neural/main.mojo:30)
2. For each sample: `x = data[i] * drive`: [neural/main.mojo](/home/jao/VSCode/distortion/meu-novo-plugin/neural/main.mojo:32)
3. Clamps `x` to `[-4.0, 4.0]`: [neural/main.mojo](/home/jao/VSCode/distortion/meu-novo-plugin/neural/main.mojo:34)
4. Applies polynomial tanh approximation: `x * (27.0 + x2) / (27.0 + 9.0 * x2)`: [neural/main.mojo](/home/jao/VSCode/distortion/meu-novo-plugin/neural/main.mojo:39)
5. Writes `saturated * output_gain` back in place: [neural/main.mojo](/home/jao/VSCode/distortion/meu-novo-plugin/neural/main.mojo:42)

Rust wrapper state:

- `MojoProcessor` stores `is_ready`, `drive`, `output_gain`: [src/bridge/mojo.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bridge/mojo.rs:13)
- `set_drive()` and `set_output_gain()` update local fields only: [src/bridge/mojo.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bridge/mojo.rs:28)
- `process_block()` returns early if not initialized: [src/bridge/mojo.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bridge/mojo.rs:55)

### Q2: What is the EXACT pattern for adding a new `FloatParam` in `plugin_params.rs`?

Pattern has two required parts.

First, add a field to `BaseIOParams` with a stable NIH parameter id:

```rust
#[id = "new_param_id"]
pub new_param: FloatParam,
```

Existing examples: [src/core/state/plugin_params.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/state/plugin_params.rs:25), [src/core/state/plugin_params.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/state/plugin_params.rs:35), [src/core/state/plugin_params.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/state/plugin_params.rs:63)

Second, initialize the field in `impl Default for BaseIOParams`:

```rust
new_param: FloatParam::new(
    "New Param Label",
    default_value,
    FloatRange::Linear { min, max },
)
.with_smoother(SmoothingStyle::Linear(50.0))
.with_unit(" unit")
.with_value_to_string(...)
.with_string_to_value(...),
```

Gain-style exact pattern:

```rust
neural_drive: FloatParam::new(
    "Neural Drive",
    util::db_to_gain(0.0),
    FloatRange::Skewed {
        min: util::db_to_gain(0.0),
        max: util::db_to_gain(30.0),
        factor: FloatRange::gain_skew_factor(0.0, 30.0),
    },
)
.with_smoother(SmoothingStyle::Logarithmic(50.0))
.with_unit(" dB")
.with_value_to_string(formatters::v2s_f32_gain_to_db(2))
.with_string_to_value(formatters::s2v_f32_gain_to_db()),
```

Source: [src/core/state/plugin_params.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/state/plugin_params.rs:140)

Then read the smoothed value in `BaseIO::process()` if it affects DSP:

```rust
let new_param = self.params.new_param.smoothed.next();
```

Existing pattern: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:553)

For plugin UI, use the existing `make_knob(ui, &state.params.some_float_param)` closure pattern: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:215)

For standalone parity, add the field to `StandaloneState`, `AudioSnapshot`, default values, UI controls, and DSP callback reads: [src/bin/standalone.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bin/standalone.rs:40), [src/bin/standalone.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bin/standalone.rs:95)

### Q3: How does the Faust EQ integrate? What is the pattern for adding a new Faust-based processor?

Current integration:

1. Faust source defines slider labels and DSP graph in `dsp/main.dsp`: [dsp/main.dsp](/home/jao/VSCode/distortion/meu-novo-plugin/dsp/main.dsp:8)
2. `build.rs` compiles `main.dsp` to `FaustModule.hpp`: [build.rs](/home/jao/VSCode/distortion/meu-novo-plugin/build.rs:72)
3. `wrapper.cpp` includes `FaustModule.hpp`: [dsp/wrapper.cpp](/home/jao/VSCode/distortion/meu-novo-plugin/dsp/wrapper.cpp:44)
4. `ParamMapUI` captures slider labels to raw parameter zones: [dsp/wrapper.cpp](/home/jao/VSCode/distortion/meu-novo-plugin/dsp/wrapper.cpp:49)
5. `wrapper.h` exposes C ABI functions: [dsp/wrapper.h](/home/jao/VSCode/distortion/meu-novo-plugin/dsp/wrapper.h:14)
6. `build.rs` uses bindgen to generate Rust bindings for `faust_.*`: [build.rs](/home/jao/VSCode/distortion/meu-novo-plugin/build.rs:112)
7. `FaustProcessor` includes generated bindings and wraps create/init/process/destroy: [src/bridge/faust.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bridge/faust.rs:6)
8. `BaseIO::process()` sets params and calls `faust.process_block(...)`: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:603)

Pattern for adding a new Faust-based processor:

- Add or modify Faust controls in `.dsp` with stable labels.
- Regenerate `FaustModule.hpp` through build.
- Add C ABI setter declarations to `wrapper.h`.
- Add setter implementations in `wrapper.cpp` using `SET_PARAM("Exact Faust Label", value)`.
- Add Rust wrapper methods in `src/bridge/faust.rs`.
- Add plugin params and read smoothed values in `BaseIO::process()`.
- Wire the stage at the intended location in `BaseIO::process()`.
- Duplicate state/UI/DSP wiring in `src/bin/standalone.rs` if standalone parity is required.

### Q4: Show the EXACT signal chain structure in `BaseIO::process()` - where would a new amp stage be inserted?

Exact current structure:

1. Read params: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:550)
2. Set latency to zero: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:572)
3. Input routing before DSP: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:575)
4. Get mutable channel slices: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:594)
5. If not bypassed:
   - Per-channel Faust EQ: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:603)
   - Per-channel pre-EQ convolver: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:621)
   - Per-channel Mojo neural drive: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:629)
   - Stereo cabinet IR: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:639)
   - Master gain and neural master volume: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:670)
6. NaN/Inf sanitization always: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:683)
7. Analyzer push: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:692)

A new amp/preamp stage should be inserted where the current Mojo neural drive lives, after pre-EQ and before cabinet:

- After pre-EQ block ending at [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:627)
- Before cabinet block beginning at [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:639)

If replacing Mojo, update the block at [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:629). If adding an additional amp stage, insert directly after [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:634).

### Q5: How does the standalone binary's DSP pipeline differ from the plugin's `BaseIO::process`?

Standalone differences:

- Uses `StandaloneState`/`AudioSnapshot` plain fields instead of NIH params: [src/bin/standalone.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bin/standalone.rs:40), [src/bin/standalone.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bin/standalone.rs:95)
- No `FloatParam` smoothing. Values are copied from mutex state each callback: [src/bin/standalone.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bin/standalone.rs:456)
- Uses CPAL input stream data, extracts selected hardware input channels into `buf_l` and `buf_r`: [src/bin/standalone.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bin/standalone.rs:453), [src/bin/standalone.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bin/standalone.rs:463)
- Runs local Faust/Mojo/convolver instances created inside stream setup: [src/bin/standalone.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bin/standalone.rs:392), [src/bin/standalone.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bin/standalone.rs:402)
- Uses separate input and output CPAL streams with an `rtrb` bridge between them: [src/bin/standalone.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bin/standalone.rs:356), [src/bin/standalone.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bin/standalone.rs:541), [src/bin/standalone.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bin/standalone.rs:601)
- Rejects non-F32 input formats instead of silently bypassing DSP: [src/bin/standalone.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bin/standalone.rs:555)
- Output path supports F32 and I16 output, with I16 output currently writing mono duplicated PCM from one popped sample at a time: [src/bin/standalone.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bin/standalone.rs:615)

Logical DSP order is intentionally similar:

- Faust EQ: [src/bin/standalone.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bin/standalone.rs:470)
- Pre-EQ convolver: [src/bin/standalone.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bin/standalone.rs:491)
- Mojo neural drive: [src/bin/standalone.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bin/standalone.rs:504)
- Cabinet IR: [src/bin/standalone.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bin/standalone.rs:515)
- Master gain and neural volume: [src/bin/standalone.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bin/standalone.rs:526)
- NaN/Inf sanitization: [src/bin/standalone.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bin/standalone.rs:537)

### Q6: Is there any existing toggle/selector/enum pattern for switching between processing modes?

Yes.

1. `EnumParam<InputSelect>`:
   - Enum definition with NIH display names: [src/core/state/plugin_params.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/state/plugin_params.rs:7)
   - Param field: [src/core/state/plugin_params.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/state/plugin_params.rs:22)
   - ComboBox UI and setter pattern: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:176)
   - DSP match: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:583)

2. `BoolParam` toggles:
   - `bypass`, `neural_amp_active`, `cabinet_bypass`, `eq_active`: [src/core/state/plugin_params.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/state/plugin_params.rs:28), [src/core/state/plugin_params.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/state/plugin_params.rs:41), [src/core/state/plugin_params.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/state/plugin_params.rs:45), [src/core/state/plugin_params.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/state/plugin_params.rs:60)
   - UI toggles use `begin_set_parameter`, `set_parameter`, `end_set_parameter`: [src/lib.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/lib.rs:199)

3. UI-only enum:
   - `ActivePanel::{None, EQ, NeuralAmp, Cabinet}` selects which bottom panel is open: [src/core/ui/signal_chain.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/ui/signal_chain.rs:3)
   - Node click toggles active panel: [src/core/ui/signal_chain.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/core/ui/signal_chain.rs:163)

4. Standalone selectors:
   - CPAL host/device/channel selection uses Egui `ComboBox` and plain app fields: [src/bin/standalone.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bin/standalone.rs:891), [src/bin/standalone.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bin/standalone.rs:916), [src/bin/standalone.rs](/home/jao/VSCode/distortion/meu-novo-plugin/src/bin/standalone.rs:940)

# MLC ZERO V SIGNATURE RESEARCH REPORT

Research date: 2026-07-06

Primary source: official MLC product page for `S_ZERO V Vogg Amplifier (Decapitated) signature`: https://mlcamps.com/en/amplifiers/14-szero-v-vogg-amplifier-decapitated-signature.html

Additional sources:

- Official MLC front/control images embedded in the product page:
  - Clean controls: https://mlcamps.com/img/cms/Products/clean-channel-e1656150582649.jpeg
  - Drive I controls: https://mlcamps.com/img/cms/Products/drive-channel1-e1656148311573.jpeg
  - Drive II controls: https://mlcamps.com/img/cms/Products/drive-channel2-e1656150067802.jpeg
  - Presence/depth controls: https://mlcamps.com/img/cms/Products/presence-depth-knobs-3-e1656156440505.jpeg
  - Master controls: https://mlcamps.com/img/cms/Products/master-knobs-1-e1656156406815.jpeg
  - MIDI controls: https://mlcamps.com/img/cms/Products/midi-control-e1656156340702.jpeg
  - FX loop controls: https://mlcamps.com/img/cms/Products/fx-loop-e1656156377894.jpeg
  - Voltage/rear power panel: https://mlcamps.com/img/cms/Products/volt-inputs.jpeg
- Reverb sold listing for `MLC Subzero V Vogg signature high gain amplifier`: https://reverb.com/item/66076732-mlc-subzero-v-vogg-signature-high-gain-amplifier
- Rig-Talk classified listing, useful only as non-official confirmation of gate/loop/footswitch/manual existence: https://www.rig-talk.com/forum/threads/mlc-vogg-sub-zero-v.333303/
- MLC old Subzero 100 page for sibling-family back-panel/impedance context, not treated as definitive S_ZERO V documentation: https://www.old.mlcamps.com/guitar-amps/subzero-100/

Important uncertainty note: I found no public downloadable S_ZERO V manual or schematic. Tube preamp complement, exact speaker-output jack labeling, and exact internal tone-stack topology were not publicly documented in sources I could verify. Those items are marked as unknown where appropriate.

## 1. Controls

### Front Panel Knobs

Exact labels visible in official front/control images:

Clean section:

- `GAIN`
- `CLEAN`
- `BASS`
- `MIDDLE`
- `TREBLE`

Drive I section:

- `GAIN`
- `DRIVE I`
- `BASS`
- `MIDDLE`
- `TREBLE`

Drive II section:

- `GAIN`
- `DRIVE II`
- Uses the same shared drive tone regulation documented for Drive I/Drive II: `BASS`, `MIDDLE`, `TREBLE`

Power amp/master section:

- `PRESENCE`
- `DEPTH`
- `MASTER I`
- `MASTER II`
- Rear-panel `GATE` knob is visible in the rear overview image, and official text describes a switchable dynamic gate.
- Rear-panel `FX LOOP LEVEL` knob is visible in the official FX loop close-up.

Input/channel:

- `CHANNEL` rotary selector on the front panel.
- `INPUT` jack on the front panel.

### Front Panel Switches and Buttons

Exact labels visible in official close-up images:

Clean section switches:

- `LO EQ`
- `I - BRIGHT - II`

Drive section switches:

- `I - BRIGHT - II`
- `OFF - M45 - ON`
- another `I - BRIGHT - II`

Power amp switch:

- `FEED LO - BACK - HI`

Channel/power toggles:

- The official front overview shows two left-side toggle switches with labels around standby/anode functions. The image resolution makes exact punctuation difficult to verify, so I would not encode those labels into a UI without a manual or direct owner photo.

### Rear Panel Controls and Labels

FX loop close-up exact labels:

- `FX LOOP`
- `SEND`
- `RETURN`
- `LEVEL`
- `ON LOOP`
- `NON MASTER`
- `LOOP OFF`
- `LOOP SWITCHED`

MIDI close-up exact labels:

- `MIDI CONTROL`
- `THRU`
- `INPUT`
- `CHANNEL`

Voltage/power close-up exact labels:

- `CAUTION SHOCK HAZARD`
- `WARNING PRE-SET VOLTAGE`
- `230V`
- `115V`
- `ANODE FUSE 1AT`
- `MAINS/FUSES`
- `230V/2AT`
- `115V/4AT`
- `SERIAL No.:`

### Gain/Drive Controls

- Clean has `GAIN` and `CLEAN` level/volume style controls.
- Drive I has `GAIN` and `DRIVE I`.
- Drive II has `GAIN` and `DRIVE II`.
- Official text describes the three independent channels and WARCLAW saturation switching as providing different tonal spectrums.

### EQ Controls

- Clean channel: `BASS`, `MIDDLE`, `TREBLE`, `LO EQ`, `I - BRIGHT - II`.
- Drive channels: shared/common `BASS`, `MIDDLE`, `TREBLE`; `I - BRIGHT - II`; `OFF - M45 - ON`.
- Power amp: `PRESENCE`, `DEPTH`, `FEED LO - BACK - HI`.

### Master/Volume Controls

- `MASTER I`
- `MASTER II`
- Official source: two independent master volumes controlled by MIDI or footswitch.

### Built-In Boost/Gate

- Official text: `WARCLAW` saturation switch/mod and switchable ultra-transparent dynamic `GATE`.
- Reverb listing: adjustable gate and built-in Warclaw boost mode.

## 2. Channels

Officially documented channels:

1. `CLEAN`
2. `DRIVE I`
3. `DRIVE II`

The official page says the amp has three independent channels, and it documents each one separately.

Channel behavior:

- `CLEAN`: American-voiced clean with punchy bottom end and high headroom. Turning the knob farther right pushes into crunch/medium gain. It responds well to pedals.
- `DRIVE I`: 2-stage hot-rodded `PLEXI 1968` voice, vintage British style with more gain, pedal-friendly.
- `DRIVE II`: 3-stage hot-rodded `PLEXI 1968` ultimate high-gain modded British tone, thick/juicy gain, rich harmonics, long sustain.

Switching/control:

- Official product page states that channel switching and functions can be operated by MIDI CC or MIDI PC over 16 MIDI channels.
- Official product page states `MASTER I & II`, gate function, programmable amp mute, and WARCLAW mod are controlled by MIDI footswitch.
- Reverb listing says the amp comes with a footswitch.

## 3. Tone Stack

### Documented Tone Stack Features

Clean channel:

- Three-band EQ: `BASS`, `MIDDLE`, `TREBLE`.
- Extra low-end shaping: `LO EQ`.
- Brightness switch: `I - BRIGHT - II`.

Drive I and Drive II:

- Official text: common tone regulation with `DRIVE I`/`DRIVE II`: `Bass`, `Middle`, `Treble`.
- Brightness switches: `I - BRIGHT - II`.
- `OFF - M45 - ON` switch: official text says when M45 mod is used, drive channels are lower gain. For Drive I it is described as more Plexi-like; for Drive II it is described as less gain like a JCM sound.

Power amp tone:

- `PRESENCE`: high frequency response control.
- `DEPTH`: low frequency response shaping in the power amp stage.
- `FEED LO - BACK - HI`: feedback voicing switch. Official text says HI is more closed, LO is farther/darker.

### Topology Assessment

The public sources do not specify whether the tone stack is passive, active, or a specific named circuit. Based only on documented controls, the practical model for feature planning should be:

- Clean 3-band tone stack plus low EQ and bright mode.
- Shared drive 3-band tone stack plus bright and M45 voicing.
- Power amp presence, depth, and feedback voicing.

Do not assume parametric mids; no source showed a parametric mid control.

## 4. Preamp / Power Amp

### Documented Power Amp

- 100-watt tube head.
- Official page says it is based on aggressive/dynamic 6L6 tube sound.
- Reverb listing specifies `4 6L6`.

### Documented Preamp/Gain Stages

Official page:

- `DRIVE I`: 2-stage hot-rodded `PLEXI 1968`, vintage British style with more gain.
- `DRIVE II`: 3-stage hot-rodded `PLEXI 1968`, ultimate high-gain modded British tone.

### Tube Complement

Known:

- Power tubes: 6L6, with Reverb listing specifying `4 6L6`.

Unknown:

- Preamp tube complement was not found in official public sources.
- Rectifier type was not found.

### Clipping / Saturation Character

Documented sources support this practical model:

- Drive channels are hot-rodded Plexi-derived cascaded gain-stage designs.
- Drive II is the higher-gain channel with rich harmonics and sustain.
- WARCLAW saturation/boost mode changes tonal spectrum and gain feel.
- Dynamic gate supports tight modern high-gain playing.
- 6L6 power section, depth, presence, and feedback switch shape the tight/aggressive power amp response.

## 5. Unique Features

Unique and signature features documented:

- `WARCLAW` saturation switch/mod.
- Built-in Warclaw boost mode, per Reverb listing.
- Switchable ultra-transparent dynamic `GATE`.
- Programmable amp mute.
- `MASTER I` and `MASTER II`, two independent master volumes.
- MIDI CC/PC control over 16 MIDI channels.
- MIDI/footswitch control of channel switching and functions.
- Series FX loop with selectable behavior:
  - `ON LOOP`
  - `NON MASTER`
  - `LOOP OFF`
  - `LOOP SWITCHED`
- `M45` mod switch for lower-gain Plexi/JCM-like drive behavior.
- Power amp `FEED LO - BACK - HI` feedback voicing.
- Worldwide voltage preset: `230V` or `115V`.

## 6. Technical Specs

### Confirmed Specs

- Model/name: `S_ZERO V Vogg Amplifier (Decapitated) signature`
- Type: tube-powered premium guitar amplifier head.
- Wattage: 100 watts.
- Channels: 3 independent channels.
- Power tubes: 6L6, with secondary listing specifying 4x 6L6.
- MIDI: CC/PC over 16 MIDI channels.
- Masters: two independent master volumes.
- Gate: switchable dynamic gate; Reverb says adjustable gate.
- FX loop: rear `SEND`, `RETURN`, `LEVEL`, loop mode switch.
- Voltage: `230V` or `115V`.
- Mains/fuses: `230V/2AT` and `115V/4AT`.
- Anode fuse: `1AT`.
- Made in Poland, per Reverb listing and MLC company context.

### Speaker Outputs and Impedance

Observed:

- Rear overview image shows speaker output jacks and an impedance selector.

Confirmed family/spec context:

- The sibling MLC Subzero 100 page documents speaker outputs with `16 Ohm`, `8 Ohm`, and `4 Ohm` options.
- Search result text for an MLC Subzero V production post also surfaced `Speaker output: 4/8/16`, but the source was not accessible enough to treat as primary documentation.

Planning-safe conclusion:

- Treat S_ZERO V as having selectable speaker impedance output, likely 4/8/16 Ohm, but verify against the actual amp manual or rear-panel photo before implementing exact labels.

### FX Loop Details

Official FX loop close-up labels:

- `SEND`
- `RETURN`
- `LEVEL`
- `ON LOOP`
- `NON MASTER`
- `LOOP OFF`
- `LOOP SWITCHED`

The older Subzero 100 page describes a transparent, non-colorized, solid-state series FX loop with modes:

1. FX loop on and master on.
2. Non-master, loop off and master off.
3. Loop switched by MIDI or footswitch and master on.

This is sibling-family documentation; it aligns with the S_ZERO V labels, but should not be treated as an exact S_ZERO V manual.

## 7. Sound Character

Official descriptions:

- More aggressive and modern.
- Higher-gain metal sound.
- Fast and heavy riffs.
- Punchy lead tone.
- Vintage classic and creamy clean thick tone.
- Lush tube overdrive.
- Aggressive high-gain lead sound.
- Clean channel has American-voiced clean sounds with bottom-end punch and high headroom.
- Drive I is vintage British hot-rodded Plexi with more gain.
- Drive II is thick, juicy, harmonic, sustained, ultimate high-gain modded British tone.
- S_ZERO V combines S_ZERO 93 simplicity/power with S_ZERO 100S high-gain harsh/cold character and crystal-clear classic Fender-style sound.

Practical modern-metal interpretation for plugin design:

- Tight low end from 6L6 power amp plus depth/feedback control.
- Bright/aggressive high end shaped by presence and bright switches.
- Gate is part of the intended high-gain experience, not an afterthought.
- WARCLAW boost/saturation is central to signature tone.
- Drive II should be the primary modern metal channel.
- Drive I should cover lower/medium gain hot-rodded British tones.
- Clean should be high-headroom American style, with crunch available when pushed.

### Modern Metal vs Vintage Behavior

Modern-metal side:

- High-gain Drive II.
- Gate.
- WARCLAW saturation/boost.
- Tight depth/feedback shaping.
- 6L6 punch and clarity.

Vintage side:

- Clean-to-crunch behavior.
- Drive I hot-rodded `PLEXI 1968`.
- `M45` mode lowering drive gain toward Plexi/JCM-style behavior.

### Research Gaps to Verify Before Exact Emulation

- Exact preamp tube count and types.
- Exact speaker impedance selector labels on the S_ZERO V rear panel.
- Whether Clean and Drive tone stacks are passive or active.
- Whether WARCLAW is pre-gain boost, clipping-mode change, tone-shaping network, or a combination.
- Whether gate is pre-preamp, post-preamp, loop-level, or sidechain-controlled.
- Exact MIDI CC/PC map.
