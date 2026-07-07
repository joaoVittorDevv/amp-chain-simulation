use crate::core::dsp;
use nih_plug::prelude::*;
use nih_plug_egui::EguiState;
use rtrb::Consumer;
use std::sync::{Arc, Mutex};

#[derive(Enum, PartialEq, Clone, Copy, Debug)]
pub enum InputSelect {
    #[name = "1/2 (Stereo)"]
    Stereo,
    #[name = "Input 1 (Mic)"]
    Input1,
    #[name = "Input 2 (Guitar)"]
    Input2,
}

#[derive(Enum, PartialEq, Eq, Clone, Copy, Debug)]
pub enum AmpModel {
    #[name = "Neural"]
    Neural,
    #[name = "MLC ZERO V"]
    MlcZeroV,
}

#[derive(Enum, PartialEq, Eq, Clone, Copy, Debug)]
pub enum MlcBright {
    #[name = "I"]
    I,
    #[name = "II"]
    Ii,
}

#[derive(Enum, PartialEq, Eq, Clone, Copy, Debug)]
pub enum MlcFeedback {
    #[name = "Lo"]
    Lo,
    #[name = "Hi"]
    Hi,
}

#[derive(Enum, PartialEq, Eq, Clone, Copy, Debug)]
pub enum MlcGatePos {
    #[name = "Pre"]
    Pre,
    #[name = "Post"]
    Post,
}

/// Selectable clipping / saturation curve for the MLC ZERO V gain stages.
/// The integer index (0-10) is passed to the Faust `clip_type` parameter and
/// must stay in sync with the `clip()` selector in `dsp/mlc_zero_v.dsp`.
#[derive(Enum, PartialEq, Eq, Clone, Copy, Debug)]
pub enum ClipType {
    #[name = "Tanh"]
    Tanh,
    #[name = "Hard Clip"]
    HardClip,
    #[name = "Soft Sine"]
    SoftSine,
    #[name = "ArcTan"]
    ArcTan,
    #[name = "Algebraic"]
    Algebraic,
    #[name = "Rational"]
    Rational,
    #[name = "Exponential"]
    Exponential,
    #[name = "Cubic"]
    Cubic,
    #[name = "Asymmetric Tanh"]
    AsymmetricTanh,
    #[name = "Wave Fold"]
    WaveFold,
    #[name = "Asymmetric Hard"]
    AsymmetricHard,
}

impl ClipType {
    /// All 11 curves in DSP index order.
    pub const ALL: [ClipType; 11] = [
        ClipType::Tanh,
        ClipType::HardClip,
        ClipType::SoftSine,
        ClipType::ArcTan,
        ClipType::Algebraic,
        ClipType::Rational,
        ClipType::Exponential,
        ClipType::Cubic,
        ClipType::AsymmetricTanh,
        ClipType::WaveFold,
        ClipType::AsymmetricHard,
    ];

    /// DSP selector index (0-10) passed to the Faust `Clip Type` parameter.
    pub fn as_f32(self) -> f32 {
        match self {
            ClipType::Tanh => 0.0,
            ClipType::HardClip => 1.0,
            ClipType::SoftSine => 2.0,
            ClipType::ArcTan => 3.0,
            ClipType::Algebraic => 4.0,
            ClipType::Rational => 5.0,
            ClipType::Exponential => 6.0,
            ClipType::Cubic => 7.0,
            ClipType::AsymmetricTanh => 8.0,
            ClipType::WaveFold => 9.0,
            ClipType::AsymmetricHard => 10.0,
        }
    }

