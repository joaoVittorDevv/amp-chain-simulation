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
pub const MLC_ZERO_V_PARAM_IDS: [&str; 42] = [
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
    "mlc_clip_type1",
    "mlc_clip_type2",
    "mlc_clip_type3",
    "mlc_tight",
    "mlc_asymmetry_enable",
    "mlc_asymmetry",
    "mlc_preshape",
    "mlc_preshape_tight",
    "mlc_preshape_bite",
    "mlc_clean_blend",
    "mlc_sag",
    "mlc_h2",
    "mlc_h3",
    "mlc_h4",
    "mlc_ts_model",
    "mlc_tube_model",
    "mlc_tube_drive",
    "mlc_tube_bypass",
    "mlc_nfb_presence",
    "mlc_nfb_resonance",
    "mlc_nfb_depth",
    "mlc_nfb_bypass",
    "mlc_mbc_bypass",
    "mlc_mbc_cf_lo",
    "mlc_mbc_cf_hi",
    "mlc_mbc_drive_lo",
    "mlc_mbc_drive_mid",
    "mlc_mbc_drive_hi",
    "mlc_adaa_order",
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
    clip_type1: f32,
    clip_type2: f32,
    clip_type3: f32,
    tight: f32,
    asymmetry_enable: f32,
    asymmetry: f32,
    preshape: f32,
    preshape_tight: f32,
    preshape_bite: f32,
    clean_blend: f32,
    sag: f32,
    h2: f32,
    h3: f32,
    h4: f32,
    // Tier 2.2 / 3.x additions.
    ts_model: f32,
    tube_model: f32,
    tube_drive: f32,
    tube_bypass: f32,
    nfb_presence: f32,
    nfb_resonance: f32,
    nfb_depth: f32,
    nfb_bypass: f32,
    mbc_bypass: f32,
    mbc_cf_lo: f32,
    mbc_cf_hi: f32,
    mbc_drive_lo: f32,
    mbc_drive_mid: f32,
    mbc_drive_hi: f32,
    adaa_order: f32,
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
                clip_type1: 0.0,
                clip_type2: 0.0,
                clip_type3: 1.0,
                tight: 1.0,
                asymmetry_enable: 1.0,
                asymmetry: 0.5,
                preshape: 0.0,
                preshape_tight: -3.0,
                preshape_bite: 3.0,
                clean_blend: 0.0,
                sag: 0.0,
                h2: 0.0,
                h3: 0.7,
                h4: 0.2,
                // Neutral / bypassed defaults so the stock voicing is unchanged
                // until the user opts in (bypass flags default to 1.0 = bypassed).
                ts_model: 0.0,
                tube_model: 0.0,
                tube_drive: 0.0,
                tube_bypass: 1.0,
                nfb_presence: 0.0,
                nfb_resonance: 0.0,
                nfb_depth: 0.7,
                nfb_bypass: 1.0,
                mbc_bypass: 1.0,
                mbc_cf_lo: 300.0,
                mbc_cf_hi: 3000.0,
                mbc_drive_lo: 1.0,
                mbc_drive_mid: 1.0,
                mbc_drive_hi: 1.0,
                adaa_order: 0.0,
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
    pub fn set_clip_type1(&mut self, value: f32) {
        self.clip_type1 = value.clamp(0.0, 2.0).round();
    }
    #[inline(always)]
    pub fn set_clip_type2(&mut self, value: f32) {
        self.clip_type2 = value.clamp(0.0, 2.0).round();
    }
    #[inline(always)]
    pub fn set_clip_type3(&mut self, value: f32) {
        self.clip_type3 = value.clamp(0.0, 2.0).round();
    }
    #[inline(always)]
    pub fn set_tight(&mut self, value: f32) {
        self.tight = value.clamp(0.0, 1.0).round();
    }
    #[inline(always)]
    pub fn set_asymmetry_enable(&mut self, value: f32) {
        self.asymmetry_enable = value.clamp(0.0, 1.0).round();
    }
    #[inline(always)]
    pub fn set_asymmetry(&mut self, value: f32) {
        self.asymmetry = value.clamp(0.0, 1.0);
    }
    #[inline(always)]
    pub fn set_preshape(&mut self, value: f32) {
        self.preshape = value.clamp(0.0, 1.0).round();
    }
    #[inline(always)]
    pub fn set_preshape_tight(&mut self, value: f32) {
        self.preshape_tight = value.clamp(-6.0, 0.0);
    }
    #[inline(always)]
    pub fn set_preshape_bite(&mut self, value: f32) {
        self.preshape_bite = value.clamp(0.0, 6.0);
    }
    #[inline(always)]
    pub fn set_clean_blend(&mut self, value: f32) {
        self.clean_blend = value.clamp(0.0, 0.25);
    }
    #[inline(always)]
    pub fn set_sag(&mut self, value: f32) {
        self.sag = value.clamp(0.0, 1.0);
    }
    #[inline(always)]
    pub fn set_h2(&mut self, value: f32) {
        self.h2 = value.clamp(0.0, 1.0);
    }
    #[inline(always)]
    pub fn set_h3(&mut self, value: f32) {
        self.h3 = value.clamp(0.0, 1.0);
    }
    #[inline(always)]
    pub fn set_h4(&mut self, value: f32) {
        self.h4 = value.clamp(0.0, 1.0);
    }

    // --- Tier 2.2 / 3.x setters ---
    #[inline(always)]
    pub fn set_ts_model(&mut self, value: f32) {
        self.ts_model = value.clamp(0.0, 24.0).round();
    }
    #[inline(always)]
    pub fn set_tube_model(&mut self, value: f32) {
        self.tube_model = value.clamp(0.0, 17.0).round();
    }
    #[inline(always)]
    pub fn set_tube_drive(&mut self, value: f32) {
        // Value is in dB; the Faust hslider converts to linear internally.
        self.tube_drive = value.clamp(-20.0, 20.0);
    }
    #[inline(always)]
    pub fn set_tube_bypass(&mut self, value: bool) {
        self.tube_bypass = if value { 1.0 } else { 0.0 };
    }
    #[inline(always)]
    pub fn set_nfb_presence(&mut self, value: f32) {
        self.nfb_presence = value.clamp(0.0, 1.0);
    }
    #[inline(always)]
    pub fn set_nfb_resonance(&mut self, value: f32) {
        self.nfb_resonance = value.clamp(0.0, 1.0);
    }
    #[inline(always)]
    pub fn set_nfb_depth(&mut self, value: f32) {
        self.nfb_depth = value.clamp(0.0, 1.0);
    }
    #[inline(always)]
    pub fn set_nfb_bypass(&mut self, value: bool) {
        self.nfb_bypass = if value { 1.0 } else { 0.0 };
    }
    #[inline(always)]
    pub fn set_mbc_bypass(&mut self, value: bool) {
        self.mbc_bypass = if value { 1.0 } else { 0.0 };
    }
    #[inline(always)]
    pub fn set_mbc_cf_lo(&mut self, value: f32) {
        self.mbc_cf_lo = value.clamp(100.0, 800.0);
    }
    #[inline(always)]
    pub fn set_mbc_cf_hi(&mut self, value: f32) {
        self.mbc_cf_hi = value.clamp(1500.0, 6000.0);
    }
    #[inline(always)]
    pub fn set_mbc_drive_lo(&mut self, value: f32) {
        self.mbc_drive_lo = value.clamp(0.1, 4.0);
    }
    #[inline(always)]
    pub fn set_mbc_drive_mid(&mut self, value: f32) {
        self.mbc_drive_mid = value.clamp(0.1, 4.0);
    }
    #[inline(always)]
    pub fn set_mbc_drive_hi(&mut self, value: f32) {
        self.mbc_drive_hi = value.clamp(0.1, 4.0);
    }
    #[inline(always)]
    pub fn set_adaa_order(&mut self, value: f32) {
        self.adaa_order = value.clamp(0.0, 2.0).round();
    }

    /// Push all cached parameter values into the Faust DSP instance.
    #[inline]
    fn push_params(&mut self) {
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
            mlc_zero_v_set_clip_type1(self.handle, self.clip_type1);
            mlc_zero_v_set_clip_type2(self.handle, self.clip_type2);
            mlc_zero_v_set_clip_type3(self.handle, self.clip_type3);
            mlc_zero_v_set_tight(self.handle, self.tight);
            mlc_zero_v_set_asymmetry_enable(self.handle, self.asymmetry_enable);
            mlc_zero_v_set_asymmetry(self.handle, self.asymmetry);
            mlc_zero_v_set_preshape(self.handle, self.preshape);
            mlc_zero_v_set_preshape_tight(self.handle, self.preshape_tight);
            mlc_zero_v_set_preshape_bite(self.handle, self.preshape_bite);
            mlc_zero_v_set_clean_blend(self.handle, self.clean_blend);
            mlc_zero_v_set_sag(self.handle, self.sag);
            mlc_zero_v_set_h2(self.handle, self.h2);
            mlc_zero_v_set_h3(self.handle, self.h3);
            mlc_zero_v_set_h4(self.handle, self.h4);
            mlc_zero_v_set_ts_model(self.handle, self.ts_model);
            mlc_zero_v_set_tube_model(self.handle, self.tube_model);
            mlc_zero_v_set_tube_drive(self.handle, self.tube_drive);
            mlc_zero_v_set_tube_bypass(self.handle, self.tube_bypass);
            mlc_zero_v_set_nfb_presence(self.handle, self.nfb_presence);
            mlc_zero_v_set_nfb_resonance(self.handle, self.nfb_resonance);
            mlc_zero_v_set_nfb_depth(self.handle, self.nfb_depth);
            mlc_zero_v_set_nfb_bypass(self.handle, self.nfb_bypass);
            mlc_zero_v_set_mbc_bypass(self.handle, self.mbc_bypass);
            mlc_zero_v_set_mbc_cf_lo(self.handle, self.mbc_cf_lo);
            mlc_zero_v_set_mbc_cf_hi(self.handle, self.mbc_cf_hi);
            mlc_zero_v_set_mbc_drive_lo(self.handle, self.mbc_drive_lo);
            mlc_zero_v_set_mbc_drive_mid(self.handle, self.mbc_drive_mid);
            mlc_zero_v_set_mbc_drive_hi(self.handle, self.mbc_drive_hi);
            mlc_zero_v_set_adaa_order(self.handle, self.adaa_order);
        }
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
        if !self.is_ready || buffer.is_null() || length == 0 {
            return;
        }
        self.push_params();
        unsafe {
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
            "mlc_clip_type1" => Some(self.clip_type1),
            "mlc_clip_type2" => Some(self.clip_type2),
            "mlc_clip_type3" => Some(self.clip_type3),
            "mlc_tight" => Some(self.tight),
            "mlc_asymmetry_enable" => Some(self.asymmetry_enable),
            "mlc_asymmetry" => Some(self.asymmetry),
            "mlc_preshape" => Some(self.preshape),
            "mlc_preshape_tight" => Some(self.preshape_tight),
            "mlc_preshape_bite" => Some(self.preshape_bite),
            "mlc_clean_blend" => Some(self.clean_blend),
            "mlc_sag" => Some(self.sag),
            "mlc_h2" => Some(self.h2),
            "mlc_h3" => Some(self.h3),
            "mlc_h4" => Some(self.h4),
            "mlc_ts_model" => Some(self.ts_model),
            "mlc_tube_model" => Some(self.tube_model),
            "mlc_tube_drive" => Some(self.tube_drive),
            "mlc_tube_bypass" => Some(self.tube_bypass),
            "mlc_nfb_presence" => Some(self.nfb_presence),
            "mlc_nfb_resonance" => Some(self.nfb_resonance),
            "mlc_nfb_depth" => Some(self.nfb_depth),
            "mlc_nfb_bypass" => Some(self.nfb_bypass),
            "mlc_mbc_bypass" => Some(self.mbc_bypass),
            "mlc_mbc_cf_lo" => Some(self.mbc_cf_lo),
            "mlc_mbc_cf_hi" => Some(self.mbc_cf_hi),
            "mlc_mbc_drive_lo" => Some(self.mbc_drive_lo),
            "mlc_mbc_drive_mid" => Some(self.mbc_drive_mid),
            "mlc_mbc_drive_hi" => Some(self.mbc_drive_hi),
            "mlc_adaa_order" => Some(self.adaa_order),
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
            "mlc_clip_type1" => self.set_clip_type1(value),
            "mlc_clip_type2" => self.set_clip_type2(value),
            "mlc_clip_type3" => self.set_clip_type3(value),
            "mlc_tight" => self.set_tight(value),
            "mlc_asymmetry_enable" => self.set_asymmetry_enable(value),
            "mlc_asymmetry" => self.set_asymmetry(value),
            "mlc_preshape" => self.set_preshape(value),
            "mlc_preshape_tight" => self.set_preshape_tight(value),
            "mlc_preshape_bite" => self.set_preshape_bite(value),
            "mlc_clean_blend" => self.set_clean_blend(value),
            "mlc_sag" => self.set_sag(value),
            "mlc_h2" => self.set_h2(value),
            "mlc_h3" => self.set_h3(value),
            "mlc_h4" => self.set_h4(value),
            "mlc_ts_model" => self.set_ts_model(value),
            "mlc_tube_model" => self.set_tube_model(value),
            "mlc_tube_drive" => self.set_tube_drive(value),
            "mlc_tube_bypass" => self.set_tube_bypass(value >= 0.5),
            "mlc_nfb_presence" => self.set_nfb_presence(value),
            "mlc_nfb_resonance" => self.set_nfb_resonance(value),
            "mlc_nfb_depth" => self.set_nfb_depth(value),
            "mlc_nfb_bypass" => self.set_nfb_bypass(value >= 0.5),
            "mlc_mbc_bypass" => self.set_mbc_bypass(value >= 0.5),
            "mlc_mbc_cf_lo" => self.set_mbc_cf_lo(value),
            "mlc_mbc_cf_hi" => self.set_mbc_cf_hi(value),
            "mlc_mbc_drive_lo" => self.set_mbc_drive_lo(value),
            "mlc_mbc_drive_mid" => self.set_mbc_drive_mid(value),
            "mlc_mbc_drive_hi" => self.set_mbc_drive_hi(value),
            "mlc_adaa_order" => self.set_adaa_order(value),
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
        ("mlc_clip_type1", "MLC Clip Type 1", (0.0, 2.0), 0.0, None),
        ("mlc_clip_type2", "MLC Clip Type 2", (0.0, 2.0), 0.0, None),
        ("mlc_clip_type3", "MLC Clip Type 3", (0.0, 2.0), 1.0, None),
        ("mlc_tight", "MLC Tight", (0.0, 1.0), 1.0, None),
        (
            "mlc_asymmetry_enable",
            "MLC Asymmetry Enable",
            (0.0, 1.0),
            1.0,
            None,
        ),
        ("mlc_asymmetry", "MLC Asymmetry", (0.0, 1.0), 0.5, None),
        ("mlc_preshape", "MLC Pre-Shape", (0.0, 1.0), 0.0, None),
        (
            "mlc_preshape_tight",
            "MLC Pre-Shape Tight",
            (-6.0, 0.0),
            -3.0,
            Some("dB"),
        ),
        (
            "mlc_preshape_bite",
            "MLC Pre-Shape Bite",
            (0.0, 6.0),
            3.0,
            Some("dB"),
        ),
        (
            "mlc_clean_blend",
            "MLC Clean Blend",
            (0.0, 0.25),
            0.0,
            None,
        ),
        ("mlc_sag", "MLC Sag", (0.0, 1.0), 0.0, None),
        ("mlc_h2", "MLC Chebyshev H2", (0.0, 1.0), 0.0, None),
        ("mlc_h3", "MLC Chebyshev H3", (0.0, 1.0), 0.7, None),
        ("mlc_h4", "MLC Chebyshev H4", (0.0, 1.0), 0.2, None),
        ("mlc_ts_model", "MLC Tone Stack Model", (0.0, 24.0), 0.0, None),
        ("mlc_tube_model", "MLC Tube Model", (0.0, 17.0), 0.0, None),
        (
            "mlc_tube_drive",
            "MLC Tube Drive",
            (-20.0, 20.0),
            0.0,
            Some("dB"),
        ),
        ("mlc_tube_bypass", "MLC Tube Bypass", (0.0, 1.0), 1.0, None),
        (
            "mlc_nfb_presence",
            "MLC NFB Presence",
            (0.0, 1.0),
            0.0,
            None,
        ),
        (
            "mlc_nfb_resonance",
            "MLC NFB Resonance",
            (0.0, 1.0),
            0.0,
            None,
        ),
        ("mlc_nfb_depth", "MLC NFB Depth", (0.0, 1.0), 0.7, None),
        ("mlc_nfb_bypass", "MLC NFB Bypass", (0.0, 1.0), 1.0, None),
        (
            "mlc_mbc_bypass",
            "MLC Multi-Band Bypass",
            (0.0, 1.0),
            1.0,
            None,
        ),
        (
            "mlc_mbc_cf_lo",
            "MLC XOver Low",
            (100.0, 800.0),
            300.0,
            Some("Hz"),
        ),
        (
            "mlc_mbc_cf_hi",
            "MLC XOver High",
            (1500.0, 6000.0),
            3000.0,
            Some("Hz"),
        ),
        ("mlc_mbc_drive_lo", "MLC Drive Lo", (0.1, 4.0), 1.0, None),
        ("mlc_mbc_drive_mid", "MLC Drive Mid", (0.1, 4.0), 1.0, None),
        ("mlc_mbc_drive_hi", "MLC Drive Hi", (0.1, 4.0), 1.0, None),
        ("mlc_adaa_order", "MLC ADAA Order", (0.0, 2.0), 0.0, None),
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

        // Per-stage clip type is quantized to the nearest valid curve index (0-2),
        // where 2 is the new Chebyshev curve.
        assert!(processor.set_param("mlc_clip_type3", 2.0));
        assert_eq!(processor.get_param("mlc_clip_type3"), Some(2.0));
        assert!(processor.set_param("mlc_clip_type1", 42.0));
        assert_eq!(processor.get_param("mlc_clip_type1"), Some(2.0));

        // Tier 1 gain-staging params.
        assert!(processor.set_param("mlc_asymmetry", 0.75));
        assert_eq!(processor.get_param("mlc_asymmetry"), Some(0.75));

        // Toggles quantize to 0/1.
        assert!(processor.set_param("mlc_tight", 0.0));
        assert_eq!(processor.get_param("mlc_tight"), Some(0.0));
        assert!(processor.set_param("mlc_preshape", 1.0));
        assert_eq!(processor.get_param("mlc_preshape"), Some(1.0));

        // Pre-Shape gains clamp to their configured ranges.
        assert!(processor.set_param("mlc_preshape_tight", -10.0));
        assert_eq!(processor.get_param("mlc_preshape_tight"), Some(-6.0));
        assert!(processor.set_param("mlc_preshape_bite", 42.0));
        assert_eq!(processor.get_param("mlc_preshape_bite"), Some(6.0));

        // Clean blend clamps to [0.0, 0.25].
        assert!(processor.set_param("mlc_clean_blend", 1.0));
        assert_eq!(processor.get_param("mlc_clean_blend"), Some(0.25));

        // Chebyshev harmonic amounts clamp to [0.0, 1.0] with the documented defaults.
        assert_eq!(processor.get_param("mlc_h3"), Some(0.7));
        assert!(processor.set_param("mlc_sag", 2.0));
        assert_eq!(processor.get_param("mlc_sag"), Some(1.0));

        assert_eq!(processor.get_param("missing"), None);
    }

    /// With every stage set to Chebyshev and all harmonic amounts maxed, the raw
    /// polynomial is unbounded; the tanh soft-clamp in the DSP must keep every
    /// output sample finite even for a hot, full-scale input block.
    #[test]
    fn test_mlc_chebyshev_output_is_finite_at_high_gain() {
        let mut processor = MlcZeroVProcessor::new().expect("mlc processor");
        processor.init(44_100.0);

        // Max gain, all three stages on the Chebyshev curve (index 2).
        processor.set_gain(1.0);
        assert!(processor.set_param("mlc_clip_type1", 2.0));
        assert!(processor.set_param("mlc_clip_type2", 2.0));
        assert!(processor.set_param("mlc_clip_type3", 2.0));
        // Push every Chebyshev harmonic amount to its maximum.
        assert!(processor.set_param("mlc_h2", 1.0));
        assert!(processor.set_param("mlc_h3", 1.0));
        assert!(processor.set_param("mlc_h4", 1.0));

        // A hot, near-full-scale block. Run several blocks so smoothed params
        // reach their targets before we assert on the output.
        let mut block = vec![0.9f32; 1024];
        for _ in 0..8 {
            block.iter_mut().for_each(|s| *s = 0.9);
            processor.process_block(block.as_mut_ptr(), block.len());
        }

        assert!(
            block.iter().all(|s| s.is_finite()),
            "Chebyshev output must stay finite (no Inf/NaN) to avoid poisoning the limiter"
        );
    }
}
