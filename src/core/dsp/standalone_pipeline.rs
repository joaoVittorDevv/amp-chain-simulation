//! The standalone binary's per-block DSP chain, extracted verbatim from the
//! audio input callback in `src/bin/standalone.rs` (T12). This is a
//! mechanical extraction: no arithmetic was changed, only relocated behind
//! a struct so it can be chunked (T14), fed by non-F32 input formats (T15),
//! and covered by allocation-free RT-safety tests (T16) without a 400-line
//! closure in the way.

use crate::bridge::mlc_zero_v::MlcZeroVProcessor;
use crate::bridge::{faust::FaustProcessor, ExternalProcessor, NeuralProcessor};
use crate::core::cabinet::{CabinetEngine, CabinetMailbox};
use crate::core::dsp::PeakLimiter;
use crate::core::state::plugin_params::{
    AmpModel, ClipType, MlcAdaaOrder, MlcBright, MlcFeedback, MlcGatePos, MlcTsModel, MlcTubeModel,
};
use fft_convolver::FFTConvolver;
use std::sync::Arc;

#[cfg(feature = "lab")]
use crate::lab::PipelineManager;

/// Length, in samples, of the equal-power crossfade applied when switching
/// amp models (`AmpModel::Neural` <-> `AmpModel::MlcZeroV`) mid-stream.
pub const CROSSFADE_LEN: usize = 480;

/// Copy-able snapshot of every parameter the pipeline reads per block. Owned
/// by the caller (UI/atomics live outside the audio thread) and passed in by
/// reference so `process()` never touches a lock.
#[derive(Clone, Copy)]
pub struct AudioSnapshot {
    pub eq_active: bool,
    pub eq_low_freq: f32,
    pub eq_low_gain: f32,
    pub eq_low_q: f32,
    pub eq_mid_freq: f32,
    pub eq_mid_gain: f32,
    pub eq_mid_q: f32,
    pub eq_high_freq: f32,
    pub eq_high_gain: f32,
    pub eq_high_q: f32,
    pub neural_active: bool,
    pub neural_drive: f32,
    pub neural_output_gain: f32,
    pub neural_amp_volume: f32,
    pub amp_model: AmpModel,
    pub mlc_gain: f32,
    pub mlc_master: f32,
    pub mlc_bass: f32,
    pub mlc_middle: f32,
    pub mlc_treble: f32,
    pub mlc_presence: f32,
    pub mlc_depth: f32,
    pub mlc_gate: f32,
    pub mlc_bright: MlcBright,
    pub mlc_m45: bool,
    pub mlc_warclaw: bool,
    pub mlc_feedback: MlcFeedback,
    pub mlc_gate_pos: MlcGatePos,
    pub mlc_clip_type1: ClipType,
    pub mlc_clip_type2: ClipType,
    pub mlc_clip_type3: ClipType,
    pub mlc_clean_blend: f32,
    pub mlc_sag: f32,
    pub mlc_h2: f32,
    pub mlc_h3: f32,
    pub mlc_h4: f32,
    pub mlc_tight: bool,
    pub mlc_asymmetry_enable: bool,
    pub mlc_asymmetry: f32,
    pub mlc_preshape: bool,
    pub mlc_preshape_tight: f32,
    pub mlc_preshape_bite: f32,
    pub mlc_ts_model: MlcTsModel,
    pub mlc_tube_model: MlcTubeModel,
    pub mlc_tube_drive: f32,
    pub mlc_tube_bypass: bool,
    pub mlc_nfb_presence: f32,
    pub mlc_nfb_resonance: f32,
    pub mlc_nfb_depth: f32,
    pub mlc_nfb_bypass: bool,
    pub mlc_mbc_bypass: bool,
    pub mlc_mbc_cf_lo: f32,
    pub mlc_mbc_cf_hi: f32,
    pub mlc_mbc_drive_lo: f32,
    pub mlc_mbc_drive_mid: f32,
    pub mlc_mbc_drive_hi: f32,
    pub mlc_adaa_order: MlcAdaaOrder,
    pub eq_tanh_bypass: bool,
    pub gain: f32,
    pub bypass: bool,
    pub cabinet_bypass: bool,
    pub cabinet_level: f32,
    pub cabinet_mix: f32,
    pub limiter_enable: bool,
    pub limiter_ceiling: f32,
    pub limiter_release: f32,
}

