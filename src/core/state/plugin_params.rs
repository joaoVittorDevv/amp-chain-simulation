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

/// Which tab of the MLC ZERO V panel is currently shown. Pure UI state — the
/// controls are split across tabs so the panel fits horizontally instead of
/// overflowing. Not a DSP parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MlcTab {
    Tone,
    GainClip,
    Harmonics,
    PowerAmp,
    Limiter,
}

impl Default for MlcTab {
    fn default() -> Self {
        MlcTab::Tone
    }
}

/// Selectable clipping / saturation curve for a MLC ZERO V gain stage.
/// The integer index (0-2) is passed to the Faust `Clip Type N` parameters and
/// must stay in sync with the `clip_sel()` selector in `dsp/mlc_zero_v.dsp`.
#[derive(Enum, PartialEq, Eq, Clone, Copy, Debug, Default)]
pub enum ClipType {
    #[default]
    #[name = "Asymmetric Tanh"]
    AsymmetricTanh,
    #[name = "Exponential"]
    Exponential,
    #[name = "Chebyshev"]
    Chebyshev,
}

impl ClipType {
    /// All curves in DSP index order.
    pub const ALL: [ClipType; 3] = [
        ClipType::AsymmetricTanh,
        ClipType::Exponential,
        ClipType::Chebyshev,
    ];

    /// DSP selector index (0-2) passed to the Faust `Clip Type N` parameter.
    pub fn as_f32(self) -> f32 {
        match self {
            ClipType::AsymmetricTanh => 0.0,
            ClipType::Exponential => 1.0,
            ClipType::Chebyshev => 2.0,
        }
    }

    /// Short display name for the curve.
    pub fn label(self) -> &'static str {
        match self {
            ClipType::AsymmetricTanh => "Asymmetric Tanh",
            ClipType::Exponential => "Exponential",
            ClipType::Chebyshev => "Chebyshev",
        }
    }

    /// Human-readable description shown in the clip-type dropdown.
    pub fn description(self) -> &'static str {
        match self {
            ClipType::AsymmetricTanh => {
                "Asymmetric Tanh — Tanh com bias assimétrico. Harmônicos pares — som valvulado rico."
            }
            ClipType::Exponential => {
                "Exponential — Muito agressivo. Timbre de pedal RAT/DS-1."
            }
            ClipType::Chebyshev => {
                "Chebyshev — Gerador explícito de harmônicos (H2/H3/H4). Brilho e mordida controláveis."
            }
        }
    }
}

/// Real passive tone-stack circuit model (Tier 2.2). Declaration order MUST match
/// the `select_ts` dispatch order (index 0..24) in `dsp/mlc_zero_v.dsp`.
#[derive(Enum, PartialEq, Eq, Clone, Copy, Debug, Default)]
pub enum MlcTsModel {
    #[default]
    #[name = "Bassman"]
    Bassman,
    #[name = "Mesa"]
    Mesa,
    #[name = "Twin"]
    Twin,
    #[name = "Princeton"]
    Princeton,
    #[name = "Fender Blues"]
    FenderBlues,
    #[name = "Fender Default"]
    FenderDefault,
    #[name = "Fender Deville"]
    FenderDeville,
    #[name = "JCM800"]
    Jcm800,
    #[name = "JCM2000"]
    Jcm2000,
    #[name = "JTM45"]
    Jtm45,
    #[name = "M Lead"]
    Mlead,
    #[name = "M2199"]
    M2199,
    #[name = "AC30"]
    Ac30,
    #[name = "AC15"]
    Ac15,
    #[name = "Soldano"]
    Soldano,
    #[name = "Sovtek"]
    Sovtek,
    #[name = "Peavey"]
    Peavey,
    #[name = "Ibanez"]
    Ibanez,
    #[name = "Roland"]
    Roland,
    #[name = "Ampeg"]
    Ampeg,
    #[name = "Ampeg Rev"]
    AmpegRev,
    #[name = "Bogner"]
    Bogner,
    #[name = "Groove"]
    Groove,
    #[name = "Crunch"]
    Crunch,
    #[name = "Gibsen"]
    Gibsen,
}

impl MlcTsModel {
    /// DSP selector index (0-24) passed to the Faust `Tone Stack Model` param.
    pub fn as_f32(self) -> f32 {
        self as u32 as f32
    }
}

