#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/bindings_mlc_zero_v.rs"));

use super::ExternalProcessor;
#[cfg(feature = "lab")]
use crate::lab::{DspVariant, ParameterMeta};

#[cfg(feature = "lab")]
pub const MLC_ZERO_V_IMPL_ID: &str = "mlc-zero-v";
#[cfg(feature = "lab")]
pub const MLC_ZERO_V_PARAM_IDS: [&str; 14] = [
    "mlc_gain",
    "mlc_master",
    "mlc_bass",
    "mlc_middle",
    "mlc_treble",
    "mlc_presence",
    "mlc_depth",
    "mlc_gate",
    "mlc_bright",
    "mlc_m45",
    "mlc_warclaw",
    "mlc_feedback",
    "mlc_gate_pos",
    "mlc_clip_type",
];

pub struct MlcZeroVProcessor {
    handle: FaustHandle,
    is_ready: bool,
    gain: f32,
    master: f32,
    bass: f32,
    middle: f32,
    treble: f32,
    presence: f32,
    depth: f32,
    gate: f32,
    bright: f32,
    m45: f32,
    warclaw: f32,
    feedback: f32,
    gate_pos: f32,
    clip_type: f32,
}

impl MlcZeroVProcessor {
    pub fn new() -> Option<Self> {
        let handle = unsafe { mlc_zero_v_create() };
        if handle.is_null() {
            None
        } else {
            Some(Self {
                handle,
                is_ready: false,
                gain: 0.25118864,
                master: 0.5011872,
                bass: 0.0,
                middle: 0.0,
                treble: 0.0,
                presence: 0.0,
                depth: 0.0,
                gate: -80.0,
                bright: 1.0,
                m45: 0.0,
                warclaw: 0.0,
                feedback: 1.0,
                gate_pos: 0.0,
                clip_type: 0.0,
            })
        }
    }

    #[inline(always)]
    pub fn set_gain(&mut self, value: f32) {
        self.gain = value;
    }
    #[inline(always)]
    pub fn set_master(&mut self, value: f32) {
        self.master = value;
    }
    #[inline(always)]
    pub fn set_bass(&mut self, value: f32) {
        self.bass = value;
    }
    #[inline(always)]
    pub fn set_middle(&mut self, value: f32) {
        self.middle = value;
    }
    #[inline(always)]
    pub fn set_treble(&mut self, value: f32) {
        self.treble = value;
    }
    #[inline(always)]
    pub fn set_presence(&mut self, value: f32) {
        self.presence = value;
    }
    #[inline(always)]
    pub fn set_depth(&mut self, value: f32) {
        self.depth = value;
    }
    #[inline(always)]
    pub fn set_gate(&mut self, value: f32) {
        self.gate = value;
    }
    #[inline(always)]
    pub fn set_bright(&mut self, value: f32) {
        self.bright = value;
    }
    #[inline(always)]
    pub fn set_m45(&mut self, value: bool) {
        self.m45 = if value { 1.0 } else { 0.0 };
    }
    #[inline(always)]
    pub fn set_warclaw(&mut self, value: bool) {
        self.warclaw = if value { 1.0 } else { 0.0 };
    }
    #[inline(always)]
    pub fn set_feedback(&mut self, value: f32) {
        self.feedback = value;
    }
    #[inline(always)]
    pub fn set_gate_pos(&mut self, value: f32) {
        self.gate_pos = value;
    }
    #[inline(always)]
    pub fn set_clip_type(&mut self, value: f32) {
        self.clip_type = value.clamp(0.0, 1.0).round();
    }
}

impl Drop for MlcZeroVProcessor {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe {
                mlc_zero_v_destroy(self.handle);
            }
        }
    }
}

unsafe impl Send for MlcZeroVProcessor {}

impl ExternalProcessor for MlcZeroVProcessor {
    fn init(&mut self, sample_rate: f32) {
        unsafe {
            mlc_zero_v_init(self.handle, sample_rate);
        }
        self.is_ready = true;
    }

