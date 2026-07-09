use crate::lab::{
    Category, CheckResult, CheckStatus, LabError, PipelineConfig, SnapshotFull, SnapshotMeta,
    SnapshotStatus, VariantMeta, VerificationCheckDef,
};
use rusqlite::{params, Connection, OptionalExtension};
use semver::Version;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::Duration;

const SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS categories (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    sort_order INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS nodes (
    id TEXT PRIMARY KEY,
    category_id TEXT NOT NULL REFERENCES categories(id),
    name TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE(category_id)
);

CREATE TABLE IF NOT EXISTS variants (
    id TEXT PRIMARY KEY,
    node_id TEXT NOT NULL REFERENCES nodes(id),
    name TEXT NOT NULL,
    impl_id TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'draft',
    active_snapshot_id TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE(node_id, name)
);

CREATE TABLE IF NOT EXISTS snapshots (
    id TEXT PRIMARY KEY,
    variant_id TEXT NOT NULL REFERENCES variants(id),
    version TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'draft',
    variant_impl_id TEXT NOT NULL,
    values_json TEXT NOT NULL,
    data_json TEXT NOT NULL,
    params_hash TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    ready_at TEXT,
    notes TEXT DEFAULT '',
    UNIQUE(variant_id, version)
);

CREATE TABLE IF NOT EXISTS dependencies (
    snapshot_id TEXT NOT NULL REFERENCES snapshots(id) ON DELETE CASCADE,
    dep_type TEXT NOT NULL,
    dep_name TEXT NOT NULL,
    dep_version TEXT,
    dep_hash TEXT,
    PRIMARY KEY (snapshot_id, dep_type, dep_name)
);

CREATE TABLE IF NOT EXISTS pipelines (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    config_json TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS verification_checks (
    id TEXT NOT NULL,
    snapshot_id TEXT NOT NULL REFERENCES snapshots(id) ON DELETE CASCADE,
    check_order INTEGER NOT NULL,
    category TEXT NOT NULL,
    \"check\" TEXT NOT NULL,
    expected TEXT NOT NULL,
    automated INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'pending',
    result_note TEXT,
    checked_at TEXT,
    PRIMARY KEY (snapshot_id, id)
);

CREATE INDEX IF NOT EXISTS idx_snapshots_variant ON snapshots(variant_id);
CREATE INDEX IF NOT EXISTS idx_variants_node ON variants(node_id);
CREATE INDEX IF NOT EXISTS idx_checks_snapshot ON verification_checks(snapshot_id);
";

/// SQLite database connection for Component Lab persistence.
pub struct Database {
    conn: Connection,
}

impl Database {
    /// Return the default on-disk Component Lab database path.
    pub fn default_path() -> Result<PathBuf, LabError> {
        let config_dir = dirs::config_dir().ok_or(LabError::ConfigDirUnavailable)?;
        Ok(config_dir.join("distortion").join("lab.db"))
    }

    /// Open or create a Component Lab database and apply the Phase 1 schema.
    pub fn open(path: &Path) -> Result<Self, LabError> {
        if path != Path::new(":memory:") {
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
        }

        let conn = Connection::open(path)?;
        conn.busy_timeout(Duration::from_millis(200))?;
        conn.pragma_update(None, "foreign_keys", "ON")?;
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.execute_batch(SCHEMA)?;

        Ok(Self { conn })
    }

    /// Insert or update one category row.
    pub fn upsert_category(&self, category: &Category) -> Result<(), LabError> {
        self.conn.execute(
            "INSERT INTO categories (id, name, description, sort_order)
             VALUES (?1, ?2, ?3, ?4)
             ON CONFLICT(id) DO UPDATE SET
                name = excluded.name,
                description = excluded.description,
                sort_order = excluded.sort_order",
            params![
                category.id,
                category.name,
                category.description,
                category.sort_order
            ],
        )?;
        Ok(())
    }

    /// Seed the default category list idempotently.
    pub fn seed_categories(&self, categories: &[Category]) -> Result<(), LabError> {
        for category in categories {
            self.upsert_category(category)?;
        }
        Ok(())
    }

    /// List categories ordered by their configured sort order.
    pub fn list_categories(&self) -> Result<Vec<Category>, LabError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, description, sort_order
             FROM categories
             ORDER BY sort_order ASC, id ASC",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(Category {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                sort_order: row.get(3)?,
            })
        })?;

        let mut categories = Vec::new();
        for row in rows {
            categories.push(row?);
        }
        Ok(categories)
    }

    /// Insert or update one node row.
    pub fn upsert_node(&self, id: &str, category_id: &str, name: &str) -> Result<(), LabError> {
        self.conn.execute(
            "INSERT INTO nodes (id, category_id, name)
             VALUES (?1, ?2, ?3)
             ON CONFLICT(id) DO UPDATE SET
                category_id = excluded.category_id,
                name = excluded.name",
            params![id, category_id, name],
        )?;
        Ok(())
    }

    /// Insert a DSP variant row and return the stored metadata.
    pub fn insert_variant(
        &self,
        id: &str,
        node_id: &str,
        name: &str,
        impl_id: &str,
    ) -> Result<VariantMeta, LabError> {
        self.conn.execute(
            "INSERT INTO variants (id, node_id, name, impl_id)
             VALUES (?1, ?2, ?3, ?4)",
            params![id, node_id, name, impl_id],
        )?;
        self.load_variant(id)?
            .ok_or_else(|| LabError::InvalidData(format!("variant '{id}' was not inserted")))
    }

    /// Insert or update one DSP variant row and return the stored metadata.
    pub fn upsert_variant(
        &self,
        id: &str,
        node_id: &str,
        name: &str,
        impl_id: &str,
    ) -> Result<VariantMeta, LabError> {
        self.conn.execute(
            "INSERT INTO variants (id, node_id, name, impl_id)
             VALUES (?1, ?2, ?3, ?4)
             ON CONFLICT(id) DO UPDATE SET
                node_id = excluded.node_id,
                name = excluded.name,
                impl_id = excluded.impl_id",
            params![id, node_id, name, impl_id],
        )?;
        self.load_variant(id)?
            .ok_or_else(|| LabError::InvalidData(format!("variant '{id}' was not upserted")))
    }

    /// Load one variant metadata row by id.
    pub fn load_variant(&self, id: &str) -> Result<Option<VariantMeta>, LabError> {
        self.conn
            .query_row(
                "SELECT id, node_id, name, impl_id, status, active_snapshot_id, created_at
                 FROM variants
                 WHERE id = ?1",
                params![id],
                |row| {
                    Ok(VariantMeta {
                        id: row.get(0)?,
                        node_id: row.get(1)?,
                        name: row.get(2)?,
                        impl_id: row.get(3)?,
                        status: row.get(4)?,
                        active_snapshot_id: row.get(5)?,
                        created_at: row.get(6)?,
                    })
                },
            )
            .optional()
            .map_err(LabError::from)
    }

    /// Persist a full snapshot and dependency index rows.
    pub fn save_snapshot(&self, full: &SnapshotFull) -> Result<(), LabError> {
        let values_json = serde_json::to_string(&full.data.parameter_values)?;
        let data_json = serde_json::to_string(&full.data)?;
        self.conn.execute(
            "INSERT INTO snapshots (
                id,
                variant_id,
                version,
                status,
                variant_impl_id,
                values_json,
                data_json,
                params_hash,
                created_at,
                ready_at,
                notes
             )
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, NULL, ?10)",
            params![
                full.meta.id,
                full.meta.variant_id,
                full.meta.version,
                full.meta.status.as_str(),
                full.meta.variant_impl_id,
                values_json,
                data_json,
                full.meta.params_hash,
                full.meta.created_at,
                full.meta.notes,
            ],
        )?;

        self.conn.execute(
            "DELETE FROM dependencies WHERE snapshot_id = ?1",
            params![full.meta.id],
        )?;
        for dep in &full.data.dependencies.rust_crates {
            let dep_json = serde_json::to_string(dep)?;
            self.conn.execute(
                "INSERT INTO dependencies (snapshot_id, dep_type, dep_name, dep_version, dep_hash)
                 VALUES (?1, 'rust_crate', ?2, ?3, ?4)",
                params![full.meta.id, dep.name, dep.version, dep_json],
            )?;
        }
        for dep in &full.data.dependencies.system_tools {
            let dep_json = serde_json::to_string(dep)?;
            self.conn.execute(
                "INSERT INTO dependencies (snapshot_id, dep_type, dep_name, dep_version, dep_hash)
                 VALUES (?1, 'system_tool', ?2, ?3, ?4)",
                params![full.meta.id, dep.name, dep.version_min, dep_json],
            )?;
        }

        Ok(())
    }

    /// Delete a snapshot and its cascading dependency and verification rows.
    pub fn delete_snapshot(&self, id: &str) -> Result<(), LabError> {
        self.conn
            .execute("DELETE FROM snapshots WHERE id = ?1", params![id])?;
        Ok(())
    }

    /// Load a full snapshot by id.
    pub fn load_snapshot(&self, id: &str) -> Result<Option<SnapshotFull>, LabError> {
        let row = self
            .conn
            .query_row(
                "SELECT
                    id,
                    variant_id,
                    version,
                    status,
                    variant_impl_id,
                    params_hash,
                    created_at,
                    notes,
                    data_json
                 FROM snapshots
                 WHERE id = ?1",
                params![id],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                        row.get::<_, String>(3)?,
                        row.get::<_, String>(4)?,
                        row.get::<_, String>(5)?,
                        row.get::<_, String>(6)?,
                        row.get::<_, String>(7)?,
                        row.get::<_, String>(8)?,
                    ))
                },
            )
            .optional()?;

        let Some((
            id,
            variant_id,
            version,
            status,
            variant_impl_id,
            params_hash,
            created_at,
            notes,
            data_json,
        )) = row
        else {
            return Ok(None);
        };

        let status = SnapshotStatus::from_str(&status).map_err(LabError::InvalidData)?;
        let data = serde_json::from_str(&data_json)?;
        Ok(Some(SnapshotFull {
            meta: SnapshotMeta {
                id,
                variant_id,
                version,
                status,
                variant_impl_id,
                params_hash,
                created_at,
                notes,
            },
            data,
        }))
    }

    /// List snapshot metadata for a variant with newest snapshots first.
    pub fn list_snapshots_for_variant(
        &self,
        variant_id: &str,
    ) -> Result<Vec<SnapshotMeta>, LabError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, variant_id, version, status, variant_impl_id, params_hash, created_at, notes
             FROM snapshots
             WHERE variant_id = ?1
             ORDER BY created_at DESC, id DESC",
        )?;
        let rows = stmt.query_map(params![variant_id], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, String>(6)?,
                row.get::<_, String>(7)?,
            ))
        })?;

        let mut snapshots = Vec::new();
        for row in rows {
            let (id, variant_id, version, status, variant_impl_id, params_hash, created_at, notes) =
                row?;
            snapshots.push(SnapshotMeta {
                id,
                variant_id,
                version,
                status: SnapshotStatus::from_str(&status).map_err(LabError::InvalidData)?,
                variant_impl_id,
                params_hash,
                created_at,
                notes,
            });
        }
        Ok(snapshots)
    }

    /// Return the highest semantic-version snapshot for a variant.
    pub fn latest_snapshot_for_variant(
        &self,
        variant_id: &str,
    ) -> Result<Option<SnapshotMeta>, LabError> {
        let mut snapshots = self.list_snapshots_for_variant(variant_id)?;
        snapshots.sort_by(|left, right| {
            let left_version =
                Version::parse(&left.version).unwrap_or_else(|_| Version::new(0, 0, 0));
            let right_version =
                Version::parse(&right.version).unwrap_or_else(|_| Version::new(0, 0, 0));
            left_version
                .cmp(&right_version)
                .then_with(|| left.created_at.cmp(&right.created_at))
                .then_with(|| left.id.cmp(&right.id))
        });
        Ok(snapshots.pop())
    }

    /// Count dependency index rows for a snapshot.
    pub fn dependency_count_for_snapshot(&self, snapshot_id: &str) -> Result<i64, LabError> {
        let count = self.conn.query_row(
            "SELECT COUNT(*) FROM dependencies WHERE snapshot_id = ?1",
            params![snapshot_id],
            |row| row.get(0),
        )?;
        Ok(count)
    }

    /// Insert or update one verification check result row.
    pub fn save_verification_result(
        &self,
        snapshot_id: &str,
        definition: &VerificationCheckDef,
        result: &CheckResult,
    ) -> Result<(), LabError> {
        let checked_at = if result.status == CheckStatus::Pending {
            None
        } else {
            Some(chrono::Utc::now().to_rfc3339())
        };
        self.conn.execute(
            "INSERT INTO verification_checks (
                id,
                snapshot_id,
                check_order,
                category,
                \"check\",
                expected,
                automated,
                status,
                result_note,
                checked_at
             )
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
             ON CONFLICT(snapshot_id, id) DO UPDATE SET
                snapshot_id = excluded.snapshot_id,
                check_order = excluded.check_order,
                category = excluded.category,
                \"check\" = excluded.\"check\",
                expected = excluded.expected,
                automated = excluded.automated,
                status = excluded.status,
                result_note = excluded.result_note,
                checked_at = excluded.checked_at",
            params![
                definition.id,
                snapshot_id,
                definition.check_order,
                definition.category.as_str(),
                definition.check.as_str(),
                definition.expected.as_str(),
                if definition.automated { 1_i64 } else { 0_i64 },
                result.status.as_str(),
                result.result_note.as_deref(),
                checked_at,
            ],
        )?;
        Ok(())
    }

    /// Load persisted verification results for a snapshot.
    pub fn list_verification_results(
        &self,
        snapshot_id: &str,
    ) -> Result<Vec<CheckResult>, LabError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, status, result_note
             FROM verification_checks
             WHERE snapshot_id = ?1
             ORDER BY check_order ASC, id ASC",
        )?;
        let rows = stmt.query_map(params![snapshot_id], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, Option<String>>(2)?,
            ))
        })?;

        let mut results = Vec::new();
        for row in rows {
            let (check_id, status, result_note) = row?;
            results.push(CheckResult {
                check_id,
                status: CheckStatus::from_str(&status).map_err(LabError::InvalidData)?,
                result_note,
                duration_ms: 0,
            });
        }
        Ok(results)
    }

    /// Save or replace a serialized pipeline configuration.
    pub fn save_pipeline(&self, id: &str, config: &PipelineConfig) -> Result<(), LabError> {
        let config_json = serde_json::to_string(config)?;
        self.conn.execute(
            "INSERT INTO pipelines (id, name, config_json)
             VALUES (?1, ?2, ?3)
             ON CONFLICT(id) DO UPDATE SET
                name = excluded.name,
                config_json = excluded.config_json,
                updated_at = datetime('now')",
            params![id, config.name, config_json],
        )?;
        Ok(())
    }

    /// Load a serialized pipeline configuration by id.
    pub fn load_pipeline(&self, id: &str) -> Result<Option<PipelineConfig>, LabError> {
        let mut stmt = self
            .conn
            .prepare("SELECT config_json FROM pipelines WHERE id = ?1")?;
        let mut rows = stmt.query(params![id])?;
        if let Some(row) = rows.next()? {
            let config_json: String = row.get(0)?;
            Ok(Some(serde_json::from_str(&config_json)?))
        } else {
            Ok(None)
        }
    }

    /// Return whether SQLite foreign key enforcement is enabled.
    pub fn foreign_keys_enabled(&self) -> Result<bool, LabError> {
        let enabled: i64 = self
            .conn
            .query_row("PRAGMA foreign_keys", [], |row| row.get(0))?;
        Ok(enabled == 1)
    }

    /// Return the current SQLite journal mode.
    pub fn journal_mode(&self) -> Result<String, LabError> {
        let mode: String = self
            .conn
            .query_row("PRAGMA journal_mode", [], |row| row.get(0))?;
        Ok(mode)
    }

    /// Return true when a table or index exists in the SQLite schema.
    pub fn schema_object_exists(&self, name: &str) -> Result<bool, LabError> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE name = ?1",
            params![name],
            |row| row.get(0),
        )?;
        Ok(count == 1)
    }
}