impl Default for AudioSnapshot {
    /// Mirrors `StandaloneState::default()` in `src/bin/standalone.rs` (kept
    /// in sync by hand: `AudioSnapshot` lives in this crate so the orphan
    /// rule forbids implementing `Default` for it from the bin crate).
    fn default() -> Self {
        Self {
            eq_active: true,
            eq_low_freq: 100.0,
            eq_low_gain: 0.0,
            eq_low_q: 0.707,
            eq_mid_freq: 1000.0,
            eq_mid_gain: 0.0,
            eq_mid_q: 1.0,
            eq_high_freq: 5000.0,
            eq_high_gain: 0.0,
            eq_high_q: 0.707,
            neural_active: true,
            neural_drive: 1.0,
            neural_output_gain: 1.0,
            neural_amp_volume: 1.0,
            amp_model: AmpModel::Neural,
            mlc_gain: 0.25118864,
            mlc_master: 0.5011872,
            mlc_bass: 0.0,
            mlc_middle: 0.0,
            mlc_treble: 0.0,
            mlc_presence: 0.0,
            mlc_depth: 0.0,
            mlc_gate: -80.0,
            mlc_bright: MlcBright::Ii,
            mlc_m45: false,
            mlc_warclaw: false,
            mlc_feedback: MlcFeedback::Hi,
            mlc_gate_pos: MlcGatePos::Pre,
            mlc_clip_type1: ClipType::AsymmetricTanh,
            mlc_clip_type2: ClipType::AsymmetricTanh,
            mlc_clip_type3: ClipType::Exponential,
            mlc_clean_blend: 0.0,
            mlc_sag: 0.0,
            mlc_h2: 0.0,
            mlc_h3: 0.7,
            mlc_h4: 0.2,
            mlc_tight: true,
            mlc_asymmetry_enable: true,
            mlc_asymmetry: 0.5,
            mlc_preshape: false,
            mlc_preshape_tight: -3.0,
            mlc_preshape_bite: 3.0,
            mlc_ts_model: MlcTsModel::Bassman,
            mlc_tube_model: MlcTubeModel::Ax7T1,
            mlc_tube_drive: 0.0,
            mlc_tube_bypass: true,
            mlc_nfb_presence: 0.0,
            mlc_nfb_resonance: 0.0,
            mlc_nfb_depth: 0.7,
            mlc_nfb_bypass: true,
            mlc_mbc_bypass: true,
            mlc_mbc_cf_lo: 300.0,
            mlc_mbc_cf_hi: 3000.0,
            mlc_mbc_drive_lo: 1.0,
            mlc_mbc_drive_mid: 1.0,
            mlc_mbc_drive_hi: 1.0,
            mlc_adaa_order: MlcAdaaOrder::Off,
            eq_tanh_bypass: false,
            gain: 1.0,
            bypass: false,
            cabinet_bypass: false,
            cabinet_level: 1.0,
            cabinet_mix: 1.0,
            limiter_enable: true,
            limiter_ceiling: -0.2,
            limiter_release: 50.0,
        }
    }
}

