//! T15: the same signal delivered as F32, I32, or I16 interleaved input
//! must produce matching pipeline output (within each format's own
//! quantization error), and all three formats must go through the exact
//! same `process_interleaved_block` call site — this test is what would
//! catch a copy-paste drift between the three `match` arms in
//! `src/bin/standalone.rs` (e.g. L/R swapped in one arm, or a different
//! chunking cap).

use distortion::core::cabinet::CabinetMailbox;
use distortion::core::dsp::{
    process_interleaved_block, sample_convert, AudioSnapshot, StandalonePipeline,
};
use distortion::core::state::plugin_params::AmpModel;

const SAMPLE_RATE: f32 = 48_000.0;
const MAX_BLOCK: usize = 256;
const CHANNELS: usize = 2;

fn new_pipeline() -> StandalonePipeline {
    // Identity pre-EQ IR (see tests/pipeline_golden.rs for why `&[]` would
    // silently zero the signal instead of passing it through).
    StandalonePipeline::new(SAMPLE_RATE, MAX_BLOCK, &[1.0], CabinetMailbox::new_arc())
}

fn linear_snapshot(gain: f32) -> AudioSnapshot {
    let mut snap = AudioSnapshot::default();
    snap.eq_active = false;
    snap.limiter_enable = false;
    snap.cabinet_bypass = true;
    snap.amp_model = AmpModel::Neural;
    snap.neural_active = false; // no-op amp stage: keeps the chain linear
    snap.gain = gain;
    snap
}

/// Distinct L/R deterministic signals so a channel swap would be caught.
fn interleaved_f32(frames: usize) -> Vec<f32> {
    let mut out = Vec::with_capacity(frames * CHANNELS);
    for i in 0..frames {
        let t = i as f32;
        let l = 0.6 * (t * 0.041).sin();
        let r = 0.3 * (t * 0.019).cos();
        out.push(l);
        out.push(r);
    }
    out
}

fn run_f32(data: &[f32], snap: &AudioSnapshot) -> (Vec<f32>, Vec<f32>) {
    let mut pipeline = new_pipeline();
    let mut buf_l = vec![0.0; MAX_BLOCK];
    let mut buf_r = vec![0.0; MAX_BLOCK];
    let mut out_l = Vec::new();
    let mut out_r = Vec::new();
    process_interleaved_block(
        &mut pipeline,
        data,
        CHANNELS,
        0,
        1,
        |s| s,
        &mut buf_l,
        &mut buf_r,
        snap,
        |l, r| {
            out_l.extend_from_slice(l);
            out_r.extend_from_slice(r);
        },
    );
    (out_l, out_r)
}

fn run_i32(data: &[i32], snap: &AudioSnapshot) -> (Vec<f32>, Vec<f32>) {
    let mut pipeline = new_pipeline();
    let mut buf_l = vec![0.0; MAX_BLOCK];
    let mut buf_r = vec![0.0; MAX_BLOCK];
    let mut out_l = Vec::new();
    let mut out_r = Vec::new();
    process_interleaved_block(
        &mut pipeline,
        data,
        CHANNELS,
        0,
        1,
        sample_convert::i32_to_f32,
        &mut buf_l,
        &mut buf_r,
        snap,
        |l, r| {
            out_l.extend_from_slice(l);
            out_r.extend_from_slice(r);
        },
    );
    (out_l, out_r)
}

fn run_i16(data: &[i16], snap: &AudioSnapshot) -> (Vec<f32>, Vec<f32>) {
    let mut pipeline = new_pipeline();
    let mut buf_l = vec![0.0; MAX_BLOCK];
    let mut buf_r = vec![0.0; MAX_BLOCK];
    let mut out_l = Vec::new();
    let mut out_r = Vec::new();
    process_interleaved_block(
        &mut pipeline,
        data,
        CHANNELS,
        0,
        1,
        sample_convert::i16_to_f32,
        &mut buf_l,
        &mut buf_r,
        snap,
        |l, r| {
            out_l.extend_from_slice(l);
            out_r.extend_from_slice(r);
        },
    );
    (out_l, out_r)
}

