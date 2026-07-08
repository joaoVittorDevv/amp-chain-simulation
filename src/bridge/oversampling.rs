//! Lanczos3 linear-phase oversampler.
//!
//! Adapted from nih_plug's `soft_vacuum` example plugin
//! (Robbert van der Helm, GPL-3.0). The only substantive change from the
//! upstream source is swapping the `nih_debug_assert*` macros for the standard
//! `debug_assert*` macros so this module does not depend on any nih_plug
//! internals — it is used from both the plugin and the standalone binary.
//!
//! Each `Lanczos3Oversampler` handles a **single** audio channel. Use one
//! instance per channel. The oversampling factor is expressed as the 2-logarithm
//! of the oversampling amount: `0` = 1x (bypass), `1` = 2x, `2` = 4x, and so on.

/// The kernel used in `Lanczos3Oversampler`. Specified here as a constant since it is a constant.
/// Precomputed since compile-time floating point arithmetic is still unstable.
///
/// Computed using:
///
/// ```python
/// LANCZOS_A = 3
///
/// x = np.arange(-LANCZOS_A * 2 + 1, LANCZOS_A * 2) / 2
/// np.sinc(x) * np.sinc(x / LANCZOS_A)
/// ```
///
/// Note the `+1` at the start of the range and the lack of `+1` at the (exclusive) end of the
/// range. This is because we can ommit the first and last point because they are always zero.
const LANCZOS3_UPSAMPLING_KERNEL: [f32; 11] = [
    0.02431708,
    -0.0,
    -0.13509491,
    0.0,
    0.6079271,
    1.0,
    0.6079271,
    0.0,
    -0.13509491,
    -0.0,
    0.02431708,
];

/// `LANCZOS3_UPSAMPLING_KERNEL` divided by two, used for downsampling so that upsampling followed
/// by downsampling results in unity gain.
const LANCZOS3_DOWNSAMPLING_KERNEL: [f32; 11] = [
    0.01215854,
    -0.0,
    -0.06754746,
    0.0,
    0.30396355,
    0.5,
    0.30396355,
    0.0,
    -0.06754746,
    -0.0,
    0.01215854,
];

/// The latency introduced by the two filter kernels defined above, in samples.
const LANZCOS3_KERNEL_LATENCY: usize = LANCZOS3_UPSAMPLING_KERNEL.len() / 2;

/// A barebones multi-stage linear-phase oversampler that uses the lanzcos kernel with a=3 for a
/// good approximation of a windowed sinc with only a 11 point kernel function (the kernel is
/// actually 13 points, but the outer two points are both zero can can thus be omitted).
///
/// This only handles a single audio channel. Use multiple instances for multichannel audio.
#[derive(Debug)]
pub struct Lanczos3Oversampler {
    /// The state used for each oversampling stage. Also contains stages that are not being used, so
    /// the number of stages can change without allocating.
    stages: Vec<Lanzcos3Stage>,

    /// The oversampler's latency. Precomputed for each possible number of active stages.
    latencies: Vec<u32>,
}

/// A single oversampling stage. Contains the ring buffers and current position in that ringbuffer
/// used for convolving the filter with the inputs in the upsampling and downsampling parts of the
/// stage.
#[derive(Debug, Clone)]
struct Lanzcos3Stage {
    /// The amount of oversampling that happens at this stage. Will be 2 for the first stage, 4 for
    /// the second stage, 8 for the third stage, and so forth.
    oversampling_amount: usize,

    /// These ring buffers contain `LANCZOS3_UPSAMPLING_KERNEL.len()` samples plus room for the
    /// additional delay needed to keep the total latency an integer at the base sample rate.
    upsampling_rb: Vec<f32>,
    upsampling_write_pos: usize,
    /// The additional delay for the upsampling needed to make this stage impose an integer amount
    /// of latency.
    additional_upsampling_latency: usize,

    downsampling_rb: [f32; LANCZOS3_DOWNSAMPLING_KERNEL.len()],
    downsampling_write_pos: usize,

    scratch_buffer: Vec<f32>,
}

impl Lanczos3Oversampler {
    /// Create a new oversampler that can oversample to up to the specified oversampling factor, or
    /// the 2-logarithm of the oversampling amount. 1x oversampling (aka, do nothing) = 0, 2x
    /// oversampling = 1, 4x oversampling = 2, etc. The actual amount of oversampling stages used is
    /// passed to the `process()` function, and must be set to `max_factor` or lower.
    pub fn new(maximum_block_size: usize, max_factor: usize) -> Self {
        let mut stages = Vec::with_capacity(max_factor);
        for stage in 0..max_factor {
            stages.push(Lanzcos3Stage::new(maximum_block_size, stage))
        }

        // Since the number of active oversampling stages is passed to the process function, we also
        // need to know the effective latencies of all possible oversampling settings in advance.
        let latencies = stages
            .iter()
            .map(|stage| stage.effective_latency())
            .scan(0, |total_latency, latency| {
                *total_latency += latency;
                Some(*total_latency)
            })
            .collect();

        Self { stages, latencies }
    }

