# Component Lab — Design

**Spec:** `.specs/features/component-lab/spec.md`
**Status:** Draft → Revision 2 (post cross-review)
**Created:** 2026-07-07
**Updated:** 2026-07-07 (addressed B1-B8 from cross-review)

---

## Architecture Overview

O módulo `src/lab/` é uma camada de metadados, persistência e seleção que envolve os nós DSP existentes **sem modificar** a pipeline de áudio. O lab fornece versionamento de configurações e seleção de variantes, mas as implementações DSP são **todas pré-compiladas no binário** via `build.rs`. NÃO há compilação Faust/Mojo em runtime.

```
┌──────────────────────────────────────────────────────────────────┐
│                         EGUI UI LAYER                            │
│  ┌─────────────┐  ┌──────────────┐  ┌─────────────────────────┐ │
│  │ Lab Panel   │  │ Variant      │  │ Verification Panel      │ │
│  │ (snapshots) │  │ Switcher     │  │ (checklist)             │ │
│  └──────┬──────┘  └──────┬───────┘  └────────────┬────────────┘ │
└─────────┼────────────────┼───────────────────────┼──────────────┘
          │                │                       │
┌─────────┼────────────────┼───────────────────────┼──────────────┐
│         ▼                ▼                       ▼              │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │                    src/lab/ (Lab Facade)                  │   │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌────────────┐  │   │
│  │  │ database │ │ snapshot │ │ variant  │ │ pipeline    │  │   │
│  │  │(rusqlite)│ │ (serde)  │ │(selector)│ │ (slots)     │  │   │
│  │  └────┬─────┘ └────┬─────┘ └────┬─────┘ └──────┬──────┘  │   │
│  └───────┼────────────┼────────────┼──────────────┼─────────┘   │
│          │            │            │              │              │
│  ┌───────▼────────────▼────────────▼──────────────▼─────────┐   │
│  │              SQLite DB (~/.config/.../lab.db)             │   │
│  │  categories | nodes | variants | snapshots | pipelines    │   │
│  └──────────────────────────────────────────────────────────┘   │
│                                                                  │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │              Export Engine (lab/export.rs)                │   │
│  │  variant.json + DSP sources → .tar.gz bundle              │   │
│  └──────────────────────────────────────────────────────────┘   │
└──────────────────────────────────────────────────────────────────┘
          │
          ▼
┌──────────────────────────────────────────────────────────────────┐
│                EXISTING DSP PIPELINE (add-only, unchanged)        │
│  BaseIO::process() → Faust EQ → PreEQ Conv → Amp → Cab → Master  │
│                                                                  │
│  Lab adds: a VariantRegistry that maps variant IDs to factory    │
│  functions. Each node selects a variant; existing pipeline       │
│  stages query the node for which DSP to use.                     │
└──────────────────────────────────────────────────────────────────┘
```

**Princípio fundamental:** O lab é **add-only**. A pipeline de áudio existente (`BaseIO::process()`) continua funcionando. O lab adiciona uma camada de seleção: cada estágio da pipeline consulta seu nó correspondente para saber qual variante DSP usar. Nenhum código de áudio existente é removido.

---

## Code Reuse Analysis

### Existing Components to Leverage

| Component | Location | How to Use |
|---|---|---|
| `rusqlite` (already in Cargo.toml) | `Cargo.toml:30` | Direct dependency for `lab/database.rs` |
| `sha2` (NEW) | `Cargo.toml` | SHA256 for MANIFEST + snapshot content hashing |
| `dirs` (already in Cargo.toml) | `Cargo.toml:32` | Locate `~/.config/` for DB path |
| `serde` / `serde_json` (already) | `Cargo.toml:33-34` | Serialize snapshots to JSON |
| `thiserror` (already in Cargo.toml) | `Cargo.toml:35` | Error types for lab module |
| `CabinetLibrary` SQLite pattern | `src/core/cabinet/library.rs` | Reuse SQLite init, WAL journal, migration patterns |
| `CabinetEngine` mailbox pattern | `src/core/cabinet/engine.rs:23-63, 173-220` | Exact pattern for variant runtime swap: audio thread owns live `Option<Arc<VariantRuntime>>` uniquely; inbox posts new runtimes via `ArcSwapOption`; old parked in single-slot `ArcSwapOption` trash for UI-thread deferred drop. Zero locks, zero allocs on audio thread. |
| `CabinetEngine::process()` pattern | `src/core/cabinet/engine.rs:173-220` | Audio thread: `&mut self` via single-writer ownership, processes through `Option<Arc<...>>` — no lock, no allocation |
| `EnumParam<AmpModel>` pattern | `D-004 (STATE.md)` | Reuse for variant selector UI |
| `EditorState` Arc sharing | `src/core/state/plugin_params.rs:85` | Share lab state between UI and audio threads |
| `BaseIOParams` pattern | `src/core/state/plugin_params.rs` | Fixed parameter layout; snapshots fill values into these slots |

### Integration Points

| System | Integration Method |
|---|---|
| Plugin `BaseIO::initialize()` | Initialize `Lab` singleton, register variant factories |
| Plugin `BaseIO::process()` | Query node for active variant; call existing DSP functions (no new trait required) |
| Standalone `StandaloneApp` | Mirror lab state in `StandaloneState`, init on startup |
| `build.rs` | **No runtime integration.** All variant DSP sources compiled at build time. `build.rs` registers variant IDs in a generated `VARIANT_REGISTRY` static. |
| `egui` panels | New `lab_panel.rs` + extend existing `signal_chain.rs` |

