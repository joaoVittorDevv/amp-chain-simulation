use nih_plug::prelude::*;
use nih_plug_egui::EguiState;
use std::sync::{Arc, Mutex};
use rtrb::Consumer;
use crate::core::dsp;

#[derive(Enum, PartialEq, Clone, Copy, Debug)]
pub enum InputSelect {
    #[name = "1/2 (Stereo)"]
    Stereo,
    #[name = "Input 1 (Mic)"]
    Input1,
    #[name = "Input 2 (Guitar)"]
    Input2,
}

#[derive(Enum, PartialEq, Clone, Copy, Debug)]
pub enum PreampChannel {
    #[name = "Clean"]
    Clean,
    #[name = "Dirty"]
    Dirty,
}

#[derive(Enum, PartialEq, Clone, Copy, Debug)]
pub enum PreampDriveMode {
    #[name = "Moderate Drive"]
    ModerateDrive,
    #[name = "High Gain"]
    HighGain,
}

#[derive(Enum, PartialEq, Clone, Copy, Debug)]
pub enum CabinetDimension {
    #[name = "1x12"]
    OneByTwelve,
    #[name = "2x12"]
    TwoByTwelve,
    #[name = "4x12"]
    FourByTwelve,
}

#[derive(Params)]
pub struct BaseIOParams {
    #[persist = "editor-state"]
    pub editor_state: Arc<EguiState>,

    #[id = "input"]
    pub input_select: EnumParam<InputSelect>,

    #[id = "gain"]
    pub gain: FloatParam,

    #[id = "bypass"]
    pub bypass: BoolParam,

    // --- Neural Amp ---
    #[id = "neural_amp_volume"]
    pub neural_amp_volume: FloatParam,

    #[id = "neural_drive"]
    pub neural_drive: FloatParam,

    #[id = "neural_output_gain"]
    pub neural_output_gain: FloatParam,

    #[id = "neural_amp_active"]
    pub neural_amp_active: BoolParam,

    // --- Preamp ---
    #[id = "preamp_input_vol"]
    pub preamp_input_vol: FloatParam,

    #[id = "preamp_gain"]
    pub preamp_gain: FloatParam,

    #[id = "preamp_bass"]
    pub preamp_bass: FloatParam,

    #[id = "preamp_mid"]
    pub preamp_mid: FloatParam,

    #[id = "preamp_treble"]
    pub preamp_treble: FloatParam,

    #[id = "preamp_master"]
    pub preamp_master: FloatParam,

    #[id = "preamp_channel"]
    pub preamp_channel: EnumParam<PreampChannel>,

    #[id = "preamp_active"]
    pub preamp_active: BoolParam,

    #[id = "preamp_drive_mode"]
    pub preamp_drive_mode: EnumParam<PreampDriveMode>,

    // --- Cabinet ---
    #[id = "mic_position"]
    pub mic_position: FloatParam,

    #[id = "mic_distance"]
    pub mic_distance: FloatParam,

    #[id = "cabinet_dimension"]
    pub cabinet_dimension: EnumParam<CabinetDimension>,

    #[id = "use_custom_ir"]
    pub use_custom_ir: BoolParam,

    #[id = "cabinet_active"]
    pub cabinet_active: BoolParam,

    #[id = "cabinet_master_volume"]
    pub cabinet_master_volume: FloatParam,
}

pub struct EditorState {
    pub params: Arc<BaseIOParams>,
    pub analyzer: dsp::AnalyzerDsp,
    pub consumer: Arc<Mutex<Option<Consumer<f32>>>>,
    pub active_panel: crate::core::ui::ActivePanel,
    pub custom_ir: Arc<arc_swap::ArcSwapOption<crate::core::dsp::cabinet::ir_convolver::IrData>>,
    pub cab_clipping_meter: Arc<nih_plug::params::smoothing::AtomicF32>,
    pub loaded_ir_name: String,
    pub ir_load_error: Option<String>,
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

        bypass: BoolParam::new("Bypass", false),

