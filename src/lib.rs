use nih_plug::prelude::*;
use nih_plug_egui::{create_egui_editor, egui};
use rtrb::{Consumer, Producer, RingBuffer};
use std::sync::{Arc, Mutex};
use fft_convolver::FFTConvolver;

pub mod core;
pub mod bridge;

use crate::core::{dsp, ui};
use crate::core::ui::ActivePanel;
use crate::bridge::{ExternalProcessor, faust::FaustProcessor, mojo::MojoProcessor};

// ─── DSP agora é conduzido por Mojo (Zero-Copy FFI) ─────────────────────────
// Não há mais inferência ONNX via tract-onnx nem thread de background.

// --- BASE CONFIG (TEMPLATE SCAFFOLDING) ---
pub const APP_NAME: &str = "Distortion";
pub const APP_ID: &str = "distortion";
pub const VENDOR: &str = "";
pub const APP_EMAIL: &str = "joaovittorh1@gmail.com";
pub const CLAP_ID: &str = ".distortion";
pub const VST3_ID: [u8; 16] = [0xA9, 0xED, 0x13, 0x81, 0x0D, 0xBC, 0x4A, 0x4A, 0x98, 0x54, 0x0F, 0x0F, 0x67, 0x1E, 0x9E, 0x1A];
// ------------------------------------------
use crate::core::state::plugin_params::{BaseIOParams, EditorState, InputSelect};

pub struct BaseIO {
    params: Arc<BaseIOParams>,
    analyzer_consumer: Arc<Mutex<Option<Consumer<f32>>>>,
    analyzer_producer: Producer<f32>,

