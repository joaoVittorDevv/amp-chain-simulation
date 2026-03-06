use nih_plug::prelude::*;
use nih_plug_egui::{create_egui_editor, egui};
use rtrb::{Consumer, Producer, RingBuffer};
use std::sync::{Arc, Mutex};

pub mod core;
use crate::core::{dsp, ui};
use crate::core::ui::ActivePanel;

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
    preamp_l: dsp::PreampProcessor,
    preamp_r: dsp::PreampProcessor,
    cabinet_l: dsp::CabinetProcessor,
    cabinet_r: dsp::CabinetProcessor,
}

impl Default for BaseIO {
    fn default() -> Self {
        let (producer, consumer) = RingBuffer::new(1024 * 64);
        
        Self {
            params: Arc::new(BaseIOParams::default()),
            analyzer_consumer: Arc::new(Mutex::new(Some(consumer))),
            analyzer_producer: producer,
            preamp_l: dsp::PreampProcessor::new(44100.0),
            preamp_r: dsp::PreampProcessor::new(44100.0),
            cabinet_l: dsp::CabinetProcessor::new(44100.0),
            cabinet_r: dsp::CabinetProcessor::new(44100.0),
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
                // 1. Dizer ao Egui para desenhar continuamente (~60 fps)
                egui_ctx.request_repaint();

                // 2 & 3. Processar áudio e desenhar
                if let Ok(mut cons_lock) = state.consumer.try_lock() {
                    if let Some(cons) = cons_lock.as_mut() {
                        state.analyzer.process_consumer(cons);
                    }
                }

                // 4. Pintar a UI de Alta Performance
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

                        // Bypass Checkbox
                        let current_bypass = state.params.bypass.value();
                        let mut new_bypass = current_bypass;
                        if ui.checkbox(&mut new_bypass, "Bypass").changed() {
                            setter.begin_set_parameter(&state.params.bypass);
                            setter.set_parameter(&state.params.bypass, new_bypass);
                            setter.end_set_parameter(&state.params.bypass);
                        }
                    });
                });

                use crate::core::state::plugin_params::{CabinetDimension, PreampChannel, PreampDriveMode};
                use nih_plug::params::enums::Enum;

                ui::render_shared_panels(
                    egui_ctx,
                    &mut state.active_panel,
                    &state.analyzer.spectrum,
                    dsp::FFT_SIZE,
                    // --- Preamp closure ---
                    |ui| {
                        ui.label("Input Vol:");
                        ui.add(nih_plug_egui::widgets::ParamSlider::for_param(&state.params.preamp_input_vol, setter).with_width(80.0));
                        ui.separator();

                        ui.label("Channel:");
                        let cur_ch = state.params.preamp_channel.value();
                        let mut new_ch = cur_ch;
                        egui::ComboBox::from_id_salt("preamp_channel")
                            .selected_text(PreampChannel::variants()[cur_ch.to_index()])
                            .show_ui(ui, |ui| {
                                for (i, &name) in PreampChannel::variants().iter().enumerate() {
                                    ui.selectable_value(&mut new_ch, PreampChannel::from_index(i), name);
                                }
                            });
                        if new_ch != cur_ch {
                            setter.begin_set_parameter(&state.params.preamp_channel);
                            setter.set_parameter(&state.params.preamp_channel, new_ch);
                            setter.end_set_parameter(&state.params.preamp_channel);
                        }

                        if new_ch == PreampChannel::Dirty {
                            ui.separator();
                            ui.label("Mode:");
                            let cur_dm = state.params.preamp_drive_mode.value();
                            let mut new_dm = cur_dm;
                            egui::ComboBox::from_id_salt("preamp_drive_mode")
                                .selected_text(PreampDriveMode::variants()[cur_dm.to_index()])
                                .show_ui(ui, |ui| {
                                    for (i, &name) in PreampDriveMode::variants().iter().enumerate() {
                                        ui.selectable_value(&mut new_dm, PreampDriveMode::from_index(i), name);
                                    }
                                });
                            if new_dm != cur_dm {
                                setter.begin_set_parameter(&state.params.preamp_drive_mode);
                                setter.set_parameter(&state.params.preamp_drive_mode, new_dm);
                                setter.end_set_parameter(&state.params.preamp_drive_mode);
                            }
                        }

                        ui.separator();
                        ui.label("Drive:");
                        ui.add(nih_plug_egui::widgets::ParamSlider::for_param(&state.params.preamp_gain, setter).with_width(80.0));
                        ui.separator();
                        ui.label("Bass:");
                        ui.add(nih_plug_egui::widgets::ParamSlider::for_param(&state.params.preamp_bass, setter).with_width(80.0));
                        ui.label("Mid:");
                        ui.add(nih_plug_egui::widgets::ParamSlider::for_param(&state.params.preamp_mid, setter).with_width(80.0));
                        ui.label("Treble:");
                        ui.add(nih_plug_egui::widgets::ParamSlider::for_param(&state.params.preamp_treble, setter).with_width(80.0));
                        ui.separator();
                        ui.label("Master:");
                        ui.add(nih_plug_egui::widgets::ParamSlider::for_param(&state.params.preamp_master, setter).with_width(80.0));
                    },
                    // --- Cabinet closure ---
                    |ui| {
                        ui.label("Mic Position:");
                        ui.add(nih_plug_egui::widgets::ParamSlider::for_param(&state.params.mic_position, setter).with_width(80.0));
                        ui.label("Mic Distance:");
                        ui.add(nih_plug_egui::widgets::ParamSlider::for_param(&state.params.mic_distance, setter).with_width(80.0));

                        ui.label("Cabinet Size:");
                        let current_cab = state.params.cabinet_dimension.value();
                        let mut new_cab = current_cab;
                        egui::ComboBox::from_id_salt("cab_selector")
                            .selected_text(CabinetDimension::variants()[current_cab.to_index()])
                            .show_ui(ui, |ui| {
                                for (i, &name) in CabinetDimension::variants().iter().enumerate() {
                                    let variant = CabinetDimension::from_index(i);
                                    ui.selectable_value(&mut new_cab, variant, name);
                                }
                            });
                        if new_cab != current_cab {
                            setter.begin_set_parameter(&state.params.cabinet_dimension);
                            setter.set_parameter(&state.params.cabinet_dimension, new_cab);
                            setter.end_set_parameter(&state.params.cabinet_dimension);
                        }
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
        let max_sr = buffer_config.sample_rate;
        self.preamp_l.initialize(max_sr);
        self.preamp_r.initialize(max_sr);
        self.cabinet_l.initialize(max_sr);
        self.cabinet_r.initialize(max_sr);
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

        // Read smoothed preamp params once per block
        let input_vol    = self.params.preamp_input_vol.smoothed.next();
        let preamp_gain  = self.params.preamp_gain.smoothed.next();
        let bass_db      = self.params.preamp_bass.smoothed.next();
        let mid_db       = self.params.preamp_mid.smoothed.next();
        let treble_db    = self.params.preamp_treble.smoothed.next();
        let master_vol   = self.params.preamp_master.smoothed.next();
        let channel      = self.params.preamp_channel.value();
        let drive_mode   = self.params.preamp_drive_mode.value();

        // Cabinet params
        let mic_pos  = self.params.mic_position.smoothed.next();
        let mic_dist = self.params.mic_distance.smoothed.next();
        let cab_dim  = self.params.cabinet_dimension.value();

        // Block-level coefficient updates (not per-sample)
        self.preamp_l.update_params(bass_db, mid_db, treble_db);
        self.preamp_r.update_params(bass_db, mid_db, treble_db);
        self.cabinet_l.update_params(mic_pos, mic_dist, cab_dim);
        self.cabinet_r.update_params(mic_pos, mic_dist, cab_dim);

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

                // 1. Preamp (BEFORE cabinet)
                l_out = self.preamp_l.process(l_out, input_vol, preamp_gain, channel, drive_mode, master_vol);
                r_out = self.preamp_r.process(r_out, input_vol, preamp_gain, channel, drive_mode, master_vol);

                // 2. Cabinet (AFTER preamp)
                l_out = self.cabinet_l.process(l_out);
                r_out = self.cabinet_r.process(r_out);
            } else {
                l_out = l_in;
                r_out = r_in;
            }

            if l_out.is_nan() || l_out.is_infinite() { l_out = 0.0; }
            if r_out.is_nan() || r_out.is_infinite() { r_out = 0.0; }

            if let Some(l) = channel_samples.get_mut(0) { *l = l_out; }
            if let Some(r) = channel_samples.get_mut(1) { *r = r_out; }

            // Tap analyzer AFTER full chain
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
