#![cfg(feature = "rt-safety-test")]

//! T16: proves `StandalonePipeline::process` and the input-format glue
//! (`process_interleaved_block`, T15) never allocate on the audio thread.
//!
//! nih_plug's `assert_process_allocs` feature (Cargo.toml) already guards
//! the plugin's `process()` at runtime under a debug build; this test
//! covers the standalone binary's equivalent path, which nih_plug's guard
//! doesn't reach, by installing a counting `GlobalAlloc` and running 1000
//! blocks through every supported input format (F32/I32/I16).
//!
//! One `#[test]` function, run sequentially, because the counter is a
//! single process-wide global: parallel test threads would corrupt each
//! other's counts.

use distortion::core::cabinet::CabinetMailbox;
use distortion::core::dsp::{
    process_interleaved_block, sample_convert, AudioSnapshot, StandalonePipeline,
};
use distortion::core::state::plugin_params::AmpModel;
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

struct CountingAllocator;

static ALLOC_COUNT: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOC_COUNT.fetch_add(1, Ordering::Relaxed);
        System.alloc(layout)
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout)
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        ALLOC_COUNT.fetch_add(1, Ordering::Relaxed);
        System.realloc(ptr, layout, new_size)
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        ALLOC_COUNT.fetch_add(1, Ordering::Relaxed);
        System.alloc_zeroed(layout)
    }
}

#[global_allocator]
static GLOBAL: CountingAllocator = CountingAllocator;

const SAMPLE_RATE: f32 = 48_000.0;
const MAX_BLOCK: usize = 256;
const CHANNELS: usize = 2;
const BLOCKS: usize = 1000;

fn new_pipeline() -> StandalonePipeline {
    StandalonePipeline::new(SAMPLE_RATE, MAX_BLOCK, &[1.0], CabinetMailbox::new_arc())
}

fn active_snapshot() -> AudioSnapshot {
    // Exercise every stage's allocation-free path, not just a no-op chain:
    // EQ on, limiter on, Neural amp active.
    let mut snap = AudioSnapshot::default();
    snap.eq_active = true;
    snap.amp_model = AmpModel::Neural;
    snap.neural_active = true;
    snap.limiter_enable = true;
    snap
}

fn test_signal(len: usize) -> Vec<f32> {
    (0..len).map(|i| 0.3 * (i as f32 * 0.05).sin()).collect()
}

#[test]
fn standalone_pipeline_is_allocation_free_over_1000_blocks_all_formats() {
    let snap = active_snapshot();

    // --- StandalonePipeline::process directly ---
    {
        let mut pipeline = new_pipeline();
        let mut l = test_signal(MAX_BLOCK);
        let mut r = test_signal(MAX_BLOCK);
        // Warm up: first calls may allocate internal state the processors
        // lazily size on first use (e.g. FFT plan caches). Only the steady
        // state has to be allocation-free.
        for _ in 0..8 {
            pipeline.process(&mut l, &mut r, &snap);
        }

        ALLOC_COUNT.store(0, Ordering::Relaxed);
        for _ in 0..BLOCKS {
            pipeline.process(&mut l, &mut r, &snap);
        }
        let allocs = ALLOC_COUNT.swap(0, Ordering::Relaxed);
        assert_eq!(
            allocs, 0,
            "StandalonePipeline::process allocated {allocs} times over {BLOCKS} blocks"
        );
    }

    // --- F32 via process_interleaved_block ---
    {
        let mut pipeline = new_pipeline();
        let interleaved = test_signal(MAX_BLOCK * CHANNELS);
        let mut buf_l = vec![0.0; MAX_BLOCK];
        let mut buf_r = vec![0.0; MAX_BLOCK];
        let mut sink = 0usize;

        for _ in 0..8 {
            process_interleaved_block(
                &mut pipeline,
                &interleaved,
                CHANNELS,
                0,
                1,
                |s| s,
                &mut buf_l,
                &mut buf_r,
                &snap,
                |l, _r| sink = sink.wrapping_add(l.len()),
            );
        }

        ALLOC_COUNT.store(0, Ordering::Relaxed);
        for _ in 0..BLOCKS {
            process_interleaved_block(
                &mut pipeline,
                &interleaved,
                CHANNELS,
                0,
                1,
                |s| s,
                &mut buf_l,
                &mut buf_r,
                &snap,
                |l, _r| sink = sink.wrapping_add(l.len()),
            );
        }
        let allocs = ALLOC_COUNT.swap(0, Ordering::Relaxed);
        assert_eq!(
            allocs, 0,
            "F32 process_interleaved_block allocated {allocs} times over {BLOCKS} blocks"
        );
        std::hint::black_box(sink);
    }

    // --- I32 via process_interleaved_block ---
    {
        let mut pipeline = new_pipeline();
        let interleaved: Vec<i32> = test_signal(MAX_BLOCK * CHANNELS)
            .into_iter()
            .map(sample_convert::f32_to_i32)
            .collect();
        let mut buf_l = vec![0.0; MAX_BLOCK];
        let mut buf_r = vec![0.0; MAX_BLOCK];
        let mut sink = 0usize;

        for _ in 0..8 {
            process_interleaved_block(
                &mut pipeline,
                &interleaved,
                CHANNELS,
                0,
                1,
                sample_convert::i32_to_f32,
                &mut buf_l,
                &mut buf_r,
                &snap,
                |l, _r| sink = sink.wrapping_add(l.len()),
            );
        }

        ALLOC_COUNT.store(0, Ordering::Relaxed);
        for _ in 0..BLOCKS {
            process_interleaved_block(
                &mut pipeline,
                &interleaved,
                CHANNELS,
                0,
                1,
                sample_convert::i32_to_f32,
                &mut buf_l,
                &mut buf_r,
                &snap,
                |l, _r| sink = sink.wrapping_add(l.len()),
            );
        }
        let allocs = ALLOC_COUNT.swap(0, Ordering::Relaxed);
        assert_eq!(
            allocs, 0,
            "I32 process_interleaved_block allocated {allocs} times over {BLOCKS} blocks"
        );
        std::hint::black_box(sink);
    }

    // --- I16 via process_interleaved_block ---
    {
        let mut pipeline = new_pipeline();
        let interleaved: Vec<i16> = test_signal(MAX_BLOCK * CHANNELS)
            .into_iter()
            .map(sample_convert::f32_to_i16)
            .collect();
        let mut buf_l = vec![0.0; MAX_BLOCK];
        let mut buf_r = vec![0.0; MAX_BLOCK];
        let mut sink = 0usize;

        for _ in 0..8 {
            process_interleaved_block(
                &mut pipeline,
                &interleaved,
                CHANNELS,
                0,
                1,
                sample_convert::i16_to_f32,
                &mut buf_l,
                &mut buf_r,
                &snap,
                |l, _r| sink = sink.wrapping_add(l.len()),
            );
        }

        ALLOC_COUNT.store(0, Ordering::Relaxed);
        for _ in 0..BLOCKS {
            process_interleaved_block(
                &mut pipeline,
                &interleaved,
                CHANNELS,
                0,
                1,
                sample_convert::i16_to_f32,
                &mut buf_l,
                &mut buf_r,
                &snap,
                |l, _r| sink = sink.wrapping_add(l.len()),
            );
        }
        let allocs = ALLOC_COUNT.swap(0, Ordering::Relaxed);
        assert_eq!(
            allocs, 0,
            "I16 process_interleaved_block allocated {allocs} times over {BLOCKS} blocks"
        );
        std::hint::black_box(sink);
    }
}
