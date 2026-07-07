# Architecture

**Analyzed:** 2026-07-06
**Updated:** 2026-07-07 (added Component Lab module documentation)

**Pattern:** Dual-target modular (Plugin + Standalone sharing core DSP bridges and UI)

## High-Level Structure

```
┌──────────────────────────────────────────────────────────────┐
│                      PLUGIN TARGET                           │
│  src/lib.rs: BaseIO (nih_plug::Plugin)                       │
│  ┌─────────────────────────────────────────────────────────┐ │
│  │          7-STAGE DSP PIPELINE (BaseIO::process)         │ │
│  │  1. Input Routing → 2. Faust EQ → 3. Pre-EQ Convolver  │ │
│  │  → 4. Mojo Neural Drive → 5. Cabinet IR → 6. Master → 7. NaN    │ │
│  └─────────────────────────────────────────────────────────┘ │
│  CLAP/VST3 exports (nih_export_clap!, nih_export_vst3!)      │
└──────────────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────────────┐
│                     STANDALONE TARGET                        │
│  src/bin/standalone.rs: StandaloneApp (eframe::App)          │
│  ┌─────────────────────────────────────────────────────────┐ │
│  │  CPAL input stream → rtrb buffer → DSP chain →          │ │
│  │  rtrb buffer → CPAL output stream                       │ │
│  └─────────────────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────────────┐
│             SHARED CORE (src/core/ + src/bridge/)            │
│  bridge/  → ExternalProcessor trait + Faust/Mojo/WaveNet     │
│  core/    → state (params), dsp (analyzer), ui (egui panels) │
└──────────────────────────────────────────────────────────────┘
```

## DSP Pipeline (7 Stages)

All stages in `BaseIO::process()` [src/lib.rs:544]:

1. **Input Routing** [src/lib.rs:575]
   - `EnumParam<InputSelect>`: Stereo / Input 1 mono / Input 2 mono
   - Copies selected input into L/R channels

2. **Faust 3-Band Parametric EQ** [src/lib.rs:603]
   - Per-channel `FaustProcessor::set_eq_params()` + `process_block()`
   - Gated by `eq_active` BoolParam
   - Low/Mid/High with freq, gain, Q per band

3. **Pre-EQ Convolution** [src/lib.rs:621]
   - Per-channel `FFTConvolver::process()` with embedded WAV IR
   - Falls back to dry passthrough on error

4. **Mojo Neural Drive** [src/lib.rs:629]
   - Per-channel zero-copy `mojo_process_block(ptr, len, drive, output_gain)`
   - Gated by `neural_amp_active` BoolParam
   - Polynomial tanh approximation in Mojo

5. **Cabinet IR Convolution** [src/lib.rs:639]
   - Stereo `CabinetEngine::process()` with bypass/level/mix
   - FFTConvolver-based, selectable IR from SQLite library

6. **Master Gain** [src/lib.rs:670]
   - Applies `gain` + optional `neural_amp_volume`

7. **NaN Sanitization** [src/lib.rs:683]
   - NaN/Inf zeroing always runs, even in bypass

## ExternalProcessor Trait Pattern

```rust
// src/bridge/mod.rs:5
pub trait ExternalProcessor {
    fn init(&mut self, sample_rate: f32);
    fn process_block(&mut self, buffer: *mut f32, length: usize);
}
```

**Implementations:**
- `FaustProcessor` [src/bridge/faust.rs:56] — wraps Faust C ABI
- `MojoProcessor` [src/bridge/mojo.rs:47] — wraps Mojo shared library
- `WavenetProcessor` [src/bridge/wavenet.rs:151] — ONNX inference (unused; removed in cleanup)

**Pattern for adding a new processor:**
1. Implement `ExternalProcessor`
2. Add to `BaseIO` struct as `Option<NewProcessor>` per channel
3. Initialize in `Default::default()` / `initialize()`
4. Wire into `process()` at desired pipeline position

## Zero-Copy Rust ↔ Mojo FFI

```rust
// Rust side (src/bridge/mojo.rs:3)
extern "C" {
    fn mojo_process_block(address: usize, size: usize, drive: f32, output_gain: f32);
}
// Cast: buffer as usize, pass by value
```

