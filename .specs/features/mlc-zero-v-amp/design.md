# MLC ZERO V Signature Amp — Design

**Feature ID:** `MLCZERO`
**Status:** Planning | **✅ Cross-vendor review COMPLETED (pi, 2026-07-06)**

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                     BaseIO::process()                           │
│  ┌──────────┐  ┌────────┐  ┌──────────┐  ┌───────────────────┐ │
│  │ Input    │→│ Faust  │→│ Pre-EQ   │→│ AMP STAGE (NEW)   │ │
│  │ Routing  │  │ EQ     │  │ Convolver│  │                   │ │
│  └──────────┘  └────────┘  └──────────┘  │ ┌───────────────┐ │ │
│                                           │ │ AmpSelector   │ │ │
│                                           │ │ EnumParam     │ │ │
│                                           │ └───┬───────────┘ │ │
│                                           │     │             │ │
│                                           │  ┌──┴──────────┐  │ │
│                                           │  │ Neural  │MLC│  │ │
│                                           │  │ (Mojo)  │ZERO│  │ │
│                                           │  │ EXIST.  │ V  │  │ │
│                                           │  └─────────────┘  │ │
│                                           └───────────────────┘ │
│                                                      │          │
│  ┌──────────┐  ┌────────┐                            │          │
│  │ NaN      │←─│ Master │←───────────────────────────┘          │
│  │ Sanitize │  │ Gain   │                                       │
│  └──────────┘  └────────┘                                       │
└─────────────────────────────────────────────────────────────────┘
```

## Component Design

### 1. Amp Model Selector (`AmpModel` enum)

```rust
// src/core/state/plugin_params.rs — alongside InputSelect

#[derive(Enum, Debug, PartialEq, Eq, Clone, Copy)]
pub enum AmpModel {
    #[name = "Neural"]
    Neural,
    #[name = "MLC ZERO V"]
    MlcZeroV,
}
```

**Pattern:** Follows `InputSelect` enum exactly [src/core/state/plugin_params.rs:7-15].

**Parameter definition:**
```rust
#[id = "amp_model"]
pub amp_model: EnumParam<AmpModel>,
```

### 2. MLC ZERO V Parameters (in `BaseIOParams`)

All parameters use the project's established `FloatParam` + smoothing pattern.

#### Gain Stage
| Param ID | Display Name | Range | Smoothing | Unit |
|----------|-------------|-------|-----------|------|
| `mlc_gain` | Gain | -60.0 to 0.0 dB | Logarithmic(50.0) | dB |
| `mlc_master` | Master | -60.0 to 0.0 dB | Logarithmic(50.0) | dB |

#### Tone Stack
| Param ID | Display Name | Range | Smoothing | Unit |
|----------|-------------|-------|-----------|------|
| `mlc_bass` | Bass | -12.0 to 12.0 dB | Linear(50.0) | dB |
| `mlc_middle` | Middle | -12.0 to 12.0 dB | Linear(50.0) | dB |
| `mlc_treble` | Treble | -12.0 to 12.0 dB | Linear(50.0) | dB |
| `mlc_presence` | Presence | -12.0 to 12.0 dB | Linear(50.0) | dB |
| `mlc_depth` | Depth | -12.0 to 12.0 dB | Linear(50.0) | dB |

#### Switches
| Param ID | Display Name | Type | Options |
|----------|-------------|------|---------|
| `mlc_bright` | Bright | EnumParam | I, II |
| `mlc_m45` | M45 Mod | BoolParam | Off, On |
| `mlc_warclaw` | WARCLAW | BoolParam | Off, On |
| `mlc_feedback` | Feedback | EnumParam | Lo, Hi |

#### Gate
| Param ID | Display Name | Range | Smoothing | Unit |
|----------|-------------|-------|-----------|------|
| `mlc_gate` | Gate | -80.0 to 0.0 dB | Linear(50.0) | dB |
| `mlc_gate_pos` | Gate Pos | EnumParam { Pre, Post } | — | — |

#### EQ Saturation Control
| Param ID | Display Name | Range | Smoothing | Unit |
|----------|-------------|-------|-----------|------|
| `eq_tanh_bypass` | EQ Tanh Bypass | BoolParam | — | — |

**Note on `eq_tanh_bypass`:** The existing EQ Faust module (`dsp/main.dsp:26`) applies `ma.tanh` after the tone stack. When MLC ZERO V is active, this creates a cascaded saturation with the MLC's own 3-stage tanh. `eq_tanh_bypass` allows the user to disable the EQ's tanh independently — defaults to `false` (EQ tanh active, preserving current behavior).

### 3. Faust DSP Module (`dsp/mlc_zero_v.dsp`)

New Faust file implementing the MLC ZERO V Drive II channel model.

**Signal chain:**
```
Gate (Pre, optional) → Input → Pre-Gain → Bright Cap
     → 3-Stage Cascaded Gain (Plexi-style)
     → Tone Stack (Bass/Mid/Treble) → WARCLAW Saturation
     → Gate (Post, optional) → Presence/Depth (Power Amp EQ)
     → Master Volume → Output