    /// Reset the oversampling filters to their initial states.
    pub fn reset(&mut self) {
        for stage in &mut self.stages {
            stage.reset();
        }
    }

    /// The maximum oversampling factor this instance was constructed for.
    pub fn max_factor(&self) -> usize {
        self.stages.len()
    }

    /// The maximum block size (at the base sample rate) this instance can process.
    pub fn max_block_size(&self) -> usize {
        // Each stage's scratch buffer is `maximum_block_size * oversampling_amount` long, and the
        // first stage oversamples by 2, so the base-rate block limit is `scratch_len / 2`.
        self.stages
            .first()
            .map_or(usize::MAX, |stage| stage.scratch_buffer.len() / 2)
    }

    /// Get the latency in samples for the given oversampling factor. Fractional latency is
    /// automatically avoided.
    ///
    /// # Panics
    ///
    /// Panics if `factor > max_factor`.
    pub fn latency(&self, factor: usize) -> u32 {
        if factor == 0 {
            0
        } else {
            self.latencies[factor - 1]
        }
    }

    /// Upsample `block` using the specified oversampling factor, process the upsampled version
    /// using `f`, and then downsample it again and write the results back to `block` with a
    /// [`latency()`][Self::latency()] sample delay.
    ///
    /// # Panics
    ///
    /// Panics if `factor > max_factor`, or if `block`'s length is longer than the maximum block
    /// size.
    pub fn process(&mut self, block: &mut [f32], factor: usize, f: impl FnOnce(&mut [f32])) {
        assert!(factor <= self.stages.len());

        // This is the 1x oversampling case, this should also modify the block to be consistent
        if factor == 0 {
            f(block);
            return;
        }

        assert!(
            block.len() <= self.stages[0].scratch_buffer.len() / 2,
            "The block's size exceeds the maximum block size"
        );

        let upsampled = self.upsample_from(block, factor);
        f(upsampled);
        self.downsample_to(block, factor)
    }

    /// Upsample `block` through `factor` oversampling stages. Returns a reference to the
    /// oversampled output stored in the last `LancZos3Stage`'s scratch buffer **with the correct
    /// length**.
    fn upsample_from(&mut self, block: &[f32], factor: usize) -> &mut [f32] {
        assert_ne!(factor, 0);
        assert!(factor <= self.stages.len());

        // The first stage is upsampled from `block`, and everything after that is upsampled from
        // the stage preceeding it
        self.stages[0].upsample_from(block);

        let mut previous_upsampled_block_len = block.len() * 2;
        for to_stage_idx in 1..factor {
            let ([.., from], [to, ..]) = self.stages.split_at_mut(to_stage_idx) else {
                unreachable!()
            };

            to.upsample_from(&from.scratch_buffer[..previous_upsampled_block_len]);
            previous_upsampled_block_len *= 2;
        }

        &mut self.stages[factor - 1].scratch_buffer[..previous_upsampled_block_len]
    }

    /// Downsample starting from the `factor`th oversampling stage, writing the results from
    /// downsampling the first stage to `block`.
    fn downsample_to(&mut self, block: &mut [f32], factor: usize) {
        assert_ne!(factor, 0);
        assert!(factor <= self.stages.len());

        let mut next_downsampled_block_len = block.len() * 2usize.pow(factor as u32 - 1);
        for to_stage_idx in (1..factor).rev() {
            let ([.., to], [from, ..]) = self.stages.split_at_mut(to_stage_idx) else {
                unreachable!()
            };

            from.downsample_to(&mut to.scratch_buffer[..next_downsampled_block_len]);
            next_downsampled_block_len /= 2;
        }

        // And then the first stage downsamples to `block`
        assert_eq!(next_downsampled_block_len, block.len());
        self.stages[0].downsample_to(block);
    }
}

