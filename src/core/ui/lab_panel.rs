use crate::lab::{
    default_variant_ids_for_category, Category, ComponentMeta, DependencyList, DspConfig,
    DspEngine, FfiInterface, IntegrationGuide, Lab, ParameterMeta, RoutingConfig, SnapshotData,
    SnapshotFull, SnapshotMeta, SnapshotStatus,
};
use nih_plug_egui::egui;

/// Minimal UI state retained for the Component Lab panel.
#[derive(Debug, Clone)]
pub struct LabUiState {
    pub is_open: bool,
    pub selected_category: Option<String>,
    pub selected_snapshot_id: String,
    pub selected_variant_id: Option<String>,
    pub available_snapshots: Vec<SnapshotMeta>,
    pub snapshots_variant_id: Option<String>,
    pub export_progress: Option<ExportProgress>,
    pub last_error: Option<String>,
    pub switch_state: Option<SwitchProgress>,
    pub last_status: Option<String>,
    pub pending_delete: bool,
    pub selected_node_for_params: Option<String>,
    pub node_params: Vec<(String, f32, ParameterMeta)>,
}

impl Default for LabUiState {
    fn default() -> Self {
        Self {
            is_open: false,
            selected_category: None,
            selected_snapshot_id: String::new(),
            selected_variant_id: None,
            available_snapshots: Vec::new(),
            snapshots_variant_id: None,
            export_progress: None,
            last_error: None,
            switch_state: None,
            last_status: None,
            pending_delete: false,
            selected_node_for_params: None,
            node_params: Vec::new(),
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

            if !categories.is_empty()
                && state
                    .selected_category
                    .as_ref()
                    .is_none_or(|id| !categories.iter().any(|category| category.id == *id))
            {
                let category = default_variant_category(&categories).unwrap_or(&categories[0]);
                state.selected_category = Some(category.id.clone());
                state.selected_variant_id = known_variants_for_category(&category.id)
                    .first()
                    .map(|id| (*id).to_string());
                refresh_snapshots_for_selected_variant(lab, state);
            }

            ui.separator();
            ui.heading("Variant Switch");

            let selected_category_label = state
                .selected_category
                .as_deref()
                .and_then(|selected| {
                    categories
                        .iter()
                        .find(|category| category.id == selected)
                        .map(|category| category.name.as_str())
                })
                .unwrap_or("Select category");

            egui::ComboBox::from_id_salt("lab_selected_category")
                .selected_text(selected_category_label)
                .show_ui(ui, |ui| {
                    for category in &categories {
                        let selected =
                            state.selected_category.as_deref() == Some(category.id.as_str());
                        if ui.selectable_label(selected, &category.name).clicked() {
                            state.selected_category = Some(category.id.clone());
                            state.selected_variant_id = known_variants_for_category(&category.id)
                                .first()
                                .map(|id| (*id).to_string());
                            refresh_snapshots_for_selected_variant(lab, state);
                        }
                    }
                });

            let selected_category_id = state.selected_category.as_deref().unwrap_or_default();
            let variants = known_variants_for_category(selected_category_id);
            if !variants.is_empty()
                && state
                    .selected_variant_id
                    .as_ref()
                    .is_none_or(|id| !variants.iter().any(|variant| *variant == id))
            {
                state.selected_variant_id = Some(variants[0].to_string());
                refresh_snapshots_for_selected_variant(lab, state);
            }
            ensure_snapshots_fresh(lab, state);

            let selected_variant_label = state
                .selected_variant_id
                .as_deref()
                .unwrap_or("Select variant");

            egui::ComboBox::from_id_salt("lab_selected_variant")
                .selected_text(selected_variant_label)
                .show_ui(ui, |ui| {
                    if variants.is_empty() {
                        ui.label("No known variants");
                    }
                    for variant_id in &variants {
                        let response = ui.selectable_value(
                            &mut state.selected_variant_id,
                            Some((*variant_id).to_string()),
                            *variant_id,
                        );
                        if response.clicked() {
                            refresh_snapshots_for_selected_variant(lab, state);
                        }
                    }
                });

            if ui.button("Switch Variant").clicked() {
                match (
                    state.selected_category.as_deref(),
                    state.selected_variant_id.as_deref(),
                ) {
                    (Some(category_id), Some(variant_id)) => {
                        match lab.switch_variant(category_id, variant_id) {
                            Ok(()) => {
                                state.switch_state = Some(SwitchProgress {
                                    category_id: category_id.to_string(),
                                    variant_id: variant_id.to_string(),
                                });
                                state.set_status(format!("switched {category_id} to {variant_id}"));
                            }
                            Err(err) => state.set_error(err.to_string()),
                        }
                    }
                    _ => state.set_error("select a category and variant first"),
                }
            }

            ui.separator();
            ui.heading("Registered Variants");
            for id in lab.registry().ids() {
                ui.label(id);
            }

            ui.separator();
            ui.label("Snapshots");
            if state.selected_variant_id.is_some() {
                if state.available_snapshots.is_empty() {
                    ui.label("No snapshots saved for this variant");
                } else {
                    let selected_snapshot_label = state
                        .available_snapshots
                        .iter()
                        .find(|snapshot| snapshot.id == state.selected_snapshot_id)
                        .map(snapshot_option_label)
                        .unwrap_or_else(|| "Select snapshot".to_string());

                    egui::ComboBox::from_id_salt("lab_selected_snapshot")
                        .selected_text(selected_snapshot_label)
                        .show_ui(ui, |ui| {
                            for snapshot in &state.available_snapshots {
                                ui.selectable_value(
                                    &mut state.selected_snapshot_id,
                                    snapshot.id.clone(),
                                    snapshot_option_label(snapshot),
                                );
                            }
                        });
                }
            } else {
                ui.label("Select a variant to browse snapshots");
            }
            ui.horizontal(|ui| {
                ui.label("or type ID:");
                ui.text_edit_singleline(&mut state.selected_snapshot_id);
            });
            ui.horizontal(|ui| {
                if ui.button("Save Snapshot").clicked() {
                    let full = minimal_snapshot(state);
                    match lab.save_snapshot(&full) {
                        Ok(()) => {
                            state.selected_snapshot_id = full.meta.id;
                            refresh_snapshots_for_selected_variant(lab, state);
                            state.set_status("snapshot saved");
                        }
                        Err(err) => state.set_error(err.to_string()),
                    }
                }
                if ui.button("Load Snapshot").clicked() {
                    match lab.load_snapshot(&state.selected_snapshot_id) {
                        Ok(full) => {
                            let snapshot_id = full.meta.id.clone();
                            state.selected_variant_id =
                                Some(full.data.component.variant_id.clone());
                            state.selected_category = Some(full.data.component.category.clone());
                            refresh_snapshots_for_selected_variant(lab, state);
                            state.selected_snapshot_id = snapshot_id.clone();
                            state.set_status(format!("loaded & applied {snapshot_id}"));
                        }
                        Err(err) => state.set_error(err.to_string()),
                    }
                }
                if ui.button("Export").clicked() {
                    match lab.export_component(&state.selected_snapshot_id) {
                        Ok(path) => state.set_status(format!("exported {}", path.display())),
                        Err(err) => state.set_error(err.to_string()),
                    }
                }
                let has_selection = !state.selected_snapshot_id.trim().is_empty();
                if ui
                    .add_enabled(has_selection, egui::Button::new("🗑 Delete"))
                    .clicked()
                {
                    state.pending_delete = true;
                }
            });

            if state.pending_delete {
                ui.horizontal(|ui| {
                    ui.colored_label(
                        egui::Color32::from_rgb(220, 90, 70),
                        format!("Delete snapshot '{}'?", state.selected_snapshot_id),
                    );
                    if ui.button("Confirm Delete").clicked() {
                        match lab.delete_snapshot(&state.selected_snapshot_id) {
                            Ok(()) => {
                                let deleted = state.selected_snapshot_id.clone();
                                state.selected_snapshot_id.clear();
                                refresh_snapshots_for_selected_variant(lab, state);
                                state.set_status(format!("deleted {deleted}"));
                            }
                            Err(err) => state.set_error(err.to_string()),
                        }
                        state.pending_delete = false;
                    }
                    if ui.button("Cancel").clicked() {
                        state.pending_delete = false;
                    }
                });
            }

            ui.separator();
            ui.heading("Node Parameters");

            let selected_node_label = state
                .selected_node_for_params
                .as_deref()
                .and_then(|selected| {
                    categories
                        .iter()
                        .find(|category| category.id == selected)
                        .map(|category| category.name.as_str())
                })
                .unwrap_or("Select node");

            let mut do_refresh = false;

            egui::ComboBox::from_id_salt("node_params_category")
                .selected_text(selected_node_label)
                .show_ui(ui, |ui| {
                    for category in &categories {
                        let selected =
                            state.selected_node_for_params.as_deref() == Some(category.id.as_str());
                        if ui.selectable_label(selected, &category.name).clicked() {
                            state.selected_node_for_params = Some(category.id.clone());
                            do_refresh = true;
                        }
                    }
                });

            ui.add_space(4.0);
            if ui.button("🔄 Refresh").clicked() {
                do_refresh = true;
            }

            if do_refresh {
                if let Some(cat_id) = &state.selected_node_for_params {
                    match lab.get_node_params(cat_id) {
                        Ok(params) => {
                            state.node_params = params;
                        }
                        Err(err) => state.set_error(err.to_string()),
                    }
                } else {
                    state.node_params.clear();
                }
            }

            if state.node_params.is_empty() {
                ui.label("No parameters available for this node");
            } else {
                egui::ScrollArea::vertical()
                    .max_height(300.0)
                    .id_salt("node_params_scroll")
                    .show(ui, |ui| {
                        egui::Grid::new("node_params_grid")
                            .num_columns(5)
                            .striped(true)
                            .show(ui, |ui| {
                                ui.strong("Name");
                                ui.strong("Value");
                                ui.strong("Range");
                                ui.strong("Unit");
                                ui.strong("Pos");
                                ui.end_row();

                                for (_id, value, meta) in &state.node_params {
                                    let min = meta.range.0;
                                    let max = meta.range.1;

                                    ui.label(&meta.name);
                                    if *value < min || *value > max {
                                        ui.colored_label(
                                            egui::Color32::from_rgb(220, 90, 70),
                                            egui::RichText::new(format!("{:.4}", value))
                                                .monospace(),
                                        );
                                    } else if (*value - min).abs() < 1e-4
                                        || (*value - max).abs() < 1e-4
                                    {
                                        ui.colored_label(
                                            egui::Color32::from_rgb(220, 180, 70),
                                            egui::RichText::new(format!("{:.4}", value))
                                                .monospace(),
                                        );
                                    } else {
                                        ui.label(
                                            egui::RichText::new(format!("{:.4}", value))
                                                .monospace(),
                                        );
                                    }
                                    ui.label(format!("[{:.2}, {:.2}]", min, max));
                                    if let Some(unit) = &meta.unit {
                                        ui.label(unit);
                                    } else {
                                        ui.label("");
                                    }
                                    let normalized = if max > min {
                                        ((value - min) / (max - min)).clamp(0.0, 1.0)
                                    } else {
                                        0.5
                                    };
                                    ui.add(
                                        egui::ProgressBar::new(normalized).desired_width(60.0),
                                    );
                                    ui.end_row();
                                }
                            });
                    });
            }

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

fn refresh_snapshots_for_selected_variant(lab: &Lab, state: &mut LabUiState) {
    let Some(variant_id) = state.selected_variant_id.as_deref() else {
        state.available_snapshots.clear();
        state.snapshots_variant_id = None;
        state.selected_snapshot_id.clear();
        return;
    };

    let variant_id = variant_id.to_string();
    match lab.list_snapshots(&variant_id) {
        Ok(snapshots) => {
            let selected_id = state.selected_snapshot_id.trim();
            let selected_is_available = snapshots
                .iter()
                .any(|snapshot| snapshot.id.as_str() == selected_id);
            if !selected_is_available {
                state.selected_snapshot_id = snapshots
                    .first()
                    .map(|snapshot| snapshot.id.clone())
                    .unwrap_or_default();
            }
            state.available_snapshots = snapshots;
            state.snapshots_variant_id = Some(variant_id);
        }
        Err(err) => {
            state.available_snapshots.clear();
            state.snapshots_variant_id = None;
            state.set_error(err.to_string());
        }
    }
}

fn snapshot_option_label(snapshot: &SnapshotMeta) -> String {
    format!(
        "v{} — {} [{}]",
        snapshot.version, snapshot.created_at, snapshot.status
    )
}

fn ensure_snapshots_fresh(lab: &Lab, state: &mut LabUiState) {
    if state.snapshots_variant_id != state.selected_variant_id {
        refresh_snapshots_for_selected_variant(lab, state);
    }
}

pub(crate) fn minimal_snapshot(state: &LabUiState) -> SnapshotFull {
    let now = chrono::Utc::now().to_rfc3339();
    let snapshot_id = uuid::Uuid::new_v4().to_string();
    let version = format!("0.1.0+{snapshot_id}");
    let category_id = state
        .selected_category
        .clone()
        .unwrap_or_else(|| "unknown".to_string());
    let variant_id = state
        .selected_variant_id
        .clone()
        .unwrap_or_else(|| "unknown".to_string());

    SnapshotFull {
        meta: SnapshotMeta {
            id: snapshot_id,
            variant_id: variant_id.clone(),
            version: version.clone(),
            status: SnapshotStatus::Draft,
            variant_impl_id: variant_id.clone(),
            params_hash: String::new(),
            created_at: now.clone(),
            notes: String::new(),
        },
        data: SnapshotData {
            format: SnapshotData::FORMAT.to_string(),
            schema_version: SnapshotData::SCHEMA_VERSION.to_string(),
            component: ComponentMeta {
                name: category_id.clone(),
                category: category_id,
                variant_id,
                version,
                status: SnapshotStatus::Draft,
                created: now,
                author: String::new(),
                description: String::new(),
            },
            parameter_values: Vec::new(),
            parameter_metadata: Vec::new(),
            dsp: DspConfig {
                engine: DspEngine::PureRust,
                language: String::new(),
                source_files: Vec::new(),
                faust_libraries: Vec::new(),
                compile_command: String::new(),
                ffi_interface: FfiInterface {
                    init_fn: String::new(),
                    process_fn: String::new(),
                    cleanup_fn: String::new(),
                    param_count: 0,
                    param_ids: Vec::new(),
                    buffer_format: String::new(),
                },
            },
            dependencies: DependencyList {
                rust_crates: Vec::new(),
                system_tools: Vec::new(),
            },
            routing: RoutingConfig {
                audio_input: String::new(),
                audio_output: String::new(),
                latency_samples: 0,
            },
            verification_checks: Vec::new(),
            engineering_notes: String::new(),
            signal_flow_description: String::new(),
            integration_guide: IntegrationGuide {
                summary: String::new(),
                host_framework: String::new(),
                target_platforms: Vec::new(),
                steps: Vec::new(),
                rust_integration_example: String::new(),
            },
        },
    }
}

fn known_variants_for_category(category_id: &str) -> Vec<&'static str> {
    default_variant_ids_for_category(category_id)
}

fn default_variant_category(categories: &[Category]) -> Option<&Category> {
    categories
        .iter()
        .find(|category| !known_variants_for_category(&category.id).is_empty())
}

#[cfg(test)]
mod tests {
    use super::{minimal_snapshot, LabUiState};

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

    #[test]
    fn minimal_snapshot_ignores_selected_snapshot_id_when_saving() {
        let state = LabUiState {
            selected_category: Some("eq".to_string()),
            selected_variant_id: Some("faust-eq".to_string()),
            selected_snapshot_id: " typed-snapshot ".to_string(),
            ..LabUiState::default()
        };

        let full = minimal_snapshot(&state);

        assert!(!full.meta.id.is_empty());
        assert_ne!(full.meta.id, "typed-snapshot");
        assert_eq!(full.meta.version, format!("0.1.0+{}", full.meta.id));
        assert_eq!(full.meta.variant_id, "faust-eq");
        assert_eq!(full.meta.variant_impl_id, "faust-eq");
        assert_eq!(full.data.component.variant_id, "faust-eq");
        assert_eq!(full.data.component.version, full.meta.version);
    }

    #[test]
    fn minimal_snapshot_generates_id_when_input_is_blank() {
        let state = LabUiState {
            selected_category: Some("eq".to_string()),
            selected_variant_id: Some("faust-eq".to_string()),
            selected_snapshot_id: "   ".to_string(),
            ..LabUiState::default()
        };

        let full = minimal_snapshot(&state);

        assert!(!full.meta.id.is_empty());
        assert_ne!(full.meta.id, state.selected_snapshot_id);
    }
}
