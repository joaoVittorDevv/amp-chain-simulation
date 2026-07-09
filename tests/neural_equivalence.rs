#![cfg(have_mojo)]

//! Numeric equivalence between the Mojo FFI backend and its Rust fallback
//! (CROSS-04). Only compiled/run when `build.rs` found and linked the Mojo
//! toolchain; on Windows (or any host without Mojo) this file is a no-op.

use distortion::bridge::mojo::MojoProcessor;
use distortion::bridge::neural_rust::RustNeuralProcessor;
use distortion::bridge::ExternalProcessor;

const SAMPLE_RATE: f32 = 48_000.0;
const TOLERANCE: f32 = 1e-6;

fn ramp_signal() -> Vec<f32> {
    let steps = 161;
    (0..steps)
        .map(|i| -2.0 + (4.0 * i as f32) / (steps - 1) as f32)
        .collect()
}

/// Deterministic xorshift32 PRNG — fixed seed, no external `rand` dependency.
fn white_noise_signal() -> Vec<f32> {
    let mut state: u32 = 0x1234_5678;
    (0..2048)
        .map(|_| {
            state ^= state << 13;
            state ^= state >> 17;
            state ^= state << 5;
            // Map to [-1.0, 1.0].
            (state as f32 / u32::MAX as f32) * 2.0 - 1.0
        })
        .collect()
}

fn dc_signal(value: f32) -> Vec<f32> {
    vec![value; 64]
}

fn clamp_boundary_signal() -> Vec<f32> {
    vec![4.0, -4.0, 4.0, -4.0, 4.0, -4.0, 4.0, -4.0]
}

fn test_signals() -> Vec<(&'static str, Vec<f32>)> {
    vec![
        ("ramp", ramp_signal()),
        ("white_noise", white_noise_signal()),
        ("dc_positive", dc_signal(1.0)),
        ("dc_negative", dc_signal(-1.0)),
        ("clamp_boundary", clamp_boundary_signal()),
    ]
}

#[test]
fn mojo_and_rust_backends_agree_within_tolerance() {
    let drives = [0.0_f32, 0.5, 1.0, 2.0, 8.0];
    let output_gains = [0.0_f32, 0.5, 1.0, 2.0];

    for &drive in &drives {
        for &output_gain in &output_gains {
            for (signal_name, signal) in test_signals() {
                let mut mojo = MojoProcessor::new();
                mojo.init(SAMPLE_RATE);
                mojo.set_drive(drive);
                mojo.set_output_gain(output_gain);

                let mut rust = RustNeuralProcessor::new();
                rust.init(SAMPLE_RATE);
                rust.set_drive(drive);
                rust.set_output_gain(output_gain);

                let mut mojo_buf = signal.clone();
                let mut rust_buf = signal.clone();

                mojo.process_block(mojo_buf.as_mut_ptr(), mojo_buf.len());
                rust.process_block(rust_buf.as_mut_ptr(), rust_buf.len());

                for (i, (&m, &r)) in mojo_buf.iter().zip(rust_buf.iter()).enumerate() {
                    assert!(
                        m.is_finite(),
                        "mojo backend produced non-finite output at signal={signal_name} drive={drive} output_gain={output_gain} idx={i}: {m}"
                    );
                    assert!(
                        r.is_finite(),
                        "rust backend produced non-finite output at signal={signal_name} drive={drive} output_gain={output_gain} idx={i}: {r}"
                    );

                    let diff = (m - r).abs();
                    assert!(
                        diff <= TOLERANCE,
                        "backends diverged at signal={signal_name} drive={drive} output_gain={output_gain} idx={i}: mojo={m} rust={r} diff={diff}"
                    );
                }
            }
        }
    }
}
