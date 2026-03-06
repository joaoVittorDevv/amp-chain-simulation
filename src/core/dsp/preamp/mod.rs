use biquad::{Biquad, Coefficients, DirectForm2Transposed, ToHertz, Type, Q_BUTTERWORTH_F32};
use crate::core::state::plugin_params::{PreampChannel, PreampDriveMode};

// ---------------------------------------------------------------------------
// Waveshaper helpers — using polynomial tanh approximation (CPU-efficient)
// All inlined so the compiler can aggressively optimize per-sample.
// ---------------------------------------------------------------------------

/// Fast polynomial tanh approximation — good for |x| < 3.0 range.
/// Uses the rational Padé [3,3] form instead of libm tanh() syscall.
#[inline(always)]
fn poly_tanh(x: f32) -> f32 {
    // Pade [3/3] approximation
    let x2 = x * x;
    let numerator = x * (27.0 + x2);
    let denominator = 27.0 + 9.0 * x2;
    numerator / denominator
}

/// Hard clip to [-ceiling, +ceiling].
#[inline(always)]
fn hard_clip(x: f32, ceiling: f32) -> f32 {
    x.clamp(-ceiling, ceiling)
}

/// Asymmetric soft-clip for a more organic "tube-like" feel.
/// Positive and negative halves are treated slightly differently.
#[inline(always)]
fn asymmetric_soft_clip(x: f32) -> f32 {
    if x >= 0.0 {
        poly_tanh(x)
    } else {
        // Slightly softer negative half — emulates asymmetric tube bias
        poly_tanh(x * 0.92) * 1.05
    }
}

// ---------------------------------------------------------------------------
// Pre-distortion HPF for High-Gain mode (removes muddy low-end before clip)
// ---------------------------------------------------------------------------
pub struct TightHpf {
    filter: DirectForm2Transposed<f32>,
}

impl TightHpf {
    pub fn new(sample_rate: f32) -> Self {
        let default_coeffs = Coefficients::<f32> { a1: 0.0, a2: 0.0, b0: 1.0, b1: 0.0, b2: 0.0 };
        let mut s = Self { filter: DirectForm2Transposed::<f32>::new(default_coeffs) };
        s.set_sample_rate(sample_rate);
        s
    }

    pub fn set_sample_rate(&mut self, sr: f32) {
        if let Ok(c) = Coefficients::<f32>::from_params(Type::HighPass, sr.hz(), 150.0_f32.hz(), Q_BUTTERWORTH_F32) {
            self.filter.update_coefficients(c);
        }
    }

    #[inline(always)]
    pub fn process(&mut self, x: f32) -> f32 {
        self.filter.run(x)
    }
}

// ---------------------------------------------------------------------------
// Post-distortion LPF for High-Gain mode (removes digital fizz)
// ---------------------------------------------------------------------------
pub struct FizzKillerLpf {
    filter: DirectForm2Transposed<f32>,
}

impl FizzKillerLpf {
    pub fn new(sample_rate: f32) -> Self {
        let default_coeffs = Coefficients::<f32> { a1: 0.0, a2: 0.0, b0: 1.0, b1: 0.0, b2: 0.0 };
        let mut s = Self { filter: DirectForm2Transposed::<f32>::new(default_coeffs) };
        s.set_sample_rate(sample_rate);
        s
    }

    pub fn set_sample_rate(&mut self, sr: f32) {
        if let Ok(c) = Coefficients::<f32>::from_params(Type::LowPass, sr.hz(), 12000.0_f32.hz(), Q_BUTTERWORTH_F32) {
            self.filter.update_coefficients(c);
        }
    }

    #[inline(always)]
    pub fn process(&mut self, x: f32) -> f32 {
        self.filter.run(x)
    }
}

// ---------------------------------------------------------------------------
// 3-Band Parametric EQ (Bass / Mid / Treble) — post-distortion
// ---------------------------------------------------------------------------
pub struct ThreeBandEq {
    bass:   DirectForm2Transposed<f32>,
    mid:    DirectForm2Transposed<f32>,
    treble: DirectForm2Transposed<f32>,
    sample_rate: f32,
}

