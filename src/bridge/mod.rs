pub mod faust;
pub mod mojo;

/// Trait comum para Processadores de Áudio Externos (Zero-Copy)
pub trait ExternalProcessor {
    /// Inicializa a instância informando a taxa de amostragem (Sample Rate)
    fn init(&mut self, sample_rate: f32);

    /// Processa o bloco de áudio in-place
    /// `buffer`: ponteiro bruto para a região contígua de amostras
    /// `length`: número de amostras a processar no buffer
    fn process_block(&mut self, buffer: *mut f32, length: usize);
}
