//! Component Lab persistence and DSP variant selection foundation.

pub mod component;
pub mod database;
pub mod export;
pub mod node;
pub mod pipeline;
pub mod registry;
pub mod snapshot;
pub mod variant_runtime;
pub mod verification;

pub use component::{
    Category, CheckCategory, CheckResult, CheckStatus, CrateDep, DependencyList, DspConfig,
    DspEngine, DspVariant, FaustLib, FfiInterface, IntegrationGuide, Pipeline, PipelineConfig,
    RoutingConfig, SlotConfig, SourceFile, SystemTool, VariantMeta, VerificationCheckDef,
};
pub use database::Database;
pub use export::{ExportEngine, ExportManifest, ManifestEntry};
pub use node::{Node, NodeLoadState};
pub use pipeline::{NodeSlot, PipelineManager};
pub use registry::{VariantFactory, VariantRegistry};
pub use snapshot::{
    ComponentMeta, ParamValue, ParamValueChange, ParameterMeta, SnapshotData, SnapshotDiff,
    SnapshotFull, SnapshotMeta, SnapshotStatus, VariantChange,
};
pub use variant_runtime::{VariantMailbox, VariantSlot};
pub use verification::VerificationRunner;

use std::path::{Path, PathBuf};
use std::sync::Mutex;
use thiserror::Error;

/// Result alias used by Component Lab APIs.
pub type Result<T> = std::result::Result<T, LabError>;

/// Error type for Component Lab persistence, serialization, and facade APIs.
#[derive(Debug, Error)]
pub enum LabError {
    /// The platform did not provide a configuration directory.
    #[error("configuration directory is unavailable")]
    ConfigDirUnavailable,
    /// File-system operation failed.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    /// SQLite operation failed.
    #[error("database error: {0}")]
    Database(#[from] rusqlite::Error),
    /// JSON serialization or deserialization failed.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    /// Persisted lab data was structurally invalid.
    #[error("invalid lab data: {0}")]
    InvalidData(String),
    /// The requested API is reserved for a later Component Lab phase.
    #[error("not implemented")]
    NotImplemented,
}

/// Top-level Component Lab facade.
pub struct Lab {
    db: Mutex<Database>,
    pipeline: Pipeline,
    registry: VariantRegistry,
    data_dir: PathBuf,
}

impl Lab {
    /// Initialize the lab database, seed categories, and create the default pipeline.
    pub fn init(data_dir: PathBuf) -> Result<Self> {
        std::fs::create_dir_all(&data_dir)?;
        let db_path = data_dir.join("lab.db");
        let db = Database::open(&db_path)?;
        let categories = default_categories();
        db.seed_categories(&categories)?;

        let mut registry = VariantRegistry::new();
        register_default_variants(&mut registry);
        seed_registered_variants(&db, &registry)?;

        let persisted_pipeline = db.load_pipeline("default")?;
        let pipeline = match persisted_pipeline {
            Some(config) => Pipeline::from_config(config),
            None => {
                let pipeline = Pipeline::from_categories(&categories);
                db.save_pipeline("default", pipeline.config())?;
                pipeline
            }
        };

        Ok(Self {
            db: Mutex::new(db),
            pipeline,
            registry,
            data_dir,
        })
    }

    /// Borrow the lab database mutex.
    pub fn db(&self) -> &Mutex<Database> {
        &self.db
    }

    /// Borrow the current pipeline state.
    pub fn pipeline(&self) -> &Pipeline {
        &self.pipeline
    }

    /// Borrow the variant registry.
    pub fn registry(&self) -> &VariantRegistry {
        &self.registry
    }

    /// Borrow the lab data directory.
    pub fn data_dir(&self) -> &Path {
        &self.data_dir
    }

