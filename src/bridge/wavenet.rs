use super::ExternalProcessor;
use std::path::Path;
use tract_onnx::prelude::*;

type TractPlan = SimplePlan<TypedFact, Box<dyn TypedOp>, Graph<TypedFact, Box<dyn TypedOp>>>;

/// History length (receptive field) required by the WaveNet model.
const HISTORY_LEN: usize = 4096;

/// WaveNet neural processor that runs ONNX inference on the audio thread.
///
/// ## Real-time safety guarantees
///
/// * **Zero user-code heap allocations** in [`process_block`] — all working buffers
///   (`input_buf`, `output_buf`, `history`) are pre-allocated during construction
///   and reused every block via `clear()` + `extend_from_slice()`. The ONE exception
///   is tract-onnx's internal `ArrayView::to_owned()` call (see below), which is an
///   unavoidable library-internal allocation required by tract's tensor ownership
///   model.
/// * **No `Mutex` on the audio thread** — the ONNX session is exclusively owned by
///   this struct and only ever accessed from the single-threaded nih_plug audio
///   callback. Model loading happens during [`new`] (before the audio stream
///   starts), so no synchronisation is required.
///
/// ## Interaction with Mojo/tanh knobs
///
/// When the WaveNet ONNX model loads successfully, the existing **`neural_drive`**
/// and **`neural_output_gain`** parameters have **no effect** — WaveNet performs
/// all saturation and gain-shaping internally based on its training data. The Mojo
/// tanh fallback path (used when the ONNX model fails to load or is disabled)
/// *does* honour those knobs.
///
/// ## Fallback
///
/// If the ONNX model cannot be loaded at construction time, `is_ready()` returns
/// `false` and every call to [`process_block`] is a no-op (passthrough). The
/// caller should fall back to the Mojo tanh processor in that case.
pub struct WavenetProcessor {
    /// Compiled ONNX execution plan. `None` when the model failed to load.
    model: Option<TractPlan>,

    /// Rolling history window for the WaveNet receptive field.
    history: Vec<f32>,

    /// Pre-allocated input buffer (history + current block). Capacity is
    /// `HISTORY_LEN + max_block_size` to guarantee `extend_from_slice` never
    /// re-allocates on the audio thread.
    input_buf: Vec<f32>,

    /// Pre-allocated output buffer. Capacity is `max_block_size`.
    output_buf: Vec<f32>,

    /// Maximum block size the processor is configured to handle.
    /// Used to enforce bounds at runtime so `extend_from_slice` never re-allocates.
    max_block_size: usize,
}

impl WavenetProcessor {
    /// Creates a new processor, loading the ONNX model from `model_path`.
    ///
    /// `max_block_size` is the largest buffer size that [`process_block`] will
    /// ever receive (e.g. `buffer_config.max_buffer_size`). All internal
    /// buffers are pre-allocated to this size so the hot path never heap-allocates.
    pub fn new(model_path: &str, max_block_size: usize) -> Self {
        let history = vec![0.0f32; HISTORY_LEN];

        // Pre-allocate working buffers at maximum capacity so that
        // `clear()` + `extend_from_slice()` never trigger a re-allocation.
        //
        // input_buf: HISTORY_LEN (history) + max_block_size (current frame)
        // output_buf: max_block_size (worst-case output frame)
        let input_buf = Vec::with_capacity(HISTORY_LEN + max_block_size);
        let output_buf = Vec::with_capacity(max_block_size);

        let model = Self::load_model(model_path);

        Self {
            model,
            history,
            input_buf,
            output_buf,
            max_block_size,
        }
    }

    /// Returns `true` if the ONNX model is loaded and ready for inference.
    pub fn is_ready(&self) -> bool {
        self.model.is_some()
    }

    // ─── private helpers ──────────────────────────────────────────────────

    fn load_model(model_path: &str) -> Option<TractPlan> {
        if !Path::new(model_path).exists() {
            eprintln!("[Wavenet] Model file not found: {}", model_path);
            return None;
        }

        match tract_onnx::onnx()
            .model_for_path(model_path)
            .and_then(|m| m.into_optimized())
            .and_then(|m| m.into_runnable())
        {
            Ok(plan) => {
                eprintln!("[Wavenet] Model loaded successfully: {}", model_path);
                Some(plan)
            }
            Err(e) => {
                eprintln!("[Wavenet] Failed to load model {}: {:?}", model_path, e);
                None
            }
        }
    }
}