#[inline(always)]
fn process_standalone_amp(
    amp_model: AmpModel,
    snap: &AudioSnapshot,
    neural_amp_l: &mut NeuralProcessor,
    neural_amp_r: &mut NeuralProcessor,
    mlc_l: &mut Option<MlcZeroVProcessor>,
    mlc_r: &mut Option<MlcZeroVProcessor>,
    buf_l: &mut [f32],
    buf_r: &mut [f32],
) {
    match amp_model {
        AmpModel::Neural => {
            if snap.neural_active {
                neural_amp_l.set_drive(snap.neural_drive);
                neural_amp_l.set_output_gain(snap.neural_output_gain);
                neural_amp_l.process_block(buf_l.as_mut_ptr(), buf_l.len());

                neural_amp_r.set_drive(snap.neural_drive);
                neural_amp_r.set_output_gain(snap.neural_output_gain);
                neural_amp_r.process_block(buf_r.as_mut_ptr(), buf_r.len());
            }
        }
        AmpModel::MlcZeroV => {
            let bright = match snap.mlc_bright {
                MlcBright::I => 0.0,
                MlcBright::Ii => 1.0,
            };
            let feedback = match snap.mlc_feedback {
                MlcFeedback::Lo => 0.0,
                MlcFeedback::Hi => 1.0,
            };
            let gate_pos = match snap.mlc_gate_pos {
                MlcGatePos::Pre => 0.0,
                MlcGatePos::Post => 1.0,
            };
            let clip_type1 = snap.mlc_clip_type1.as_f32();
            let clip_type2 = snap.mlc_clip_type2.as_f32();
            let clip_type3 = snap.mlc_clip_type3.as_f32();
            let tight = if snap.mlc_tight { 1.0 } else { 0.0 };
            let asymmetry_enable = if snap.mlc_asymmetry_enable { 1.0 } else { 0.0 };
            let preshape = if snap.mlc_preshape { 1.0 } else { 0.0 };
            let ts_model = snap.mlc_ts_model.as_f32();
            let tube_model = snap.mlc_tube_model.as_f32();
            let adaa_order = snap.mlc_adaa_order.as_f32();
            // One MLC instance costs roughly two thirds of the real-time
            // budget on a typical machine, so running both channels always
            // overruns the callback and the output degrades into clicks. With
            // a mono source (the standalone's "Entrada Mono" default) the two
            // buffers are bit-identical all the way to this stage — the same
            // deterministic per-channel processors saw the same input — so
            // the right channel can be a copy instead of a second full pass.
            // True stereo exits the comparison at the first differing sample.
            let mono_input = buf_l[..] == buf_r[..];
            if let Some(mlc) = mlc_l {
                mlc.set_gain(snap.mlc_gain);
                mlc.set_master(snap.mlc_master);
                mlc.set_bass(snap.mlc_bass);
                mlc.set_middle(snap.mlc_middle);
                mlc.set_treble(snap.mlc_treble);
                mlc.set_presence(snap.mlc_presence);
                mlc.set_depth(snap.mlc_depth);
                mlc.set_gate(snap.mlc_gate);
                mlc.set_bright(bright);
                mlc.set_m45(snap.mlc_m45);
                mlc.set_warclaw(snap.mlc_warclaw);
                mlc.set_feedback(feedback);
                mlc.set_gate_pos(gate_pos);
                mlc.set_clip_type1(clip_type1);
                mlc.set_clip_type2(clip_type2);
                mlc.set_clip_type3(clip_type3);
                mlc.set_tight(tight);
                mlc.set_asymmetry_enable(asymmetry_enable);
                mlc.set_asymmetry(snap.mlc_asymmetry);
                mlc.set_preshape(preshape);
                mlc.set_preshape_tight(snap.mlc_preshape_tight);
                mlc.set_preshape_bite(snap.mlc_preshape_bite);
                mlc.set_clean_blend(snap.mlc_clean_blend);
                mlc.set_sag(snap.mlc_sag);
                mlc.set_h2(snap.mlc_h2);
                mlc.set_h3(snap.mlc_h3);
                mlc.set_h4(snap.mlc_h4);
                mlc.set_ts_model(ts_model);
                mlc.set_tube_model(tube_model);
                mlc.set_tube_drive(snap.mlc_tube_drive);
                mlc.set_tube_bypass(snap.mlc_tube_bypass);
                mlc.set_nfb_presence(snap.mlc_nfb_presence);
                mlc.set_nfb_resonance(snap.mlc_nfb_resonance);
                mlc.set_nfb_depth(snap.mlc_nfb_depth);
                mlc.set_nfb_bypass(snap.mlc_nfb_bypass);
                mlc.set_mbc_bypass(snap.mlc_mbc_bypass);
                mlc.set_mbc_cf_lo(snap.mlc_mbc_cf_lo);
                mlc.set_mbc_cf_hi(snap.mlc_mbc_cf_hi);
                mlc.set_mbc_drive_lo(snap.mlc_mbc_drive_lo);
                mlc.set_mbc_drive_mid(snap.mlc_mbc_drive_mid);
                mlc.set_mbc_drive_hi(snap.mlc_mbc_drive_hi);
                mlc.set_adaa_order(adaa_order);
                mlc.process_block(buf_l.as_mut_ptr(), buf_l.len());
            }
            if mono_input {
                // The right instance idles while the source is mono; its state
                // goes stale, so the first stereo block after a switch carries
                // one small transient — accepted in exchange for halving the
                // steady-state cost.
                buf_r.copy_from_slice(buf_l);
            } else if let Some(mlc) = mlc_r {
                mlc.set_gain(snap.mlc_gain);
                mlc.set_master(snap.mlc_master);
                mlc.set_bass(snap.mlc_bass);
                mlc.set_middle(snap.mlc_middle);
                mlc.set_treble(snap.mlc_treble);
                mlc.set_presence(snap.mlc_presence);
                mlc.set_depth(snap.mlc_depth);
                mlc.set_gate(snap.mlc_gate);
                mlc.set_bright(bright);
                mlc.set_m45(snap.mlc_m45);
                mlc.set_warclaw(snap.mlc_warclaw);
                mlc.set_feedback(feedback);
                mlc.set_gate_pos(gate_pos);
                mlc.set_clip_type1(clip_type1);
                mlc.set_clip_type2(clip_type2);
                mlc.set_clip_type3(clip_type3);
                mlc.set_tight(tight);
                mlc.set_asymmetry_enable(asymmetry_enable);
                mlc.set_asymmetry(snap.mlc_asymmetry);
                mlc.set_preshape(preshape);
                mlc.set_preshape_tight(snap.mlc_preshape_tight);
                mlc.set_preshape_bite(snap.mlc_preshape_bite);
                mlc.set_clean_blend(snap.mlc_clean_blend);
                mlc.set_sag(snap.mlc_sag);
                mlc.set_h2(snap.mlc_h2);
                mlc.set_h3(snap.mlc_h3);
                mlc.set_h4(snap.mlc_h4);
                mlc.set_ts_model(ts_model);
                mlc.set_tube_model(tube_model);
                mlc.set_tube_drive(snap.mlc_tube_drive);
                mlc.set_tube_bypass(snap.mlc_tube_bypass);
                mlc.set_nfb_presence(snap.mlc_nfb_presence);
                mlc.set_nfb_resonance(snap.mlc_nfb_resonance);
                mlc.set_nfb_depth(snap.mlc_nfb_depth);
                mlc.set_nfb_bypass(snap.mlc_nfb_bypass);
                mlc.set_mbc_bypass(snap.mlc_mbc_bypass);
                mlc.set_mbc_cf_lo(snap.mlc_mbc_cf_lo);
                mlc.set_mbc_cf_hi(snap.mlc_mbc_cf_hi);
                mlc.set_mbc_drive_lo(snap.mlc_mbc_drive_lo);
                mlc.set_mbc_drive_mid(snap.mlc_mbc_drive_mid);
                mlc.set_mbc_drive_hi(snap.mlc_mbc_drive_hi);
                mlc.set_adaa_order(adaa_order);
                mlc.process_block(buf_r.as_mut_ptr(), buf_r.len());
            }
        }
    }
}

