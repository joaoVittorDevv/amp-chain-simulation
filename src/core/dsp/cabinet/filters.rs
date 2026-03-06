use biquad::{Biquad, Coefficients, DirectForm2Transposed, ToHertz, Type, Q_BUTTERWORTH_F32};
use crate::core::state::plugin_params::CabinetDimension;

pub struct PreConditioningFilter {
    hpf: DirectForm2Transposed<f32>,
    lpf: DirectForm2Transposed<f32>,
}

impl PreConditioningFilter {
    pub fn new(sample_rate: f32) -> Self {
        // Use dummy coefficients to initialize, then update
        let default_coeffs = Coefficients::<f32> { a1: 0.0, a2: 0.0, b0: 1.0, b1: 0.0, b2: 0.0 };
        
        let mut filter = Self {
            hpf: DirectForm2Transposed::<f32>::new(default_coeffs),
            lpf: DirectForm2Transposed::<f32>::new(default_coeffs),
        };
        
        filter.update_sample_rate(sample_rate);
        filter
    }
    
    pub fn update_sample_rate(&mut self, sample_rate: f32) {
        let fs = sample_rate.hz();
        
        if let Ok(c) = Coefficients::<f32>::from_params(Type::HighPass, fs, 70.0_f32.hz(), Q_BUTTERWORTH_F32) {
            self.hpf.update_coefficients(c);
        }
        if let Ok(c) = Coefficients::<f32>::from_params(Type::LowPass, fs, 8000.0_f32.hz(), Q_BUTTERWORTH_F32) {
            self.lpf.update_coefficients(c);
        }
    }

    pub fn process(&mut self, sample: f32) -> f32 {
        self.lpf.run(self.hpf.run(sample))
    }
}

pub struct SpeakerModel {
    mic_position_lpf: DirectForm2Transposed<f32>,
    mic_position_shelf: DirectForm2Transposed<f32>,
    mic_distance_shelf: DirectForm2Transposed<f32>,
    cab_resonance: DirectForm2Transposed<f32>,
    cab_notch: DirectForm2Transposed<f32>,
    v30_presence_peak: DirectForm2Transposed<f32>,
    sample_rate: f32,
    cab_dim: CabinetDimension,
}

impl SpeakerModel {
    pub fn new(sample_rate: f32) -> Self {
        let default_coeffs = Coefficients::<f32> { a1: 0.0, a2: 0.0, b0: 1.0, b1: 0.0, b2: 0.0 };
        
        Self {
            mic_position_lpf: DirectForm2Transposed::<f32>::new(default_coeffs),
            mic_position_shelf: DirectForm2Transposed::<f32>::new(default_coeffs),
            mic_distance_shelf: DirectForm2Transposed::<f32>::new(default_coeffs),
            cab_resonance: DirectForm2Transposed::<f32>::new(default_coeffs),
            cab_notch: DirectForm2Transposed::<f32>::new(default_coeffs),
            v30_presence_peak: DirectForm2Transposed::<f32>::new(default_coeffs),
            sample_rate,
            cab_dim: CabinetDimension::OneByTwelve,
        }
    }
    
    pub fn update_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
    }

    pub fn update_params(&mut self, mic_pos: f32, mic_dist: f32, cab_dim: CabinetDimension) {
        let fs = self.sample_rate.hz();
        
        // Mic Position: 0.0 (Center) -> 1.0 (Edge)
        // 1. LPF from 10kHz down to 7kHz
        let lpf_hz = 10000.0 - (3000.0 * mic_pos);
        if let Ok(c) = Coefficients::<f32>::from_params(Type::LowPass, fs, lpf_hz.hz(), 0.707) {
            self.mic_position_lpf.update_coefficients(c);
        }
        
        // 2. High shelf down to -8dB above 3kHz (reduced from -12dB for more air)
        let high_shelf_gain = -8.0 * mic_pos;
        if let Ok(c) = Coefficients::<f32>::from_params(Type::HighShelf(high_shelf_gain), fs, 3000.0_f32.hz(), 0.707) {
            self.mic_position_shelf.update_coefficients(c);
        }

        // Mic Distance: 0.0 (Grille) -> 1.0 (Room)
        // 1. Low Shelf proximity effect +4dB down to 0dB around 120Hz
        let low_shelf_gain = 4.0 * (1.0 - mic_dist);
        if let Ok(c) = Coefficients::<f32>::from_params(Type::LowShelf(low_shelf_gain), fs, 120.0_f32.hz(), 0.707) {
            self.mic_distance_shelf.update_coefficients(c);
        }
        
        // Cabinet Dimension: 1x12 (200Hz), 2x12 (140Hz), 4x12 (100Hz)
        self.cab_dim = cab_dim;
        let res_hz = match cab_dim {
            CabinetDimension::OneByTwelve => 200.0_f32,
            CabinetDimension::TwoByTwelve => 140.0_f32,
            CabinetDimension::FourByTwelve => 100.0_f32,
        };
        // Resonance is a +3dB peak
        if let Ok(c) = Coefficients::<f32>::from_params(Type::PeakingEQ(3.0), fs, res_hz.hz(), 2.0) {
            self.cab_resonance.update_coefficients(c);
        }
        
        // Comb filter notches for 4x12
        if cab_dim == CabinetDimension::FourByTwelve {
            if let Ok(c) = Coefficients::<f32>::from_params(Type::Notch, fs, 5500.0_f32.hz(), 4.0) {
                self.cab_notch.update_coefficients(c);
            }
        }

        // V30 speaker presence peak: +4.5dB at 2500Hz, Q=1.2
        // Signature "bark" of a Vintage 30 — adds presence and cut
        let v30_gain_lin = 10.0_f32.powf(4.5 / 20.0); // +4.5 dB → ≈1.679 linear
        if let Ok(c) = Coefficients::<f32>::from_params(
            Type::PeakingEQ(v30_gain_lin), fs, 2500.0_f32.hz(), 1.2,
        ) {
            self.v30_presence_peak.update_coefficients(c);
        }
    }

    pub fn process(&mut self, mut sample: f32) -> f32 {
        sample = self.mic_position_lpf.run(sample);
        sample = self.mic_position_shelf.run(sample);
        sample = self.mic_distance_shelf.run(sample);
        sample = self.v30_presence_peak.run(sample);
        sample = self.cab_resonance.run(sample);
        if self.cab_dim == CabinetDimension::FourByTwelve {
            sample = self.cab_notch.run(sample);
        }
        sample
    }
}
