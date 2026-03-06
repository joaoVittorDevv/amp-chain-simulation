use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Stream;
use eframe::egui;
use rtrb::{RingBuffer, Consumer};
use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};
use distortion::core::dsp::{AnalyzerDsp, CabinetProcessor, FFT_SIZE};
use distortion::core::state::plugin_params::CabinetDimension;
use distortion::core::ui::render_shared_panels;
use nih_plug::params::enums::Enum;

/// Estado do cabinet compartilhado entre a UI (eframe) e a thread de áudio (CPAL).
/// Escrito pela UI e lido pelo audio callback: uso de Mutex é seguro aqui pois
/// o audio callback usa `try_lock()` — nunca bloqueia a thread RT.
#[derive(Clone, Copy)]
struct CabinetState {
    mic_pos: f32,
    mic_dist: f32,
    cab_dim: CabinetDimension,
    bypass: bool,
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
    DevicesRefreshed { inputs: Vec<DeviceContext>, outputs: Vec<DeviceContext> },
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
    panel_open: bool,

    /// Estado do Cabinet: UI escreve, worker de áudio lê via try_lock().
    cabinet_state: Arc<Mutex<CabinetState>>,

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
            let prefix = if raw.starts_with("sysdefault") { "(Padrão)" } else { "(Hardware Direto)" };
            let icon = if is_input { "🎙️" } else { "🔊" };
            return (format!("{} {} {}", icon, card_name.replace("_", " "), prefix), false);
        }
    }
    
    (format!("{} (Cru)", raw), false)
}

