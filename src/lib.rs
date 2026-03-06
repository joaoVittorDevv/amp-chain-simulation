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
    custom_ir: Arc<arc_swap::ArcSwapOption<crate::core::dsp::cabinet::ir_convolver::IrData>>,
    cab_clipping_meter: Arc<nih_plug::params::smoothing::AtomicF32>,
}

impl Default for BaseIO {
    fn default() -> Self {
        let (producer, consumer) = RingBuffer::new(1024 * 64);
        
        let custom_ir = Arc::new(arc_swap::ArcSwapOption::const_empty());
        let cab_clipping_meter = Arc::new(nih_plug::params::smoothing::AtomicF32::new(0.0));
        
        Self {
            params: Arc::new(BaseIOParams::default()),
            analyzer_consumer: Arc::new(Mutex::new(Some(consumer))),
            analyzer_producer: producer,
            preamp_l: dsp::PreampProcessor::new(44100.0),
            preamp_r: dsp::PreampProcessor::new(44100.0),
            cabinet_l: dsp::CabinetProcessor::new(44100.0, custom_ir.clone(), cab_clipping_meter.clone()),
            cabinet_r: dsp::CabinetProcessor::new(44100.0, custom_ir.clone(), cab_clipping_meter.clone()),
            custom_ir,
            cab_clipping_meter,
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
                custom_ir: self.custom_ir.clone(),
                cab_clipping_meter: self.cab_clipping_meter.clone(),
                loaded_ir_name: "No IR loaded".to_string(),
                ir_load_error: None,
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
                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                let current_custom_ir = state.params.use_custom_ir.value();
                                let mut new_custom_ir = current_custom_ir;
                                if ui.checkbox(&mut new_custom_ir, "Use Custom IR").changed() {
                                    setter.begin_set_parameter(&state.params.use_custom_ir);
                                    setter.set_parameter(&state.params.use_custom_ir, new_custom_ir);
                                    setter.end_set_parameter(&state.params.use_custom_ir);
                                }

                                if new_custom_ir {
                                    if ui.button("📁 Load IR").clicked() {
                                        if let Some(path) = rfd::FileDialog::new().add_filter("wav", &["wav"]).pick_file() {
                                            match hound::WavReader::open(&path) {
                                                Ok(mut reader) => {
                                                    let spec = reader.spec();
                                                    if spec.sample_format == hound::SampleFormat::Int || spec.sample_format == hound::SampleFormat::Float {
                                                        let mut samples: Vec<f32> = match spec.sample_format {
                                                            hound::SampleFormat::Int => {
                                                                if spec.bits_per_sample == 16 {
                                                                    reader.samples::<i16>().filter_map(|s| s.ok()).map(|s| s as f32 / i16::MAX as f32).collect()
                                                                } else if spec.bits_per_sample == 24 {
                                                                    reader.samples::<i32>().filter_map(|s| s.ok()).map(|s| s as f32 / 8388607.0).collect()
                                                                } else {
                                                                    reader.samples::<i32>().filter_map(|s| s.ok()).map(|s| s as f32 / i32::MAX as f32).collect()
                                                                }
                                                            },
                                                            hound::SampleFormat::Float => {
                                                                reader.samples::<f32>().filter_map(|s| s.ok()).collect()
                                                            }
                                                        };
                                                        
                                                        let channels = spec.channels as usize;
                                                        if channels > 1 && !samples.is_empty() {
                                                            samples = samples.chunks(channels).map(|c| c[0]).collect();
                                                        }
                                                        
                                                        if !samples.is_empty() {
                                                            let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                                                            let ds = crate::core::dsp::cabinet::ir_convolver::IrData::new(&samples, name.clone());
                                                            state.custom_ir.store(Some(Arc::new(ds)));
                                                            state.loaded_ir_name = name;
                                                            state.ir_load_error = None;
                                                        }
                                                    } else {
                                                        state.ir_load_error = Some("Unsupported WAV format".to_string());
                                                    }
                                                },
                                                Err(e) => {
                                                    state.ir_load_error = Some(format!("Error: {}", e));
                                                }
                                            }
                                        }
                                    }
                                    
                                    ui.label(format!("Loaded: {}", state.loaded_ir_name));
                                    
                                    if let Some(err) = &state.ir_load_error {
                                        ui.colored_label(egui::Color32::RED, err);
                                    }
                                }
                            });

                            ui.separator();
                            
                            ui.horizontal(|ui| {
                                if !state.params.use_custom_ir.value() {
                                    ui.label("Mic Position:");
                                    ui.add(nih_plug_egui::widgets::ParamSlider::for_param(&state.params.mic_position, setter).with_width(80.0));
                                    ui.label("Mic Distance:");
                                    ui.add(nih_plug_egui::widgets::ParamSlider::for_param(&state.params.mic_distance, setter).with_width(80.0));

                                    ui.label("Size:");
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
                                } else {
                                    ui.label("Algorithmic cabinet bypassed.");
                                }
                                
                                ui.separator();
                                ui.label("Master Vol:");
                                ui.add(nih_plug_egui::widgets::ParamSlider::for_param(&state.params.cabinet_master_volume, setter).with_width(80.0));
                                
                                // Peak meter logic
                                let current_peak = state.cab_clipping_meter.load(std::sync::atomic::Ordering::Relaxed);
                                // Decay meter visually so it does not stay frozen. Approximately 0.9 per frame is a fast decay.
                                state.cab_clipping_meter.store(current_peak * 0.90, std::sync::atomic::Ordering::Relaxed);
                                let peak_db: f32 = if current_peak > 1e-4 { 20.0 * current_peak.log10() } else { -60.0 };
                                let fraction = (peak_db.max(-60.0_f32) / 60.0 + 1.0).clamp(0.0_f32, 1.0_f32);
                                let meter_width = 60.0;
                                let rect = ui.allocate_exact_size(egui::vec2(meter_width, 10.0), egui::Sense::hover()).0;
                                let filled_rect = egui::Rect::from_min_max(
                                    rect.min,
                                    egui::pos2(rect.min.x + (meter_width * fraction), rect.max.y)
                                );
                                let meter_color = if current_peak > 0.99 { egui::Color32::RED } else { egui::Color32::from_rgb(0, 200, 50) };
                                
                                ui.painter().rect_filled(rect, 0.0, egui::Color32::from_rgb(40, 40, 40));
                                ui.painter().rect_filled(filled_rect, 0.0, meter_color);
                                if current_peak > 0.99 {
                                    ui.colored_label(egui::Color32::RED, "CLIP");
                                }
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
        let use_custom_ir = self.params.use_custom_ir.value();
        let cab_master_vol = self.params.cabinet_master_volume.smoothed.next();

        // Block-level coefficient updates (not per-sample)
        self.preamp_l.update_params(bass_db, mid_db, treble_db);
        self.preamp_r.update_params(bass_db, mid_db, treble_db);
        self.cabinet_l.update_params(mic_pos, mic_dist, cab_dim, use_custom_ir, cab_master_vol);
        self.cabinet_r.update_params(mic_pos, mic_dist, cab_dim, use_custom_ir, cab_master_vol);

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
