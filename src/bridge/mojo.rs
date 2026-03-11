use super::ExternalProcessor;

// Define as interfaces C que a biblioteca do Mojo deverá expor (.so / .dylib)
extern "C" {
    fn mojo_init(sample_rate: f32);
    /// drive: ganho de entrada (pre-gain)
    /// output_gain: ganho de saída (post-saturação)
    fn mojo_process_block(address: usize, size: usize, drive: f32, output_gain: f32);
}

pub struct MojoProcessor {
    is_ready: bool,
    drive: f32,
    output_gain: f32,
}

impl MojoProcessor {
    pub fn new() -> Self {
        Self {
            is_ready: false,
            drive: 1.0,
            output_gain: 1.0,
        }
    }

    /// Define o drive (pre-gain / distorção) que será passado ao Mojo.
    /// Real-Time Safe: apenas atualiza campo local.
    #[inline(always)]
    pub fn set_drive(&mut self, drive: f32) {
        self.drive = drive;
    }

    /// Define o ganho de saída que será passado ao Mojo.
    /// Real-Time Safe: apenas atualiza campo local.
    #[inline(always)]
    pub fn set_output_gain(&mut self, gain: f32) {
        self.output_gain = gain;
    }

    pub fn is_ready(&self) -> bool {
        self.is_ready
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
            // ZERO-COPY: o ponteiro é convertido para usize.
            // drive e output_gain são passados por valor — sem alocação.
            let ptr = buffer as usize;
            mojo_process_block(ptr, length, self.drive, self.output_gain);
        }
    }
}
