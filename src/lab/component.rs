use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Metadata describing a component category stored in the lab database.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Category {
    /// Stable category identifier used by pipeline slots.
    pub id: String,
    /// Human-readable category name.
    pub name: String,
    /// Optional category description.
    pub description: Option<String>,
    /// Stable sort order for signal-chain presentation.
    pub sort_order: i64,
}

/// Metadata describing a registered DSP variant for a node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VariantMeta {
    /// Stable variant identifier.
    pub id: String,
    /// Node this variant belongs to.
    pub node_id: String,
    /// Human-readable variant name.
    pub name: String,
    /// Compiled implementation identifier used by the variant registry.
    pub impl_id: String,
    /// Variant lifecycle status.
    pub status: String,
    /// Currently active snapshot for this variant, when selected.
    pub active_snapshot_id: Option<String>,
    /// Creation timestamp as stored by SQLite.
    pub created_at: String,
}

/// Serializable pipeline configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PipelineConfig {
    /// Pipeline name.
    pub name: String,
    /// Pipeline config version.
    pub version: String,
    /// Ordered category slots.
    pub slots: Vec<SlotConfig>,
    /// Output trim applied after all slots.
    pub master_gain_db: f32,
}

impl PipelineConfig {
    /// Create a default pipeline config from category rows.
    pub fn from_categories(categories: &[Category]) -> Self {
        Self {
            name: "default".to_string(),
            version: "1.0".to_string(),
            slots: categories
                .iter()
                .map(|category| SlotConfig {
                    category_id: category.id.clone(),
                    active_variant_id: None,
                    bypassed: false,
                })
                .collect(),
            master_gain_db: 0.0,
        }
    }
}

/// Configuration for one pipeline category slot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SlotConfig {
    /// Category id represented by this slot.
    pub category_id: String,
    /// Active variant id for this category, when loaded.
    pub active_variant_id: Option<String>,
    /// Whether this slot is bypassed.
    pub bypassed: bool,
}

/// Runtime pipeline state owned by the lab facade.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pipeline {
    config: PipelineConfig,
}

impl Pipeline {
    /// Build a pipeline from category metadata.
    pub fn from_categories(categories: &[Category]) -> Self {
        Self {
            config: PipelineConfig::from_categories(categories),
        }
    }

    /// Build a pipeline from an existing serialized configuration.
    pub fn from_config(config: PipelineConfig) -> Self {
        Self { config }
    }

    /// Borrow the serializable pipeline configuration.
    pub fn config(&self) -> &PipelineConfig {
        &self.config
    }
}

/// DSP engine used by a component implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DspEngine {
    /// Faust-generated DSP.
    Faust,
    /// Mojo-backed DSP.
    Mojo,
    /// Native Rust DSP.
    PureRust,
}

/// DSP source and FFI metadata needed to rebuild a component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DspConfig {
    /// DSP engine family.
    pub engine: DspEngine,
    /// Source language name.
    pub language: String,
    /// Source files included in the component.
    pub source_files: Vec<SourceFile>,
    /// Faust libraries referenced by the source, when applicable.
    pub faust_libraries: Vec<FaustLib>,
    /// Build command used for this DSP.
    pub compile_command: String,
    /// FFI function and buffer metadata.
    pub ffi_interface: FfiInterface,
}

/// DSP source file reference.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceFile {
    /// Path to the source file.
    pub path: String,
    /// Content hash for integrity checks.
    pub hash: String,
    /// Human-readable source description.
    pub description: String,
}

/// Faust library dependency metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FaustLib {
    /// Library name.
    pub name: String,
    /// Library version or revision.
    pub version: String,
}

/// FFI function names and parameter layout for a DSP implementation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FfiInterface {
    /// Initialization function name.
    pub init_fn: String,
    /// Process function name.
    pub process_fn: String,
    /// Cleanup function name.
    pub cleanup_fn: String,
    /// Number of exported parameters.
    pub param_count: u32,
    /// Stable parameter ids in exported order.
    pub param_ids: Vec<String>,
    /// Audio buffer ABI description.
    pub buffer_format: String,
}

/// Integration guidance emitted in AI-readable snapshot exports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntegrationGuide {
    /// Short implementation summary.
    pub summary: String,
    /// Target host framework.
    pub host_framework: String,
    /// Target platforms for the component.
    pub target_platforms: Vec<String>,
    /// Ordered integration steps.
    pub steps: Vec<String>,
    /// Rust example showing host-side integration.
    pub rust_integration_example: String,
}

