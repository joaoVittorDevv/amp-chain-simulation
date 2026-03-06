use eframe::egui;
use crate::core::ui::{draw_signal_chain, draw_spectrum};

pub fn render_shared_panels(
    ctx: &egui::Context,
    panel_open: &mut bool,
    spectrum: &[f32],
    fft_size: usize,
    mut draw_controls: impl FnMut(&mut egui::Ui),
) {
    if *panel_open {
        egui::TopBottomPanel::bottom("plugin_panel")
            .resizable(true)
            .min_height(100.0)
            .show(ctx, |ui| {
                ui.heading("Cabinet Simulator");
                ui.separator();
                ui.horizontal(|ui| {
                    draw_controls(ui);
                });
            });
    }

    egui::TopBottomPanel::bottom("signal_chain_panel")
        .resizable(false)
        .show(ctx, |ui| {
            draw_signal_chain(ui, panel_open);
        });

    egui::CentralPanel::default().show(ctx, |ui| {
        draw_spectrum(ui, spectrum, fft_size);
    });
}
