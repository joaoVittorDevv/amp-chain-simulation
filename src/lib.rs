use nih_plug::prelude::*;
use nih_plug_egui::{create_egui_editor, egui};
use rtrb::{Consumer, Producer, RingBuffer};
use std::sync::{Arc, Mutex};

pub mod core;
pub mod bridge;

use crate::core::{dsp, ui};
use crate::core::ui::ActivePanel;
use crate::bridge::{ExternalProcessor, faust::FaustProcessor, mojo::MojoProcessor};

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
    neural_amp: dsp::NeuralAmpProcessor,

    faust: Option<FaustProcessor>,
    mojo: Option<MojoProcessor>,
}

impl Default for BaseIO {
    fn default() -> Self {
        let (producer, consumer) = RingBuffer::new(1024 * 64);
        
        Self {
            params: Arc::new(BaseIOParams::default()),
            analyzer_consumer: Arc::new(Mutex::new(Some(consumer))),
            analyzer_producer: producer,
            neural_amp: dsp::NeuralAmpProcessor::new("src/models/modelo_amp.onnx", 4096),
            faust: FaustProcessor::new(),
            mojo: Some(MojoProcessor::new()),
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
                let g_bypass = state.params.bypass.value();

                ui::render_shared_panels(
                    egui_ctx,
                    &mut state.active_panel,
                    &state.analyzer.spectrum,
                    dsp::FFT_SIZE,
                    n_active,
                    g_bypass,
                    || {
                        setter.begin_set_parameter(&state.params.neural_amp_active);
                        setter.set_parameter(&state.params.neural_amp_active, !n_active);
                        setter.end_set_parameter(&state.params.neural_amp_active);
                    },
                    |ui| {
                        ui.label("Neural Drive:");
                        ui.add(nih_plug_egui::widgets::ParamSlider::for_param(&state.params.neural_drive, setter).with_width(120.0));
                        
                        ui.label("Neural Makeup:");
                        ui.add(nih_plug_egui::widgets::ParamSlider::for_param(&state.params.neural_output_gain, setter).with_width(120.0));

                        ui.label("Master Output:");
                        ui.add(nih_plug_egui::widgets::ParamSlider::for_param(&state.params.neural_amp_volume, setter).with_width(120.0));
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
        if let Some(faust) = &mut self.faust {
            faust.init(sample_rate);
        }
        if let Some(mojo) = &mut self.mojo {
            mojo.init(sample_rate);
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
        let input_mode   = self.params.input_select.value();
        let bypass       = self.params.bypass.value();
        
        let neural_vol          = self.params.neural_amp_volume.smoothed.next();
        let neural_drive        = self.params.neural_drive.smoothed.next();
        let neural_output_gain  = self.params.neural_output_gain.smoothed.next();
        let neural_active       = self.params.neural_amp_active.value();
        
        if neural_active && self.neural_amp.is_ready() {
            _context.set_latency_samples(self.neural_amp.latency());
        } else {
            _context.set_latency_samples(0);
        }

        // Recuperar referências slice-based do buffer do nih-plug para FFI zero-copy!
        let buffer_slices = buffer.as_slice();
        let num_samples = buffer_slices[0].len();
        
        // 1. Processar com a abstração Faust FFI O(1) in-place
        if let Some(faust) = &mut self.faust {
            // Se o Faust for mono/stereo na interface abstrata, passamos os canais.
            // Para simplicidade na trait atual de "1 canal", processamos cada canal L, R.
            for channel in buffer_slices.iter_mut() {
                faust.process_block(channel.as_mut_ptr(), num_samples);
            }
        }

        // 2. Processar com a abstração Mojo FFI O(1) in-place
        if let Some(mojo) = &mut self.mojo {
            for channel in buffer_slices.iter_mut() {
                mojo.process_block(channel.as_mut_ptr(), num_samples);
            }
        }

        for mut channel_samples in buffer.iter_samples() {
            let gain = self.params.gain.smoothed.next();

            let mut l_in = *channel_samples.get_mut(0).unwrap_or(&mut 0.0);
            let r_in = *channel_samples.get_mut(1).unwrap_or(&mut l_in);

            let (mut l_out, mut r_out) = match input_mode {
                InputSelect::Stereo => (l_in, r_in),
                InputSelect::Input1 => (l_in, l_in),
                InputSelect::Input2 => (r_in, r_in),
            };

            if !bypass {
                l_out *= gain;
                r_out *= gain;

                if neural_active && self.neural_amp.is_ready() {
                    let mono_in = (l_out + r_out) * 0.5;

                    let peak_a = (mono_in * neural_drive).abs();
                    
                    self.neural_amp.push(mono_in * neural_drive);

                    let (n_out, is_fallback) = if let Some(processed) = self.neural_amp.pop() {
                        (processed * neural_output_gain, false)
                    } else {
                        (0.0, true) 
                    };

                    let peak_b = n_out.abs();

                    thread_local! {
                        static SAMPLE_COUNT: std::cell::Cell<u64> = std::cell::Cell::new(0);
                    }
                    SAMPLE_COUNT.with(|count| {
                        let c = count.get();
                        if c % 22050 == 0 {
                            println!("[TELEMETRIA] Ponto A (Drive): {:.5} | Ponto B (Output): {:.5} | Fallback: {}", peak_a, peak_b, is_fallback);
                        }
                        count.set(c + 1);
                    });

                    l_out = n_out * neural_vol;
                    r_out = n_out * neural_vol;
                }

            } else {
                l_out = l_in;
                r_out = r_in;
            }

            if l_out.is_nan() || l_out.is_infinite() { l_out = 0.0; }
            if r_out.is_nan() || r_out.is_infinite() { r_out = 0.0; }

            if let Some(l) = channel_samples.get_mut(0) { *l = l_out; }
            if let Some(r) = channel_samples.get_mut(1) { *r = r_out; }

            let visual_sample = (l_out + r_out) * 0.5;
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