---

## Key Architectural Constraint: No Runtime Compilation

**Todas as implementações DSP são pré-compiladas no binário.** O `build.rs` compila todos os `.dsp` (Faust → C++ → link) e `.mojo` (→ .so) em tempo de build. Um `VariantRegistry` estático mapeia variant IDs para factory functions que instanciam o DSP já compilado.

```rust
// Generated by build.rs — static registry of all compiled-in variants
type VariantFactory = fn(sample_rate: f32) -> Box<dyn DspVariant>;

static VARIANT_REGISTRY: &[(&str, VariantFactory)] = &[
    ("mlc_zero_v_drive2", mlc_zero_v_factory),
    ("mojo_neural",        mojo_neural_factory),
    ("faust_eq_3band",     faust_eq_factory),
    // ... registered at build time
];
```

**Snapshot → Variant mapping:** Um snapshot armazena um `variant_impl_id` (string) que referencia uma entrada na `VARIANT_REGISTRY`. Carregar um snapshot que aponta para um DSP não compilado resulta em erro em runtime — o DSP deve ser adicionado ao `build.rs` e o binário recompilado.

---

## Components

### 1. Lab (Facade)

- **Purpose:** Top-level coordinator — owns the database connection, pipeline state, variant registry, and export engine. Single instance per application lifetime. DB access is UI-thread-only (behind `Mutex<Database>` for background thread safety).
- **Location:** `src/lab/mod.rs`
- **Interfaces:**
  - `Lab::init(data_dir: PathBuf) -> Result<Self>` — Initialize DB, run migrations, register variant factories, load last pipeline
  - `Lab::save_snapshot(node_id: &str, notes: &str) -> Result<Snapshot>` — Capture current param values + variant ID as snapshot
  - `Lab::load_snapshot(node_id: &str, snapshot_id: &str) -> Result<()>` — Apply saved param values to node's parameter slots
  - `Lab::switch_variant(node_id: &str, variant_id: &str) -> Result<SwitchHandle>` — Begin async variant swap (mailbox+trash pattern)
  - `Lab::export_component(snapshot_id: &str, dest: PathBuf) -> Result<PathBuf>` — Generate .tar.gz bundle
  - `Lab::pipeline(&self) -> &Pipeline` — Read current pipeline state
  - `Lab::categories(&self) -> &[Category]` — List registered categories
  - `Lab::variant_registry(&self) -> &VariantRegistry` — For audio thread to look up factories
- **Dependencies:** database (behind `Mutex`), pipeline, export, verification
- **Reuses:** `rusqlite::Connection` (existing), `dirs::config_dir()` (existing)

### 2. Database

- **Purpose:** SQLite CRUD operations for categories, nodes, variants, snapshots, pipelines, and verification checks. DB is owned by the UI thread. Background variant loading reads via `Mutex<Database>`.
- **Location:** `src/lab/database.rs`
- **Interfaces:**
  - `Database::open(path: &Path) -> Result<Self>` — Open or create DB, run migrations
  - `Database::insert_snapshot(variant_id, version, values_json, variant_impl_id, params_hash) -> Result<SnapshotRow>`
  - `Database::get_snapshots_for_variant(variant_id) -> Result<Vec<SnapshotRow>>`
  - `Database::get_variants_for_node(node_id) -> Result<Vec<VariantRow>>`
  - `Database::insert_variant(node_id, name, impl_id) -> Result<VariantRow>`
  - `Database::list_categories() -> Result<Vec<CategoryRow>>`
  - `Database::save_pipeline(json) -> Result<()>`
  - `Database::load_pipeline(name) -> Result<PipelineConfig>`
  - `Database::set_active_snapshot(variant_id, snapshot_id) -> Result<()>`
- **Dependencies:** `rusqlite` (bundled)
- **Reuses:** Migration pattern from `CabinetLibrary::open_or_create()` [src/core/cabinet/library.rs:89]; PRAGMA journal_mode=WAL

### 3. Snapshot

- **Purpose:** Data model for component snapshots — parameter value capture, serialization, hashing, diff generation. **Snapshots store values, not NIH parameter definitions.**
- **Location:** `src/lab/snapshot.rs`
- **Interfaces:**
  - `Snapshot::capture(node: &Node) -> Result<Self>` — Capture current param values + variant ID from live node
  - `Snapshot::to_json(&self) -> String` — Serialize to AI-readable JSON
  - `Snapshot::from_json(json: &str) -> Result<Self>`
  - `Snapshot::apply_to(&self, params: &mut BaseIOParams)` — Write stored values into NIH FloatParam slots (by param ID)
  - `Snapshot::diff(&self, other: &Snapshot) -> SnapshotDiff` — Value changes, variant changes
- **Dependencies:** `serde`, `serde_json`, `sha2` (NEW), `chrono` (NEW), `semver` (NEW)
- **Reuses:** None (new hashing strategy using SHA256 exclusively)

### 4. Variant Runtime (Mailbox+Trash Pattern)

