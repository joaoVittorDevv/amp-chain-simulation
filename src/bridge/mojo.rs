use super::ExternalProcessor;
#[cfg(feature = "lab")]
use crate::lab::{DspVariant, ParameterMeta};

#[cfg(feature = "lab")]
pub const MOJO_NEURAL_PARAM_IDS: [&str; 2] = ["drive", "output_gain"];

// Define as interfaces C que a biblioteca do Mojo deverá expor (.so / .dylib)
extern "C" {
    /// sample_rate: `f32` para casar com `Float32` no lado Mojo (ABI de 4 bytes).
    /// Atualmente no-op no Mojo; mantido para consistência de API / uso futuro.
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

    fn get_param(&self, id: &str) -> Option<f32> {
        match id {
            "drive" | "neural_drive" => Some(self.drive),
            "output_gain" | "neural_output_gain" => Some(self.output_gain),
            _ => None,
        }
    }

    fn set_param(&mut self, id: &str, value: f32) -> bool {
        match id {
            "drive" | "neural_drive" => {
                self.set_drive(value);
                true
            }
            "output_gain" | "neural_output_gain" => {
                self.set_output_gain(value);
                true
            }
            _ => false,
        }
    }

    fn param_metadata(&self) -> Vec<ParameterMeta> {
        mojo_param_metadata()
    }
}

#[cfg(feature = "lab")]
impl DspVariant for MojoProcessor {
    fn process_block(&mut self, buffer: *mut f32, length: usize) {
        ExternalProcessor::process_block(self, buffer, length);
    }

    fn param_count(&self) -> usize {
        MOJO_NEURAL_PARAM_IDS.len()
    }

    fn param_ids(&self) -> &[&str] {
        &MOJO_NEURAL_PARAM_IDS
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
fn mojo_param_metadata() -> Vec<ParameterMeta> {
    [
        ("drive", "Drive", (0.0, 1.0), 1.0),
        ("output_gain", "Output Gain", (0.0, 2.0), 1.0),
    ]
    .into_iter()
    .enumerate()
    .map(|(index, (id, name, range, default))| ParameterMeta {
        id: id.to_string(),
        name: name.to_string(),
        description: name.to_string(),
        range,
        default,
        unit: None,
        smoothing: "50 ms".to_string(),
        index: index as u32,
        backend: Some("mojo"),
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::MojoProcessor;
    use crate::bridge::ExternalProcessor;

    #[test]
    fn test_mojo_get_param_returns_stored_value() {
        let mut processor = MojoProcessor::new();

        processor.set_drive(0.35);
        processor.set_output_gain(1.4);

        assert_eq!(processor.get_param("drive"), Some(0.35));
        assert_eq!(processor.get_param("output_gain"), Some(1.4));

        assert!(processor.set_param("drive", 0.75));
        assert_eq!(processor.get_param("drive"), Some(0.75));
        assert_eq!(processor.get_param("missing"), None);
    }
}
