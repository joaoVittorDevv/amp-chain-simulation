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

        let persisted_pipeline = db.load_pipeline("default")?;
        let pipeline = match persisted_pipeline {
            Some(config) => Pipeline::from_config(config),
            None => {
                let pipeline = Pipeline::from_categories(&categories);
                db.save_pipeline("default", pipeline.config())?;
                pipeline
            }
        };

        let mut registry = VariantRegistry::new();
        register_default_variants(&mut registry);

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

    /// Save a component snapshot.
    pub fn save_snapshot(&self) -> Result<()> {
        Err(LabError::NotImplemented)
    }

    /// Load a component snapshot.
    pub fn load_snapshot(&self) -> Result<()> {
        Err(LabError::NotImplemented)
    }

    /// Switch a node to another compiled variant.
    pub fn switch_variant(&self) -> Result<()> {
        Err(LabError::NotImplemented)
    }

    /// Export a component snapshot bundle.
    pub fn export_component(&self) -> Result<PathBuf> {
        Err(LabError::NotImplemented)
    }
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
    use super::{default_categories, Lab, LabError};
    use std::path::PathBuf;
    use uuid::Uuid;

    fn temp_lab_dir(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("distortion-lab-{name}-{}", Uuid::new_v4()))
    }

    #[test]
    fn lab_init_seeds_default_categories_and_pipeline() {
        let dir = temp_lab_dir("init");
        let lab = Lab::init(dir.clone()).expect("init lab");
        let categories = lab
            .db()
            .lock()
            .expect("db lock")
            .list_categories()
            .expect("categories");

        assert_eq!(categories, default_categories());
        assert_eq!(lab.pipeline().config().slots.len(), 8);
        assert_eq!(lab.data_dir(), dir.as_path());
        assert_eq!(lab.registry().len(), 3);

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
    fn facade_stubs_return_not_implemented() {
        let dir = temp_lab_dir("stubs");
        let lab = Lab::init(dir.clone()).expect("init lab");

        assert!(matches!(lab.save_snapshot(), Err(LabError::NotImplemented)));
        assert!(matches!(lab.load_snapshot(), Err(LabError::NotImplemented)));
        assert!(matches!(
            lab.switch_variant(),
            Err(LabError::NotImplemented)
        ));
        assert!(matches!(
            lab.export_component(),
            Err(LabError::NotImplemented)
        ));

        let _ = std::fs::remove_dir_all(dir);
    }
}
