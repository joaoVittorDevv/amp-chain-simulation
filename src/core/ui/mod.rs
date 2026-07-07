pub mod cabinet_panel;
#[cfg(feature = "lab")]
pub mod lab_panel;
pub mod main_view;
pub mod mlc_zero_v_panel;
pub mod signal_chain;
pub mod spectrum;

pub use cabinet_panel::draw_cabinet_panel;
#[cfg(feature = "lab")]
pub use lab_panel::{draw_lab_panel, ExportProgress, LabUiState, SwitchProgress};
pub use main_view::render_shared_panels;
pub use mlc_zero_v_panel::draw_mlc_zero_v_panel;
pub use signal_chain::{draw_signal_chain, ActivePanel};
pub use spectrum::draw_spectrum;