- **Purpose:** Runtime representation of a DSP variant — wraps the compiled DSP implementation. Follows `CabinetEngine`'s exact hand-off pattern: audio thread owns live runtime uniquely (`Option<Arc<VariantRuntime>>`); mailbox posts new runtimes lock-free via `ArcSwapOption`; old runtime parked in single-slot `ArcSwapOption` trash for UI-thread deferred drop. **Zero locks, zero allocations on audio thread.**
- **Location:** `src/lab/variant_runtime.rs`
- **Pattern (exactly mirrors CabinetEngine engine.rs:23–27, 173–220):**

```rust
/// Exactly mirrors CabinetMailbox — lock-free delivery.
/// inbox: UI/worker publishes new runtime (ArcSwapOption, lock-free)
/// trash: audio thread parks old runtime after swap (single slot, UI drains)
pub struct VariantMailbox {
    inbox: ArcSwapOption<VariantRuntime>,
    trash: ArcSwapOption<VariantRuntime>,
    clear_flag: AtomicBool,  // set by UI to unload variant; polled by audio thread
}

/// Owned by one "engine" struct — audio thread processes through this.
/// current: Option<Arc<VariantRuntime>>, NOT behind a lock.
/// mailbox: published by UI thread, consumed by audio thread at safe points.
pub struct VariantSlot {
    current: Option<Arc<VariantRuntime>>,  // audio thread owns uniquely
    mailbox: VariantMailbox,
}

impl VariantSlot {
    /// Audio thread: process with current variant (lock-free, alloc-free).
    /// If no variant loaded, audio passes through unchanged.
    pub fn process(&mut self, buffer: *mut f32, length: usize) {
        let rt = match &mut self.current {
            Some(rt) => rt,
            None => return, // empty slot: valid pass-through (spec P5)
        };
        // Arc::get_mut returns Some only when no other Arc references exist.
        match Arc::get_mut(rt) {
            Some(rt) => rt.process_block(buffer, length),
            None => debug_assert!(false, "variant runtime Arc unexpectedly shared; get_mut failed"),
        }
    }

    /// Audio thread: check for new runtime from inbox (lock-free swap).
    /// Called once per block at a safe point. No lock, no allocation.
    pub fn collect_mailbox(&mut self) {
        // Check for unload request first.
        if self.mailbox.clear_flag.load(Ordering::Acquire) {
            if let Some(old_rt) = self.current.take() {
                self.mailbox.trash.store(Some(old_rt));
            }
            self.mailbox.clear_flag.store(false, Ordering::Relaxed);
            return;
        }
        if let Some(new_rt) = self.mailbox.inbox.swap(None) {
            let old = self.current.replace(new_rt);
            if let Some(old_rt) = old {
                // Park old variant into single-slot trash.
                // UI thread drains this — never dropped on audio thread.
                self.mailbox.trash.store(Some(old_rt));
            }
        }
    }
}

// UI thread:
impl VariantSlot {
    /// UI/worker posts a newly-built runtime to the inbox for audio pickup.
    pub fn install(&self, new_runtime: Arc<VariantRuntime>) {
        // Clear any pending unload flag — installing a new runtime supersedes it.
        self.mailbox.clear_flag.store(false, Ordering::Relaxed);
        // Drain previous trash first to avoid leaking
        let _ = self.mailbox.trash.swap(None);
        self.mailbox.inbox.store(Some(new_runtime));
    }

    /// UI thread: request unloading the current variant (go to passthrough).
    /// Audio thread will drain it on next collect_mailbox.
    pub fn clear(&self) {
        self.mailbox.clear_flag.store(true, Ordering::Release);
    }

    /// UI thread periodic: drain trash (drop old DSP runtime here, NEVER audio).
    pub fn collect_garbage(&self) {
        let _ = self.mailbox.trash.swap(None);
    }
}

// Safety invariant: VariantSlot requires exclusive &mut access on audio
// thread (single-writer). Arc::get_mut returns Some only when no other
// Arc references exist. Drops are deferred to UI thread via trash.
// Same invariant as CabinetEngine (engine.rs:260).
```

- **DspVariant trait (replaces ExternalProcessor):**

```rust
/// Trait for compiled-in DSP variant implementations.
/// Keeps the in-place, zero-copy buffer model from ExternalProcessor.
pub trait DspVariant: Send {
    /// Process audio in-place. Matches ExternalProcessor::process_block signature.
    fn process_block(&mut self, buffer: *mut f32, length: usize);

    /// Number of parameter slots this variant exposes (for snapshot validation).
    fn param_count(&self) -> usize;

    /// Parameter IDs this variant expects, in order.
    fn param_ids(&self) -> &[&str];

    /// Latency in samples.
    fn latency(&self) -> usize;
}
```

**Key differences from original design:**
- Keeps the `*mut f32` in-place zero-copy contract (matches existing Faust/Mojo FFI)
- Removes `param_defs()`, `set_param_defs()`, `dsp_config()`, `compile()`, `engineering_notes()` — those belong in snapshot metadata, not runtime traits
- Audio thread never drops DSP objects (single-slot ArcSwapOption trash, drained by UI thread)
- `VariantSlot` provides `&mut self` to audio thread via single-writer ownership (mirrors CabinetEngine exactly: `ArcSwapOption` inbox+trash, no `Mutex`, no `Vec::push`, no alloc on audio thread)
- `VariantRuntime` wraps `Box<dyn DspVariant>`; audio-thread mutation via safe `Arc::get_mut` (same pattern as CabinetEngine — no `unsafe`, no `Arc::as_ptr` cast)

