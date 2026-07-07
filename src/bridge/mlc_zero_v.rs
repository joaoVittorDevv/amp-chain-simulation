#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/bindings_mlc_zero_v.rs"));

use super::ExternalProcessor;
#[cfg(feature = "lab")]
use crate::lab::DspVariant;

#[cfg(feature = "lab")]
pub const MLC_ZERO_V_IMPL_ID: &str = "mlc-zero-v";
#[cfg(feature = "lab")]
pub const MLC_ZERO_V_PARAM_IDS: [&str; 13] = [
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
            mlc_zero_v_process(self.handle, buffer, length as _);
        }
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