    /// Read all current parameter values and metadata for a node's active variant.
    ///
    /// Returns `(id, value, metadata)` triples in parameter order. An empty vector
    /// is returned when the category has no active variant.
    ///
    /// TODO: This rebuilds a `PipelineManager` (and re-instantiates the variant)
    /// on every call. Once the facade owns live parameter state, cache the manager
    /// or the resolved parameters instead of reconstructing them here. Intended for
    /// UI/snapshot use only — never call from the audio thread.
    pub fn get_node_params(
        &self,
        category_id: &str,
    ) -> Result<Vec<(String, f32, ParameterMeta)>> {
        let config = {
            let db = self.db.lock().unwrap();
            db.load_pipeline("default")?
                .unwrap_or_else(|| self.pipeline.config().clone())
        };

        let Some(variant_id) = config
            .slots
            .iter()
            .find(|slot| slot.category_id == category_id)
            .and_then(|slot| slot.active_variant_id.clone())
        else {
            return Ok(Vec::new());
        };

        let factory = self.registry.get(&variant_id).ok_or_else(|| {
            LabError::InvalidData(format!("variant '{variant_id}' not registered"))
        })?;

        let categories = self.db.lock().unwrap().list_categories()?;
        let mut manager = PipelineManager::from_config(&categories, config)?;
        manager.request_switch(category_id, &variant_id, factory, 48000.0)?;

        let node = manager.get_node_mut(category_id).ok_or_else(|| {
            LabError::InvalidData(format!("category '{category_id}' is not registered"))
        })?;
        // Move the freshly installed variant from the mailbox into the slot so its
        // runtime parameters can be read back.
        node.collect_mailbox();

        let params = node
            .param_metadata()
            .into_iter()
            .map(|meta| {
                let value = node.get_param(&meta.id).unwrap_or(meta.default);
                (meta.id.clone(), value, meta)
            })
            .collect();
        Ok(params)
    }

    /// Save a component snapshot, capturing current runtime parameter values.
    pub fn save_snapshot(&self, full: &SnapshotFull) -> Result<()> {
        let mut full = full.clone();
        // Capture live parameter values and metadata from the running variant so the
        // snapshot reflects the actual DSP state at save time. When the node has no
        // active variant, keep any caller-provided values untouched.
        if let Ok(params) = self.get_node_params(&full.data.component.category) {
            if !params.is_empty() {
                full.data.parameter_values = params
                    .iter()
                    .map(|(id, value, _)| ParamValue {
                        id: id.clone(),
                        value: *value,
                    })
                    .collect();
                full.data.parameter_metadata =
                    params.into_iter().map(|(_, _, meta)| meta).collect();
            }
        }
        let db = self.db.lock().unwrap();
        db.save_snapshot(&full)?;
        Ok(())
    }

    /// Load a component snapshot.
    pub fn load_snapshot(&self, id: &str) -> Result<SnapshotFull> {
        let full = {
            let db = self.db.lock().unwrap();
            db.load_snapshot(id)?
                .ok_or_else(|| LabError::InvalidData(format!("snapshot '{id}' not found")))?
        };

        let variant_id = full.data.component.variant_id.clone();
        let category = full.data.component.category.clone();
        let factory = self.registry.get(&variant_id).ok_or_else(|| {
            LabError::InvalidData(format!("variant '{variant_id}' not registered"))
        })?;

        let categories = self.db.lock().unwrap().list_categories()?;
        let mut manager =
            PipelineManager::from_config(&categories, self.pipeline.config().clone())?;

        // Loading applies the saved variant plus its saved parameter values to a
        // freshly instantiated runtime. Unknown parameter ids are ignored so a
        // snapshot from a different variant still loads its variant/routing config.
        manager.request_switch(&category, &variant_id, factory, 48000.0)?;
        for param in &full.data.parameter_values {
            manager.set_node_param(&category, &param.id, param.value)?;
        }

        let updated_config = manager.to_config("default");
        self.db
            .lock()
            .unwrap()
            .save_pipeline("default", &updated_config)?;
        Ok(full)
    }

    /// List snapshot metadata for a variant.
    pub fn list_snapshots(&self, variant_id: &str) -> Result<Vec<SnapshotMeta>> {
        let db = self.db.lock().unwrap();
        db.list_snapshots_for_variant(variant_id)
    }

    /// Delete a component snapshot.
    pub fn delete_snapshot(&self, id: &str) -> Result<()> {
        let db = self.db.lock().unwrap();
        db.delete_snapshot(id)
    }

    /// Switch a node to another compiled variant.
    pub fn switch_variant(&self, category_id: &str, variant_id: &str) -> Result<()> {
        let factory = self.registry.get(variant_id).ok_or_else(|| {
            LabError::InvalidData(format!("variant '{variant_id}' not registered"))
        })?;

        let categories = self.db.lock().unwrap().list_categories()?;
        let mut manager =
            PipelineManager::from_config(&categories, self.pipeline.config().clone())?;

        // Use default sample rate since Lab doesn't have access to audio thread state
        manager.request_switch(category_id, variant_id, factory, 48000.0)?;

        let updated_config = manager.to_config("default");
        self.db
            .lock()
            .unwrap()
            .save_pipeline("default", &updated_config)?;
        Ok(())
    }