### 5. Node

- **Purpose:** Manages a single slot in the pipeline — owns the VariantSlot, provides `&mut self` to audio thread (single-writer).
- **Location:** `src/lab/node.rs`
- **Interfaces:**
  - `Node::new(category: Category) -> Self`
  - `Node::add_variant_meta(&mut self, name: &str, impl_id: &str) -> Result<&VariantMeta>` — Register a variant (metadata only; DSP is pre-compiled)
  - `Node::request_switch(&mut self, variant_id: &str, registry: &VariantRegistry) -> Result<SwitchHandle>` — Begin async build+install
  - `Node::variant_slot_mut(&mut self) -> &mut VariantSlot` — For audio thread to call process() with exclusive access
  - `Node::active_variant_id(&self) -> Option<String>`
- **Dependencies:** variant_runtime, database
- **Reuses:** `ArcSwapOption` from cabinet

### 6. Pipeline

- **Purpose:** Registry of category slots, enforces 1-node-per-category rule, amp exclusivity group (amp-modeler|amp-capture), serializes active pipeline configuration.
- **Location:** `src/lab/pipeline.rs`
- **Interfaces:**
  - `Pipeline::register_category(cat: Category) -> Result<&Node>`
  - `Pipeline::get_node(category_id: &str) -> Option<&Node>`
  - `Pipeline::get_node_mut(category_id: &str) -> Option<&mut Node>`
  - `Pipeline::validate(&self) -> Result<()>` — Check 1-slot-per-category + amp exclusivity
  - `Pipeline::to_config(&self) -> PipelineConfig`
  - `Pipeline::from_config(config: &PipelineConfig, registry: &VariantRegistry) -> Result<Self>`
- **Dependencies:** node, category
- **Reuses:** `BTreeMap<String, Node>` keyed by category_id (deterministic ordering)

### 7. Export

- **Purpose:** Generate self-contained `.tar.gz` bundles from ready snapshots.
- **Location:** `src/lab/export.rs`
- **Interfaces:**
  - `ExportEngine::export(snapshot: &SnapshotFull, dest: &Path) -> Result<PathBuf>`
  - `ExportEngine::estimate_size(snapshot: &SnapshotFull) -> u64` — Sum source files before copy
  - `ExportEngine::generate_manifest(files: &[ExportFile]) -> Manifest`
- **Dependencies:** `flate2` (NEW), `tar` (NEW), `sha2` (NEW)
- **Reuses:** File copy + SHA256 hashing

### 8. Verification

- **Purpose:** Run automated checks against a snapshot and track manual check status.
- **Location:** `src/lab/verification.rs`
- **Interfaces:**
  - `Verifier::run_automated(snapshot: &SnapshotFull, variant: &VariantRuntime) -> Result<Vec<CheckResult>>`
    - Checks: param count matches, null input silence (RMS < -96dBFS at 48kHz, 1024 samples), sine sweep no NaN/Inf, source file hashes match
  - `Verifier::checklist(snapshot_id: &str) -> Result<Vec<VerificationCheck>>`
  - `Verifier::mark_manual(check_id: &str, passed: bool, note: &str) -> Result<()>`
  - `Verifier::all_passed(snapshot_id: &str) -> bool`
- **Dependencies:** snapshot, database
- **Reuses:** None (new subsystem)

### 9. UI — Lab Panel

- **Purpose:** egui panel for managing snapshots, variants, and exports.
- **Location:** `src/core/ui/lab_panel.rs`
- **Interfaces:**
  - `lab_panel(ctx: &egui::Context, lab: &Lab, node_id: &str)` — Main panel
  - `variant_switcher(ctx: &egui::Context, node: &mut Node)` — Dropdown + switch button
  - `snapshot_list(ctx: &egui::Context, snapshots: &[SnapshotMeta])` — Version history
  - `verification_panel(ctx: &egui::Context, checks: &[VerificationCheck])` — Checklist
  - `export_dialog(ctx: &egui::Context, snapshot: &Snapshot)` — Export path + progress
- **Dependencies:** `eframe`/`egui` (existing), `rfd` (existing)
- **Reuses:** Panel layout pattern from `cabinet_panel.rs`

### 10. Export UI State

- **Purpose:** Explicit state model for long-running export operations (toasts + progress).
- **Location:** `src/core/ui/lab_panel.rs` (same file)
- **Structs:**
  - `LabUiState { export_progress: Option<ExportProgress>, last_error: Option<String>, switch_state: Option<SwitchProgress> }`

---

## Data Models

### Category

```rust
struct Category {
    id: String,           // "amp-modeler"
    name: String,         // "Amp Modeler"
    description: String,
    sort_order: u32,      // position in pipeline
}
```

### ParamValue (what snapshots store)

```rust
/// A single parameter value at snapshot time.
/// The param's definition (range, smoothing, label) lives in BaseIOParams (compile-time).
/// Snapshots only store the value.
#[derive(Serialize, Deserialize)]
struct ParamValue {
    id: String,           // matches FloatParam #[id = "..."]
    value: f32,           // the saved value
}
```