/// External dependencies needed by a component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencyList {
    /// Rust crate dependencies.
    pub rust_crates: Vec<CrateDep>,
    /// Required system tools.
    pub system_tools: Vec<SystemTool>,
}

/// Rust crate dependency metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrateDep {
    /// Crate name.
    pub name: String,
    /// Version requirement.
    pub version: String,
    /// Enabled cargo features.
    pub features: Vec<String>,
    /// Whether this is only required at build time.
    pub build_only: bool,
}

/// System tool dependency metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemTool {
    /// Tool name.
    pub name: String,
    /// Minimum required version.
    pub version_min: String,
    /// Whether the tool is required or optional.
    pub required: bool,
}

/// Audio routing metadata for a component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoutingConfig {
    /// Input routing description.
    pub audio_input: String,
    /// Output routing description.
    pub audio_output: String,
    /// Reported DSP latency in samples.
    pub latency_samples: u32,
}

/// Verification check definition stored with a snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerificationCheckDef {
    /// Stable check identifier.
    pub id: String,
    /// Display and execution order.
    pub check_order: i64,
    /// Check category.
    pub category: CheckCategory,
    /// Check description.
    pub check: String,
    /// Expected result.
    pub expected: String,
    /// Whether the check can run automatically.
    pub automated: bool,
}

/// Verification check category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CheckCategory {
    /// Build and compilation checks.
    Build,
    /// Audio behavior checks.
    Audio,
    /// Parameter layout checks.
    Params,
    /// User-interface checks.
    Ui,
    /// Dependency integrity checks.
    Deps,
}

impl CheckCategory {
    /// Return the stable lowercase database representation.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Build => "build",
            Self::Audio => "audio",
            Self::Params => "params",
            Self::Ui => "ui",
            Self::Deps => "deps",
        }
    }
}

impl fmt::Display for CheckCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for CheckCategory {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "build" | "Build" => Ok(Self::Build),
            "audio" | "Audio" => Ok(Self::Audio),
            "params" | "Params" => Ok(Self::Params),
            "ui" | "Ui" => Ok(Self::Ui),
            "deps" | "Deps" => Ok(Self::Deps),
            other => Err(format!("unknown check category '{other}'")),
        }
    }
}

/// Result of executing one verification check.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckResult {
    /// Check identifier this result belongs to.
    pub check_id: String,
    /// Check execution status.
    pub status: CheckStatus,
    /// Optional note with details from the result.
    pub result_note: Option<String>,
    /// Execution duration in milliseconds.
    pub duration_ms: u64,
}

/// Verification result status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CheckStatus {
    /// Check passed.
    Pass,
    /// Check failed.
    Fail,
    /// Check has not run yet.
    Pending,
}

impl CheckStatus {
    /// Return the stable lowercase database representation.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pass => "pass",
            Self::Fail => "fail",
            Self::Pending => "pending",
        }
    }
}

impl fmt::Display for CheckStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for CheckStatus {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "pass" | "Pass" => Ok(Self::Pass),
            "fail" | "Fail" => Ok(Self::Fail),
            "pending" | "Pending" => Ok(Self::Pending),
            other => Err(format!("unknown check status '{other}'")),
        }
    }
}

/// Trait implemented by compiled DSP variants that can run inside a lab slot.
pub trait DspVariant: Send {
    /// Process an in-place mono float buffer.
    fn process_block(&mut self, buffer: *mut f32, length: usize);

    /// Return the number of exported parameters.
    fn param_count(&self) -> usize;

    /// Return the stable parameter identifiers.
    fn param_ids(&self) -> &[&str];

    /// Return latency in samples.
    fn latency(&self) -> usize;
}

#[cfg(test)]
mod tests {
    use super::{Category, PipelineConfig};

    #[test]
    fn pipeline_config_uses_category_order() {
        let categories = vec![
            Category {
                id: "eq".to_string(),
                name: "EQ".to_string(),
                description: None,
                sort_order: 1,
            },
            Category {
                id: "cab-sim".to_string(),
                name: "Cab Sim".to_string(),
                description: None,
                sort_order: 2,
            },
        ];

        let config = PipelineConfig::from_categories(&categories);

        assert_eq!(config.slots.len(), 2);
        assert_eq!(config.slots[0].category_id, "eq");
        assert_eq!(config.slots[1].category_id, "cab-sim");
    }
}