#[test]
fn format_equivalence_f32_i32_i16_agree_within_quantization_error() {
    // 700 frames spans multiple MAX_BLOCK-sized chunks, exercising the same
    // chunking loop the callback uses for blocks larger than the pipeline's
    // pre-allocated capacity (T14).
    let frames = 700;
    let snap = linear_snapshot(0.8);

    let f32_data = interleaved_f32(frames);
    let i32_data: Vec<i32> = f32_data.iter().map(|&s| sample_convert::f32_to_i32(s)).collect();
    let i16_data: Vec<i16> = f32_data.iter().map(|&s| sample_convert::f32_to_i16(s)).collect();

    let (f32_l, f32_r) = run_f32(&f32_data, &snap);
    let (i32_l, i32_r) = run_i32(&i32_data, &snap);
    let (i16_l, i16_r) = run_i16(&i16_data, &snap);

    assert_eq!(f32_l.len(), frames);
    assert_eq!(i32_l.len(), frames);
    assert_eq!(i16_l.len(), frames);

    // I32 quantization step is ~2^-31; any DSP-introduced difference should
    // be negligible relative to f32 rounding itself.
    for i in 0..frames {
        assert!(
            (f32_l[i] - i32_l[i]).abs() < 5e-6,
            "L[{i}]: f32={} i32={}",
            f32_l[i],
            i32_l[i]
        );
        assert!(
            (f32_r[i] - i32_r[i]).abs() < 5e-6,
            "R[{i}]: f32={} i32={}",
            f32_r[i],
            i32_r[i]
        );
    }

    // I16 quantization step is ~2^-15 ~= 3.05e-5; allow a small multiple of
    // that to cover the master-gain multiply.
    for i in 0..frames {
        assert!(
            (f32_l[i] - i16_l[i]).abs() < 2e-4,
            "L[{i}]: f32={} i16={}",
            f32_l[i],
            i16_l[i]
        );
        assert!(
            (f32_r[i] - i16_r[i]).abs() < 2e-4,
            "R[{i}]: f32={} i16={}",
            f32_r[i],
            i16_r[i]
        );
    }
}

#[test]
fn format_equivalence_channels_are_not_swapped_in_any_format() {
    // L and R carry visibly different amplitudes; a channel-index bug in any
    // of the three `match` arms would show up as L/R output matching the
    // *wrong* input channel.
    let frames = 64;
    let snap = linear_snapshot(1.0);
    let mut data = Vec::with_capacity(frames * CHANNELS);
    for i in 0..frames {
        data.push(0.9); // L: constant loud
        data.push(0.1 * (i as f32).sin()); // R: quiet, varying
    }

    let i32_data: Vec<i32> = data.iter().map(|&s| sample_convert::f32_to_i32(s)).collect();
    let i16_data: Vec<i16> = data.iter().map(|&s| sample_convert::f32_to_i16(s)).collect();

    let (f32_l, f32_r) = run_f32(&data, &snap);
    let (i32_l, i32_r) = run_i32(&i32_data, &snap);
    let (i16_l, i16_r) = run_i16(&i16_data, &snap);

    for l in [&f32_l, &i32_l, &i16_l] {
        assert!(l.iter().all(|&s| (s - 0.9).abs() < 2e-4), "L channel drifted: {l:?}");
    }
    for r in [&f32_r, &i32_r, &i16_r] {
        assert!(r.iter().all(|&s| s.abs() < 0.15), "R channel drifted: {r:?}");
    }
}

#[test]
fn format_equivalence_zero_length_block_produces_no_output() {
    let snap = linear_snapshot(1.0);
    let empty_f32: [f32; 0] = [];
    let empty_i32: [i32; 0] = [];
    let empty_i16: [i16; 0] = [];

    let (l, r) = run_f32(&empty_f32, &snap);
    assert!(l.is_empty() && r.is_empty());
    let (l, r) = run_i32(&empty_i32, &snap);
    assert!(l.is_empty() && r.is_empty());
    let (l, r) = run_i16(&empty_i16, &snap);
    assert!(l.is_empty() && r.is_empty());
}
