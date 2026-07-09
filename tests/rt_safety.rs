//! T16: proves `StandalonePipeline::process` and the input-format glue
//! (`process_interleaved_block`, T15) never allocate on the audio thread.
//!
//! nih_plug's `assert_process_allocs` feature (Cargo.toml, kept active by
//! default per T16) installs its own `#[global_allocator]`
//! (`assert_no_alloc::AllocDisabler`) whenever `debug_assertions` is on —
//! i.e. under a plain debug `cargo test`. A binary can only have one
//! `#[global_allocator]`, so this file's own counting allocator is only
//! declared in configurations where nih_plug hasn't already claimed that
//! slot (release builds, since `debug_assertions` is off there). That's
//! exactly the configuration the T16 gate uses:
//! `cargo test --test rt_safety --release`. Under plain debug `cargo test`
//! this test still runs the same 1000-block loop (so a panic/crash in the
//! pipeline itself is still caught), it just can't assert on allocation
//! count in that configuration without conflicting with nih_plug's own
//! allocator.

use distortion::core::cabinet::CabinetMailbox;
use distortion::core::dsp::{
    process_interleaved_block, sample_convert, AudioSnapshot, StandalonePipeline,
};
use distortion::core::state::plugin_params::AmpModel;

#[cfg(not(all(debug_assertions, feature = "nih-plug-assert")))]
mod counting_alloc {
    use std::alloc::{GlobalAlloc, Layout, System};
    use std::sync::atomic::{AtomicUsize, Ordering};

    pub static ALLOC_COUNT: AtomicUsize = AtomicUsize::new(0);

    struct CountingAllocator;

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
}

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

/// Runs `body` a warmup round plus `BLOCKS` times. Where this build has its
/// own counting allocator installed (see module doc), asserts zero
/// allocations occurred during the counted round; otherwise just exercises
/// the loop.
fn assert_allocation_free(label: &str, mut body: impl FnMut()) {
    for _ in 0..8 {
        body();
    }

    #[cfg(not(all(debug_assertions, feature = "nih-plug-assert")))]
    counting_alloc::ALLOC_COUNT.store(0, std::sync::atomic::Ordering::Relaxed);

    for _ in 0..BLOCKS {
        body();
    }

    #[cfg(not(all(debug_assertions, feature = "nih-plug-assert")))]
    {
        let allocs = counting_alloc::ALLOC_COUNT.swap(0, std::sync::atomic::Ordering::Relaxed);
        assert_eq!(
            allocs, 0,
            "{label} allocated {allocs} times over {BLOCKS} blocks"
        );
    }
}

#[test]
fn standalone_pipeline_is_allocation_free_over_1000_blocks_all_formats() {
    let snap = active_snapshot();

    // --- StandalonePipeline::process directly ---
    {
        let mut pipeline = new_pipeline();
        let mut l = test_signal(MAX_BLOCK);
        let mut r = test_signal(MAX_BLOCK);
        assert_allocation_free("StandalonePipeline::process", || {
            pipeline.process(&mut l, &mut r, &snap);
        });
    }

    // --- F32 via process_interleaved_block ---
    {
        let mut pipeline = new_pipeline();
        let interleaved = test_signal(MAX_BLOCK * CHANNELS);
        let mut buf_l = vec![0.0; MAX_BLOCK];
        let mut buf_r = vec![0.0; MAX_BLOCK];
        let mut sink = 0usize;
        assert_allocation_free("F32 process_interleaved_block", || {
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
        });
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
        assert_allocation_free("I32 process_interleaved_block", || {
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
        });
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
        assert_allocation_free("I16 process_interleaved_block", || {
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
        });
        std::hint::black_box(sink);
    }
}