/// The full linear + nonlinear signal chain shared by every standalone input
/// format arm (F32/I32/I16, T15). Owns every DSP processor and every scratch
/// buffer up front so `process()` never allocates (T16).
pub struct StandalonePipeline {
    faust_l: Option<FaustProcessor>,
    faust_r: Option<FaustProcessor>,
    neural_l: NeuralProcessor,
    neural_r: NeuralProcessor,
    mlc_l: Option<MlcZeroVProcessor>,
    mlc_r: Option<MlcZeroVProcessor>,
    pre_eq_l: FFTConvolver<f32>,
    pre_eq_r: FFTConvolver<f32>,
    cabinet_engine: CabinetEngine,
    limiter: PeakLimiter,
    temp_l: Vec<f32>,
    temp_r: Vec<f32>,
    crossfade_buf: [Vec<f32>; 2],
    previous_amp_model: AmpModel,
    crossfade_sample: usize,
    sample_rate: f32,
    #[cfg(feature = "lab")]
    lab_pipeline: Option<PipelineManager>,
}

impl StandalonePipeline {
    /// Builds every processor and pre-allocates every scratch buffer used by
    /// `process()`, sized to `max_block` frames. `pre_eq_ir`, when non-empty,
    /// is the fixed tone-stack impulse response (mono, already decoded to
    /// f32); passing it in keeps embedded-asset decoding out of this
    /// (reusable, testable) module. `cabinet_mailbox` is the lock-free
    /// channel the UI thread uses to hot-swap cabinet IRs; the caller keeps
    /// its own clone to publish into.
    pub fn new(
        sample_rate: f32,
        max_block: usize,
        pre_eq_ir: &[f32],
        cabinet_mailbox: Arc<CabinetMailbox>,
    ) -> Self {
        let mut faust_l = FaustProcessor::new();
        let mut faust_r = FaustProcessor::new();
        if let Some(f) = &mut faust_l {
            f.init(sample_rate);
        }
        if let Some(f) = &mut faust_r {
            f.init(sample_rate);
        }

        let mut neural_l = NeuralProcessor::new();
        let mut neural_r = NeuralProcessor::new();
        neural_l.init(sample_rate);
        neural_r.init(sample_rate);
        neural_l.set_drive(2.0);
        neural_l.set_output_gain(0.5);
        neural_r.set_drive(2.0);
        neural_r.set_output_gain(0.5);

        let mut mlc_l = MlcZeroVProcessor::new();
        let mut mlc_r = MlcZeroVProcessor::new();
        if let Some(mlc) = &mut mlc_l {
            mlc.init(sample_rate);
        }
        if let Some(mlc) = &mut mlc_r {
            mlc.init(sample_rate);
        }

        let limiter = PeakLimiter::new(-1.0, 50.0, sample_rate);

        let mut pre_eq_l = FFTConvolver::default();
        let mut pre_eq_r = FFTConvolver::default();
        if !pre_eq_ir.is_empty() {
            let _ = pre_eq_l.init(max_block, pre_eq_ir);
            let _ = pre_eq_r.init(max_block, pre_eq_ir);
        }

        let mut cabinet_engine = CabinetEngine::with_mailbox(cabinet_mailbox);
        cabinet_engine.set_sample_rate(sample_rate);

        Self {
            faust_l,
            faust_r,
            neural_l,
            neural_r,
            mlc_l,
            mlc_r,
            pre_eq_l,
            pre_eq_r,
            cabinet_engine,
            limiter,
            temp_l: vec![0.0; max_block],
            temp_r: vec![0.0; max_block],
            crossfade_buf: [vec![0.0; max_block], vec![0.0; max_block]],
            previous_amp_model: AmpModel::Neural,
            crossfade_sample: CROSSFADE_LEN,
            sample_rate,
            #[cfg(feature = "lab")]
            lab_pipeline: PipelineManager::from_categories(&crate::lab::default_categories()).ok(),
        }
    }

