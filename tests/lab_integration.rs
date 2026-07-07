use distortion::lab::{
    default_categories, Category, ComponentMeta, CrateDep, Database, DependencyList, DspConfig,
    DspEngine, DspVariant, FaustLib, FfiInterface, IntegrationGuide, Lab, PipelineManager,
    RoutingConfig, SnapshotData, SnapshotFull, SnapshotMeta, SnapshotStatus, SourceFile,
    SystemTool, VerificationCheckDef,
};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

struct GainVariant {
    gain: f32,
}

impl DspVariant for GainVariant {
    fn process_block(&mut self, buffer: *mut f32, length: usize) {
        let samples = unsafe { std::slice::from_raw_parts_mut(buffer, length) };
        for sample in samples {
            *sample *= self.gain;
        }
    }

    fn param_count(&self) -> usize {
        1
    }

    fn param_ids(&self) -> &[&str] {
        &["gain"]
    }

    fn latency(&self) -> usize {
        0
    }
}

fn double_factory(_sample_rate: f32) -> Box<dyn DspVariant> {
    Box::new(GainVariant { gain: 2.0 })
}

fn triple_factory(_sample_rate: f32) -> Box<dyn DspVariant> {
    Box::new(GainVariant { gain: 3.0 })
}

fn temp_db_path(name: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    std::env::temp_dir().join(format!("distortion-lab-integration-{name}-{nonce}.db"))
}

fn temp_lab_dir(name: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    std::env::temp_dir().join(format!("distortion-lab-integration-{name}-{nonce}"))
}

fn category(id: &str, order: i64) -> Category {
    Category {
        id: id.to_string(),
        name: id.to_string(),
        description: None,
        sort_order: order,
    }
}

fn seed_snapshot_variant(db: &Database) {
    db.seed_categories(&default_categories())
        .expect("categories");
    db.upsert_node("node-eq", "eq", "EQ").expect("node");
    db.insert_variant("variant-eq", "node-eq", "EQ", "faust-eq")
        .expect("variant");
}

fn snapshot(value: f32) -> SnapshotFull {
    SnapshotFull {
        meta: SnapshotMeta {
            id: "snapshot-eq-1".to_string(),
            variant_id: "variant-eq".to_string(),
            version: "1.0.0".to_string(),
            status: SnapshotStatus::Draft,
            variant_impl_id: "faust-eq".to_string(),
            params_hash: format!("hash-{value}"),
            created_at: "2026-07-07T00:00:00Z".to_string(),
            notes: "round trip".to_string(),
        },
        data: SnapshotData {
            format: SnapshotData::FORMAT.to_string(),
            schema_version: SnapshotData::SCHEMA_VERSION.to_string(),
            component: ComponentMeta {
                name: "EQ".to_string(),
                category: "eq".to_string(),
                variant_id: "variant-eq".to_string(),
                version: "1.0.0".to_string(),
                status: SnapshotStatus::Draft,
                created: "2026-07-07T00:00:00Z".to_string(),
                author: "integration-test".to_string(),
                description: "integration snapshot".to_string(),
            },
            parameter_values: vec![distortion::lab::ParamValue {
                id: "gain".to_string(),
                value,
            }],
            parameter_metadata: Vec::new(),
            dsp: DspConfig {
                engine: DspEngine::PureRust,
                language: "rust".to_string(),
                source_files: vec![SourceFile {
                    path: "src/lib.rs".to_string(),
                    hash: "hash".to_string(),
                    description: "source".to_string(),
                }],
                faust_libraries: Vec::<FaustLib>::new(),
                compile_command: "cargo build".to_string(),
                ffi_interface: FfiInterface {
                    init_fn: "init".to_string(),
                    process_fn: "process".to_string(),
                    cleanup_fn: "cleanup".to_string(),
                    param_count: 1,
                    param_ids: vec!["gain".to_string()],
                    buffer_format: "mono f32".to_string(),
                },
            },
            dependencies: DependencyList {
                rust_crates: vec![CrateDep {
                    name: "distortion".to_string(),
                    version: "0.1".to_string(),
                    features: Vec::new(),
                    build_only: false,
                }],
                system_tools: Vec::<SystemTool>::new(),
            },
            routing: RoutingConfig {
                audio_input: "mono".to_string(),
                audio_output: "mono".to_string(),
                latency_samples: 0,
            },
            verification_checks: Vec::<VerificationCheckDef>::new(),
            engineering_notes: "notes".to_string(),
            signal_flow_description: "gain".to_string(),
            integration_guide: IntegrationGuide {
                summary: "summary".to_string(),
                host_framework: "nih-plug".to_string(),
                target_platforms: vec!["linux".to_string()],
                steps: vec!["load".to_string()],
                rust_integration_example: "variant.process_block(ptr, len);".to_string(),
            },
        },
    }
}

