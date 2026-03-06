pub mod filters;
pub mod ambience;
pub mod ir_convolver;

use arc_swap::ArcSwapOption;
use std::sync::Arc;
use filters::{PreConditioningFilter, SpeakerModel};
use ambience::RoomAmbience;
use crate::core::state::plugin_params::CabinetDimension;
use ir_convolver::{IrConvolver, IrData};
use nih_plug::params::smoothing::AtomicF32;

pub struct CabinetProcessor {
    pre_hpf_lpf: PreConditioningFilter,
    speaker_model: SpeakerModel,
    ir_convolver: IrConvolver,
    room_ambience: RoomAmbience,
    use_custom_ir: bool,
    master_volume: f32,
    clipping_meter: Arc<AtomicF32>,
}

impl CabinetProcessor {
    pub fn new(sample_rate: f32, custom_ir: Arc<ArcSwapOption<IrData>>, meter: Arc<AtomicF32>) -> Self {
        Self {
            pre_hpf_lpf: PreConditioningFilter::new(sample_rate),
            speaker_model: SpeakerModel::new(sample_rate),
            ir_convolver: IrConvolver::new(custom_ir),
            room_ambience: RoomAmbience::new(),
            use_custom_ir: false,
            master_volume: 1.0,
            clipping_meter: meter,
        }
    }

    pub fn initialize(&mut self, max_sample_rate: f32) {
        self.pre_hpf_lpf.update_sample_rate(max_sample_rate);
        self.speaker_model.update_sample_rate(max_sample_rate);
        self.room_ambience.initialize(max_sample_rate);
        self.room_ambience.set_sample_rate(max_sample_rate);
    }
    
    pub fn reset_sample_rate(&mut self, sample_rate: f32) {
        self.pre_hpf_lpf.update_sample_rate(sample_rate);
        self.speaker_model.update_sample_rate(sample_rate);
        self.room_ambience.set_sample_rate(sample_rate);
    }

    pub fn update_params(&mut self, mic_pos: f32, mic_dist: f32, cab_dim: CabinetDimension, use_custom_ir: bool, master_vol: f32) {
        self.speaker_model.update_params(mic_pos, mic_dist, cab_dim);
        self.room_ambience.set_mix(mic_dist * 0.4); 
        self.use_custom_ir = use_custom_ir;
        self.master_volume = master_vol;
    }

    pub fn process(&mut self, mut sample: f32) -> f32 {
        sample = self.pre_hpf_lpf.process(sample);
        
        if self.use_custom_ir {
            sample = self.ir_convolver.process(sample);
        } else {
            sample = self.speaker_model.process(sample);
        }
        
        sample = self.room_ambience.process(sample);
        sample *= self.master_volume;

        // Visual Metering update (store max absolute peak)
        let abs_s = sample.abs();
        let current_peak = self.clipping_meter.load(std::sync::atomic::Ordering::Relaxed);
        if abs_s > current_peak {
            self.clipping_meter.store(abs_s, std::sync::atomic::Ordering::Relaxed);
        }

        sample
    }
}