fn audio_worker(
    rx_cmd: Receiver<AudioCommand>,
    tx_event: Sender<AudioEvent>,
    consumer_mutex: Arc<Mutex<Option<Consumer<f32>>>>,
    cabinet_state: Arc<Mutex<CabinetState>>,
) {
    let mut _input_stream: Option<Stream> = None;
    let mut _output_stream: Option<Stream> = None;

    for cmd in rx_cmd {
        match cmd {
            AudioCommand::RefreshDevices(host_id) => {
                let mut inputs_list = vec![];
                let mut outputs_list = vec![];
                
                if let Ok(host) = cpal::host_from_id(host_id) {
                    // Refresh Inputs
                    if let Ok(devs) = host.input_devices() {
                        for dev in devs {
                            if let Ok(config) = dev.default_input_config() {
                                let raw_name = dev.name().unwrap_or_else(|_| "Unknown Device".to_string());
                                let mut b_name = raw_name.clone();
                                if cfg!(target_os = "linux") {
                                    let (n, _) = beautify_linux_name(&raw_name, true);
                                    b_name = n;
                                    if b_name.contains("(Ocultar)") { continue; }
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
                    // Refresh Outputs
                    if let Ok(devs) = host.output_devices() {
                        for dev in devs {
                            if let Ok(config) = dev.default_output_config() {
                                let raw_name = dev.name().unwrap_or_else(|_| "Unknown Device".to_string());
                                let mut b_name = raw_name.clone();
                                if cfg!(target_os = "linux") {
                                    let (n, _) = beautify_linux_name(&raw_name, false);
                                    b_name = n;
                                    if b_name.contains("(Ocultar)") { continue; }
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
                let _ = tx_event.send(AudioEvent::DevicesRefreshed { inputs: inputs_list, outputs: outputs_list });
            }
            AudioCommand::ApplyRouting { host_id, input, output, buffer_size } => {
                let mut err_msg = None;
                _input_stream = None; 
                _output_stream = None;
                
                // RingBuffer for Analyzer
                let (mut analyzer_producer, analyzer_consumer) = RingBuffer::new(1024 * 64);
                if let Ok(mut cons_lock) = consumer_mutex.lock() {
                    *cons_lock = Some(analyzer_consumer);
                }
                
                // Passthrough RingBuffer (L + R separados para preserve stereo)
                let has_both = input.is_some() && output.is_some();
                let (mut pt_producer, mut pt_consumer) = if has_both {
                    let (p, c) = RingBuffer::new((buffer_size * 8).max(2048) as usize);
                    (Some(p), Some(c))
                } else {
                    (None, None)
                };

                // Criar processadores de cabinet para input stream (L+R).
                // Pré-alocados aqui (safe zone), nunca alocados no callback de áudio.
                let sample_rate_for_cabinet = if let Some(ref inp) = input {
                    // Tentamos pegar o SR do dispositivo; fallback 44100
                    let _ = inp;
                    44100.0_f32
                } else { 44100.0_f32 };
                let mut cabinet_l = CabinetProcessor::new(sample_rate_for_cabinet);
                let mut cabinet_r = CabinetProcessor::new(sample_rate_for_cabinet);
                cabinet_l.initialize(192000.0); // worst-case pre-alloc
                cabinet_r.initialize(192000.0);
                let cabinet_state_clone = cabinet_state.clone();

                if let Ok(host) = cpal::host_from_id(host_id) {
                    
                    // SETUP INPUT
                    if let Some((raw_name, left, right)) = input {
                        if let Ok(devs) = host.input_devices() {
                            if let Some(device) = devs.into_iter().find(|d| d.name().unwrap_or_default() == raw_name) {
                                if let Ok(config) = device.default_input_config() {
                                    let mut strict_config: cpal::StreamConfig = config.clone().into();
                                    strict_config.buffer_size = cpal::BufferSize::Fixed(buffer_size);
                                    let channels = strict_config.channels;
                                    let l_idx = left.min((channels.saturating_sub(1)) as usize);
                                    let r_idx = right.min((channels.saturating_sub(1)) as usize);
                                    
                                    let stream_res = match config.sample_format() {
                                        cpal::SampleFormat::F32 => {
                                            device.build_input_stream(
                                                &strict_config,
                                                move |data: &[f32], _: &_| {
                                                    // Lê parâmetros da UI via try_lock (nunca bloqueia)
                                                    let cab_snapshot = cabinet_state_clone
                                                        .try_lock()
                                                        .map(|g| *g)
                                                        .unwrap_or(CabinetState {
                                                            mic_pos: 0.5, mic_dist: 0.0,
                                                            cab_dim: CabinetDimension::OneByTwelve,
                                                            bypass: false,
                                                        });

                                                    // Atualiza params dos processadores (bloco-a-bloco)
                                                    if !cab_snapshot.bypass {
                                                        cabinet_l.update_params(
                                                            cab_snapshot.mic_pos,
                                                            cab_snapshot.mic_dist,
                                                            cab_snapshot.cab_dim,
                                                        );
                                                        cabinet_r.update_params(
                                                            cab_snapshot.mic_pos,
                                                            cab_snapshot.mic_dist,
                                                            cab_snapshot.cab_dim,
                                                        );
                                                    }

                                                    for frame in data.chunks(channels as usize) {
                                                        let mut l = frame.get(l_idx).copied().unwrap_or(0.0);
                                                        let mut r = frame.get(r_idx).copied().unwrap_or(0.0);

                                                        // Aplica DSP do cabinet (ou bypass)
                                                        if !cab_snapshot.bypass {
                                                            l = cabinet_l.process(l);
                                                            r = cabinet_r.process(r);
                                                            if l.is_nan() || l.is_infinite() { l = 0.0; }
                                                            if r.is_nan() || r.is_infinite() { r = 0.0; }
                                                        }

                                                        // Tap do analyzer APÓS o cabinet (mostra resultado real)
                                                        let mix = (l + r) * 0.5;
                                                        let _ = analyzer_producer.push(mix);

                                                        // Passthrough para output: envia L e R
                                                        if let Some(pp) = pt_producer.as_mut() {
                                                            let _ = pp.push(l);
                                                            let _ = pp.push(r);
                                                        }
                                                    }
                                                },
                                                |err| eprintln!("Input error: {:?}", err),
                                                None
                                            )
                                        },
                                        cpal::SampleFormat::I16 => {
                                            // Processadores separados para o branch I16 (F32 já moveu os anteriores)
                                            let mut cab_l_i16 = CabinetProcessor::new(44100.0);
                                            let mut cab_r_i16 = CabinetProcessor::new(44100.0);
                                            cab_l_i16.initialize(192000.0);
                                            cab_r_i16.initialize(192000.0);
                                            let cab_state_i16 = cabinet_state.clone();
                                            device.build_input_stream(
                                                &strict_config,
                                                move |data: &[i16], _: &_| {
                                                    let cab_snapshot = cab_state_i16
                                                        .try_lock()
                                                        .map(|g| *g)
                                                        .unwrap_or(CabinetState {
                                                            mic_pos: 0.5, mic_dist: 0.0,
                                                            cab_dim: CabinetDimension::OneByTwelve,
                                                            bypass: false,
                                                        });
                                                    if !cab_snapshot.bypass {
                                                        cab_l_i16.update_params(cab_snapshot.mic_pos, cab_snapshot.mic_dist, cab_snapshot.cab_dim);
                                                        cab_r_i16.update_params(cab_snapshot.mic_pos, cab_snapshot.mic_dist, cab_snapshot.cab_dim);
                                                    }
                                                    for frame in data.chunks(channels as usize) {
                                                        let mut l = frame.get(l_idx).copied().unwrap_or(0) as f32 / i16::MAX as f32;
                                                        let mut r = frame.get(r_idx).copied().unwrap_or(0) as f32 / i16::MAX as f32;
                                                        if !cab_snapshot.bypass {
                                                            l = cab_l_i16.process(l);
                                                            r = cab_r_i16.process(r);
                                                            if l.is_nan() || l.is_infinite() { l = 0.0; }
                                                            if r.is_nan() || r.is_infinite() { r = 0.0; }
                                                        }
                                                        let mix = (l + r) * 0.5;
                                                        let _ = analyzer_producer.push(mix);
                                                        if let Some(pp) = pt_producer.as_mut() {
                                                            let _ = pp.push(l);
                                                            let _ = pp.push(r);
                                                        }
                                                    }
                                                },
                                                |err| eprintln!("Input error: {:?}", err),
                                                None
                                            )
                                        },
                                        _ => Err(cpal::BuildStreamError::StreamConfigNotSupported)
                                    };
                                    
                                    match stream_res {
                                        Ok(str) => {
                                            if let Err(e) = str.play() {
                                                err_msg = Some(format!("❌ Falha no Play da Entrada: {:?}", e));
                                            } else {
                                                _input_stream = Some(str);
                                            }
                                        }
                                        Err(e) => { 
                                            err_msg = Some(format!(
                                                "Sua placa de som de Captação negou o pedido de buffer ultra-fino de {} frames.\nErro recebido: {:?}",
                                                buffer_size, e
                                            )); 
                                        }
                                    }
                                }
                            } else {
                                err_msg = Some("Input Físico não encontrado.".to_string());
                            }
                        }
                    }

                    // SETUP OUTPUT
                    if let Some((raw_name, left, right)) = output {
                        if let Ok(devs) = host.output_devices() {
                            if let Some(device) = devs.into_iter().find(|d| d.name().unwrap_or_default() == raw_name) {
                                if let Ok(config) = device.default_output_config() {
                                    let mut strict_config: cpal::StreamConfig = config.clone().into();
                                    strict_config.buffer_size = cpal::BufferSize::Fixed(buffer_size);
                                    let channels = strict_config.channels;
                                    let l_idx = left.min((channels.saturating_sub(1)) as usize);
                                    let r_idx = right.min((channels.saturating_sub(1)) as usize);
                                    
                                    let stream_res = match config.sample_format() {
                                        cpal::SampleFormat::F32 => {
                                            device.build_output_stream(
                                                &strict_config,
                                                move |data: &mut [f32], _: &_| {
                                                    for frame in data.chunks_mut(channels as usize) {
                                                        // Recebe L e R separados do ring buffer
                                                        let (l_sample, r_sample) = match pt_consumer.as_mut() {
                                                            Some(pc) => {
                                                                let l = pc.pop().unwrap_or(0.0);
                                                                let r = pc.pop().unwrap_or(l);
                                                                (l, r)
                                                            }
                                                            None => (0.0, 0.0),
                                                        };
                                                        if let Some(l) = frame.get_mut(l_idx) { *l = l_sample; }
                                                        if let Some(r) = frame.get_mut(r_idx) { *r = r_sample; }
                                                    }
                                                },
                                                |err| eprintln!("Output error: {:?}", err),
                                                None
                                            )
                                        },
                                        cpal::SampleFormat::I16 => {
                                            device.build_output_stream(
                                                &strict_config,
                                                move |data: &mut [i16], _: &_| {
                                                    for frame in data.chunks_mut(channels as usize) {
                                                        let sample = match pt_consumer.as_mut() {
                                                            Some(pc) => pc.pop().unwrap_or(0.0),
                                                            None => 0.0,
                                                        };
                                                        let pcm = (sample * i16::MAX as f32) as i16;
                                                        if let Some(l) = frame.get_mut(l_idx) { *l = pcm; }
                                                        if let Some(r) = frame.get_mut(r_idx) { *r = pcm; }
                                                    }
                                                },
                                                |err| eprintln!("Output error: {:?}", err),
                                                None
                                            )
                                        },
                                        _ => Err(cpal::BuildStreamError::StreamConfigNotSupported)
                                    };
                                    
                                    match stream_res {
                                        Ok(str) => {
                                            if let Err(e) = str.play() {
                                                err_msg = Some(format!("❌ Falha no Play da Saída: {:?}", e));
                                            } else {
                                                _output_stream = Some(str);
                                            }
                                        }
                                        Err(e) => { 
                                            err_msg = Some(format!(
                                                "O seu Driver de Áudio primário de Saída bloqueou o processamento fino de {} frames.\nErro recebido: {:?}",
                                                buffer_size, e
                                            )); 
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

        let cabinet_state = Arc::new(Mutex::new(CabinetState {
            mic_pos: 0.5,
            mic_dist: 0.0,
            cab_dim: CabinetDimension::OneByTwelve,
            bypass: false,
        }));

        let cons_clone = cons_arc.clone();
        let cab_state_clone = cabinet_state.clone();
        thread::spawn(move || {
            audio_worker(rx_worker, tx_worker, cons_clone, cab_state_clone);
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

            buffer_power: 8,       // 256 frames default
            buffer_range_min: 5,   // 32 minimum starting
            buffer_range_max: 10,  // 1024 max starting

            sample_rate_warning: None,
            last_audio_error: None,
            last_audio_error_details: None,
            show_error_popup: false,

            input_is_mono: true,
            output_is_mono: false,

            show_settings: false,
            is_loading: true,
            panel_open: true,
            cabinet_state,
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
                        self.last_audio_error = Some("⚠️ Limitação de Hardware: Buffer negado".to_string());
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
        let _ = self.tx_cmd.send(AudioCommand::RefreshDevices(self.selected_host));
    }
    
    fn apply_audio_routing(&mut self) {
        self.sample_rate_warning = None;

        let input_params = if let Some(idx) = self.selected_input_idx {
            if let Some(dev_ctx) = self.available_inputs.get(idx) {
                self.input_left = self.input_left.min((dev_ctx.channels.saturating_sub(1)) as usize);
                self.input_right = self.input_right.min((dev_ctx.channels.saturating_sub(1)) as usize);
                Some((dev_ctx.raw_name.clone(), self.input_left, self.input_right))
            } else { None }
        } else { None };

        let output_params = if let Some(idx) = self.selected_output_idx {
            if let Some(dev_ctx) = self.available_outputs.get(idx) {
                self.output_left = self.output_left.min((dev_ctx.channels.saturating_sub(1)) as usize);
                self.output_right = self.output_right.min((dev_ctx.channels.saturating_sub(1)) as usize);
                Some((dev_ctx.raw_name.clone(), self.output_left, self.output_right))
            } else { None }
        } else { None };

        // Checar Mismatch de Sample Rate
        if let (Some(in_idx), Some(out_idx)) = (self.selected_input_idx, self.selected_output_idx) {
            if let (Some(in_dev), Some(out_dev)) = (self.available_inputs.get(in_idx), self.available_outputs.get(out_idx)) {
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
                
                // Botão de Bypass — escreve no estado compartilhado
                let current_bypass = self.cabinet_state.lock().map(|g| g.bypass).unwrap_or(false);
                let mut bypass_toggled = current_bypass;
                if ui.checkbox(&mut bypass_toggled, "Bypass").changed() {
                    if let Ok(mut st) = self.cabinet_state.lock() {
                        st.bypass = bypass_toggled;
                    }
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button(if self.show_settings { "❌ Fechar" } else { "⚙ Configurações de Áudio" }).clicked() {
                        self.show_settings = !self.show_settings;
                    }
                    if self.is_loading {
                        ui.label("⚙ Processando Áudio...");
                        ui.spinner();
                    } else if let Some(warn) = &self.sample_rate_warning {
                        ui.visuals_mut().override_text_color = Some(egui::Color32::from_rgb(255, 165, 0));
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
                            
                            // Captura flags e layer_id ANTES de qualquer move do valor
                            let is_pressed    = res.is_pointer_button_down_on();
                            let drag_finished = res.drag_stopped();
                            let layer_id      = res.layer_id;
                            
                            // show_tooltip_at_pointer é instantâneo (sem delay de hover)
                            // is_pointer_button_down_on dispara desde o primeiro clique, mesmo sem arrasto
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

        // Lê estado atual para a UI
        let (cur_pos, cur_dist, cur_dim) = self.cabinet_state.lock()
            .map(|g| (g.mic_pos, g.mic_dist, g.cab_dim))
            .unwrap_or((0.5, 0.0, CabinetDimension::OneByTwelve));

        let mut ui_pos = cur_pos;
        let mut ui_dist = cur_dist;
        let mut ui_dim = cur_dim;

        render_shared_panels(ctx, &mut self.panel_open, &self.analyzer.spectrum, FFT_SIZE, |ui| {
            ui.label("Mic Position:");
            if ui.add(egui::Slider::new(&mut ui_pos, 0.0..=1.0)).changed() {
                if let Ok(mut st) = self.cabinet_state.lock() { st.mic_pos = ui_pos; }
            }

            ui.label("Mic Distance:");
            if ui.add(egui::Slider::new(&mut ui_dist, 0.0..=1.0)).changed() {
                if let Ok(mut st) = self.cabinet_state.lock() { st.mic_dist = ui_dist; }
            }

            ui.label("Cabinet Size:");
            egui::ComboBox::from_id_salt("cab_selector")
                .selected_text(CabinetDimension::variants()[ui_dim.to_index()])
                .show_ui(ui, |ui| {
                    for (i, &name) in CabinetDimension::variants().iter().enumerate() {
                        let variant = CabinetDimension::from_index(i);
                        if ui.selectable_value(&mut ui_dim, variant, name).changed() {
                            if let Ok(mut st) = self.cabinet_state.lock() { st.cab_dim = ui_dim; }
                        }
                    }
                });
        });
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
