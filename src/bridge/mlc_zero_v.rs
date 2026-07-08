#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/bindings_mlc_zero_v.rs"));

use super::oversampling::Lanczos3Oversampler;
use super::ExternalProcessor;
#[cfg(feature = "lab")]
use crate::lab::{DspVariant, ParameterMeta};

#[cfg(feature = "lab")]
pub const MLC_ZERO_V_IMPL_ID: &str = "mlc-zero-v";
#[cfg(feature = "lab")]
pub const MLC_ZERO_V_PARAM_IDS: [&str; 27] = [
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
];

/// Maximum block size (at the base sample rate) the internal oversampler can
/// handle. Larger blocks fall back to 1x processing to avoid reallocation on the
/// audio thread.
const OVS_MAX_BLOCK: usize = 8192;
/// Maximum oversampling factor (2-log): 2 stages of 2x = up to 4x.
const OVS_MAX_FACTOR: usize = 2;

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

    // --- Oversampling (Rust-side, wraps the Faust DSP) ---
    /// Requested oversampling factor as a 2-log: 0 = 1x, 1 = 2x, 2 = 4x.
    ovs_factor: usize,
    /// The oversampling factor (2-log) the last `process_block` actually used.
    /// This can differ from `ovs_factor` when a block larger than
    /// `OVS_MAX_BLOCK` forces a fall back to 1x, so latency reporting to the
    /// host reflects the audio path rather than the requested factor.
    effective_ovs_factor: usize,
    oversampler: Lanczos3Oversampler,
    /// Base engine sample rate, cached from `init`.
    base_sample_rate: f32,
    /// The oversampling multiplier (1/2/4) the Faust DSP is currently initialized
    /// for, so we only re-init its coefficients when the factor actually changes.
    active_mult: usize,
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
                ovs_factor: 0,
                effective_ovs_factor: 0,
                oversampler: Lanczos3Oversampler::new(OVS_MAX_BLOCK, OVS_MAX_FACTOR),
                base_sample_rate: 44100.0,
                active_mult: 1,
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

    /// Set the oversampling factor as a 2-log: 0 = 1x, 1 = 2x, 2 = 4x. Values are
    /// clamped to the oversampler's maximum factor.
    #[inline(always)]
    pub fn set_ovs_factor(&mut self, value: i32) {
        self.ovs_factor = (value.max(0) as usize).min(OVS_MAX_FACTOR);
    }

    /// Plugin delay compensation for the oversampling factor the audio path is
    /// actually running, in samples at the base sample rate. This tracks the
    /// *effective* factor (updated by `process_block`) rather than the requested
    /// one, so PDC never over-reports when a large block forces a 1x fall back.
    #[inline(always)]
    pub fn latency_samples(&self) -> u32 {
        self.oversampler.latency(self.effective_ovs_factor)
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
        self.base_sample_rate = sample_rate;
        self.active_mult = 1;
        self.oversampler.reset();
        unsafe {
            mlc_zero_v_init(self.handle, sample_rate);
        }
        self.is_ready = true;
    }

    fn process_block(&mut self, buffer: *mut f32, length: usize) {
        if !self.is_ready || buffer.is_null() || length == 0 {
            return;
        }

        // Determine the oversampling factor we can actually honor for this block.
        // Blocks larger than the oversampler's scratch space fall back to 1x so we
        // never allocate on the audio thread.
        let mut factor = self.ovs_factor.min(self.oversampler.max_factor());
        if factor > 0 && length > self.oversampler.max_block_size() {
            factor = 0;
        }
        // Record the factor the audio path is really using so `latency_samples`
        // reports the truth to the host, even on the 1x large-block fall back.
        self.effective_ovs_factor = factor;
        let mult = 1usize << factor;

        // The Faust DSP runs at the oversampled rate, so its filter coefficients
        // must be recomputed whenever the effective sample rate changes.
        if mult != self.active_mult {
            unsafe {
                mlc_zero_v_init(self.handle, self.base_sample_rate * mult as f32);
            }
            self.active_mult = mult;
        }

        self.push_params();

        let handle = self.handle;
        if factor == 0 {
            unsafe {
                mlc_zero_v_process(handle, buffer, length as _);
            }
        } else {
            let block = unsafe { std::slice::from_raw_parts_mut(buffer, length) };
            self.oversampler.process(block, factor, |upsampled| unsafe {
                mlc_zero_v_process(handle, upsampled.as_mut_ptr(), upsampled.len() as _);
            });
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
        self.latency_samples() as usize
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

    /// The bridge-level oversampling wrapper must report the *effective* latency
    /// — the factor the audio path actually ran — which is only known after a
    /// block has been processed.
    #[test]
    fn test_mlc_oversampling_latency_tracks_effective_factor() {
        let mut processor = MlcZeroVProcessor::new().expect("mlc processor");
        processor.init(44_100.0);

        // Before any block is processed the effective factor is 1x.
        assert_eq!(processor.latency_samples(), 0);

        // A normal-sized block honours the requested 4x factor → 8 samples PDC.
        processor.set_ovs_factor(2); // 4x
        let mut block = vec![0.1f32; 512];
        processor.process_block(block.as_mut_ptr(), block.len());
        assert_eq!(processor.latency_samples(), 8);

        // 2x reports 5 samples once a block has run at that factor.
        processor.set_ovs_factor(1); // 2x
        processor.process_block(block.as_mut_ptr(), block.len());
        assert_eq!(processor.latency_samples(), 5);

        // A block larger than OVS_MAX_BLOCK forces a 1x fall back, so the
        // reported latency must drop to 0 to stay honest with the DAW's PDC.
        processor.set_ovs_factor(2); // request 4x again
        let mut big_block = vec![0.1f32; super::OVS_MAX_BLOCK + 808];
        processor.process_block(big_block.as_mut_ptr(), big_block.len());
        assert_eq!(processor.latency_samples(), 0);
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
