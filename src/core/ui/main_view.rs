use crate::core::state::plugin_params::AmpModel;
use crate::core::ui::draw_spectrum;
use crate::core::ui::{draw_signal_chain, ActivePanel};
use nih_plug_egui::egui;

pub fn draw_eq_band(
    ui: &mut egui::Ui,
    title: &str,
    add_freq: impl FnOnce(&mut egui::Ui),
    add_gain: impl FnOnce(&mut egui::Ui),
    add_q: impl FnOnce(&mut egui::Ui),
) {
    ui.vertical_centered(|ui| {
        ui.label(
            egui::RichText::new(title)
                .strong()
                .color(egui::Color32::from_rgb(200, 200, 255)),
        );
        ui.add_space(5.0);

        ui.group(|ui| {
            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new("Freq")
                        .small()
                        .color(egui::Color32::GRAY),
                );
                add_freq(ui);
                ui.add_space(4.0);

                ui.label(
                    egui::RichText::new("Gain")
                        .small()
                        .color(egui::Color32::GRAY),
                );
                add_gain(ui);
                ui.add_space(4.0);

                ui.label(egui::RichText::new("Q").small().color(egui::Color32::GRAY));
                add_q(ui);
            });
        });
    });
}

#[allow(clippy::too_many_arguments)]
pub fn render_shared_panels(
    ctx: &egui::Context,
    active_panel: &mut ActivePanel,
    spectrum: &[f32],
    fft_size: usize,
    eq_active: bool,
    neural_active: bool,
    cabinet_active: bool,
    global_bypass: bool,
    amp_model: AmpModel,
    on_eq_toggle: impl FnMut(),
    on_neural_toggle: impl FnMut(),
    on_cabinet_toggle: impl FnMut(),
    on_neural_select: impl FnMut(),
    on_mlc_select: impl FnMut(),
    mut draw_eq_controls: impl FnMut(&mut egui::Ui),
    mut draw_neural_controls: impl FnMut(&mut egui::Ui),
    mut draw_mlc_controls: impl FnMut(&mut egui::Ui),
    mut draw_cabinet_controls: impl FnMut(&mut egui::Ui),
) {
    // --- Bottom panel: Controls ---
    if *active_panel != ActivePanel::None {
        egui::TopBottomPanel::bottom("plugin_panel")
            .resizable(true)
            .min_height(110.0)
            .max_height(ctx.available_rect().height() * 0.7)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, true])
                    .show(ui, |ui| match *active_panel {
                        ActivePanel::EQ => {
                            ui.heading("🎛 Parametric EQ (Faust)");
                            ui.separator();
                            ui.horizontal_wrapped(|ui| {
                                draw_eq_controls(ui);
                            });
                        }
                        ActivePanel::NeuralAmp => {
                            ui.heading("🧠 Neural Amp (Mojo)");
                            ui.separator();
                            ui.horizontal_wrapped(|ui| {
                                draw_neural_controls(ui);
                            });
                        }
                        ActivePanel::MlcZeroV => {
                            ui.heading("MLC ZERO V Signature");
                            ui.separator();
                            draw_mlc_controls(ui);
                        }
                        ActivePanel::Cabinet => {
                            draw_cabinet_controls(ui);
                        }
                        ActivePanel::None => {}
                    });
            });
    }

    // --- Signal chain always at the bottom ---
    egui::TopBottomPanel::bottom("signal_chain_panel")
        .resizable(false)
        .show(ctx, |ui| {
            draw_signal_chain(
                ui,
                active_panel,
                eq_active,
                neural_active,
                cabinet_active,
                global_bypass,
                amp_model,
                on_eq_toggle,
                on_neural_toggle,
                on_cabinet_toggle,
                on_neural_select,
                on_mlc_select,
            );
        });

    // --- Spectrum analyzer in the center ---
    egui::CentralPanel::default().show(ctx, |ui| {
        draw_spectrum(ui, spectrum, fft_size);
    });
}
