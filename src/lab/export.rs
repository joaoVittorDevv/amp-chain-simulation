use crate::lab::{Database, LabError, SnapshotFull};
use flate2::write::GzEncoder;
use flate2::Compression;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use tar::Builder;

/// File entry in an exported component manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestEntry {
    /// Bundle-relative file path.
    pub path: String,
    /// Lowercase hexadecimal SHA256 digest.
    pub sha256: String,
    /// File size in bytes.
    pub bytes: u64,
}

/// Export manifest written as `MANIFEST.json`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportManifest {
    /// Snapshot id exported into this bundle.
    pub snapshot_id: String,
    /// Snapshot semantic version.
    pub version: String,
    /// Files included in the bundle, excluding `MANIFEST.json`.
    pub files: Vec<ManifestEntry>,
}

/// Creates source bundles from saved component snapshots.
pub struct ExportEngine<'db> {
    db: &'db Database,
}

impl<'db> ExportEngine<'db> {
    /// Create an export engine backed by the lab database.
    pub fn new(db: &'db Database) -> Self {
        Self { db }
    }

    /// Export a snapshot into `output_dir`, returning the generated `.tar.gz`.
    pub fn export_snapshot(
        &self,
        snapshot_id: &str,
        output_dir: &Path,
    ) -> Result<PathBuf, LabError> {
        let snapshot = self
            .db
            .load_snapshot(snapshot_id)?
            .ok_or_else(|| LabError::InvalidData(format!("snapshot '{snapshot_id}' not found")))?;

        fs::create_dir_all(output_dir)?;
        let bundle_dir = output_dir.join(format!("component-{}", snapshot.meta.id));
        if bundle_dir.exists() {
            fs::remove_dir_all(&bundle_dir)?;
        }
        fs::create_dir_all(&bundle_dir)?;

        self.write_bundle_files(&snapshot, &bundle_dir)?;
        let manifest = self.build_manifest(&snapshot, &bundle_dir)?;
        let manifest_path = bundle_dir.join("MANIFEST.json");
        fs::write(&manifest_path, serde_json::to_vec_pretty(&manifest)?)?;

        let archive_path = output_dir.join(format!("component-{}.tar.gz", snapshot.meta.id));
        if archive_path.exists() {
            fs::remove_file(&archive_path)?;
        }
        self.create_archive(&bundle_dir, &archive_path)?;
        Ok(archive_path)
    }

    fn write_bundle_files(
        &self,
        snapshot: &SnapshotFull,
        bundle_dir: &Path,
    ) -> Result<(), LabError> {
        fs::write(
            bundle_dir.join("variant.json"),
            serde_json::to_vec_pretty(&snapshot.data)?,
        )?;

        let dsp_dir = bundle_dir.join("dsp");
        fs::create_dir_all(&dsp_dir)?;
        for source in &snapshot.data.dsp.source_files {
            let source_path = Path::new(&source.path);
            if !source_path.exists() {
                return Err(LabError::InvalidData(format!(
                    "DSP source '{}' does not exist",
                    source.path
                )));
            }
            let file_name = source_path.file_name().ok_or_else(|| {
                LabError::InvalidData(format!("DSP source '{}' has no file name", source.path))
            })?;
            fs::copy(source_path, dsp_dir.join(file_name))?;
        }

        let templates_dir = bundle_dir.join("templates");
        fs::create_dir_all(&templates_dir)?;
        fs::write(
            templates_dir.join("Cargo.toml.template"),
            cargo_template(snapshot),
        )?;
        fs::write(
            templates_dir.join("build.rs.template"),
            build_template(snapshot),
        )?;
        Ok(())
    }

    fn build_manifest(
        &self,
        snapshot: &SnapshotFull,
        bundle_dir: &Path,
    ) -> Result<ExportManifest, LabError> {
        let mut files = Vec::new();
        collect_manifest_entries(bundle_dir, bundle_dir, &mut files)?;
        files.sort_by(|left, right| left.path.cmp(&right.path));
        Ok(ExportManifest {
            snapshot_id: snapshot.meta.id.clone(),
            version: snapshot.meta.version.clone(),
            files,
        })
    }

