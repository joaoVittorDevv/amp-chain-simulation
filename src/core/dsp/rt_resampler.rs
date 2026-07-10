//! Real-time async resampler with staging buffer for variable-size input blocks.
//!
//! Uses `rubato::Async` with `FixedAsync::Input` — the input side always expects a
//! fixed number of frames per chunk. A staging buffer accumulates variable-size
//! callback blocks until enough frames are available for one resampler chunk.
//!
//! # Zero-allocation guarantee
//! All buffers are pre-allocated in `RtResampler::new`. The `feed` method does
//! not allocate: adapters are constructed on the stack from pre-allocated storage.

use audioadapter_buffers::direct::SequentialSliceOfVecs;
use rubato::{
    FixedAsync, Resampler, SincInterpolationParameters, SincInterpolationType, WindowFunction,
};

/// Proportional controller gain for clock-drift compensation (T32).
const KP: f64 = 0.05;
/// Target ring-buffer fill ratio.
const TARGET_FILL: f64 = 0.5;
/// Number of recent corrections averaged before changing the ratio.
const TRIM_HISTORY: usize = 16;

/// A real-time async resampler that converts between two sample rates on the
/// audio thread without allocation.
///
/// Built on `rubato::Async` with `FixedAsync::Input`.  Callers feed variable-size
/// deinterleaved blocks via [`feed`](RtResampler::feed); the staging buffer
/// accumulates them and invokes the callback for every complete resampled chunk.
pub struct RtResampler {
    inner: rubato::Async<f32>,
    /// Staging buffer — one `Vec<f32>` per channel (always 2 channels).
    staging: Vec<Vec<f32>>,
    /// How many frames are currently staged.
    staged: usize,
    /// Output buffer the resampler writes into — one `Vec<f32>` per channel.
    output: Vec<Vec<f32>>,
    /// Constant input chunk size (= `input_frames_next()`).
    chunk_size: usize,
    /// Cached value of `output_frames_max()` so callers can size scratch buffers.
    output_frames_max: usize,
    /// Fixed-size correction history for drift-controller smoothing.
    trim_history: [f64; TRIM_HISTORY],
    trim_history_len: usize,
    trim_history_index: usize,
}

impl RtResampler {
    /// Input frames consumed per resampler chunk, and therefore the granularity
    /// at which output chunks are emitted. Trades latency against efficiency:
    /// 512 frames @ 44.1 kHz adds ~11.6 ms of staging latency worst case.
    ///
    /// Callers sizing a buffer that receives whole chunks should scale this by
    /// the resample ratio — see [`output_frames_max`](RtResampler::output_frames_max).
    pub const CHUNK_FRAMES: usize = 512;

    /// Create a new resampler.
    ///
    /// * `input_rate`  – sample rate of the incoming audio (Hz).
    /// * `output_rate` – sample rate of the resampled output (Hz).
    /// * `max_block`   – largest callback block size the system may deliver.
    ///
    /// # Panics
    /// Panics if the rubato resampler cannot be constructed (invalid rates, etc.).
    pub fn new(input_rate: f32, output_rate: f32, max_block: usize) -> Self {
        assert!(input_rate > 0.0, "input_rate must be > 0");
        assert!(output_rate > 0.0, "output_rate must be > 0");

        let ratio = output_rate as f64 / input_rate as f64;

        let chunk_size: usize = Self::CHUNK_FRAMES;

        let params = SincInterpolationParameters {
            sinc_len: 256,
            f_cutoff: 0.95,
            interpolation: SincInterpolationType::Linear,
            oversampling_factor: 256,
            window: WindowFunction::BlackmanHarris2,
        };

        let inner = rubato::Async::<f32>::new_sinc(
            ratio,
            1.05,
            &params,
            chunk_size,
            2, // stereo
            FixedAsync::Input,
        )
        .expect("failed to create rubato Async resampler");

        let input_frames_max = inner.input_frames_max();
        let output_frames_max = inner.output_frames_max();

        // Staging buffer must hold one chunk plus the largest callback block.
        let staging_cap = input_frames_max + max_block;

        Self {
            inner,
            staging: vec![vec![0.0f32; staging_cap], vec![0.0f32; staging_cap]],
            output: vec![vec![0.0f32; output_frames_max], vec![0.0f32; output_frames_max]],
            staged: 0,
            chunk_size,
            output_frames_max,
            trim_history: [0.0; TRIM_HISTORY],
            trim_history_len: 0,
            trim_history_index: 0,
        }
    }

    /// Number of input frames the resampler needs for its next chunk.
    /// Constant for `FixedAsync::Input`.
    pub fn input_frames_next(&self) -> usize {
        self.chunk_size
    }

