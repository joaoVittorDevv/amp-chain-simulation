use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Stream;
use distortion::core::audio_config::{
    pick_config, pick_full_duplex, reconcile_sample_rate, PickedConfig, StreamDirection,
    FALLBACK_MAX_BLOCK,
};
use distortion::core::audio_status::{AudioStatus, ErrorKind};
use distortion::core::cabinet::{CabinetLibrary, CabinetMailbox, CabinetRuntime};
use distortion::core::device_context::{DeviceContext, Direction};
use distortion::core::device_identity::resolve_device;
use distortion::core::dsp::rt_resampler::RtResampler;
use distortion::core::dsp::{
    process_interleaved_block, sample_convert, AnalyzerDsp, AudioSnapshot, StandalonePipeline,
    FFT_SIZE,
};
use distortion::core::state::plugin_params::{
    AmpModel, ClipType, MlcAdaaOrder, MlcBright, MlcFeedback, MlcGatePos, MlcTab, MlcTsModel,
    MlcTubeModel,
};
#[cfg(feature = "lab")]
use distortion::core::ui::{draw_lab_panel, LabUiState};
use distortion::core::ui::{render_shared_panels, ActivePanel};
#[cfg(feature = "lab")]
use distortion::lab::Lab;
use eframe::egui;
use nih_plug::prelude::Enum;
use rtrb::{Consumer, Producer, RingBuffer};
use std::collections::VecDeque;
use std::sync::atomic::{AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

/// Default cabinet IR embedded in the standalone binary (relative to src/bin/).
const DEFAULT_CABINET_IR: &[u8] = include_bytes!("../../neural/drive/cabinet_ir.wav");

/// Pre-EQ (tone-stack) IR embedded in the standalone binary. Replaces the old
/// hardcoded absolute path so the linear pre-EQ stage no longer depends on a file.
const DEFAULT_PRE_EQ_IR: &[u8] = include_bytes!("../../neural/drive/pre_eq_ir.wav");

#[cfg(feature = "lab")]
fn lab_data_dir() -> Result<std::path::PathBuf, distortion::lab::LabError> {
    Ok(dirs::config_dir()
        .ok_or(distortion::lab::LabError::ConfigDirUnavailable)?
        .join("distortion"))
}

/// Decode embedded WAV bytes to a flat f32 sample vector (i16 or f32), failing on
/// any decode error. Used for the fixed pre-EQ IR.
fn decode_wav_flat(bytes: &[u8]) -> Option<Vec<f32>> {
    let mut reader = hound::WavReader::new(std::io::Cursor::new(bytes)).ok()?;
    let spec = reader.spec();
    match spec.sample_format {
        hound::SampleFormat::Float => reader.samples::<f32>().collect::<Result<Vec<_>, _>>().ok(),
        hound::SampleFormat::Int => {
            let max = (1i64 << (spec.bits_per_sample - 1)) as f32;
            reader
                .samples::<i32>()
                .collect::<Result<Vec<_>, _>>()
                .ok()
                .map(|v| v.into_iter().map(|s| s as f32 / max).collect())
        }
    }
}

#[derive(Clone)]
struct StandaloneState {
    eq_active: bool,
    eq_low_freq: f32,
    eq_low_gain: f32,
    eq_low_q: f32,
    eq_mid_freq: f32,
    eq_mid_gain: f32,
    eq_mid_q: f32,
    eq_high_freq: f32,
    eq_high_gain: f32,
    eq_high_q: f32,
    neural_active: bool,
    neural_drive: f32,
    neural_output_gain: f32,
    neural_amp_volume: f32,
    amp_model: AmpModel,
    mlc_gain: f32,
    mlc_master: f32,
    mlc_bass: f32,
    mlc_middle: f32,
    mlc_treble: f32,
    mlc_presence: f32,
    mlc_depth: f32,
    mlc_gate: f32,
    mlc_bright: MlcBright,
    mlc_m45: bool,
    mlc_warclaw: bool,
    mlc_feedback: MlcFeedback,
    mlc_gate_pos: MlcGatePos,
    mlc_clip_type1: ClipType,
    mlc_clip_type2: ClipType,
    mlc_clip_type3: ClipType,
    mlc_clean_blend: f32,
    mlc_sag: f32,
    mlc_h2: f32,
    mlc_h3: f32,
    mlc_h4: f32,
    mlc_tight: bool,
    mlc_asymmetry_enable: bool,
    mlc_asymmetry: f32,
    mlc_preshape: bool,
    mlc_preshape_tight: f32,
    mlc_preshape_bite: f32,
    // --- Tier 2.2 / 3.x additions ---
    mlc_ts_model: MlcTsModel,
    mlc_tube_model: MlcTubeModel,
    mlc_tube_drive: f32,
    mlc_tube_bypass: bool,
    mlc_nfb_presence: f32,
    mlc_nfb_resonance: f32,
    mlc_nfb_depth: f32,
    mlc_nfb_bypass: bool,
    mlc_mbc_bypass: bool,
    mlc_mbc_cf_lo: f32,
    mlc_mbc_cf_hi: f32,
    mlc_mbc_drive_lo: f32,
    mlc_mbc_drive_mid: f32,
    mlc_mbc_drive_hi: f32,
    mlc_adaa_order: MlcAdaaOrder,
    eq_tanh_bypass: bool,
    gain: f32,
    bypass: bool,
    // --- Cabinet IR ---
    cabinet_bypass: bool,
    cabinet_level: f32,
    cabinet_mix: f32,
    cab_active_hash: String,
    // --- Brickwall Limiter ---
    limiter_enable: bool,
    limiter_ceiling: f32,
    limiter_release: f32,
}

impl Default for StandaloneState {
    fn default() -> Self {
        Self {
            eq_active: true,
            eq_low_freq: 100.0,
            eq_low_gain: 0.0,
            eq_low_q: 0.707,
            eq_mid_freq: 1000.0,
            eq_mid_gain: 0.0,
            eq_mid_q: 1.0,
            eq_high_freq: 5000.0,
            eq_high_gain: 0.0,
            eq_high_q: 0.707,
            neural_active: true,
            neural_drive: 1.0,
            neural_output_gain: 1.0,
            neural_amp_volume: 1.0,
            amp_model: AmpModel::Neural,
            mlc_gain: 0.25118864,
            mlc_master: 0.5011872,
            mlc_bass: 0.0,
            mlc_middle: 0.0,
            mlc_treble: 0.0,
            mlc_presence: 0.0,
            mlc_depth: 0.0,
            mlc_gate: -80.0,
            mlc_bright: MlcBright::Ii,
            mlc_m45: false,
            mlc_warclaw: false,
            mlc_feedback: MlcFeedback::Hi,
            mlc_gate_pos: MlcGatePos::Pre,
            mlc_clip_type1: ClipType::AsymmetricTanh,
            mlc_clip_type2: ClipType::AsymmetricTanh,
            mlc_clip_type3: ClipType::Exponential,
            mlc_clean_blend: 0.0,
            mlc_sag: 0.0,
            mlc_h2: 0.0,
            mlc_h3: 0.7,
            mlc_h4: 0.2,
            mlc_tight: true,
            mlc_asymmetry_enable: true,
            mlc_asymmetry: 0.5,
            mlc_preshape: false,
            mlc_preshape_tight: -3.0,
            mlc_preshape_bite: 3.0,
            mlc_ts_model: MlcTsModel::Bassman,
            mlc_tube_model: MlcTubeModel::Ax7T1,
            mlc_tube_drive: 0.0,
            mlc_tube_bypass: true,
            mlc_nfb_presence: 0.0,
            mlc_nfb_resonance: 0.0,
            mlc_nfb_depth: 0.7,
            mlc_nfb_bypass: true,
            mlc_mbc_bypass: true,
            mlc_mbc_cf_lo: 300.0,
            mlc_mbc_cf_hi: 3000.0,
            mlc_mbc_drive_lo: 1.0,
            mlc_mbc_drive_mid: 1.0,
            mlc_mbc_drive_hi: 1.0,
            mlc_adaa_order: MlcAdaaOrder::Off,
            eq_tanh_bypass: false,
            gain: 1.0,
            bypass: false,
            cabinet_bypass: false,
            cabinet_level: 1.0,
            cabinet_mix: 1.0,
            cab_active_hash: String::new(),
            limiter_enable: true,
            limiter_ceiling: -0.2,
            limiter_release: 50.0,
        }
    }
}

impl StandaloneState {
    fn audio(&self) -> AudioSnapshot {
        AudioSnapshot {
            eq_active: self.eq_active,
            eq_low_freq: self.eq_low_freq,
            eq_low_gain: self.eq_low_gain,
            eq_low_q: self.eq_low_q,
            eq_mid_freq: self.eq_mid_freq,
            eq_mid_gain: self.eq_mid_gain,
            eq_mid_q: self.eq_mid_q,
            eq_high_freq: self.eq_high_freq,
            eq_high_gain: self.eq_high_gain,
            eq_high_q: self.eq_high_q,
            neural_active: self.neural_active,
            neural_drive: self.neural_drive,
            neural_output_gain: self.neural_output_gain,
            neural_amp_volume: self.neural_amp_volume,
            amp_model: self.amp_model,
            mlc_gain: self.mlc_gain,
            mlc_master: self.mlc_master,
            mlc_bass: self.mlc_bass,
            mlc_middle: self.mlc_middle,
            mlc_treble: self.mlc_treble,
            mlc_presence: self.mlc_presence,
            mlc_depth: self.mlc_depth,
            mlc_gate: self.mlc_gate,
            mlc_bright: self.mlc_bright,
            mlc_m45: self.mlc_m45,
            mlc_warclaw: self.mlc_warclaw,
            mlc_feedback: self.mlc_feedback,
            mlc_gate_pos: self.mlc_gate_pos,
            mlc_clip_type1: self.mlc_clip_type1,
            mlc_clip_type2: self.mlc_clip_type2,
            mlc_clip_type3: self.mlc_clip_type3,
            mlc_clean_blend: self.mlc_clean_blend,
            mlc_sag: self.mlc_sag,
            mlc_h2: self.mlc_h2,
            mlc_h3: self.mlc_h3,
            mlc_h4: self.mlc_h4,
            mlc_tight: self.mlc_tight,
            mlc_asymmetry_enable: self.mlc_asymmetry_enable,
            mlc_asymmetry: self.mlc_asymmetry,
            mlc_preshape: self.mlc_preshape,
            mlc_preshape_tight: self.mlc_preshape_tight,
            mlc_preshape_bite: self.mlc_preshape_bite,
            mlc_ts_model: self.mlc_ts_model,
            mlc_tube_model: self.mlc_tube_model,
            mlc_tube_drive: self.mlc_tube_drive,
            mlc_tube_bypass: self.mlc_tube_bypass,
            mlc_nfb_presence: self.mlc_nfb_presence,
            mlc_nfb_resonance: self.mlc_nfb_resonance,
            mlc_nfb_depth: self.mlc_nfb_depth,
            mlc_nfb_bypass: self.mlc_nfb_bypass,
            mlc_mbc_bypass: self.mlc_mbc_bypass,
            mlc_mbc_cf_lo: self.mlc_mbc_cf_lo,
            mlc_mbc_cf_hi: self.mlc_mbc_cf_hi,
            mlc_mbc_drive_lo: self.mlc_mbc_drive_lo,
            mlc_mbc_drive_mid: self.mlc_mbc_drive_mid,
            mlc_mbc_drive_hi: self.mlc_mbc_drive_hi,
            mlc_adaa_order: self.mlc_adaa_order,
            eq_tanh_bypass: self.eq_tanh_bypass,
            gain: self.gain,
            bypass: self.bypass,
            cabinet_bypass: self.cabinet_bypass,
            cabinet_level: self.cabinet_level,
            cabinet_mix: self.cabinet_mix,
            limiter_enable: self.limiter_enable,
            limiter_ceiling: self.limiter_ceiling,
            limiter_release: self.limiter_release,
        }
    }
}

/// Open the standalone's cabinet library (same location as the plugin) and seed
/// the embedded default IR when empty.
fn open_cabinet_library() -> CabinetLibrary {
    let lib = dirs::data_dir()
        .map(|d| d.join("distortion"))
        .and_then(|dir| {
            std::fs::create_dir_all(&dir).ok()?;
            CabinetLibrary::new(&dir.join("cabinet_irs.db")).ok()
        })
        .or_else(|| CabinetLibrary::new(std::path::Path::new(":memory:")).ok())
        .expect("failed to open cabinet library (even in-memory)");
    let _ = lib.seed_default_ir(DEFAULT_CABINET_IR);
    lib
}

/// Path to the standalone selection persistence file.
fn standalone_config_path() -> Option<std::path::PathBuf> {
    dirs::config_dir().map(|d| d.join("distortion").join("standalone.json"))
}

/// Load the persisted active cabinet hash, if any.
fn load_persisted_hash() -> Option<String> {
    let path = standalone_config_path()?;
    let data = std::fs::read_to_string(path).ok()?;
    let json: serde_json::Value = serde_json::from_str(&data).ok()?;
    json.get("cab_active_hash")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

/// Persist the active cabinet hash to `standalone.json`, merging into any
/// existing config so unrelated keys are preserved (N2).
fn save_persisted_hash(hash: &str) {
    if let Some(path) = standalone_config_path() {
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        // Load-and-merge rather than overwrite the whole document.
        let mut json = std::fs::read_to_string(&path)
            .ok()
            .and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok())
            .filter(|v| v.is_object())
            .unwrap_or_else(|| serde_json::json!({}));
        json["cab_active_hash"] = serde_json::Value::String(hash.to_string());
        let _ = std::fs::write(
            path,
            serde_json::to_string_pretty(&json).unwrap_or_default(),
        );
    }
}

/// A device the user routed audio through, identified well enough to be found
/// again in a later enumeration. See `core::device_identity::resolve_device`.
#[derive(Clone)]
struct DeviceSelection {
    raw_name: String,
    enum_index: usize,
    left: usize,
    right: usize,
}

enum AudioCommand {
    RefreshDevices(cpal::HostId),
    ApplyRouting {
        host_id: cpal::HostId,
        input: Option<DeviceSelection>,
        output: Option<DeviceSelection>,
        buffer_size: u32,
    },
    Stop,
}

enum AudioEvent {
    DevicesRefreshed {
        inputs: Vec<DeviceContext>,
        outputs: Vec<DeviceContext>,
    },
    StreamStarted {
        result: Result<(), String>,
        sample_rate_warning: Option<String>,
    },
}

struct StandaloneApp {
    analyzer: AnalyzerDsp,
    consumer: Arc<Mutex<Option<Consumer<f32>>>>,

    hosts: Vec<cpal::HostId>,
    selected_host: cpal::HostId,

    available_inputs: Vec<DeviceContext>,
    selected_input_idx: Option<usize>,
    input_left: usize,
    input_right: usize,

    available_outputs: Vec<DeviceContext>,
    selected_output_idx: Option<usize>,
    output_left: usize,
    output_right: usize,

    buffer_power: u32,
    buffer_range_min: u32,
    buffer_range_max: u32,

    latency_stats: Arc<LatencyStats>,

    sample_rate_warning: Option<String>,
    /// Lock-free channel for stream errors raised on the audio thread (T23).
    audio_status: Arc<AudioStatus>,
    last_audio_error: Option<String>,
    last_audio_error_details: Option<String>,
    show_error_popup: bool,

    /// underruns+overflows total last seen by the UI; detects new glitch
    /// episodes and counter resets (a new routing zeroes the counters).
    prev_glitch_total: u64,
    /// Timestamps of recent glitch episodes (rolling 60 s window). Three or
    /// more within the window raise the clock-drift warning banner.
    recent_glitches: VecDeque<Instant>,

    input_is_mono: bool,
    output_is_mono: bool,

    show_settings: bool,
    is_loading: bool,
    active_panel: ActivePanel,

    /// Currently selected tab of the MLC ZERO V panel (UI-thread only).
    mlc_tab: MlcTab,

    standalone_state: Arc<Mutex<StandaloneState>>,

    // --- Cabinet IR (UI-thread handles) ---
    cabinet_library: Arc<Mutex<CabinetLibrary>>,
    cabinet_mailbox: Arc<CabinetMailbox>,
    cabinet_sr: Arc<AtomicU32>,
    cabinet_max_block: Arc<AtomicUsize>,
    cabinet_error: Arc<Mutex<Option<String>>>,

    #[cfg(feature = "lab")]
    lab: Option<Arc<Lab>>,
    #[cfg(feature = "lab")]
    lab_error: Arc<Mutex<Option<String>>>,
    #[cfg(feature = "lab")]
    lab_ui: LabUiState,

    tx_cmd: Sender<AudioCommand>,
    rx_event: Receiver<AudioEvent>,
}

fn beautify_linux_name(raw: &str, is_input: bool) -> (String, bool) {
    if raw.starts_with("jack") || raw.contains("pipewire") || raw.contains("pulse") {
        return ("Servidor Áudio: JACK / PipeWire".to_string(), true);
    }

    if raw.starts_with("sysdefault:CARD=") || raw.starts_with("hw:CARD=") {
        let parts: Vec<&str> = raw.split("CARD=").collect();
        if parts.len() > 1 {
            let sub: Vec<&str> = parts[1].split(',').collect();
            let card_name = sub[0];
            let prefix = if raw.starts_with("sysdefault") {
                "(Padrão)"
            } else {
                "(Hardware Direto)"
            };
            let icon = if is_input { "🎙️" } else { "🔊" };
            return (
                format!("{} {} {}", icon, card_name.replace("_", " "), prefix),
                false,
            );
        }
    }
    (format!("{} (Cru)", raw), false)
}

/// One row of a device ComboBox. Returns `true` when the row was clicked.
///
/// A device the host cannot open stays on the list — greyed out, suffixed, and
/// explaining itself on hover — rather than silently disappearing (CROSS-16).
/// It is never selectable, so callers can assume a click means a usable device.
fn device_entry(ui: &mut egui::Ui, dev: &DeviceContext, selected: bool) -> bool {
    if dev.usable {
        return ui.selectable_label(selected, &dev.name).clicked();
    }

    let reason = dev
        .unusable_reason
        .as_deref()
        .unwrap_or("Dispositivo indisponível.");
    let label = egui::SelectableLabel::new(false, format!("{} — indisponível", dev.name));
    ui.add_enabled(false, label).on_disabled_hover_text(reason);
    false
}

/// Distinct sample formats the host reports for `dev`, in enumeration order.
///
/// A host that refuses to enumerate ranges yields an empty list rather than an
/// error: the formats only enrich the tooltip on an unusable device, so failing
/// to read them must not itself hide the device.
fn enumerate_sample_formats(dev: &cpal::Device, direction: Direction) -> Vec<cpal::SampleFormat> {
    let ranges: Vec<cpal::SupportedStreamConfigRange> = if direction.is_input() {
        dev.supported_input_configs()
            .map(|it| it.collect())
            .unwrap_or_default()
    } else {
        dev.supported_output_configs()
            .map(|it| it.collect())
            .unwrap_or_default()
    };

    let mut formats = Vec::new();
    for range in ranges {
        let format = range.sample_format();
        if !formats.contains(&format) {
            formats.push(format);
        }
    }
    formats
}

/// Re-point a picker selection that a refresh invalidated.
///
/// Device lists are rebuilt from scratch on each refresh, so an index that named
/// a working device can come back naming an unusable one — or naming nothing at
/// all, if the list shrank. Routing already skips such a selection, but the
/// picker would go on drawing it as selected, with channel controls sized from
/// its now-zero channel count. Fall back to the first usable device, or to
/// `None` when there is none.
///
/// An existing `None` is left alone: that is the user's "disabled" choice, not
/// a stale index.
fn revalidate_selection(selected: &mut Option<usize>, devices: &[DeviceContext]) {
    let Some(idx) = *selected else { return };
    if !devices.get(idx).is_some_and(|d| d.usable) {
        *selected = devices.iter().position(|d| d.usable);
    }
}

/// Resolve the single `Device` that ASIO's input and output streams must share (CROSS-15).
///
/// An ASIO driver owns exactly one input/output stream pair behind a shared
/// `Arc<Mutex<AsioStreams>>`. Enumerating the driver twice — once via
/// `input_devices()`, once via `output_devices()` — yields two `Device`s whose Arcs
/// are independent, so building the second stream tears the first one down. Cloning
/// one `Device` keeps both streams on the same Arc.
///
/// Returns `None` on every other host, and when only one direction is routed, so the
/// per-direction lookup stays exactly as it was.
fn asio_duplex_device(
    host: &cpal::Host,
    host_id: cpal::HostId,
    input: &Option<DeviceSelection>,
    output: &Option<DeviceSelection>,
) -> Option<cpal::Device> {
    // `HostId::Asio` is only generated for `all(windows, feature = "asio")`, so the
    // body cannot even name it elsewhere.
    #[cfg(all(target_os = "windows", feature = "asio"))]
    {
        if host_id != cpal::HostId::Asio {
            return None;
        }
        let in_name = &input.as_ref()?.raw_name;
        let out_name = &output.as_ref()?.raw_name;
        if in_name != out_name {
            eprintln!(
                "[Audio Engine] ASIO expõe um único device duplex; entrada {in_name:?} e \
                 saída {out_name:?} diferem — usando {out_name:?} para ambas."
            );
        }
        host.output_devices()
            .ok()?
            .find(|d| d.name().unwrap_or_default() == *out_name)
    }
    #[cfg(not(all(target_os = "windows", feature = "asio")))]
    {
        let _ = (host, host_id, input, output);
        None
    }
}

/// Configs for the streams we are about to open, or the message explaining why
/// we will not open them.
type NegotiatedConfigs = Result<(Vec<PickedConfig>, Vec<PickedConfig>), String>;

/// Maps a live cpal stream error into the lock-free `AudioStatus` taxonomy so the
/// audio thread can flag it without allocating or blocking (T23).
fn stream_error_kind(err: cpal::StreamError) -> ErrorKind {
    match err {
        cpal::StreamError::DeviceNotAvailable => ErrorKind::DeviceDisconnected,
        cpal::StreamError::BackendSpecific { .. } => ErrorKind::StreamError,
    }
}

/// Shared handle to a ring producer written from an audio callback.
type SharedProducer = Arc<Mutex<Producer<f32>>>;

/// Lock-free latency telemetry: each audio callback publishes what it actually
/// observes (block size, rate, ring fill) and the UI composes the end-to-end
/// figure. Values stay zero until the corresponding stream reports in.
struct LatencyStats {
    input_frames: AtomicU64,
    input_rate: AtomicU64,
    ring_fill_samples: AtomicU64,
    output_frames: AtomicU64,
    output_rate: AtomicU64,
}

impl LatencyStats {
    const fn new() -> Self {
        Self {
            input_frames: AtomicU64::new(0),
            input_rate: AtomicU64::new(0),
            ring_fill_samples: AtomicU64::new(0),
            output_frames: AtomicU64::new(0),
            output_rate: AtomicU64::new(0),
        }
    }

    fn reset(&self) {
        self.input_frames.store(0, Ordering::Relaxed);
        self.input_rate.store(0, Ordering::Relaxed);
        self.ring_fill_samples.store(0, Ordering::Relaxed);
        self.output_frames.store(0, Ordering::Relaxed);
        self.output_rate.store(0, Ordering::Relaxed);
    }
}

/// Pops one stereo frame from the playthrough ring, or silence while the ring
/// refills after an underrun.
///
/// An empty ring flips `refilling` on: one underrun episode is counted, and the
/// callback outputs silence until the fill recovers to the target slack. This
/// turns clock drift (input clock slower than output) into a single bounded gap
/// that also restores the latency target, instead of a persistent crackle.
fn pop_playthrough_frame(
    consumer: &mut Consumer<f32>,
    refilling: &mut bool,
    target_fill_samples: usize,
    status: &AudioStatus,
) -> (f32, f32) {
    if *refilling {
        if consumer.slots() >= target_fill_samples {
            *refilling = false;
        } else {
            return (0.0, 0.0);
        }
    }
    if consumer.slots() < 2 {
        status.note_underrun();
        *refilling = true;
        return (0.0, 0.0);
    }
    (
        consumer.pop().unwrap_or(0.0),
        consumer.pop().unwrap_or(0.0),
    )
}

/// Clock-drift safety net for the opposite direction: input clock faster than
/// output makes the ring fill — and thus the latency — creep upward without
/// bound. Once the fill passes 3× the target slack, drop the excess back down
/// to the target in one bounded skip and count one overflow episode.
///
/// Only whole frames are dropped (even sample count) so L/R never swap.
fn trim_playthrough_excess(
    consumer: &mut Consumer<f32>,
    target_fill_samples: usize,
    status: &AudioStatus,
) {
    let fill = consumer.slots();
    if fill > target_fill_samples.saturating_mul(3) {
        let excess = (fill - target_fill_samples) & !1;
        for _ in 0..excess {
            if consumer.pop().is_err() {
                break;
            }
        }
        status.note_overflow();
    }
}

/// Lock-free handoff of ring-fill measurements from the output callback to the
/// input callback that owns the resampler.
struct DriftObservation {
    fill_ratio_bits: AtomicU64,
    sequence: AtomicU64,
}

impl DriftObservation {
    fn new() -> Self {
        Self {
            fill_ratio_bits: AtomicU64::new(0.5f64.to_bits()),
            sequence: AtomicU64::new(0),
        }
    }

    fn publish(&self, fill_ratio: f64) {
        self.fill_ratio_bits
            .store(fill_ratio.to_bits(), Ordering::Relaxed);
        self.sequence.fetch_add(1, Ordering::Release);
    }

    fn apply_to(&self, resampler: &mut Option<RtResampler>, last_sequence: &mut u64) {
        let sequence = self.sequence.load(Ordering::Acquire);
        if sequence == *last_sequence {
            return;
        }

        let fill_ratio = f64::from_bits(self.fill_ratio_bits.load(Ordering::Relaxed));
        if let Some(resampler) = resampler {
            resampler.trim_ratio(fill_ratio);
        }
        *last_sequence = sequence;
    }
}

/// Publish a ring-fill measurement after approximately one second of output.
///
/// The measurement is normalized so that 0.5 means "fill equals the target
/// slack": the drift controller in `RtResampler::trim_ratio` steers toward 0.5,
/// so this keeps the steady-state latency at the user-chosen slack instead of
/// half the (much larger) ring capacity.
fn observe_playthrough_fill(
    fill_samples: usize,
    target_fill_samples: usize,
    frames_processed: usize,
    sample_rate: u32,
    frames_since_observation: &mut usize,
    observation: &DriftObservation,
) {
    *frames_since_observation += frames_processed;
    if *frames_since_observation < sample_rate as usize {
        return;
    }
    *frames_since_observation %= sample_rate as usize;

    if target_fill_samples == 0 {
        return;
    }
    let fill_norm = fill_samples as f64 / (2.0 * target_fill_samples as f64);
    observation.publish(fill_norm);
}

/// Publishes one block of processed frames: the mono mix to the analyzer ring,
/// and — when playthrough is active — interleaved L/R to the output ring.
///
/// Both rings are lock-free; `try_lock` only guards the `Arc<Mutex<..>>` handle
/// that lets a callback own its producer, and a contended or full ring drops the
/// block rather than blocking the audio thread.
fn publish_frames(
    analyzer: &SharedProducer,
    playthrough: &Option<SharedProducer>,
    audio_status: &AudioStatus,
    out_l: &[f32],
    out_r: &[f32],
) {
    for i in 0..out_l.len().min(out_r.len()) {
        let mix = (out_l[i] + out_r[i]) * 0.5;
        if let Ok(mut ap) = analyzer.try_lock() {
            // A full analyzer ring only drops visualization data — the UI thread
            // drains it at its own pace — so it does not count as an audio-path
            // overflow.
            let _ = ap.push(mix);
        }
        if let Some(ref pp) = playthrough {
            if let Ok(mut p) = pp.try_lock() {
                // Push L/R only as a pair: a half-written frame would swap the
                // channels of everything that follows.
                if p.slots() >= 2 {
                    let _ = p.push(out_l[i]);
                    let _ = p.push(out_r[i]);
                } else {
                    audio_status.note_overflow();
                }
            }
        }
    }
}

/// Emits one processed block downstream (T22).
///
/// The output device is the master clock. When the input device could not be
/// opened at the output's rate, `resampler` is `Some` and the block is converted
/// to the output rate before it reaches the rings — the output callback drains
/// the playthrough ring one frame per output frame, so anything pushed at the
/// input rate would otherwise play back pitch- and speed-shifted.
///
/// `RtResampler` emits on chunk boundaries, so a given call may publish several
/// chunks, exactly one, or none at all while it stages a partial chunk.
fn emit_processed_block(
    resampler: &mut Option<RtResampler>,
    analyzer: &SharedProducer,
    playthrough: &Option<SharedProducer>,
    audio_status: &AudioStatus,
    out_l: &[f32],
    out_r: &[f32],
) {
    match resampler {
        Some(rs) => rs.feed(out_l, out_r, |res_l, res_r| {
            publish_frames(analyzer, playthrough, audio_status, res_l, res_r)
        }),
        None => publish_frames(analyzer, playthrough, audio_status, out_l, out_r),
    }
}

fn audio_worker(
    rx_cmd: Receiver<AudioCommand>,
    tx_event: Sender<AudioEvent>,
    consumer_mutex: Arc<Mutex<Option<Consumer<f32>>>>,
    standalone_state: Arc<Mutex<StandaloneState>>,
    cabinet_mailbox: Arc<CabinetMailbox>,
    cabinet_library: Arc<Mutex<CabinetLibrary>>,
    cabinet_sr: Arc<AtomicU32>,
    cabinet_max_block: Arc<AtomicUsize>,
    latency_stats: Arc<LatencyStats>,
    audio_status: Arc<AudioStatus>,
) {
    let mut _input_stream: Option<Stream> = None;
    let mut _output_stream: Option<Stream> = None;

    for cmd in rx_cmd {
        match cmd {
            AudioCommand::RefreshDevices(host_id) => {
                let mut inputs_list = vec![];
                let mut outputs_list = vec![];

                if let Ok(host) = cpal::host_from_id(host_id) {
                    // `enum_index` counts every device the host yields, including
                    // those skipped below. It has to index the *unfiltered*
                    // enumeration, because that is what `ApplyRouting` walks
                    // when it looks the device back up.
                    if let Ok(devs) = host.input_devices() {
                        for (enum_index, dev) in devs.enumerate() {
                            let raw_name =
                                dev.name().unwrap_or_else(|_| "Unknown Device".to_string());
                            let mut b_name = raw_name.clone();
                            if cfg!(target_os = "linux") {
                                let (n, _) = beautify_linux_name(&raw_name, true);
                                b_name = n;
                                if b_name.contains("(Ocultar)") {
                                    continue;
                                }
                            }
                            // A failing config no longer skips the device: it is listed
                            // as unusable so the user sees it exists (CROSS-16).
                            inputs_list.push(DeviceContext::from_config_result(
                                b_name,
                                raw_name,
                                enum_index,
                                Direction::Input,
                                dev.default_input_config(),
                                enumerate_sample_formats(&dev, Direction::Input),
                            ));
                        }
                    }
                    if let Ok(devs) = host.output_devices() {
                        for (enum_index, dev) in devs.enumerate() {
                            let raw_name =
                                dev.name().unwrap_or_else(|_| "Unknown Device".to_string());
                            let mut b_name = raw_name.clone();
                            if cfg!(target_os = "linux") {
                                let (n, _) = beautify_linux_name(&raw_name, false);
                                b_name = n;
                                if b_name.contains("(Ocultar)") {
                                    continue;
                                }
                            }
                            outputs_list.push(DeviceContext::from_config_result(
                                b_name,
                                raw_name,
                                enum_index,
                                Direction::Output,
                                dev.default_output_config(),
                                enumerate_sample_formats(&dev, Direction::Output),
                            ));
                        }
                    }
                }
                let _ = tx_event.send(AudioEvent::DevicesRefreshed {
                    inputs: inputs_list,
                    outputs: outputs_list,
                });
            }
            AudioCommand::ApplyRouting {
                host_id,
                input,
                output,
                buffer_size,
            } => {
                let mut err_msg = None;
                _input_stream = None;
                _output_stream = None;

                // Fresh telemetry per routing: counters and latency figures must
                // describe the streams being opened now, not previous sessions.
                audio_status.reset();
                latency_stats.reset();

                let (analyzer_producer, analyzer_consumer) = RingBuffer::new(1024 * 64);
                if let Ok(mut cons_lock) = consumer_mutex.lock() {
                    *cons_lock = Some(analyzer_consumer);
                }

                // The actual cpal callback block can exceed the UI slider's
                // `buffer_size`: `strict_config.buffer_size` is left at
                // `cpal::BufferSize::Default` below, so the OS/driver picks the
                // real block size. Negotiate every config up front — both
                // directions, and single-direction routing too — so that
                // `max_block` and every buffer sized from it (pipeline scratch
                // space, playthrough ring buffer) come from the block the device
                // we are about to open actually promises, not just the slider.
                let has_both = input.is_some() && output.is_some();

                let host = cpal::host_from_id(host_id).ok();
                // CROSS-15 runs first: an ASIO driver owns a single duplex
                // stream pair, so when both directions are routed the lookup
                // happens once and the resulting `Device` is cloned for both.
                // Building the two streams on independent `Device`s would tear
                // each other down — and resolving the directions independently
                // would also try to hold two loaded drivers at once, which
                // asio-sys forbids.
                let resolve_pair = || {
                    if let Some(shared) = host
                        .as_ref()
                        .and_then(|h| asio_duplex_device(h, host_id, &input, &output))
                    {
                        return (Some(shared.clone()), Some(shared));
                    }
                    let input_device = match (&host, input.as_ref()) {
                        (Some(h), Some(sel)) => resolve_device(
                            || h.input_devices().ok(),
                            sel.enum_index,
                            &sel.raw_name,
                            |d| d.name().ok(),
                        ),
                        _ => None,
                    };
                    let output_device = match (&host, output.as_ref()) {
                        (Some(h), Some(sel)) => resolve_device(
                            || h.output_devices().ok(),
                            sel.enum_index,
                            &sel.raw_name,
                            |d| d.name().ok(),
                        ),
                        _ => None,
                    };
                    (input_device, output_device)
                };
                // A driver released by the routing we just tore down can take a
                // moment to accept a new client (common with ASIO): retry the
                // lookup briefly before declaring the device missing. This runs
                // on the worker thread, never the audio thread.
                let (mut input_device, mut output_device) = resolve_pair();
                for _ in 0..2 {
                    let input_ok = input.is_none() || input_device.is_some();
                    let output_ok = output.is_none() || output_device.is_some();
                    if input_ok && output_ok {
                        break;
                    }
                    thread::sleep(Duration::from_millis(150));
                    (input_device, output_device) = resolve_pair();
                }

                let negotiated: NegotiatedConfigs = if host.is_none() {
                    Err("Falha ao comunicar com o Host do S.O.".to_string())
                } else if input.is_some() && input_device.is_none() {
                    Err(format!(
                        "A entrada \"{}\" está listada mas não pôde ser aberta agora. \
                         Em ASIO isso costuma significar que outro aplicativo está \
                         usando o driver (DAW, ASIO Link Pro, painel do fabricante) \
                         ou que o aparelho foi desconectado. Feche o outro \
                         aplicativo, reconecte a interface e clique em Atualizar \
                         Dispositivos.",
                        input.as_ref().map(|s| s.raw_name.as_str()).unwrap_or("?")
                    ))
                } else if output.is_some() && output_device.is_none() {
                    Err(format!(
                        "A saída \"{}\" está listada mas não pôde ser aberta agora. \
                         Em ASIO isso costuma significar que outro aplicativo está \
                         usando o driver (DAW, ASIO Link Pro, painel do fabricante) \
                         ou que o aparelho foi desconectado. Feche o outro \
                         aplicativo, reconecte a interface e clique em Atualizar \
                         Dispositivos.",
                        output.as_ref().map(|s| s.raw_name.as_str()).unwrap_or("?")
                    ))
                } else {
                    match (input_device.as_ref(), output_device.as_ref()) {
                        (Some(in_dev), Some(out_dev)) => match pick_full_duplex(in_dev, out_dev) {
                            Ok((in_cfgs, out_cfgs, _rate)) => Ok((in_cfgs, out_cfgs)),
                            Err(_) => {
                                let in_cfgs =
                                    pick_config(in_dev, StreamDirection::Input).map_err(|e| {
                                        format!("❌ Entrada sem configuração utilizável: {e}")
                                    });
                                let out_cfgs = pick_config(out_dev, StreamDirection::Output)
                                    .map_err(|e| {
                                        format!("❌ Saída sem configuração utilizável: {e}")
                                    });

                                match (in_cfgs, out_cfgs) {
                                    (Ok(in_cfgs), Ok(out_cfgs)) => Ok((in_cfgs, out_cfgs)),
                                    (Err(msg), _) | (_, Err(msg)) => Err(msg),
                                }
                            }
                        },
                        (Some(in_dev), None) => pick_config(in_dev, StreamDirection::Input)
                            .map(|cfgs| (cfgs, Vec::new()))
                            .map_err(|e| format!("❌ Entrada sem configuração utilizável: {e}")),
                        (None, Some(out_dev)) => pick_config(out_dev, StreamDirection::Output)
                            .map(|cfgs| (Vec::new(), cfgs))
                            .map_err(|e| format!("❌ Saída sem configuração utilizável: {e}")),
                        (None, None) => Ok((Vec::new(), Vec::new())),
                    }
                };

                let (input_configs, output_configs) = match negotiated {
                    Ok(configs) => configs,
                    Err(msg) => {
                        let _ = tx_event.send(AudioEvent::StreamStarted {
                            result: Err(msg),
                            sample_rate_warning: None,
                        });
                        continue;
                    }
                };

                let sample_rate_warning = match input_configs.first().zip(output_configs.first()) {
                    Some((input_config, output_config)) => {
                        let (effective_rate, needs_resampling) =
                            reconcile_sample_rate(input_config, output_config);
                        needs_resampling.then(|| {
                            format!(
                                "⚠️ resampling ativo: {} → {} Hz",
                                input_config.config.sample_rate().0,
                                effective_rate
                            )
                        })
                    }
                    None => None,
                };

                // Rate the playthrough ring is drained at. The output config that
                // actually opens is chosen in a later retry loop, so this takes the
                // preferred candidate — the same one `sample_rate_warning` reports.
                let target_output_rate: Option<u32> = if has_both {
                    output_configs.first().map(|cfg| cfg.config.sample_rate().0)
                } else {
                    None
                };

                // Worst-case rate conversion across every input candidate we may
                // retry: a ratio above 1.0 means the input side pushes more frames
                // than it consumes, so the ring must grow to match.
                let max_resample_ratio: f64 = match target_output_rate {
                    Some(out_rate) => input_configs
                        .iter()
                        .map(|cfg| out_rate as f64 / cfg.config.sample_rate().0 as f64)
                        .fold(1.0, f64::max),
                    None => 1.0,
                };

                // Size the shared ring for the largest negotiated retry candidate.
                // Input DSP scratch space is sized from the concrete config attempted below.
                let ring_buffer_max_block: usize = input_configs
                    .iter()
                    .chain(output_configs.iter())
                    .map(|cfg| cfg.max_block)
                    .max()
                    .unwrap_or(FALLBACK_MAX_BLOCK)
                    .max(buffer_size as usize);

                let playthrough_capacity = if has_both {
                    // Two samples per frame, four blocks of slack — scaled by the
                    // ratio because the resampler pushes output-rate frames.
                    let ring_frames =
                        (ring_buffer_max_block as f64 * max_resample_ratio).ceil() as usize;
                    // The resampler emits a whole chunk at once while the output
                    // callback drains gradually, so the ring must also hold several
                    // full bursts however small the driver block turns out to be.
                    let burst_frames =
                        (RtResampler::CHUNK_FRAMES as f64 * max_resample_ratio).ceil() as usize;
                    let floor_samples = (burst_frames * 4 * 2).max(2048);
                    (ring_frames * 8).max(floor_samples)
                } else {
                    0
                };
                let (pt_producer, pt_consumer) = if has_both {
                    let (p, c) = RingBuffer::new(playthrough_capacity);
                    // Wrap in Arc<Mutex<>> so they can be cloned for each config attempt
                    (Some(Arc::new(Mutex::new(p))), Some(Arc::new(Mutex::new(c))))
                } else {
                    (None, None)
                };

                // Slack the ring should hold in steady state — the UI slider's
                // value, bounded so the 3× overflow watermark still fits inside
                // the ring. This is the figure the latency display reports and
                // the drift controller steers toward.
                let target_fill_samples = (buffer_size as usize * 2)
                    .clamp(128, (playthrough_capacity / 4).max(128));

                // Prefill with silence up to the target so the startup latency is
                // deterministic instead of a race between the two callbacks, and
                // the first output callbacks never see an empty ring.
                if let Some(p) = &pt_producer {
                    if let Ok(mut prod) = p.lock() {
                        for _ in 0..target_fill_samples.min(playthrough_capacity) {
                            let _ = prod.push(0.0);
                        }
                    }
                }

                let drift_observation = Arc::new(DriftObservation::new());

                let state_clone = standalone_state.clone();
                // Wrap analyzer_producer in Arc<Mutex<>> so it can be cloned for each config attempt
                let analyzer_producer = Arc::new(Mutex::new(analyzer_producer));

                if let (Some(DeviceSelection { left, right, .. }), Some(device)) =
                    (input.as_ref(), input_device.as_ref())
                {
                    let (left, right) = (*left, *right);
                    // Try each config until one succeeds (Issue 1 fix)
                    let mut input_built = false;
                    for picked_config in &input_configs {
                        let config = &picked_config.config;
                        let mut strict_config: cpal::StreamConfig = config.clone().into();
                        strict_config.buffer_size = cpal::BufferSize::Default;
                        let channels = strict_config.channels;
                        let l_idx = left.min((channels.saturating_sub(1)) as usize);
                        let r_idx = right.min((channels.saturating_sub(1)) as usize);
                        // The slider no longer inflates the DSP block: the driver
                        // picks the real callback size (`BufferSize::Default`), so
                        // scratch space comes from the negotiated config alone.
                        // Blocks larger than this are chunked by
                        // `process_interleaved_block`, so a short buffer is safe.
                        let max_block = picked_config.max_block;

                        // Clone Arc-wrapped values for this config attempt
                        let state_for_callback = state_clone.clone();
                        let analyzer_for_callback = analyzer_producer.clone();
                        let pt_for_callback = pt_producer.clone();
                        let latency_for_callback = latency_stats.clone();
                        let drift_for_callback = drift_observation.clone();
                        let status_for_callback = audio_status.clone();

                        // INICIALIZAÇÃO DSP: Fora do loop de processamento! Comum aos
                        // três formatos nativos suportados (F32/I32/I16, T15) — cada
                        // um só difere na conversão de amostra usada ao desinterlear
                        // (sample_convert) e no tipo que o cpal exige no callback; a
                        // pipeline e os buffers de scratch são construídos uma única
                        // vez e movidos para dentro de qual braço for de fato
                        // utilizado.
                        let cfg_rate = picked_config.config.sample_rate().0;
                        let s_rate = cfg_rate as f32;

                        // ESTÁGIO 4: Cabinet IR gerenciado (biblioteca + engine, sem path hardcoded).
                        let cabinet_runtime = {
                            // Resolver o IR ativo e preparar um runtime para este sample rate.
                            let mut runtime = None;
                            let mut hash = state_clone
                                .lock()
                                .ok()
                                .map(|s| s.cab_active_hash.clone())
                                .unwrap_or_default();
                            if hash.is_empty() {
                                hash = cabinet_library
                                    .lock()
                                    .ok()
                                    .and_then(|l| l.get_selected_hash().ok().flatten())
                                    .unwrap_or_default();
                            }
                            if !hash.is_empty() {
                                if let Ok(l) = cabinet_library.lock() {
                                    if let Ok((_, bytes)) = l.get_ir_by_hash(&hash) {
                                        if let Ok(rt) =
                                            CabinetRuntime::build(&bytes, s_rate, max_block)
                                        {
                                            runtime = Some(rt);
                                        }
                                    }
                                }
                            }
                            runtime
                        };

                        // Pré-EQ (tone-stack) — IR fixo embedado no binário (sem path absoluto).
                        let pre_eq_ir = decode_wav_flat(DEFAULT_PRE_EQ_IR).unwrap_or_default();

                        let mut pipeline = StandalonePipeline::new(
                            s_rate,
                            max_block,
                            &pre_eq_ir,
                            cabinet_mailbox.clone(),
                        );

                        // ESTÁGIO 6: reamostragem para a taxa do dispositivo de saída
                        // (T22). A pipeline roda em `cfg_rate` — a taxa que este
                        // config de entrada de fato abriu — e só o que sai dela é
                        // convertido, de modo que EQ, IR e cabinet continuam
                        // coerentes com o `s_rate` usado para construí-los.
                        // `None` quando as taxas já coincidem, e o bloco vai direto
                        // para os rings sem custo algum.
                        let mut resampler = target_output_rate
                            .filter(|&out_rate| out_rate != cfg_rate)
                            .map(|out_rate| RtResampler::new(s_rate, out_rate as f32, max_block));

                        // Buffers temporários para processamento in-place em bloco
                        let mut buf_l = vec![0.0; max_block];
                        let mut buf_r = vec![0.0; max_block];
                        let channels_usize = channels as usize;

                        let stream_res = match config.sample_format() {
                            cpal::SampleFormat::F32 => {
                                let status_err = audio_status.clone();
                                let mut last_drift_sequence = 0;
                                device.build_input_stream(
                                &strict_config,
                                move |data: &[f32], _: &_| {
                                    latency_for_callback.input_frames.store(
                                        (data.len() / channels_usize.max(1)) as u64,
                                        Ordering::Relaxed,
                                    );
                                    latency_for_callback
                                        .input_rate
                                        .store(cfg_rate as u64, Ordering::Relaxed);
                                    let snap = state_for_callback
                                        .try_lock()
                                        .map(|g| g.audio())
                                        .unwrap_or_else(|_| AudioSnapshot::default());
                                    process_interleaved_block(
                                        &mut pipeline,
                                        data,
                                        channels_usize,
                                        l_idx,
                                        r_idx,
                                        |s| s,
                                        &mut buf_l,
                                        &mut buf_r,
                                        &snap,
                                        |out_l, out_r| {
                                            emit_processed_block(
                                                &mut resampler,
                                                &analyzer_for_callback,
                                                &pt_for_callback,
                                                &status_for_callback,
                                                out_l,
                                                out_r,
                                            )
                                        },
                                    );
                                    drift_for_callback.apply_to(
                                        &mut resampler,
                                        &mut last_drift_sequence,
                                    );
                                },
                                move |err| status_err.set_error(stream_error_kind(err)),
                                None,
                            )
                            }
                            cpal::SampleFormat::I32 => {
                                let status_err = audio_status.clone();
                                let state_for_i32 = state_for_callback.clone();
                                let analyzer_for_i32 = analyzer_for_callback.clone();
                                let pt_for_i32 = pt_for_callback.clone();
                                let latency_for_i32 = latency_for_callback.clone();
                                let drift_for_i32 = drift_for_callback.clone();
                                let mut last_drift_sequence = 0;
                                let status_for_i32 = status_for_callback.clone();
                                device.build_input_stream(
                                    &strict_config,
                                    move |data: &[i32], _: &_| {
                                        latency_for_i32.input_frames.store(
                                            (data.len() / channels_usize.max(1)) as u64,
                                            Ordering::Relaxed,
                                        );
                                        latency_for_i32
                                            .input_rate
                                            .store(cfg_rate as u64, Ordering::Relaxed);
                                        let snap = state_for_i32
                                            .try_lock()
                                            .map(|g| g.audio())
                                            .unwrap_or_else(|_| AudioSnapshot::default());
                                        process_interleaved_block(
                                            &mut pipeline,
                                            data,
                                            channels_usize,
                                            l_idx,
                                            r_idx,
                                            sample_convert::i32_to_f32,
                                            &mut buf_l,
                                            &mut buf_r,
                                            &snap,
                                            |out_l, out_r| {
                                                emit_processed_block(
                                                    &mut resampler,
                                                    &analyzer_for_i32,
                                                    &pt_for_i32,
                                                    &status_for_i32,
                                                    out_l,
                                                    out_r,
                                                )
                                            },
                                        );
                                        drift_for_i32.apply_to(
                                            &mut resampler,
                                            &mut last_drift_sequence,
                                        );
                                    },
                                    move |err| status_err.set_error(stream_error_kind(err)),
                                    None,
                                )
                            }
                            cpal::SampleFormat::I16 => {
                                let status_err = audio_status.clone();
                                let state_for_i16 = state_for_callback.clone();
                                let analyzer_for_i16 = analyzer_for_callback.clone();
                                let pt_for_i16 = pt_for_callback.clone();
                                let latency_for_i16 = latency_for_callback.clone();
                                let drift_for_i16 = drift_for_callback.clone();
                                let mut last_drift_sequence = 0;
                                let status_for_i16 = status_for_callback.clone();
                                device.build_input_stream(
                                    &strict_config,
                                    move |data: &[i16], _: &_| {
                                        latency_for_i16.input_frames.store(
                                            (data.len() / channels_usize.max(1)) as u64,
                                            Ordering::Relaxed,
                                        );
                                        latency_for_i16
                                            .input_rate
                                            .store(cfg_rate as u64, Ordering::Relaxed);
                                        let snap = state_for_i16
                                            .try_lock()
                                            .map(|g| g.audio())
                                            .unwrap_or_else(|_| AudioSnapshot::default());
                                        process_interleaved_block(
                                            &mut pipeline,
                                            data,
                                            channels_usize,
                                            l_idx,
                                            r_idx,
                                            sample_convert::i16_to_f32,
                                            &mut buf_l,
                                            &mut buf_r,
                                            &snap,
                                            |out_l, out_r| {
                                                emit_processed_block(
                                                    &mut resampler,
                                                    &analyzer_for_i16,
                                                    &pt_for_i16,
                                                    &status_for_i16,
                                                    out_l,
                                                    out_r,
                                                )
                                            },
                                        );
                                        drift_for_i16.apply_to(
                                            &mut resampler,
                                            &mut last_drift_sequence,
                                        );
                                    },
                                    move |err| status_err.set_error(stream_error_kind(err)),
                                    None,
                                )
                            }
                            // Any other native format cpal could report is rejected
                            // explicitly rather than silently passed through DSP-free.
                            _ => Err(cpal::BuildStreamError::StreamConfigNotSupported),
                        };

                        match stream_res {
                            Ok(str) => {
                                if let Err(e) = str.play() {
                                    // Play failed, try next config
                                    eprintln!(
                                        "⚠ Play failed for input config {:?}: {:?}, trying next...",
                                        config.sample_format(),
                                        e
                                    );
                                    continue;
                                } else {
                                    cabinet_sr.store(cfg_rate, Ordering::Relaxed);
                                    cabinet_max_block.store(max_block, Ordering::Relaxed);
                                    if let Some(rt) = cabinet_runtime {
                                        cabinet_mailbox.publish(rt);
                                    }
                                    _input_stream = Some(str);
                                    input_built = true;
                                    break; // Success!
                                }
                            }
                            Err(e) => {
                                // Build failed, try next config
                                eprintln!(
                                    "⚠ Build failed for input config {:?}: {:?}, trying next...",
                                    config.sample_format(),
                                    e
                                );
                                continue;
                            }
                        }
                    }

                    // If no config succeeded, report error
                    if !input_built {
                        err_msg = Some(format!(
                            "❌ Failed to build input stream after trying {} config(s). \
                             Check device compatibility or try a different driver.",
                            input_configs.len()
                        ));
                    }
                }

                if let (Some(DeviceSelection { left, right, .. }), Some(device)) =
                    (output.as_ref(), output_device.as_ref())
                {
                    let (left, right) = (*left, *right);
                    // Try each config until one succeeds (Issue 1 fix)
                    let mut output_built = false;
                    for picked_config in &output_configs {
                        let config = &picked_config.config;
                        let mut strict_config: cpal::StreamConfig = config.clone().into();
                        strict_config.buffer_size = cpal::BufferSize::Default;
                        let channels = strict_config.channels;
                        let l_idx = left.min((channels.saturating_sub(1)) as usize);
                        let r_idx = right.min((channels.saturating_sub(1)) as usize);

                        // Clone Arc-wrapped values for this config attempt
                        let pt_for_callback = pt_consumer.clone();
                        let drift_for_callback = drift_observation.clone();
                        let output_rate = strict_config.sample_rate.0;

                        let stream_res = match config.sample_format() {
                            cpal::SampleFormat::F32 => {
                                let status_err = audio_status.clone();
                                let mut frames_since_observation = 0;
                                let mut refilling = false;
                                let status_for_callback = audio_status.clone();
                                let latency_for_out = latency_stats.clone();
                                device.build_output_stream(
                                &strict_config,
                                move |data: &mut [f32], _: &_| {
                                    let mut ring = pt_for_callback
                                        .as_ref()
                                        .and_then(|pc| pc.try_lock().ok());
                                    if let Some(c) = ring.as_deref_mut() {
                                        trim_playthrough_excess(
                                            c,
                                            target_fill_samples,
                                            &status_for_callback,
                                        );
                                    }
                                    for frame in data.chunks_mut(channels as usize) {
                                        let (l_sample, r_sample) = match ring.as_deref_mut() {
                                            Some(c) => pop_playthrough_frame(
                                                c,
                                                &mut refilling,
                                                target_fill_samples,
                                                &status_for_callback,
                                            ),
                                            None => (0.0, 0.0),
                                        };
                                        if let Some(l) = frame.get_mut(l_idx) {
                                            *l = l_sample;
                                        }
                                        if let Some(r) = frame.get_mut(r_idx) {
                                            *r = r_sample;
                                        }
                                    }
                                    let frames = data.len() / channels as usize;
                                    let fill =
                                        ring.as_deref().map(|c| c.slots()).unwrap_or(0);
                                    drop(ring);
                                    latency_for_out
                                        .output_frames
                                        .store(frames as u64, Ordering::Relaxed);
                                    latency_for_out
                                        .output_rate
                                        .store(output_rate as u64, Ordering::Relaxed);
                                    latency_for_out
                                        .ring_fill_samples
                                        .store(fill as u64, Ordering::Relaxed);
                                    observe_playthrough_fill(
                                        fill,
                                        target_fill_samples,
                                        frames,
                                        output_rate,
                                        &mut frames_since_observation,
                                        &drift_for_callback,
                                    );
                                },
                                move |err| status_err.set_error(stream_error_kind(err)),
                                None,
                            )
                            }
                            cpal::SampleFormat::I32 => {
                                let status_err = audio_status.clone();
                                let status_for_i32 = audio_status.clone();
                                let pt_for_i32 = pt_for_callback.clone();
                                let drift_for_i32 = drift_for_callback.clone();
                                let mut frames_since_observation = 0;
                                let mut refilling = false;
                                let latency_for_i32 = latency_stats.clone();
                                device.build_output_stream(
                                    &strict_config,
                                    move |data: &mut [i32], _: &_| {
                                        let mut ring = pt_for_i32
                                            .as_ref()
                                            .and_then(|pc| pc.try_lock().ok());
                                        if let Some(c) = ring.as_deref_mut() {
                                            trim_playthrough_excess(
                                                c,
                                                target_fill_samples,
                                                &status_for_i32,
                                            );
                                        }
                                        for frame in data.chunks_mut(channels as usize) {
                                            let (l_sample, r_sample) = match ring.as_deref_mut()
                                            {
                                                Some(c) => pop_playthrough_frame(
                                                    c,
                                                    &mut refilling,
                                                    target_fill_samples,
                                                    &status_for_i32,
                                                ),
                                                None => (0.0, 0.0),
                                            };
                                            if let Some(l) = frame.get_mut(l_idx) {
                                                *l = sample_convert::f32_to_i32(l_sample);
                                            }
                                            if let Some(r) = frame.get_mut(r_idx) {
                                                *r = sample_convert::f32_to_i32(r_sample);
                                            }
                                        }
                                        let frames = data.len() / channels as usize;
                                        let fill =
                                            ring.as_deref().map(|c| c.slots()).unwrap_or(0);
                                        drop(ring);
                                        latency_for_i32
                                            .output_frames
                                            .store(frames as u64, Ordering::Relaxed);
                                        latency_for_i32
                                            .output_rate
                                            .store(output_rate as u64, Ordering::Relaxed);
                                        latency_for_i32
                                            .ring_fill_samples
                                            .store(fill as u64, Ordering::Relaxed);
                                        observe_playthrough_fill(
                                            fill,
                                            target_fill_samples,
                                            frames,
                                            output_rate,
                                            &mut frames_since_observation,
                                            &drift_for_i32,
                                        );
                                    },
                                    move |err| status_err.set_error(stream_error_kind(err)),
                                    None,
                                )
                            }
                            cpal::SampleFormat::I16 => {
                                let status_err = audio_status.clone();
                                let status_for_i16 = audio_status.clone();
                                let pt_for_i16 = pt_for_callback.clone();
                                let drift_for_i16 = drift_for_callback.clone();
                                let mut frames_since_observation = 0;
                                let mut refilling = false;
                                let latency_for_i16 = latency_stats.clone();
                                device.build_output_stream(
                                    &strict_config,
                                    move |data: &mut [i16], _: &_| {
                                        let mut ring = pt_for_i16
                                            .as_ref()
                                            .and_then(|pc| pc.try_lock().ok());
                                        if let Some(c) = ring.as_deref_mut() {
                                            trim_playthrough_excess(
                                                c,
                                                target_fill_samples,
                                                &status_for_i16,
                                            );
                                        }
                                        for frame in data.chunks_mut(channels as usize) {
                                            let (l_sample, r_sample) = match ring.as_deref_mut()
                                            {
                                                Some(c) => pop_playthrough_frame(
                                                    c,
                                                    &mut refilling,
                                                    target_fill_samples,
                                                    &status_for_i16,
                                                ),
                                                None => (0.0, 0.0),
                                            };
                                            if let Some(l) = frame.get_mut(l_idx) {
                                                *l = sample_convert::f32_to_i16(l_sample);
                                            }
                                            if let Some(r) = frame.get_mut(r_idx) {
                                                *r = sample_convert::f32_to_i16(r_sample);
                                            }
                                        }
                                        let frames = data.len() / channels as usize;
                                        let fill =
                                            ring.as_deref().map(|c| c.slots()).unwrap_or(0);
                                        drop(ring);
                                        latency_for_i16
                                            .output_frames
                                            .store(frames as u64, Ordering::Relaxed);
                                        latency_for_i16
                                            .output_rate
                                            .store(output_rate as u64, Ordering::Relaxed);
                                        latency_for_i16
                                            .ring_fill_samples
                                            .store(fill as u64, Ordering::Relaxed);
                                        observe_playthrough_fill(
                                            fill,
                                            target_fill_samples,
                                            frames,
                                            output_rate,
                                            &mut frames_since_observation,
                                            &drift_for_i16,
                                        );
                                    },
                                    move |err| status_err.set_error(stream_error_kind(err)),
                                    None,
                                )
                            }
                            _ => Err(cpal::BuildStreamError::StreamConfigNotSupported),
                        };

                        match stream_res {
                            Ok(str) => {
                                if let Err(e) = str.play() {
                                    // Play failed, try next config
                                    eprintln!("⚠ Play failed for output config {:?}: {:?}, trying next...",
                                             config.sample_format(), e);
                                    continue;
                                } else {
                                    _output_stream = Some(str);
                                    output_built = true;
                                    break; // Success!
                                }
                            }
                            Err(e) => {
                                // Build failed, try next config
                                eprintln!(
                                    "⚠ Build failed for output config {:?}: {:?}, trying next...",
                                    config.sample_format(),
                                    e
                                );
                                continue;
                            }
                        }
                    }

                    // If no config succeeded, report error
                    if !output_built {
                        err_msg = Some(format!(
                            "❌ Failed to build output stream after trying {} config(s). \
                             Check device compatibility or try a different driver.",
                            output_configs.len()
                        ));
                    }
                }

                if let Some(err) = err_msg {
                    let _ = tx_event.send(AudioEvent::StreamStarted {
                        result: Err(err),
                        sample_rate_warning,
                    });
                } else {
                    let _ = tx_event.send(AudioEvent::StreamStarted {
                        result: Ok(()),
                        sample_rate_warning,
                    });
                }
            }
            AudioCommand::Stop => {
                _input_stream = None;
                _output_stream = None;
                break;
            }
        }
    }
}

impl StandaloneApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let (_, consumer) = RingBuffer::new(1024 * 64);
        let cons_arc = Arc::new(Mutex::new(Some(consumer)));

        // Defaults to CPAL available host
        let hosts = cpal::available_hosts();
        let default_host = cpal::default_host().id();

        let (tx_cmd, rx_worker) = channel();
        let (tx_worker, rx_event) = channel();

        // --- Cabinet IR setup (shared library + lock-free mailbox) ---
        let cabinet_library = Arc::new(Mutex::new(open_cabinet_library()));
        let cabinet_mailbox = CabinetMailbox::new_arc();
        let cabinet_sr = Arc::new(AtomicU32::new(48_000));
        let cabinet_max_block = Arc::new(AtomicUsize::new(2048));
        let cabinet_error = Arc::new(Mutex::new(None));
        #[cfg(feature = "lab")]
        let lab_error = Arc::new(Mutex::new(None));
        #[cfg(feature = "lab")]
        let lab = match lab_data_dir().and_then(Lab::init) {
            Ok(lab) => Some(Arc::new(lab)),
            Err(err) => {
                if let Ok(mut slot) = lab_error.lock() {
                    *slot = Some(err.to_string());
                }
                None
            }
        };

        // Resolve the persisted (or seeded default) selection into the shared state.
        let mut initial_state = StandaloneState::default();
        let resolved_hash = load_persisted_hash().unwrap_or_default();
        let resolved_hash = if resolved_hash.is_empty() {
            cabinet_library
                .lock()
                .ok()
                .and_then(|l| l.get_selected_hash().ok().flatten())
                .unwrap_or_default()
        } else {
            resolved_hash
        };
        initial_state.cab_active_hash = resolved_hash;

        let standalone_state = Arc::new(Mutex::new(initial_state));

        let latency_stats = Arc::new(LatencyStats::new());
        let audio_status = Arc::new(AudioStatus::new());

        let cons_clone = cons_arc.clone();
        let state_clone = standalone_state.clone();
        let mailbox_clone = cabinet_mailbox.clone();
        let library_clone = cabinet_library.clone();
        let sr_clone = cabinet_sr.clone();
        let maxb_clone = cabinet_max_block.clone();
        let latency_clone = latency_stats.clone();
        let status_clone = audio_status.clone();
        thread::spawn(move || {
            audio_worker(
                rx_worker,
                tx_worker,
                cons_clone,
                state_clone,
                mailbox_clone,
                library_clone,
                sr_clone,
                maxb_clone,
                latency_clone,
                status_clone,
            );
        });

        let mut app = Self {
            analyzer: AnalyzerDsp::default(),
            consumer: cons_arc,
            hosts,
            selected_host: default_host,

            available_inputs: vec![],
            selected_input_idx: Some(0),
            input_left: 0,
            input_right: 1,

            available_outputs: vec![],
            selected_output_idx: None,
            output_left: 0,
            output_right: 1,

            buffer_power: 11,     // 2048 frames default
            buffer_range_min: 5,  // 32 minimum starting
            buffer_range_max: 13, // 8192 max starting

            latency_stats,

            sample_rate_warning: None,
            audio_status,
            last_audio_error: None,
            last_audio_error_details: None,
            show_error_popup: false,

            prev_glitch_total: 0,
            recent_glitches: VecDeque::new(),

            input_is_mono: true,
            output_is_mono: false,

            show_settings: false,
            is_loading: true,
            active_panel: ActivePanel::None,
            mlc_tab: MlcTab::default(),
            standalone_state,
            cabinet_library,
            cabinet_mailbox,
            cabinet_sr,
            cabinet_max_block,
            cabinet_error,
            #[cfg(feature = "lab")]
            lab,
            #[cfg(feature = "lab")]
            lab_error,
            #[cfg(feature = "lab")]
            lab_ui: LabUiState::default(),
            tx_cmd,
            rx_event,
        };

        app.refresh_devices();
        app
    }

    fn poll_events(&mut self) {
        while let Ok(event) = self.rx_event.try_recv() {
            match event {
                AudioEvent::DevicesRefreshed { inputs, outputs } => {
                    self.available_inputs = inputs;
                    self.available_outputs = outputs;

                    // Unusable devices are listed but never auto-selected.
                    if self.selected_input_idx.is_none() {
                        self.selected_input_idx =
                            self.available_inputs.iter().position(|d| d.usable);
                    }
                    // The refresh may have replaced either selection with an
                    // unusable device, or dropped it from the list entirely.
                    revalidate_selection(&mut self.selected_input_idx, &self.available_inputs);
                    revalidate_selection(&mut self.selected_output_idx, &self.available_outputs);
                    self.apply_audio_routing();
                }
                AudioEvent::StreamStarted {
                    result,
                    sample_rate_warning,
                } => {
                    self.is_loading = false;
                    self.sample_rate_warning = sample_rate_warning;
                    if let Err(msg) = result {
                        eprintln!("[Audio Engine] ERRO Crítico do CPAL: {}", msg);
                        // A device that vanished from the enumeration is not a
                        // buffer problem — label it so the guidance fits.
                        let title = if msg.contains("não pôde ser aberta") {
                            "⚠️ Dispositivo indisponível"
                        } else {
                            "⚠️ Limitação de Hardware: Buffer negado"
                        };
                        self.last_audio_error = Some(title.to_string());
                        self.last_audio_error_details = Some(msg);
                    } else {
                        self.last_audio_error = None;
                        self.last_audio_error_details = None;
                        self.show_error_popup = false;
                    }
                }
            }
        }
    }

    fn refresh_devices(&mut self) {
        self.is_loading = true;
        let _ = self
            .tx_cmd
            .send(AudioCommand::RefreshDevices(self.selected_host));
    }

    fn apply_audio_routing(&mut self) {
        self.sample_rate_warning = None;

        // `.filter(usable)` guards the case where a stale index, or a device that
        // became unusable across a refresh, still sits in `selected_*_idx`.
        let input_params = if let Some(idx) = self.selected_input_idx {
            if let Some(dev_ctx) = self.available_inputs.get(idx).filter(|d| d.usable) {
                self.input_left = self
                    .input_left
                    .min((dev_ctx.channels.saturating_sub(1)) as usize);
                self.input_right = self
                    .input_right
                    .min((dev_ctx.channels.saturating_sub(1)) as usize);
                Some(DeviceSelection {
                    raw_name: dev_ctx.raw_name.clone(),
                    enum_index: dev_ctx.enum_index,
                    left: self.input_left,
                    right: self.input_right,
                })
            } else {
                None
            }
        } else {
            None
        };

        let output_params = if let Some(idx) = self.selected_output_idx {
            if let Some(dev_ctx) = self.available_outputs.get(idx).filter(|d| d.usable) {
                self.output_left = self
                    .output_left
                    .min((dev_ctx.channels.saturating_sub(1)) as usize);
                self.output_right = self
                    .output_right
                    .min((dev_ctx.channels.saturating_sub(1)) as usize);
                Some(DeviceSelection {
                    raw_name: dev_ctx.raw_name.clone(),
                    enum_index: dev_ctx.enum_index,
                    left: self.output_left,
                    right: self.output_right,
                })
            } else {
                None
            }
        } else {
            None
        };

        self.is_loading = true;
        let _ = self.tx_cmd.send(AudioCommand::ApplyRouting {
            host_id: self.selected_host,
            input: input_params,
            output: output_params,
            buffer_size: 1 << self.buffer_power,
        });
    }
}