impl Lanzcos3Stage {
    /// Create a `stage_number`th oversampling stage, where `stage_number` is this stage's
    /// zero-based index in a list of stages. Stage 0 handles the 2x oversampling, stage 1 handles
    /// the 4x oversampling, stage 2 handles the 8x oversampling, etc.
    pub fn new(maximum_block_size: usize, stage_number: usize) -> Self {
        let oversampling_amount = 2usize.pow(stage_number as u32 + 1);

        assert!(LANCZOS3_UPSAMPLING_KERNEL.len() == LANCZOS3_DOWNSAMPLING_KERNEL.len());
        assert!(LANCZOS3_UPSAMPLING_KERNEL.len() % 2 == 1);

        // This is the latency of the upsampling and downsampling filter, at the base sample rate.
        // The delay imposed at the higher sample rate must be divisible by `oversampling_amount` so
        // the effective latency at the base rate is an integer amount.
        let uncompensated_stage_latency = LANZCOS3_KERNEL_LATENCY + LANZCOS3_KERNEL_LATENCY;

        let additional_delay_required = (-(uncompensated_stage_latency as isize))
            .rem_euclid(oversampling_amount as isize)
            as usize;

        Self {
            oversampling_amount,

            upsampling_rb: vec![0.0; LANCZOS3_UPSAMPLING_KERNEL.len() + additional_delay_required],
            upsampling_write_pos: 0,
            additional_upsampling_latency: additional_delay_required,

            downsampling_rb: [0.0; LANCZOS3_DOWNSAMPLING_KERNEL.len()],
            downsampling_write_pos: 0,

            scratch_buffer: vec![0.0; maximum_block_size * oversampling_amount],
        }
    }

    pub fn reset(&mut self) {
        self.upsampling_rb.fill(0.0);
        self.upsampling_write_pos = 0;

        self.downsampling_rb.fill(0.0);
        self.downsampling_write_pos = 0;
    }

    /// The stage's effect on the oversampling's latency as a whole. This is already divided by the
    /// stage's oversampling amount.
    pub fn effective_latency(&self) -> u32 {
        let uncompensated_stage_latency = LANZCOS3_KERNEL_LATENCY + LANZCOS3_KERNEL_LATENCY;
        let total_stage_latency = uncompensated_stage_latency + self.additional_upsampling_latency;

        let effective_latency = total_stage_latency as f32 / self.oversampling_amount as f32;
        assert!(effective_latency.fract() == 0.0);

        effective_latency as u32
    }

    /// Upsample `block` 2x and write the results to this stage's scratch buffer.
    pub fn upsample_from(&mut self, block: &[f32]) {
        let output_length = block.len() * 2;
        assert!(output_length <= self.scratch_buffer.len());

        // We'll first zero-stuff the input, and then run that through the lanczos halfband filter
        for (input_sample_idx, input_sample) in block.iter().enumerate() {
            let output_sample_idx = input_sample_idx * 2;
            self.scratch_buffer[output_sample_idx] = *input_sample;
            self.scratch_buffer[output_sample_idx + 1] = 0.0;
        }

        let mut direct_read_pos =
            (self.upsampling_write_pos + LANZCOS3_KERNEL_LATENCY) % self.upsampling_rb.len();
        for output_sample_idx in 0..output_length {
            self.upsampling_rb[self.upsampling_write_pos] = self.scratch_buffer[output_sample_idx];

            self.upsampling_write_pos += 1;
            if self.upsampling_write_pos == self.upsampling_rb.len() {
                self.upsampling_write_pos = 0;
            }

            direct_read_pos += 1;
            if direct_read_pos == self.upsampling_rb.len() {
                direct_read_pos = 0;
            }

            self.scratch_buffer[output_sample_idx] =
                if output_sample_idx % 2 == (LANZCOS3_KERNEL_LATENCY % 2) {
                    debug_assert_eq!(
                        self.upsampling_rb[(direct_read_pos + self.upsampling_rb.len() - 1)
                            % self.upsampling_rb.len()],
                        0.0
                    );
                    debug_assert_eq!(
                        self.upsampling_rb[(direct_read_pos + 1) % self.upsampling_rb.len()],
                        0.0
                    );

                    self.upsampling_rb[direct_read_pos]
                } else {
                    convolve_rb(
                        &self.upsampling_rb,
                        &LANCZOS3_UPSAMPLING_KERNEL,
                        self.upsampling_write_pos,
                    )
                };
        }
    }