/// LUT tube waveshaping model (Tier 3.2): 6 tube types × 3 stages (18 total).
/// Declaration order MUST match the `tube_stage` dispatch (index 0..17).
#[derive(Enum, PartialEq, Eq, Clone, Copy, Debug, Default)]
pub enum MlcTubeModel {
    #[default]
    #[name = "12AX7 T1"]
    Ax7T1,
    #[name = "12AX7 T2"]
    Ax7T2,
    #[name = "12AX7 T3"]
    Ax7T3,
    #[name = "12AT7 T1"]
    At7T1,
    #[name = "12AT7 T2"]
    At7T2,
    #[name = "12AT7 T3"]
    At7T3,
    #[name = "12AU7 T1"]
    Au7T1,
    #[name = "12AU7 T2"]
    Au7T2,
    #[name = "12AU7 T3"]
    Au7T3,
    #[name = "6V6 T1"]
    V6T1,
    #[name = "6V6 T2"]
    V6T2,
    #[name = "6V6 T3"]
    V6T3,
    #[name = "6DJ8 T1"]
    Dj8T1,
    #[name = "6DJ8 T2"]
    Dj8T2,
    #[name = "6DJ8 T3"]
    Dj8T3,
    #[name = "6C16 T1"]
    C16T1,
    #[name = "6C16 T2"]
    C16T2,
    #[name = "6C16 T3"]
    C16T3,
}

impl MlcTubeModel {
    /// DSP selector index (0-17) passed to the Faust `Tube Model` param.
    pub fn as_f32(self) -> f32 {
        self as u32 as f32
    }
}

/// ADAA anti-aliasing order (Tier 3.6). 0 = Off, 1 = ADAA1, 2 = ADAA2.
#[derive(Enum, PartialEq, Eq, Clone, Copy, Debug, Default)]
pub enum MlcAdaaOrder {
    #[default]
    #[name = "Off"]
    Off,
    #[name = "ADAA1"]
    Adaa1,
    #[name = "ADAA2"]
    Adaa2,
}

