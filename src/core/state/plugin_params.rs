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

    #[id = "mic_position"]
    pub mic_position: FloatParam,

    #[id = "mic_distance"]
    pub mic_distance: FloatParam,

    #[id = "cabinet_dimension"]
    pub cabinet_dimension: EnumParam<CabinetDimension>,
}

pub struct EditorState {
    pub params: Arc<BaseIOParams>,
    pub analyzer: dsp::AnalyzerDsp,
    pub consumer: Arc<Mutex<Option<Consumer<f32>>>>,
    pub panel_open: bool,
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
        }
    }
}
