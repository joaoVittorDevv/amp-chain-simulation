pub mod filters;
pub mod ambience;

use filters::{PreConditioningFilter, SpeakerModel};
use ambience::RoomAmbience;
use crate::core::state::plugin_params::CabinetDimension;

pub struct CabinetProcessor {
    pre_hpf_lpf: PreConditioningFilter,
    speaker_model: SpeakerModel,
    room_ambience: RoomAmbience,
}

impl CabinetProcessor {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            pre_hpf_lpf: PreConditioningFilter::new(sample_rate),
            speaker_model: SpeakerModel::new(sample_rate),
            room_ambience: RoomAmbience::new(),
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

    pub fn update_params(&mut self, mic_pos: f32, mic_dist: f32, cab_dim: CabinetDimension) {
        self.speaker_model.update_params(mic_pos, mic_dist, cab_dim);
        self.room_ambience.set_mix(mic_dist * 0.4); // slightly less than 50% max wet
    }

    pub fn process(&mut self, mut sample: f32) -> f32 {
        sample = self.pre_hpf_lpf.process(sample);
        sample = self.speaker_model.process(sample);
        sample = self.room_ambience.process(sample);
        sample
    }
}