### SnapshotFull

```rust
struct SnapshotFull {
    meta: SnapshotMeta,
    data: SnapshotData,
}

struct SnapshotMeta {
    id: String,
    variant_id: String,
    version: String,              // semver
    status: SnapshotStatus,        // draft | testing | ready | deprecated
    variant_impl_id: String,       // which compiled-in DSP this uses
    params_hash: String,           // SHA256 of serialized param values
    created_at: String,
    notes: String,
}

struct SnapshotData {
    format: String,                 // "dsp-component-snapshot"
    schema_version: String,        // "1.0"
    component: ComponentMeta,
    parameter_values: Vec<ParamValue>,     // runtime restore (values only)
    parameter_metadata: Vec<ParameterMeta>, // export-only: full param definitions for LLM
    dsp: DspConfig,                // DSP source metadata (not runtime)
    dependencies: DependencyList,
    routing: RoutingConfig,
    verification_checks: Vec<VerificationCheckDef>,
    engineering_notes: String,     // PROSE: design intent, gain staging, saturation character
    signal_flow_description: String, // TEXTUAL DIAGRAM
    integration_guide: IntegrationGuide, // STRUCTURED: summary + steps + code snippet
}

struct ComponentMeta {
    name: String,                  // "MLC ZERO V"
    category: String,              // "amp-modeler"
    variant_id: String,
    version: String,
    status: String,                // "ready"
    created: String,
    author: String,
    description: String,
}
```

### IntegrationGuide (AI-readable export field)

```rust
#[derive(Serialize, Deserialize)]
struct IntegrationGuide {
    summary: String,
    /// "To integrate this amp model into a nih_plug project:"
    steps: Vec<String>,
    /// Each step is a concrete instruction
    rust_integration_example: String,
    /// Actual Rust code snippet showing how to wire params → DSP
    host_framework: String,
    /// "nih_plug 0.7"
    target_platforms: Vec<String>,
    /// ["linux", "macos", "windows"]
}

/// Export-only full parameter definition for an LLM to recreate.
/// Separate from ParamValue (which is values-only for runtime restore).
#[derive(Serialize, Deserialize)]
struct ParameterMeta {
    id: String,
    name: String,
    description: String,           // what this parameter controls, in prose
    range: (f32, f32),             // min, max
    default: f32,
    unit: Option<String>,          // "dB", "%", "ms"
    smoothing: String,             // "logarithmic(50ms)", "linear(10ms)"
    index: u32,                    // FFI parameter index (0-based)
}
```

### DspConfig (source metadata, not runtime)

```rust
enum DspEngine { Faust, Mojo, PureRust }

struct DspConfig {
    engine: DspEngine,
    language: String,
    source_files: Vec<SourceFile>,
    faust_libraries: Vec<FaustLib>,
    compile_command: String,       // exact faust CLI command
    ffi_interface: FfiInterface,
}

struct FfiInterface {
    init_fn: String,
    process_fn: String,
    cleanup_fn: String,
    param_count: u32,
    param_ids: Vec<String>,
    buffer_format: String,
}

struct SourceFile {
    path: String,
    hash: String,                  // SHA256
    description: String,
}
```

### PipelineConfig

```rust
struct PipelineConfig {
    name: String,
    version: String,
    slots: Vec<SlotConfig>,
    master_gain_db: f32,
}

struct SlotConfig {
    category_id: String,
    active_variant_id: Option<String>,
    bypassed: bool,
}
```

### VerificationCheck

```rust
#[derive(Serialize, Deserialize)]
struct VerificationCheckDef {
    id: String,
    check_order: u32,
    category: CheckCategory,    // Build | Audio | Params | UI | Deps
    check: String,
    expected: String,
    automated: bool,
}

enum CheckCategory { Build, Audio, Params, Ui, Deps }

struct CheckResult {
    check_id: String,
    status: CheckStatus,        // Pass | Fail | Pending
    result_note: Option<String>,
    duration_ms: u64,
}
```

---

## Database Schema (SQLite)

