use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use distortion::core::cabinet::CabinetMailbox;
use distortion::core::dsp::{process_interleaved_block, AudioSnapshot, StandalonePipeline};
use std::f32::consts::TAU;
use std::sync::Arc;

const SAMPLE_RATE: f32 = 48_000.0;
const BUFFER_SIZES: [usize; 5] = [64, 128, 256, 512, 1024];

struct PipelineInput {
    interleaved: Vec<f32>,
    left: Vec<f32>,
    right: Vec<f32>,
    snapshot: AudioSnapshot,
}

fn sine_input(frames: usize) -> Vec<f32> {
    (0..frames)
        .flat_map(|frame| {
            let phase = TAU * 440.0 * frame as f32 / SAMPLE_RATE;
            let sample = 0.25 * phase.sin();
            [sample, sample]
        })
        .collect()
}

fn pipeline(frames: usize, mailbox: Arc<CabinetMailbox>) -> StandalonePipeline {
    StandalonePipeline::new(SAMPLE_RATE, frames, &[1.0], mailbox)
}

fn process(pipeline: &mut StandalonePipeline, input: &mut PipelineInput) {
    process_interleaved_block(
        pipeline,
        &input.interleaved,
        2,
        0,
        1,
        |sample| sample,
        &mut input.left,
        &mut input.right,
        &input.snapshot,
        |left, right| {
            black_box(left);
            black_box(right);
        },
    );
}

fn benchmark_dsp_pipeline(c: &mut Criterion) {
    let mut group = c.benchmark_group("dsp_pipeline");

    for frames in BUFFER_SIZES {
        let interleaved = sine_input(frames);

        group.bench_with_input(BenchmarkId::new("cold", frames), &frames, |b, &frames| {
            let mailbox = CabinetMailbox::new_arc();
            let mut input = PipelineInput {
                interleaved: interleaved.clone(),
                left: vec![0.0; frames],
                right: vec![0.0; frames],
                snapshot: AudioSnapshot::default(),
            };
            b.iter(|| {
                let mut pipeline = pipeline(frames, Arc::clone(&mailbox));
                process(&mut pipeline, &mut input);
                black_box(pipeline);
            });
        });

        group.bench_with_input(BenchmarkId::new("hot", frames), &frames, |b, &frames| {
            let mut pipeline = pipeline(frames, CabinetMailbox::new_arc());
            let mut input = PipelineInput {
                interleaved: interleaved.clone(),
                left: vec![0.0; frames],
                right: vec![0.0; frames],
                snapshot: AudioSnapshot::default(),
            };

            b.iter(|| process(black_box(&mut pipeline), black_box(&mut input)));
        });
    }

    group.finish();
}

criterion_group!(benches, benchmark_dsp_pipeline);
criterion_main!(benches);