impl MlcAdaaOrder {
    /// DSP selector index (0-2) passed to the Faust `ADAA Order` param.
    pub fn as_f32(self) -> f32 {
        self as u32 as f32
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

    #[id = "mlc_clip_type1"]
    pub mlc_clip_type1: EnumParam<ClipType>,

    #[id = "mlc_clip_type2"]
    pub mlc_clip_type2: EnumParam<ClipType>,

    #[id = "mlc_clip_type3"]
    pub mlc_clip_type3: EnumParam<ClipType>,

    #[id = "mlc_clean_blend"]
    pub mlc_clean_blend: FloatParam,

    #[id = "mlc_sag"]
    pub mlc_sag: FloatParam,

    #[id = "mlc_h2"]
    pub mlc_h2: FloatParam,

    #[id = "mlc_h3"]
    pub mlc_h3: FloatParam,

    #[id = "mlc_h4"]
    pub mlc_h4: FloatParam,

    #[id = "mlc_tight"]
    pub mlc_tight: BoolParam,

    #[id = "mlc_asymmetry_enable"]
    pub mlc_asymmetry_enable: BoolParam,

    #[id = "mlc_asymmetry"]
    pub mlc_asymmetry: FloatParam,

    #[id = "mlc_preshape"]
    pub mlc_preshape: BoolParam,

    #[id = "mlc_preshape_tight"]
    pub mlc_preshape_tight: FloatParam,

    #[id = "mlc_preshape_bite"]
    pub mlc_preshape_bite: FloatParam,

    // --- Tier 2.2  Tone Stack model ---
    #[id = "mlc_ts_model"]
    pub mlc_ts_model: EnumParam<MlcTsModel>,

    // --- Tier 3.2  Tube waveshaping ---
    #[id = "mlc_tube_model"]
    pub mlc_tube_model: EnumParam<MlcTubeModel>,

    #[id = "mlc_tube_drive"]
    pub mlc_tube_drive: FloatParam,

    #[id = "mlc_tube_bypass"]
    pub mlc_tube_bypass: BoolParam,

    // --- Tier 3.4  Power-amp NFB loop ---
    #[id = "mlc_nfb_presence"]
    pub mlc_nfb_presence: FloatParam,

    #[id = "mlc_nfb_resonance"]
    pub mlc_nfb_resonance: FloatParam,

    #[id = "mlc_nfb_depth"]
    pub mlc_nfb_depth: FloatParam,

    #[id = "mlc_nfb_bypass"]
    pub mlc_nfb_bypass: BoolParam,

    // --- Tier 3.5  Multi-band clipping ---
    #[id = "mlc_mbc_bypass"]
    pub mlc_mbc_bypass: BoolParam,

    #[id = "mlc_mbc_cf_lo"]
    pub mlc_mbc_cf_lo: FloatParam,

    #[id = "mlc_mbc_cf_hi"]
    pub mlc_mbc_cf_hi: FloatParam,

    #[id = "mlc_mbc_drive_lo"]
    pub mlc_mbc_drive_lo: FloatParam,

    #[id = "mlc_mbc_drive_mid"]
    pub mlc_mbc_drive_mid: FloatParam,

    #[id = "mlc_mbc_drive_hi"]
    pub mlc_mbc_drive_hi: FloatParam,

    // --- Tier 3.6  ADAA anti-aliasing ---
    #[id = "mlc_adaa_order"]
    pub mlc_adaa_order: EnumParam<MlcAdaaOrder>,

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

    /// Currently selected tab of the MLC ZERO V panel (UI-thread only).
    pub mlc_tab: MlcTab,

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
            mlc_clip_type1: EnumParam::new("MLC Clip Type 1", ClipType::AsymmetricTanh),
            mlc_clip_type2: EnumParam::new("MLC Clip Type 2", ClipType::AsymmetricTanh),
            mlc_clip_type3: EnumParam::new("MLC Clip Type 3", ClipType::Exponential),

            mlc_clean_blend: FloatParam::new(
                "MLC Clean Blend",
                0.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 0.25,
                },
            )
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_value_to_string(formatters::v2s_f32_rounded(2)),
            mlc_sag: FloatParam::new("MLC Sag", 0.0, FloatRange::Linear { min: 0.0, max: 1.0 })
                .with_smoother(SmoothingStyle::Linear(50.0))
                .with_value_to_string(formatters::v2s_f32_rounded(2)),
            mlc_h2: FloatParam::new(
                "MLC Chebyshev H2",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_value_to_string(formatters::v2s_f32_rounded(2)),
            mlc_h3: FloatParam::new(
                "MLC Chebyshev H3",
                0.7,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_value_to_string(formatters::v2s_f32_rounded(2)),
            mlc_h4: FloatParam::new(
                "MLC Chebyshev H4",
                0.2,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_value_to_string(formatters::v2s_f32_rounded(2)),

            mlc_tight: BoolParam::new("Tight", true),
            mlc_asymmetry_enable: BoolParam::new("Asymmetry Enable", true),
            mlc_asymmetry: FloatParam::new(
                "Asymmetry",
                0.5,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_value_to_string(formatters::v2s_f32_rounded(2)),
            mlc_preshape: BoolParam::new("Pre-Shape", false),
            mlc_preshape_tight: FloatParam::new(
                "Pre-Shape Tight",
                -3.0,
                FloatRange::Linear {
                    min: -6.0,
                    max: 0.0,
                },
            )
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_rounded(1)),
            mlc_preshape_bite: FloatParam::new(
                "Pre-Shape Bite",
                3.0,
                FloatRange::Linear { min: 0.0, max: 6.0 },
            )
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_rounded(1)),

            // --- Tier 2.2 / 3.x defaults (neutral / bypassed) ---
            mlc_ts_model: EnumParam::new("MLC Tone Stack", MlcTsModel::Bassman),
            mlc_tube_model: EnumParam::new("MLC Tube Model", MlcTubeModel::Ax7T1),
            mlc_tube_drive: FloatParam::new(
                "MLC Tube Drive",
                0.0,
                FloatRange::Linear {
                    min: -20.0,
                    max: 20.0,
                },
            )
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_rounded(1)),
            mlc_tube_bypass: BoolParam::new("MLC Tube Bypass", true),

            mlc_nfb_presence: FloatParam::new(
                "MLC NFB Presence",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_value_to_string(formatters::v2s_f32_rounded(2)),
            mlc_nfb_resonance: FloatParam::new(
                "MLC NFB Resonance",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_value_to_string(formatters::v2s_f32_rounded(2)),
            mlc_nfb_depth: FloatParam::new(
                "MLC NFB Depth",
                0.7,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_value_to_string(formatters::v2s_f32_rounded(2)),
            mlc_nfb_bypass: BoolParam::new("MLC NFB Bypass", true),

            mlc_mbc_bypass: BoolParam::new("MLC Multi-Band Bypass", true),
            mlc_mbc_cf_lo: FloatParam::new(
                "MLC XOver Low",
                300.0,
                FloatRange::Skewed {
                    min: 100.0,
                    max: 800.0,
                    factor: FloatRange::skew_factor(-1.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" Hz")
            .with_value_to_string(formatters::v2s_f32_rounded(0)),
            mlc_mbc_cf_hi: FloatParam::new(
                "MLC XOver High",
                3000.0,
                FloatRange::Skewed {
                    min: 1500.0,
                    max: 6000.0,
                    factor: FloatRange::skew_factor(-1.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" Hz")
            .with_value_to_string(formatters::v2s_f32_rounded(0)),
            mlc_mbc_drive_lo: FloatParam::new(
                "MLC Drive Lo",
                1.0,
                FloatRange::Linear { min: 0.1, max: 4.0 },
            )
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_value_to_string(formatters::v2s_f32_rounded(2)),
            mlc_mbc_drive_mid: FloatParam::new(
                "MLC Drive Mid",
                1.0,
                FloatRange::Linear { min: 0.1, max: 4.0 },
            )
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_value_to_string(formatters::v2s_f32_rounded(2)),
            mlc_mbc_drive_hi: FloatParam::new(
                "MLC Drive Hi",
                1.0,
                FloatRange::Linear { min: 0.1, max: 4.0 },
            )
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_value_to_string(formatters::v2s_f32_rounded(2)),

            mlc_adaa_order: EnumParam::new("MLC ADAA Order", MlcAdaaOrder::Off),

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
                -0.2,
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
