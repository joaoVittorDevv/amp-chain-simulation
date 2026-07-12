//! Diagnostic harness for the MLC ZERO V amp model (crackling report).
//!
//! Measures the real-time factor of one `MlcZeroVProcessor` instance at the
//! rates/block sizes the standalone actually runs, and checks the output for
//! NaN/Inf (which the pipeline sanitizer would turn into clicks). Run with:
//!
//! ```text
//! cargo test --release --test mlc_diagnosis -- --nocapture
//! ```

use distortion::bridge::mlc_zero_v::MlcZeroVProcessor;
use distortion::bridge::ExternalProcessor;
use std::f32::consts::TAU;
use std::time::Instant;

const SAMPLE_RATE: f32 = 48_000.0;

fn fill_sine(block: &mut [f32], start_frame: usize, amplitude: f32) {
    for (i, s) in block.iter_mut().enumerate() {
        let phase = TAU * 220.0 * (start_frame + i) as f32 / SAMPLE_RATE;
        *s = amplitude * phase.sin();
    }
}

fn measure(frames_per_block: usize, seconds: f32, amplitude: f32) -> (f64, bool) {
    let mut p = MlcZeroVProcessor::new().expect("mlc processor");
    p.init(SAMPLE_RATE);

    let mut block = vec![0.0f32; frames_per_block];
    let blocks = ((SAMPLE_RATE * seconds) as usize) / frames_per_block;

    // Warm-up: let smoothers settle and pull code/data into cache.
    for b in 0..32 {
        fill_sine(&mut block, b * frames_per_block, amplitude);
        p.process_block(block.as_mut_ptr(), frames_per_block);
    }

    let mut saw_non_finite = false;
    let t0 = Instant::now();
    for b in 0..blocks {
        fill_sine(&mut block, b * frames_per_block, amplitude);
        p.process_block(block.as_mut_ptr(), frames_per_block);
        if block.iter().any(|s| !s.is_finite()) {
            saw_non_finite = true;
        }
    }
    let elapsed = t0.elapsed().as_secs_f64();
    let audio_seconds = (blocks * frames_per_block) as f64 / SAMPLE_RATE as f64;

    // Real-time factor: fraction of the audio budget one instance consumes.
    // The standalone runs TWO instances (L and R), so > 0.5 already glitches.
    (elapsed / audio_seconds, saw_non_finite)
}

#[test]
fn mlc_realtime_factor_and_stability() {
    println!("--- MLC ZERO V @ {SAMPLE_RATE} Hz, defaults, hot signal ---");
    for frames in [128usize, 480, 1024] {
        let (rtf, non_finite) = measure(frames, 3.0, 0.5);
        println!(
            "block {frames:>5}: RTF={rtf:.3} (x2 instancias = {:.3}) non_finite={non_finite}",
            rtf * 2.0
        );
    }

    // Quiet input probes denormal behavior in the filter/feedback tails.
    let (rtf_quiet, non_finite_quiet) = measure(480, 3.0, 1.0e-8);
    println!("block   480 (entrada quase silencio): RTF={rtf_quiet:.3} non_finite={non_finite_quiet}");
}