    /// Downsample 2x from this stage's scratch buffer, writing the results to `block`.
    pub fn downsample_to(&mut self, block: &mut [f32]) {
        let input_length = block.len() * 2;
        assert!(input_length <= self.scratch_buffer.len());

        for input_sample_idx in 0..input_length {
            self.downsampling_rb[self.downsampling_write_pos] =
                self.scratch_buffer[input_sample_idx];

            self.downsampling_write_pos += 1;
            if self.downsampling_write_pos == LANCZOS3_DOWNSAMPLING_KERNEL.len() {
                self.downsampling_write_pos = 0;
            }

            if input_sample_idx % 2 == 0 {
                let output_sample_idx = input_sample_idx / 2;
                block[output_sample_idx] = convolve_rb(
                    &self.downsampling_rb,
                    &LANCZOS3_DOWNSAMPLING_KERNEL,
                    self.downsampling_write_pos,
                )
            }
        }
    }
}

/// Convolve `input_ring_buffer` with `kernel`, with `input_ring_buffer` rotated so that it starts
/// at `ring_buffer_pos` and then wraps back around to the start.
fn convolve_rb(input_ring_buffer: &[f32], kernel: &[f32], ring_buffer_pos: usize) -> f32 {
    let mut total = 0.0;

    debug_assert!(input_ring_buffer.len() >= kernel.len());

    let num_samples_until_wraparound =
        (input_ring_buffer.len() - ring_buffer_pos).min(kernel.len());
    for (read_pos_offset, kernel_sample) in kernel
        .iter()
        .rev()
        .take(num_samples_until_wraparound)
        .enumerate()
    {
        total += kernel_sample * input_ring_buffer[ring_buffer_pos + read_pos_offset];
    }

    for (read_pos, kernel_sample) in kernel
        .iter()
        .rev()
        .skip(num_samples_until_wraparound)
        .enumerate()
    {
        total += kernel_sample * input_ring_buffer[read_pos];
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    fn argmax(iter: impl IntoIterator<Item = f32>) -> usize {
        iter.into_iter()
            .enumerate()
            .max_by(|(_, value_a), (_, value_b)| value_a.total_cmp(value_b))
            .unwrap()
            .0
    }

    /// Makes sure that the reported latency matches where the delta impulse lands.
    fn test_latency(oversampling_factor: usize) {
        let mut delta_impulse = [0.0f32; 64];
        delta_impulse[0] = 1.0;

        let mut oversampler = Lanczos3Oversampler::new(delta_impulse.len(), oversampling_factor);

        let reported_latency = oversampler.latency(oversampling_factor) as usize;
        assert!(delta_impulse.len() > reported_latency);

        oversampler.process(&mut delta_impulse, oversampling_factor, |_| ());

        let new_impulse_idx = argmax(delta_impulse);
        assert_eq!(new_impulse_idx, reported_latency);
        assert!(delta_impulse[new_impulse_idx] > delta_impulse[new_impulse_idx - 1]);
        assert!(delta_impulse[new_impulse_idx] > delta_impulse[new_impulse_idx + 1]);
    }

    #[test]
    fn latency_1x_is_zero() {
        let oversampler = Lanczos3Oversampler::new(64, 2);
        assert_eq!(oversampler.latency(0), 0);
    }

    #[test]
    fn latency_2x() {
        test_latency(1);
    }

    #[test]
    fn latency_4x() {
        test_latency(2);
    }

    /// 1x oversampling (factor 0) must still run the callback and leave the block otherwise
    /// unchanged apart from the callback's own effect.
    #[test]
    fn bypass_runs_callback_without_latency() {
        let mut block = [0.25f32, -0.5, 0.75, -1.0];
        let mut oversampler = Lanczos3Oversampler::new(block.len(), 2);
        oversampler.process(&mut block, 0, |up| {
            for s in up.iter_mut() {
                *s *= 2.0;
            }
        });
        assert_eq!(block, [0.5, -1.0, 1.5, -2.0]);
    }

    /// A sine passed through the oversampler and gained up should match the gained input once the
    /// reported latency is compensated for.
    #[test]
    fn sine_output_4x_matches_input() {
        const GAIN: f32 = 2.0;
        const FREQUENCY: f32 = 0.125;

        let mut input = [0.0f32; 128];
        for (i, sample) in input.iter_mut().enumerate() {
            *sample = (i as f32 * (FREQUENCY * 2.0 * std::f32::consts::PI)).sin();
        }

        let mut output = input;
        let mut oversampler = Lanczos3Oversampler::new(output.len(), 2);
        oversampler.process(&mut output, 2, |upsampled| {
            for sample in upsampled {
                *sample *= GAIN;
            }
        });

        let reported_latency = oversampler.latency(2) as usize;
        for (input_sample_idx, input_sample) in input
            .into_iter()
            .enumerate()
            .take(input.len() - reported_latency)
        {
            let output_sample = output[input_sample_idx + reported_latency];
            assert!((input_sample * GAIN - output_sample).abs() < 0.1);
        }
    }
}
