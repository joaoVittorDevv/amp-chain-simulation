//! T12 golden-vector coverage for `StandalonePipeline`.
//!
//! The pipeline was extracted mechanically from a ~400-line closure that
//! used to live inline in the standalone binary's cpal input callback
//! (`src/bin/standalone.rs` on `main`, before this branch). That closure
//! wasn't independently callable, so instead of diffing raw audio against a
//! captured `main` run, these tests pin down the documented behavior of
//! each stage analytically (bypass is identity except NaN/Inf sanitization,
//! master gain is a linear per-sample multiply, the limiter's instant
//! attack guarantees `|out| <= ceiling` on every sample, a crossfade
//! starts at `t = 0` so the very first post-switch sample must equal the
//! outgoing model's own output, zero/partial blocks must not panic or read
//! past the given length) and checks the extracted `process()` against
//! those invariants. A regression in the hand-transcribed orchestration
//! (buffer swap order, off-by-one in the crossfade index, wrong stage
//! order) breaks one of these, which is what "golden" is standing in for
//! here given no offline harness existed on `main` to capture reference
//! output from.

use distortion::core::cabinet::CabinetMailbox;
use distortion::core::dsp::{AudioSnapshot, StandalonePipeline};
use distortion::core::state::plugin_params::AmpModel;

const SAMPLE_RATE: f32 = 48_000.0;
const MAX_BLOCK: usize = 512;

fn new_pipeline() -> StandalonePipeline {
    // A single-tap [1.0] "identity" impulse response initializes the pre-EQ
    // FFTConvolver as a passthrough. An empty IR is NOT equivalent: an
    // uninitialized FFTConvolver's `process()` returns `Ok(())` and *zeroes*
    // its output (see fft-convolver's `active_seg_count == 0` branch) rather
    // than erroring, so passing `&[]` here would silently zero every test
    // signal before it reached the stage under test.
    StandalonePipeline::new(SAMPLE_RATE, MAX_BLOCK, &[1.0], CabinetMailbox::new_arc())
}

/// Deterministic pseudo-random test signal, no RNG crate dependency.
fn test_signal(len: usize, seed: f32) -> Vec<f32> {
    (0..len)
        .map(|i| {
            let t = i as f32;
            0.4 * (t * 0.037 + seed).sin() + 0.15 * (t * 0.011 + seed * 0.7).cos()
        })
        .collect()
}

fn minimal_snapshot() -> AudioSnapshot {
    let mut snap = AudioSnapshot::default();
    // Isolate the stage under test: EQ off, no cabinet IR loaded (mailbox is
    // empty so the cabinet stage is already a no-op regardless), limiter off.
    snap.eq_active = false;
    snap.limiter_enable = false;
    snap.cabinet_bypass = true;
    snap
}

#[test]
fn pipeline_golden_zero_block_does_not_panic() {
    let mut pipeline = new_pipeline();
    let snap = minimal_snapshot();
    let mut l: [f32; 0] = [];
    let mut r: [f32; 0] = [];
    pipeline.process(&mut l, &mut r, &snap);
}

#[test]
fn pipeline_golden_partial_block_processes_only_given_length() {
    let mut pipeline = new_pipeline();
    let snap = minimal_snapshot();
    let mut l = test_signal(37, 1.0);
    let mut r = test_signal(37, 2.0);
    // Must not panic reading/writing past a block far shorter than MAX_BLOCK,
    // and must return a full 37-sample result.
    pipeline.process(&mut l, &mut r, &snap);
    assert_eq!(l.len(), 37);
    assert_eq!(r.len(), 37);
    assert!(l.iter().all(|s| s.is_finite()));
    assert!(r.iter().all(|s| s.is_finite()));
}

#[test]
fn pipeline_golden_bypass_is_identity_except_nan_sanitization() {
    let mut pipeline = new_pipeline();
    let mut snap = minimal_snapshot();
    snap.bypass = true;

    let dry_l = test_signal(256, 3.0);
    let dry_r = test_signal(256, 4.0);
    let mut l = dry_l.clone();
    let mut r = dry_r.clone();
    pipeline.process(&mut l, &mut r, &snap);

    assert_eq!(l, dry_l, "bypass must not alter L samples");
    assert_eq!(r, dry_r, "bypass must not alter R samples");
}