    /// Frames this pipeline's scratch buffers were sized for. `process()`
    /// silently clamps to this length, matching the pre-extraction
    /// `max_len = num_frames.min(buffer_size)` behavior.
    pub fn max_block(&self) -> usize {
        self.temp_l.len()
    }

    /// Runs the full chain in place: Faust 3-band EQ, pre-EQ (tone-stack)
    /// convolution, the selected amp model (with equal-power crossfade on
    /// model switch), cabinet IR convolution, master + neural volume gain,
    /// the lab pipeline (if enabled), the brickwall limiter, and NaN/Inf
    /// sanitization. `buf_l`/`buf_r` carry dry input on entry and wet output
    /// on exit. No allocation occurs on this path.
    pub fn process(&mut self, buf_l: &mut [f32], buf_r: &mut [f32], snap: &AudioSnapshot) {
        let max_len = buf_l
            .len()
            .min(buf_r.len())
            .min(self.temp_l.len())
            .min(self.temp_r.len());
        let buf_l = &mut buf_l[..max_len];
        let buf_r = &mut buf_r[..max_len];

        if !snap.bypass {
            // A) Faust EQ (processa os dois canais num único array de ponteiros - se a API local_faust aguentar)
            // A nossa interface Faust process_block aplica a um array plano in-place
            if snap.eq_active {
                if let Some(f) = &mut self.faust_l {
                    f.set_eq_params(
                        snap.eq_low_freq,
                        snap.eq_low_gain,
                        snap.eq_low_q,
                        snap.eq_mid_freq,
                        snap.eq_mid_gain,
                        snap.eq_mid_q,
                        snap.eq_high_freq,
                        snap.eq_high_gain,
                        snap.eq_high_q,
                    );
                    f.set_eq_tanh_bypass(snap.eq_tanh_bypass);
                    f.process_block(buf_l.as_mut_ptr(), max_len);
                }
                if let Some(f) = &mut self.faust_r {
                    f.set_eq_params(
                        snap.eq_low_freq,
                        snap.eq_low_gain,
                        snap.eq_low_q,
                        snap.eq_mid_freq,
                        snap.eq_mid_gain,
                        snap.eq_mid_q,
                        snap.eq_high_freq,
                        snap.eq_high_gain,
                        snap.eq_high_q,
                    );
                    f.set_eq_tanh_bypass(snap.eq_tanh_bypass);
                    f.process_block(buf_r.as_mut_ptr(), max_len);
                }
            }

            // B) Wiener-Hammerstein Gray-Box
            // ESTÁGIO 1: PRÉ-EQUALIZAÇÃO LTI (Tone Stack) // Somente se válido
            // Precisamos proteger contra uninit calls de arrays zerados. Se array não for vazio, tem certeza q inicializou.
            // Porém, falhas no convolver geram f32 zerados.
            self.temp_l[..max_len].copy_from_slice(&buf_l[..max_len]);
            self.temp_r[..max_len].copy_from_slice(&buf_r[..max_len]);
            if self
                .pre_eq_l
                .process(&self.temp_l[..max_len], &mut buf_l[..max_len])
                .is_err()
            {
                buf_l[..max_len].copy_from_slice(&self.temp_l[..max_len]);
            }
            if self
                .pre_eq_r
                .process(&self.temp_r[..max_len], &mut buf_r[..max_len])
                .is_err()
            {
                buf_r[..max_len].copy_from_slice(&self.temp_r[..max_len]);
            }

            // ESTÁGIO 3: Modelo de amp selecionado
            if snap.amp_model != self.previous_amp_model && self.crossfade_sample >= CROSSFADE_LEN {
                self.crossfade_sample = 0;
            } else if snap.amp_model == self.previous_amp_model
                && self.crossfade_sample < CROSSFADE_LEN
            {
                self.crossfade_sample = CROSSFADE_LEN;
            }
            let crossfade_start = self.crossfade_sample;
            let crossfading =
                snap.amp_model != self.previous_amp_model && crossfade_start < CROSSFADE_LEN;

            if crossfading {
                self.temp_l[..max_len].copy_from_slice(&buf_l[..max_len]);
                self.temp_r[..max_len].copy_from_slice(&buf_r[..max_len]);

                process_standalone_amp(
                    self.previous_amp_model,
                    snap,
                    &mut self.neural_l,
                    &mut self.neural_r,
                    &mut self.mlc_l,
                    &mut self.mlc_r,
                    &mut buf_l[..max_len],
                    &mut buf_r[..max_len],
                );
                self.crossfade_buf[0][..max_len].copy_from_slice(&buf_l[..max_len]);
                self.crossfade_buf[1][..max_len].copy_from_slice(&buf_r[..max_len]);

                buf_l[..max_len].copy_from_slice(&self.temp_l[..max_len]);
                buf_r[..max_len].copy_from_slice(&self.temp_r[..max_len]);

                process_standalone_amp(
                    snap.amp_model,
                    snap,
                    &mut self.neural_l,
                    &mut self.neural_r,
                    &mut self.mlc_l,
                    &mut self.mlc_r,
                    &mut buf_l[..max_len],
                    &mut buf_r[..max_len],
                );

                for i in 0..max_len {
                    let fade_pos = (crossfade_start + i).min(CROSSFADE_LEN);
                    let t = fade_pos as f32 / CROSSFADE_LEN as f32;
                    let old_l = self.crossfade_buf[0][i];
                    let old_r = self.crossfade_buf[1][i];
                    buf_l[i] = old_l + (buf_l[i] - old_l) * t;
                    buf_r[i] = old_r + (buf_r[i] - old_r) * t;
                }

                self.crossfade_sample = (crossfade_start + max_len).min(CROSSFADE_LEN);
                if self.crossfade_sample >= CROSSFADE_LEN {
                    self.previous_amp_model = snap.amp_model;
                }
            } else {
                process_standalone_amp(
                    snap.amp_model,
                    snap,
                    &mut self.neural_l,
                    &mut self.neural_r,
                    &mut self.mlc_l,
                    &mut self.mlc_r,
                    &mut buf_l[..max_len],
                    &mut buf_r[..max_len],
                );
            }

            // ESTÁGIO 4: GABINETE (Cabinet IR gerenciado — troca em runtime via ArcSwap)
            self.cabinet_engine.process(
                &mut buf_l[..max_len],
                &mut buf_r[..max_len],
                &mut self.temp_l[..max_len],
                &mut self.temp_r[..max_len],
                snap.cabinet_bypass,
                snap.cabinet_level,
                snap.cabinet_mix,
            );

            // ESTÁGIO 5: Ganho Master
            for l in &mut buf_l[..max_len] {
                *l *= snap.gain;
            }
            for r in &mut buf_r[..max_len] {
                *r *= snap.gain;
            }

            // ESTÁGIO 6: Volume Master Neural (após processamento)
            if snap.amp_model == AmpModel::Neural && snap.neural_active {
                for l in &mut buf_l[..max_len] {
                    *l *= snap.neural_amp_volume;
                }
                for r in &mut buf_r[..max_len] {
                    *r *= snap.neural_amp_volume;
                }
            }

            #[cfg(feature = "lab")]
            if let Some(pipeline) = self.lab_pipeline.as_mut() {
                pipeline.process_block(buf_l.as_mut_ptr(), max_len);
                pipeline.process_block(buf_r.as_mut_ptr(), max_len);
            }

            // ESTÁGIO: Brickwall Limiter (após master
            // gain, antes do saneamento NaN).
            if snap.limiter_enable {
                self.limiter.set_params(
                    snap.limiter_ceiling,
                    snap.limiter_release,
                    self.sample_rate,
                );
                for l in &mut buf_l[..max_len] {
                    *l = self.limiter.process(*l);
                }
                for r in &mut buf_r[..max_len] {
                    *r = self.limiter.process(*r);
                }
            }
        }

        // ESTÁGIO 7: Saneamento de NaN/Infinito (SEMPRE executado, mesmo em bypass)
        for l in &mut buf_l[..max_len] {
            if l.is_nan() || l.is_infinite() {
                *l = 0.0;
            }
        }
        for r in &mut buf_r[..max_len] {
            if r.is_nan() || r.is_infinite() {
                *r = 0.0;
            }
        }
    }
}