```
**Gate position:** User-selectable via `mlc_gate_pos` enum — `Pre` inserts gate before pre-gain (traditional input gate), `Post` inserts after tone stack (modern style). Only one position active at a time.

**Faust architecture:**

```faust
// dsp/mlc_zero_v.dsp
import("stdfaust.lib");

// --- Controls ---
gain    = hslider("Gain", 0.5, 0.0, 1.0, 0.01);
bass    = hslider("Bass", 0.5, 0.0, 1.0, 0.01);
middle  = hslider("Middle", 0.5, 0.0, 1.0, 0.01);
treble  = hslider("Treble", 0.5, 0.0, 1.0, 0.01);
presence = hslider("Presence", 0.5, 0.0, 1.0, 0.01);
depth   = hslider("Depth", 0.5, 0.0, 1.0, 0.01);
master  = hslider("Master", 0.5, 0.0, 1.0, 0.01);
bright  = nentry("Bright", 1, 0, 1, 1);    // 0=I, 1=II
m45     = nentry("M45", 0, 0, 1, 1);       // 0=Off, 1=On
warclaw = nentry("WARCLAW", 0, 0, 1, 1);   // 0=Off, 1=On
feedback = nentry("Feedback", 1, 0, 1, 1);  // 0=Lo, 1=Hi
gate_thresh = hslider("Gate", 0.1, 0.0, 1.0, 0.01);

// --- Gain Stages (3-stage cascaded Plexi) ---
// Stage 1: Clean boost → soft clip
// Stage 2: Overdrive → asymmetric clip
// Stage 3: High-gain → hard clip
// Each stage uses tanh saturation with gain-dependent drive

// --- Tone Stack (FMV-style passive) ---
// Classic Fender/Marshall/Vox tone stack topology
// Bass, Middle, Treble with interaction modeling

// --- WARCLAW Saturation ---
// Additional saturation stage with voicing shift
// Activated by warclaw==1

// --- Gate ---
// Simple expander/gate with threshold
// Smooth attack/release envelope

// --- Power Amp EQ ---
// Presence: high-shelf in feedback loop
// Depth: low-shelf resonance
// Feedback switch: alters damping factor

// --- Output ---
process = pre_gain : gain_stages : tone_stack
        : warclaw_stage : gate_stage
        : power_amp_eq : *(master);