#[test]
fn pipeline_golden_bypass_still_sanitizes_nan_and_inf() {
    let mut pipeline = new_pipeline();
    let mut snap = minimal_snapshot();
    snap.bypass = true;

    let mut l = vec![f32::NAN, f32::INFINITY, f32::NEG_INFINITY, 0.5];
    let mut r = vec![0.25, f32::NAN, 1.0, f32::INFINITY];
    pipeline.process(&mut l, &mut r, &snap);

    assert_eq!(l, vec![0.0, 0.0, 0.0, 0.5]);
    assert_eq!(r, vec![0.25, 0.0, 1.0, 0.0]);
}

#[test]
fn pipeline_golden_master_gain_scales_linearly_with_neural_inactive() {
    // With the Neural model selected but inactive, the amp stage is a no-op,
    // so with EQ/cabinet/limiter also disabled the only stage that touches
    // amplitude is "ESTÁGIO 5: Ganho Master" (a per-sample `*= snap.gain`).
    let mut pipeline_unity = new_pipeline();
    let mut pipeline_half = new_pipeline();
    let mut snap_unity = minimal_snapshot();
    snap_unity.amp_model = AmpModel::Neural;
    snap_unity.neural_active = false;
    snap_unity.gain = 1.0;
    let mut snap_half = snap_unity;
    snap_half.gain = 0.5;

    let dry_l = test_signal(200, 5.0);
    let dry_r = test_signal(200, 6.0);

    let mut l_unity = dry_l.clone();
    let mut r_unity = dry_r.clone();
    pipeline_unity.process(&mut l_unity, &mut r_unity, &snap_unity);

    let mut l_half = dry_l.clone();
    let mut r_half = dry_r.clone();
    pipeline_half.process(&mut l_half, &mut r_half, &snap_half);

    for i in 0..200 {
        assert!(
            (l_half[i] - l_unity[i] * 0.5).abs() < 1e-5,
            "sample {i}: half={} unity*0.5={}",
            l_half[i],
            l_unity[i] * 0.5
        );
        assert!((r_half[i] - r_unity[i] * 0.5).abs() < 1e-5);
    }
}

#[test]
fn pipeline_golden_neural_model_is_deterministic() {
    let mut pipeline_a = new_pipeline();
    let mut pipeline_b = new_pipeline();
    let mut snap = minimal_snapshot();
    snap.amp_model = AmpModel::Neural;
    snap.neural_active = true;
    snap.neural_drive = 3.0;
    snap.neural_output_gain = 0.8;

    let dry_l = test_signal(300, 7.0);
    let dry_r = test_signal(300, 8.0);

    let mut l_a = dry_l.clone();
    let mut r_a = dry_r.clone();
    pipeline_a.process(&mut l_a, &mut r_a, &snap);

    let mut l_b = dry_l.clone();
    let mut r_b = dry_r.clone();
    pipeline_b.process(&mut l_b, &mut r_b, &snap);

    assert_eq!(l_a, l_b, "same input+params must give bit-identical output");
    assert_eq!(r_a, r_b);
    assert!(l_a.iter().all(|s| s.is_finite()));
}

#[test]
fn pipeline_golden_mlc_model_is_deterministic() {
    let mut pipeline_a = new_pipeline();
    let mut pipeline_b = new_pipeline();
    let mut snap = minimal_snapshot();
    snap.amp_model = AmpModel::MlcZeroV;

    let dry_l = test_signal(300, 9.0);
    let dry_r = test_signal(300, 10.0);

    let mut l_a = dry_l.clone();
    let mut r_a = dry_r.clone();
    pipeline_a.process(&mut l_a, &mut r_a, &snap);

    let mut l_b = dry_l.clone();
    let mut r_b = dry_r.clone();
    pipeline_b.process(&mut l_b, &mut r_b, &snap);

    assert_eq!(l_a, l_b, "same input+params must give bit-identical output");
    assert_eq!(r_a, r_b);
    assert!(l_a.iter().all(|s| s.is_finite()));
}

