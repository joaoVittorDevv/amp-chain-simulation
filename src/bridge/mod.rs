pub mod faust;
pub mod mlc_zero_v;
pub mod mojo;
pub mod oversampling;

use crate::lab::ParameterMeta;

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
