//! Managed, persisted library of cabinet impulse responses.
//!
//! Three responsibilities kept deliberately separate:
//! - [`library::CabinetLibrary`] — the library (which IRs exist), SQLite-backed,
//!   accessed only from the UI/worker thread.
//! - [`engine::CabinetEngine`] — the audio object, owned by the audio thread,
//!   receiving fully-built runtimes via a lock-free [`engine::CabinetMailbox`].
//! - [`runtime::CabinetRuntime`] — a built convolver pair ready for the audio thread.

pub mod engine;
pub mod library;
pub mod runtime;
pub mod types;

pub use engine::{CabinetEngine, CabinetMailbox};
pub use library::CabinetLibrary;
pub use types::*; // IrMeta, CabinetRuntime, CabinetError
