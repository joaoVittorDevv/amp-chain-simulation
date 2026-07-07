//! Peak envelope-following brickwall limiter.
//!
//! Zero-latency, sample-accurate attack, exponential release. Pure Rust — no
//! FFI, no oversampling, no lookahead — so plugin latency stays at 0.

/// Peak envelope-following brickwall limiter.
///
/// Guarantees that `|output| <= ceiling` for every sample: the envelope tracks
/// the instantaneous peak with an instant attack and an exponential release,
/// and gain reduction is applied by dividing down to the ceiling whenever the
/// envelope exceeds it.
pub struct PeakLimiter {
    /// Linear gain ceiling, e.g. `10^(-1/20) ≈ 0.891` for -1 dB.
    ceiling: f32,
    /// Current peak envelope state.
    envelope: f32,
    /// Per-sample exponential release coefficient in `(0, 1]`.
    release_coeff: f32,
}

impl PeakLimiter {
    /// Create a limiter for the given ceiling (dB), release (ms), and sample rate (Hz).
    pub fn new(ceiling_db: f32, release_ms: f32, sample_rate: f32) -> Self {
        let mut limiter = Self {
            ceiling: 1.0,
            envelope: 0.0,
            release_coeff: 1.0,
        };
        limiter.set_params(ceiling_db, release_ms, sample_rate);
        limiter
    }

    /// Recompute the linear ceiling and release coefficient from human-facing units.
    pub fn set_params(&mut self, ceiling_db: f32, release_ms: f32, sample_rate: f32) {
        self.ceiling = 10.0_f32.powf(ceiling_db / 20.0);
        let tau_samples = (release_ms * 0.001 * sample_rate).max(1.0);
        self.release_coeff = 1.0 - (-1.0 / tau_samples).exp();
    }

    /// Process one sample, returning the limited value (`|out| <= ceiling`).
    pub fn process(&mut self, sample: f32) -> f32 {
        let abs_sample = sample.abs();

        if abs_sample > self.envelope {
            // Instant attack — peaks are captured sample-accurate.
            self.envelope = abs_sample;
        } else {
            // Exponential release back down toward the current level.
            self.envelope += (abs_sample - self.envelope) * self.release_coeff;
        }

        if self.envelope > self.ceiling {
            sample * (self.ceiling / self.envelope)
        } else {
            sample
        }
    }

    /// Clear the envelope state (e.g. on transport reset).
    pub fn reset(&mut self) {
        self.envelope = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SR: f32 = 48_000.0;

    #[test]
    fn test_limiter_passes_quiet_signal() {
        let mut lim = PeakLimiter::new(-1.0, 50.0, SR);
        let out = lim.process(0.1);
        assert!((out - 0.1).abs() < 1e-6, "quiet signal altered: {out}");
    }

    #[test]
    fn test_limiter_attenuates_loud_signal() {
        let mut lim = PeakLimiter::new(-1.0, 50.0, SR);
        let out = lim.process(0.95);
        // -1 dB ceiling => 10^(-1/20) ≈ 0.8913
        assert!((out - 0.8913).abs() < 1e-3, "expected ~0.891, got {out}");
    }

    #[test]
    fn test_limiter_instant_attack() {
        let mut lim = PeakLimiter::new(-1.0, 50.0, SR);
        // Very first sample above the ceiling must already be attenuated.
        let out = lim.process(0.99);
        assert!(out.abs() <= 0.8913 + 1e-4, "no instant attack: {out}");
    }

    #[test]
    fn test_limiter_release_envelope() {
        let mut lim = PeakLimiter::new(-1.0, 50.0, SR);
        lim.process(0.95); // drive envelope up to 0.95
        let env_after_peak = lim.envelope;
        // Feed silence; envelope must decay toward 0.
        for _ in 0..100 {
            lim.process(0.0);
        }
        assert!(
            lim.envelope < env_after_peak,
            "envelope did not decay: {} -> {}",
            env_after_peak,
            lim.envelope
        );
        assert!(lim.envelope >= 0.0);
    }

    #[test]
    fn test_limiter_ceiling_zero_db() {
        let mut lim = PeakLimiter::new(0.0, 50.0, SR);
        let out = lim.process(1.0);
        // Ceiling is exactly 1.0; a 1.0 peak is not *above* it, so it passes.
        assert!((out - 1.0).abs() < 1e-6, "0 dB ceiling altered 1.0: {out}");
    }

    #[test]
    fn test_limiter_ceiling_minus_twelve_db() {
        let mut lim = PeakLimiter::new(-12.0, 50.0, SR);
        let out = lim.process(1.0);
        // -12 dB => 10^(-12/20) ≈ 0.2512
        assert!((out - 0.2512).abs() < 1e-3, "expected ~0.251, got {out}");
    }

    #[test]
    fn test_limiter_reset() {
        let mut lim = PeakLimiter::new(-1.0, 50.0, SR);
        lim.process(0.95);
        assert!(lim.envelope > 0.0);
        lim.reset();
        assert_eq!(lim.envelope, 0.0);
    }
}