impl ThreeBandEq {
    pub fn new(sample_rate: f32) -> Self {
        let default_coeffs = Coefficients::<f32> { a1: 0.0, a2: 0.0, b0: 1.0, b1: 0.0, b2: 0.0 };
        let mut eq = Self {
            bass:   DirectForm2Transposed::<f32>::new(default_coeffs),
            mid:    DirectForm2Transposed::<f32>::new(default_coeffs),
            treble: DirectForm2Transposed::<f32>::new(default_coeffs),
            sample_rate,
        };
        // Initialize with 0 dB (flat)
        eq.update_params(0.0, 0.0, 0.0);
        eq
    }

    pub fn set_sample_rate(&mut self, sr: f32) {
        self.sample_rate = sr;
        // Recalculate with current gains — require stored values, so pass 0 for now.
        // update_params() is called on each block anyway.
    }

    /// gain_db values in range [-15.0, +15.0].
    /// FIX: converte dB → linear antes de passar para PeakingEQ.
    /// A crate `biquad` espera voltage gain (1.0 = flat, 5.62 = +15dB),
    /// não dB bruto. Passar 15.0 diretamente = +23.5dB real, causando
    /// acúmulo massivo nos 3 filtros em cascata.
    pub fn update_params(&mut self, bass_db: f32, mid_db: f32, treble_db: f32) {
        let fs = self.sample_rate.hz();

        // dB → linear: 10^(dB/20)
        let bass_lin   = 10.0_f32.powf(bass_db   / 20.0);
        let mid_lin    = 10.0_f32.powf(mid_db    / 20.0);
        let treble_lin = 10.0_f32.powf(treble_db / 20.0);

        // FIX Q: Bass Q 0.9 → 0.5 (shelf-like, característico de amp)
        if let Ok(c) = Coefficients::<f32>::from_params(Type::PeakingEQ(bass_lin),   fs, 100.0_f32.hz(), 0.5) {
            self.bass.update_coefficients(c);
        }
        if let Ok(c) = Coefficients::<f32>::from_params(Type::PeakingEQ(mid_lin),    fs, 700.0_f32.hz(), 1.2) {
            self.mid.update_coefficients(c);
        }
        if let Ok(c) = Coefficients::<f32>::from_params(Type::PeakingEQ(treble_lin), fs, 3000.0_f32.hz(), 1.2) {
            self.treble.update_coefficients(c);
        }
    }

    #[inline(always)]
    pub fn process(&mut self, mut x: f32) -> f32 {
        x = self.bass.run(x);
        x = self.mid.run(x);
        x = self.treble.run(x);
        x
    }
}

// ---------------------------------------------------------------------------
// PreampProcessor — public API
// ---------------------------------------------------------------------------
pub struct PreampProcessor {
    tight_hpf:     TightHpf,
    fizz_lpf:      FizzKillerLpf,
    pre_mid_boost: DirectForm2Transposed<f32>,
    eq:            ThreeBandEq,
    sample_rate:   f32,
}

impl PreampProcessor {
    pub fn new(sample_rate: f32) -> Self {
        // Dummy coefficients for zero-allocation init
        let default_coeffs = Coefficients::<f32> { a1: 0.0, a2: 0.0, b0: 1.0, b1: 0.0, b2: 0.0 };
        let mut proc = Self {
            tight_hpf:     TightHpf::new(sample_rate),
            fizz_lpf:      FizzKillerLpf::new(sample_rate),
            pre_mid_boost: DirectForm2Transposed::<f32>::new(default_coeffs),
            eq:            ThreeBandEq::new(sample_rate),
            sample_rate,
        };
        proc.update_pre_mid_boost(sample_rate);
        proc
    }

    /// Pre-allocate internal state for worst-case sample rate.
    /// Must be called during `initialize()` on the plugin—NOT the audio thread.
    pub fn initialize(&mut self, max_sample_rate: f32) {
        self.set_sample_rate(max_sample_rate);
    }