/// Deinterleaves one raw input block of native samples (F32/I32/I16 — any
/// format `convert` can turn into f32), runs it through `pipeline` in
/// `pipeline.max_block()`-sized chunks, and calls `on_chunk` once per chunk
/// with the processed L/R slices.
///
/// This is the single call site `pipeline.process()` is reached from for
/// every standalone input format arm (T15), so adding a new native format
/// only means adding a `convert` function — never a second copy of the
/// chunking loop or the DSP call. Decoupled from cpal/rtrb on purpose so it
/// can be exercised directly in tests without a live audio device.
#[allow(clippy::too_many_arguments)]
pub fn process_interleaved_block<T: Copy>(
    pipeline: &mut StandalonePipeline,
    data: &[T],
    channels: usize,
    l_idx: usize,
    r_idx: usize,
    convert: impl Fn(T) -> f32,
    buf_l: &mut [f32],
    buf_r: &mut [f32],
    snap: &AudioSnapshot,
    mut on_chunk: impl FnMut(&[f32], &[f32]),
) {
    if channels == 0 {
        return;
    }
    let num_frames = data.len() / channels;
    let cap = buf_l.len().min(buf_r.len());
    if cap == 0 {
        return;
    }

    for chunk_start in (0..num_frames).step_by(cap) {
        let max_len = (num_frames - chunk_start).min(cap);

        for (i, frame) in data[chunk_start * channels..]
            .chunks(channels)
            .enumerate()
            .take(max_len)
        {
            buf_l[i] = frame.get(l_idx).copied().map(&convert).unwrap_or(0.0);
            buf_r[i] = frame.get(r_idx).copied().map(&convert).unwrap_or(0.0);
        }

        pipeline.process(&mut buf_l[..max_len], &mut buf_r[..max_len], snap);
        on_chunk(&buf_l[..max_len], &buf_r[..max_len]);
    }
}
