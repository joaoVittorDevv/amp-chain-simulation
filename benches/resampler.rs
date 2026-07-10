use criterion::{black_box, criterion_group, criterion_main, Criterion};
use distortion::core::dsp::rt_resampler::RtResampler;
use std::f32::consts::TAU;

const INPUT_RATE: f32 = 44_100.0;
const OUTPUT_RATE: f32 = 48_000.0;
const FRAMES: usize = RtResampler::CHUNK_FRAMES;

fn sine_input() -> Vec<f32> {
    (0..FRAMES)
        .map(|frame| 0.25 * (TAU * 440.0 * frame as f32 / INPUT_RATE).sin())
        .collect()
}

fn benchmark_resampler(c: &mut Criterion) {
    let input_left = sine_input();
    let input_right = input_left.clone();
    let mut resampler = RtResampler::new(INPUT_RATE, OUTPUT_RATE, FRAMES);

    c.bench_function("resampler/feed/44100_to_48000/512", |b| {
        b.iter(|| {
            resampler.feed(
                black_box(&input_left),
                black_box(&input_right),
                |left, right| {
                    black_box(left);
                    black_box(right);
                },
            );
        });
    });
}

criterion_group!(benches, benchmark_resampler);
criterion_main!(benches);