    /// Short display name for the curve.
    pub fn label(self) -> &'static str {
        match self {
            ClipType::Tanh => "Tanh",
            ClipType::HardClip => "Hard Clip",
            ClipType::SoftSine => "Soft Sine",
            ClipType::ArcTan => "ArcTan",
            ClipType::Algebraic => "Algebraic",
            ClipType::Rational => "Rational",
            ClipType::Exponential => "Exponential",
            ClipType::Cubic => "Cubic",
            ClipType::AsymmetricTanh => "Asymmetric Tanh",
            ClipType::WaveFold => "Wave Fold",
            ClipType::AsymmetricHard => "Asymmetric Hard",
        }
    }

    /// Human-readable description shown in the clip-type dropdown.
    pub fn description(self) -> &'static str {
        match self {
            ClipType::Tanh => {
                "Tanh — Compressão suave e cremosa. Referência clássica de amplificador valvulado."
            }
            ClipType::HardClip => {
                "Hard Clip — Clip abrupto. Fuzz agressivo tipo transistor (Big Muff, RAT)."
            }
            ClipType::SoftSine => {
                "Soft Sine — Extremamente suave. Som limpo com leve saturação 'beira da distorção'."
            }
            ClipType::ArcTan => {
                "ArcTan — Saturação mais aberta que Tanh. Som limpo e articulado com menos compressão."
            }
            ClipType::Algebraic => {
                "Algebraic — Cremoso nos médios, sem fizz nos agudos. Potencialmente a mais musical."
            }
            ClipType::Rational => {
                "Rational — Agressivo e fuzzy. Caráter de transistor de germânio."
            }
            ClipType::Exponential => {
                "Exponential — Muito agressivo. Timbre de pedal RAT/DS-1."
            }
            ClipType::Cubic => {
                "Cubic — Boost limpo com saturação muito sutil. Ideal para pré-amplificador."
            }
            ClipType::AsymmetricTanh => {
                "Asymmetric Tanh — Tanh com bias assimétrico. Harmônicos pares — som valvulado rico."
            }
            ClipType::WaveFold => {
                "Wave Fold — Dobramento de onda. Metálico, cortante, timbre tipo 'djent'."
            }
            ClipType::AsymmetricHard => {
                "Asymmetric Hard — Hard clip com thresholds diferentes. Distorção assimétrica agressiva."
            }
        }
    }
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

    #[id = "amp_model"]
    pub amp_model: EnumParam<AmpModel>,

    // --- Neural Amp ---
    #[id = "neural_amp_volume"]
    pub neural_amp_volume: FloatParam,

    #[id = "neural_drive"]
    pub neural_drive: FloatParam,

    #[id = "neural_output_gain"]
    pub neural_output_gain: FloatParam,

    #[id = "neural_amp_active"]
    pub neural_amp_active: BoolParam,

    // --- MLC ZERO V ---
    #[id = "mlc_gain"]
    pub mlc_gain: FloatParam,

    #[id = "mlc_master"]
    pub mlc_master: FloatParam,

    #[id = "mlc_bass"]
    pub mlc_bass: FloatParam,

    #[id = "mlc_middle"]
    pub mlc_middle: FloatParam,

    #[id = "mlc_treble"]
    pub mlc_treble: FloatParam,

    #[id = "mlc_presence"]
    pub mlc_presence: FloatParam,

    #[id = "mlc_depth"]
    pub mlc_depth: FloatParam,

    #[id = "mlc_gate"]
    pub mlc_gate: FloatParam,

    #[id = "mlc_bright"]
    pub mlc_bright: EnumParam<MlcBright>,

    #[id = "mlc_m45"]
    pub mlc_m45: BoolParam,

    #[id = "mlc_warclaw"]
    pub mlc_warclaw: BoolParam,

    #[id = "mlc_feedback"]
    pub mlc_feedback: EnumParam<MlcFeedback>,

    #[id = "mlc_gate_pos"]
    pub mlc_gate_pos: EnumParam<MlcGatePos>,

    #[id = "mlc_clip_type"]
    pub mlc_clip_type: EnumParam<ClipType>,

    // --- Cabinet IR ---
    #[id = "cab_bypass"]
    pub cabinet_bypass: BoolParam,

    #[id = "cab_level"]
    pub cabinet_level: FloatParam,

    #[id = "cab_mix"]
    pub cabinet_mix: FloatParam,

    /// Content hash of the selected cabinet IR (empty = none). Not automatable;
    /// persisted per DAW project so the selection survives reload.
    #[persist = "cab_active_hash"]
    pub cab_active_hash: std::sync::RwLock<String>,

    // --- Parametric EQ ---
    #[id = "eq_active"]
    pub eq_active: BoolParam,

    #[id = "eq_tanh_bypass"]
    pub eq_tanh_bypass: BoolParam,

    #[id = "eq_low_freq"]
    pub eq_low_freq: FloatParam,
    #[id = "eq_low_gain"]
    pub eq_low_gain: FloatParam,
    #[id = "eq_low_q"]
    pub eq_low_q: FloatParam,

    #[id = "eq_mid_freq"]
    pub eq_mid_freq: FloatParam,
    #[id = "eq_mid_gain"]
    pub eq_mid_gain: FloatParam,
    #[id = "eq_mid_q"]
    pub eq_mid_q: FloatParam,

    #[id = "eq_high_freq"]
    pub eq_high_freq: FloatParam,
    #[id = "eq_high_gain"]
    pub eq_high_gain: FloatParam,
    #[id = "eq_high_q"]
    pub eq_high_q: FloatParam,

    // --- Brickwall Limiter (MLC ZERO V) ---
    #[id = "lim_en"]
    pub limiter_enable: BoolParam,

    #[id = "lim_ceil"]
    pub limiter_ceiling: FloatParam,

    #[id = "lim_rel"]
    pub limiter_release: FloatParam,
}

