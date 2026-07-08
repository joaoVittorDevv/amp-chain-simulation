# Project Plan: Guitar Cabinet Simulator

## 1. Overview
This plan details the implementation of a high-quality, hybrid Guitar Cabinet Simulator in the existing modular Rust plugin project (`nih_plug` + `eframe`/`egui`). The cabinet will act as the final acoustic filter to tame raw, heavily distorted signals using algorithmic IIR filtering (speaker modeling) and FFT/delay-line room ambience.

## 2. Project Type
**BACKEND / DSP + DESKTOP UI** (Rust)

## 3. Success Criteria
- [ ] UI properly displays "CABINET" block and its controls.
- [ ] DSP correctly processes audio with high-pass (70Hz) and low-pass (8-10kHz) pre-conditioning filters.
- [ ] Mic Position parameter correctly interpolates High-Shelf and LPF cutoff.
- [ ] Mic Distance parameter correctly interpolates Low-Shelf (proximity effect) and Room mix.
- [ ] Cabinet Dimension enum selects correct Helmholtz resonance frequency and activates 4x12 notches if applicable.
- [ ] DSP `process` loop must remain completely lock-free and allocation-free (no Garbabe Collection pauses, strictly bounded `rtrb` communication, and memory pre-allocated in `initialize`/`reset`).

## 4. Tech Stack & Roles
- **Core DSP Filter & FFT:** `biquad`, `realfft`, `rustfft`
- **Utility / Concurrency:** `rubato` (SRC), `rtrb` (lock-free ring buffers), `fast-math`, `nalgebra`
- **Plugin Framework:** `nih_plug`
- **UI Framework:** `eframe`, `egui`

Assigned Agents: 
- `rust-pro` (Primary for DSP implementation + lock-free memory constraints)
- `frontend-specialist` / `rust-pro` (For egui UI updates)

## 5. File Structure
```text
src/
├── core/
│   ├── state/
│   │   ├── plugin_params.rs       (Update with new Params)
│   ├── dsp/
│   │   ├── cabinet/               (New module)
│   │   │   ├── mod.rs
│   │   │   ├── filters.rs         (Pre-conditioning and IIR speaker models)
│   │   │   ├── ambience.rs        (FFT-based / delay-line room reflections)
│   ├── ui/
│   │   ├── signal_chain.rs        (Update to replace DIST with CABINET)
│   │   ├── main_view.rs           (Update to expose Cabinet parameters)
```

## 6. Task Breakdown

### Phase 1: Parameter Definition
- **Task 1.1: Define Cabinet Parameters**
  - **Agent:** `rust-pro`
  - **Action:** Update `src/core/state/plugin_params.rs`. Add `mic_position` [0.0 - 1.0], `mic_distance` [0.0 - 1.0], and `cabinet_dimension` [1x12, 2x12, 4x12].
  - **INPUT → OUTPUT:** Code containing `nih_plug` parameter definitions for cabinet. → Updated struct.
  - **VERIFY:** Plugin compiles. Parameters are visible in the host.

### Phase 2: Fundamental Filtering & IIR Setup
- **Task 2.1: Pre-Conditioning Filters**
  - **Agent:** `rust-pro`
  - **Action:** Create `src/core/dsp/cabinet/filters.rs`. Implement 70Hz High-Pass and 8kHz-10kHz Low-Pass. Pre-allocate structs.
  - **INPUT → OUTPUT:** DSP formulas using `biquad`. → `PreConditioningFilter` struct with `process()` method.
  - **VERIFY:** Lock-free, allocation-free filter processing passes data correctly.

- **Task 2.2: Algorithmic Speaker Modeling Filters**
  - **Agent:** `rust-pro`
  - **Action:** In `filters.rs`, implement mapping logic for: 
    - Mic Position (High-Shelf & LPF cutoff adjustments).
    - Mic Distance (Low-Shelf gain + room mix tracking).
    - Cab Dimension (Peak filter for Helmholtz at 200/140/100Hz) and Notch filter matrix for 4x12.
  - **INPUT → OUTPUT:** Multi-stage biquad cascade. → `SpeakerModel` struct.
  - **VERIFY:** Interpolation math dynamically recalculates coefficients without heap allocation.

### Phase 3: Room Ambience
- **Task 3.1: FFT / Delay Line Engine**
  - **Agent:** `rust-pro`
  - **Action:** Create `src/core/dsp/cabinet/ambience.rs`. Setup `realfft`/`rustfft` buffers OR a static delay-line array. Memory must be initialized in the plugin's `initialize` or `reset` callback.
  - **INPUT → OUTPUT:** Buffer structures. → `RoomAmbience` struct with pre-allocated vectors.
  - **VERIFY:** No allocation during `process()`; use of ring buffers bounds data.

### Phase 4: Integration
- **Task 4.1: Integrate DSP into Main Chain**
  - **Agent:** `rust-pro`
  - **Action:** Hook `Cabinet` processor into the main DSP signal chain. Link parameters from `state` to trigger recalculations in `Cabinet`.
  - **INPUT → OUTPUT:** Main audio callback. → Refactored `process()` in plugin root.
  - **VERIFY:** Audio passes perfectly; extreme settings don't cause CPU spikes (RT-safe).

### Phase 5: UI Refactoring
- **Task 5.1: Replace DIST with CABINET block**
  - **Agent:** `frontend-specialist` (or `rust-pro` for `eframe`)
  - **Action:** Update `src/core/ui/signal_chain.rs` to render the "CABINET" visual block at the end.
  - **INPUT → OUTPUT:** `egui` code. → Working signal block.
  - **VERIFY:** UI updates accurately when block is clicked.

- **Task 5.2: Wire Main View Controls**
  - **Agent:** `frontend-specialist`
  - **Action:** Update `src/core/ui/main_view.rs` to show sliders/knobs for Mic Position, Mic Distance, and a dropdown/radio for Cabinet Dimension.
  - **INPUT → OUTPUT:** `egui` layout code. → Fully interactive Cabinet UI.
  - **VERIFY:** Sliders modify plugin state properly.

## 7. Verification Checklist (Phase X)
- [x] `cargo check` passes with no warnings.
- [x] `cargo clippy` run successfully.
- [x] No `alloc` calls (`Box::new`, `Vec::push`, `format!`) inside the `process()` audio thread loop.
- [x] All DSP structs initialize in the `initialize()` / `reset()` functions.
- [x] UI runs in its own thread, utilizing `rtrb` / `nih_plug`'s param mechanism.
- [x] Build standalone `cargo run --features standalone` without errors.

## ✅ PHASE X COMPLETE
_To be marked after implementation._
- Lint: ✅ Pass
- Security: ✅ No critical issues
- Build: ✅ Success
- Date: 2026-03-06