    /// Export a component snapshot bundle.
    pub fn export_component(&self, snapshot_id: &str) -> Result<PathBuf> {
        let db = self.db.lock().unwrap();
        let engine = ExportEngine::new(&db);
        engine.export_snapshot(snapshot_id, &self.data_dir)
    }
}

#[derive(Clone, Copy)]
struct DefaultVariantSeed {
    id: &'static str,
    node_id: &'static str,
    category_id: &'static str,
    node_name: &'static str,
    name: &'static str,
    impl_id: &'static str,
}

const DEFAULT_VARIANT_SEEDS: &[DefaultVariantSeed] = &[
    DefaultVariantSeed {
        id: crate::bridge::faust::FAUST_EQ_IMPL_ID,
        node_id: "node-eq",
        category_id: "eq",
        node_name: "EQ",
        name: "Faust EQ",
        impl_id: crate::bridge::faust::FAUST_EQ_IMPL_ID,
    },
    DefaultVariantSeed {
        id: crate::bridge::mojo::MOJO_NEURAL_IMPL_ID,
        node_id: "node-amp-modeler",
        category_id: "amp-modeler",
        node_name: "Amp Modeler",
        name: "Mojo Neural",
        impl_id: crate::bridge::mojo::MOJO_NEURAL_IMPL_ID,
    },
    DefaultVariantSeed {
        id: crate::bridge::mlc_zero_v::MLC_ZERO_V_IMPL_ID,
        node_id: "node-amp-modeler",
        category_id: "amp-modeler",
        node_name: "Amp Modeler",
        name: "MLC Zero V",
        impl_id: crate::bridge::mlc_zero_v::MLC_ZERO_V_IMPL_ID,
    },
];

fn seed_registered_variants(db: &Database, registry: &VariantRegistry) -> Result<()> {
    for seed in DEFAULT_VARIANT_SEEDS {
        if registry.get(seed.impl_id).is_some() {
            db.upsert_node(seed.node_id, seed.category_id, seed.node_name)?;
            db.upsert_variant(seed.id, seed.node_id, seed.name, seed.impl_id)?;
        }
    }
    Ok(())
}

/// Return default variant ids for a category in UI order.
pub fn default_variant_ids_for_category(category_id: &str) -> Vec<&'static str> {
    DEFAULT_VARIANT_SEEDS
        .iter()
        .filter(|seed| seed.category_id == category_id)
        .map(|seed| seed.id)
        .collect()
}

fn register_default_variants(registry: &mut VariantRegistry) {
    registry.register(
        crate::bridge::faust::FAUST_EQ_IMPL_ID,
        crate::bridge::faust::faust_eq_factory,
    );
    registry.register(
        crate::bridge::mojo::MOJO_NEURAL_IMPL_ID,
        crate::bridge::mojo::mojo_neural_factory,
    );
    registry.register(
        crate::bridge::mlc_zero_v::MLC_ZERO_V_IMPL_ID,
        crate::bridge::mlc_zero_v::mlc_zero_v_factory,
    );
}