    faust: [Option<FaustProcessor>; 2],
    /// Processador neural principal — substitui tract-onnx/ONNX.
    /// Executa saturação suave (tanh) in-place via FFI Zero-Copy.
    mojo: [Option<MojoProcessor>; 2],
    pre_eq_convolver: [Option<FFTConvolver<f32>>; 2],
    cabinet_convolver: [Option<FFTConvolver<f32>>; 2],
    temp_buffer: Vec<f32>,
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
            pre_eq_convolver: [None, None],
            cabinet_convolver: [None, None],
            temp_buffer: Vec::new(),
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
                                ui.selectable_value(&mut new_in, InputSelect::Stereo, "1/2 (Stereo)");
                                ui.selectable_value(&mut new_in, InputSelect::Input1, "Input 1 (Mic)");
                                ui.selectable_value(&mut new_in, InputSelect::Input2, "Input 2 (Guitar)");
                            });

                        if new_in != current_in {
                            setter.begin_set_parameter(&state.params.input_select);
                            setter.set_parameter(&state.params.input_select, new_in);
                            setter.end_set_parameter(&state.params.input_select);
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

use egui_knob::{Knob, KnobStyle};

                let make_knob = |ui: &mut egui::Ui, param: &FloatParam| {
                    let mut value = param.value();
                    let (min, max) = match param.range() {
                        FloatRange::Linear { min, max } => (min, max),
                        FloatRange::Skewed { min, max, .. } => (min, max),
                        _ => (0.0, 1.0),
                    };
                    
                    let response = ui.add(
                        Knob::new(
                            &mut value,
                            min,
                            max,
                            KnobStyle::Wiper,
                        )
                        .with_size(45.0),
                    );

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

                ui::render_shared_panels(
                    egui_ctx,
                    &mut state.active_panel,
                    &state.analyzer.spectrum,
                    dsp::FFT_SIZE,
                    eq_active,
                    n_active,
                    g_bypass,
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
                    |ui| {
                        ui.columns(3, |columns| {
                            crate::core::ui::main_view::draw_eq_band(&mut columns[0], "BASS",
                                |ui| { make_knob(ui, &state.params.eq_low_freq); },
                                |ui| { make_knob(ui, &state.params.eq_low_gain); },
                                |ui| { make_knob(ui, &state.params.eq_low_q); }
                            );
                            crate::core::ui::main_view::draw_eq_band(&mut columns[1], "MID",
                                |ui| { make_knob(ui, &state.params.eq_mid_freq); },
                                |ui| { make_knob(ui, &state.params.eq_mid_gain); },
                                |ui| { make_knob(ui, &state.params.eq_mid_q); }
                            );
                            crate::core::ui::main_view::draw_eq_band(&mut columns[2], "TREBLE",
                                |ui| { make_knob(ui, &state.params.eq_high_freq); },
                                |ui| { make_knob(ui, &state.params.eq_high_gain); },
                                |ui| { make_knob(ui, &state.params.eq_high_q); }
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
                );
            }
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
            if let Some(faust) = f { faust.init(sample_rate); }
        }
        for m in &mut self.mojo {
            if let Some(mojo) = m { mojo.init(sample_rate); }
        }

        let base_path = "/home/jao/VSCode/distortion/meu-novo-plugin/neural/drive/";

        // Helper genérico para carregar WAV f32 ou i16
        let load_ir = |p: &str| -> Option<Vec<f32>> {
            let mut reader = hound::WavReader::open(p).ok()?;
            let pcm = reader.spec().sample_format == hound::SampleFormat::Int;
            if pcm {
                Some(reader.samples::<i16>().filter_map(Result::ok).map(|s| s as f32 / i16::MAX as f32).collect())
            } else {
                Some(reader.samples::<f32>().filter_map(Result::ok).collect())
            }
        };

        // 1. Carregar o Pré-EQ
        let pre_eq_path = format!("{}{}", base_path, "pre_eq_ir.wav");
        if let Some(ir_samples) = load_ir(&pre_eq_path) {
            let mut conv_l = FFTConvolver::default();
            let mut conv_r = FFTConvolver::default();
            if !ir_samples.is_empty() && conv_l.init(buffer_config.max_buffer_size as usize, &ir_samples).is_ok() &&
               conv_r.init(buffer_config.max_buffer_size as usize, &ir_samples).is_ok() {
                self.pre_eq_convolver = [Some(conv_l), Some(conv_r)];
            }
        }

        // 2. Carregar a Caixa (Cabinet)
        let cab_path = format!("{}{}", base_path, "cabinet_ir.wav");
        if let Some(ir_samples) = load_ir(&cab_path) {
            let mut conv_l = FFTConvolver::default();
            let mut conv_r = FFTConvolver::default();
            if !ir_samples.is_empty() && conv_l.init(buffer_config.max_buffer_size as usize, &ir_samples).is_ok() &&
               conv_r.init(buffer_config.max_buffer_size as usize, &ir_samples).is_ok() {
                self.cabinet_convolver = [Some(conv_l), Some(conv_r)];
            }
        }

        self.temp_buffer.resize(buffer_config.max_buffer_size as usize, 0.0);

        true
    }

    fn reset(&mut self) {}

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let input_mode   = self.params.input_select.value();
        let bypass       = self.params.bypass.value();
        
        let neural_vol          = self.params.neural_amp_volume.smoothed.next();
        let neural_drive        = self.params.neural_drive.smoothed.next();
        let neural_output_gain  = self.params.neural_output_gain.smoothed.next();
        let neural_active       = self.params.neural_amp_active.value();

        // Mojo é síncrono e zero-copy — latência sempre 0 (sem PDC necessário)
        _context.set_latency_samples(0);

        // Apply mono / input routing FIRST before doing any DSP processing!
        for mut channel_samples in buffer.iter_samples() {
            let mut l_in = 0.0;

            if let Some(l) = channel_samples.get_mut(0) { l_in = *l; }
            // If there's a second channel, use it for Input2, otherwise default to l_in
            let r_source = channel_samples.get_mut(1).map_or(l_in, |r| *r);

            let (l_selected, r_selected) = match input_mode {
                InputSelect::Stereo => (l_in, r_source),
                InputSelect::Input1 => (l_in, l_in),
                InputSelect::Input2 => (r_source, r_source),
            };

            // Aplica a seleção de input na própria buffer
            if let Some(l) = channel_samples.get_mut(0) { *l = l_selected; }
            if let Some(r) = channel_samples.get_mut(1) { *r = r_selected; }
        }

        // Recuperar referências slice-based do buffer do nih-plug para FFI zero-copy!
        let buffer_slices = buffer.as_slice();

        // Processamento UNIFICADO (single-pass) seguindo o padrão do standalone
        for (channel_idx, channel_samples) in buffer_slices.iter_mut().enumerate() {
            let len = channel_samples.len();
            let safe_idx = channel_idx.min(1);

            if !bypass {
                // ESTÁGIO 1: Faust EQ (3-band parametric)
                if self.params.eq_active.value() {
                    if let Some(faust) = &mut self.faust[safe_idx] {
                        faust.set_eq_params(
                            self.params.eq_low_freq.smoothed.next(),
                            self.params.eq_low_gain.smoothed.next(),
                            self.params.eq_low_q.smoothed.next(),
                            self.params.eq_mid_freq.smoothed.next(),
                            self.params.eq_mid_gain.smoothed.next(),
                            self.params.eq_mid_q.smoothed.next(),
                            self.params.eq_high_freq.smoothed.next(),
                            self.params.eq_high_gain.smoothed.next(),
                            self.params.eq_high_q.smoothed.next(),
                        );
                        faust.process_block(channel_samples.as_mut_ptr(), len);
                    }
                }

                // ESTÁGIO 2: PRÉ-EQUALIZAÇÃO LTI (Pre-EQ Convolver)
                if let Some(pre_eq) = &mut self.pre_eq_convolver[safe_idx] {
                    self.temp_buffer[..len].copy_from_slice(&channel_samples[..len]);
                    if pre_eq.process(&self.temp_buffer[..len], &mut channel_samples[..len]).is_err() {
                        channel_samples[..len].copy_from_slice(&self.temp_buffer[..len]);
                    }
                }

                // ESTÁGIO 3: INFERÊNCIA NEURAL (Drive Zero-Copy via MojoProcessor)
                if neural_active {
                    if let Some(mojo) = &mut self.mojo[safe_idx] {
                        mojo.set_drive(neural_drive);
                        mojo.set_output_gain(neural_output_gain);
                        mojo.process_block(channel_samples.as_mut_ptr(), len);
                    }
                }

                // ESTÁGIO 4: GABINETE (Cabinet IR Convolver)
                if let Some(cabinet) = &mut self.cabinet_convolver[safe_idx] {
                    self.temp_buffer[..len].copy_from_slice(&channel_samples[..len]);
                    if cabinet.process(&self.temp_buffer[..len], &mut channel_samples[..len]).is_err() {
                        channel_samples[..len].copy_from_slice(&self.temp_buffer[..len]);
                    }
                }

                // ESTÁGIO 5: Ganho Master
                let gain = self.params.gain.smoothed.next();
                for sample in channel_samples.iter_mut() {
                    *sample *= gain;
                }

                // ESTÁGIO 6: Volume Master Neural (após processamento)
                if neural_active {
                    for sample in channel_samples.iter_mut() {
                        *sample *= neural_vol;
                    }
                }
            }

            // ESTÁGIO 7: Saneamento de NaN/Infinito (SEMPRE executado, mesmo em bypass)
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
            if let Some(l) = channel_samples.get_mut(0) { l_final = *l; }
            // If there's a second channel, use it for Input2, otherwise default to l_in
            let visual_sample = if let Some(r) = channel_samples.get_mut(1) { (l_final + *r) * 0.5 } else { l_final };
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