    fn create_archive(&self, bundle_dir: &Path, archive_path: &Path) -> Result<(), LabError> {
        let archive_file = fs::File::create(archive_path)?;
        let encoder = GzEncoder::new(archive_file, Compression::default());
        let mut archive = Builder::new(encoder);
        let root_name = bundle_dir
            .file_name()
            .ok_or_else(|| LabError::InvalidData("bundle directory has no name".to_string()))?;
        archive.append_dir_all(root_name, bundle_dir)?;
        archive.finish()?;
        let mut encoder = archive.into_inner()?;
        encoder.flush()?;
        encoder.finish()?;
        Ok(())
    }
}

fn collect_manifest_entries(
    root: &Path,
    current: &Path,
    entries: &mut Vec<ManifestEntry>,
) -> Result<(), LabError> {
    for entry in fs::read_dir(current)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_manifest_entries(root, &path, entries)?;
            continue;
        }
        if path.file_name().and_then(|name| name.to_str()) == Some("MANIFEST.json") {
            continue;
        }

        let rel = path
            .strip_prefix(root)
            .map_err(|err| LabError::InvalidData(err.to_string()))?;
        let bytes = fs::read(&path)?;
        let digest = Sha256::digest(&bytes);
        entries.push(ManifestEntry {
            path: rel.to_string_lossy().replace('\\', "/"),
            sha256: format!("{digest:x}"),
            bytes: bytes.len() as u64,
        });
    }
    Ok(())
}

fn cargo_template(snapshot: &SnapshotFull) -> String {
    format!(
        "[package]\nname = \"{}\"\nversion = \"{}\"\nedition = \"2021\"\n\n[dependencies]\n",
        snapshot
            .data
            .component
            .name
            .replace(' ', "-")
            .to_lowercase(),
        snapshot.meta.version
    )
}

fn build_template(snapshot: &SnapshotFull) -> String {
    format!(
        "fn main() {{\n    // Build template for {} ({})\n}}\n",
        snapshot.data.component.name, snapshot.meta.variant_impl_id
    )
}

#[cfg(test)]
mod tests {
    use super::{ExportEngine, ExportManifest};
    use crate::lab::{
        default_categories, CheckCategory, ComponentMeta, CrateDep, Database, DependencyList,
        DspConfig, DspEngine, FaustLib, FfiInterface, IntegrationGuide, ParamValue, PipelineConfig,
        RoutingConfig, SnapshotData, SnapshotFull, SnapshotMeta, SnapshotStatus, SourceFile,
        VerificationCheckDef,
    };
    use flate2::read::GzDecoder;
    use std::path::{Path, PathBuf};
    use tar::Archive;
    use uuid::Uuid;

