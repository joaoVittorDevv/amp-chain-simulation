use nih_plug_egui::egui;
use crate::core::ui::{draw_signal_chain, ActivePanel};
use crate::core::ui::draw_spectrum;

pub fn render_shared_panels(
    ctx: &egui::Context,
    active_panel: &mut ActivePanel,
    spectrum: &[f32],
    fft_size: usize,
    neural_active: bool,
    preamp_active: bool,
    cabinet_active: bool,
    global_bypass: bool,
    on_neural_toggle: impl FnMut(),
    on_preamp_toggle: impl FnMut(),
    on_cabinet_toggle: impl FnMut(),
    mut draw_neural_controls: impl FnMut(&mut egui::Ui),
    mut draw_preamp_controls: impl FnMut(&mut egui::Ui),
    mut draw_cabinet_controls: impl FnMut(&mut egui::Ui),
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
                    ActivePanel::Preamp => {
                        ui.heading("🎸 Preamp");
                        ui.separator();
                        ui.horizontal_wrapped(|ui| {
                            draw_preamp_controls(ui);
                        });
                    }
                    ActivePanel::Cabinet => {
                        ui.heading("🔊 Cabinet Simulator");
                        ui.separator();
                        ui.horizontal_wrapped(|ui| {
                            draw_cabinet_controls(ui);
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
                preamp_active,
                cabinet_active,
                global_bypass,
                on_neural_toggle,
                on_preamp_toggle,
                on_cabinet_toggle
            );
        });

    // --- Spectrum analyzer in the center ---
    egui::CentralPanel::default().show(ctx, |ui| {
        draw_spectrum(ui, spectrum, fft_size);
    });
}