/// Return the Phase 1 default category set in pipeline order.
pub fn default_categories() -> Vec<Category> {
    [
        (
            "input-routing",
            "Input Routing",
            "Input selection and gain staging",
        ),
        ("eq", "EQ", "Equalization"),
        (
            "pre-eq-conv",
            "Pre-EQ Convolution",
            "Pre-amplifier convolution",
        ),
        (
            "amp-modeler",
            "Amp Modeler",
            "Algorithmic or neural amp model",
        ),
        ("amp-capture", "Amp Capture", "Captured amp model"),
        ("cab-sim", "Cab Sim", "Cabinet simulation"),
        ("ir-loader", "IR Loader", "Impulse response loader"),
        ("output-stage", "Output Stage", "Output trim and routing"),
    ]
    .into_iter()
    .enumerate()
    .map(|(index, (id, name, description))| Category {
        id: id.to_string(),
        name: name.to_string(),
        description: Some(description.to_string()),
        sort_order: index as i64,
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::{default_categories, Lab};
    use crate::core::ui::lab_panel::{minimal_snapshot, LabUiState};
    use std::path::PathBuf;
    use uuid::Uuid;

    fn temp_lab_dir(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("distortion-lab-{name}-{}", Uuid::new_v4()))
    }

    #[test]
    fn lab_init_seeds_default_categories_and_pipeline() {
        let dir = temp_lab_dir("init");
        let lab = Lab::init(dir.clone()).expect("init lab");
        let db = lab.db().lock().expect("db lock");
        let categories = db.list_categories().expect("categories");

        assert_eq!(categories, default_categories());
        assert_eq!(lab.pipeline().config().slots.len(), 8);
        assert_eq!(lab.data_dir(), dir.as_path());
        assert_eq!(lab.registry().len(), 3);
        assert_eq!(
            db.load_variant("faust-eq")
                .expect("load faust variant")
                .expect("faust variant exists")
                .impl_id,
            "faust-eq"
        );
        assert_eq!(
            db.load_variant("mojo-neural")
                .expect("load mojo variant")
                .expect("mojo variant exists")
                .node_id,
            "node-amp-modeler"
        );
        assert!(db
            .load_variant("mlc-zero-v")
            .expect("load mlc variant")
            .is_some());
        drop(db);

        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn lab_init_reuses_saved_pipeline() {
        let dir = temp_lab_dir("reuse");
        let first = Lab::init(dir.clone()).expect("first init");
        let first_slots = first.pipeline().config().slots.clone();
        drop(first);

        let second = Lab::init(dir.clone()).expect("second init");

        assert_eq!(second.pipeline().config().slots, first_slots);

        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn facade_save_and_load_snapshot_round_trips() {
        let dir = temp_lab_dir("facade-round-trip");
        let lab = Lab::init(dir.clone()).expect("init lab");
        let state = LabUiState {
            selected_category: Some("eq".to_string()),
            selected_variant_id: Some("faust-eq".to_string()),
            selected_snapshot_id: "snapshot-facade-1".to_string(),
            ..LabUiState::default()
        };
        let full = minimal_snapshot(&state);

        lab.save_snapshot(&full).expect("save snapshot");
        let loaded = lab.load_snapshot(&full.meta.id).expect("load snapshot");

        assert_eq!(full.meta.variant_id, "faust-eq");
        assert_eq!(loaded, full);

        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn facade_save_snapshot_creates_new_id_when_existing_snapshot_is_selected() {
        let dir = temp_lab_dir("facade-existing-selected");
        let lab = Lab::init(dir.clone()).expect("init lab");
        let first_state = LabUiState {
            selected_category: Some("eq".to_string()),
            selected_variant_id: Some("faust-eq".to_string()),
            selected_snapshot_id: "manual-input-is-not-used".to_string(),
            ..LabUiState::default()
        };
        let first = minimal_snapshot(&first_state);
        lab.save_snapshot(&first).expect("save first snapshot");

        let second_state = LabUiState {
            selected_category: Some("eq".to_string()),
            selected_variant_id: Some("faust-eq".to_string()),
            selected_snapshot_id: first.meta.id.clone(),
            ..LabUiState::default()
        };
        let second = minimal_snapshot(&second_state);
        lab.save_snapshot(&second).expect("save second snapshot");

        assert_ne!(second.meta.id, first.meta.id);
        assert!(lab.load_snapshot(&first.meta.id).is_ok());
        assert!(lab.load_snapshot(&second.meta.id).is_ok());

        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn facade_save_snapshot_does_not_overwrite_switched_pipeline() {
        let dir = temp_lab_dir("facade-pipeline");
        let lab = Lab::init(dir.clone()).expect("init lab");
        lab.switch_variant("eq", "faust-eq")
            .expect("switch variant");

        let before = lab
            .db()
            .lock()
            .expect("db lock")
            .load_pipeline("default")
            .expect("load pipeline")
            .expect("pipeline exists");
        assert_eq!(
            before
                .slots
                .iter()
                .find(|slot| slot.category_id == "eq")
                .and_then(|slot| slot.active_variant_id.as_deref()),
            Some("faust-eq")
        );

        let state = LabUiState {
            selected_category: Some("eq".to_string()),
            selected_variant_id: Some("faust-eq".to_string()),
            selected_snapshot_id: "snapshot-after-switch".to_string(),
            ..LabUiState::default()
        };
        lab.save_snapshot(&minimal_snapshot(&state))
            .expect("save snapshot");

        let after = lab
            .db()
            .lock()
            .expect("db lock")
            .load_pipeline("default")
            .expect("load pipeline")
            .expect("pipeline exists");
        assert_eq!(after, before);

        let _ = std::fs::remove_dir_all(dir);
    }
}