        // --- Neural Amp defaults ---
        neural_amp_volume: FloatParam::new(
            "Neural Volume",
            util::db_to_gain(0.0),
            FloatRange::Skewed {
                min: util::db_to_gain(-24.0),
                max: util::db_to_gain(12.0),
                factor: FloatRange::gain_skew_factor(-24.0, 12.0),
            },
        )
        .with_smoother(SmoothingStyle::Logarithmic(50.0))
        .with_unit(" dB")
        .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
        .with_string_to_value(formatters::s2v_f32_gain_to_db()),

        neural_drive: FloatParam::new(
            "Neural Drive",
            util::db_to_gain(0.0),
            FloatRange::Skewed {
                min: util::db_to_gain(0.0),
                max: util::db_to_gain(30.0),
                factor: FloatRange::gain_skew_factor(0.0, 30.0),
            },
        )
        .with_smoother(SmoothingStyle::Logarithmic(50.0))
        .with_unit(" dB")
        .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
        .with_string_to_value(formatters::s2v_f32_gain_to_db()),

        neural_output_gain: FloatParam::new(
            "Neural Makeup",
            util::db_to_gain(0.0),
            FloatRange::Skewed {
                min: util::db_to_gain(-24.0),
                max: util::db_to_gain(12.0),
                factor: FloatRange::gain_skew_factor(-24.0, 12.0),
            },
        )
        .with_smoother(SmoothingStyle::Logarithmic(50.0))
        .with_unit(" dB")
        .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
        .with_string_to_value(formatters::s2v_f32_gain_to_db()),

        neural_amp_active: BoolParam::new("Neural Amp Active", true),

        // --- Preamp defaults ---
        preamp_input_vol: FloatParam::new(
            "Input Volume",
            0.5,
            FloatRange::Linear { min: 0.0, max: 1.0 },
        )
        .with_smoother(SmoothingStyle::Linear(20.0))
        .with_unit(""),

        preamp_gain: FloatParam::new(
            "Drive",
            0.0,
            FloatRange::Linear { min: 0.0, max: 1.0 },
        )
        .with_smoother(SmoothingStyle::Linear(30.0))
        .with_unit(""),

        preamp_bass: FloatParam::new(
            "Bass",
            0.0,
            FloatRange::Linear { min: -15.0, max: 15.0 },
        )
        .with_smoother(SmoothingStyle::Linear(50.0))
        .with_unit(" dB"),

        preamp_mid: FloatParam::new(
            "Mid",
            0.0,
            FloatRange::Linear { min: -15.0, max: 15.0 },
        )
        .with_smoother(SmoothingStyle::Linear(50.0))
        .with_unit(" dB"),

        preamp_treble: FloatParam::new(
            "Treble",
            0.0,
            FloatRange::Linear { min: -15.0, max: 15.0 },
        )
        .with_smoother(SmoothingStyle::Linear(50.0))
        .with_unit(" dB"),

        preamp_master: FloatParam::new(
            "Master Volume",
            0.7,
            FloatRange::Linear { min: 0.0, max: 1.0 },
        )
        .with_smoother(SmoothingStyle::Linear(20.0))
        .with_unit(""),

        preamp_channel: EnumParam::new("Channel", PreampChannel::Clean),
        preamp_active: BoolParam::new("Preamp Active", true),
        preamp_drive_mode: EnumParam::new("Drive Mode", PreampDriveMode::ModerateDrive),

        // --- Cabinet defaults ---
        mic_position: FloatParam::new(
            "Mic Position",
            0.5,
            FloatRange::Linear { min: 0.0, max: 1.0 },
        )
        .with_smoother(SmoothingStyle::Linear(50.0)),

        mic_distance: FloatParam::new(
            "Mic Distance",
            0.0,
            FloatRange::Linear { min: 0.0, max: 1.0 },
        )
        .with_smoother(SmoothingStyle::Linear(50.0)),

        cabinet_dimension: EnumParam::new("Cabinet", CabinetDimension::OneByTwelve),

        use_custom_ir: BoolParam::new("Use Custom IR", false),
        cabinet_active: BoolParam::new("Cabinet Active", true),

        cabinet_master_volume: FloatParam::new(
            "Master Volume",
            1.0,
            FloatRange::Linear { min: 0.0, max: 2.0 },
        )
        .with_smoother(SmoothingStyle::Linear(20.0)),
        }
    }
}
