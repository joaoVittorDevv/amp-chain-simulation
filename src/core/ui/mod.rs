pub mod spectrum;
pub mod signal_chain;
pub mod main_view;
pub mod cabinet_panel;

pub use spectrum::draw_spectrum;
pub use signal_chain::{draw_signal_chain, ActivePanel};
pub use main_view::render_shared_panels;
pub use cabinet_panel::draw_cabinet_panel;
