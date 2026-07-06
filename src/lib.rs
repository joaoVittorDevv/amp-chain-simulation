use fft_convolver::FFTConvolver;
use nih_plug::prelude::*;
use nih_plug_egui::{create_egui_editor, egui};
use rtrb::{Consumer, Producer, RingBuffer};
use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

pub mod bridge;
pub mod core;

use crate::bridge::{
    faust::FaustProcessor, mlc_zero_v::MlcZeroVProcessor, mojo::MojoProcessor, ExternalProcessor,
};
use crate::core::cabinet::{CabinetEngine, CabinetLibrary, CabinetRuntime};
use crate::core::ui::ActivePanel;
use crate::core::{dsp, ui};

/// Default cabinet IR embedded in the binary, used to seed an empty library on
/// first run. Replaces the old hardcoded absolute path.
const DEFAULT_CABINET_IR: &[u8] = include_bytes!("../neural/drive/cabinet_ir.wav");

/// Pre-EQ (tone-stack) impulse response embedded in the binary. Replaces the old
/// hardcoded absolute path so the linear pre-EQ stage no longer depends on a file.
const DEFAULT_PRE_EQ_IR: &[u8] = include_bytes!("../neural/drive/pre_eq_ir.wav");

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

/// Open the shared cabinet IR library at `dirs::data_dir()/distortion`, falling
/// back to an in-memory DB if the on-disk location is unavailable. Seeds the
/// embedded default IR when the library is empty.
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

// ─── DSP agora é conduzido por Mojo (Zero-Copy FFI) ─────────────────────────
// Não há mais inferência neural em background.

// --- BASE CONFIG (TEMPLATE SCAFFOLDING) ---
pub const APP_NAME: &str = "Distortion";
pub const APP_ID: &str = "distortion";
pub const VENDOR: &str = "";
pub const APP_EMAIL: &str = "joaovittorh1@gmail.com";
pub const CLAP_ID: &str = ".distortion";
pub const VST3_ID: [u8; 16] = [
    0xA9, 0xED, 0x13, 0x81, 0x0D, 0xBC, 0x4A, 0x4A, 0x98, 0x54, 0x0F, 0x0F, 0x67, 0x1E, 0x9E, 0x1A,
];
// ------------------------------------------
use crate::core::state::plugin_params::{
    AmpModel, BaseIOParams, EditorState, InputSelect, MlcBright, MlcFeedback, MlcGatePos,
};

const MAX_BLOCK: usize = 8192;
const CROSSFADE_LEN: usize = 480;

#[derive(Clone, Copy)]
struct MlcBlockParams {
    gain: f32,
    master: f32,
    bass: f32,
    middle: f32,
    treble: f32,
    presence: f32,
    depth: f32,
    gate: f32,
    bright: f32,
    m45: bool,
    warclaw: bool,
    feedback: f32,
    gate_pos: f32,
}

#[inline(always)]
fn configure_mlc(mlc: &mut MlcZeroVProcessor, params: MlcBlockParams) {
    mlc.set_gain(params.gain);
    mlc.set_master(params.master);
    mlc.set_bass(params.bass);
    mlc.set_middle(params.middle);
    mlc.set_treble(params.treble);
    mlc.set_presence(params.presence);
    mlc.set_depth(params.depth);
    mlc.set_gate(params.gate);
    mlc.set_bright(params.bright);
    mlc.set_m45(params.m45);
    mlc.set_warclaw(params.warclaw);
    mlc.set_feedback(params.feedback);
    mlc.set_gate_pos(params.gate_pos);
}

pub struct BaseIO {
    params: Arc<BaseIOParams>,
    analyzer_consumer: Arc<Mutex<Option<Consumer<f32>>>>,
    analyzer_producer: Producer<f32>,

