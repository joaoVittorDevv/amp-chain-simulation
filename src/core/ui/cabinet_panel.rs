use crate::core::cabinet::IrMeta;
use nih_plug_egui::egui;

/// Transient rename editor state, stashed in egui temp memory so the shared
/// panel stays stateless across the plugin/standalone targets.
#[derive(Clone, Default)]
struct RenameState {
    hash: String,
    buf: String,
    active: bool,
}

/// Draw the Cabinet IR control panel using the project's closure-injection
/// pattern. All persistence/audio side effects happen in the injected closures
/// so this body is shared verbatim between the plugin and standalone targets.
#[allow(clippy::too_many_arguments)]
pub fn draw_cabinet_panel(
    ui: &mut egui::Ui,
    ir_list: &[IrMeta],
    active_hash: &str,
    error: Option<&str>,
    get_bypass: impl Fn() -> bool,
    mut set_bypass: impl FnMut(bool),
    get_level: impl Fn() -> f32,
    mut set_level: impl FnMut(f32),
    get_mix: impl Fn() -> f32,
    mut set_mix: impl FnMut(f32),
    mut select_ir: impl FnMut(String),
    mut import_ir: impl FnMut(),
    mut delete_ir: impl FnMut(String),
    mut rename_ir: impl FnMut(String, String),
    mut export_ir: impl FnMut(String),
) {
    use egui_knob::{Knob, KnobStyle};

    let rename_id = egui::Id::new("cabinet_rename_state");
    let mut rename: RenameState = ui
        .data_mut(|d| d.get_temp::<RenameState>(rename_id))
        .unwrap_or_default();

    // --- Header: bypass toggle top-right ---
    ui.horizontal(|ui| {
        ui.heading("📦 Cabinet IR");
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let mut bypassed = get_bypass();
            if ui.checkbox(&mut bypassed, "Bypass").changed() {
                set_bypass(bypassed);
            }
        });
    });
    ui.separator();

    // --- Knobs row: LEVEL, MIX ---
    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.label(
                egui::RichText::new("LEVEL")
                    .small()
                    .color(egui::Color32::GRAY),
            );
            // level is stored as a linear gain; show it in dB.
            let mut level = get_level();
            if ui
                .add(Knob::new(&mut level, 0.0631, 3.9811, KnobStyle::Wiper).with_size(45.0))
                .changed()
            {
                set_level(level);
            }
            let db = 20.0 * get_level().max(1e-6).log10();
            ui.label(egui::RichText::new(format!("{:+.1} dB", db)).small());
        });

        ui.add_space(16.0);

        ui.vertical(|ui| {
            ui.label(
                egui::RichText::new("MIX")
                    .small()
                    .color(egui::Color32::GRAY),
            );
            let mut mix = get_mix();
            if ui
                .add(Knob::new(&mut mix, 0.0, 1.0, KnobStyle::Wiper).with_size(45.0))
                .changed()
            {
                set_mix(mix);
            }
            ui.label(egui::RichText::new(format!("{:.0}%", get_mix() * 100.0)).small());
        });
    });

    ui.add_space(6.0);

    // --- Active IR info line ---
    let active = ir_list.iter().find(|m| m.content_hash == active_hash);
    match active {
        Some(m) => {
            ui.label(egui::RichText::new(format!("IR Ativo: {}", m.name)).strong());
            let ch = if m.channels == 1 { "mono" } else { "stereo" };
            ui.label(
                egui::RichText::new(format!(
                    "Sample rate: {} Hz | Canais: {} | {} frames | {} bit",
                    m.sample_rate, ch, m.num_frames, m.bit_depth
                ))
                .small()
                .color(egui::Color32::GRAY),
            );
        }
        None => {
            ui.label(
                egui::RichText::new("Nenhum IR de caixa selecionado")
                    .italics()
                    .color(egui::Color32::from_rgb(200, 170, 90)),
            );
        }
    }

    // --- Error area ---
    if let Some(err) = error {
        ui.add_space(4.0);
        ui.label(
            egui::RichText::new(format!("⚠ {}", err)).color(egui::Color32::from_rgb(220, 90, 90)),
        );
    }

    ui.add_space(6.0);

    // --- IR library browser ---
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("IR Library").strong());
        ui.label(
            egui::RichText::new(format!("[{}]", ir_list.len()))
                .small()
                .color(egui::Color32::GRAY),
        );
    });

    if ir_list.is_empty() {
        ui.add_space(8.0);
        ui.vertical_centered(|ui| {
            ui.label(
                egui::RichText::new("Nenhum IR de caixa carregado").color(egui::Color32::GRAY),
            );
            if ui.button("📂 Load IR…").clicked() {
                import_ir();
            }
        });
    } else {
        egui::ScrollArea::vertical()
            .max_height(120.0)
            .auto_shrink([false, true])
            .show(ui, |ui| {
                for m in ir_list {
                    let is_active = m.content_hash == active_hash;
                    let bullet = if is_active { "● " } else { "   " };
                    let ch = if m.channels == 1 { "mono" } else { "stereo" };
                    let label = format!("{}{}   {}k {}", bullet, m.name, m.sample_rate / 1000, ch);
                    if ui.selectable_label(is_active, label).clicked() && !is_active {
                        select_ir(m.content_hash.clone());
                    }
                }
            });
    }

    ui.add_space(6.0);

    // --- Rename inline editor (if active) ---
    if rename.active {
        ui.horizontal(|ui| {
            ui.label("Novo nome:");
            ui.text_edit_singleline(&mut rename.buf);
            if ui.button("OK").clicked() && !rename.buf.trim().is_empty() {
                rename_ir(rename.hash.clone(), rename.buf.trim().to_string());
                rename.active = false;
            }
            if ui.button("Cancelar").clicked() {
                rename.active = false;
            }
        });
    }

    // --- Action buttons ---
    ui.horizontal(|ui| {
        if ui.button("📂 Load IR…").clicked() {
            import_ir();
        }
        let has_active = !active_hash.is_empty() && active.is_some();
        ui.add_enabled_ui(has_active, |ui| {
            if ui.button("🗑 Delete").clicked() {
                delete_ir(active_hash.to_string());
            }
            if ui.button("✏ Rename").clicked() {
                rename = RenameState {
                    hash: active_hash.to_string(),
                    buf: active.map(|m| m.name.clone()).unwrap_or_default(),
                    active: true,
                };
            }
            if ui.button("💾 Export").clicked() {
                export_ir(active_hash.to_string());
            }
        });
    });

    ui.data_mut(|d| d.insert_temp(rename_id, rename));
}