pub struct EditorState {
    pub params: Arc<BaseIOParams>,
    pub analyzer: dsp::AnalyzerDsp,
    pub consumer: Arc<Mutex<Option<Consumer<f32>>>>,
    pub active_panel: crate::core::ui::ActivePanel,

    // --- Cabinet IR (UI-thread only) ---
    pub cabinet_library: Arc<Mutex<crate::core::cabinet::CabinetLibrary>>,
    pub cabinet_mailbox: Arc<crate::core::cabinet::CabinetMailbox>,
    /// Current engine sample rate (Hz), kept fresh by `initialize()`.
    pub cabinet_sr: Arc<std::sync::atomic::AtomicU32>,
    /// Current engine max block size, kept fresh by `initialize()`.
    pub cabinet_max_block: Arc<std::sync::atomic::AtomicUsize>,
    /// Last import/decode error, shown in the panel until the next successful op.
    pub cabinet_error: Arc<Mutex<Option<String>>>,

    /// Shared Component Lab facade for UI/database operations.
    #[cfg(feature = "lab")]
    pub lab: Option<Arc<crate::lab::Lab>>,
    /// Last Component Lab initialization or UI error.
    #[cfg(feature = "lab")]
    pub lab_error: Arc<Mutex<Option<String>>>,
    /// Retained Component Lab panel state.
    #[cfg(feature = "lab")]
    pub lab_ui: crate::core::ui::LabUiState,
    /// Lab pipeline mailboxes collected on the UI thread.
    #[cfg(feature = "lab")]
    pub lab_mailboxes: Vec<Arc<crate::lab::VariantMailbox>>,
}