    faust: [Option<FaustProcessor>; 2],
    /// Processador neural principal.
    /// Executa saturação suave (tanh) in-place via FFI Zero-Copy.
    mojo: [Option<MojoProcessor>; 2],
    mlc_zero_v: [Option<MlcZeroVProcessor>; 2],
    previous_amp_model: AmpModel,
    crossfade_sample: usize,
    crossfade_buf: [[f32; MAX_BLOCK]; 2],
    pre_eq_convolver: [Option<FFTConvolver<f32>>; 2],
    temp_buffer: Vec<f32>,

    // --- Cabinet IR ---
    cabinet_engine: CabinetEngine,
    cabinet_library: Arc<Mutex<CabinetLibrary>>,
    cabinet_scratch_l: Vec<f32>,
    cabinet_scratch_r: Vec<f32>,
    /// Shared with the editor so runtimes are built with the live engine rate.
    cabinet_sr: Arc<AtomicU32>,
    cabinet_max_block: Arc<AtomicUsize>,
    cabinet_error: Arc<Mutex<Option<String>>>,
}

impl Default for BaseIO {
    fn default() -> Self {
        let (producer, consumer) = RingBuffer::new(1024 * 64);

        Self {
            params: Arc::new(BaseIOParams::default()),
            analyzer_consumer: Arc::new(Mutex::new(Some(consumer))),
            analyzer_producer: producer,
            faust: [FaustProcessor::new(), FaustProcessor::new()],
            mojo: [Some(MojoProcessor::new()), Some(MojoProcessor::new())],
            mlc_zero_v: [MlcZeroVProcessor::new(), MlcZeroVProcessor::new()],
            previous_amp_model: AmpModel::Neural,
            crossfade_sample: CROSSFADE_LEN,
            crossfade_buf: [[0.0; MAX_BLOCK]; 2],
            pre_eq_convolver: [None, None],
            temp_buffer: Vec::new(),

            cabinet_engine: CabinetEngine::new(),
            cabinet_library: Arc::new(Mutex::new(open_cabinet_library())),
            cabinet_scratch_l: Vec::new(),
            cabinet_scratch_r: Vec::new(),
            cabinet_sr: Arc::new(AtomicU32::new(48_000)),
            cabinet_max_block: Arc::new(AtomicUsize::new(512)),
            cabinet_error: Arc::new(Mutex::new(None)),
        }
    }
}

