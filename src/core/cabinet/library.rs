use super::types::{CabinetError, IrMeta};
use rusqlite::{params, Connection, OptionalExtension};
use std::io::Cursor;
use std::path::Path;

/// Maximum allowed IR file size (10 MB). Typical cabinet IRs are < 1 MB.
const MAX_IR_BYTES: usize = 10 * 1024 * 1024;

/// Owns the SQLite connection to the shared cabinet IR library.
///
/// All access happens on the UI / worker thread — the audio thread never
/// touches this. See the module docs for the thread-safety contract.
pub struct CabinetLibrary {
    conn: Connection,
}

impl CabinetLibrary {
    /// Open (or create) the library database at `db_path`, running schema
    /// creation and enabling WAL mode with a short busy timeout for concurrent
    /// plugin + standalone access.
    pub fn new(db_path: &Path) -> Result<Self, CabinetError> {
        let conn = Connection::open(db_path).map_err(|e| CabinetError::Database(e.to_string()))?;

        conn.busy_timeout(std::time::Duration::from_millis(200))
            .map_err(|e| CabinetError::Database(e.to_string()))?;
        conn.pragma_update(None, "journal_mode", "WAL")
            .map_err(|e| CabinetError::Database(e.to_string()))?;

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS cabinet_irs (
                id           INTEGER PRIMARY KEY AUTOINCREMENT,
                name         TEXT NOT NULL,
                filename     TEXT NOT NULL,
                content_hash TEXT NOT NULL UNIQUE,
                sample_rate  INTEGER NOT NULL,
                channels     INTEGER NOT NULL,
                num_frames   INTEGER NOT NULL,
                bit_depth    INTEGER NOT NULL,
                byte_size    INTEGER NOT NULL,
                ir_data      BLOB NOT NULL,
                created_at   TEXT NOT NULL DEFAULT (datetime('now')),
                last_used_at TEXT
            );
            CREATE TABLE IF NOT EXISTS cabinet_state (
                key   TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );",
        )
        .map_err(|e| CabinetError::Database(e.to_string()))?;

        // Record schema version for future migrations (no-op if already set).
        conn.execute(
            "INSERT OR IGNORE INTO cabinet_state (key, value) VALUES ('schema_version', '1')",
            [],
        )
        .map_err(|e| CabinetError::Database(e.to_string()))?;