impl Default for BaseIOParams {
    fn default() -> Self {
        Self {
            editor_state: EguiState::from_size(1200, 800),

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

            amp_model: EnumParam::new("Amp Model", AmpModel::Neural),

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

            // --- MLC ZERO V defaults ---
            mlc_gain: FloatParam::new(
                "MLC Gain",
                util::db_to_gain(-12.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-60.0),
                    max: util::db_to_gain(0.0),
                    factor: FloatRange::gain_skew_factor(-60.0, 0.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),

            mlc_master: FloatParam::new(
                "MLC Master",
                util::db_to_gain(-6.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-60.0),
                    max: util::db_to_gain(0.0),
                    factor: FloatRange::gain_skew_factor(-60.0, 0.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),

            mlc_bass: FloatParam::new(
                "MLC Bass",
                0.0,
                FloatRange::Linear {
                    min: -12.0,
                    max: 12.0,
                },
            )
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
            mlc_middle: FloatParam::new(
                "MLC Middle",
                0.0,
                FloatRange::Linear {
                    min: -12.0,
                    max: 12.0,
                },
            )
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
            mlc_treble: FloatParam::new(
                "MLC Treble",
                0.0,
                FloatRange::Linear {
                    min: -12.0,
                    max: 12.0,
                },
            )
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
            mlc_presence: FloatParam::new(
                "MLC Presence",
                0.0,
                FloatRange::Linear {
                    min: -12.0,
                    max: 12.0,
                },
            )
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
            mlc_depth: FloatParam::new(
                "MLC Depth",
                0.0,
                FloatRange::Linear {
                    min: -12.0,
                    max: 12.0,
                },
            )
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
            mlc_gate: FloatParam::new(
                "MLC Gate",
                -80.0,
                FloatRange::Linear {
                    min: -80.0,
                    max: 0.0,
                },
            )
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),

            mlc_bright: EnumParam::new("MLC Bright", MlcBright::Ii),
            mlc_m45: BoolParam::new("MLC M45", false),
            mlc_warclaw: BoolParam::new("MLC WARCLAW", false),
            mlc_feedback: EnumParam::new("MLC Feedback", MlcFeedback::Hi),
            mlc_gate_pos: EnumParam::new("MLC Gate Pos", MlcGatePos::Pre),
            mlc_clip_type: EnumParam::new("MLC Clip Type", ClipType::Tanh),

            // --- Cabinet IR defaults ---
            cabinet_bypass: BoolParam::new("Cabinet Bypass", false),

            cabinet_level: FloatParam::new(
                "Cabinet Level",
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

            cabinet_mix: FloatParam::new(
                "Cabinet Mix",
                1.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_unit(" %")
            .with_value_to_string(formatters::v2s_f32_percentage(0))
            .with_string_to_value(formatters::s2v_f32_percentage()),

            cab_active_hash: std::sync::RwLock::new(String::new()),

            // --- EQ Defaults ---
            eq_active: BoolParam::new("EQ Active", true),
            eq_tanh_bypass: BoolParam::new("EQ Tanh Bypass", false),

            eq_low_freq: FloatParam::new(
                "Low Freq",
                100.0,
                FloatRange::Skewed {
                    min: 20.0,
                    max: 1000.0,
                    factor: FloatRange::skew_factor(150.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" Hz")
            .with_value_to_string(formatters::v2s_f32_hz_then_khz(2))
            .with_string_to_value(formatters::s2v_f32_hz_then_khz()),
            eq_low_gain: FloatParam::new(
                "Low Gain",
                0.0,
                FloatRange::Linear {
                    min: -12.0,
                    max: 12.0,
                },
            )
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
            eq_low_q: FloatParam::new(
                "Low Q",
                0.707,
                FloatRange::Skewed {
                    min: 0.707,
                    max: 10.0,
                    factor: FloatRange::skew_factor(1.0),
                },
            )
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_value_to_string(formatters::v2s_f32_rounded(2)),

            eq_mid_freq: FloatParam::new(
                "Mid Freq",
                1000.0,
                FloatRange::Skewed {
                    min: 100.0,
                    max: 10000.0,
                    factor: FloatRange::skew_factor(1000.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" Hz")
            .with_value_to_string(formatters::v2s_f32_hz_then_khz(2))
            .with_string_to_value(formatters::s2v_f32_hz_then_khz()),
            eq_mid_gain: FloatParam::new(
                "Mid Gain",
                0.0,
                FloatRange::Linear {
                    min: -12.0,
                    max: 12.0,
                },
            )
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
            eq_mid_q: FloatParam::new(
                "Mid Q",
                0.707,
                FloatRange::Skewed {
                    min: 0.707,
                    max: 10.0,
                    factor: FloatRange::skew_factor(1.0),
                },
            )
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_value_to_string(formatters::v2s_f32_rounded(2)),

            eq_high_freq: FloatParam::new(
                "High Freq",
                5000.0,
                FloatRange::Skewed {
                    min: 1000.0,
                    max: 20000.0,
                    factor: FloatRange::skew_factor(5000.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" Hz")
            .with_value_to_string(formatters::v2s_f32_hz_then_khz(2))
            .with_string_to_value(formatters::s2v_f32_hz_then_khz()),
            eq_high_gain: FloatParam::new(
                "High Gain",
                0.0,
                FloatRange::Linear {
                    min: -12.0,
                    max: 12.0,
                },
            )
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
            eq_high_q: FloatParam::new(
                "High Q",
                0.707,
                FloatRange::Skewed {
                    min: 0.707,
                    max: 10.0,
                    factor: FloatRange::skew_factor(1.0),
                },
            )
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_value_to_string(formatters::v2s_f32_rounded(2)),

            // --- Brickwall Limiter defaults ---
            limiter_enable: BoolParam::new("Limiter Enable", true),
            limiter_ceiling: FloatParam::new(
                "Limiter Ceiling",
                -1.0,
                FloatRange::Linear {
                    min: -12.0,
                    max: 0.0,
                },
            )
            .with_smoother(SmoothingStyle::Linear(10.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_rounded(1)),
            limiter_release: FloatParam::new(
                "Limiter Release",
                50.0,
                FloatRange::Linear {
                    min: 10.0,
                    max: 500.0,
                },
            )
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_unit(" ms")
            .with_value_to_string(formatters::v2s_f32_rounded(0)),
        }
    }
}