```sql
CREATE TABLE categories (
    id          TEXT PRIMARY KEY,
    name        TEXT NOT NULL UNIQUE,
    description TEXT,
    sort_order  INTEGER NOT NULL
);

CREATE TABLE nodes (
    id          TEXT PRIMARY KEY,
    category_id TEXT NOT NULL REFERENCES categories(id),
    name        TEXT NOT NULL,
    created_at  TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE(category_id)
);

CREATE TABLE variants (
    id          TEXT PRIMARY KEY,
    node_id     TEXT NOT NULL REFERENCES nodes(id),
    name        TEXT NOT NULL,
    impl_id     TEXT NOT NULL,             -- references VARIANT_REGISTRY entry
    status      TEXT NOT NULL DEFAULT 'draft',
    active_snapshot_id TEXT,               -- which snapshot is currently loaded
    created_at  TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE(node_id, name)
);

CREATE TABLE snapshots (
    id              TEXT PRIMARY KEY,
    variant_id      TEXT NOT NULL REFERENCES variants(id),
    version         TEXT NOT NULL,
    status          TEXT NOT NULL DEFAULT 'draft',  -- draft | testing | ready | deprecated
    variant_impl_id TEXT NOT NULL,                   -- which compiled-in DSP
    values_json     TEXT NOT NULL,                   -- serialized Vec<ParamValue> (runtime restore)
    data_json       TEXT NOT NULL,                   -- full SnapshotData JSON (export reconstruction)
    params_hash     TEXT NOT NULL,                   -- SHA256 of values_json
    created_at      TEXT NOT NULL DEFAULT (datetime('now')),
    ready_at        TEXT,                            -- when status became 'ready'
    notes           TEXT DEFAULT '',
    UNIQUE(variant_id, version)
);

CREATE TABLE dependencies (
    snapshot_id TEXT NOT NULL REFERENCES snapshots(id) ON DELETE CASCADE,
    dep_type    TEXT NOT NULL,
    dep_name    TEXT NOT NULL,
    dep_version TEXT,
    dep_hash    TEXT,
    PRIMARY KEY (snapshot_id, dep_type, dep_name)
);

CREATE TABLE pipelines (
    id          TEXT PRIMARY KEY,
    name        TEXT NOT NULL,
    config_json TEXT NOT NULL,
    created_at  TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at  TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE verification_checks (
    id           TEXT PRIMARY KEY,
    snapshot_id  TEXT NOT NULL REFERENCES snapshots(id) ON DELETE CASCADE,
    check_order  INTEGER NOT NULL,
    category     TEXT NOT NULL,
    check        TEXT NOT NULL,
    expected     TEXT NOT NULL,
    automated    INTEGER NOT NULL DEFAULT 0,
    status       TEXT NOT NULL DEFAULT 'pending',
    result_note  TEXT,
    checked_at   TEXT
);

CREATE INDEX idx_snapshots_variant ON snapshots(variant_id);
CREATE INDEX idx_variants_node ON variants(node_id);
CREATE INDEX idx_checks_snapshot ON verification_checks(snapshot_id);
```

**Storage location:** `~/.config/distortion/lab.db` (Linux), determined via `dirs::config_dir()`.

---

## Variant Switch Protocol (Mailbox+Trash)

```
User clicks "Switch to JCM900"
        │
        ▼
┌─ UI Thread ──────────────────────────────────┐
│ 1. node.request_switch("jcm900_v1")          │
│ 2. Look up factory in VariantRegistry        │
│ 3. spawn std::thread::spawn(build_task)      │
└──────────────────────────────────────────────┘
        │
        ▼
┌─ Background Thread ──────────────────────────┐
│ 4. Call variant_factory(sample_rate)         │
│    → Box<dyn DspVariant>                     │
│ 5. Wrap in VariantRuntime (with interior     │
│    mutability for process_block)             │
│ 6. node.install(Arc::new(runtime))           │
│    → posts to VariantSlot.mailbox            │
└──────────────────────────────────────────────┘
        │
        ▼
┌─ Audio Thread (next safe point) ─────────────┐
│ 7. slot.collect_mailbox()                    │
│    - Takes new runtime from mailbox          │
│    - ArcSwap::swap — old runtime → trash     │
│    - Audio continues uninterrupted           │
│    (Variant A was processing until swap)     │
└──────────────────────────────────────────────┘
        │
        ▼
┌─ UI Thread (periodic) ───────────────────────┐
│ 8. slot.collect_garbage()                    │
│    - Drains single-slot trash (ArcSwapOption) │
│    - Drops old Arc<VariantRuntime> here      │
│    (never on audio thread)                   │
└──────────────────────────────────────────────┘
```

**Audio thread access:** audio engine processes each node via `&mut self` (single-writer). `collect_mailbox()` called once per block at safe point — lock-free, alloc-free. Bypassed nodes pass audio through unchanged.

**Failure handling:** If step 4-5 fails, the error is sent back to the UI thread. The current variant stays active in the slot (no destructive unload). The mailbox is simply not populated with a new runtime.

---

## Export Bundle Structure

```
exports/<variant_name>-v<version>/
├── MANIFEST.json           # {files: [{path, sha256}], exported_at, variant, version}
├── variant.json            # snapshot AI-readable completo (SnapshotData serialized)
├── dsp/
│   ├── <name>.dsp          # fonte Faust
│   └── wrapper.{cpp,h}     # wrapper C ABI
├── neural/                 # (se Mojo)
│   └── <name>.mojo
├── deps/                   # libs locais referenciadas
├── templates/
│   ├── build.rs.template
│   ├── Cargo.toml.template
│   └── plugin.rs.template
├── VERIFICATION.md         # checklist formatada
└── README.md               # instruções
```

---

## Concrete variant.json Example (AI-Readable)

