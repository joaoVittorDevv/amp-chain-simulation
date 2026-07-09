use super::ExternalProcessor;
#[cfg(feature = "lab")]
use crate::lab::{DspVariant, ParameterMeta};

#[cfg(feature = "lab")]
pub const RUST_NEURAL_PARAM_IDS: [&str; 2] = ["drive", "output_gain"];

/// Rust-native fallback for `MojoProcessor`. Bit-for-bit port of the tanh
/// polynomial approximation in `neural/main.mojo:30-42`, used when the Mojo
/// toolchain is unavailable (Windows, or any host without the SDK).
pub struct RustNeuralProcessor {
    sample_rate: f32,
    drive: f32,
    output_gain: f32,
    ready: bool,
}

impl RustNeuralProcessor {
    pub fn new() -> Self {
        Self {
            sample_rate: 48_000.0,
            drive: 1.0,
            output_gain: 1.0,
            ready: false,
        }
    }

    #[inline(always)]
    pub fn set_drive(&mut self, drive: f32) {
        self.drive = drive;
    }

    #[inline(always)]
    pub fn set_output_gain(&mut self, gain: f32) {
        self.output_gain = gain;
    }

    pub fn is_ready(&self) -> bool {
        self.ready
    }
}

impl Default for RustNeuralProcessor {
    fn default() -> Self {
        Self::new()
    }
}

/// Polynomial tanh approximation, mirroring `neural/main.mojo:32-41` exactly
/// so the Rust and Mojo backends stay within 1e-6 of each other (CROSS-04).
#[inline]
fn saturate(sample: f32, drive: f32, output_gain: f32) -> f32 {
    let x = (sample * drive).clamp(-4.0, 4.0);
    let x2 = x * x;
    let saturated = x * (27.0 + x2) / (27.0 + 9.0 * x2);
    saturated * output_gain
}

impl ExternalProcessor for RustNeuralProcessor {
    fn init(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.ready = true;
    }

    fn process_block(&mut self, buffer: *mut f32, length: usize) {
        if !self.ready {
            return;
        }

        // ZERO-COPY: processa in-place via ponteiro bruto, sem alocação.
        unsafe {
            let samples = std::slice::from_raw_parts_mut(buffer, length);
            for sample in samples {
                *sample = saturate(*sample, self.drive, self.output_gain);
            }
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
        rust_neural_param_metadata()
    }
}

#[cfg(feature = "lab")]
impl DspVariant for RustNeuralProcessor {
    fn process_block(&mut self, buffer: *mut f32, length: usize) {
        ExternalProcessor::process_block(self, buffer, length);
    }

    fn param_count(&self) -> usize {
        RUST_NEURAL_PARAM_IDS.len()
    }

    fn param_ids(&self) -> &[&str] {
        &RUST_NEURAL_PARAM_IDS
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
pub fn rust_neural_factory(sample_rate: f32) -> Box<dyn DspVariant> {
    let mut processor = RustNeuralProcessor::new();
    processor.init(sample_rate);
    Box::new(processor)
}

#[cfg(feature = "lab")]
fn rust_neural_param_metadata() -> Vec<ParameterMeta> {
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
        backend: Some("rust"),
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::RustNeuralProcessor;
    use crate::bridge::ExternalProcessor;

    #[test]
    fn test_rust_neural_get_param_returns_stored_value() {
        let mut processor = RustNeuralProcessor::new();

        processor.set_drive(0.35);
        processor.set_output_gain(1.4);

        assert_eq!(processor.get_param("drive"), Some(0.35));
        assert_eq!(processor.get_param("output_gain"), Some(1.4));

        assert!(processor.set_param("drive", 0.75));
        assert_eq!(processor.get_param("drive"), Some(0.75));
        assert_eq!(processor.get_param("missing"), None);

        assert!(processor.set_param("neural_output_gain", 0.9));
        assert_eq!(processor.get_param("neural_output_gain"), Some(0.9));
    }

    #[test]
    fn test_rust_neural_drive_zero_is_silence() {
        let mut processor = RustNeuralProcessor::new();
        processor.init(48_000.0);
        processor.set_drive(0.0);
        processor.set_output_gain(1.0);

        let mut buffer = [0.5_f32, -0.8, 1.0, -1.0, 0.25];
        processor.process_block(buffer.as_mut_ptr(), buffer.len());

        for sample in buffer {
            assert_eq!(sample, 0.0);
        }
    }

    #[test]
    fn test_rust_neural_extreme_input_never_produces_nan() {
        let mut processor = RustNeuralProcessor::new();
        processor.init(48_000.0);
        processor.set_drive(2.0);
        processor.set_output_gain(1.5);

        let mut buffer = [10.0_f32, -10.0, f32::MAX, f32::MIN, 0.0];
        processor.process_block(buffer.as_mut_ptr(), buffer.len());

        for sample in buffer {
            assert!(sample.is_finite(), "expected finite sample, got {sample}");
        }
    }

    #[test]
    fn test_rust_neural_not_ready_before_init_is_noop() {
        let mut processor = RustNeuralProcessor::new();
        let mut buffer = [1.0_f32, 2.0, 3.0];
        let original = buffer;

        processor.process_block(buffer.as_mut_ptr(), buffer.len());

        assert_eq!(buffer, original);
    }
}
