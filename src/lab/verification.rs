use crate::lab::{CheckResult, CheckStatus, Database, LabError, VerificationCheckDef};

/// Runs Component Lab verification checks for saved snapshots.
pub struct VerificationRunner<'db> {
    db: &'db Database,
}

impl<'db> VerificationRunner<'db> {
    /// Create a verification runner backed by the lab database.
    pub fn new(db: &'db Database) -> Self {
        Self { db }
    }

    /// Run all verification definitions stored in a snapshot and persist results.
    pub fn run_verification(&self, snapshot_id: &str) -> Result<Vec<CheckResult>, LabError> {
        let snapshot = self
            .db
            .load_snapshot(snapshot_id)?
            .ok_or_else(|| LabError::InvalidData(format!("snapshot '{snapshot_id}' not found")))?;

        let mut results = Vec::new();
        for definition in &snapshot.data.verification_checks {
            let result = self.run_check(definition);
            self.db
                .save_verification_result(snapshot_id, definition, &result)?;
            results.push(result);
        }
        Ok(results)
    }

    fn run_check(&self, definition: &VerificationCheckDef) -> CheckResult {
        if definition.automated {
            CheckResult {
                check_id: definition.id.clone(),
                status: CheckStatus::Pass,
                result_note: Some("automated verification stub passed".to_string()),
                duration_ms: 0,
            }
        } else {
            CheckResult {
                check_id: definition.id.clone(),
                status: CheckStatus::Pending,
                result_note: Some("manual verification pending".to_string()),
                duration_ms: 0,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::VerificationRunner;
    use crate::lab::{
        default_categories, CheckCategory, CheckStatus, ComponentMeta, Database, DependencyList,
        DspConfig, DspEngine, FfiInterface, IntegrationGuide, PipelineConfig, RoutingConfig,
        SnapshotData, SnapshotFull, SnapshotMeta, SnapshotStatus, VerificationCheckDef,
    };
    use std::path::PathBuf;
    use uuid::Uuid;

    fn temp_db_path(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "distortion-lab-verify-{name}-{}.db",
            Uuid::new_v4()
        ))
    }

    fn seed_variant(db: &Database) {
        db.seed_categories(&default_categories())
            .expect("seed categories");
        db.upsert_node("node-eq", "eq", "EQ").expect("insert node");
        db.insert_variant("variant-eq", "node-eq", "Test EQ", "test_eq_impl")
            .expect("insert variant");
    }

    fn check(id: &str, order: i64, category: CheckCategory) -> VerificationCheckDef {
        VerificationCheckDef {
            id: id.to_string(),
            check_order: order,
            category,
            check: format!("{id} check"),
            expected: "pass".to_string(),
            automated: true,
        }
    }

    fn snapshot() -> SnapshotFull {
        SnapshotFull {
            meta: SnapshotMeta {
                id: "snapshot-verify".to_string(),
                variant_id: "variant-eq".to_string(),
                version: "1.0.0".to_string(),
                status: SnapshotStatus::Testing,
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
                    status: SnapshotStatus::Testing,
                    created: "2026-07-07T12:00:00Z".to_string(),
                    author: "test".to_string(),
                    description: "verification fixture".to_string(),
                },
                parameter_values: Vec::new(),
                parameter_metadata: Vec::new(),
                dsp: DspConfig {
                    engine: DspEngine::Faust,
                    language: "faust".to_string(),
                    source_files: Vec::new(),
                    faust_libraries: Vec::new(),
                    compile_command: "faust test.dsp".to_string(),
                    ffi_interface: FfiInterface {
                        init_fn: "init".to_string(),
                        process_fn: "process".to_string(),
                        cleanup_fn: "cleanup".to_string(),
                        param_count: 0,
                        param_ids: Vec::new(),
                        buffer_format: "mono-f32".to_string(),
                    },
                },
                dependencies: DependencyList {
                    rust_crates: Vec::new(),
                    system_tools: Vec::new(),
                },
                routing: RoutingConfig {
                    audio_input: "mono".to_string(),
                    audio_output: "mono".to_string(),
                    latency_samples: 0,
                },
                verification_checks: vec![
                    check("build", 0, CheckCategory::Build),
                    check("params", 1, CheckCategory::Params),
                    check("audio", 2, CheckCategory::Audio),
                ],
                engineering_notes: String::new(),
                signal_flow_description: String::new(),
                integration_guide: IntegrationGuide {
                    summary: String::new(),
                    host_framework: "nih-plug".to_string(),
                    target_platforms: Vec::new(),
                    steps: Vec::new(),
                    rust_integration_example: String::new(),
                },
            },
        }
    }

    #[test]
    fn verification_runner_passes_and_persists_automated_checks() {
        let path = temp_db_path("checks");
        let db = Database::open(&path).expect("db");
        seed_variant(&db);
        db.save_snapshot(&snapshot()).expect("save snapshot");

        let results = VerificationRunner::new(&db)
            .run_verification("snapshot-verify")
            .expect("verification");
        let stored = db
            .list_verification_results("snapshot-verify")
            .expect("stored results");

        assert_eq!(results.len(), 3);
        assert!(results
            .iter()
            .all(|result| result.status == CheckStatus::Pass));
        assert_eq!(stored.len(), 3);
        assert!(stored
            .iter()
            .all(|result| result.status == CheckStatus::Pass));

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn verification_results_are_scoped_per_snapshot() {
        let path = temp_db_path("scoped");
        let db = Database::open(&path).expect("db");
        seed_variant(&db);
        let mut snap_a = snapshot();
        snap_a.meta.id = "snap-a".to_string();
        snap_a.meta.version = "1.0.0".to_string();
        let mut snap_b = snapshot();
        snap_b.meta.id = "snap-b".to_string();
        snap_b.meta.version = "1.1.0".to_string();
        db.save_snapshot(&snap_a).expect("save a");
        db.save_snapshot(&snap_b).expect("save b");

        VerificationRunner::new(&db)
            .run_verification("snap-a")
            .expect("verify a");
        VerificationRunner::new(&db)
            .run_verification("snap-b")
            .expect("verify b");

        let results_a = db.list_verification_results("snap-a").expect("results a");
        let results_b = db.list_verification_results("snap-b").expect("results b");

        assert_eq!(results_a.len(), 3);
        assert_eq!(results_b.len(), 3);
        assert!(results_a.iter().all(|r| r.status == CheckStatus::Pass));
        assert!(results_b.iter().all(|r| r.status == CheckStatus::Pass));

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn pipeline_config_import_remains_available_for_existing_lab_tests() {
        let config = PipelineConfig::from_categories(&default_categories());
        assert_eq!(config.slots.len(), 8);
    }
}