impl Plugin for BaseIO {
    const NAME: &'static str = APP_NAME;
    const VENDOR: &'static str = VENDOR;
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = APP_EMAIL;
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),
        aux_input_ports: &[],
        aux_output_ports: &[],
        names: PortNames::const_default(),
    }];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        let consumer_mutex = self.analyzer_consumer.clone();

        create_egui_editor(
            self.params.editor_state.clone(),
            EditorState {
                params: self.params.clone(),
                analyzer: dsp::AnalyzerDsp::default(),
                consumer: consumer_mutex,
                active_panel: ActivePanel::None,
                cabinet_library: self.cabinet_library.clone(),
                cabinet_mailbox: self.cabinet_engine.mailbox(),
                cabinet_sr: self.cabinet_sr.clone(),
                cabinet_max_block: self.cabinet_max_block.clone(),
                cabinet_error: self.cabinet_error.clone(),
            },
            |_, _| {},
            move |egui_ctx, setter, state| {
                egui_ctx.request_repaint();

                if let Ok(mut cons_lock) = state.consumer.try_lock() {
                    if let Some(cons) = cons_lock.as_mut() {
                        state.analyzer.process_consumer(cons);
                    }
                }

                egui::TopBottomPanel::top("header_panel").show(egui_ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(format!("{} Spectrum Analyzer", APP_NAME));
                        ui.separator();
                        ui.label("Input Source:");

                        let current_in = state.params.input_select.value();
                        let mut new_in = current_in;

                        egui::ComboBox::from_id_salt("input_selector")
                            .selected_text(match current_in {
                                InputSelect::Stereo => "1/2 (Stereo)",
                                InputSelect::Input1 => "Input 1 (Mic)",
                                InputSelect::Input2 => "Input 2 (Guitar)",
                            })
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut new_in,
                                    InputSelect::Stereo,
                                    "1/2 (Stereo)",
                                );
                                ui.selectable_value(
                                    &mut new_in,
                                    InputSelect::Input1,
                                    "Input 1 (Mic)",
                                );
                                ui.selectable_value(
                                    &mut new_in,
                                    InputSelect::Input2,
                                    "Input 2 (Guitar)",
                                );
                            });

                        if new_in != current_in {
                            setter.begin_set_parameter(&state.params.input_select);
                            setter.set_parameter(&state.params.input_select, new_in);
                            setter.end_set_parameter(&state.params.input_select);
                        }

                        ui.separator();
                        ui.label("Amp:");

                        let current_amp = state.params.amp_model.value();
                        let mut new_amp = current_amp;

                        egui::ComboBox::from_id_salt("amp_model_selector")
                            .selected_text(match current_amp {
                                AmpModel::Neural => "Neural",
                                AmpModel::MlcZeroV => "MLC ZERO V",
                            })
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut new_amp, AmpModel::Neural, "Neural");
                                ui.selectable_value(&mut new_amp, AmpModel::MlcZeroV, "MLC ZERO V");
                            });

                        if new_amp != current_amp {
                            setter.begin_set_parameter(&state.params.amp_model);
                            setter.set_parameter(&state.params.amp_model, new_amp);
                            setter.end_set_parameter(&state.params.amp_model);
                            state.active_panel = match new_amp {
                                AmpModel::Neural => ActivePanel::NeuralAmp,
                                AmpModel::MlcZeroV => ActivePanel::MlcZeroV,
                            };
                        }

                        ui.separator();

                        let current_bypass = state.params.bypass.value();
                        let mut new_bypass = current_bypass;
                        if ui.checkbox(&mut new_bypass, "Bypass").changed() {
                            setter.begin_set_parameter(&state.params.bypass);
                            setter.set_parameter(&state.params.bypass, new_bypass);
                            setter.end_set_parameter(&state.params.bypass);
                        }
                    });
                });

                let n_active = state.params.neural_amp_active.value();
                let eq_active = state.params.eq_active.value();
                let g_bypass = state.params.bypass.value();
                let amp_model = state.params.amp_model.value();

                use egui_knob::{Knob, KnobStyle};

                let make_knob = |ui: &mut egui::Ui, param: &FloatParam| {
                    let mut value = param.value();
                    let (min, max) = match param.range() {
                        FloatRange::Linear { min, max } => (min, max),
                        FloatRange::Skewed { min, max, .. } => (min, max),
                        _ => (0.0, 1.0),
                    };

                    let response =
                        ui.add(Knob::new(&mut value, min, max, KnobStyle::Wiper).with_size(45.0));

                    if response.drag_started() {
                        setter.begin_set_parameter(param);
                    }
                    if response.changed() {
                        setter.set_parameter(param, value);
                    }
                    if response.drag_stopped() {
                        setter.end_set_parameter(param);
                    }
                    response
                };

                // Drop any parked (old) cabinet runtime on this UI thread.
                state.cabinet_mailbox.collect_garbage();

                let cab_bypass_now = state.params.cabinet_bypass.value();
                let cab_active = !cab_bypass_now;
                let cab_lib = state.cabinet_library.clone();
                let cab_mailbox = state.cabinet_mailbox.clone();
                let cab_sr = state.cabinet_sr.clone();
                let cab_maxb = state.cabinet_max_block.clone();
                let cab_err = state.cabinet_error.clone();
                let cab_ir_list = cab_lib
                    .lock()
                    .ok()
                    .map(|l| l.list_irs().unwrap_or_default())
                    .unwrap_or_default();
                let cab_active_hash = state
                    .params
                    .cab_active_hash
                    .read()
                    .map(|g| g.clone())
                    .unwrap_or_default();
                let cab_err_now = cab_err.lock().ok().and_then(|e| e.clone());

                ui::render_shared_panels(
                    egui_ctx,
                    &mut state.active_panel,
                    &state.analyzer.spectrum,
                    dsp::FFT_SIZE,
                    eq_active,
                    n_active,
                    cab_active,
                    g_bypass,
                    amp_model,
                    || {
                        setter.begin_set_parameter(&state.params.eq_active);
                        setter.set_parameter(&state.params.eq_active, !eq_active);
                        setter.end_set_parameter(&state.params.eq_active);
                    },
                    || {
                        setter.begin_set_parameter(&state.params.neural_amp_active);
                        setter.set_parameter(&state.params.neural_amp_active, !n_active);
                        setter.end_set_parameter(&state.params.neural_amp_active);
                    },
                    || {
                        setter.begin_set_parameter(&state.params.cabinet_bypass);
                        setter.set_parameter(&state.params.cabinet_bypass, !cab_bypass_now);
                        setter.end_set_parameter(&state.params.cabinet_bypass);
                    },
                    || {
                        setter.begin_set_parameter(&state.params.amp_model);
                        setter.set_parameter(&state.params.amp_model, AmpModel::Neural);
                        setter.end_set_parameter(&state.params.amp_model);
                    },
                    || {
                        setter.begin_set_parameter(&state.params.amp_model);
                        setter.set_parameter(&state.params.amp_model, AmpModel::MlcZeroV);
                        setter.end_set_parameter(&state.params.amp_model);
                    },
                    |ui| {
                        let mut tanh_bypass = state.params.eq_tanh_bypass.value();
                        if ui.checkbox(&mut tanh_bypass, "EQ Tanh Bypass").changed() {
                            setter.begin_set_parameter(&state.params.eq_tanh_bypass);
                            setter.set_parameter(&state.params.eq_tanh_bypass, tanh_bypass);
                            setter.end_set_parameter(&state.params.eq_tanh_bypass);
                        }
                        ui.add_space(6.0);
                        ui.columns(3, |columns| {
                            crate::core::ui::main_view::draw_eq_band(
                                &mut columns[0],
                                "BASS",
                                |ui| {
                                    make_knob(ui, &state.params.eq_low_freq);
                                },
                                |ui| {
                                    make_knob(ui, &state.params.eq_low_gain);
                                },
                                |ui| {
                                    make_knob(ui, &state.params.eq_low_q);
                                },
                            );
                            crate::core::ui::main_view::draw_eq_band(
                                &mut columns[1],
                                "MID",
                                |ui| {
                                    make_knob(ui, &state.params.eq_mid_freq);
                                },
                                |ui| {
                                    make_knob(ui, &state.params.eq_mid_gain);
                                },
                                |ui| {
                                    make_knob(ui, &state.params.eq_mid_q);
                                },
                            );
                            crate::core::ui::main_view::draw_eq_band(
                                &mut columns[2],
                                "TREBLE",
                                |ui| {
                                    make_knob(ui, &state.params.eq_high_freq);
                                },
                                |ui| {
                                    make_knob(ui, &state.params.eq_high_gain);
                                },
                                |ui| {
                                    make_knob(ui, &state.params.eq_high_q);
                                },
                            );
                        });
                    },
                    |ui| {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.label("Neural Drive:");
                                make_knob(ui, &state.params.neural_drive);
                            });
                            ui.add_space(10.0);
                            ui.vertical(|ui| {
                                ui.label("Neural Makeup:");
                                make_knob(ui, &state.params.neural_output_gain);
                            });
                            ui.add_space(10.0);
                            ui.vertical(|ui| {
                                ui.label("Master Output:");
                                make_knob(ui, &state.params.neural_amp_volume);
                            });
                        });
                    },
                    |ui| {
                        crate::core::ui::draw_mlc_zero_v_panel(ui, setter, &state.params);
                    },
                    |ui| {
                        // Build a runtime and, only on success, hand it to the audio
                        // thread. Returns whether the build succeeded so callers can
                        // gate persistence on it (B6: never persist a broken selection).
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

                        crate::core::ui::draw_cabinet_panel(
                            ui,
                            &cab_ir_list,
                            &cab_active_hash,
                            cab_err_now.as_deref(),
                            || state.params.cabinet_bypass.value(),
                            |v| {
                                setter.begin_set_parameter(&state.params.cabinet_bypass);
                                setter.set_parameter(&state.params.cabinet_bypass, v);
                                setter.end_set_parameter(&state.params.cabinet_bypass);
                            },
                            || state.params.cabinet_level.value(),
                            |v| {
                                setter.begin_set_parameter(&state.params.cabinet_level);
                                setter.set_parameter(&state.params.cabinet_level, v);
                                setter.end_set_parameter(&state.params.cabinet_level);
                            },
                            || state.params.cabinet_mix.value(),
                            |v| {
                                setter.begin_set_parameter(&state.params.cabinet_mix);
                                setter.set_parameter(&state.params.cabinet_mix, v);
                                setter.end_set_parameter(&state.params.cabinet_mix);
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
                                        if let Ok(mut g) = state.params.cab_active_hash.write() {
                                            *g = hash.clone();
                                        }
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
                                            let imported =
                                                cab_lib.lock().ok().and_then(|l| {
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
                                                    if let Ok(mut g) =
                                                        state.params.cab_active_hash.write()
                                                    {
                                                        *g = hash.clone();
                                                    }
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
                                if state
                                    .params
                                    .cab_active_hash
                                    .read()
                                    .map(|g| *g == hash)
                                    .unwrap_or(false)
                                {
                                    if let Ok(mut g) = state.params.cab_active_hash.write() {
                                        g.clear();
                                    }
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
            },
        )
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        let sample_rate = buffer_config.sample_rate;
        for f in &mut self.faust {
            if let Some(faust) = f {
                faust.init(sample_rate);
            }
        }
        for m in &mut self.mojo {
            if let Some(mojo) = m {
                mojo.init(sample_rate);
            }
        }
        for m in &mut self.mlc_zero_v {
            if let Some(mlc) = m {
                mlc.init(sample_rate);
            }
        }

        // 1. Pré-EQ (tone-stack) — IR fixo embedado no binário (sem path absoluto).
        if let Some(ir_samples) = decode_wav_flat(DEFAULT_PRE_EQ_IR) {
            let mut conv_l = FFTConvolver::default();
            let mut conv_r = FFTConvolver::default();
            if !ir_samples.is_empty()
                && conv_l
                    .init(buffer_config.max_buffer_size as usize, &ir_samples)
                    .is_ok()
                && conv_r
                    .init(buffer_config.max_buffer_size as usize, &ir_samples)
                    .is_ok()
            {
                self.pre_eq_convolver = [Some(conv_l), Some(conv_r)];
            }
        }

        // 2. Cabinet IR: managed via CabinetLibrary + CabinetEngine (no hardcoded path).
        let max_block = buffer_config.max_buffer_size as usize;
        self.temp_buffer.resize(max_block, 0.0);
        self.cabinet_scratch_l.resize(max_block, 0.0);
        self.cabinet_scratch_r.resize(max_block, 0.0);

        self.cabinet_engine.set_sample_rate(sample_rate);
        self.cabinet_sr.store(sample_rate as u32, Ordering::Relaxed);
        self.cabinet_max_block.store(max_block, Ordering::Relaxed);

        // Resolve the active IR: prefer the persisted per-project hash, otherwise
        // fall back to the library's stored selection (the seeded default IR).
        let mut active_hash = self
            .params
            .cab_active_hash
            .read()
            .map(|g| g.clone())
            .unwrap_or_default();
        if let Ok(lib) = self.cabinet_library.lock() {
            if active_hash.is_empty() {
                active_hash = lib.get_selected_hash().ok().flatten().unwrap_or_default();
                if let Ok(mut g) = self.params.cab_active_hash.write() {
                    *g = active_hash.clone();
                }
            }
            if !active_hash.is_empty() {
                let result = lib
                    .get_ir_by_hash(&active_hash)
                    .and_then(|(_, bytes)| CabinetRuntime::build(&bytes, sample_rate, max_block));
                match result {
                    Ok(rt) => self.cabinet_engine.load_runtime(rt),
                    Err(e) => {
                        // Surface the failure in the UI (N3) and the log.
                        nih_log!("Cabinet IR init failed: {}", e);
                        if let Ok(mut err) = self.cabinet_error.lock() {
                            *err = Some(format!("Falha ao carregar IR: {}", e));
                        }
                    }
                }
            }
        }

        true
    }

    fn reset(&mut self) {}

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let input_mode = self.params.input_select.value();
        let amp_model = self.params.amp_model.value();
        let bypass = self.params.bypass.value();

        let gain = self.params.gain.smoothed.next();
        let neural_vol = self.params.neural_amp_volume.smoothed.next();
        let neural_drive = self.params.neural_drive.smoothed.next();
        let neural_output_gain = self.params.neural_output_gain.smoothed.next();
        let neural_active = self.params.neural_amp_active.value();
        let mlc_params = MlcBlockParams {
            gain: self.params.mlc_gain.smoothed.next(),
            master: self.params.mlc_master.smoothed.next(),
            bass: self.params.mlc_bass.smoothed.next(),
            middle: self.params.mlc_middle.smoothed.next(),
            treble: self.params.mlc_treble.smoothed.next(),
            presence: self.params.mlc_presence.smoothed.next(),
            depth: self.params.mlc_depth.smoothed.next(),
            gate: self.params.mlc_gate.smoothed.next(),
            bright: match self.params.mlc_bright.value() {
                MlcBright::I => 0.0,
                MlcBright::Ii => 1.0,
            },
            m45: self.params.mlc_m45.value(),
            warclaw: self.params.mlc_warclaw.value(),
            feedback: match self.params.mlc_feedback.value() {
                MlcFeedback::Lo => 0.0,
                MlcFeedback::Hi => 1.0,
            },
            gate_pos: match self.params.mlc_gate_pos.value() {
                MlcGatePos::Pre => 0.0,
                MlcGatePos::Post => 1.0,
            },
        };
        let eq_active = self.params.eq_active.value();
        let eq_tanh_bypass = self.params.eq_tanh_bypass.value();
        let eq_low_freq = self.params.eq_low_freq.smoothed.next();
        let eq_low_gain = self.params.eq_low_gain.smoothed.next();
        let eq_low_q = self.params.eq_low_q.smoothed.next();
        let eq_mid_freq = self.params.eq_mid_freq.smoothed.next();
        let eq_mid_gain = self.params.eq_mid_gain.smoothed.next();
        let eq_mid_q = self.params.eq_mid_q.smoothed.next();
        let eq_high_freq = self.params.eq_high_freq.smoothed.next();
        let eq_high_gain = self.params.eq_high_gain.smoothed.next();
        let eq_high_q = self.params.eq_high_q.smoothed.next();
        let cab_bypass = self.params.cabinet_bypass.value();
        let cab_level = self.params.cabinet_level.smoothed.next();
        let cab_mix = self.params.cabinet_mix.smoothed.next();

        // Mojo é síncrono e zero-copy — latência sempre 0 (sem PDC necessário)
        _context.set_latency_samples(0);

        // Apply mono / input routing FIRST before doing any DSP processing!
        for mut channel_samples in buffer.iter_samples() {
            let mut l_in = 0.0;

            if let Some(l) = channel_samples.get_mut(0) {
                l_in = *l;
            }
            // If there's a second channel, use it for Input2, otherwise default to l_in
            let r_source = channel_samples.get_mut(1).map_or(l_in, |r| *r);

            let (l_selected, r_selected) = match input_mode {
                InputSelect::Stereo => (l_in, r_source),
                InputSelect::Input1 => (l_in, l_in),
                InputSelect::Input2 => (r_source, r_source),
            };

            // Aplica a seleção de input na própria buffer
            if let Some(l) = channel_samples.get_mut(0) {
                *l = l_selected;
            }
            if let Some(r) = channel_samples.get_mut(1) {
                *r = r_selected;
            }
        }

        // Recuperar referências slice-based do buffer do nih-plug para FFI zero-copy!
        let buffer_slices = buffer.as_slice();

        if !bypass {
            if amp_model == self.previous_amp_model && self.crossfade_sample < CROSSFADE_LEN {
                self.crossfade_sample = CROSSFADE_LEN;
            } else if amp_model != self.previous_amp_model && self.crossfade_sample >= CROSSFADE_LEN
            {
                self.crossfade_sample = 0;
            }
            let crossfade_start = self.crossfade_sample;
            let crossfading =
                amp_model != self.previous_amp_model && crossfade_start < CROSSFADE_LEN;

            // ESTÁGIOS 1-3 (per-channel): Faust EQ → Pré-EQ → Neural drive
            for (channel_idx, channel_samples) in buffer_slices.iter_mut().enumerate() {
                let len = channel_samples.len();
                let safe_idx = channel_idx.min(1);

                // ESTÁGIO 1: Faust EQ (3-band parametric)
                if eq_active {
                    if let Some(faust) = &mut self.faust[safe_idx] {
                        faust.set_eq_params(
                            eq_low_freq,
                            eq_low_gain,
                            eq_low_q,
                            eq_mid_freq,
                            eq_mid_gain,
                            eq_mid_q,
                            eq_high_freq,
                            eq_high_gain,
                            eq_high_q,
                        );
                        faust.set_eq_tanh_bypass(eq_tanh_bypass);
                        faust.process_block(channel_samples.as_mut_ptr(), len);
                    }
                }

                // ESTÁGIO 2: PRÉ-EQUALIZAÇÃO LTI (Pre-EQ Convolver)
                if let Some(pre_eq) = &mut self.pre_eq_convolver[safe_idx] {
                    self.temp_buffer[..len].copy_from_slice(&channel_samples[..len]);
                    if pre_eq
                        .process(&self.temp_buffer[..len], &mut channel_samples[..len])
                        .is_err()
                    {
                        channel_samples[..len].copy_from_slice(&self.temp_buffer[..len]);
                    }
                }

                if crossfading {
                    let fade_len = len.min(MAX_BLOCK);
                    self.temp_buffer[..len].copy_from_slice(&channel_samples[..len]);

                    match self.previous_amp_model {
                        AmpModel::Neural => {
                            if neural_active {
                                if let Some(mojo) = &mut self.mojo[safe_idx] {
                                    mojo.set_drive(neural_drive);
                                    mojo.set_output_gain(neural_output_gain);
                                    mojo.process_block(channel_samples.as_mut_ptr(), len);
                                }
                            }
                        }
                        AmpModel::MlcZeroV => {
                            if let Some(mlc) = &mut self.mlc_zero_v[safe_idx] {
                                configure_mlc(mlc, mlc_params);
                                mlc.process_block(channel_samples.as_mut_ptr(), len);
                            }
                        }
                    }

                    self.crossfade_buf[safe_idx][..fade_len]
                        .copy_from_slice(&channel_samples[..fade_len]);
                    channel_samples[..len].copy_from_slice(&self.temp_buffer[..len]);

                    match amp_model {
                        AmpModel::Neural => {
                            if neural_active {
                                if let Some(mojo) = &mut self.mojo[safe_idx] {
                                    mojo.set_drive(neural_drive);
                                    mojo.set_output_gain(neural_output_gain);
                                    mojo.process_block(channel_samples.as_mut_ptr(), len);
                                }
                            }
                        }
                        AmpModel::MlcZeroV => {
                            if let Some(mlc) = &mut self.mlc_zero_v[safe_idx] {
                                configure_mlc(mlc, mlc_params);
                                mlc.process_block(channel_samples.as_mut_ptr(), len);
                            }
                        }
                    }

                    for (i, sample) in channel_samples[..fade_len].iter_mut().enumerate() {
                        let fade_pos = (crossfade_start + i).min(CROSSFADE_LEN);
                        let t = fade_pos as f32 / CROSSFADE_LEN as f32;
                        let old = self.crossfade_buf[safe_idx][i];
                        *sample = old + (*sample - old) * t;
                    }
                } else {
                    match amp_model {
                        AmpModel::Neural => {
                            if neural_active {
                                if let Some(mojo) = &mut self.mojo[safe_idx] {
                                    mojo.set_drive(neural_drive);
                                    mojo.set_output_gain(neural_output_gain);
                                    mojo.process_block(channel_samples.as_mut_ptr(), len);
                                }
                            }
                        }
                        AmpModel::MlcZeroV => {
                            if let Some(mlc) = &mut self.mlc_zero_v[safe_idx] {
                                configure_mlc(mlc, mlc_params);
                                mlc.process_block(channel_samples.as_mut_ptr(), len);
                            }
                        }
                    }
                }
            }

            if crossfading {
                let len = buffer_slices.first().map_or(0, |c| c.len());
                self.crossfade_sample = (crossfade_start + len).min(CROSSFADE_LEN);
                if self.crossfade_sample >= CROSSFADE_LEN {
                    self.previous_amp_model = amp_model;
                }
            }

            // ESTÁGIO 4: GABINETE (Cabinet IR — troca em runtime via ArcSwap).
            // Processado em stereo para manter rampa/crossfade coerente entre L/R.
            let len = buffer_slices.first().map_or(0, |c| c.len());
            if len > 0 {
                if buffer_slices.len() >= 2 {
                    let (l_part, r_part) = buffer_slices.split_at_mut(1);
                    self.cabinet_engine.process(
                        l_part[0],
                        r_part[0],
                        &mut self.cabinet_scratch_l[..len],
                        &mut self.cabinet_scratch_r[..len],
                        cab_bypass,
                        cab_level,
                        cab_mix,
                    );
                } else {
                    // Mono host (never hit under the enforced stereo layout, but kept
                    // sound): feed the single channel as L and a throwaway mirror as R.
                    self.temp_buffer[..len].copy_from_slice(&buffer_slices[0][..len]);
                    self.cabinet_engine.process(
                        &mut buffer_slices[0][..len],
                        &mut self.temp_buffer[..len],
                        &mut self.cabinet_scratch_l[..len],
                        &mut self.cabinet_scratch_r[..len],
                        cab_bypass,
                        cab_level,
                        cab_mix,
                    );
                }
            }

            // ESTÁGIOS 5-6 (per-channel): Ganho Master → Volume Master Neural
            for channel_samples in buffer_slices.iter_mut() {
                for sample in channel_samples.iter_mut() {
                    *sample *= gain;
                }
                if amp_model == AmpModel::Neural && neural_active {
                    for sample in channel_samples.iter_mut() {
                        *sample *= neural_vol;
                    }
                }
            }
        }

        // ESTÁGIO 7: Saneamento de NaN/Infinito (SEMPRE executado, mesmo em bypass)
        for channel_samples in buffer_slices.iter_mut() {
            for sample in channel_samples.iter_mut() {
                if sample.is_nan() || sample.is_infinite() {
                    *sample = 0.0;
                }
            }
        }

        // Analyzer: amostra para visualização pós-processamento
        // e garante loop safe lendo as amostras resultantes
        for mut channel_samples in buffer.iter_samples() {
            let mut l_final = 0.0;
            if let Some(l) = channel_samples.get_mut(0) {
                l_final = *l;
            }
            // If there's a second channel, use it for Input2, otherwise default to l_in
            let visual_sample = if let Some(r) = channel_samples.get_mut(1) {
                (l_final + *r) * 0.5
            } else {
                l_final
            };
            let _ = self.analyzer_producer.push(visual_sample);
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for BaseIO {
    const CLAP_ID: &'static str = CLAP_ID;
    const CLAP_DESCRIPTION: Option<&'static str> = Some("BaseIO Audio Engine Template");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::AudioEffect, ClapFeature::Stereo];
}

impl Vst3Plugin for BaseIO {
    const VST3_CLASS_ID: [u8; 16] = VST3_ID;
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[Vst3SubCategory::Analyzer];
}

nih_export_clap!(BaseIO);
nih_export_vst3!(BaseIO);
