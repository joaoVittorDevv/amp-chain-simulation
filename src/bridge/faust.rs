#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

// Importa dinamicamente os bindings gerados pelo build.rs via bindgen
include!(concat!(env!("OUT_DIR"), "/bindings_faust.rs"));

use super::ExternalProcessor;
#[cfg(feature = "lab")]
use crate::lab::{DspVariant, ParameterMeta};

#[cfg(feature = "lab")]
pub const FAUST_EQ_IMPL_ID: &str = "faust-eq";
#[cfg(feature = "lab")]
pub const FAUST_EQ_PARAM_IDS: [&str; 9] = [
    "eq_low_freq",
    "eq_low_gain",
    "eq_low_q",
    "eq_mid_freq",
    "eq_mid_gain",
    "eq_mid_q",
    "eq_high_freq",
    "eq_high_gain",
    "eq_high_q",
];

pub struct FaustProcessor {
    handle: FaustHandle,
    cached_low_freq: f32,
    cached_low_gain: f32,
    cached_low_q: f32,
    cached_mid_freq: f32,
    cached_mid_gain: f32,
    cached_mid_q: f32,
    cached_high_freq: f32,
    cached_high_gain: f32,
    cached_high_q: f32,
    cached_tanh_bypass: bool,
}

impl FaustProcessor {
    pub fn new() -> Option<Self> {
        let handle = unsafe { faust_create() };
        if handle.is_null() {
            None
        } else {
            Some(Self {
                handle,
                cached_low_freq: 100.0,
                cached_low_gain: 0.0,
                cached_low_q: 0.707,
                cached_mid_freq: 1000.0,
                cached_mid_gain: 0.0,
                cached_mid_q: 0.707,
                cached_high_freq: 5000.0,
                cached_high_gain: 0.0,
                cached_high_q: 0.707,
                cached_tanh_bypass: false,
            })
        }
    }

    pub fn set_eq_params(
        &mut self,
        low_f: f32,
        low_g: f32,
        low_q: f32,
        mid_f: f32,
        mid_g: f32,
        mid_q: f32,
        high_f: f32,
        high_g: f32,
        high_q: f32,
    ) {
        self.cached_low_freq = low_f;
        self.cached_low_gain = low_g;
        self.cached_low_q = low_q;
        self.cached_mid_freq = mid_f;
        self.cached_mid_gain = mid_g;
        self.cached_mid_q = mid_q;
        self.cached_high_freq = high_f;
        self.cached_high_gain = high_g;
        self.cached_high_q = high_q;

        unsafe {
            faust_set_eq_low_freq(self.handle, low_f);
            faust_set_eq_low_gain(self.handle, low_g);
            faust_set_eq_low_q(self.handle, low_q);

            faust_set_eq_mid_freq(self.handle, mid_f);
            faust_set_eq_mid_gain(self.handle, mid_g);
            faust_set_eq_mid_q(self.handle, mid_q);

            faust_set_eq_high_freq(self.handle, high_f);
            faust_set_eq_high_gain(self.handle, high_g);
            faust_set_eq_high_q(self.handle, high_q);
        }
    }

    pub fn set_eq_tanh_bypass(&mut self, bypass: bool) {
        self.cached_tanh_bypass = bypass;
        unsafe {
            faust_set_eq_tanh_bypass(self.handle, if bypass { 1.0 } else { 0.0 });
        }
    }
}

impl Drop for FaustProcessor {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe {
                faust_destroy(self.handle);
            }
        }
    }
}

// O FaustProcessor ganha Send devido ao ponteiro opaco poder trafegar entre as threads do VST/Clap
// (O Faust lida com concorrências seguras em seu process() se single-thread por loop)
unsafe impl Send for FaustProcessor {}

impl ExternalProcessor for FaustProcessor {
    fn init(&mut self, sample_rate: f32) {
        unsafe {
            faust_init(self.handle, sample_rate);
        }
    }

    fn process_block(&mut self, buffer: *mut f32, length: usize) {
        // Zero-copy: chamamos a função FFI passando o ponteiro bruto que veio do nih-plug!
        unsafe {
            faust_process(self.handle, buffer, length as _);
        }
    }

    fn get_param(&self, id: &str) -> Option<f32> {
        match id {
            "eq_low_freq" => Some(self.cached_low_freq),
            "eq_low_gain" => Some(self.cached_low_gain),
            "eq_low_q" => Some(self.cached_low_q),
            "eq_mid_freq" => Some(self.cached_mid_freq),
            "eq_mid_gain" => Some(self.cached_mid_gain),
            "eq_mid_q" => Some(self.cached_mid_q),
            "eq_high_freq" => Some(self.cached_high_freq),
            "eq_high_gain" => Some(self.cached_high_gain),
            "eq_high_q" => Some(self.cached_high_q),
            "eq_tanh_bypass" => Some(if self.cached_tanh_bypass { 1.0 } else { 0.0 }),
            _ => None,
        }
    }

