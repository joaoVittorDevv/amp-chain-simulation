pub mod faust;
pub mod mlc_zero_v;
#[cfg(have_mojo)]
pub mod mojo;
pub mod neural_rust;

use crate::lab::ParameterMeta;

/// Neural backend resolved by capability, not by target OS (CROSS-03).
/// `have_mojo` is emitted by `build.rs` only when the Mojo toolchain was
/// found and the shared library compiled successfully.
#[cfg(have_mojo)]
pub type NeuralProcessor = mojo::MojoProcessor;
#[cfg(not(have_mojo))]
pub type NeuralProcessor = neural_rust::RustNeuralProcessor;

/// Canonical impl id persisted in `lab.db` and exported snapshots. Stays
/// `"mojo-neural"` regardless of which backend is compiled in, so a project
/// saved on one OS/toolchain combination opens on any other (CROSS-03).
#[cfg(feature = "lab")]
pub const MOJO_NEURAL_IMPL_ID: &str = "mojo-neural";

/// Registry factory for the neural amp modeler. Resolves to whichever
/// backend `NeuralProcessor` aliases to at compile time.
#[cfg(feature = "lab")]
pub fn mojo_neural_factory(sample_rate: f32) -> Box<dyn crate::lab::DspVariant> {
    let mut processor = NeuralProcessor::new();
    processor.init(sample_rate);
    Box::new(processor)
}

/// Trait comum para Processadores de Áudio Externos (Zero-Copy)
pub trait ExternalProcessor {
    /// Inicializa a instância informando a taxa de amostragem (Sample Rate)
    fn init(&mut self, sample_rate: f32);

    /// Processa o bloco de áudio in-place
    /// `buffer`: ponteiro bruto para a região contígua de amostras
    /// `length`: número de amostras a processar no buffer
    fn process_block(&mut self, buffer: *mut f32, length: usize);

    /// Read the current runtime value of a parameter by its ID.
    /// Returns None if the parameter ID is unknown.
    fn get_param(&self, _id: &str) -> Option<f32> {
        None
    }

    /// Set a parameter value by its ID.
    /// Returns true if the parameter was found and updated.
    fn set_param(&mut self, _id: &str, _value: f32) -> bool {
        false
    }

    /// Return metadata (id, name, range, default, unit) for all parameters.
    fn param_metadata(&self) -> Vec<ParameterMeta> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{ExternalProcessor, NeuralProcessor};

    #[test]
    fn neural_processor_alias_constructs_and_initializes() {
        let mut processor = NeuralProcessor::new();
        assert!(!processor.is_ready());

        processor.init(48_000.0);

        assert!(processor.is_ready());
    }
}