#[test]
fn snapshot_round_trip_survives_database_restart() {
    let path = temp_db_path("roundtrip");
    let first = Database::open(&path).expect("open first");
    seed_snapshot_variant(&first);
    first.save_snapshot(&snapshot(0.75)).expect("save snapshot");
    drop(first);

    let second = Database::open(&path).expect("open second");
    let loaded = second
        .load_snapshot("snapshot-eq-1")
        .expect("load snapshot")
        .expect("snapshot exists");

    assert_eq!(loaded.data.parameter_values[0].id, "gain");
    assert_eq!(loaded.data.parameter_values[0].value, 0.75);
    assert_eq!(loaded.meta.variant_impl_id, "faust-eq");

    let _ = std::fs::remove_file(path);
}

#[test]
fn variant_switch_changes_processed_output() {
    let mut manager = PipelineManager::from_categories(&[category("eq", 1)]).expect("pipeline");

    manager
        .request_switch("eq", "double", double_factory, 48_000.0)
        .expect("switch double");
    let mut first = [1.0, 2.0];
    manager.process_block(first.as_mut_ptr(), first.len());

    manager
        .request_switch("eq", "triple", triple_factory, 48_000.0)
        .expect("switch triple");
    let mut second = [1.0, 2.0];
    manager.process_block(second.as_mut_ptr(), second.len());

    assert_eq!(first, [2.0, 4.0]);
    assert_eq!(second, [3.0, 6.0]);
}

#[test]
fn pipeline_bypass_leaves_audio_unchanged() {
    let mut manager = PipelineManager::from_categories(&[category("eq", 1)]).expect("pipeline");
    manager
        .request_switch("eq", "double", double_factory, 48_000.0)
        .expect("switch");
    manager.set_bypass("eq", true).expect("bypass");
    let mut buffer = [1.0, -0.5];

    manager.process_block(buffer.as_mut_ptr(), buffer.len());

    assert_eq!(buffer, [1.0, -0.5]);
}

#[test]
fn amp_exclusivity_is_enforced() {
    let mut manager =
        PipelineManager::from_categories(&[category("amp-modeler", 1), category("amp-capture", 2)])
            .expect("pipeline");

    manager
        .request_switch("amp-modeler", "modeler", double_factory, 48_000.0)
        .expect("switch modeler");
    let err = manager
        .request_switch("amp-capture", "capture", triple_factory, 48_000.0)
        .expect_err("exclusive amps");

    assert!(err.to_string().contains("cannot both be active"));
}

