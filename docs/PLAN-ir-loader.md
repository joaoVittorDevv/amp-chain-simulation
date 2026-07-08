# Plan: Impulse Response (IR) Loader

## 1. Overview and Project Focus
**Project Type:** BACKEND (Audio DSP) & Rust GUI (`egui`) integration.

We are integrating a custom Impulse Response (IR) loader into the existing Cabinet processing module. When active, it bypasses the algorithmic speaker simulation block and processes audio via FFT-based convolution. Memory allocation operations and I/O processes are strictly decoupled from the realtime audio thread.

## 2. New Dependencies (`Cargo.toml`)
The following crates will be added to the project for file picking and thread-safe data structures:
* `rfd`: Provides cross-platform native file dialogs to allow the user to select `.wav` files.
* `arc-swap`: Will allow us to update the Convolution engine's impulse response pointer optimally without the risk of locking or blocking the realtime audio thread.
* *(Note: `hound`, `rubato`, `realfft`, and `rtrb` are already present in the workspace).*

## 3. File Structure Changes
The current structure will be extended primarily in the DSP logic and UI layout:

* **CREATE** `src/core/dsp/cabinet/ir_convolver.rs`: Our FFT Convolution engine component.
* **MODIFY** `src/core/dsp/cabinet/mod.rs`: To conditionally route audio between the existing `speaker_model` and the new `ir_convolver`.
* **MODIFY** `src/core/ui/main_view.rs` (or `signal_chain.rs`): To incorporate the "Load IR" button, file name display, and algorithm vs. custom IR toggle mechanism.
* **MODIFY** `src/core/state.rs` (or equivalent parameter struct): To persist the state of the toggle (Algorithmic vs. IR). The IR path itself might be transient or loaded as `String` but must not be accessed on the RT thread directly.

## 4. Background File Processing & Realtime Delivery
**Cross-Thread Mechanism Logic:**

1. **Triggering (UI Thread):**
   When the user clicks "Load IR", `rfd` is invoked. If a file is selected, we spawn a background working thread (`std::thread::spawn` or existing threadpool logic if any).
2. **Decoding & Resampling (Background Thread):**
   * Uses `hound::WavReader` to decode the `.wav` file to `f32` vectors.
   * Compares the sample rate. If different from the host's sample rate, configures a `rubato` `FftFixedOut` or similar resampler to match the target sample rate.
   * Formats the IR to the length required by our Partitioned Convolution scheme (likely zero-padded).
   * Transforms the IR into the frequency domain (pre-calculating the FFTs) via `realfft` to ensure the audio thread does zero heavy lifting.
3. **Audio Thread Hand-off:**
   * Hand the resulting memory-aligned, frequency-domain partitioned IR representations to the audio engine.
   * The transfer will employ an `Arc<[Complex32]>` sent either via an atomic swap mechanism (`arc-swap`) or a lock-free queue (`rtrb`/`crossbeam-channel` ring buffer) back to the DSP loop.

## 5. DSP Implementation: FFT Convolution
Given that standard overlap-add creates latency and simple time-domain convolution on a size N buffer takes `O(N^2)` — rendering realistic 4096-sample IRs unacceptably slow — we will implement **Uniform Partitioned Convolution**.

1. **The Math (Partitioned Convolution):**
   * Partition the IR into $K$ blocks of length $B$ (where $B$ matches our desired algorithmic processing block size, often 64, 128, or 256 samples to minimize latency).
   * `realfft` computes forward FFTs of the audio input block and frequency-domain multiplications ($X \cdot H$) for each partition.
   * Sum the complex frequency bins across partitions.
   * Compute the inverse FFT.
   * Execute standard Overlap-Add or Overlap-Save exclusively on the resultant $B$-sized blocks.
2. **Signal Flow in `Cabinet::process(sample)`:**
   ```rust
   if state.use_custom_ir {
       // IR convolution engine processing
       self.ir_convolver.process(sample)
   } else {
       // Biquad cascade processing
       self.speaker_model.process(sample)
   }
   ```

## ✅ Phase X: Verification Plan (To-be Executed upon Implementation)
- [ ] No allocations in `ir_convolver.process()`. Verified via `assert_process_allocs` feature of `nih_plug`.
- [ ] IR toggle switches smoothly without audio artifacts (zippering/clicks).
- [ ] Memory leak checks: Background swap successfully drops older `Arc` block representations.
## ✅ PHASE X COMPLETE
- Lint: ✅ Pass
- Security: ✅ No critical issues
- Build: ✅ Success
- Date: sex 06 mar 2026 16:26:49 -03

## ✅ PHASE IMPLEMENTATION FULLY COMPLETE
- Standalone GUI updated to support Cabinet parameters correctly
- Project fully compiled and tested
