use serde::{Deserialize, Serialize};

/// Metadata for a single cabinet impulse response stored in the library.
///
/// The `content_hash` (BLAKE3 hex of the exact WAV bytes) is the stable,
/// cross-session identity of the IR — used for dedup, persistence and
/// integrity verification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IrMeta {
    pub content_hash: String,
    pub name: String,
    pub filename: String,
    pub sample_rate: u32,
    pub channels: u16,
    pub num_frames: usize,
    pub bit_depth: u16,
    pub byte_size: usize,
    pub created_at: String,
    pub last_used_at: Option<String>,
}

/// A fully-built, audio-thread-ready cabinet convolver pair.
///
/// Constructed off the audio thread (allocates, initializes FFTs, resamples)
/// and handed to the [`super::engine::CabinetEngine`] via `ArcSwap`.
/// (`FFTConvolver` is not `Debug`, so this type isn't either.)
#[derive(Clone)]
pub struct CabinetRuntime {
    pub convolver_l: fft_convolver::FFTConvolver<f32>,
    pub convolver_r: fft_convolver::FFTConvolver<f32>,
    pub ir_hash: String,
    pub num_frames: usize,
}

/// Errors that can occur while managing the cabinet IR library and runtime.
#[derive(Debug, thiserror::Error)]
pub enum CabinetError {
    #[error("Database error: {0}")]
    Database(String),
    #[error("WAV decode error: {0}")]
    WavDecode(String),
    #[error("IR not found: {0}")]
    NotFound(String),
    #[error("Corrupt IR: {0}")]
    Corrupt(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Resample error: {0}")]
    Resample(String),
}