#[cfg(test)]
mod tests {
    use super::Database;
    use crate::lab::{
        default_categories, CheckCategory, ComponentMeta, CrateDep, DependencyList, DspConfig,
        DspEngine, FaustLib, FfiInterface, IntegrationGuide, ParamValue, ParameterMeta,
        PipelineConfig, RoutingConfig, SnapshotData, SnapshotFull, SnapshotMeta, SnapshotStatus,
        SourceFile, SystemTool, VerificationCheckDef,
    };
    use std::path::PathBuf;
    use uuid::Uuid;

    fn temp_db_path(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("distortion-lab-{name}-{}.db", Uuid::new_v4()))
    }

    fn seed_variant(db: &Database) {
        db.seed_categories(&default_categories())
            .expect("seed categories");
        db.upsert_node("node-eq", "eq", "EQ").expect("insert node");
        db.insert_variant("variant-eq", "node-eq", "Test EQ", "test_eq_impl")
            .expect("insert variant");
    }

    fn snapshot(id: &str, version: &str) -> SnapshotFull {
        SnapshotFull {
            meta: SnapshotMeta {
                id: id.to_string(),
                variant_id: "variant-eq".to_string(),
                version: version.to_string(),
                status: SnapshotStatus::Draft,
                variant_impl_id: "test_eq_impl".to_string(),
                params_hash: format!("hash-{version}"),
                created_at: "2026-07-07T12:00:00Z".to_string(),
                notes: "round trip".to_string(),
            },
            data: SnapshotData {
                format: SnapshotData::FORMAT.to_string(),
                schema_version: SnapshotData::SCHEMA_VERSION.to_string(),
                component: ComponentMeta {
                    name: "Test EQ".to_string(),
                    category: "eq".to_string(),
                    variant_id: "variant-eq".to_string(),
                    version: version.to_string(),
                    status: SnapshotStatus::Draft,
                    created: "2026-07-07T12:00:00Z".to_string(),
                    author: "test".to_string(),
                    description: "fixture snapshot".to_string(),
                },
                parameter_values: vec![
                    ParamValue {
                        id: "gain".to_string(),
                        value: 0.25,
                    },
                    ParamValue {
                        id: "tone".to_string(),
                        value: 0.75,
                    },
                ],
                parameter_metadata: vec![ParameterMeta {
                    id: "gain".to_string(),
                    name: "Gain".to_string(),
                    description: "Input gain".to_string(),
                    range: (0.0, 1.0),
                    default: 0.5,
                    unit: Some("linear".to_string()),
                    smoothing: "none".to_string(),
                    index: 0,
                    backend: None,
                }],
                dsp: DspConfig {
                    engine: DspEngine::Faust,
                    language: "faust".to_string(),
                    source_files: vec![SourceFile {
                        path: "dsp/test.dsp".to_string(),
                        hash: "abc123".to_string(),
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
                        param_count: 2,
                        param_ids: vec!["gain".to_string(), "tone".to_string()],
                        buffer_format: "mono-f32".to_string(),
                    },
                },
                dependencies: DependencyList {
                    rust_crates: vec![CrateDep {
                        name: "serde".to_string(),
                        version: "1".to_string(),
                        features: vec!["derive".to_string()],
                        build_only: false,
                    }],
                    system_tools: vec![SystemTool {
                        name: "faust".to_string(),
                        version_min: "2.70".to_string(),
                        required: true,
                    }],
                },
                routing: RoutingConfig {
                    audio_input: "mono".to_string(),
                    audio_output: "mono".to_string(),
                    latency_samples: 0,
                },
                verification_checks: vec![VerificationCheckDef {
                    id: "params".to_string(),
                    check_order: 0,
                    category: CheckCategory::Params,
                    check: "params match".to_string(),
                    expected: "same ids".to_string(),
                    automated: true,
                }],
                engineering_notes: "preserve this exact note".to_string(),
                signal_flow_description: "input -> eq -> output".to_string(),
                integration_guide: IntegrationGuide {
                    summary: "Use as a mono EQ".to_string(),
                    host_framework: "nih-plug".to_string(),
                    target_platforms: vec!["linux".to_string()],
                    steps: vec!["compile".to_string(), "load".to_string()],
                    rust_integration_example: "variant.process_block(ptr, len);".to_string(),
                },
            },
        }
    }

    #[test]
    fn open_creates_schema_objects() {
        let path = temp_db_path("schema");
        let db = Database::open(&path).expect("open database");

        for object in [
            "categories",
            "nodes",
            "variants",
            "snapshots",
            "dependencies",
            "pipelines",
            "verification_checks",
            "idx_snapshots_variant",
            "idx_variants_node",
            "idx_checks_snapshot",
        ] {
            assert!(db.schema_object_exists(object).expect("schema object"));
        }

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn open_enables_foreign_keys_and_wal() {
        let path = temp_db_path("pragma");
        let db = Database::open(&path).expect("open database");

        assert!(db.foreign_keys_enabled().expect("foreign keys"));
        assert_eq!(db.journal_mode().expect("journal mode"), "wal");

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn seed_categories_is_idempotent_and_ordered() {
        let path = temp_db_path("categories");
        let db = Database::open(&path).expect("open database");

        db.seed_categories(&default_categories())
            .expect("first category seed");
        db.seed_categories(&default_categories())
            .expect("second category seed");

        let categories = db.list_categories().expect("list categories");
        assert_eq!(categories.len(), 8);
        assert_eq!(categories[0].id, "input-routing");
        assert_eq!(categories[7].id, "output-stage");

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn pipeline_config_persists_as_json() {
        let path = temp_db_path("pipeline");
        let db = Database::open(&path).expect("open database");
        let config = PipelineConfig::from_categories(&default_categories());

        db.save_pipeline("default", &config).expect("save pipeline");
        let loaded = db
            .load_pipeline("default")
            .expect("load pipeline")
            .expect("pipeline exists");

        assert_eq!(loaded, config);

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn snapshot_round_trips_full_data_json() {
        let path = temp_db_path("snapshot-roundtrip");
        let db = Database::open(&path).expect("open database");
        seed_variant(&db);
        let full = snapshot("snapshot-1", "1.0.0");

        db.save_snapshot(&full).expect("save snapshot");
        let loaded = db
            .load_snapshot("snapshot-1")
            .expect("load snapshot")
            .expect("snapshot exists");

        assert_eq!(loaded.meta.status, SnapshotStatus::Draft);
        assert_eq!(loaded.meta, full.meta);
        assert_eq!(loaded.data, full.data);
        assert_eq!(db.dependency_count_for_snapshot("snapshot-1").unwrap(), 2);

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn list_snapshots_for_variant_orders_newest_first() {
        let path = temp_db_path("snapshot-order");
        let db = Database::open(&path).expect("open database");
        seed_variant(&db);

        let mut first = snapshot("snapshot-1", "1.0.0");
        first.meta.created_at = "2026-07-07T12:00:00Z".to_string();
        let mut second = snapshot("snapshot-2", "1.0.1");
        second.meta.created_at = "2026-07-07T12:01:00Z".to_string();
        let mut third = snapshot("snapshot-3", "1.0.2");
        third.meta.created_at = "2026-07-07T12:01:00Z".to_string();

        db.save_snapshot(&first).expect("save first");
        db.save_snapshot(&second).expect("save second");
        db.save_snapshot(&third).expect("save third");

        let snapshots = db
            .list_snapshots_for_variant("variant-eq")
            .expect("list snapshots");

        assert_eq!(
            snapshots
                .iter()
                .map(|snapshot| snapshot.id.as_str())
                .collect::<Vec<_>>(),
            vec!["snapshot-3", "snapshot-2", "snapshot-1"]
        );

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn delete_snapshot_removes_it_from_listing() {
        let path = temp_db_path("snapshot-delete");
        let db = Database::open(&path).expect("open database");
        seed_variant(&db);

        db.save_snapshot(&snapshot("snapshot-1", "1.0.0"))
            .expect("save snapshot");
        assert_eq!(
            db.list_snapshots_for_variant("variant-eq")
                .expect("list snapshots")
                .len(),
            1
        );

        db.delete_snapshot("snapshot-1").expect("delete snapshot");
        assert!(db
            .list_snapshots_for_variant("variant-eq")
            .expect("list snapshots")
            .is_empty());

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn latest_snapshot_for_variant_uses_semver_order() {
        let path = temp_db_path("snapshot-latest");
        let db = Database::open(&path).expect("open database");
        seed_variant(&db);

        db.save_snapshot(&snapshot("snapshot-1", "1.9.0"))
            .expect("save first");
        db.save_snapshot(&snapshot("snapshot-2", "1.10.0"))
            .expect("save second");
        db.save_snapshot(&snapshot("snapshot-3", "1.2.0"))
            .expect("save third");

        let latest = db
            .latest_snapshot_for_variant("variant-eq")
            .expect("latest snapshot")
            .expect("snapshot exists");

        assert_eq!(latest.id, "snapshot-2");
        assert_eq!(latest.version, "1.10.0");

        let _ = std::fs::remove_file(path);
    }
}
