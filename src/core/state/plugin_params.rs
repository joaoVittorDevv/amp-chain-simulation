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
}

pub struct EditorState {
    pub params: Arc<BaseIOParams>,
    pub analyzer: dsp::AnalyzerDsp,
    pub consumer: Arc<Mutex<Option<Consumer<f32>>>>,
    pub active_panel: crate::core::ui::ActivePanel,
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
        }
    }
}