#[test]
fn pipeline_golden_limiter_enforces_ceiling_every_sample() {
    let mut pipeline = new_pipeline();
    let mut snap = minimal_snapshot();
    snap.amp_model = AmpModel::Neural;
    snap.neural_active = false;
    snap.gain = 1.0;
    snap.limiter_enable = true;
    snap.limiter_ceiling = -1.0;
    snap.limiter_release = 50.0;

    // Loud signal, well above the -1 dB (~0.891) ceiling.
    let mut l: Vec<f32> = (0..256).map(|i| if i % 2 == 0 { 0.99 } else { -0.99 }).collect();
    let mut r = l.clone();
    pipeline.process(&mut l, &mut r, &snap);

    let ceiling = 10f32.powf(-1.0 / 20.0);
    for (i, &s) in l.iter().enumerate() {
        assert!(
            s.abs() <= ceiling + 1e-4,
            "sample {i} exceeded ceiling: {s} > {ceiling}"
        );
    }
}

#[test]
fn pipeline_golden_crossfade_starts_at_t_zero_on_model_switch() {
    // Per the extracted crossfade math: switching amp_model resets
    // crossfade_sample to 0, so at the very first post-switch sample,
    // fade_pos = 0 and t = 0 / CROSSFADE_LEN = 0, meaning
    // buf[0] = old_l + (new_l - old_l) * 0 == old_l exactly — the first
    // sample of a freshly-triggered crossfade must equal what the
    // *outgoing* model alone would have produced.
    let mut pipeline = new_pipeline();
    let mut reference = new_pipeline();

    let mut snap_neural = minimal_snapshot();
    snap_neural.amp_model = AmpModel::Neural;
    snap_neural.neural_active = true;
    snap_neural.neural_drive = 2.0;
    snap_neural.neural_output_gain = 0.7;

    let settle_l = test_signal(64, 20.0);
    let settle_r = test_signal(64, 21.0);
    // Run enough blocks on the Neural model so previous_amp_model == Neural
    // and crossfade_sample has settled back to CROSSFADE_LEN (not mid-fade).
    for _ in 0..10 {
        let mut l = settle_l.clone();
        let mut r = settle_r.clone();
        pipeline.process(&mut l, &mut r, &snap_neural);
        let mut l2 = settle_l.clone();
        let mut r2 = settle_r.clone();
        reference.process(&mut l2, &mut r2, &snap_neural);
    }

    // `reference` stays on Neural and gives us "what Neural alone produces"
    // for the exact same input block used for the switch below.
    let switch_input_l = test_signal(64, 22.0);
    let switch_input_r = test_signal(64, 23.0);
    let mut ref_l = switch_input_l.clone();
    let mut ref_r = switch_input_r.clone();
    reference.process(&mut ref_l, &mut ref_r, &snap_neural);

    // `pipeline` switches to MlcZeroV for one block shorter than
    // CROSSFADE_LEN (480), so it's mid-crossfade for the whole block.
    let mut snap_mlc = snap_neural;
    snap_mlc.amp_model = AmpModel::MlcZeroV;
    let mut l = switch_input_l.clone();
    let mut r = switch_input_r.clone();
    pipeline.process(&mut l, &mut r, &snap_mlc);

    assert!(
        (l[0] - ref_l[0]).abs() < 1e-5,
        "first crossfade sample must equal the outgoing model's own output: got {}, expected {}",
        l[0],
        ref_l[0]
    );
    assert!(
        (r[0] - ref_r[0]).abs() < 1e-5,
        "first crossfade sample (R) must equal the outgoing model's own output: got {}, expected {}",
        r[0],
        ref_r[0]
    );
}

#[test]
fn pipeline_golden_max_block_reports_preallocated_capacity() {
    let pipeline = new_pipeline();
    assert_eq!(pipeline.max_block(), MAX_BLOCK);
}