        Ok(Self { conn })
    }

    /// Import raw WAV bytes into the library.
    ///
    /// Hashes the bytes (BLAKE3), and if the hash already exists this is a no-op
    /// that just refreshes `last_used_at` and returns the existing metadata
    /// (dedup). Otherwise parses metadata with hound (read-only) and INSERTs the
    /// exact bytes unchanged.
    pub fn import_ir(&self, raw_bytes: &[u8], filename: &str) -> Result<IrMeta, CabinetError> {
        if raw_bytes.len() > MAX_IR_BYTES {
            return Err(CabinetError::WavDecode(format!(
                "IR too large: {} bytes (max {} bytes)",
                raw_bytes.len(),
                MAX_IR_BYTES
            )));
        }

        let content_hash = blake3::hash(raw_bytes).to_hex().to_string();

        // Dedup: if it already exists, just touch last_used_at and return it.
        if let Some(existing) = self.try_get_meta(&content_hash)? {
            self.conn
                .execute(
                    "UPDATE cabinet_irs SET last_used_at = datetime('now') WHERE content_hash = ?1",
                    params![content_hash],
                )
                .map_err(|e| CabinetError::Database(e.to_string()))?;
            return Ok(existing);
        }

        // Parse metadata (read-only) — must succeed before INSERT.
        let reader = hound::WavReader::new(Cursor::new(raw_bytes))
            .map_err(|e| CabinetError::WavDecode(e.to_string()))?;
        let spec = reader.spec();
        let num_frames = reader.duration() as usize;

        let stem = Path::new(filename)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or(filename)
            .to_string();

        self.conn
            .execute(
                "INSERT INTO cabinet_irs
                    (name, filename, content_hash, sample_rate, channels,
                     num_frames, bit_depth, byte_size, ir_data, last_used_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, datetime('now'))",
                params![
                    stem,
                    filename,
                    content_hash,
                    spec.sample_rate,
                    spec.channels,
                    num_frames as i64,
                    spec.bits_per_sample,
                    raw_bytes.len() as i64,
                    raw_bytes,
                ],
            )
            .map_err(|e| CabinetError::Database(e.to_string()))?;

        self.get_meta(&content_hash)
    }

    /// Fetch metadata + the exact stored WAV bytes for `hash`, verifying
    /// integrity by re-hashing the BLOB.
    pub fn get_ir_by_hash(&self, hash: &str) -> Result<(IrMeta, Vec<u8>), CabinetError> {
        let meta = self.get_meta(hash)?;
        let data: Vec<u8> = self
            .conn
            .query_row(
                "SELECT ir_data FROM cabinet_irs WHERE content_hash = ?1",
                params![hash],
                |row| row.get(0),
            )
            .map_err(|e| CabinetError::Database(e.to_string()))?;

        let actual = blake3::hash(&data).to_hex().to_string();
        if actual != hash {
            return Err(CabinetError::Corrupt(format!(
                "hash mismatch for {} (stored bytes are corrupt)",
                hash
            )));
        }

        Ok((meta, data))
    }

    /// List all IRs in the library, newest first.
    pub fn list_irs(&self) -> Result<Vec<IrMeta>, CabinetError> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT content_hash, name, filename, sample_rate, channels,
                        num_frames, bit_depth, byte_size, created_at, last_used_at
                 FROM cabinet_irs ORDER BY created_at DESC, id DESC",
            )
            .map_err(|e| CabinetError::Database(e.to_string()))?;

        let rows = stmt
            .query_map([], Self::row_to_meta)
            .map_err(|e| CabinetError::Database(e.to_string()))?;

        let mut out = Vec::new();
        for r in rows {
            out.push(r.map_err(|e| CabinetError::Database(e.to_string()))?);
        }
        Ok(out)
    }

    /// Delete an IR by hash. If it was the selected IR, clears the selection.
    pub fn delete_ir(&self, hash: &str) -> Result<(), CabinetError> {
        self.conn
            .execute(
                "DELETE FROM cabinet_irs WHERE content_hash = ?1",
                params![hash],
            )
            .map_err(|e| CabinetError::Database(e.to_string()))?;

        if self.get_selected_hash()?.as_deref() == Some(hash) {
            self.conn
                .execute(
                    "DELETE FROM cabinet_state WHERE key = 'selected_hash'",
                    [],
                )
                .map_err(|e| CabinetError::Database(e.to_string()))?;
        }
        Ok(())
    }

    /// Rename an IR's editable label.
    pub fn rename_ir(&self, hash: &str, new_name: &str) -> Result<(), CabinetError> {
        self.conn
            .execute(
                "UPDATE cabinet_irs SET name = ?1 WHERE content_hash = ?2",
                params![new_name, hash],
            )
            .map_err(|e| CabinetError::Database(e.to_string()))?;
        Ok(())
    }

    /// Get the currently selected IR hash, if any.
    pub fn get_selected_hash(&self) -> Result<Option<String>, CabinetError> {
        self.conn
            .query_row(
                "SELECT value FROM cabinet_state WHERE key = 'selected_hash'",
                [],
                |row| row.get(0),
            )
            .optional()
            .map_err(|e| CabinetError::Database(e.to_string()))
    }

    /// Set the currently selected IR hash.
    pub fn set_selected_hash(&self, hash: &str) -> Result<(), CabinetError> {
        self.conn
            .execute(
                "INSERT INTO cabinet_state (key, value) VALUES ('selected_hash', ?1)
                 ON CONFLICT(key) DO UPDATE SET value = excluded.value",
                params![hash],
            )
            .map_err(|e| CabinetError::Database(e.to_string()))?;
        Ok(())
    }

    /// Seed the default embedded IR — only if the library is currently empty.
    /// Returns the metadata of the seeded (or already-present) default IR.
    pub fn seed_default_ir(&self, default_bytes: &[u8]) -> Result<IrMeta, CabinetError> {
        let count: i64 = self
            .conn
            .query_row("SELECT COUNT(*) FROM cabinet_irs", [], |row| row.get(0))
            .map_err(|e| CabinetError::Database(e.to_string()))?;

        if count == 0 {
            let m = self.import_ir(default_bytes, "Default Cabinet.wav")?;
            self.rename_ir(&m.content_hash, "Default Cabinet")?;
            self.set_selected_hash(&m.content_hash)?;
            return self.get_meta(&m.content_hash);
        }

        // Already populated — resolve the default's hash if present, otherwise
        // fall back to the first available IR so callers always get something.
        let hash = blake3::hash(default_bytes).to_hex().to_string();
        if let Some(meta) = self.try_get_meta(&hash)? {
            return Ok(meta);
        }
        self.list_irs()?
            .into_iter()
            .next()
            .ok_or_else(|| CabinetError::NotFound("no IRs in library".to_string()))
    }

    // --- internal helpers ---

    fn try_get_meta(&self, hash: &str) -> Result<Option<IrMeta>, CabinetError> {
        self.conn
            .query_row(
                "SELECT content_hash, name, filename, sample_rate, channels,
                        num_frames, bit_depth, byte_size, created_at, last_used_at
                 FROM cabinet_irs WHERE content_hash = ?1",
                params![hash],
                Self::row_to_meta,
            )
            .optional()
            .map_err(|e| CabinetError::Database(e.to_string()))
    }

    fn get_meta(&self, hash: &str) -> Result<IrMeta, CabinetError> {
        self.try_get_meta(hash)?
            .ok_or_else(|| CabinetError::NotFound(hash.to_string()))
    }

    fn row_to_meta(row: &rusqlite::Row) -> rusqlite::Result<IrMeta> {
        Ok(IrMeta {
            content_hash: row.get(0)?,
            name: row.get(1)?,
            filename: row.get(2)?,
            sample_rate: row.get(3)?,
            channels: row.get(4)?,
            num_frames: row.get::<_, i64>(5)? as usize,
            bit_depth: row.get(6)?,
            byte_size: row.get::<_, i64>(7)? as usize,
            created_at: row.get(8)?,
            last_used_at: row.get(9)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::cabinet::CabinetRuntime;

    const DEFAULT_IR: &[u8] = include_bytes!("../../../neural/drive/cabinet_ir.wav");

    fn mem_lib() -> CabinetLibrary {
        CabinetLibrary::new(std::path::Path::new(":memory:")).expect("open in-memory db")
    }

    #[test]
    fn seed_lists_and_selects_default() {
        let lib = mem_lib();
        let meta = lib.seed_default_ir(DEFAULT_IR).expect("seed");
        assert_eq!(meta.byte_size, DEFAULT_IR.len());

        let all = lib.list_irs().expect("list");
        assert_eq!(all.len(), 1, "exactly one seeded IR");

        let selected = lib.get_selected_hash().expect("selected").expect("some");
        assert_eq!(selected, meta.content_hash);
    }

    #[test]
    fn integrity_and_dedup_and_rename() {
        let lib = mem_lib();
        let m1 = lib.import_ir(DEFAULT_IR, "cab.wav").expect("import");

        // Byte-exact round trip + integrity check passes.
        let (_meta, bytes) = lib.get_ir_by_hash(&m1.content_hash).expect("get");
        assert_eq!(bytes, DEFAULT_IR);

        // Re-importing the same bytes dedups (still one row, same hash).
        let m2 = lib.import_ir(DEFAULT_IR, "other-name.wav").expect("re-import");
        assert_eq!(m1.content_hash, m2.content_hash);
        assert_eq!(lib.list_irs().unwrap().len(), 1);

        // Rename updates the label.
        lib.rename_ir(&m1.content_hash, "My Cab").expect("rename");
        let renamed = lib.list_irs().unwrap().into_iter().next().unwrap();
        assert_eq!(renamed.name, "My Cab");
    }

    #[test]
    fn delete_clears_selection() {
        let lib = mem_lib();
        let m = lib.seed_default_ir(DEFAULT_IR).expect("seed");
        lib.delete_ir(&m.content_hash).expect("delete");
        assert!(lib.list_irs().unwrap().is_empty());
        assert!(lib.get_selected_hash().unwrap().is_none());
    }

    #[test]
    fn size_guard_rejects_oversized() {
        let lib = mem_lib();
        let huge = vec![0u8; MAX_IR_BYTES + 1];
        assert!(lib.import_ir(&huge, "huge.wav").is_err());
    }

    #[test]
    fn runtime_builds_from_stored_bytes() {
        let lib = mem_lib();
        let m = lib.seed_default_ir(DEFAULT_IR).expect("seed");
        let (_meta, bytes) = lib.get_ir_by_hash(&m.content_hash).unwrap();

        // Same rate: no resample.
        let rt = CabinetRuntime::build(&bytes, 48_000.0, 512).expect("build");
        assert!(rt.num_frames > 0);
        assert_eq!(rt.ir_hash, m.content_hash);

        // Different rate: exercises the rubato resample path.
        let rt2 = CabinetRuntime::build(&bytes, 44_100.0, 256).expect("build resampled");
        assert!(rt2.num_frames > 0);
    }
}