```

**Key design decisions:**
- **Gain staging:** 3 cascaded tanh saturators with inter-stage attenuation (models Plexi 1968 hot-rod topology)
- **Tone stack:** FMV (Fender/Marshall/Vox) passive tone stack — the industry standard for guitar amps
- **WARCLAW:** Additional pre-tone-stack saturation with mid boost voicing
- **Gate:** Simple RMS envelope follower with attack=1ms, release=50ms
- **Presence/Depth:** Modeled as shelving filters in the power amp feedback loop
- **Feedback switch:** Alters the depth and presence filter parameters (Hi = tighter, Lo = looser)

### 4. C ABI Wrapper (`dsp/mlc_zero_v_wrapper.cpp` + `.h`)

Separate wrapper file for the MLC ZERO V Faust module. Same pattern as existing `dsp/wrapper.cpp` but with MLC-specific parameters.

**C ABI functions:**
```c
FaustHandle* mlc_zero_v_create();
void mlc_zero_v_init(FaustHandle* handle, float sample_rate);
void mlc_zero_v_process(FaustHandle* handle, float* buffer, int length);
void mlc_zero_v_destroy(FaustHandle* handle);
void mlc_zero_v_set_gain(FaustHandle* handle, float value);
void mlc_zero_v_set_bass(FaustHandle* handle, float value);
// ... all param setters
```

**build.rs changes:**
- Compile `dsp/mlc_zero_v.dsp` to `dsp/MlcZeroVModule.hpp` with Faust class name `-cn mlczerov` to prevent symbol collision with the existing `mydsp` class (EQ module)
- Compile `dsp/mlc_zero_v_wrapper.cpp` with `cc::Build`
- Generate bindgen for `mlc_zero_v_.*` functions

### 5. Rust Bridge (`src/bridge/mlc_zero_v.rs`)

```rust
// src/bridge/mlc_zero_v.rs

include!(concat!(env!("OUT_DIR"), "/bindings_mlc_zero_v.rs"));

pub struct MlcZeroVProcessor {
    handle: *mut FaustHandle,
    is_ready: bool,
    gain: f32,
    bass: f32,
    middle: f32,
    treble: f32,
    presence: f32,
    depth: f32,
    master: f32,
    bright: i32,
    m45: i32,
    warclaw: i32,
    feedback: i32,
    gate_thresh: f32,
}

impl MlcZeroVProcessor {
    pub fn new() -> Self { /* call mlc_zero_v_create() */ }
    pub fn set_gain(&mut self, v: f32) { self.gain = v; }
    // ... all setters
}

impl ExternalProcessor for MlcZeroVProcessor {
    fn init(&mut self, sample_rate: f32) { /* mlc_zero_v_init */ }
    fn process_block(&mut self, buffer: *mut f32, length: usize) {
        // Set all params via C ABI, then call mlc_zero_v_process
    }
}

unsafe impl Send for MlcZeroVProcessor {}
```

### 6. DSP Pipeline Integration (`src/lib.rs`)

**Added to `BaseIO` struct:**
```rust
pub struct BaseIO {
    // ... existing fields ...
    mlc_zero_v: [Option<MlcZeroVProcessor>; 2],  // per-channel
}
```

**In `process()`:**
```rust
let amp_model = self.params.amp_model.value();

match amp_model {
    AmpModel::Neural => {
        // Existing Mojo code (unchanged)
        if neural_active {
            mojo.set_drive(neural_drive);
            mojo.set_output_gain(neural_output_gain);
            mojo.process_block(channel_samples.as_mut_ptr(), len);
        }
    }
    AmpModel::MlcZeroV => {
        // New MLC ZERO V path
        if let Some(ref mut mlc) = self.mlc_zero_v[channel_idx] {
            mlc.set_gain(mlc_gain);
            mlc.set_bass(mlc_bass);
            // ... all params ...
            mlc.process_block(channel_samples.as_mut_ptr(), len);
        }
    }
}
```

**Crossfade on transition:** Track previous amp model; if changed, apply linear crossfade over ~10ms (480 samples at 48kHz) between the two outputs.

**Required state fields in `BaseIO`:**
```rust
previous_amp_model: AmpModel,         // track last active model
crossfade_sample: usize,              // counts 0..CROSSFADE_LEN per block
crossfade_buf: [[f32; MAX_BLOCK]; 2],  // per-channel dry buffer (old output)
```

**Crossfade algorithm:** When `amp_model != previous_amp_model`, save current output into `crossfade_buf[ch]`, then run the new model, then blend linearly: `out[i] = old[i] * (1 - t) + new[i] * t` where `t = i / CROSSFADE_LEN`. Reset `crossfade_sample` when the block completes. If model changes mid-crossfade, abort current fade and start fresh (prevents click from rapid toggling).

### 7. Standalone Parity (`src/bin/standalone.rs`)

**Added to `StandaloneState`:**
```rust
pub struct StandaloneState {
    // ... existing fields ...
    pub amp_model: AmpModel,
    pub mlc_gain: f32,
    pub mlc_bass: f32,
    // ... all MLC params ...
}
```

**Added to `AudioSnapshot`:**
```rust
pub struct AudioSnapshot {
    // ... existing fields ...
    pub amp_model: AmpModel,
    // ... all MLC params (plain f32/bool) ...
}
```

**DSP callback:** Same match on `amp_model`, calling local `MlcZeroVProcessor` instances.

### 8. UI Panel (`src/core/ui/mlc_zero_v_panel.rs`)

New egui panel file rendering MLC ZERO V controls.

**Layout:** Grouped by section (matching real amp front panel):
- **Gain section:** Gain knob, Master knob
- **EQ section:** Bass, Middle, Treble knobs
- **Power Amp section:** Presence, Depth knobs
- **Switches section:** Bright I/II, M45, WARCLAW, Feedback LO/HI
- **Gate section:** Gate threshold knob

Uses `egui_knob` for all knobs (consistent with existing UI).
Uses `egui::ComboBox` or toggle buttons for switches.

**Integration in `main_view.rs`:** 
1. Add `MlcZeroV` variant to the `ActivePanel` enum (prerequisite for panel routing)
2. Add `AmpModel::MlcZeroV` case to the amp panel routing, following the existing `ActivePanel::Neural` pattern

## Data Flow

```
User turns knob in UI
    │
    ▼
