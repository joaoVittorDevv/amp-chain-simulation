use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use distortion::bridge::{neural_rust::RustNeuralProcessor, ExternalProcessor};
use std::f32::consts::TAU;

const SAMPLE_RATE: f32 = 48_000.0;
const BUFFER_SIZES: [usize; 5] = [64, 128, 256, 512, 1024];

fn sine_input(frames: usize) -> Vec<f32> {
    (0..frames)
        .map(|frame| 0.25 * (TAU * 440.0 * frame as f32 / SAMPLE_RATE).sin())
        .collect()
}

fn benchmark_neural_drive(c: &mut Criterion) {
    assert!(
        std::env::var_os("DISTORTION_FORCE_RUST_NEURAL").is_some(),
        "run neural benchmarks with DISTORTION_FORCE_RUST_NEURAL=1"
    );

    let mut group = c.benchmark_group("neural_drive/rust/process");
    let mut processor = RustNeuralProcessor::new();
    processor.init(SAMPLE_RATE);
    processor.set_drive(2.0);
    processor.set_output_gain(0.5);

    for frames in BUFFER_SIZES {
        let input = sine_input(frames);
        group.bench_with_input(BenchmarkId::from_parameter(frames), &frames, |b, _| {
            b.iter_batched_ref(
                || input.clone(),
                |buffer| {
                    processor.process_block(buffer.as_mut_ptr(), buffer.len());
                    black_box(buffer);
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

criterion_group!(benches, benchmark_neural_drive);
criterion_main!(benches);