```mojo
// Mojo side (neural/main.mojo:16)
@export
fn mojo_process_block(address: Int, size: Int, drive: Float32, output_gain: Float32):
    var data = UnsafePointer[Float32, MutAnyOrigin](unsafe_from_address=address)
    // In-place processing, no allocation
```

## Faust C Wrapper Pattern

```
dsp/main.dsp ──faust──▶ dsp/FaustModule.hpp
                              │
                 dsp/wrapper.cpp includes it
                 dsp/wrapper.h declares C ABI
                              │
              build.rs: bindgen ──▶ bindings_faust.rs
                              │
                 src/bridge/faust.rs includes bindings
```

## Parameter Architecture

**`BaseIOParams`** [src/core/state/plugin_params.rs:17]:
- Derives `nih_plug::Params`
- Fields use `#[id = "..."]` for host automation
- `#[persist]` for editor state and cabinet selection

**`EditorState`** [src/core/state/plugin_params.rs:85]:
- UI-only shared state (not Param fields)
- Carries params Arc, analyzer, cabinet handles, active panel

**Smoothing convention:**
- Gain/log params: `SmoothingStyle::Logarithmic(50.0)`
- Linear/percent params: `SmoothingStyle::Linear(50.0)`
- Read once per block: `self.params.param_name.smoothed.next()`

## FFT Analyzer

- `FFT_SIZE = 2048`, 48kHz fixed reference [src/core/dsp/analyzer.rs:6]
- Audio thread pushes post-DSP mono mix into `rtrb::Producer`
- UI thread drains `Consumer`, applies Blackman-Harris window, runs `realfft`
- Spectrum smoothed with `prev * 0.85 + db * 0.15`

## Existing Mode Toggle Patterns

1. **`EnumParam<InputSelect>`** — ComboBox + match in DSP [src/core/state/plugin_params.rs:7]
2. **`BoolParam`** — Checkbox toggles (`bypass`, `neural_active`, `eq_active`, `cabinet_bypass`)
3. **`ActivePanel` enum** — UI-only panel selector [src/core/ui/signal_chain.rs:3]

The `EnumParam` pattern is the primary candidate for an amp model selector.

## Component Lab Module (src/lab/) — v0.3 In Progress

The `src/lab/` module provides persistence, snapshot metadata, export, verification, and runtime DSP variant slots for component experiments. It wraps existing DSP processors additively; the hardcoded audio chain remains the fallback path.

### Hierarchy

```
Category (ex: "amp-modeler")   ← 1 slot in pipeline
  └── Node (ex: "Amp Modeler")  ← 1 node per category
        └── Variant A: "MLC ZERO V"   ← multiple implementations
        │     └── Snapshot v1.0.0     ← versioned state
        └── Variant B: "JCM900"
              └── Snapshot v1.0.0
```

### Key Patterns

- **Variant Switching:** `VariantMailbox` uses `ArcSwapOption` inbox/trash and an audio-owned `VariantSlot`.
- **Audio Thread:** `PipelineManager::process_block()` iterates pre-owned node slots; no DB access, no `Mutex`, no allocation.
- **Garbage Collection:** old variants are parked in mailbox trash and collected from the UI thread.
- **Storage:** SQLite at `~/.config/distortion/lab.db` (bundled rusqlite)
- **Registry:** `VariantRegistry` maps implementation ids to factories for `faust-eq`, `mojo-neural`, and `mlc-zero-v`.
- **Export:** AI-readable JSON + source bundle as `.tar.gz`

### DspVariant Trait

Existing processors implement `DspVariant` without changing their `ExternalProcessor` behavior:

```rust
pub trait DspVariant: Send {
    fn process_block(&mut self, buffer: *mut f32, length: usize);
    fn param_count(&self) -> usize;
    fn param_ids(&self) -> &[&str];
    fn latency(&self) -> usize;
}
```

### Module Structure

```
src/lab/
├── mod.rs              # Lab facade (init, save, load, export)
├── node.rs             # Node + variant switching lifecycle
├── variant_runtime.rs  # VariantMailbox + VariantSlot
├── snapshot.rs         # Snapshot data model + serde
├── component.rs        # DspVariant trait + data model types
├── database.rs         # SQLite CRUD + migrations
├── pipeline.rs         # Pipeline slot management
├── export.rs           # .tar.gz bundle generation
├── verification.rs     # Automated + manual checks
└── registry.rs         # VariantFactory + VariantRegistry
```