    fn temp_dir(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("distortion-lab-export-{name}-{}", Uuid::new_v4()))
    }

    fn seed_variant(db: &Database) {
        db.seed_categories(&default_categories())
            .expect("seed categories");
        db.upsert_node("node-eq", "eq", "EQ").expect("insert node");
        db.insert_variant("variant-eq", "node-eq", "Test EQ", "test_eq_impl")
            .expect("insert variant");
    }

    fn snapshot(source_path: &Path) -> SnapshotFull {
        SnapshotFull {
            meta: SnapshotMeta {
                id: "snapshot-export".to_string(),
                variant_id: "variant-eq".to_string(),
                version: "1.0.0".to_string(),
                status: SnapshotStatus::Ready,
                variant_impl_id: "test_eq_impl".to_string(),
                params_hash: "params".to_string(),
                created_at: "2026-07-07T12:00:00Z".to_string(),
                notes: String::new(),
            },
            data: SnapshotData {
                format: SnapshotData::FORMAT.to_string(),
                schema_version: SnapshotData::SCHEMA_VERSION.to_string(),
                component: ComponentMeta {
                    name: "Test EQ".to_string(),
                    category: "eq".to_string(),
                    variant_id: "variant-eq".to_string(),
                    version: "1.0.0".to_string(),
                    status: SnapshotStatus::Ready,
                    created: "2026-07-07T12:00:00Z".to_string(),
                    author: "test".to_string(),
                    description: "export fixture".to_string(),
                },
                parameter_values: vec![ParamValue {
                    id: "gain".to_string(),
                    value: 0.5,
                }],
                parameter_metadata: Vec::new(),
                dsp: DspConfig {
                    engine: DspEngine::Faust,
                    language: "faust".to_string(),
                    source_files: vec![SourceFile {
                        path: source_path.to_string_lossy().to_string(),
                        hash: "source-hash".to_string(),
                        description: "test source".to_string(),
                    }],
                    faust_libraries: vec![FaustLib {
                        name: "stdfaust.lib".to_string(),
                        version: "1.0".to_string(),
                    }],
                    compile_command: "faust test.dsp".to_string(),
                    ffi_interface: FfiInterface {
                        init_fn: "init".to_string(),
                        process_fn: "process".to_string(),
                        cleanup_fn: "cleanup".to_string(),
                        param_count: 1,
                        param_ids: vec!["gain".to_string()],
                        buffer_format: "mono-f32".to_string(),
                    },
                },
                dependencies: DependencyList {
                    rust_crates: vec![CrateDep {
                        name: "serde".to_string(),
                        version: "1".to_string(),
                        features: Vec::new(),
                        build_only: false,
                    }],
                    system_tools: Vec::new(),
                },
                routing: RoutingConfig {
                    audio_input: "mono".to_string(),
                    audio_output: "mono".to_string(),
                    latency_samples: 0,
                },
                verification_checks: vec![VerificationCheckDef {
                    id: "build".to_string(),
                    check_order: 0,
                    category: CheckCategory::Build,
                    check: "build".to_string(),
                    expected: "pass".to_string(),
                    automated: true,
                }],
                engineering_notes: "notes".to_string(),
                signal_flow_description: "flow".to_string(),
                integration_guide: IntegrationGuide {
                    summary: "summary".to_string(),
                    host_framework: "nih-plug".to_string(),
                    target_platforms: vec!["linux".to_string()],
                    steps: vec!["copy".to_string()],
                    rust_integration_example: "example".to_string(),
                },
            },
        }
    }

    #[test]
    fn export_produces_valid_tar_gz_with_expected_files() {
        let dir = temp_dir("bundle");
        std::fs::create_dir_all(&dir).expect("temp dir");
        let db_path = dir.join("lab.db");
        let source_path = dir.join("test.dsp");
        std::fs::write(&source_path, "process = _;").expect("source");
        let db = Database::open(&db_path).expect("db");
        seed_variant(&db);
        db.save_snapshot(&snapshot(&source_path))
            .expect("save snapshot");

        let archive_path = ExportEngine::new(&db)
            .export_snapshot("snapshot-export", &dir)
            .expect("export");

        assert!(archive_path.exists());

        let archive_file = std::fs::File::open(&archive_path).expect("archive");
        let decoder = GzDecoder::new(archive_file);
        let mut archive = Archive::new(decoder);
        let mut names = archive
            .entries()
            .expect("entries")
            .map(|entry| {
                entry
                    .expect("entry")
                    .path()
                    .expect("path")
                    .to_string_lossy()
                    .replace('\\', "/")
            })
            .collect::<Vec<_>>();
        names.sort();

        assert!(names.iter().any(|name| name.ends_with("MANIFEST.json")));
        assert!(names.iter().any(|name| name.ends_with("variant.json")));
        assert!(names.iter().any(|name| name.ends_with("dsp/test.dsp")));
        assert!(names
            .iter()
            .any(|name| name.ends_with("templates/Cargo.toml.template")));
        assert!(names
            .iter()
            .any(|name| name.ends_with("templates/build.rs.template")));

        let manifest_json =
            std::fs::read_to_string(dir.join("component-snapshot-export").join("MANIFEST.json"))
                .expect("manifest");
        let manifest: ExportManifest = serde_json::from_str(&manifest_json).expect("manifest json");
        assert!(manifest
            .files
            .iter()
            .any(|file| file.path == "variant.json"));
        assert!(manifest.files.iter().all(|file| file.sha256.len() == 64));

        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn pipeline_config_import_remains_available_for_lab_facade() {
        let config = PipelineConfig::from_categories(&default_categories());
        assert_eq!(config.slots.len(), 8);
    }
}