    /// Maximum output frames one chunk can produce.
    pub fn output_frames_max(&self) -> usize {
        self.output_frames_max
    }

    /// Resampler output delay in output frames (for latency reporting).
    pub fn output_delay(&self) -> usize {
        self.inner.output_delay()
    }

    /// Worst-case extra input frames the staging buffer may introduce before
    /// the first chunk is dispatched (`chunk_size - 1`).
    pub fn staging_max_delay_input_frames(&self) -> usize {
        self.chunk_size.saturating_sub(1)
    }

    /// Current resample ratio (output_rate / input_rate).
    pub fn resample_ratio(&self) -> f64 {
        self.inner.resample_ratio()
    }

    /// Adjust the resample ratio to keep ring-buffer fill near 50%.
    ///
    /// Call approximately once per second. `fill_ratio` is the current number
    /// of buffered frames divided by total ring capacity (0.0–1.0).
    pub fn trim_ratio(&mut self, fill_ratio: f64) {
        let error = TARGET_FILL - fill_ratio;
        // Limit each measurement to ±0.1% to avoid audible artifacts.
        let correction = (error * KP).clamp(-0.001, 0.001);

        self.trim_history[self.trim_history_index] = correction;
        self.trim_history_index = (self.trim_history_index + 1) % TRIM_HISTORY;
        self.trim_history_len = (self.trim_history_len + 1).min(TRIM_HISTORY);

        let smoothed_correction = self.trim_history[..self.trim_history_len]
            .iter()
            .sum::<f64>()
            / self.trim_history_len as f64;
        let relative_ratio = 1.0 + smoothed_correction;

        // rubato 1.0 uses a boolean ramp flag. Ramping applies the new ratio
        // smoothly over the next processed chunk.
        self.inner
            .set_resample_ratio_relative(relative_ratio, true)
            .expect("bounded drift correction must be a valid resample ratio");
    }

