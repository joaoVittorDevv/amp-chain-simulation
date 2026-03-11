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
