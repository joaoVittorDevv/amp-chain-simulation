use nih_plug_egui::egui;
use crate::core::ui::{draw_signal_chain, ActivePanel};
use crate::core::ui::draw_spectrum;

pub fn render_shared_panels(
    ctx: &egui::Context,
    active_panel: &mut ActivePanel,
    spectrum: &[f32],
    fft_size: usize,
    neural_active: bool,
    global_bypass: bool,
    on_neural_toggle: impl FnMut(),
    mut draw_neural_controls: impl FnMut(&mut egui::Ui),
) {
    // --- Bottom panel: Controls ---
    if *active_panel != ActivePanel::None {
        egui::TopBottomPanel::bottom("plugin_panel")
            .resizable(true)
            .min_height(110.0)
            .show(ctx, |ui| {
                match *active_panel {
                    ActivePanel::NeuralAmp => {
                        ui.heading("🧠 Neural Amp (PyTorch)");
                        ui.separator();
                        ui.horizontal_wrapped(|ui| {
                            draw_neural_controls(ui);
                        });
                    }
                    ActivePanel::None => {}
                }
            });
    }

    // --- Signal chain always at the bottom ---
    egui::TopBottomPanel::bottom("signal_chain_panel")
        .resizable(false)
        .show(ctx, |ui| {
            draw_signal_chain(
                ui,
                active_panel,
                neural_active,
                global_bypass,
                on_neural_toggle,
            );
        });

    // --- Spectrum analyzer in the center ---
    egui::CentralPanel::default().show(ctx, |ui| {
        draw_spectrum(ui, spectrum, fft_size);
    });
}