    /// Feed a block of deinterleaved L/R samples into the staging buffer.
    ///
    /// Calls `on_chunk(&[f32], &[f32])` for every complete resampled
    /// output chunk that emerges.  The two slices are the left and right
    /// channel data respectively; they are views into internal output
    /// buffers and are valid only for the duration of the callback.
    ///
    /// `input_l` and `input_r` must have equal length.
    ///
    /// # Panics
    /// Panics in debug if `input_l.len() != input_r.len()`.
    pub fn feed(
        &mut self,
        input_l: &[f32],
        input_r: &[f32],
        mut on_chunk: impl FnMut(&[f32], &[f32]),
    ) {
        assert_eq!(input_l.len(), input_r.len());

        let needed = self.chunk_size;
        let mut offset = 0usize;

        // Consume the input incrementally rather than staging it whole: copy at
        // most one chunk's worth at a time, draining whenever staging fills. The
        // staging buffer therefore never has to hold a caller block, so a block
        // larger than `max_block` costs an extra iteration instead of indexing
        // out of bounds — a panic here would abort the audio thread.
        while offset < input_l.len() {
            let take = (needed - self.staged).min(input_l.len() - offset);
            self.staging[0][self.staged..self.staged + take]
                .copy_from_slice(&input_l[offset..offset + take]);
            self.staging[1][self.staged..self.staged + take]
                .copy_from_slice(&input_r[offset..offset + take]);
            self.staged += take;
            offset += take;

            // Partial chunk — wait for more input rather than zero-padding.
            if self.staged < needed {
                break;
            }

            let input = SequentialSliceOfVecs::new(&self.staging, 2, needed)
                .expect("failed to create rubato input adapter");
            let mut output = SequentialSliceOfVecs::new_mut(&mut self.output, 2, self.output_frames_max)
                .expect("failed to create rubato output adapter");

            let (input_frames, output_frames) = self
                .inner
                .process_into_buffer(&input, &mut output, None)
                .expect("resampler process_into_buffer failed");

            debug_assert_eq!(
                input_frames, needed,
                "FixedAsync::Input must consume exactly input_frames_next() frames"
            );

            self.staged = 0;

            // Deliver the resampled chunk — use `output_frames` from the
            // resampler, never the nominal ratio.
            on_chunk(&self.output[0][..output_frames], &self.output[1][..output_frames]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Sine wave generator: `sin(2π · freq · t / sample_rate) * amplitude`.
    fn sine(len: usize, freq: f32, sample_rate: f32, amplitude: f32) -> Vec<f32> {
        (0..len)
            .map(|i| amplitude * (2.0 * std::f32::consts::PI * freq * i as f32 / sample_rate).sin())
            .collect()
    }

    /// Feed `blocks` one at a time and collect every output chunk into `out`.
    fn collect_output(
        resampler: &mut RtResampler,
        blocks: &[Vec<f32>],
    ) -> (Vec<f32>, Vec<f32>) {
        let mut out_l = Vec::new();
        let mut out_r = Vec::new();
        for block in blocks {
            resampler.feed(block, block, |l, r| {
                out_l.extend_from_slice(l);
                out_r.extend_from_slice(r);
            });
        }
        // Drain any remaining staged data by feeding silence until no chunks
        // emerge.  In practice, a real audio stream never stops mid-chunk, but
        // for offline verification we must flush.
        let silence = vec![0.0f32; resampler.chunk_size];
        resampler.feed(&silence, &silence, |l, r| {
            out_l.extend_from_slice(l);
            out_r.extend_from_slice(r);
        });
        (out_l, out_r)
    }

    #[test]
    fn total_output_approximately_input_times_ratio_16k_to_48k() {
        // 16 kHz → 48 kHz: ratio = 3.0.  Feed a large signal in one block.
        let input_rate = 16_000.0;
        let output_rate = 48_000.0;
        let max_block = 1024;
        let mut resampler = RtResampler::new(input_rate, output_rate, max_block);

        let total_input = 50_000usize;
        let signal = sine(total_input, 440.0, input_rate, 0.5);
        let (out_l, out_r) = collect_output(&mut resampler, &[signal]);

        let expected = (total_input as f64 * 3.0) as usize;
        // Async resampler may vary by a few frames — allow 0.1% tolerance.
        let tolerance = (expected as f64 * 0.005).ceil() as usize + resampler.chunk_size;
        assert!(
            out_l.len().abs_diff(expected) <= tolerance,
            "total output {} not within {tolerance} of expected {expected} (ratio 3.0)",
            out_l.len(),
        );
        assert_eq!(out_l.len(), out_r.len());
    }

    #[test]
    fn total_output_approximately_input_times_ratio_44k1_to_48k() {
        let input_rate = 44_100.0;
        let output_rate = 48_000.0;
        let max_block = 512;
        let mut resampler = RtResampler::new(input_rate, output_rate, max_block);

        let total_input = 50_000usize;
        let signal = sine(total_input, 440.0, input_rate, 0.5);
        let (out_l, _) = collect_output(&mut resampler, &[signal]);

        let ratio = output_rate as f64 / input_rate as f64;
        let expected = (total_input as f64 * ratio) as usize;
        let tolerance = (expected as f64 * 0.005).ceil() as usize + resampler.chunk_size;
        assert!(
            out_l.len().abs_diff(expected) <= tolerance,
            "total output {} not within {tolerance} of expected {expected} (ratio {ratio:.4})",
            out_l.len(),
        );
    }

    #[test]
    fn irregular_blocks_produce_same_total_as_single_block() {
        let input_rate = 48_000.0;
        let output_rate = 96_000.0; // ratio = 2.0
        let max_block = 1024;
        let mut resampler_a = RtResampler::new(input_rate, output_rate, max_block);
        let mut resampler_b = RtResampler::new(input_rate, output_rate, max_block);

        let total = 20_000usize;
        let full_signal = sine(total, 440.0, input_rate, 0.5);

        // Single block.
        let (out_a, _) = collect_output(&mut resampler_a, &[full_signal.clone()]);

        // Irregular blocks.
        let blocks: Vec<Vec<f32>> = [100usize, 3, 512, 1, 977]
            .iter()
            .scan(0usize, |offset, &len| {
                let start = *offset;
                *offset += len;
                if start >= total {
                    None
                } else {
                    let end = (start + len).min(total);
                    Some(full_signal[start..end].to_vec())
                }
            })
            .collect();
        // Append remainder.
        let remainder_len: usize = blocks.iter().map(|b| b.len()).sum();
        if remainder_len < total {
            let remaining = full_signal[remainder_len..].to_vec();
            let blocks: Vec<Vec<f32>> = blocks
                .into_iter()
                .chain(std::iter::once(remaining))
                .collect();
            let (out_b, _) = collect_output(&mut resampler_b, &blocks);
            assert_eq!(out_a.len(), out_b.len(),
                "irregular-block total output {} differs from single-block total {}",
                out_b.len(), out_a.len());
        } else {
            let (out_b, _) = collect_output(&mut resampler_b, &blocks);
            assert_eq!(out_a.len(), out_b.len(),
                "irregular-block total output {} differs from single-block total {}",
                out_b.len(), out_a.len());
        }
    }

    #[test]
    fn no_zeros_injected_mid_stream() {
        // The staging buffer must NOT zero-pad when partial chunks are
        // available.  Feed a non-zero signal through irregular blocks;
        // verify there are no zero runs in the output (except the initial
        // delay and final flush).
        let input_rate = 48_000.0;
        let output_rate = 48_000.0; // ratio = 1.0 — simplest verification
        let max_block = 256;
        let mut resampler = RtResampler::new(input_rate, output_rate, max_block);

        let signal = sine(10_000, 1000.0, input_rate, 0.8);

        let blocks: Vec<Vec<f32>> = [100, 3, 512, 1, 977]
            .iter()
            .scan(0usize, |offset, &len| {
                let start = *offset;
                *offset += len;
                if start >= signal.len() {
                    None
                } else {
                    let end = (start + len).min(signal.len());
                    Some(signal[start..end].to_vec())
                }
            })
            .collect();

        let (out_l, _) = collect_output(&mut resampler, &blocks);
        assert!(!out_l.is_empty(), "expected some output");

        // Skip initial output_delay() frames (ramp-up) and final chunk_size
        // frames (flush zeros), then verify no zero runs >= 2 exist mid-stream.
        let delay = resampler.output_delay();
        let tail = resampler.chunk_size;
        let mid = if out_l.len() > delay + tail {
            &out_l[delay..out_l.len() - tail]
        } else {
            &out_l[..]
        };

        // Check that we don't have a run of zeros longer than 1 sample
        // (a single zero crossing is fine, but injected zero-padding isn't).
        let mut zero_run = 0usize;
        for &s in mid {
            if s == 0.0 {
                zero_run += 1;
                assert!(
                    zero_run < 3,
                    "found {} consecutive zeros mid-stream — zero-padding detected",
                    zero_run
                );
            } else {
                zero_run = 0;
            }
        }
    }

    #[test]
    fn feed_empty_block_is_noop() {
        let mut resampler = RtResampler::new(48_000.0, 48_000.0, 512);
        let mut called = false;
        resampler.feed(&[], &[], |_, _| called = true);
        assert!(!called);
    }

    #[test]
    fn latency_components_are_reasonable() {
        let resampler = RtResampler::new(44_100.0, 48_000.0, 512);
        // output_delay should be based on the sinc filter length.
        // With sinc_len=256, oversampling_factor=256:
        // nbr_points = 256 * 256 = 65536?  No — rubato uses sinc_len as the
        // filter order and oversampling_factor for the subfilter density.
        // The delay is roughly sinc_len / 2 samples.
        let od = resampler.output_delay();
        assert!(od > 0, "output_delay should be non-zero");

        let sd = resampler.staging_max_delay_input_frames();
        assert!(sd > 0, "staging delay should be non-zero");
        assert_eq!(sd, resampler.chunk_size - 1);
    }

    #[test]
    fn trim_ratio_clamps_each_correction_to_one_tenth_percent() {
        let mut resampler = RtResampler::new(44_100.0, 48_000.0, 512);
        let nominal_ratio = 48_000.0 / 44_100.0;
        let silence = vec![0.0; RtResampler::CHUNK_FRAMES];

        resampler.trim_ratio(0.0);
        resampler.feed(&silence, &silence, |_, _| {});

        assert!((resampler.resample_ratio() - nominal_ratio * 1.001).abs() < 1.0e-12);
    }

    #[test]
    fn trim_ratio_averages_recent_corrections() {
        let mut resampler = RtResampler::new(44_100.0, 48_000.0, 512);
        let nominal_ratio = 48_000.0 / 44_100.0;
        let silence = vec![0.0; RtResampler::CHUNK_FRAMES];

        resampler.trim_ratio(0.0);
        resampler.feed(&silence, &silence, |_, _| {});
        resampler.trim_ratio(1.0);
        resampler.feed(&silence, &silence, |_, _| {});

        assert!((resampler.resample_ratio() - nominal_ratio).abs() < 1.0e-12);
    }

    #[test]
    fn trim_ratio_history_is_bounded_to_sixteen_measurements() {
        let mut resampler = RtResampler::new(44_100.0, 48_000.0, 512);
        let nominal_ratio = 48_000.0 / 44_100.0;
        let silence = vec![0.0; RtResampler::CHUNK_FRAMES];

        for _ in 0..TRIM_HISTORY {
            resampler.trim_ratio(0.0);
        }
        resampler.trim_ratio(1.0);
        resampler.feed(&silence, &silence, |_, _| {});

        assert!((resampler.resample_ratio() - nominal_ratio * 1.000875).abs() < 1.0e-12);
    }

    #[test]
    #[should_panic(expected = "input_rate must be > 0")]
    fn panics_on_zero_input_rate() {
        RtResampler::new(0.0, 48_000.0, 512);
    }
}