    fn process_block(&mut self, buffer: *mut f32, length: usize) {
        if !self.is_ready {
            return;
        }

        unsafe {
            mlc_zero_v_set_gain(self.handle, self.gain);
            mlc_zero_v_set_master(self.handle, self.master);
            mlc_zero_v_set_bass(self.handle, self.bass);
            mlc_zero_v_set_middle(self.handle, self.middle);
            mlc_zero_v_set_treble(self.handle, self.treble);
            mlc_zero_v_set_presence(self.handle, self.presence);
            mlc_zero_v_set_depth(self.handle, self.depth);
            mlc_zero_v_set_gate(self.handle, self.gate);
            mlc_zero_v_set_bright(self.handle, self.bright);
            mlc_zero_v_set_m45(self.handle, self.m45);
            mlc_zero_v_set_warclaw(self.handle, self.warclaw);
            mlc_zero_v_set_feedback(self.handle, self.feedback);
            mlc_zero_v_set_gate_pos(self.handle, self.gate_pos);
            mlc_zero_v_set_clip_type(self.handle, self.clip_type);
            mlc_zero_v_process(self.handle, buffer, length as _);
        }
    }

    fn get_param(&self, id: &str) -> Option<f32> {
        match id {
            "mlc_gain" => Some(self.gain),
            "mlc_master" => Some(self.master),
            "mlc_bass" => Some(self.bass),
            "mlc_middle" => Some(self.middle),
            "mlc_treble" => Some(self.treble),
            "mlc_presence" => Some(self.presence),
            "mlc_depth" => Some(self.depth),
            "mlc_gate" => Some(self.gate),
            "mlc_bright" => Some(self.bright),
            "mlc_m45" => Some(self.m45),
            "mlc_warclaw" => Some(self.warclaw),
            "mlc_feedback" => Some(self.feedback),
            "mlc_gate_pos" => Some(self.gate_pos),
            "mlc_clip_type" => Some(self.clip_type),
            _ => None,
        }
    }

    fn set_param(&mut self, id: &str, value: f32) -> bool {
        match id {
            "mlc_gain" => self.set_gain(value),
            "mlc_master" => self.set_master(value),
            "mlc_bass" => self.set_bass(value),
            "mlc_middle" => self.set_middle(value),
            "mlc_treble" => self.set_treble(value),
            "mlc_presence" => self.set_presence(value),
            "mlc_depth" => self.set_depth(value),
            "mlc_gate" => self.set_gate(value),
            "mlc_bright" => self.set_bright(enum_value(value)),
            "mlc_m45" => self.set_m45(value >= 0.5),
            "mlc_warclaw" => self.set_warclaw(value >= 0.5),
            "mlc_feedback" => self.set_feedback(enum_value(value)),
            "mlc_gate_pos" => self.set_gate_pos(enum_value(value)),
            "mlc_clip_type" => self.set_clip_type(value),
            _ => return false,
        }
        true
    }

    fn param_metadata(&self) -> Vec<ParameterMeta> {
        mlc_zero_v_param_metadata()
    }
}

#[cfg(feature = "lab")]
impl DspVariant for MlcZeroVProcessor {
    fn process_block(&mut self, buffer: *mut f32, length: usize) {
        ExternalProcessor::process_block(self, buffer, length);
    }

    fn param_count(&self) -> usize {
        MLC_ZERO_V_PARAM_IDS.len()
    }

    fn param_ids(&self) -> &[&str] {
        &MLC_ZERO_V_PARAM_IDS
    }

    fn latency(&self) -> usize {
        0
    }

    fn get_param(&self, id: &str) -> Option<f32> {
        ExternalProcessor::get_param(self, id)
    }

    fn set_param(&mut self, id: &str, value: f32) -> bool {
        ExternalProcessor::set_param(self, id, value)
    }

    fn param_metadata(&self) -> Vec<ParameterMeta> {
        ExternalProcessor::param_metadata(self)
    }
}

#[cfg(feature = "lab")]
struct MlcZeroVBypassVariant;

#[cfg(feature = "lab")]
impl DspVariant for MlcZeroVBypassVariant {
    fn process_block(&mut self, _buffer: *mut f32, _length: usize) {}

