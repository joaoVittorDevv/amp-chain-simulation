use super::ExternalProcessor;

// Define as interfaces C que a biblioteca do Mojo deverá expor (.so / .dylib)
extern "C" {
    fn mojo_init(sample_rate: f32);
    fn mojo_process_block(address: usize, size: usize);
}

pub struct MojoProcessor {
    is_ready: bool,
}

impl MojoProcessor {
    pub fn new() -> Self {
        Self { is_ready: false }
    }
}

impl ExternalProcessor for MojoProcessor {
    fn init(&mut self, sample_rate: f32) {
        unsafe {
            mojo_init(sample_rate);
        }
        self.is_ready = true;
    }

    fn process_block(&mut self, buffer: *mut f32, length: usize) {
        if !self.is_ready {
            return;
        }
        
        unsafe {
            let ptr = buffer as usize;
            mojo_process_block(ptr, length);
        }
    }
}