    /// Update the pre-distortion mid-push filter ("Tubescreamer" boost).
    /// 700Hz, +4.0dB, Q=1.0 — only used in HighGain mode.
    fn update_pre_mid_boost(&mut self, sr: f32) {
        // +4.0 dB → linear voltage gain = 10^(4.0/20) ≈ 1.585
        let gain_lin = 10.0_f32.powf(4.0 / 20.0);
        if let Ok(c) = Coefficients::<f32>::from_params(
            Type::PeakingEQ(gain_lin), sr.hz(), 700.0_f32.hz(), 1.0,
        ) {
            self.pre_mid_boost.update_coefficients(c);
        }
    }

    pub fn set_sample_rate(&mut self, sr: f32) {
        self.sample_rate = sr;
        self.tight_hpf.set_sample_rate(sr);
        self.fizz_lpf.set_sample_rate(sr);
        self.update_pre_mid_boost(sr);
        self.eq.set_sample_rate(sr);
    }

    /// Called once per block (NOT per sample) with smoothed param values.
    /// Zero allocations — only coefficient math.
    pub fn update_params(
        &mut self,
        bass_db: f32,
        mid_db: f32,
        treble_db: f32,
    ) {
        self.eq.update_params(bass_db, mid_db, treble_db);
    }

    /// Per-sample processing. Allocation-free.
    ///
    /// Signal flow:
    ///   input_vol → [HPF if HiGain] → waveshaper → [LPF if HiGain] → EQ → master_vol
    #[inline]
    pub fn process(
        &mut self,
        x: f32,
        input_vol: f32,
        gain: f32,
        channel: PreampChannel,
        drive_mode: PreampDriveMode,
        master_vol: f32,
    ) -> f32 {
        // 1. Input volume (linear)
        let mut s = x * input_vol * 2.0; // 0.5 = unity gain

        // 2. Channel processing
        s = match channel {
            PreampChannel::Clean => {
                // FIX: drive quadrático para sensação musical no fader.
                // Antes: threshold 0.9 nunca atingido (sinal de guitarra ~0.005).
                // Agora: tanh SEMPRE ativo, controlado por drive_factor.
                // gain=0.0 → drive_factor=1.0 (quase linear)
                // gain=1.0 → drive_factor=9.0 (+19dB, warm overdrive)
                let drive_factor = 1.0 + gain * gain * 8.0;
                asymmetric_soft_clip(s * drive_factor)
            }
            PreampChannel::Dirty => {
                // FIX: escala logarítmica para o drive.
                // Antes: drive_linear = 1 + gain*15 → máx 16x (+24dB).
                //        Um sinal de 0.005 * 16 = 0.08 → zona linear do tanh.
                // Agora: 10^(gain * 3) → 1x a 1000x (+0 a +60dB).
                //        0.005 * 100x = 0.5 → saturação clara a gain=0.67.
                //        0.005 * 1000x = 5.0 → clipagem total a gain=1.0.

                match drive_mode {
                    PreampDriveMode::ModerateDrive => {
                        // Moderate: sweep de 10x a 316x (+20dB a +50dB)
                        let drive_linear = 10.0_f32.powf(1.0 + gain * 2.5);

                        // Pre-distortion HPF: prevent fuzz-like low-end mud
                        s = self.tight_hpf.process(s);

                        asymmetric_soft_clip(s * drive_linear)
                    }
                    PreampDriveMode::HighGain => {
                        // High-Gain: começa mais quente, sweep de 50x a 2000x (+34dB a +66dB)
                        let drive_linear = 10.0_f32.powf(1.7 + gain * 2.8);

                        // 1. Pre-distortion HPF: remove low-end antes da clipagem
                        s = self.tight_hpf.process(s);

                        // 2. Tubescreamer mid-push: +4dB at 700Hz for cut & presence
                        s = self.pre_mid_boost.run(s);

                        // 3. Multi-stage saturation
                        s *= drive_linear;
                        s = hard_clip(s, 1.2);          // stage 1: hard clip
                        s = asymmetric_soft_clip(s);    // stage 2: tube softening
                        s *= 1.15;                      // hot boost post-clip
                        s = hard_clip(s, 1.0);          // output guard

                        // 4. Post-distortion LPF: remove fizz digital
                        s = self.fizz_lpf.process(s);
                        s
                    }
                }
            }
        };

        // 3. 3-Band EQ (applied post-distortion)
        s = self.eq.process(s);

        // 4. Master Volume
        s * master_vol
    }
}