```json
{
  "format": "dsp-component-snapshot",
  "schema_version": "1.0",
  "component": {
    "name": "MLC ZERO V",
    "category": "amp-modeler",
    "variant_id": "mlc_zero_v_drive2",
    "version": "1.1.0",
    "status": "ready",
    "created": "2026-07-07T15:00:00Z",
    "author": "dev",
    "description": "High-gain amp model: 4 gain stages with Fender-style tonestack"
  },
  "engineering_notes": "The preamp uses 4 cascaded 12AX7 triode stages modeled via Faust's valve.lib. The tonestack is a classic Fender-style 3-band with a fixed mid scoop at 500Hz. Power amp sag is simulated with a 10ms envelope follower on the output. All 5 parameters use Logarithmic smoothing at 50ms.",
  "signal_flow_description": "Input → [Gate] → [Gain×4 stages (12AX7)] → [Tonestack (Bass/Mid/Treble)] → [Power Amp Sag (envelope)] → Output. All processing at project sample rate, single-pass, no oversampling.",
  "parameter_values": [
    {"id": "amp_gain", "value": 50.0},
    {"id": "amp_bass", "value": 0.0},
    {"id": "amp_mid", "value": -2.0},
    {"id": "amp_treble", "value": 2.0},
    {"id": "amp_master", "value": -6.0}
  ],
  "parameter_metadata": [
    {"id": "amp_gain", "name": "Amp Gain", "description": "Preamp gain driving 4 cascaded 12AX7 triode stages. 0 = clean, 50 = crunch, 100 = full saturation.", "range": [0.0, 100.0], "default": 50.0, "unit": "%", "smoothing": "logarithmic(50ms)", "index": 0},
    {"id": "amp_bass", "name": "Bass", "description": "Fender-style tonestack low shelf ±12dB at 100Hz.", "range": [-12.0, 12.0], "default": 0.0, "unit": "dB", "smoothing": "logarithmic(50ms)", "index": 1},
    {"id": "amp_mid", "name": "Mid", "description": "Tonestack mid band ±12dB at 500Hz with fixed scoop.", "range": [-12.0, 12.0], "default": 0.0, "unit": "dB", "smoothing": "logarithmic(50ms)", "index": 2},
    {"id": "amp_treble", "name": "Treble", "description": "Tonestack high shelf ±12dB at 8kHz.", "range": [-12.0, 12.0], "default": 0.0, "unit": "dB", "smoothing": "logarithmic(50ms)", "index": 3},
    {"id": "amp_master", "name": "Master", "description": "Output level after power amp sag stage.", "range": [-24.0, 6.0], "default": -6.0, "unit": "dB", "smoothing": "linear(10ms)", "index": 4}
  ],
  "dsp": {
    "engine": "Faust",
    "language": "faust",
    "source_files": [
      {"path": "dsp/mlc_zero_v.dsp", "hash": "sha256:abc123...", "description": "Main DSP: 4-stage preamp + tonestack + sag"}
    ],
    "faust_libraries": [
      {"name": "valve.lib", "version": "2.60.0"},
      {"name": "stdfaust.lib", "version": "2.60.0"}
    ],
    "compile_command": "faust -lang cpp -cn mlczerov -vec -I faust-ddsp dsp/mlc_zero_v.dsp -o dsp/MlcZeroVModule.hpp",
    "ffi_interface": {
      "init_fn": "mlc_zero_v_init",
      "process_fn": "mlc_zero_v_process_block",
      "cleanup_fn": "mlc_zero_v_cleanup",
      "param_count": 5,
      "param_ids": ["amp_gain", "amp_bass", "amp_mid", "amp_treble", "amp_master"],
      "buffer_format": "in_place_interleaved_stereo_f32"
    }
  },
  "dependencies": {
    "rust_crates": [
      {"name": "nih_plug", "version": "0.7", "features": []},
      {"name": "bindgen", "version": "0.71", "build_only": true}
    ],
    "system_tools": [
      {"name": "faust", "version_min": "2.60.0", "required": true}
    ]
  },
  "routing": {
    "audio_input": "stereo",
    "audio_output": "stereo",
    "latency_samples": 0
  },
  "integration_guide": {
    "summary": "To integrate this amp model into a nih_plug project:",
    "host_framework": "nih_plug 0.7",
    "target_platforms": ["linux", "macos", "windows"],
    "steps": [
      "1. Copy dsp/mlc_zero_v.dsp to your project's dsp/ directory",
      "2. Add Faust compile step to build.rs: faust -lang cpp -cn mlczerov -vec dsp/mlc_zero_v.dsp -o dsp/MlcZeroVModule.hpp",
      "3. Add wrapper C ABI files (wrapper.cpp/h) — see templates/",
      "4. Use cc::Build + bindgen in build.rs to link the Faust DSP",
      "5. Add FloatParam entries to your Params struct for amp_gain, amp_bass, amp_mid, amp_treble, amp_master",
      "6. In process(), call mlc_zero_v_process_block(buf_ptr, num_samples) per channel",
      "7. Wire each param.smoothed.next() to the Faust parameter by index"
    ],
    "rust_integration_example": "// In BaseIOParams:\n#[id = \"amp_gain\"]\npub amp_gain: FloatParam,\n// ...\n\n// In process():\nfaust_set_param(0, self.params.amp_gain.smoothed.next());\nfaust_set_param(1, self.params.amp_bass.smoothed.next());\n// ...\nfaust_process_block(left_ptr, num_samples);\nfaust_process_block(right_ptr, num_samples);\n"
  },
  "verification_checks": [
    {"id": 1, "check_order": 1, "category": "Params", "check": "5 parameter values match expected IDs", "expected": "ids=[amp_gain,amp_bass,amp_mid,amp_treble,amp_master]", "automated": true},
    {"id": 2, "check_order": 2, "category": "Audio", "check": "null input produces silence", "expected": "RMS < -96dBFS at 48kHz, 1024 samples", "automated": true},
    {"id": 3, "check_order": 3, "category": "Audio", "check": "sine sweep no NaN/Inf", "expected": "all samples finite for 20Hz-20kHz sweep", "automated": true},
    {"id": 4, "check_order": 4, "category": "Build", "check": "DSP source hashes match snapshot", "expected": "sha256 matches stored hashes", "automated": true},
    {"id": 5, "check_order": 5, "category": "Audio", "check": "tonestack frequency response matches design", "expected": "bass ±12dB at 100Hz, mid ±12dB at 500Hz, treble ±12dB at 8kHz", "automated": false},
    {"id": 6, "check_order": 6, "category": "Audio", "check": "gain at 100% produces audible distortion", "expected": "THD > 10% with 1kHz sine at -6dBFS input", "automated": false}
  ]
}
```