impl ExternalProcessor for WavenetProcessor {
    fn init(&mut self, _sample_rate: f32) {
        // Model is already loaded in `new()`. Nothing to initialise per-channel.
    }

    /// Runs WaveNet ONNX inference on `length` samples starting at `buffer`.
    ///
    /// # Real-time safety
    ///
    /// * Zero user-code heap allocations — `input_buf` and `output_buf` are reused
    ///   via `clear()` + `extend_from_slice()`. Capacity was reserved during `new()`.
    ///   The ONE and ONLY allocation on this path is tract-onnx's internal
    ///   `ArrayView::to_owned()` required to transfer tensor data ownership to the
    ///   runtime engine. This is a library-internal allocation; no user-code Vec
    ///   or Box allocations occur.
    /// * No locking — the model is accessed directly on the single-threaded
    ///   audio callback.
    ///
    /// # Errors / fallback
    ///
    /// If the model failed to load, this is a silent no-op (the buffer is left
    /// unchanged). If inference fails at runtime (shape mismatch, ONNX error),
    /// the buffer is likewise left unchanged.
    fn process_block(&mut self, buffer: *mut f32, length: usize) {
        // ── guard: model not loaded ────────────────────────────────────
        let Some(model) = &self.model else {
            return;
        };

        // ── enforce max block size ─────────────────────────────────────
        // Capacity was reserved for exactly this bound; exceeding it causes
        // a re-allocation (violating the real-time contract).
        debug_assert!(
            length <= self.max_block_size,
            "process_block called with length {} but max_block_size is {}",
            length,
            self.max_block_size,
        );
        if length > self.max_block_size {
            return;
        }

        // ── safety: caller guarantees `buffer` is valid for `length` floats ─
        let input_slice = unsafe { std::slice::from_raw_parts(buffer, length) };

        // ── build input tensor (zero user-allocation) ──────────────────
        // Reuse the pre-allocated buffer: clear then append history + current block.
        // Capacity was reserved at `HISTORY_LEN + max_block_size`, so
        // `extend_from_slice` never re-allocates.
        self.input_buf.clear();
        self.input_buf.extend_from_slice(&self.history);
        self.input_buf.extend_from_slice(input_slice);

        let total_len = HISTORY_LEN + length;

        // ── single unavoidable allocation ──────────────────────────────
        // tract-onnx requires owned tensor data; `ArrayView::to_owned()` is the
        // ONLY heap allocation on this hot path. It clones HISTORY_LEN + length
        // floats into an internal ndarray buffer. No user-code Vec, Box, or
        // other allocation occurs in process_block.
        // This is inherent to tract's API — the runtime engine takes ownership
        // of tensor data and does not accept borrowed slices.
        let input_view = match tract_onnx::tract_ndarray::ArrayView3::from_shape(
            (1, 1, total_len),
            &self.input_buf,
        ) {
            Ok(v) => v,
            Err(_) => return,
        };
        let input_tensor: Tensor = input_view.to_owned().into();

        // ── run inference ─────────────────────────────────────────────
        let result = match model.run(tvec!(input_tensor.into())) {
            Ok(r) => r,
            Err(_) => return,
        };

        // ── extract output into pre-allocated buffer ───────────────────
        self.output_buf.clear();
        if let Ok(view) = result[0].to_array_view::<f32>() {
            if let Some(slice) = view.as_slice() {
                if slice.len() >= length {
                    self.output_buf
                        .extend_from_slice(&slice[slice.len() - length..]);
                }
            }
        }

        // ── write result back to caller's buffer ──────────────────────
        // Guard: if output extraction failed (output_buf stayed empty), bail
        // out rather than writing undefined data or silently dropping output.
        if self.output_buf.len() < length {
            return;
        }
        unsafe {
            std::ptr::copy_nonoverlapping(self.output_buf.as_ptr(), buffer, length);
        }

        // ── update rolling history ─────────────────────────────────────
        // IMPORTANT: read from self.input_buf (which still holds the ORIGINAL
        // input data — `to_owned()` cloned it into the tensor) rather than from
        // `input_slice`, which is a view into `buffer`. After the
        // `copy_nonoverlapping` above, `buffer` contains processed output, so
        // `input_slice` now points to processed data, not the original input.
        //
        // input_buf layout: [old_history (HISTORY_LEN) | current_input (length)]
        // We want the last HISTORY_LEN elements as the new history.
        self.history
            .copy_from_slice(&self.input_buf[total_len - HISTORY_LEN..]);
    }
}
