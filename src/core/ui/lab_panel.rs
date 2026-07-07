use crate::lab::Lab;
use nih_plug_egui::egui;

/// Minimal UI state retained for the Component Lab panel.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LabUiState {
    pub is_open: bool,
    pub export_progress: Option<ExportProgress>,
    pub last_error: Option<String>,
    pub switch_state: Option<SwitchProgress>,
    pub last_status: Option<String>,
}

impl Default for LabUiState {
    fn default() -> Self {
        Self {
            is_open: false,
            export_progress: None,
            last_error: None,
            switch_state: None,
            last_status: None,
        }
    }
}

impl LabUiState {
    pub fn set_error(&mut self, error: impl Into<String>) {
        self.last_error = Some(error.into());
        self.last_status = None;
    }

    pub fn set_status(&mut self, status: impl Into<String>) {
        self.last_status = Some(status.into());
        self.last_error = None;
    }

    pub fn clear_error(&mut self) {
        self.last_error = None;
    }
}

/// Export operation progress shown by the lab panel.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportProgress {
    pub label: String,
    pub files_done: usize,
    pub files_total: usize,
}

/// Variant switch progress shown by the lab panel.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SwitchProgress {
    pub category_id: String,
    pub variant_id: String,
}

pub fn draw_lab_panel(
    ctx: &egui::Context,
    lab: Option<&Lab>,
    state: &mut LabUiState,
    init_error: Option<&str>,
) {
    if !state.is_open {
        return;
    }

    let mut open = state.is_open;
    egui::Window::new("Component Lab")
        .open(&mut open)
        .resizable(true)
        .default_width(420.0)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("State:");
                if lab.is_some() {
                    ui.colored_label(egui::Color32::from_rgb(70, 180, 100), "ready");
                } else {
                    ui.colored_label(egui::Color32::from_rgb(220, 90, 70), "unavailable");
                }
            });

            if let Some(error) = init_error {
                ui.colored_label(egui::Color32::from_rgb(220, 120, 70), error);
            }

            let Some(lab) = lab else {
                return;
            };

            ui.label(format!(
                "Database: {}",
                lab.data_dir().join("lab.db").display()
            ));
            ui.label(format!("Factories: {}", lab.registry().len()));

            ui.separator();
            ui.heading("Categories");

            let categories = lab
                .db()
                .lock()
                .ok()
                .and_then(|db| db.list_categories().ok())
                .unwrap_or_default();
            ui.label(format!("Loaded: {}", categories.len()));
            egui::Grid::new("lab_categories_grid")
                .num_columns(3)
                .striped(true)
                .show(ui, |ui| {
                    ui.strong("Category");
                    ui.strong("Variants");
                    ui.strong("Status");
                    ui.end_row();
                    for category in &categories {
                        ui.label(&category.name);
                        ui.label(known_variants_for_category(&category.id).join(", "));
                        ui.label("loaded");
                        ui.end_row();
                    }
                });

            ui.separator();
            ui.heading("Registered Variants");
            for id in lab.registry().ids() {
                ui.label(id);
            }

            ui.separator();
            ui.horizontal(|ui| {
                if ui.button("Save Snapshot").clicked() {
                    match lab.save_snapshot() {
                        Ok(()) => state.set_status("snapshot saved"),
                        Err(err) => state.set_error(err.to_string()),
                    }
                }
                if ui.button("Load Snapshot").clicked() {
                    match lab.load_snapshot() {
                        Ok(()) => state.set_status("snapshot loaded"),
                        Err(err) => state.set_error(err.to_string()),
                    }
                }
                if ui.button("Export").clicked() {
                    match lab.export_component() {
                        Ok(path) => state.set_status(format!("exported {}", path.display())),
                        Err(err) => state.set_error(err.to_string()),
                    }
                }
            });

            if let Some(progress) = &state.export_progress {
                ui.label(format!(
                    "Export: {} ({}/{})",
                    progress.label, progress.files_done, progress.files_total
                ));
            }
            if let Some(progress) = &state.switch_state {
                ui.label(format!(
                    "Switching {} -> {}",
                    progress.category_id, progress.variant_id
                ));
            }
            if let Some(status) = &state.last_status {
                ui.colored_label(egui::Color32::from_rgb(90, 170, 220), status);
            }
            if let Some(error) = state.last_error.clone() {
                ui.horizontal(|ui| {
                    ui.colored_label(egui::Color32::from_rgb(220, 90, 70), error);
                    if ui.button("Dismiss").clicked() {
                        state.clear_error();
                    }
                });
            }
        });
    state.is_open = open;
}

fn known_variants_for_category(category_id: &str) -> Vec<&'static str> {
    match category_id {
        "eq" => vec!["faust-eq"],
        "amp-modeler" => vec!["mojo-neural", "mlc-zero-v"],
        _ => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::LabUiState;

    #[test]
    fn status_clears_previous_error() {
        let mut state = LabUiState::default();

        state.set_error("failed");
        state.set_status("ok");

        assert_eq!(state.last_status.as_deref(), Some("ok"));
        assert!(state.last_error.is_none());
    }

    #[test]
    fn error_clears_previous_status() {
        let mut state = LabUiState::default();

        state.set_status("ok");
        state.set_error("failed");

        assert_eq!(state.last_error.as_deref(), Some("failed"));
        assert!(state.last_status.is_none());
    }
}