    fn param_count(&self) -> usize {
        MLC_ZERO_V_PARAM_IDS.len()
    }

    fn param_ids(&self) -> &[&str] {
        &MLC_ZERO_V_PARAM_IDS
    }

    fn latency(&self) -> usize {
        0
    }

    fn get_param(&self, id: &str) -> Option<f32> {
        mlc_zero_v_param_metadata()
            .into_iter()
            .find(|meta| meta.id == id)
            .map(|meta| meta.default)
    }

    fn param_metadata(&self) -> Vec<ParameterMeta> {
        mlc_zero_v_param_metadata()
    }
}

#[cfg(feature = "lab")]
pub fn mlc_zero_v_factory(sample_rate: f32) -> Box<dyn DspVariant> {
    match MlcZeroVProcessor::new() {
        Some(mut processor) => {
            processor.init(sample_rate);
            Box::new(processor)
        }
        None => Box::new(MlcZeroVBypassVariant),
    }
}

fn enum_value(value: f32) -> f32 {
    value.clamp(0.0, 1.0).round()
}

#[cfg(feature = "lab")]
fn mlc_zero_v_param_metadata() -> Vec<ParameterMeta> {
    [
        ("mlc_gain", "MLC Gain", (0.001, 1.0), 0.25118864, Some("dB")),
        ("mlc_master", "MLC Master", (0.001, 1.0), 0.5011872, Some("dB")),
        ("mlc_bass", "MLC Bass", (-12.0, 12.0), 0.0, Some("dB")),
        ("mlc_middle", "MLC Middle", (-12.0, 12.0), 0.0, Some("dB")),
        ("mlc_treble", "MLC Treble", (-12.0, 12.0), 0.0, Some("dB")),
        (
            "mlc_presence",
            "MLC Presence",
            (-12.0, 12.0),
            0.0,
            Some("dB"),
        ),
        ("mlc_depth", "MLC Depth", (-12.0, 12.0), 0.0, Some("dB")),
        ("mlc_gate", "MLC Gate", (-80.0, 0.0), -80.0, Some("dB")),
        ("mlc_bright", "MLC Bright", (0.0, 1.0), 1.0, None),
        ("mlc_m45", "MLC M45", (0.0, 1.0), 0.0, None),
        ("mlc_warclaw", "MLC WARCLAW", (0.0, 1.0), 0.0, None),
        ("mlc_feedback", "MLC Feedback", (0.0, 1.0), 1.0, None),
        ("mlc_gate_pos", "MLC Gate Pos", (0.0, 1.0), 0.0, None),
        ("mlc_clip_type", "MLC Clip Type", (0.0, 1.0), 0.0, None),
    ]
    .into_iter()
    .enumerate()
    .map(|(index, (id, name, range, default, unit))| ParameterMeta {
        id: id.to_string(),
        name: name.to_string(),
        description: name.to_string(),
        range,
        default,
        unit: unit.map(str::to_string),
        smoothing: "50 ms".to_string(),
        index: index as u32,
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::MlcZeroVProcessor;
    use crate::bridge::ExternalProcessor;

    #[test]
    fn test_mlc_get_param_returns_stored_value() {
        let mut processor = MlcZeroVProcessor::new().expect("mlc processor");

        processor.set_gain(0.5);
        processor.set_m45(true);
        processor.set_feedback(0.0);

        assert_eq!(processor.get_param("mlc_gain"), Some(0.5));
        assert_eq!(processor.get_param("mlc_m45"), Some(1.0));
        assert_eq!(processor.get_param("mlc_feedback"), Some(0.0));

        assert!(processor.set_param("mlc_bright", 0.0));
        assert_eq!(processor.get_param("mlc_bright"), Some(0.0));

        // Clip type is quantized to the nearest valid curve index (0-1).
        assert!(processor.set_param("mlc_clip_type", 1.0));
        assert_eq!(processor.get_param("mlc_clip_type"), Some(1.0));
        assert!(processor.set_param("mlc_clip_type", 42.0));
        assert_eq!(processor.get_param("mlc_clip_type"), Some(1.0));

        assert_eq!(processor.get_param("missing"), None);
    }
}
