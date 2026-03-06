use eframe::egui;
use crate::core::ui::{draw_signal_chain, ActivePanel};
use crate::core::ui::draw_spectrum;

pub fn render_shared_panels(
    ctx: &egui::Context,
    active_panel: &mut ActivePanel,
    spectrum: &[f32],
    fft_size: usize,
    mut draw_preamp_controls: impl FnMut(&mut egui::Ui),
    mut draw_cabinet_controls: impl FnMut(&mut egui::Ui),
) {
    // --- Bottom panel: Preamp or Cabinet controls ---
    if *active_panel != ActivePanel::None {
        egui::TopBottomPanel::bottom("plugin_panel")
            .resizable(true)
            .min_height(110.0)
            .show(ctx, |ui| {
                match *active_panel {
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
            draw_signal_chain(ui, active_panel);
        });

    // --- Spectrum analyzer in the center ---
    egui::CentralPanel::default().show(ctx, |ui| {
        draw_spectrum(ui, spectrum, fft_size);
    });
}