#[test]
fn lab_load_snapshot_applies_snapshot_variant_to_pipeline_config() {
    let dir = temp_lab_dir("load-applies-variant");
    let lab = Lab::init(dir.clone()).expect("init lab");
    let mut full = snapshot(0.5);
    full.meta.id = "snapshot-mojo-neural".to_string();
    full.meta.variant_id = "mojo-neural".to_string();
    full.meta.variant_impl_id = "mojo-neural".to_string();
    full.meta.version = "1.0.1".to_string();
    full.data.component.name = "Amp Modeler".to_string();
    full.data.component.category = "amp-modeler".to_string();
    full.data.component.variant_id = "mojo-neural".to_string();
    full.data.component.version = full.meta.version.clone();

    lab.save_snapshot(&full).expect("save snapshot");
    lab.switch_variant("amp-modeler", "mlc-zero-v")
        .expect("switch away from snapshot variant");

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
            .find(|slot| slot.category_id == "amp-modeler")
            .and_then(|slot| slot.active_variant_id.as_deref()),
        Some("mlc-zero-v")
    );

    let loaded = lab.load_snapshot(&full.meta.id).expect("load snapshot");
    assert_eq!(loaded.meta.id, full.meta.id);

    let after = lab
        .db()
        .lock()
        .expect("db lock")
        .load_pipeline("default")
        .expect("load pipeline")
        .expect("pipeline exists");
    assert_eq!(
        after
            .slots
            .iter()
            .find(|slot| slot.category_id == "amp-modeler")
            .and_then(|slot| slot.active_variant_id.as_deref()),
        Some("mojo-neural")
    );

    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn test_snapshot_captures_parameter_values() {
    let dir = temp_lab_dir("snapshot-captures-params");
    let lab = Lab::init(dir.clone()).expect("init lab");

    // Activate the Faust EQ variant so its runtime parameters can be read back.
    lab.switch_variant("eq", "faust-eq").expect("switch variant");

    let mut full = snapshot(0.0);
    full.meta.id = "snapshot-capture-eq".to_string();
    full.meta.variant_id = "faust-eq".to_string();
    full.meta.variant_impl_id = "faust-eq".to_string();
    full.data.component.category = "eq".to_string();
    full.data.component.variant_id = "faust-eq".to_string();
    // Start with no stored values: save_snapshot must populate them from runtime.
    full.data.parameter_values.clear();
    full.data.parameter_metadata.clear();

    lab.save_snapshot(&full).expect("save snapshot");

    let loaded = lab.load_snapshot(&full.meta.id).expect("load snapshot");
    assert_eq!(loaded.data.parameter_values.len(), 9);
    assert_eq!(loaded.data.parameter_metadata.len(), 9);

    let low_freq = loaded
        .data
        .parameter_values
        .iter()
        .find(|param| param.id == "eq_low_freq")
        .expect("eq_low_freq captured");
    assert_eq!(low_freq.value, 100.0);

    let _ = std::fs::remove_dir_all(dir);
}

#[test]
fn test_load_snapshot_applies_parameter_values() {
    let dir = temp_lab_dir("load-applies-params");
    let lab = Lab::init(dir.clone()).expect("init lab");

    // A snapshot carrying an explicit, non-default parameter value. The eq slot is
    // left inactive so save_snapshot's capture pass does not overwrite the value.
    let mut full = snapshot(0.0);
    full.meta.id = "snapshot-eq-params".to_string();
    full.meta.variant_id = "faust-eq".to_string();
    full.meta.variant_impl_id = "faust-eq".to_string();
    full.data.component.category = "eq".to_string();
    full.data.component.variant_id = "faust-eq".to_string();
    full.data.parameter_values = vec![distortion::lab::ParamValue {
        id: "eq_low_gain".to_string(),
        value: 6.0,
    }];
    full.data.parameter_metadata.clear();

    lab.save_snapshot(&full).expect("save snapshot");

    // Loading applies the saved variant and parameter values without error and
    // preserves the stored value in the returned snapshot.
    let loaded = lab.load_snapshot(&full.meta.id).expect("load snapshot");
    assert_eq!(loaded.data.parameter_values.len(), 1);
    assert_eq!(loaded.data.parameter_values[0].id, "eq_low_gain");
    assert_eq!(loaded.data.parameter_values[0].value, 6.0);

    // Verify the apply mechanism used by load_snapshot mutates the live variant.
    let (categories, config) = {
        let db = lab.db().lock().expect("db lock");
        let categories = db.list_categories().expect("categories");
        let config = db
            .load_pipeline("default")
            .expect("load pipeline")
            .expect("pipeline exists");
        (categories, config)
    };
    let mut manager = PipelineManager::from_config(&categories, config).expect("manager");
    let factory = lab.registry().get("faust-eq").expect("faust factory");
    manager
        .request_switch("eq", "faust-eq", factory, 48_000.0)
        .expect("switch");
    manager
        .set_node_param("eq", "eq_low_gain", 6.0)
        .expect("set param");
    let node = manager.get_node("eq").expect("eq node");
    assert_eq!(node.get_param("eq_low_gain"), Some(6.0));

    let _ = std::fs::remove_dir_all(dir);
}
