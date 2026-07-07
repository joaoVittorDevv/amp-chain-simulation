use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Stream;
use distortion::bridge::{mlc_zero_v::MlcZeroVProcessor, mojo::MojoProcessor, ExternalProcessor};
use distortion::core::cabinet::{CabinetEngine, CabinetLibrary, CabinetMailbox, CabinetRuntime};
use distortion::core::dsp::{AnalyzerDsp, FFT_SIZE};
use distortion::core::state::plugin_params::{AmpModel, MlcBright, MlcFeedback, MlcGatePos};
#[cfg(feature = "lab")]
use distortion::core::ui::{draw_lab_panel, LabUiState};
use distortion::core::ui::{render_shared_panels, ActivePanel};
#[cfg(feature = "lab")]
use distortion::lab::{default_categories, Lab, PipelineManager};
use eframe::egui;
use fft_convolver::FFTConvolver;
use rtrb::{Consumer, RingBuffer};
use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

/// Default cabinet IR embedded in the standalone binary (relative to src/bin/).
const DEFAULT_CABINET_IR: &[u8] = include_bytes!("../../neural/drive/cabinet_ir.wav");

/// Pre-EQ (tone-stack) IR embedded in the standalone binary. Replaces the old
/// hardcoded absolute path so the linear pre-EQ stage no longer depends on a file.
const DEFAULT_PRE_EQ_IR: &[u8] = include_bytes!("../../neural/drive/pre_eq_ir.wav");

const CROSSFADE_LEN: usize = 480;

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
    eq_tanh_bypass: bool,
    gain: f32,
    bypass: bool,
    // --- Cabinet IR ---
    cabinet_bypass: bool,
    cabinet_level: f32,
    cabinet_mix: f32,
    cab_active_hash: String,
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
            eq_tanh_bypass: false,
            gain: 1.0,
            bypass: false,
            cabinet_bypass: false,
            cabinet_level: 1.0,
            cabinet_mix: 1.0,
            cab_active_hash: String::new(),
        }
    }
}

/// Copy-able subset of [`StandaloneState`] read by the audio callback, so the
/// audio thread never clones the `cab_active_hash` String (no heap allocation).
#[derive(Clone, Copy)]
struct AudioSnapshot {
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
    eq_tanh_bypass: bool,
    gain: f32,
    bypass: bool,
    cabinet_bypass: bool,
    cabinet_level: f32,
    cabinet_mix: f32,
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
            eq_tanh_bypass: self.eq_tanh_bypass,
            gain: self.gain,
            bypass: self.bypass,
            cabinet_bypass: self.cabinet_bypass,
            cabinet_level: self.cabinet_level,
            cabinet_mix: self.cabinet_mix,
        }
    }
}

impl Default for AudioSnapshot {
    fn default() -> Self {
        StandaloneState::default().audio()
    }
}