impl eframe::App for StandaloneApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();
        self.poll_events();

        // Drain lock-free stream errors raised on the audio thread (T23). One
        // `take_error` per frame is enough: `AudioStatus` coalesces bursts and
        // tallies the rest in `dropped_errors`.
        if let Some(kind) = self.audio_status.take_error() {
            let (msg, details) = match kind {
                ErrorKind::DeviceDisconnected => (
                    "⚠️ Dispositivo de áudio desconectado",
                    "O dispositivo de áudio foi removido ou ficou indisponível durante a \
                     reprodução. Reconecte-o e clique em 'Atualizar Dispositivos'.",
                ),
                ErrorKind::StreamError => (
                    "⚠️ Erro no stream de áudio",
                    "O driver de áudio relatou um erro durante a reprodução. Verifique o \
                     dispositivo ou aumente a latência.",
                ),
                ErrorKind::ClockDriftUnrecoverable => (
                    "⚠️ Deriva de clock irrecuperável",
                    "A diferença de clock entre entrada e saída não pôde ser compensada.",
                ),
                ErrorKind::NoError => ("", ""),
            };
            if !msg.is_empty() {
                self.last_audio_error = Some(msg.to_string());
                self.last_audio_error_details = Some(details.to_string());
            }
        }
        // Reset the coalesced-error tally each frame so it never grows unbounded.
        let _ = self.audio_status.take_dropped_errors();

        // Track glitch episodes (underrun/overflow resyncs) in a rolling 60 s
        // window. Counters only ever grow between routings; a drop means the
        // worker reset them for a new routing, so the history restarts too.
        let glitch_total = self.audio_status.underruns() + self.audio_status.overflows();
        if glitch_total > self.prev_glitch_total {
            self.recent_glitches.push_back(Instant::now());
        } else if glitch_total < self.prev_glitch_total {
            self.recent_glitches.clear();
        }
        self.prev_glitch_total = glitch_total;
        while self
            .recent_glitches
            .front()
            .is_some_and(|t| t.elapsed() > Duration::from_secs(60))
        {
            self.recent_glitches.pop_front();
        }
        // Repeated resyncs mean the input and output clocks drift faster than
        // the ring slack absorbs — surface it once via the existing error banner.
        if self.recent_glitches.len() >= 3 && self.last_audio_error.is_none() {
            self.last_audio_error =
                Some("⚠️ Deriva de clock entre entrada e saída".to_string());
            self.last_audio_error_details = Some(
                "O ring de playthrough precisou ressincronizar 3+ vezes no último \
                 minuto — os clocks dos dispositivos de entrada e saída estão \
                 derivando. O sistema se recupera sozinho (com estalos breves). \
                 Para eliminar: aumente a folga do ring, use o mesmo dispositivo \
                 físico para entrada e saída, ou use um driver ASIO."
                    .to_string(),
            );
        }

        if let Ok(mut cons_lock) = self.consumer.try_lock() {
            if let Some(cons) = cons_lock.as_mut() {
                self.analyzer.process_consumer(cons);
            }
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("🎙️ BaseIO | Analisador Standalone");

                ui.add_space(20.0);

                let current_bypass = self
                    .standalone_state
                    .lock()
                    .map(|g| g.bypass)
                    .unwrap_or(false);
                let mut bypass_toggled = current_bypass;
                if ui.checkbox(&mut bypass_toggled, "Bypass").changed() {
                    if let Ok(mut st) = self.standalone_state.lock() {
                        st.bypass = bypass_toggled;
                    }
                }

                let current_amp = self
                    .standalone_state
                    .lock()
                    .map(|g| g.amp_model)
                    .unwrap_or(AmpModel::Neural);
                let mut selected_amp = current_amp;
                ui.label("Amp:");
                egui::ComboBox::from_id_salt("standalone_amp_model")
                    .selected_text(match current_amp {
                        AmpModel::Neural => "Neural",
                        AmpModel::MlcZeroV => "MLC ZERO V",
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut selected_amp, AmpModel::Neural, "Neural");
                        ui.selectable_value(&mut selected_amp, AmpModel::MlcZeroV, "MLC ZERO V");
                    });
                if selected_amp != current_amp {
                    if let Ok(mut st) = self.standalone_state.lock() {
                        st.amp_model = selected_amp;
                    }
                    self.active_panel = match selected_amp {
                        AmpModel::Neural => ActivePanel::NeuralAmp,
                        AmpModel::MlcZeroV => ActivePanel::MlcZeroV,
                    };
                }

                #[cfg(feature = "lab")]
                {
                    ui.separator();
                    if ui.button("Lab").clicked() {
                        self.lab_ui.is_open = !self.lab_ui.is_open;
                    }
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui
                        .button(if self.show_settings {
                            "❌ Fechar"
                        } else {
                            "⚙ Configurações de Áudio"
                        })
                        .clicked()
                    {
                        self.show_settings = !self.show_settings;
                    }
                    if self.is_loading {
                        ui.label("⚙ Processando Áudio...");
                        ui.spinner();
                    } else if let Some(warn) = &self.sample_rate_warning {
                        ui.visuals_mut().override_text_color =
                            Some(egui::Color32::from_rgb(255, 165, 0));
                        ui.label(warn);
                        ui.visuals_mut().override_text_color = None;
                    }
                });
            });

            if let Some(err_desc) = self.last_audio_error.clone() {
                ui.horizontal(|ui| {
                    let dark_red = egui::Color32::from_rgb(200, 70, 70);
                    ui.label(egui::RichText::new(&err_desc).color(dark_red).strong());
                    if ui.button("❓ Detalhes").clicked() {
                        self.show_error_popup = true;
                    }
                });
            }
        });

        if self.show_settings {
            egui::SidePanel::right("settings_panel").min_width(280.0).show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.add_enabled_ui(!self.is_loading, |ui| {

                        ui.heading("Sistema de Áudio");
                        ui.horizontal(|ui| {
                            ui.label("Driver:");
                            let current_host = self.selected_host;
                            let mut host_changed = false;
                            egui::ComboBox::from_id_salt("host_cb").selected_text(format!("{:?}", current_host))
                                .show_ui(ui, |ui| {
                                    for h in &self.hosts {
                                        if ui.selectable_value(&mut self.selected_host, *h, format!("{:?}", h)).clicked() {
                                            host_changed = true;
                                        }
                                    }
                                });
                            if host_changed { self.refresh_devices(); }
                            // Manual re-enumeration for when a device is (re)connected
                            // while the app is running (T23).
                            if ui.button("🔄 Atualizar Dispositivos").clicked() {
                                let _ = self
                                    .tx_cmd
                                    .send(AudioCommand::RefreshDevices(self.selected_host));
                                self.is_loading = true;
                            }
                        });

                        // Health verdict for the current routing: counters are
                        // zeroed by the worker on every Apply, so zero here
                        // really means "clean since the streams opened".
                        let underruns = self.audio_status.underruns();
                        let overflows = self.audio_status.overflows();
                        let streams_live =
                            self.latency_stats.input_rate.load(Ordering::Relaxed) > 0
                                || self.latency_stats.output_rate.load(Ordering::Relaxed) > 0;
                        if streams_live {
                            ui.separator();
                            ui.heading("📊 Telemetria de Áudio");
                            if underruns == 0 && overflows == 0 {
                                ui.label(
                                    egui::RichText::new(
                                        "✅ Estável — sem underruns/overflows neste routing",
                                    )
                                    .color(egui::Color32::from_rgb(80, 200, 120)),
                                );
                            } else {
                                if underruns != 0 {
                                    ui.label(format!(
                                        "Underruns (ring esvaziou): {underruns}"
                                    ));
                                }
                                if overflows != 0 {
                                    ui.label(format!(
                                        "Overflows (excesso descartado): {overflows}"
                                    ));
                                }
                                if let Some(last) = self.recent_glitches.back() {
                                    ui.label(format!(
                                        "Última ocorrência há {:.0} s",
                                        last.elapsed().as_secs_f32()
                                    ));
                                }
                                ui.label(
                                    egui::RichText::new(
                                        "Recuperação automática ativa: o ring ressincroniza \
                                         sozinho na folga alvo.",
                                    )
                                    .small()
                                    .color(egui::Color32::GRAY),
                                );
                            }
                        }

                        ui.separator();

                        ui.heading("🎙️ Routing de Entrada");
                        let mut route_changed = false;

                        let in_text = if let Some(idx) = self.selected_input_idx {
                            self.available_inputs.get(idx).map(|d| d.name.clone()).unwrap_or_else(|| "Desconhecido".into())
                        } else { "Nenhum (Desativado)".to_string() };

                        egui::ComboBox::from_id_salt("in_cb").selected_text(in_text).width(ui.available_width())
                            .show_ui(ui, |ui| {
                                if ui.selectable_label(self.selected_input_idx.is_none(), "Nenhum (Desativado)").clicked() {
                                    self.selected_input_idx = None;
                                    route_changed = true;
                                }
                                for (idx, dev) in self.available_inputs.iter().enumerate() {
                                    if device_entry(ui, dev, self.selected_input_idx == Some(idx)) {
                                        self.selected_input_idx = Some(idx);
                                        route_changed = true;
                                    }
                                }
                            });

                        if let Some(idx) = self.selected_input_idx {
                            if let Some(dev) = self.available_inputs.get(idx) {
                                if ui.checkbox(&mut self.input_is_mono, "Entrada Mono (Mesclar para L e R)").changed() {
                                    if self.input_is_mono { self.input_right = self.input_left; }
                                    route_changed = true;
                                }

                                if self.input_is_mono {
                                    ui.horizontal(|ui| {
                                        ui.label("Canal Físico");
                                        if egui::ComboBox::from_id_salt("in_mono").selected_text(format!("Channel {}", self.input_left + 1))
                                            .show_ui(ui, |ui| {
                                                let mut changed = false;
                                                for i in 0..(dev.channels as usize) {
                                                    if ui.selectable_value(&mut self.input_left, i, format!("Channel {}", i + 1)).clicked() { changed = true; }
                                                }
                                                changed
                                            }).inner.unwrap_or(false) {
                                                self.input_right = self.input_left;
                                                route_changed = true;
                                            }
                                    });
                                } else {
                                    ui.horizontal(|ui| {
                                        ui.label("Input L");
                                        if egui::ComboBox::from_id_salt("in_l").selected_text(format!("Channel {}", self.input_left + 1))
                                            .show_ui(ui, |ui| {
                                                let mut changed = false;
                                                for i in 0..(dev.channels as usize) {
                                                    if ui.selectable_value(&mut self.input_left, i, format!("Channel {}", i + 1)).clicked() { changed = true; }
                                                }
                                                changed
                                            }).inner.unwrap_or(false) { route_changed = true; }
                                    });
                                    ui.horizontal(|ui| {
                                        ui.label("Input R");
                                        if egui::ComboBox::from_id_salt("in_r").selected_text(format!("Channel {}", self.input_right + 1))
                                            .show_ui(ui, |ui| {
                                                let mut changed = false;
                                                for i in 0..(dev.channels as usize) {
                                                    if ui.selectable_value(&mut self.input_right, i, format!("Channel {}", i + 1)).clicked() { changed = true; }
                                                }
                                                changed
                                            }).inner.unwrap_or(false) { route_changed = true; }
                                    });
                                }
                            }
                        }

                        ui.separator();

                        ui.heading("🔊 Routing de Saída");
                        let out_text = if let Some(idx) = self.selected_output_idx {
                            self.available_outputs.get(idx).map(|d| d.name.clone()).unwrap_or_else(|| "Desconhecido".into())
                        } else { "Nenhum (Apenas GUI)".to_string() };

                        egui::ComboBox::from_id_salt("out_cb").selected_text(out_text).width(ui.available_width())
                            .show_ui(ui, |ui| {
                                if ui.selectable_label(self.selected_output_idx.is_none(), "Nenhum (Apenas GUI)").clicked() {
                                    self.selected_output_idx = None;
                                    route_changed = true;
                                }
                                for (idx, dev) in self.available_outputs.iter().enumerate() {
                                    if device_entry(ui, dev, self.selected_output_idx == Some(idx)) {
                                        self.selected_output_idx = Some(idx);
                                        route_changed = true;
                                    }
                                }
                            });

                        if let Some(idx) = self.selected_output_idx {
                            if let Some(dev) = self.available_outputs.get(idx) {
                                if ui.checkbox(&mut self.output_is_mono, "Saída Mono (Mixado para 1 Canal)").changed() {
                                    if self.output_is_mono { self.output_right = self.output_left; }
                                    route_changed = true;
                                }

                                if self.output_is_mono {
                                    ui.horizontal(|ui| {
                                        ui.label("Canal Físico");
                                        if egui::ComboBox::from_id_salt("out_mono").selected_text(format!("Channel {}", self.output_left + 1))
                                            .show_ui(ui, |ui| {
                                                let mut changed = false;
                                                for i in 0..(dev.channels as usize) {
                                                    if ui.selectable_value(&mut self.output_left, i, format!("Channel {}", i + 1)).clicked() { changed = true; }
                                                }
                                                changed
                                            }).inner.unwrap_or(false) {
                                                self.output_right = self.output_left;
                                                route_changed = true;
                                            }
                                    });
                                } else {
                                    ui.horizontal(|ui| {
                                        ui.label("Output L");
                                        if egui::ComboBox::from_id_salt("out_l").selected_text(format!("Channel {}", self.output_left + 1))
                                            .show_ui(ui, |ui| {
                                                let mut changed = false;
                                                for i in 0..(dev.channels as usize) {
                                                    if ui.selectable_value(&mut self.output_left, i, format!("Channel {}", i + 1)).clicked() { changed = true; }
                                                }
                                                changed
                                            }).inner.unwrap_or(false) { route_changed = true; }
                                    });
                                    ui.horizontal(|ui| {
                                        ui.label("Output R");
                                        if egui::ComboBox::from_id_salt("out_r").selected_text(format!("Channel {}", self.output_right + 1))
                                            .show_ui(ui, |ui| {
                                                let mut changed = false;
                                                for i in 0..(dev.channels as usize) {
                                                    if ui.selectable_value(&mut self.output_right, i, format!("Channel {}", i + 1)).clicked() { changed = true; }
                                                }
                                                changed
                                            }).inner.unwrap_or(false) { route_changed = true; }
                                    });
                                }
                            }
                        }

                        ui.separator();

                        ui.heading("⏱️ Ring buffer slack");
                        ui.add_space(2.0);

                        // End-to-end latency composed from what each callback
                        // actually observed: input block + playthrough ring fill
                        // + output block. Partial figures (input-only or
                        // output-only routing) show whichever stages exist.
                        {
                            let in_frames =
                                self.latency_stats.input_frames.load(Ordering::Relaxed);
                            let in_rate = self.latency_stats.input_rate.load(Ordering::Relaxed);
                            let fill =
                                self.latency_stats.ring_fill_samples.load(Ordering::Relaxed);
                            let out_frames =
                                self.latency_stats.output_frames.load(Ordering::Relaxed);
                            let out_rate =
                                self.latency_stats.output_rate.load(Ordering::Relaxed);

                            let in_ms = if in_rate > 0 {
                                in_frames as f64 * 1000.0 / in_rate as f64
                            } else {
                                0.0
                            };
                            let (ring_ms, out_ms) = if out_rate > 0 {
                                (
                                    (fill / 2) as f64 * 1000.0 / out_rate as f64,
                                    out_frames as f64 * 1000.0 / out_rate as f64,
                                )
                            } else {
                                (0.0, 0.0)
                            };
                            let total_ms = in_ms + ring_ms + out_ms;
                            if total_ms > 0.0 {
                                ui.label(format!("Latência total ≈ {total_ms:.1} ms"));
                                ui.label(
                                    egui::RichText::new(format!(
                                        "entrada {in_ms:.1} + ring {ring_ms:.1} + saída {out_ms:.1} ms"
                                    ))
                                    .small()
                                    .color(egui::Color32::GRAY),
                                );
                                ui.add_space(2.0);
                            }
                        }

                        let mut slider_changed = false;
                        ui.horizontal(|ui| {
                            let slider = egui::Slider::new(&mut self.buffer_power, self.buffer_range_min..=self.buffer_range_max)
                                .text("Frames")
                                .custom_formatter(|n, _| format!("{:04}", 1 << (n as u32)));

                            let res = ui.add(slider);

                            let is_pressed    = res.is_pointer_button_down_on();
                            let drag_finished = res.drag_stopped();
                            let layer_id      = res.layer_id;

                            if is_pressed {
                                egui::show_tooltip_at_pointer(ctx, layer_id, egui::Id::new("slider_drag_hint"), |ui| {
                                    ui.label(egui::RichText::new("🎯 Solte o clique para aplicar a latência!").color(egui::Color32::from_rgb(255, 180, 50)).strong());
                                });
                            }

                            if drag_finished {
                                slider_changed = true;
                            }
                        });

                        let buf_size = 1 << self.buffer_power;
                        let caption = match buf_size {
                            0..=128 => "Folga alvo mínima no ring de playthrough.\n⚠️ Risco EXTREMO de estalos/breaks.",
                            129..=512 => "Folga alvo equilibrada (menor latência de ring).\n✅ Recomendado para a maioria dos drivers.",
                            _ => "Folga alvo ampla (mais latência de ring).\n🔥 Estabilidade máxima contra drift e jitter.",
                        };

                        ui.add_space(5.0);
                        ui.label(egui::RichText::new(caption).small().color(egui::Color32::GRAY));

                        ui.add_space(8.0);
                        if self.buffer_power == self.buffer_range_max
                            && ui.button("➕ Preciso de mais Estabilidade (Max Buffers)").clicked() {
                            self.buffer_range_min = self.buffer_range_max;
                            self.buffer_range_max = (self.buffer_range_max + 3).min(15);
                        }
                        if self.buffer_power == self.buffer_range_min
                            && ui.button("➖ Preciso de menos Latência (Baixar Piso)").clicked() {
                            self.buffer_range_max = self.buffer_range_min;
                            self.buffer_range_min = self.buffer_range_min.saturating_sub(3).max(4);
                        }

                        if let Some(err_desc) = &self.last_audio_error {
                            ui.add_space(8.0);
                            let dark_red = egui::Color32::from_rgb(180, 50, 50);
                            ui.label(egui::RichText::new(err_desc).color(dark_red));
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new("Aumente a latência.").color(dark_red));
                                if ui.button("❓ Saiba por que isso ocorreu").clicked() {
                                    self.show_error_popup = true;
                                }
                            });
                        }

                        if route_changed || slider_changed {
                            self.apply_audio_routing();
                        }
                    });
                });
            });
        }

        if self.show_error_popup {
            let mut open = self.show_error_popup;
            egui::Window::new("ℹ️ Detalhes do erro de áudio")
                .open(&mut open)
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    if let Some(details) = &self.last_audio_error_details {
                        let device_missing = self.last_audio_error.as_deref()
                            == Some("⚠️ Dispositivo indisponível");
                        if device_missing {
                            ui.label("O dispositivo selecionado sumiu da enumeração ou o driver recusou abrir. Com ASIO isso geralmente significa que outro aplicativo está usando o driver (DAW, ASIO Link Pro, painel do fabricante) — drivers ASIO aceitam um cliente por vez.\n");
                            ui.label("👉 Feche outros aplicativos de áudio, reconecte a interface e clique em 'Atualizar Dispositivos'.\n\nErro exato das engrenagens do S.O:");
                        } else {
                            ui.label("Algumas placas de som ou drivers recusam trabalhar em latências tão precisas. O aplicativo exige esse número e eles respondem: Ocupado/Inválido.\n");
                            ui.label("👉 Arraste o buffer para a direita até esse erro sumir. Se a barra chegou no limite, clique em 'Preciso de mais estabilidade'.\n\nErro exato das engrenagens do S.O:");
                        }

                        egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                            ui.add(
                                egui::TextEdit::multiline(&mut details.as_str())
                                    .font(egui::TextStyle::Monospace) // for code
                                    .code_editor()
                                    .desired_rows(4)
                                    .lock_focus(true)
                                    .desired_width(f32::INFINITY),
                            );
                        });
                    }
                });
            self.show_error_popup = open;
        }

        // Drop any parked (old) cabinet runtime on this UI thread.
        self.cabinet_mailbox.collect_garbage();

        let snap_ui = self
            .standalone_state
            .lock()
            .map(|g| g.clone())
            .unwrap_or_else(|_| StandaloneState::default());

        let mut ui_neural_drive = snap_ui.neural_drive;
        let mut ui_neural_output_gain = snap_ui.neural_output_gain;
        let mut ui_neural_amp_volume = snap_ui.neural_amp_volume;
        let mut ui_gain = snap_ui.gain;
        let mut ui_mlc_gain = snap_ui.mlc_gain;
        let mut ui_mlc_master = snap_ui.mlc_master;
        let mut ui_mlc_bass = snap_ui.mlc_bass;
        let mut ui_mlc_middle = snap_ui.mlc_middle;
        let mut ui_mlc_treble = snap_ui.mlc_treble;
        let mut ui_mlc_presence = snap_ui.mlc_presence;
        let mut ui_mlc_depth = snap_ui.mlc_depth;
        let mut ui_mlc_gate = snap_ui.mlc_gate;
        let mut ui_mlc_asymmetry = snap_ui.mlc_asymmetry;
        let mut ui_mlc_preshape_tight = snap_ui.mlc_preshape_tight;
        let mut ui_mlc_preshape_bite = snap_ui.mlc_preshape_bite;
        let mut ui_mlc_clean_blend = snap_ui.mlc_clean_blend;
        let mut ui_mlc_sag = snap_ui.mlc_sag;
        let mut ui_mlc_h2 = snap_ui.mlc_h2;
        let mut ui_mlc_h3 = snap_ui.mlc_h3;
        let mut ui_mlc_h4 = snap_ui.mlc_h4;
        let mut ui_mlc_tube_drive = snap_ui.mlc_tube_drive;
        let mut ui_mlc_nfb_presence = snap_ui.mlc_nfb_presence;
        let mut ui_mlc_nfb_resonance = snap_ui.mlc_nfb_resonance;
        let mut ui_mlc_nfb_depth = snap_ui.mlc_nfb_depth;
        let mut ui_mlc_mbc_cf_lo = snap_ui.mlc_mbc_cf_lo;
        let mut ui_mlc_mbc_cf_hi = snap_ui.mlc_mbc_cf_hi;
        let mut ui_mlc_mbc_drive_lo = snap_ui.mlc_mbc_drive_lo;
        let mut ui_mlc_mbc_drive_mid = snap_ui.mlc_mbc_drive_mid;
        let mut ui_mlc_mbc_drive_hi = snap_ui.mlc_mbc_drive_hi;
        let mut ui_limiter_enable = snap_ui.limiter_enable;
        let mut ui_limiter_ceiling = snap_ui.limiter_ceiling;
        let mut ui_limiter_release = snap_ui.limiter_release;

        // MLC panel tab selection (UI-thread only); written back after the panels
        // render so the closure borrows a plain local rather than `self`.
        let mut ui_mlc_tab = self.mlc_tab;

        let mut ui_eq_low_freq = snap_ui.eq_low_freq;
        let mut ui_eq_low_gain = snap_ui.eq_low_gain;
        let mut ui_eq_low_q = snap_ui.eq_low_q;

        let mut ui_eq_mid_freq = snap_ui.eq_mid_freq;
        let mut ui_eq_mid_gain = snap_ui.eq_mid_gain;
        let mut ui_eq_mid_q = snap_ui.eq_mid_q;

        let mut ui_eq_high_freq = snap_ui.eq_high_freq;
        let mut ui_eq_high_gain = snap_ui.eq_high_gain;
        let mut ui_eq_high_q = snap_ui.eq_high_q;

        // Prepare cabinet handles/data for the shared panel (UI thread only).
        let cab_lib = self.cabinet_library.clone();
        let cab_mailbox = self.cabinet_mailbox.clone();
        let cab_sr = self.cabinet_sr.clone();
        let cab_maxb = self.cabinet_max_block.clone();
        let cab_err = self.cabinet_error.clone();
        let cab_state = self.standalone_state.clone();
        let cab_ir_list = cab_lib
            .lock()
            .ok()
            .map(|l| l.list_irs().unwrap_or_default())
            .unwrap_or_default();
        let cab_active_hash = snap_ui.cab_active_hash.clone();
        let cab_err_now = cab_err.lock().ok().and_then(|e| e.clone());
        let cab_active = !snap_ui.cabinet_bypass;

        render_shared_panels(
            ctx,
            &mut self.active_panel,
            &self.analyzer.spectrum,
            FFT_SIZE,
            snap_ui.eq_active,
            snap_ui.neural_active,
            cab_active,
            snap_ui.bypass,
            snap_ui.amp_model,
            || {
                if let Ok(mut st) = self.standalone_state.lock() {
                    st.eq_active = !snap_ui.eq_active;
                }
            },
            || {
                if let Ok(mut st) = self.standalone_state.lock() {
                    st.neural_active = !snap_ui.neural_active;
                }
            },
            || {
                if let Ok(mut st) = self.standalone_state.lock() {
                    st.cabinet_bypass = !snap_ui.cabinet_bypass;
                }
            },
            || {
                if let Ok(mut st) = self.standalone_state.lock() {
                    st.amp_model = AmpModel::Neural;
                }
            },
            || {
                if let Ok(mut st) = self.standalone_state.lock() {
                    st.amp_model = AmpModel::MlcZeroV;
                }
            },
            |ui| {
                let mut eq_tanh_bypass = snap_ui.eq_tanh_bypass;
                if ui.checkbox(&mut eq_tanh_bypass, "EQ Tanh Bypass").changed() {
                    if let Ok(mut st) = self.standalone_state.lock() {
                        st.eq_tanh_bypass = eq_tanh_bypass;
                    }
                }
                ui.add_space(6.0);
                ui.columns(3, |columns| {
                    let eq_changed = std::cell::Cell::new(false);
                    use egui_knob::{Knob, KnobStyle};

                    distortion::core::ui::main_view::draw_eq_band(
                        &mut columns[0],
                        "BASS",
                        |ui| {
                            if ui
                                .add(
                                    Knob::new(&mut ui_eq_low_freq, 20.0, 1000.0, KnobStyle::Wiper)
                                        .with_size(45.0),
                                )
                                .changed()
                            {
                                eq_changed.set(true);
                            }
                            ui.label(
                                egui::RichText::new(format!("{:.0} Hz", ui_eq_low_freq))
                                    .small()
                                    .monospace(),
                            );
                        },
                        |ui| {
                            if ui
                                .add(
                                    Knob::new(&mut ui_eq_low_gain, -12.0, 12.0, KnobStyle::Wiper)
                                        .with_size(45.0),
                                )
                                .changed()
                            {
                                eq_changed.set(true);
                            }
                            ui.label(
                                egui::RichText::new(format!("{:+.1} dB", ui_eq_low_gain))
                                    .small()
                                    .monospace(),
                            );
                        },
                        |ui| {
                            if ui
                                .add(
                                    Knob::new(&mut ui_eq_low_q, 0.707, 10.0, KnobStyle::Wiper)
                                        .with_size(45.0),
                                )
                                .changed()
                            {
                                eq_changed.set(true);
                            }
                            ui.label(
                                egui::RichText::new(format!("{:.2}", ui_eq_low_q))
                                    .small()
                                    .monospace(),
                            );
                        },
                    );

                    distortion::core::ui::main_view::draw_eq_band(
                        &mut columns[1],
                        "MID",
                        |ui| {
                            if ui
                                .add(
                                    Knob::new(
                                        &mut ui_eq_mid_freq,
                                        100.0,
                                        10000.0,
                                        KnobStyle::Wiper,
                                    )
                                    .with_size(45.0),
                                )
                                .changed()
                            {
                                eq_changed.set(true);
                            }
                            ui.label(
                                egui::RichText::new(format!("{:.0} Hz", ui_eq_mid_freq))
                                    .small()
                                    .monospace(),
                            );
                        },
                        |ui| {
                            if ui
                                .add(
                                    Knob::new(&mut ui_eq_mid_gain, -12.0, 12.0, KnobStyle::Wiper)
                                        .with_size(45.0),
                                )
                                .changed()
                            {
                                eq_changed.set(true);
                            }
                            ui.label(
                                egui::RichText::new(format!("{:+.1} dB", ui_eq_mid_gain))
                                    .small()
                                    .monospace(),
                            );
                        },
                        |ui| {
                            if ui
                                .add(
                                    Knob::new(&mut ui_eq_mid_q, 0.707, 10.0, KnobStyle::Wiper)
                                        .with_size(45.0),
                                )
                                .changed()
                            {
                                eq_changed.set(true);
                            }
                            ui.label(
                                egui::RichText::new(format!("{:.2}", ui_eq_mid_q))
                                    .small()
                                    .monospace(),
                            );
                        },
                    );

                    distortion::core::ui::main_view::draw_eq_band(
                        &mut columns[2],
                        "TREBLE",
                        |ui| {
                            if ui
                                .add(
                                    Knob::new(
                                        &mut ui_eq_high_freq,
                                        1000.0,
                                        20000.0,
                                        KnobStyle::Wiper,
                                    )
                                    .with_size(45.0),
                                )
                                .changed()
                            {
                                eq_changed.set(true);
                            }
                            ui.label(
                                egui::RichText::new(format!("{:.0} Hz", ui_eq_high_freq))
                                    .small()
                                    .monospace(),
                            );
                        },
                        |ui| {
                            if ui
                                .add(
                                    Knob::new(&mut ui_eq_high_gain, -12.0, 12.0, KnobStyle::Wiper)
                                        .with_size(45.0),
                                )
                                .changed()
                            {
                                eq_changed.set(true);
                            }
                            ui.label(
                                egui::RichText::new(format!("{:+.1} dB", ui_eq_high_gain))
                                    .small()
                                    .monospace(),
                            );
                        },
                        |ui| {
                            if ui
                                .add(
                                    Knob::new(&mut ui_eq_high_q, 0.707, 10.0, KnobStyle::Wiper)
                                        .with_size(45.0),
                                )
                                .changed()
                            {
                                eq_changed.set(true);
                            }
                            ui.label(
                                egui::RichText::new(format!("{:.2}", ui_eq_high_q))
                                    .small()
                                    .monospace(),
                            );
                        },
                    );

                    if eq_changed.get() {
                        if let Ok(mut st) = self.standalone_state.lock() {
                            st.eq_low_freq = ui_eq_low_freq;
                            st.eq_low_gain = ui_eq_low_gain;
                            st.eq_low_q = ui_eq_low_q;
                            st.eq_mid_freq = ui_eq_mid_freq;
                            st.eq_mid_gain = ui_eq_mid_gain;
                            st.eq_mid_q = ui_eq_mid_q;
                            st.eq_high_freq = ui_eq_high_freq;
                            st.eq_high_gain = ui_eq_high_gain;
                            st.eq_high_q = ui_eq_high_q;
                        }
                    }
                });
            },
            |ui| {
                let mut changed = false;
                use egui_knob::{Knob, KnobStyle};
                // Ranges espelham exatamente o min..max dos FloatParam do plugin
                // (o widget do plugin também é linear sobre min..max — o skew é ignorado pela knob).
                // gain:              db_to_gain(-30..30) ≈ 0.0316..31.62
                // neural_drive:      db_to_gain(0..30)   ≈ 1.0..31.62
                // neural_output_gain:db_to_gain(-24..12) ≈ 0.0631..3.981
                // neural_amp_volume: db_to_gain(-24..12) ≈ 0.0631..3.981
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label("Neural Drive:");
                        if ui
                            .add(
                                Knob::new(&mut ui_neural_drive, 1.0, 31.6228, KnobStyle::Wiper)
                                    .with_size(45.0),
                            )
                            .changed()
                        {
                            changed = true;
                        }
                        ui.label(
                            egui::RichText::new(format!("{:.2}", ui_neural_drive))
                                .small()
                                .monospace(),
                        );
                    });
                    ui.add_space(10.0);
                    ui.vertical(|ui| {
                        ui.label("Neural Makeup:");
                        if ui
                            .add(
                                Knob::new(
                                    &mut ui_neural_output_gain,
                                    0.0631,
                                    3.9811,
                                    KnobStyle::Wiper,
                                )
                                .with_size(45.0),
                            )
                            .changed()
                        {
                            changed = true;
                        }
                        ui.label(
                            egui::RichText::new(format!("{:.2}", ui_neural_output_gain))
                                .small()
                                .monospace(),
                        );
                    });
                    ui.add_space(10.0);
                    ui.vertical(|ui| {
                        ui.label("Master Output:");
                        if ui
                            .add(
                                Knob::new(
                                    &mut ui_neural_amp_volume,
                                    0.0631,
                                    3.9811,
                                    KnobStyle::Wiper,
                                )
                                .with_size(45.0),
                            )
                            .changed()
                        {
                            changed = true;
                        }
                        ui.label(
                            egui::RichText::new(format!("{:.2}", ui_neural_amp_volume))
                                .small()
                                .monospace(),
                        );
                    });
                    ui.add_space(10.0);
                    ui.vertical(|ui| {
                        ui.label("Master Gain:");
                        if ui
                            .add(
                                Knob::new(&mut ui_gain, 0.0316, 31.6228, KnobStyle::Wiper)
                                    .with_size(45.0),
                            )
                            .changed()
                        {
                            changed = true;
                        }
                        ui.label(
                            egui::RichText::new(format!("{:.2}", ui_gain))
                                .small()
                                .monospace(),
                        );
                    });
                });
                if changed {
                    if let Ok(mut st) = self.standalone_state.lock() {
                        st.neural_drive = ui_neural_drive;
                        st.neural_output_gain = ui_neural_output_gain;
                        st.neural_amp_volume = ui_neural_amp_volume;
                        st.gain = ui_gain;
                    }
                }
            },
            |ui| {
                let mut changed = false;
                use egui_knob::{Knob, KnobStyle};
                // Tab bar. Splitting controls across tabs keeps each row within the
                // panel width instead of overflowing horizontally.
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut ui_mlc_tab, MlcTab::Tone, "Tone");
                    ui.selectable_value(&mut ui_mlc_tab, MlcTab::GainClip, "Gain/Clip");
                    ui.selectable_value(&mut ui_mlc_tab, MlcTab::Harmonics, "Harmonics");
                    ui.selectable_value(&mut ui_mlc_tab, MlcTab::PowerAmp, "Power Amp");
                    ui.selectable_value(&mut ui_mlc_tab, MlcTab::Limiter, "Limiter");
                });
                ui.separator();
                ui.horizontal_wrapped(|ui| {
                    if ui_mlc_tab == MlcTab::GainClip {
                        ui.group(|ui| {
                            ui.label(egui::RichText::new("Gain + Clip").strong());
                            ui.horizontal(|ui| {
                                ui.vertical(|ui| {
                                    ui.label("Gain");
                                    if ui
                                        .add(
                                            Knob::new(
                                                &mut ui_mlc_gain,
                                                0.001,
                                                1.0,
                                                KnobStyle::Wiper,
                                            )
                                            .with_size(45.0),
                                        )
                                        .changed()
                                    {
                                        changed = true;
                                    }
                                    ui.label(
                                        egui::RichText::new(format!("{:.2}", ui_mlc_gain))
                                            .small()
                                            .monospace(),
                                    );
                                });
                                ui.vertical(|ui| {
                                    ui.label("Master");
                                    if ui
                                        .add(
                                            Knob::new(
                                                &mut ui_mlc_master,
                                                0.001,
                                                1.0,
                                                KnobStyle::Wiper,
                                            )
                                            .with_size(45.0),
                                        )
                                        .changed()
                                    {
                                        changed = true;
                                    }
                                    ui.label(
                                        egui::RichText::new(format!("{:.2}", ui_mlc_master))
                                            .small()
                                            .monospace(),
                                    );
                                });
                                ui.vertical(|ui| {
                                    ui.label("Clip per stage");
                                    let stages: [(&str, ClipType); 3] = [
                                        ("1", snap_ui.mlc_clip_type1),
                                        ("2", snap_ui.mlc_clip_type2),
                                        ("3", snap_ui.mlc_clip_type3),
                                    ];
                                    for (idx, (tag, current)) in stages.into_iter().enumerate() {
                                        let mut clip_type = current;
                                        ui.horizontal(|ui| {
                                            ui.label(tag);
                                            egui::ComboBox::from_id_salt(format!(
                                                "standalone_mlc_clip_type{idx}"
                                            ))
                                            .width(120.0)
                                            .selected_text(clip_type.label())
                                            .show_ui(
                                                ui,
                                                |ui| {
                                                    for clip in ClipType::ALL {
                                                        ui.selectable_value(
                                                            &mut clip_type,
                                                            clip,
                                                            clip.label(),
                                                        );
                                                    }
                                                },
                                            );
                                        });
                                        if clip_type != current {
                                            changed = true;
                                            if let Ok(mut st) = self.standalone_state.lock() {
                                                match idx {
                                                    0 => st.mlc_clip_type1 = clip_type,
                                                    1 => st.mlc_clip_type2 = clip_type,
                                                    _ => st.mlc_clip_type3 = clip_type,
                                                }
                                            }
                                        }
                                    }
                                });
                            });
                        });
                    }
                    if ui_mlc_tab == MlcTab::Tone {
                        ui.group(|ui| {
                            ui.label(egui::RichText::new("EQ").strong());
                            ui.horizontal(|ui| {
                                ui.vertical(|ui| {
                                    ui.label("Bass");
                                    if ui
                                        .add(
                                            Knob::new(
                                                &mut ui_mlc_bass,
                                                -12.0,
                                                12.0,
                                                KnobStyle::Wiper,
                                            )
                                            .with_size(45.0),
                                        )
                                        .changed()
                                    {
                                        changed = true;
                                    }
                                    ui.label(
                                        egui::RichText::new(format!("{:+.1} dB", ui_mlc_bass))
                                            .small()
                                            .monospace(),
                                    );
                                });
                                ui.vertical(|ui| {
                                    ui.label("Middle");
                                    if ui
                                        .add(
                                            Knob::new(
                                                &mut ui_mlc_middle,
                                                -12.0,
                                                12.0,
                                                KnobStyle::Wiper,
                                            )
                                            .with_size(45.0),
                                        )
                                        .changed()
                                    {
                                        changed = true;
                                    }
                                    ui.label(
                                        egui::RichText::new(format!("{:+.1} dB", ui_mlc_middle))
                                            .small()
                                            .monospace(),
                                    );
                                });
                                ui.vertical(|ui| {
                                    ui.label("Treble");
                                    if ui
                                        .add(
                                            Knob::new(
                                                &mut ui_mlc_treble,
                                                -12.0,
                                                12.0,
                                                KnobStyle::Wiper,
                                            )
                                            .with_size(45.0),
                                        )
                                        .changed()
                                    {
                                        changed = true;
                                    }
                                    ui.label(
                                        egui::RichText::new(format!("{:+.1} dB", ui_mlc_treble))
                                            .small()
                                            .monospace(),
                                    );
                                });
                            });
                        });
                    }
                    if ui_mlc_tab == MlcTab::Tone {
                        ui.group(|ui| {
                            ui.label(egui::RichText::new("Power Amp").strong());
                            ui.horizontal(|ui| {
                                ui.vertical(|ui| {
                                    ui.label("Presence");
                                    if ui
                                        .add(
                                            Knob::new(
                                                &mut ui_mlc_presence,
                                                -12.0,
                                                12.0,
                                                KnobStyle::Wiper,
                                            )
                                            .with_size(45.0),
                                        )
                                        .changed()
                                    {
                                        changed = true;
                                    }
                                    ui.label(
                                        egui::RichText::new(format!("{:+.1} dB", ui_mlc_presence))
                                            .small()
                                            .monospace(),
                                    );
                                });
                                ui.vertical(|ui| {
                                    ui.label("Depth");
                                    if ui
                                        .add(
                                            Knob::new(
                                                &mut ui_mlc_depth,
                                                -12.0,
                                                12.0,
                                                KnobStyle::Wiper,
                                            )
                                            .with_size(45.0),
                                        )
                                        .changed()
                                    {
                                        changed = true;
                                    }
                                    ui.label(
                                        egui::RichText::new(format!("{:+.1} dB", ui_mlc_depth))
                                            .small()
                                            .monospace(),
                                    );
                                });
                            });
                        });
                    }
                    if ui_mlc_tab == MlcTab::Tone {
                        ui.group(|ui| {
                            ui.label(egui::RichText::new("Switches").strong());
                            let mut bright = snap_ui.mlc_bright;
                            let mut feedback = snap_ui.mlc_feedback;
                            let mut gate_pos = snap_ui.mlc_gate_pos;
                            let mut m45 = snap_ui.mlc_m45;
                            let mut warclaw = snap_ui.mlc_warclaw;
                            ui.horizontal(|ui| {
                                ui.label("Bright");
                                egui::ComboBox::from_id_salt("standalone_mlc_bright")
                                    .selected_text(match bright {
                                        MlcBright::I => "I",
                                        MlcBright::Ii => "II",
                                    })
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(&mut bright, MlcBright::I, "I");
                                        ui.selectable_value(&mut bright, MlcBright::Ii, "II");
                                    });
                                if ui.checkbox(&mut m45, "M45").changed() {
                                    changed = true;
                                }
                                if ui.checkbox(&mut warclaw, "WARCLAW").changed() {
                                    changed = true;
                                }
                                ui.label("Feedback");
                                egui::ComboBox::from_id_salt("standalone_mlc_feedback")
                                    .selected_text(match feedback {
                                        MlcFeedback::Lo => "Lo",
                                        MlcFeedback::Hi => "Hi",
                                    })
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(&mut feedback, MlcFeedback::Lo, "Lo");
                                        ui.selectable_value(&mut feedback, MlcFeedback::Hi, "Hi");
                                    });
                                ui.label("Gate Pos");
                                egui::ComboBox::from_id_salt("standalone_mlc_gate_pos")
                                    .selected_text(match gate_pos {
                                        MlcGatePos::Pre => "Pre",
                                        MlcGatePos::Post => "Post",
                                    })
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(&mut gate_pos, MlcGatePos::Pre, "Pre");
                                        ui.selectable_value(
                                            &mut gate_pos,
                                            MlcGatePos::Post,
                                            "Post",
                                        );
                                    });
                            });
                            if bright != snap_ui.mlc_bright
                                || feedback != snap_ui.mlc_feedback
                                || gate_pos != snap_ui.mlc_gate_pos
                                || m45 != snap_ui.mlc_m45
                                || warclaw != snap_ui.mlc_warclaw
                            {
                                changed = true;
                                if let Ok(mut st) = self.standalone_state.lock() {
                                    st.mlc_bright = bright;
                                    st.mlc_feedback = feedback;
                                    st.mlc_gate_pos = gate_pos;
                                    st.mlc_m45 = m45;
                                    st.mlc_warclaw = warclaw;
                                }
                            }
                        });
                    }
                    if ui_mlc_tab == MlcTab::Tone {
                        ui.group(|ui| {
                            ui.label(egui::RichText::new("Tight").strong());
                            let mut tight = snap_ui.mlc_tight;
                            ui.horizontal(|ui| {
                                if ui.checkbox(&mut tight, "Enable").changed() {
                                    changed = true;
                                    if let Ok(mut st) = self.standalone_state.lock() {
                                        st.mlc_tight = tight;
                                    }
                                }
                                ui.label(
                                    egui::RichText::new("HPF 80Hz entre estágios")
                                        .small()
                                        .color(egui::Color32::GRAY),
                                );
                            });
                        });
                    }
                    if ui_mlc_tab == MlcTab::Harmonics {
                        ui.group(|ui| {
                            ui.label(egui::RichText::new("Harmonics").strong());
                            let mut asymmetry_enable = snap_ui.mlc_asymmetry_enable;
                            ui.vertical(|ui| {
                                if ui.checkbox(&mut asymmetry_enable, "Enable").changed() {
                                    changed = true;
                                    if let Ok(mut st) = self.standalone_state.lock() {
                                        st.mlc_asymmetry_enable = asymmetry_enable;
                                    }
                                }
                                ui.horizontal(|ui| {
                                    ui.label("odd");
                                    if ui
                                        .add(egui::Slider::new(&mut ui_mlc_asymmetry, 0.0..=1.0))
                                        .changed()
                                    {
                                        changed = true;
                                    }
                                    ui.label("even");
                                });
                            });
                        });
                    }
                    if ui_mlc_tab == MlcTab::GainClip {
                        ui.group(|ui| {
                            ui.label(egui::RichText::new("Pre-Shape").strong());
                            let mut preshape = snap_ui.mlc_preshape;
                            ui.vertical(|ui| {
                                if ui.checkbox(&mut preshape, "Enable").changed() {
                                    changed = true;
                                    if let Ok(mut st) = self.standalone_state.lock() {
                                        st.mlc_preshape = preshape;
                                    }
                                }
                                ui.horizontal(|ui| {
                                    ui.vertical(|ui| {
                                        ui.label("Tight");
                                        if ui
                                            .add(
                                                Knob::new(
                                                    &mut ui_mlc_preshape_tight,
                                                    -6.0,
                                                    0.0,
                                                    KnobStyle::Wiper,
                                                )
                                                .with_size(45.0),
                                            )
                                            .changed()
                                        {
                                            changed = true;
                                        }
                                        ui.label(
                                            egui::RichText::new(format!(
                                                "{:+.1} dB",
                                                ui_mlc_preshape_tight
                                            ))
                                            .small()
                                            .monospace(),
                                        );
                                    });
                                    ui.vertical(|ui| {
                                        ui.label("Bite");
                                        if ui
                                            .add(
                                                Knob::new(
                                                    &mut ui_mlc_preshape_bite,
                                                    0.0,
                                                    6.0,
                                                    KnobStyle::Wiper,
                                                )
                                                .with_size(45.0),
                                            )
                                            .changed()
                                        {
                                            changed = true;
                                        }
                                        ui.label(
                                            egui::RichText::new(format!(
                                                "{:+.1} dB",
                                                ui_mlc_preshape_bite
                                            ))
                                            .small()
                                            .monospace(),
                                        );
                                    });
                                });
                            });
                        });
                    }
                    if ui_mlc_tab == MlcTab::GainClip {
                        ui.group(|ui| {
                            ui.label(egui::RichText::new("Clean Blend + Sag").strong());
                            ui.vertical(|ui| {
                                if ui
                                    .add(
                                        egui::Slider::new(&mut ui_mlc_clean_blend, 0.0..=0.25)
                                            .text("Dry"),
                                    )
                                    .changed()
                                {
                                    changed = true;
                                }
                                if ui
                                    .add(egui::Slider::new(&mut ui_mlc_sag, 0.0..=1.0).text("Sag"))
                                    .changed()
                                {
                                    changed = true;
                                }
                            });
                        });
                    }
                    if ui_mlc_tab == MlcTab::Harmonics {
                        ui.group(|ui| {
                            ui.label(egui::RichText::new("Chebyshev").strong());
                            ui.vertical(|ui| {
                                if ui
                                    .add(egui::Slider::new(&mut ui_mlc_h2, 0.0..=1.0).text("H2"))
                                    .changed()
                                {
                                    changed = true;
                                }
                                if ui
                                    .add(egui::Slider::new(&mut ui_mlc_h3, 0.0..=1.0).text("H3"))
                                    .changed()
                                {
                                    changed = true;
                                }
                                if ui
                                    .add(egui::Slider::new(&mut ui_mlc_h4, 0.0..=1.0).text("H4"))
                                    .changed()
                                {
                                    changed = true;
                                }
                            });
                        });
                    }
                    if ui_mlc_tab == MlcTab::Tone {
                        ui.group(|ui| {
                            ui.label(egui::RichText::new("Gate").strong());
                            ui.vertical(|ui| {
                                ui.label("Threshold");
                                if ui
                                    .add(
                                        Knob::new(&mut ui_mlc_gate, -80.0, 0.0, KnobStyle::Wiper)
                                            .with_size(45.0),
                                    )
                                    .changed()
                                {
                                    changed = true;
                                }
                                ui.label(
                                    egui::RichText::new(format!("{:.0} dB", ui_mlc_gate))
                                        .small()
                                        .monospace(),
                                );
                            });
                        });
                    }
                    if ui_mlc_tab == MlcTab::Limiter {
                        ui.group(|ui| {
                            ui.label(egui::RichText::new("LIMITER").strong());
                            ui.horizontal(|ui| {
                                ui.vertical(|ui| {
                                    ui.add_space(14.0);
                                    if ui.checkbox(&mut ui_limiter_enable, "Enable").changed() {
                                        changed = true;
                                    }
                                });
                                ui.vertical_centered(|ui| {
                                    ui.label(
                                        egui::RichText::new("Ceiling")
                                            .small()
                                            .color(egui::Color32::GRAY),
                                    );
                                    if ui
                                        .add(
                                            Knob::new(
                                                &mut ui_limiter_ceiling,
                                                -12.0,
                                                0.0,
                                                KnobStyle::Wiper,
                                            )
                                            .with_size(45.0),
                                        )
                                        .changed()
                                    {
                                        changed = true;
                                    }
                                    ui.label(
                                        egui::RichText::new(format!(
                                            "{:.1} dB",
                                            ui_limiter_ceiling
                                        ))
                                        .small()
                                        .monospace(),
                                    );
                                });
                                ui.vertical_centered(|ui| {
                                    ui.label(
                                        egui::RichText::new("Release")
                                            .small()
                                            .color(egui::Color32::GRAY),
                                    );
                                    if ui
                                        .add(
                                            Knob::new(
                                                &mut ui_limiter_release,
                                                10.0,
                                                500.0,
                                                KnobStyle::Wiper,
                                            )
                                            .with_size(45.0),
                                        )
                                        .changed()
                                    {
                                        changed = true;
                                    }
                                    ui.label(
                                        egui::RichText::new(format!(
                                            "{:.0} ms",
                                            ui_limiter_release
                                        ))
                                        .small()
                                        .monospace(),
                                    );
                                });
                            });
                        });
                    }
                    // --- Tier 2.2 / 3.x controls ---
                    // Compact knob helper (value passed by ref, `changed` flag by ref).
                    let add_knob = |ui: &mut egui::Ui,
                                    label: &str,
                                    val: &mut f32,
                                    lo: f32,
                                    hi: f32,
                                    unit: &str,
                                    ch: &mut bool| {
                        ui.vertical(|ui| {
                            ui.label(label);
                            if ui
                                .add(Knob::new(val, lo, hi, KnobStyle::Wiper).with_size(45.0))
                                .changed()
                            {
                                *ch = true;
                            }
                            ui.label(
                                egui::RichText::new(format!("{:.1}{}", *val, unit))
                                    .small()
                                    .monospace(),
                            );
                        });
                    };

                    if ui_mlc_tab == MlcTab::Tone {
                        ui.group(|ui| {
                            ui.label(egui::RichText::new("Tone Stack").strong());
                            let mut ts_model = snap_ui.mlc_ts_model;
                            let names = MlcTsModel::variants();
                            egui::ComboBox::from_id_salt("standalone_mlc_ts_model")
                                .width(150.0)
                                .selected_text(
                                    names.get(ts_model.to_index()).copied().unwrap_or(""),
                                )
                                .show_ui(ui, |ui| {
                                    for (i, name) in names.iter().enumerate() {
                                        ui.selectable_value(
                                            &mut ts_model,
                                            MlcTsModel::from_index(i),
                                            *name,
                                        );
                                    }
                                });
                            if ts_model != snap_ui.mlc_ts_model {
                                changed = true;
                                if let Ok(mut st) = self.standalone_state.lock() {
                                    st.mlc_ts_model = ts_model;
                                }
                            }
                        });
                    }

                    if ui_mlc_tab == MlcTab::GainClip {
                        ui.group(|ui| {
                            ui.label(egui::RichText::new("Tube").strong());
                            ui.horizontal(|ui| {
                                ui.vertical(|ui| {
                                    ui.label("Model");
                                    let mut tube_model = snap_ui.mlc_tube_model;
                                    let names = MlcTubeModel::variants();
                                    egui::ComboBox::from_id_salt("standalone_mlc_tube_model")
                                        .width(110.0)
                                        .selected_text(
                                            names.get(tube_model.to_index()).copied().unwrap_or(""),
                                        )
                                        .show_ui(ui, |ui| {
                                            for (i, name) in names.iter().enumerate() {
                                                ui.selectable_value(
                                                    &mut tube_model,
                                                    MlcTubeModel::from_index(i),
                                                    *name,
                                                );
                                            }
                                        });
                                    let mut tube_bypass = snap_ui.mlc_tube_bypass;
                                    if ui.checkbox(&mut tube_bypass, "Bypass").changed() {
                                        changed = true;
                                        if let Ok(mut st) = self.standalone_state.lock() {
                                            st.mlc_tube_bypass = tube_bypass;
                                        }
                                    }
                                    if tube_model != snap_ui.mlc_tube_model {
                                        changed = true;
                                        if let Ok(mut st) = self.standalone_state.lock() {
                                            st.mlc_tube_model = tube_model;
                                        }
                                    }
                                });
                                add_knob(
                                    ui,
                                    "Drive",
                                    &mut ui_mlc_tube_drive,
                                    -20.0,
                                    20.0,
                                    " dB",
                                    &mut changed,
                                );
                            });
                        });
                        ui.group(|ui| {
                            ui.label(egui::RichText::new("Multi-Band Clip").strong());
                            let mut mbc_bypass = snap_ui.mlc_mbc_bypass;
                            if ui.checkbox(&mut mbc_bypass, "Bypass").changed() {
                                changed = true;
                                if let Ok(mut st) = self.standalone_state.lock() {
                                    st.mlc_mbc_bypass = mbc_bypass;
                                }
                            }
                            ui.horizontal(|ui| {
                                add_knob(
                                    ui,
                                    "XOver Lo",
                                    &mut ui_mlc_mbc_cf_lo,
                                    100.0,
                                    800.0,
                                    " Hz",
                                    &mut changed,
                                );
                                add_knob(
                                    ui,
                                    "XOver Hi",
                                    &mut ui_mlc_mbc_cf_hi,
                                    1500.0,
                                    6000.0,
                                    " Hz",
                                    &mut changed,
                                );
                                add_knob(
                                    ui,
                                    "Drv Lo",
                                    &mut ui_mlc_mbc_drive_lo,
                                    0.1,
                                    4.0,
                                    "",
                                    &mut changed,
                                );
                                add_knob(
                                    ui,
                                    "Drv Mid",
                                    &mut ui_mlc_mbc_drive_mid,
                                    0.1,
                                    4.0,
                                    "",
                                    &mut changed,
                                );
                                add_knob(
                                    ui,
                                    "Drv Hi",
                                    &mut ui_mlc_mbc_drive_hi,
                                    0.1,
                                    4.0,
                                    "",
                                    &mut changed,
                                );
                            });
                        });
                    }

                    if ui_mlc_tab == MlcTab::Harmonics {
                        ui.group(|ui| {
                            ui.label(egui::RichText::new("Quality (ADAA)").strong());
                            let mut adaa = snap_ui.mlc_adaa_order;
                            let names = MlcAdaaOrder::variants();
                            egui::ComboBox::from_id_salt("standalone_mlc_adaa_order")
                                .width(90.0)
                                .selected_text(names.get(adaa.to_index()).copied().unwrap_or(""))
                                .show_ui(ui, |ui| {
                                    for (i, name) in names.iter().enumerate() {
                                        ui.selectable_value(
                                            &mut adaa,
                                            MlcAdaaOrder::from_index(i),
                                            *name,
                                        );
                                    }
                                });
                            if adaa != snap_ui.mlc_adaa_order {
                                changed = true;
                                if let Ok(mut st) = self.standalone_state.lock() {
                                    st.mlc_adaa_order = adaa;
                                }
                            }
                        });
                    }

                    if ui_mlc_tab == MlcTab::PowerAmp {
                        ui.group(|ui| {
                            ui.label(egui::RichText::new("NFB Loop").strong());
                            let mut nfb_bypass = snap_ui.mlc_nfb_bypass;
                            if ui.checkbox(&mut nfb_bypass, "Bypass").changed() {
                                changed = true;
                                if let Ok(mut st) = self.standalone_state.lock() {
                                    st.mlc_nfb_bypass = nfb_bypass;
                                }
                            }
                            ui.horizontal(|ui| {
                                add_knob(
                                    ui,
                                    "Presence",
                                    &mut ui_mlc_nfb_presence,
                                    0.0,
                                    1.0,
                                    "",
                                    &mut changed,
                                );
                                add_knob(
                                    ui,
                                    "Resonance",
                                    &mut ui_mlc_nfb_resonance,
                                    0.0,
                                    1.0,
                                    "",
                                    &mut changed,
                                );
                                add_knob(
                                    ui,
                                    "Depth",
                                    &mut ui_mlc_nfb_depth,
                                    0.0,
                                    1.0,
                                    "",
                                    &mut changed,
                                );
                            });
                        });
                    }
                });
                if changed {
                    if let Ok(mut st) = self.standalone_state.lock() {
                        st.mlc_gain = ui_mlc_gain;
                        st.mlc_master = ui_mlc_master;
                        st.mlc_bass = ui_mlc_bass;
                        st.mlc_middle = ui_mlc_middle;
                        st.mlc_treble = ui_mlc_treble;
                        st.mlc_presence = ui_mlc_presence;
                        st.mlc_depth = ui_mlc_depth;
                        st.mlc_gate = ui_mlc_gate;
                        st.mlc_asymmetry = ui_mlc_asymmetry;
                        st.mlc_preshape_tight = ui_mlc_preshape_tight;
                        st.mlc_preshape_bite = ui_mlc_preshape_bite;
                        st.mlc_clean_blend = ui_mlc_clean_blend;
                        st.mlc_sag = ui_mlc_sag;
                        st.mlc_h2 = ui_mlc_h2;
                        st.mlc_h3 = ui_mlc_h3;
                        st.mlc_h4 = ui_mlc_h4;
                        st.mlc_tube_drive = ui_mlc_tube_drive;
                        st.mlc_nfb_presence = ui_mlc_nfb_presence;
                        st.mlc_nfb_resonance = ui_mlc_nfb_resonance;
                        st.mlc_nfb_depth = ui_mlc_nfb_depth;
                        st.mlc_mbc_cf_lo = ui_mlc_mbc_cf_lo;
                        st.mlc_mbc_cf_hi = ui_mlc_mbc_cf_hi;
                        st.mlc_mbc_drive_lo = ui_mlc_mbc_drive_lo;
                        st.mlc_mbc_drive_mid = ui_mlc_mbc_drive_mid;
                        st.mlc_mbc_drive_hi = ui_mlc_mbc_drive_hi;
                        st.limiter_enable = ui_limiter_enable;
                        st.limiter_ceiling = ui_limiter_ceiling;
                        st.limiter_release = ui_limiter_release;
                    }
                }
            },
            |ui| {
                // Build a runtime and, only on success, hand it to the audio thread.
                // Returns whether the build succeeded so callers gate persistence on it
                // (B6: never persist a broken selection).
                let build_and_publish = |bytes: &[u8]| -> bool {
                    let srate = cab_sr.load(Ordering::Relaxed) as f32;
                    let maxb = cab_maxb.load(Ordering::Relaxed);
                    match CabinetRuntime::build(bytes, srate, maxb) {
                        Ok(rt) => {
                            cab_mailbox.publish(rt);
                            if let Ok(mut e) = cab_err.lock() {
                                *e = None;
                            }
                            true
                        }
                        Err(err) => {
                            if let Ok(mut e) = cab_err.lock() {
                                *e = Some(err.to_string());
                            }
                            false
                        }
                    }
                };

                distortion::core::ui::draw_cabinet_panel(
                    ui,
                    &cab_ir_list,
                    &cab_active_hash,
                    cab_err_now.as_deref(),
                    || snap_ui.cabinet_bypass,
                    |v| {
                        if let Ok(mut st) = cab_state.lock() {
                            st.cabinet_bypass = v;
                        }
                    },
                    || snap_ui.cabinet_level,
                    |v| {
                        if let Ok(mut st) = cab_state.lock() {
                            st.cabinet_level = v;
                        }
                    },
                    || snap_ui.cabinet_mix,
                    |v| {
                        if let Ok(mut st) = cab_state.lock() {
                            st.cabinet_mix = v;
                        }
                    },
                    // select_ir — build first, persist selection only on success.
                    |hash: String| {
                        let bytes = match cab_lib.lock() {
                            Ok(l) => match l.get_ir_by_hash(&hash) {
                                Ok((_, b)) => Some(b),
                                Err(err) => {
                                    if let Ok(mut e) = cab_err.lock() {
                                        *e = Some(err.to_string());
                                    }
                                    None
                                }
                            },
                            Err(_) => None,
                        };
                        if let Some(b) = bytes {
                            if build_and_publish(&b) {
                                if let Ok(l) = cab_lib.lock() {
                                    let _ = l.set_selected_hash(&hash);
                                }
                                if let Ok(mut st) = cab_state.lock() {
                                    st.cab_active_hash = hash.clone();
                                }
                                save_persisted_hash(&hash);
                            }
                        }
                    },
                    // import_ir — store, then build; persist selection only on success.
                    || {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("WAV", &["wav"])
                            .pick_file()
                        {
                            match std::fs::read(&path) {
                                Ok(bytes) => {
                                    let fname = path
                                        .file_name()
                                        .and_then(|s| s.to_str())
                                        .unwrap_or("imported.wav")
                                        .to_string();
                                    let imported = cab_lib.lock().ok().and_then(|l| {
                                        match l.import_ir(&bytes, &fname) {
                                            Ok(meta) => Some(meta.content_hash),
                                            Err(err) => {
                                                if let Ok(mut e) = cab_err.lock() {
                                                    *e = Some(err.to_string());
                                                }
                                                None
                                            }
                                        }
                                    });
                                    if let Some(hash) = imported {
                                        if build_and_publish(&bytes) {
                                            if let Ok(l) = cab_lib.lock() {
                                                let _ = l.set_selected_hash(&hash);
                                            }
                                            if let Ok(mut st) = cab_state.lock() {
                                                st.cab_active_hash = hash.clone();
                                            }
                                            save_persisted_hash(&hash);
                                        }
                                    }
                                }
                                Err(err) => {
                                    if let Ok(mut e) = cab_err.lock() {
                                        *e = Some(err.to_string());
                                    }
                                }
                            }
                        }
                    },
                    // delete_ir
                    |hash: String| {
                        if let Ok(l) = cab_lib.lock() {
                            let _ = l.delete_ir(&hash);
                        }
                        let was_active = cab_state
                            .lock()
                            .map(|st| st.cab_active_hash == hash)
                            .unwrap_or(false);
                        if was_active {
                            if let Ok(mut st) = cab_state.lock() {
                                st.cab_active_hash.clear();
                            }
                            save_persisted_hash("");
                            cab_mailbox.clear();
                        }
                    },
                    // rename_ir
                    |hash: String, name: String| {
                        if let Ok(l) = cab_lib.lock() {
                            let _ = l.rename_ir(&hash, &name);
                        }
                    },
                    // export_ir
                    |hash: String| {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("WAV", &["wav"])
                            .set_file_name("cabinet_ir.wav")
                            .save_file()
                        {
                            let bytes = cab_lib
                                .lock()
                                .ok()
                                .and_then(|l| l.get_ir_by_hash(&hash).ok().map(|(_, b)| b));
                            if let Some(b) = bytes {
                                if let Err(err) = std::fs::write(&path, &b) {
                                    if let Ok(mut e) = cab_err.lock() {
                                        *e = Some(err.to_string());
                                    }
                                }
                            }
                        }
                    },
                );
            },
        );
        self.mlc_tab = ui_mlc_tab;

        #[cfg(feature = "lab")]
        {
            let lab_error = self.lab_error.lock().ok().and_then(|error| error.clone());
            draw_lab_panel(
                ctx,
                self.lab.as_deref(),
                &mut self.lab_ui,
                lab_error.as_deref(),
            );
        }
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        let _ = self.tx_cmd.send(AudioCommand::Stop);
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([1000.0, 700.0]),
        ..Default::default()
    };
    eframe::run_native(
        "BaseIO Standalone",
        options,
        Box::new(|cc| Ok(Box::new(StandaloneApp::new(cc)))),
    )
}
