#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

// Importa dinamicamente os bindings gerados pelo build.rs via bindgen
include!(concat!(env!("OUT_DIR"), "/bindings_faust.rs"));

use super::ExternalProcessor;

pub struct FaustProcessor {
    handle: FaustHandle,
}

impl FaustProcessor {
    pub fn new() -> Option<Self> {
        let handle = unsafe { faust_create() };
        if handle.is_null() {
            None
        } else {
            Some(Self { handle })
        }
    }

    pub fn set_eq_params(&mut self, low_f: f32, low_g: f32, low_q: f32, mid_f: f32, mid_g: f32, mid_q: f32, high_f: f32, high_g: f32, high_q: f32) {
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
}