---

## Error Handling Strategy

| Error Scenario | Handling | User Impact |
|---|---|---|
| SQLite connection fails | Recreate DB from scratch, log warning | Fresh lab (existing snapshots in git are safe) |
| Snapshot JSON parse error | Return `Err(LabError::CorruptedSnapshot)`, skip loading | Error toast in UI |
| Variant factory not found in registry | Return `Err(LabError::VariantNotFound)`, keep current active | Error toast, current variant stays active |
| DSP initialization fails (background thread) | Keep current variant active, log error | Error toast; no audio interruption |
| DSP source file missing at verify time | Verification check fails with path | Red X in checklist |
| Export destination unwritable | Return `Err(LabError::ExportFailed)` | Error toast, retry dialog |
| Variant switch while already switching | Queue second request | UI shows "queued" indicator |
| Disk full during snapshot save | Rollback SQLite transaction, return error | Error toast, no partial snapshot saved |

**FFI safety note:** The existing pipeline's stage 7 NaN sanitizer catches bad DSP output. We do NOT use `catch_unwind` at FFI boundaries (UB risk). Instead, we validate DSP output after processing and log anomalies.

---

## Disk Footprint

| Component | Expected Size | Growth Rate |
|---|---|---|
| SQLite DB (empty) | ~100 KB | Schema + indexes |
| Per snapshot (JSON) | ~5-15 KB | Linear with param count |
| Per snapshot (with DSP source copies) | ~50-500 KB | Depends on .dsp/.mojo file sizes |
| Export .tar.gz bundle | ~100 KB - 10 MB | Source + templates |
| Total for 100 snapshots | ~10-50 MB | Manageable |

**Retention:** No automatic pruning in V1. Snapshots are git-committed JSON files. The SQLite DB acts as a fast local cache. Users manually delete old snapshots. Export temp dirs are cleaned up after bundle creation.

**Build impact:** The 6 new crates (sha2, semver, chrono, uuid, flate2, tar) add ~2-5 MB to compile artifacts. Prefer `cargo check` during development; full `--release` builds only for integration testing.

---

## Tech Decisions

| Decision | Choice | Rationale |
|---|---|---|
| DSP compilation model | All pre-compiled in binary; snapshots select | `build.rs` runs at compile time only; runtime Fausto/Mojo not feasible |
| Audio thread hand-off | CabinetEngine mailbox pattern exactly (ArcSwapOption inbox+trash, no Mutex) | Proven, lock-free, no audio-thread deallocation or allocation |
| Buffer model | In-place `*mut f32` (matches ExternalProcessor) | Zero-copy FFI; no allocation on audio thread |
| Snapshot param model | Values only (not NIH definitions) | Plugin params are static `#[derive(Params)]`; hosts require stable layout |
| DB engine | SQLite (bundled rusqlite) with WAL journal | Already in Cargo.toml; single-file; same pattern as CabinetLibrary |
| Variant switching | Build → mailbox → ArcSwap::swap → trash → UI drop | No audio interruption; no destructive unload; old DSP dropped off audio thread |
| Hashing algorithm | SHA256 exclusively (sha2 crate) | Standard; MANIFEST compatibility; deterministic |
| Snapshot format | JSON (serde) with concrete IntegrationGuide schema | AI-readable; human-editable; already in stack |
| Bundle compression | `.tar.gz` (flate2 + tar) | Universal format; no external tools for extraction |
| Versioning | SemVer (semver crate), auto-increment PATCH | Standard; predictable |
| Category enforcement | BTreeMap keyed by category_id + amp exclusivity group | Deterministic ordering; prevents amp-modeler + amp-capture both active |
| DB access model | `Mutex<Database>`, used only off audio thread | Prevents SQLite blocking audio; background threads share via Mutex |
| Global state | Single `~/.config/distortion/lab.db` for all instances | Documented limitation; per-instance state is P2 |
| Export gating | "Export" for ready snapshots; "Export Draft" (with warning) for others | Safety + flexibility; draft bundles marked as draft |

---

## New Rust Dependencies

```toml
[dependencies]
sha2 = "0.10"
semver = "1"
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1", features = ["v4", "serde"] }
flate2 = "1"
tar = "0.4"
```

---

## Config File Adjustments

### .specs/project/STATE.md
- Decisions D-011 through D-016 documented (D-016 added: Pre-compiled DSP model, D-017: Mailbox+trash audio pattern)

### .specs/project/ROADMAP.md
- v0.3 section with LAB-01 through LAB-16

### .specs/codebase/ARCHITECTURE.md
- `src/lab/` module documented with mailbox+trash pattern, VariantRegistry, DspVariant trait