#[inline(always)]
fn process_standalone_amp(
    amp_model: AmpModel,
    snap: &AudioSnapshot,
    neural_amp_l: &mut MojoProcessor,
    neural_amp_r: &mut MojoProcessor,
    mlc_l: &mut Option<MlcZeroVProcessor>,
    mlc_r: &mut Option<MlcZeroVProcessor>,
    buf_l: &mut [f32],
    buf_r: &mut [f32],
) {
    match amp_model {
        AmpModel::Neural => {
            if snap.neural_active {
                neural_amp_l.set_drive(snap.neural_drive);
                neural_amp_l.set_output_gain(snap.neural_output_gain);
                neural_amp_l.process_block(buf_l.as_mut_ptr(), buf_l.len());

                neural_amp_r.set_drive(snap.neural_drive);
                neural_amp_r.set_output_gain(snap.neural_output_gain);
                neural_amp_r.process_block(buf_r.as_mut_ptr(), buf_r.len());
            }
        }
        AmpModel::MlcZeroV => {
            let bright = match snap.mlc_bright {
                MlcBright::I => 0.0,
                MlcBright::Ii => 1.0,
            };
            let feedback = match snap.mlc_feedback {
                MlcFeedback::Lo => 0.0,
                MlcFeedback::Hi => 1.0,
            };
            let gate_pos = match snap.mlc_gate_pos {
                MlcGatePos::Pre => 0.0,
                MlcGatePos::Post => 1.0,
            };
            if let Some(mlc) = mlc_l {
                mlc.set_gain(snap.mlc_gain);
                mlc.set_master(snap.mlc_master);
                mlc.set_bass(snap.mlc_bass);
                mlc.set_middle(snap.mlc_middle);
                mlc.set_treble(snap.mlc_treble);
                mlc.set_presence(snap.mlc_presence);
                mlc.set_depth(snap.mlc_depth);
                mlc.set_gate(snap.mlc_gate);
                mlc.set_bright(bright);
                mlc.set_m45(snap.mlc_m45);
                mlc.set_warclaw(snap.mlc_warclaw);
                mlc.set_feedback(feedback);
                mlc.set_gate_pos(gate_pos);
                mlc.process_block(buf_l.as_mut_ptr(), buf_l.len());
            }
            if let Some(mlc) = mlc_r {
                mlc.set_gain(snap.mlc_gain);
                mlc.set_master(snap.mlc_master);
                mlc.set_bass(snap.mlc_bass);
                mlc.set_middle(snap.mlc_middle);
                mlc.set_treble(snap.mlc_treble);
                mlc.set_presence(snap.mlc_presence);
                mlc.set_depth(snap.mlc_depth);
                mlc.set_gate(snap.mlc_gate);
                mlc.set_bright(bright);
                mlc.set_m45(snap.mlc_m45);
                mlc.set_warclaw(snap.mlc_warclaw);
                mlc.set_feedback(feedback);
                mlc.set_gate_pos(gate_pos);
                mlc.process_block(buf_r.as_mut_ptr(), buf_r.len());
            }
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

#[derive(Clone)]
struct DeviceContext {
    name: String,
    raw_name: String,
    channels: u16,
    sample_rate: u32,
}

enum AudioCommand {
    RefreshDevices(cpal::HostId),
    ApplyRouting {
        host_id: cpal::HostId,
        input: Option<(String, usize, usize)>,
        output: Option<(String, usize, usize)>,
        buffer_size: u32,
    },
    Stop,
}

enum AudioEvent {
    DevicesRefreshed {
        inputs: Vec<DeviceContext>,
        outputs: Vec<DeviceContext>,
    },
    StreamStarted(Result<(), String>),
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

    sample_rate_warning: Option<String>,
    last_audio_error: Option<String>,
    last_audio_error_details: Option<String>,
    show_error_popup: bool,

    input_is_mono: bool,
    output_is_mono: bool,

    show_settings: bool,
    is_loading: bool,
    active_panel: ActivePanel,

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

fn audio_worker(
    rx_cmd: Receiver<AudioCommand>,
    tx_event: Sender<AudioEvent>,
    consumer_mutex: Arc<Mutex<Option<Consumer<f32>>>>,
    standalone_state: Arc<Mutex<StandaloneState>>,
    cabinet_mailbox: Arc<CabinetMailbox>,
    cabinet_library: Arc<Mutex<CabinetLibrary>>,
    cabinet_sr: Arc<AtomicU32>,
    cabinet_max_block: Arc<AtomicUsize>,
) {
    let mut _input_stream: Option<Stream> = None;
    let mut _output_stream: Option<Stream> = None;

    for cmd in rx_cmd {
        match cmd {
            AudioCommand::RefreshDevices(host_id) => {
                let mut inputs_list = vec![];
                let mut outputs_list = vec![];

                if let Ok(host) = cpal::host_from_id(host_id) {
                    if let Ok(devs) = host.input_devices() {
                        for dev in devs {
                            if let Ok(config) = dev.default_input_config() {
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
                                inputs_list.push(DeviceContext {
                                    name: b_name,
                                    raw_name: raw_name.clone(),
                                    channels: config.channels(),
                                    sample_rate: config.sample_rate().0,
                                });
                            }
                        }
                    }
                    if let Ok(devs) = host.output_devices() {
                        for dev in devs {
                            if let Ok(config) = dev.default_output_config() {
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
                                outputs_list.push(DeviceContext {
                                    name: b_name,
                                    raw_name: raw_name.clone(),
                                    channels: config.channels(),
                                    sample_rate: config.sample_rate().0,
                                });
                            }
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

                let (mut analyzer_producer, analyzer_consumer) = RingBuffer::new(1024 * 64);
                if let Ok(mut cons_lock) = consumer_mutex.lock() {
                    *cons_lock = Some(analyzer_consumer);
                }

                let has_both = input.is_some() && output.is_some();
                let (mut pt_producer, mut pt_consumer) = if has_both {
                    let (p, c) = RingBuffer::new((buffer_size * 8).max(2048) as usize);
                    (Some(p), Some(c))
                } else {
                    (None, None)
                };

                // Processadores Neurais Mojo (L/R) — FFI Zero-Copy
                let mut neural_amp_l = MojoProcessor::new();
                let mut neural_amp_r = MojoProcessor::new();
                neural_amp_l.init(44100.0);
                neural_amp_r.init(44100.0);
                neural_amp_l.set_drive(2.0);
                neural_amp_l.set_output_gain(0.5);
                neural_amp_r.set_drive(2.0);
                neural_amp_r.set_output_gain(0.5);
                let state_clone = standalone_state.clone();

                if let Ok(host) = cpal::host_from_id(host_id) {
                    if let Some((raw_name, left, right)) = input {
                        if let Ok(devs) = host.input_devices() {
                            if let Some(device) = devs
                                .into_iter()
                                .find(|d| d.name().unwrap_or_default() == raw_name)
                            {
                                if let Ok(config) = device.default_input_config() {
                                    let mut strict_config: cpal::StreamConfig =
                                        config.clone().into();
                                    strict_config.buffer_size = cpal::BufferSize::Default;
                                    let channels = strict_config.channels;
                                    let l_idx = left.min((channels.saturating_sub(1)) as usize);
                                    let r_idx = right.min((channels.saturating_sub(1)) as usize);

                                    let stream_res = match config.sample_format() {
                                        cpal::SampleFormat::F32 => {
                                            // INICIALIZAÇÃO DSP: Fora do loop de processamento!
                                            // Pegamos o sample_rate do config e inicializamos uma única vez
                                            let s_rate = strict_config.sample_rate.0 as f32;

                                            // Faust
                                            let mut local_faust_l =
                                                distortion::bridge::faust::FaustProcessor::new();
                                            let mut local_faust_r =
                                                distortion::bridge::faust::FaustProcessor::new();
                                            if let Some(f) = &mut local_faust_l {
                                                f.init(s_rate);
                                            }
                                            if let Some(f) = &mut local_faust_r {
                                                f.init(s_rate);
                                            }

                                            // Mojo (Substituindo o dummy que estava fora do loop antes)
                                            neural_amp_l.init(s_rate);
                                            neural_amp_r.init(s_rate);

                                            let mut mlc_l = MlcZeroVProcessor::new();
                                            let mut mlc_r = MlcZeroVProcessor::new();
                                            if let Some(mlc) = &mut mlc_l {
                                                mlc.init(s_rate);
                                            }
                                            if let Some(mlc) = &mut mlc_r {
                                                mlc.init(s_rate);
                                            }

                                            let mut pre_eq_l = FFTConvolver::default();
                                            let mut pre_eq_r = FFTConvolver::default();

                                            // Pré-EQ (tone-stack) — IR fixo embedado no binário (sem path absoluto).
                                            if let Some(ir) = decode_wav_flat(DEFAULT_PRE_EQ_IR) {
                                                if !ir.is_empty() {
                                                    let _ =
                                                        pre_eq_l.init(buffer_size as usize, &ir);
                                                    let _ =
                                                        pre_eq_r.init(buffer_size as usize, &ir);
                                                }
                                            }

                                            // ESTÁGIO 4: Cabinet IR gerenciado (biblioteca + engine, sem path hardcoded).
                                            cabinet_sr.store(s_rate as u32, Ordering::Relaxed);
                                            cabinet_max_block
                                                .store(buffer_size as usize, Ordering::Relaxed);
                                            let mut cabinet_engine = CabinetEngine::with_mailbox(
                                                cabinet_mailbox.clone(),
                                            );
                                            cabinet_engine.set_sample_rate(s_rate);
                                            {
                                                // Resolver o IR ativo e publicar um runtime para este sample rate.
                                                let mut hash = state_clone
                                                    .lock()
                                                    .ok()
                                                    .map(|s| s.cab_active_hash.clone())
                                                    .unwrap_or_default();
                                                if hash.is_empty() {
                                                    hash = cabinet_library
                                                        .lock()
                                                        .ok()
                                                        .and_then(|l| {
                                                            l.get_selected_hash().ok().flatten()
                                                        })
                                                        .unwrap_or_default();
                                                }
                                                if !hash.is_empty() {
                                                    if let Ok(l) = cabinet_library.lock() {
                                                        if let Ok((_, bytes)) =
                                                            l.get_ir_by_hash(&hash)
                                                        {
                                                            if let Ok(rt) = CabinetRuntime::build(
                                                                &bytes,
                                                                s_rate,
                                                                buffer_size as usize,
                                                            ) {
                                                                cabinet_mailbox.publish(rt);
                                                            }
                                                        }
                                                    }
                                                }
                                            }

                                            // Buffers temporários para processamento in-place em bloco
                                            let mut buf_l = vec![0.0; buffer_size as usize];
                                            let mut buf_r = vec![0.0; buffer_size as usize];
                                            let mut temp_l = vec![0.0; buffer_size as usize];
                                            let mut temp_r = vec![0.0; buffer_size as usize];
                                            let mut previous_amp_model = AmpModel::Neural;
                                            let mut crossfade_sample = CROSSFADE_LEN;
                                            let mut crossfade_buf =
                                                vec![vec![0.0; buffer_size as usize]; 2];
                                            #[cfg(feature = "lab")]
                                            let mut lab_pipeline =
                                                PipelineManager::from_categories(
                                                    &default_categories(),
                                                )
                                                .ok();

                                            device.build_input_stream(
                                                &strict_config,
                                                move |data: &[f32], _: &_| {
                                                    let snap = state_clone
                                                        .try_lock()
                                                        .map(|g| g.audio())
                                                        .unwrap_or_else(|_| {
                                                            AudioSnapshot::default()
                                                        });

                                                    // 1. Extrair canais da interface para nossos buffers
                                                    // Preenchemos buf_l e buf_r iterando sobre os frames do CPAL
                                                    let num_frames =
                                                        data.len() / (channels as usize);
                                                    let max_len =
                                                        num_frames.min(buffer_size as usize);

                                                    for (i, frame) in data
                                                        .chunks(channels as usize)
                                                        .enumerate()
                                                        .take(max_len)
                                                    {
                                                        buf_l[i] = frame
                                                            .get(l_idx)
                                                            .copied()
                                                            .unwrap_or(0.0);
                                                        buf_r[i] = frame
                                                            .get(r_idx)
                                                            .copied()
                                                            .unwrap_or(0.0);
                                                    }

                                                    // 2. Processamento DSP em Bloco
                                                    if !snap.bypass {
                                                        // A) Faust EQ (processa os dois canais num único array de ponteiros - se a API local_faust aguentar)
                                                        // A nossa interface Faust process_block aplica a um array plano in-place
                                                        if snap.eq_active {
                                                            if let Some(f) = &mut local_faust_l {
                                                                f.set_eq_params(
                                                                    snap.eq_low_freq,
                                                                    snap.eq_low_gain,
                                                                    snap.eq_low_q,
                                                                    snap.eq_mid_freq,
                                                                    snap.eq_mid_gain,
                                                                    snap.eq_mid_q,
                                                                    snap.eq_high_freq,
                                                                    snap.eq_high_gain,
                                                                    snap.eq_high_q,
                                                                );
                                                                f.set_eq_tanh_bypass(
                                                                    snap.eq_tanh_bypass,
                                                                );
                                                                f.process_block(
                                                                    buf_l.as_mut_ptr(),
                                                                    max_len,
                                                                );
                                                            }
                                                            if let Some(f) = &mut local_faust_r {
                                                                f.set_eq_params(
                                                                    snap.eq_low_freq,
                                                                    snap.eq_low_gain,
                                                                    snap.eq_low_q,
                                                                    snap.eq_mid_freq,
                                                                    snap.eq_mid_gain,
                                                                    snap.eq_mid_q,
                                                                    snap.eq_high_freq,
                                                                    snap.eq_high_gain,
                                                                    snap.eq_high_q,
                                                                );
                                                                f.set_eq_tanh_bypass(
                                                                    snap.eq_tanh_bypass,
                                                                );
                                                                f.process_block(
                                                                    buf_r.as_mut_ptr(),
                                                                    max_len,
                                                                );
                                                            }
                                                        }

                                                        // B) Wiener-Hammerstein Gray-Box
                                                        // ESTÁGIO 1: PRÉ-EQUALIZAÇÃO LTI (Tone Stack) // Somente se válido
                                                        // Precisamos proteger contra uninit calls de arrays zerados. Se array não for vazio, tem certeza q inicializou.
                                                        // Porém, falhas no convolver geram f32 zerados.
                                                        temp_l[..max_len]
                                                            .copy_from_slice(&buf_l[..max_len]);
                                                        temp_r[..max_len]
                                                            .copy_from_slice(&buf_r[..max_len]);
                                                        if pre_eq_l
                                                            .process(
                                                                &temp_l[..max_len],
                                                                &mut buf_l[..max_len],
                                                            )
                                                            .is_err()
                                                        {
                                                            buf_l[..max_len].copy_from_slice(
                                                                &temp_l[..max_len],
                                                            );
                                                        }
                                                        if pre_eq_r
                                                            .process(
                                                                &temp_r[..max_len],
                                                                &mut buf_r[..max_len],
                                                            )
                                                            .is_err()
                                                        {
                                                            buf_r[..max_len].copy_from_slice(
                                                                &temp_r[..max_len],
                                                            );
                                                        }

                                                        // ESTÁGIO 3: Modelo de amp selecionado
                                                        if snap.amp_model != previous_amp_model
                                                            && crossfade_sample >= CROSSFADE_LEN
                                                        {
                                                            crossfade_sample = 0;
                                                        } else if snap.amp_model
                                                            == previous_amp_model
                                                            && crossfade_sample < CROSSFADE_LEN
                                                        {
                                                            crossfade_sample = CROSSFADE_LEN;
                                                        }
                                                        let crossfade_start = crossfade_sample;
                                                        let crossfading = snap.amp_model
                                                            != previous_amp_model
                                                            && crossfade_start < CROSSFADE_LEN;

                                                        if crossfading {
                                                            temp_l[..max_len]
                                                                .copy_from_slice(&buf_l[..max_len]);
                                                            temp_r[..max_len]
                                                                .copy_from_slice(&buf_r[..max_len]);

                                                            process_standalone_amp(
                                                                previous_amp_model,
                                                                &snap,
                                                                &mut neural_amp_l,
                                                                &mut neural_amp_r,
                                                                &mut mlc_l,
                                                                &mut mlc_r,
                                                                &mut buf_l[..max_len],
                                                                &mut buf_r[..max_len],
                                                            );
                                                            crossfade_buf[0][..max_len]
                                                                .copy_from_slice(&buf_l[..max_len]);
                                                            crossfade_buf[1][..max_len]
                                                                .copy_from_slice(&buf_r[..max_len]);

                                                            buf_l[..max_len].copy_from_slice(
                                                                &temp_l[..max_len],
                                                            );
                                                            buf_r[..max_len].copy_from_slice(
                                                                &temp_r[..max_len],
                                                            );

                                                            process_standalone_amp(
                                                                snap.amp_model,
                                                                &snap,
                                                                &mut neural_amp_l,
                                                                &mut neural_amp_r,
                                                                &mut mlc_l,
                                                                &mut mlc_r,
                                                                &mut buf_l[..max_len],
                                                                &mut buf_r[..max_len],
                                                            );

                                                            for i in 0..max_len {
                                                                let fade_pos = (crossfade_start
                                                                    + i)
                                                                    .min(CROSSFADE_LEN);
                                                                let t = fade_pos as f32
                                                                    / CROSSFADE_LEN as f32;
                                                                let old_l = crossfade_buf[0][i];
                                                                let old_r = crossfade_buf[1][i];
                                                                buf_l[i] =
                                                                    old_l + (buf_l[i] - old_l) * t;
                                                                buf_r[i] =
                                                                    old_r + (buf_r[i] - old_r) * t;
                                                            }

                                                            crossfade_sample = (crossfade_start
                                                                + max_len)
                                                                .min(CROSSFADE_LEN);
                                                            if crossfade_sample >= CROSSFADE_LEN {
                                                                previous_amp_model = snap.amp_model;
                                                            }
                                                        } else {
                                                            process_standalone_amp(
                                                                snap.amp_model,
                                                                &snap,
                                                                &mut neural_amp_l,
                                                                &mut neural_amp_r,
                                                                &mut mlc_l,
                                                                &mut mlc_r,
                                                                &mut buf_l[..max_len],
                                                                &mut buf_r[..max_len],
                                                            );
                                                        }

                                                        // ESTÁGIO 4: GABINETE (Cabinet IR gerenciado — troca em runtime via ArcSwap)
                                                        cabinet_engine.process(
                                                            &mut buf_l[..max_len],
                                                            &mut buf_r[..max_len],
                                                            &mut temp_l[..max_len],
                                                            &mut temp_r[..max_len],
                                                            snap.cabinet_bypass,
                                                            snap.cabinet_level,
                                                            snap.cabinet_mix,
                                                        );

                                                        // ESTÁGIO 5: Ganho Master
                                                        for l in &mut buf_l[..max_len] {
                                                            *l *= snap.gain;
                                                        }
                                                        for r in &mut buf_r[..max_len] {
                                                            *r *= snap.gain;
                                                        }

                                                        // ESTÁGIO 6: Volume Master Neural (após processamento)
                                                        if snap.amp_model == AmpModel::Neural
                                                            && snap.neural_active
                                                        {
                                                            for l in &mut buf_l[..max_len] {
                                                                *l *= snap.neural_amp_volume;
                                                            }
                                                            for r in &mut buf_r[..max_len] {
                                                                *r *= snap.neural_amp_volume;
                                                            }
                                                        }

                                                        #[cfg(feature = "lab")]
                                                        if let Some(pipeline) =
                                                            lab_pipeline.as_mut()
                                                        {
                                                            pipeline.process_block(
                                                                buf_l.as_mut_ptr(),
                                                                max_len,
                                                            );
                                                            pipeline.process_block(
                                                                buf_r.as_mut_ptr(),
                                                                max_len,
                                                            );
                                                        }
                                                    }

                                                    // ESTÁGIO 7: Saneamento de NaN/Infinito (SEMPRE executado, mesmo em bypass)
                                                    for l in &mut buf_l[..max_len] {
                                                        if l.is_nan() || l.is_infinite() {
                                                            *l = 0.0;
                                                        }
                                                    }
                                                    for r in &mut buf_r[..max_len] {
                                                        if r.is_nan() || r.is_infinite() {
                                                            *r = 0.0;
                                                        }
                                                    }

                                                    // 3. Enviar para Output da Interface e Analisador Gráfico
                                                    for i in 0..max_len {
                                                        let mix = (buf_l[i] + buf_r[i]) * 0.5;
                                                        let _ = analyzer_producer.push(mix);
                                                        if let Some(pp) = pt_producer.as_mut() {
                                                            let _ = pp.push(buf_l[i]);
                                                            let _ = pp.push(buf_r[i]);
                                                        }
                                                    }
                                                },
                                                |err| eprintln!("Input error: {:?}", err),
                                                None,
                                            )
                                        }
                                        // Non-F32 input formats are rejected explicitly rather than
                                        // silently passed through DSP-free (which broke plugin/standalone
                                        // parity). The whole DSP chain — Faust, Neural and Cabinet — runs
                                        // in f32; select an F32-capable device/driver to enable it.
                                        _ => Err(cpal::BuildStreamError::StreamConfigNotSupported),
                                    };

                                    match stream_res {
                                        Ok(str) => {
                                            if let Err(e) = str.play() {
                                                err_msg = Some(format!(
                                                    "❌ Falha no Play da Entrada: {:?}",
                                                    e
                                                ));
                                            } else {
                                                _input_stream = Some(str);
                                            }
                                        }
                                        Err(e) => {
                                            err_msg = Some(match config.sample_format() {
                                                cpal::SampleFormat::F32 => format!("Erro buffer: {:?}", e),
                                                other => format!(
                                                    "Formato de entrada {:?} não suportado — a cadeia de DSP (Faust/Neural/Cabinet) requer amostras F32. Selecione outro dispositivo ou driver de áudio.",
                                                    other
                                                ),
                                            });
                                        }
                                    }
                                }
                            } else {
                                err_msg = Some("Input Físico não encontrado.".to_string());
                            }
                        }
                    }

                    if let Some((raw_name, left, right)) = output {
                        if let Ok(devs) = host.output_devices() {
                            if let Some(device) = devs
                                .into_iter()
                                .find(|d| d.name().unwrap_or_default() == raw_name)
                            {
                                if let Ok(config) = device.default_output_config() {
                                    let mut strict_config: cpal::StreamConfig =
                                        config.clone().into();
                                    strict_config.buffer_size = cpal::BufferSize::Default;
                                    let channels = strict_config.channels;
                                    let l_idx = left.min((channels.saturating_sub(1)) as usize);
                                    let r_idx = right.min((channels.saturating_sub(1)) as usize);

                                    let stream_res = match config.sample_format() {
                                        cpal::SampleFormat::F32 => device.build_output_stream(
                                            &strict_config,
                                            move |data: &mut [f32], _: &_| {
                                                for frame in data.chunks_mut(channels as usize) {
                                                    let (l_sample, r_sample) =
                                                        match pt_consumer.as_mut() {
                                                            Some(pc) => (
                                                                pc.pop().unwrap_or(0.0),
                                                                pc.pop().unwrap_or(0.0),
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
                                            },
                                            |err| eprintln!("Output error: {:?}", err),
                                            None,
                                        ),
                                        cpal::SampleFormat::I16 => device.build_output_stream(
                                            &strict_config,
                                            move |data: &mut [i16], _: &_| {
                                                for frame in data.chunks_mut(channels as usize) {
                                                    let l_sample = match pt_consumer.as_mut() {
                                                        Some(pc) => pc.pop().unwrap_or(0.0),
                                                        None => 0.0,
                                                    };
                                                    let pcm = (l_sample * i16::MAX as f32) as i16;
                                                    if let Some(l) = frame.get_mut(l_idx) {
                                                        *l = pcm;
                                                    }
                                                    if let Some(r) = frame.get_mut(r_idx) {
                                                        *r = pcm;
                                                    }
                                                }
                                            },
                                            |err| eprintln!("Output error: {:?}", err),
                                            None,
                                        ),
                                        _ => Err(cpal::BuildStreamError::StreamConfigNotSupported),
                                    };

                                    match stream_res {
                                        Ok(str) => {
                                            if let Err(e) = str.play() {
                                                err_msg = Some(format!(
                                                    "❌ Falha no Play da Saída: {:?}",
                                                    e
                                                ));
                                            } else {
                                                _output_stream = Some(str);
                                            }
                                        }
                                        Err(e) => {
                                            err_msg = Some(format!("Erro out: {:?}", e));
                                        }
                                    }
                                }
                            } else {
                                err_msg = Some("Output Físico não encontrado.".to_string());
                            }
                        }
                    }
                } else {
                    err_msg = Some("Falha ao comunicar com o Host do S.O.".to_string());
                }

                if let Some(err) = err_msg {
                    let _ = tx_event.send(AudioEvent::StreamStarted(Err(err)));
                } else {
                    let _ = tx_event.send(AudioEvent::StreamStarted(Ok(())));
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

        let cons_clone = cons_arc.clone();
        let state_clone = standalone_state.clone();
        let mailbox_clone = cabinet_mailbox.clone();
        let library_clone = cabinet_library.clone();
        let sr_clone = cabinet_sr.clone();
        let maxb_clone = cabinet_max_block.clone();
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

            sample_rate_warning: None,
            last_audio_error: None,
            last_audio_error_details: None,
            show_error_popup: false,

            input_is_mono: true,
            output_is_mono: false,

            show_settings: false,
            is_loading: true,
            active_panel: ActivePanel::None,
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

                    if !self.available_inputs.is_empty() && self.selected_input_idx.is_none() {
                        self.selected_input_idx = Some(0);
                    }
                    self.apply_audio_routing();
                }
                AudioEvent::StreamStarted(res) => {
                    self.is_loading = false;
                    if let Err(msg) = res {
                        eprintln!("[Audio Engine] ERRO Crítico do CPAL: {}", msg);
                        self.last_audio_error =
                            Some("⚠️ Limitação de Hardware: Buffer negado".to_string());
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

        let input_params = if let Some(idx) = self.selected_input_idx {
            if let Some(dev_ctx) = self.available_inputs.get(idx) {
                self.input_left = self
                    .input_left
                    .min((dev_ctx.channels.saturating_sub(1)) as usize);
                self.input_right = self
                    .input_right
                    .min((dev_ctx.channels.saturating_sub(1)) as usize);
                Some((dev_ctx.raw_name.clone(), self.input_left, self.input_right))
            } else {
                None
            }
        } else {
            None
        };

        let output_params = if let Some(idx) = self.selected_output_idx {
            if let Some(dev_ctx) = self.available_outputs.get(idx) {
                self.output_left = self
                    .output_left
                    .min((dev_ctx.channels.saturating_sub(1)) as usize);
                self.output_right = self
                    .output_right
                    .min((dev_ctx.channels.saturating_sub(1)) as usize);
                Some((
                    dev_ctx.raw_name.clone(),
                    self.output_left,
                    self.output_right,
                ))
            } else {
                None
            }
        } else {
            None
        };

        // Checar Mismatch de Sample Rate
        if let (Some(in_idx), Some(out_idx)) = (self.selected_input_idx, self.selected_output_idx) {
            if let (Some(in_dev), Some(out_dev)) = (
                self.available_inputs.get(in_idx),
                self.available_outputs.get(out_idx),
            ) {
                if in_dev.sample_rate != out_dev.sample_rate {
                    self.sample_rate_warning = Some(format!(
                        "⚠️ Taxas Incompatíveis: IN {}Hz vs OUT {}Hz",
                        in_dev.sample_rate, out_dev.sample_rate
                    ));
                }
            }
        }

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
                        });

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
                                    if ui.selectable_label(self.selected_input_idx == Some(idx), &dev.name).clicked() {
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
                                    if ui.selectable_label(self.selected_output_idx == Some(idx), &dev.name).clicked() {
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

                        ui.heading("⏱️ Latência Física (Buffer Size)");
                        ui.add_space(2.0);

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
                            0..=128 => "Guitarras / Baterias. Baixíssima latência.\n⚠️ Risco EXTREMO de estalos/breaks.",
                            129..=512 => "Produção e Gravação Dinâmica.\n✅ Relação Equilibrada.",
                            _ => "Mixagem e Masterização Isolada.\n🔥 Estabilidade máxima. Latência alta.",
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
            egui::Window::new("ℹ️ Sobre a Limitação do Buffer")
                .open(&mut open)
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    if let Some(details) = &self.last_audio_error_details {
                        ui.label("Algumas placas de som ou drivers recusam trabalhar em latências tão precisas. O aplicativo exige esse número e eles respondem: Ocupado/Inválido.\n");
                        ui.label("👉 Arraste o buffer para a direita até esse erro sumir. Se a barra chegou no limite, clique em 'Preciso de mais estabilidade'.\n\nErro exato das engrenagens do S.O:");

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
                ui.horizontal_wrapped(|ui| {
                    ui.group(|ui| {
                        ui.label(egui::RichText::new("Gain").strong());
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.label("Gain");
                                if ui
                                    .add(
                                        Knob::new(&mut ui_mlc_gain, 0.001, 1.0, KnobStyle::Wiper)
                                            .with_size(45.0),
                                    )
                                    .changed()
                                {
                                    changed = true;
                                }
                            });
                            ui.vertical(|ui| {
                                ui.label("Master");
                                if ui
                                    .add(
                                        Knob::new(&mut ui_mlc_master, 0.001, 1.0, KnobStyle::Wiper)
                                            .with_size(45.0),
                                    )
                                    .changed()
                                {
                                    changed = true;
                                }
                            });
                        });
                    });
                    ui.group(|ui| {
                        ui.label(egui::RichText::new("EQ").strong());
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.label("Bass");
                                if ui
                                    .add(
                                        Knob::new(&mut ui_mlc_bass, -12.0, 12.0, KnobStyle::Wiper)
                                            .with_size(45.0),
                                    )
                                    .changed()
                                {
                                    changed = true;
                                }
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
                            });
                        });
                    });
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
                            });
                            ui.vertical(|ui| {
                                ui.label("Depth");
                                if ui
                                    .add(
                                        Knob::new(&mut ui_mlc_depth, -12.0, 12.0, KnobStyle::Wiper)
                                            .with_size(45.0),
                                    )
                                    .changed()
                                {
                                    changed = true;
                                }
                            });
                        });
                    });
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
                                    ui.selectable_value(&mut gate_pos, MlcGatePos::Post, "Post");
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
                        });
                    });
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
        viewport: egui::ViewportBuilder::default().with_inner_size([1000.0, 500.0]),
        ..Default::default()
    };
    eframe::run_native(
        "BaseIO Standalone",
        options,
        Box::new(|cc| Ok(Box::new(StandaloneApp::new(cc)))),
    )
}
