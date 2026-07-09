use crate::lab::{
    DependencyList, DspConfig, IntegrationGuide, RoutingConfig, VerificationCheckDef,
};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Saved value for one DSP parameter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParamValue {
    /// Stable parameter identifier.
    pub id: String,
    /// Saved normalized or native parameter value.
    pub value: f32,
}

/// Parameter metadata included with an exported component snapshot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParameterMeta {
    /// Stable parameter identifier.
    pub id: String,
    /// Display name.
    pub name: String,
    /// Human-readable parameter description.
    pub description: String,
    /// Inclusive parameter range.
    pub range: (f32, f32),
    /// Default parameter value.
    pub default: f32,
    /// Optional display unit.
    pub unit: Option<String>,
    /// Smoothing behavior description.
    pub smoothing: String,
    /// Stable parameter index.
    pub index: u32,
    /// Which backend produced this parameter (`"mojo"` | `"rust"`), when applicable.
    /// In-memory only — never persisted in snapshots or the lab database.
    #[serde(skip, default)]
    pub backend: Option<&'static str>,
}

/// Database metadata for a saved snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotMeta {
    /// Stable snapshot identifier.
    pub id: String,
    /// Variant this snapshot belongs to.
    pub variant_id: String,
    /// Semantic version string.
    pub version: String,
    /// Snapshot lifecycle status.
    pub status: SnapshotStatus,
    /// Compiled variant implementation id.
    pub variant_impl_id: String,
    /// Hash of the saved parameter values.
    pub params_hash: String,
    /// Creation timestamp.
    pub created_at: String,
    /// Free-form snapshot notes.
    pub notes: String,
}

/// Snapshot lifecycle status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SnapshotStatus {
    /// Work-in-progress snapshot.
    Draft,
    /// Snapshot under verification.
    Testing,
    /// Snapshot approved for export.
    Ready,
    /// Snapshot retained for history but superseded.
    Deprecated,
}

impl SnapshotStatus {
    /// Return the stable lowercase database representation.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Testing => "testing",
            Self::Ready => "ready",
            Self::Deprecated => "deprecated",
        }
    }
}

impl fmt::Display for SnapshotStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for SnapshotStatus {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "draft" | "Draft" => Ok(Self::Draft),
            "testing" | "Testing" => Ok(Self::Testing),
            "ready" | "Ready" => Ok(Self::Ready),
            "deprecated" | "Deprecated" => Ok(Self::Deprecated),
            other => Err(format!("unknown snapshot status '{other}'")),
        }
    }
}

/// Full AI-readable snapshot payload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SnapshotData {
    /// Snapshot file format identifier.
    pub format: String,
    /// Snapshot schema version.
    pub schema_version: String,
    /// Component metadata.
    pub component: ComponentMeta,
    /// Saved parameter values.
    pub parameter_values: Vec<ParamValue>,
    /// Parameter definitions and export descriptions.
    pub parameter_metadata: Vec<ParameterMeta>,
    /// DSP source and FFI metadata.
    pub dsp: DspConfig,
    /// External dependency metadata.
    pub dependencies: DependencyList,
    /// Audio routing metadata.
    pub routing: RoutingConfig,
    /// Verification checks generated for this snapshot.
    pub verification_checks: Vec<VerificationCheckDef>,
    /// Engineering notes for downstream replication.
    pub engineering_notes: String,
    /// Human-readable signal-flow description.
    pub signal_flow_description: String,
    /// Structured integration guidance.
    pub integration_guide: IntegrationGuide,
}

impl SnapshotData {
    /// Canonical format identifier for Component Lab snapshots.
    pub const FORMAT: &'static str = "dsp-component-snapshot";

    /// Canonical schema version for Phase 1 snapshots.
    pub const SCHEMA_VERSION: &'static str = "1.0";
}

/// Full snapshot including database metadata and export data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SnapshotFull {
    /// Database metadata.
    pub meta: SnapshotMeta,
    /// AI-readable snapshot payload.
    pub data: SnapshotData,
}

/// Component metadata included in snapshot exports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComponentMeta {
    /// Component display name.
    pub name: String,
    /// Component category id.
    pub category: String,
    /// Variant id.
    pub variant_id: String,
    /// Component version.
    pub version: String,
    /// Component status.
    pub status: SnapshotStatus,
    /// Creation timestamp.
    pub created: String,
    /// Component author.
    pub author: String,
    /// Component description.
    pub description: String,
}

/// Difference between two snapshots.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SnapshotDiff {
    /// Parameter value changes.
    pub value_changes: Vec<ParamValueChange>,
    /// Variant implementation change, when the snapshots use different variants.
    pub variant_change: Option<VariantChange>,
}

/// Difference for one parameter value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParamValueChange {
    /// Parameter id.
    pub id: String,
    /// Previous value.
    pub before: f32,
    /// New value.
    pub after: f32,
}

/// Difference between variant implementations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VariantChange {
    /// Previous variant implementation id.
    pub before_impl_id: String,
    /// New variant implementation id.
    pub after_impl_id: String,
}

#[cfg(test)]
mod tests {
    use super::ParamValue;

    #[test]
    fn param_value_round_trips_json() {
        let value = ParamValue {
            id: "gain".to_string(),
            value: 0.75,
        };

        let json = serde_json::to_string(&value).expect("serialize");
        let decoded: ParamValue = serde_json::from_str(&json).expect("deserialize");

        assert_eq!(decoded, value);
    }
}
