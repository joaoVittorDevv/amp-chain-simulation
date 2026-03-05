use nih_plug::prelude::*;
use nih_plug_egui::{create_egui_editor, egui, EguiState};
use rtrb::{Consumer, Producer, RingBuffer};
use std::sync::{Arc, Mutex};

pub mod core;
use crate::core::{dsp, ui};

// --- BASE CONFIG (TEMPLATE SCAFFOLDING) ---
pub const APP_NAME: &str = "Distortion";
pub const APP_ID: &str = "distortion";
pub const VENDOR: &str = "";
pub const APP_EMAIL: &str = "joaovittorh1@gmail.com";
pub const CLAP_ID: &str = ".distortion";
pub const VST3_ID: [u8; 16] = [0xA9, 0xED, 0x13, 0x81, 0x0D, 0xBC, 0x4A, 0x4A, 0x98, 0x54, 0x0F, 0x0F, 0x67, 0x1E, 0x9E, 0x1A]; 
// ------------------------------------------

#[derive(Enum, PartialEq, Clone, Copy, Debug)]
pub enum InputSelect {
    #[name = "1/2 (Stereo)"]
    Stereo,
    #[name = "Input 1 (Mic)"]
    Input1,
    #[name = "Input 2 (Guitar)"]
    Input2,
}

pub struct BaseIO {
    params: Arc<BaseIOParams>,
    analyzer_consumer: Arc<Mutex<Option<Consumer<f32>>>>,
    analyzer_producer: Producer<f32>,
}

#[derive(Params)]
struct BaseIOParams {
    #[persist = "editor-state"]
    editor_state: Arc<EguiState>,

    #[id = "input"]
    pub input_select: EnumParam<InputSelect>,

    #[id = "gain"]
    pub gain: FloatParam,
}

// O estado interno para a GUI interagir
struct EditorState {
    params: Arc<BaseIOParams>,
    analyzer: dsp::AnalyzerDsp,
    consumer: Arc<Mutex<Option<Consumer<f32>>>>,
}

impl Default for BaseIO {
    fn default() -> Self {
        let (producer, consumer) = RingBuffer::new(1024 * 64);
        
        Self {
            params: Arc::new(BaseIOParams::default()),
            analyzer_consumer: Arc::new(Mutex::new(Some(consumer))),
            analyzer_producer: producer,
        }
    }
}

impl Default for BaseIOParams {
    fn default() -> Self {
        Self {
            editor_state: EguiState::from_size(800, 450),

            input_select: EnumParam::new("Input Source", InputSelect::Stereo),

            gain: FloatParam::new(
                "Gain",
                util::db_to_gain(0.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-30.0),
                    max: util::db_to_gain(30.0),
                    factor: FloatRange::gain_skew_factor(-30.0, 30.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
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
            },
            |_, _| {},
            move |egui_ctx, setter, state| {
                // 1. Dizer ao Egui para desenhar continuamente (~60 fps)
                egui_ctx.request_repaint();

                // 2 & 3. Processar áudio e desenhar
                if let Ok(mut cons_lock) = state.consumer.try_lock() {
                    if let Some(cons) = cons_lock.as_mut() {
                        state.analyzer.process_consumer(cons);
                    }
                }

                // 4. Pintar a UI de Alta Performance
                egui::CentralPanel::default().show(egui_ctx, |ui| {
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
                    });
                    ui.separator();
                    
                    ui::draw_spectrum(ui, &state.analyzer.spectrum, dsp::FFT_SIZE);
                });
            }
        )
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
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
        
        for mut channel_samples in buffer.iter_samples() {
            let gain = self.params.gain.smoothed.next();

            // Pega os samples originais. Se o canal 2 não existir, assume o canal 1 por segurança
            let mut l_in = *channel_samples.get_mut(0).unwrap_or(&mut 0.0);
            let r_in = *channel_samples.get_mut(1).unwrap_or(&mut l_in);
            
            let (mut l_out, mut r_out) = match input_mode {
                InputSelect::Stereo => (l_in, r_in),
                InputSelect::Input1 => (l_in, l_in),
                InputSelect::Input2 => (r_in, r_in),
            };
            
            l_out *= gain;
            r_out *= gain;
            
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