Plugin: set_parameter() → FloatParam.smoothed
Standalone: mutex.lock() → StandaloneState field
    │
    ▼
BaseIO::process() — once per block:
  params.param.smoothed.next() → local f32
    │
    ▼
mlc.set_gain(value) → stores in MlcZeroVProcessor field
    │
    ▼
mlc.process_block(ptr, len):
  mlc_zero_v_set_gain(handle, self.gain)  // C ABI
  mlc_zero_v_process(handle, buffer, len)  // Faust DSP
    │
    ▼
Faust DSP processes buffer in-place (zero-copy)
    │
    ▼
Buffer continues to Cabinet IR stage
```

## File Manifest

| File | Action | Purpose |
|------|--------|---------|
| `dsp/mlc_zero_v.dsp` | CREATE | Faust DSP model of MLC ZERO V Drive II |
| `dsp/mlc_zero_v_wrapper.cpp` | CREATE | C ABI wrapper for MLC Faust module |
| `dsp/mlc_zero_v_wrapper.h` | CREATE | C ABI header |
| `src/bridge/mlc_zero_v.rs` | CREATE | Rust bridge for MLC ZERO V FFI |
| `src/bridge/mod.rs` | MODIFY | Add `pub mod mlc_zero_v;` |
| `src/core/state/plugin_params.rs` | MODIFY | Add `AmpModel` enum + all MLC params |
| `src/lib.rs` | MODIFY | Add MlcZeroVProcessor to BaseIO, wire DSP |
| `src/bin/standalone.rs` | MODIFY | Add MLC state, DSP, UI to standalone |
| `src/core/ui/mlc_zero_v_panel.rs` | CREATE | MLC control panel UI |
| `src/core/ui/mod.rs` | MODIFY | Add `pub mod mlc_zero_v_panel;` |
| `src/core/ui/main_view.rs` | MODIFY | Route to MLC panel when selected |
| `src/core/ui/signal_chain.rs` | MODIFY | Add MLC to signal chain UI nodes |
| `build.rs` | MODIFY | Add MLC Faust compilation + bindgen |

## Risk Mitigation

| Risk | Mitigation |
|------|------------|
| Faust tone stack doesn't sound right | Iterate on FMV component values; compare against reference tone stack implementations |
| 3-stage gain model unstable | Clamp internal gain at each stage; use tanh with configurable drive |
| Gate cuts sustained notes | Use slow release (50ms) and RMS detection; test with long decays |
| Build complexity increases | Reuse existing Faust/bindgen pipeline pattern exactly |
| Standalone parity broken | Add all MLC params to standalone in same commit as plugin params |