    fn set_param(&mut self, id: &str, value: f32) -> bool {
        match id {
            "eq_low_freq" => {
                self.cached_low_freq = value;
                unsafe { faust_set_eq_low_freq(self.handle, value) };
                true
            }
            "eq_low_gain" => {
                self.cached_low_gain = value;
                unsafe { faust_set_eq_low_gain(self.handle, value) };
                true
            }
            "eq_low_q" => {
                self.cached_low_q = value;
                unsafe { faust_set_eq_low_q(self.handle, value) };
                true
            }
            "eq_mid_freq" => {
                self.cached_mid_freq = value;
                unsafe { faust_set_eq_mid_freq(self.handle, value) };
                true
            }
            "eq_mid_gain" => {
                self.cached_mid_gain = value;
                unsafe { faust_set_eq_mid_gain(self.handle, value) };
                true
            }
            "eq_mid_q" => {
                self.cached_mid_q = value;
                unsafe { faust_set_eq_mid_q(self.handle, value) };
                true
            }
            "eq_high_freq" => {
                self.cached_high_freq = value;
                unsafe { faust_set_eq_high_freq(self.handle, value) };
                true
            }
            "eq_high_gain" => {
                self.cached_high_gain = value;
                unsafe { faust_set_eq_high_gain(self.handle, value) };
                true
            }
            "eq_high_q" => {
                self.cached_high_q = value;
                unsafe { faust_set_eq_high_q(self.handle, value) };
                true
            }
            "eq_tanh_bypass" => {
                self.set_eq_tanh_bypass(value >= 0.5);
                true
            }
            _ => false,
        }
    }

    fn param_metadata(&self) -> Vec<ParameterMeta> {
        faust_eq_param_metadata()
    }
}

#[cfg(feature = "lab")]
impl DspVariant for FaustProcessor {
    fn process_block(&mut self, buffer: *mut f32, length: usize) {
        ExternalProcessor::process_block(self, buffer, length);
    }

    fn param_count(&self) -> usize {
        FAUST_EQ_PARAM_IDS.len()
    }

    fn param_ids(&self) -> &[&str] {
        &FAUST_EQ_PARAM_IDS
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
struct FaustBypassVariant;

#[cfg(feature = "lab")]
impl DspVariant for FaustBypassVariant {
    fn process_block(&mut self, _buffer: *mut f32, _length: usize) {}

    fn param_count(&self) -> usize {
        FAUST_EQ_PARAM_IDS.len()
    }

    fn param_ids(&self) -> &[&str] {
        &FAUST_EQ_PARAM_IDS
    }

    fn latency(&self) -> usize {
        0
    }

    fn get_param(&self, id: &str) -> Option<f32> {
        faust_eq_param_metadata()
            .into_iter()
            .find(|meta| meta.id == id)
            .map(|meta| meta.default)
    }

    fn param_metadata(&self) -> Vec<ParameterMeta> {
        faust_eq_param_metadata()
    }
}

#[cfg(feature = "lab")]
pub fn faust_eq_factory(sample_rate: f32) -> Box<dyn DspVariant> {
    match FaustProcessor::new() {
        Some(mut processor) => {
            processor.init(sample_rate);
            Box::new(processor)
        }
        None => Box::new(FaustBypassVariant),
    }
}

#[cfg(feature = "lab")]
fn faust_eq_param_metadata() -> Vec<ParameterMeta> {
    [
        ("eq_low_freq", "Low Freq", (20.0, 1000.0), 100.0, Some("Hz")),
        ("eq_low_gain", "Low Gain", (-12.0, 12.0), 0.0, Some("dB")),
        ("eq_low_q", "Low Q", (0.707, 10.0), 0.707, None),
        ("eq_mid_freq", "Mid Freq", (100.0, 10000.0), 1000.0, Some("Hz")),
        ("eq_mid_gain", "Mid Gain", (-12.0, 12.0), 0.0, Some("dB")),
        ("eq_mid_q", "Mid Q", (0.707, 10.0), 0.707, None),
        (
            "eq_high_freq",
            "High Freq",
            (1000.0, 20000.0),
            5000.0,
            Some("Hz"),
        ),
        ("eq_high_gain", "High Gain", (-12.0, 12.0), 0.0, Some("dB")),
        ("eq_high_q", "High Q", (0.707, 10.0), 0.707, None),
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
    use super::FaustProcessor;
    use crate::bridge::ExternalProcessor;

    #[test]
    fn test_faust_eq_get_param_returns_cached_value() {
        let mut processor = FaustProcessor::new().expect("faust processor");

        processor.set_eq_params(120.0, 1.5, 0.9, 900.0, -2.0, 1.2, 7000.0, 3.0, 2.0);
        assert_eq!(processor.get_param("eq_low_freq"), Some(120.0));
        assert_eq!(processor.get_param("eq_mid_gain"), Some(-2.0));
        assert_eq!(processor.get_param("eq_high_q"), Some(2.0));

        assert!(processor.set_param("eq_low_gain", -6.0));
        assert_eq!(processor.get_param("eq_low_gain"), Some(-6.0));
        assert_eq!(processor.get_param("missing"), None);
    }
}
