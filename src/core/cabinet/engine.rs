use super::types::CabinetRuntime;
use arc_swap::ArcSwapOption;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Total mute duration on an IR switch (split evenly into fade-out + fade-in).
const MUTE_RAMP_MS: f32 = 5.0;
const BYPASS_FADE_MS: f32 = 10.0;

/// Lock-free hand-off channel between the UI/worker thread and the audio thread.
///
/// - `inbox`: the UI publishes a freshly-built [`CabinetRuntime`] here; the audio
///   thread consumes it exactly once, moving the `Arc` (no clone/alloc).
/// - `clear_flag`: the UI requests removing the active IR (audio stage becomes
///   pass-through). A flag rather than an inbox value keeps the audio side alloc-free.
/// - `trash`: the audio thread parks the *previously* active runtime here so the
///   `Arc` is dropped (and its FFT buffers freed) by the UI thread, never on the
///   audio thread. The UI drains it before every publish and every frame.
///
/// Because IR switches are user-paced (tens of ms apart at minimum), the audio
/// thread parks at most one runtime per publish and the UI drains it well before
/// the next one, so no allocation or free ever happens on the audio thread.
pub struct CabinetMailbox {
    inbox: ArcSwapOption<CabinetRuntime>,
    trash: ArcSwapOption<CabinetRuntime>,
    clear_flag: AtomicBool,
}

impl CabinetMailbox {
    fn new() -> Self {
        Self {
            inbox: ArcSwapOption::empty(),
            trash: ArcSwapOption::empty(),
            clear_flag: AtomicBool::new(false),
        }
    }

    /// Create a standalone shared mailbox (used when the audio engine lives on a
    /// different thread than the one that owns the mailbox handle).
    pub fn new_arc() -> Arc<Self> {
        Arc::new(Self::new())
    }

    /// UI thread: publish a new runtime for the audio thread to pick up.
    /// Drains any parked old runtime first (dropped here, on the UI thread).
    pub fn publish(&self, runtime: CabinetRuntime) {
        self.trash.store(None);
        // A pending publish supersedes a pending clear.
        self.clear_flag.store(false, Ordering::Release);
        self.inbox.store(Some(Arc::new(runtime)));
    }

    /// UI thread: drop any parked old runtime. Safe to call every frame.
    pub fn collect_garbage(&self) {
        self.trash.store(None);
    }

    /// UI thread: clear the active IR (audio stage fades out to pass-through).
    pub fn clear(&self) {
        self.trash.store(None);
        self.inbox.store(None);
        self.clear_flag.store(true, Ordering::Release);
    }
}

/// Tracks a smooth wet↔dry crossfade used to (un)bypass the cabinet without clicks.
struct BypassFade {
    /// Current wet gain, 1.0 = fully engaged, 0.0 = fully bypassed.
    wet_gain: f32,
    /// Per-sample step toward the target (1.0 / fade_len).
    step: f32,
}

impl BypassFade {
    fn new() -> Self {
        Self {
            wet_gain: 1.0,
            step: 1.0 / (BYPASS_FADE_MS * 0.001 * 48_000.0),
        }
    }

    fn set_sample_rate(&mut self, sr: f32) {
        let len = (BYPASS_FADE_MS * 0.001 * sr).max(1.0);
        self.step = 1.0 / len;
    }
}

/// Mute-ramp state machine that hides the discontinuity of switching convolvers:
/// fade the current wet path out, swap at silence, then fade the new one in.
#[derive(Clone, Copy, PartialEq)]
enum MutePhase {
    Idle,
    FadeOut,
    FadeIn,
}

/// Audio-thread convolution engine with atomic runtime hand-off, a mute ramp on
/// IR switch (fade-out → swap → fade-in), and a wet↔dry crossfade on bypass.
///
/// Owned mutably by the plugin/standalone so its per-block `process` has
/// exclusive access to the convolver state. The UI publishes runtimes through
/// the shared [`CabinetMailbox`] (see [`CabinetEngine::mailbox`]).
pub struct CabinetEngine {
    mailbox: Arc<CabinetMailbox>,
    /// Audio-thread-owned live runtime (holds both channel convolvers).
    current: Option<Arc<CabinetRuntime>>,
    /// Runtime queued to install once the fade-out completes. The inner
    /// `Option` is `None` for a clear request (fade out to pass-through).
    pending: Option<Option<Arc<CabinetRuntime>>>,
    phase: MutePhase,
    /// Remaining samples in the current fade phase.
    ramp_remaining: usize,
    /// Length of each fade phase in samples (~half of `MUTE_RAMP_MS`).
    ramp_len: usize,
    bypass_fade: BypassFade,
    pub cabinet_bypass: bool,
    pub cabinet_level: f32,
    pub cabinet_mix: f32,
}

impl Default for CabinetEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl CabinetEngine {
    pub fn new() -> Self {
        Self::with_mailbox(Arc::new(CabinetMailbox::new()))
    }

    /// Build an engine that shares an externally-owned mailbox. Used by the
    /// standalone where the audio callback and the UI live on different threads.
    pub fn with_mailbox(mailbox: Arc<CabinetMailbox>) -> Self {
        Self {
            mailbox,
            current: None,
            pending: None,
            phase: MutePhase::Idle,
            ramp_remaining: 0,
            ramp_len: ((MUTE_RAMP_MS * 0.001 * 48_000.0) * 0.5) as usize,
            bypass_fade: BypassFade::new(),
            cabinet_bypass: false,
            cabinet_level: 1.0,
            cabinet_mix: 1.0,
        }
    }

    /// Shared handle the UI uses to publish runtimes / collect garbage.
    pub fn mailbox(&self) -> Arc<CabinetMailbox> {
        self.mailbox.clone()
    }

    /// UI thread: publish a freshly-built runtime (convenience wrapper).
    pub fn load_runtime(&self, runtime: CabinetRuntime) {
        self.mailbox.publish(runtime);
    }

    /// Configure ramp lengths for the current engine sample rate. Call from
    /// `initialize()`.
    pub fn set_sample_rate(&mut self, sr: f32) {
        self.ramp_len = ((MUTE_RAMP_MS * 0.001 * sr) * 0.5).max(1.0) as usize;
        self.bypass_fade.set_sample_rate(sr);
    }

    /// Hash of the currently active IR, if any. Borrowed from the live `Arc`, so
    /// this never allocates.
    pub fn current_hash(&self) -> Option<&str> {
        self.current.as_ref().map(|rt| rt.ir_hash.as_str())
    }

    /// Bypass request (the authoritative value is also passed each block to
    /// `process`, so this is mostly for parity with the spec's API).
    pub fn set_bypass(&mut self, _bypass: bool) {}

    /// Consume any pending command (clear/load) from the mailbox and begin the
    /// corresponding mute transition. Never allocates on the audio thread: the
    /// runtime `Arc` is moved, not cloned, and the old one is parked in the trash.
    fn poll_commands(&mut self) {
        if self.mailbox.clear_flag.swap(false, Ordering::AcqRel) {
            self.begin_swap(None);
        }
        if let Some(new_rt) = self.mailbox.inbox.swap(None) {
            self.begin_swap(Some(new_rt));
        }
    }

    /// Queue a swap to `target` (`None` = clear). If a runtime is currently
    /// active we fade it out first; otherwise we install immediately and fade in.
    fn begin_swap(&mut self, target: Option<Arc<CabinetRuntime>>) {
        // Park a superseded pending install (rare: two switches within one fade).
        if let Some(old_pending) = self.pending.take() {
            self.mailbox.trash.store(old_pending);
        }

        if self.current.is_none() {
            // Nothing to fade out — install now and fade the new one in.
            self.current = target;
            if self.current.is_some() {
                self.phase = MutePhase::FadeIn;
                self.ramp_remaining = self.ramp_len;
            } else {
                self.phase = MutePhase::Idle;
                self.ramp_remaining = 0;
            }
        } else {
            self.pending = Some(target);
            self.phase = MutePhase::FadeOut;
            self.ramp_remaining = self.ramp_len;
        }
    }

    /// Called at a block boundary once the fade-out has fully muted the wet path:
    /// swap in the queued runtime and start the fade-in (or settle to pass-through).
    fn install_pending(&mut self) {
        let target = self.pending.take().unwrap_or(None);
        let old = std::mem::replace(&mut self.current, target);
        self.mailbox.trash.store(old);
        if self.current.is_some() {
            self.phase = MutePhase::FadeIn;
            self.ramp_remaining = self.ramp_len;
        } else {
            self.phase = MutePhase::Idle;
            self.ramp_remaining = 0;
        }
    }

    /// Process one stereo block in place.
    ///
    /// `left`/`right` carry the dry input to this stage on entry and the wet+dry
    /// mixed, leveled output on exit. `scratch_l`/`scratch_r` are caller-owned
    /// scratch buffers of at least the block length. `bypass`, `level` and `mix`
    /// are per-block smoothed values supplied by the caller.
    ///
    /// Real-time safe: no allocation, no locking, no file/DB access.
    pub fn process(
        &mut self,
        left: &mut [f32],
        right: &mut [f32],
        scratch_l: &mut [f32],
        scratch_r: &mut [f32],
        bypass: bool,
        level: f32,
        mix: f32,
    ) {
        self.poll_commands();

        self.cabinet_bypass = bypass;
        self.cabinet_level = level;
        self.cabinet_mix = mix;

        let len = left
            .len()
            .min(right.len())
            .min(scratch_l.len())
            .min(scratch_r.len());
        if len == 0 {
            return;
        }

        let target_wet = if bypass { 0.0 } else { 1.0 };

        // No IR loaded → cabinet is pure pass-through. Settle the fades so a later
        // engage doesn't jump. (When `current` is `None`, `phase` is always Idle.)
        if self.current.is_none() {
            self.bypass_fade.wet_gain = target_wet;
            return;
        }

        // Preserve the dry signal, then convolve into the sample buffers.
        scratch_l[..len].copy_from_slice(&left[..len]);
        scratch_r[..len].copy_from_slice(&right[..len]);

        match self.current.as_mut().and_then(Arc::get_mut) {
            Some(rt) => {
                if rt
                    .convolver_l
                    .process(&scratch_l[..len], &mut left[..len])
                    .is_err()
                {
                    left[..len].copy_from_slice(&scratch_l[..len]);
                }
                if rt
                    .convolver_r
                    .process(&scratch_r[..len], &mut right[..len])
                    .is_err()
                {
                    right[..len].copy_from_slice(&scratch_r[..len]);
                }
            }
            None => {
                // The live runtime should be uniquely owned on the audio thread;
                // a shared Arc here is a bug. Fail loudly in debug, pass dry in release.
                debug_assert!(
                    false,
                    "cabinet runtime Arc unexpectedly shared; get_mut failed"
                );
                return;
            }
        }

        let mut wet_gain = self.bypass_fade.wet_gain;
        let step = self.bypass_fade.step;
        let phase = self.phase;
        let mut ramp = self.ramp_remaining;
        let ramp_len = self.ramp_len.max(1);

        for i in 0..len {
            // Mute ramp: fade the wet path out (1→0) or in (0→1) during a switch.
            let mute = match phase {
                MutePhase::Idle => 1.0,
                MutePhase::FadeOut => ramp as f32 / ramp_len as f32,
                MutePhase::FadeIn => (ramp_len - ramp) as f32 / ramp_len as f32,
            };
            if ramp > 0 {
                ramp -= 1;
            }

            let dry_l = scratch_l[i];
            let dry_r = scratch_r[i];
            let wet_l = left[i] * level * mute;
            let wet_r = right[i] * level * mute;

            // Wet/dry mix around the convolver.
            let mixed_l = dry_l * (1.0 - mix) + wet_l * mix;
            let mixed_r = dry_r * (1.0 - mix) + wet_r * mix;

            // Bypass crossfade between the processed signal and the pure dry input.
            left[i] = dry_l * (1.0 - wet_gain) + mixed_l * wet_gain;
            right[i] = dry_r * (1.0 - wet_gain) + mixed_r * wet_gain;

            // Advance the bypass crossfade toward its target.
            if wet_gain < target_wet {
                wet_gain = (wet_gain + step).min(target_wet);
            } else if wet_gain > target_wet {
                wet_gain = (wet_gain - step).max(target_wet);
            }
        }

        self.bypass_fade.wet_gain = wet_gain;
        self.ramp_remaining = ramp;

        // Advance the mute state machine at the block boundary.
        match self.phase {
            MutePhase::FadeOut if self.ramp_remaining == 0 => self.install_pending(),
            MutePhase::FadeIn if self.ramp_remaining == 0 => self.phase = MutePhase::Idle,
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::cabinet::CabinetRuntime;

    const DEFAULT_IR: &[u8] = include_bytes!("../../../neural/drive/cabinet_ir.wav");

    fn scratch(n: usize) -> (Vec<f32>, Vec<f32>) {
        (vec![0.0; n], vec![0.0; n])
    }

    #[test]
    fn passthrough_when_no_runtime() {
        let mut eng = CabinetEngine::new();
        eng.set_sample_rate(48_000.0);
        let mut l = vec![0.5f32; 64];
        let mut r = vec![-0.5f32; 64];
        let (mut sl, mut sr) = scratch(64);
        eng.process(&mut l, &mut r, &mut sl, &mut sr, false, 1.0, 1.0);
        // No IR loaded → untouched pass-through.
        assert!(l.iter().all(|&x| (x - 0.5).abs() < 1e-6));
        assert!(eng.current_hash().is_none());
    }

    #[test]
    fn installs_runtime_and_produces_finite_output() {
        let mut eng = CabinetEngine::new();
        eng.set_sample_rate(48_000.0);
        let rt = CabinetRuntime::build(DEFAULT_IR, 48_000.0, 64).expect("build");
        let hash = rt.ir_hash.clone();
        eng.load_runtime(rt);

        let mut l = vec![1.0f32; 64];
        let mut r = vec![1.0f32; 64];
        let (mut sl, mut sr) = scratch(64);
        eng.process(&mut l, &mut r, &mut sl, &mut sr, false, 1.0, 1.0);

        assert_eq!(eng.current_hash(), Some(hash.as_str()));
        assert!(l.iter().all(|x| x.is_finite()));
        assert!(r.iter().all(|x| x.is_finite()));
    }

    #[test]
    fn full_bypass_returns_dry() {
        let mut eng = CabinetEngine::new();
        eng.set_sample_rate(48_000.0);
        let rt = CabinetRuntime::build(DEFAULT_IR, 48_000.0, 64).expect("build");
        eng.load_runtime(rt);
        // Settle the bypass crossfade fully by processing several bypassed blocks.
        for _ in 0..64 {
            let mut l = vec![0.25f32; 64];
            let mut r = vec![0.25f32; 64];
            let (mut sl, mut sr) = scratch(64);
            eng.process(&mut l, &mut r, &mut sl, &mut sr, true, 1.0, 1.0);
            if l.iter().all(|&x| (x - 0.25).abs() < 1e-4) {
                return;
            }
        }
        panic!("bypass crossfade never reached full dry");
    }

    #[test]
    fn clear_removes_active_runtime() {
        let mut eng = CabinetEngine::new();
        eng.set_sample_rate(48_000.0);
        let rt = CabinetRuntime::build(DEFAULT_IR, 48_000.0, 64).expect("build");
        eng.load_runtime(rt);

        // Install + fade in.
        let mut l = vec![1.0f32; 64];
        let mut r = vec![1.0f32; 64];
        let (mut sl, mut sr) = scratch(64);
        eng.process(&mut l, &mut r, &mut sl, &mut sr, false, 1.0, 1.0);
        assert!(eng.current_hash().is_some(), "runtime installed");

        // Request clear, then process enough blocks to complete the fade-out.
        eng.mailbox().clear();
        for _ in 0..64 {
            let mut l = vec![1.0f32; 64];
            let mut r = vec![1.0f32; 64];
            let (mut sl, mut sr) = scratch(64);
            eng.process(&mut l, &mut r, &mut sl, &mut sr, false, 1.0, 1.0);
            if eng.current_hash().is_none() {
                break;
            }
        }
        assert!(
            eng.current_hash().is_none(),
            "clear removed the active runtime"
        );

        // After clear the stage is pass-through.
        let mut l = vec![0.3f32; 64];
        let mut r = vec![0.3f32; 64];
        let (mut sl, mut sr) = scratch(64);
        eng.process(&mut l, &mut r, &mut sl, &mut sr, false, 1.0, 1.0);
        assert!(
            l.iter().all(|&x| (x - 0.3).abs() < 1e-6),
            "pass-through after clear"
        );
    }

    #[test]
    fn ir_switch_stays_finite() {
        let mut eng = CabinetEngine::new();
        eng.set_sample_rate(48_000.0);
        eng.load_runtime(CabinetRuntime::build(DEFAULT_IR, 48_000.0, 64).expect("build"));

        // Run, switch to a resampled build (different runtime), keep running.
        for k in 0..200 {
            if k == 20 {
                eng.load_runtime(CabinetRuntime::build(DEFAULT_IR, 44_100.0, 64).expect("build"));
            }
            let mut l = vec![0.8f32; 64];
            let mut r = vec![0.8f32; 64];
            let (mut sl, mut sr) = scratch(64);
            eng.process(&mut l, &mut r, &mut sl, &mut sr, false, 1.0, 1.0);
            assert!(l.iter().all(|x| x.is_finite()) && r.iter().all(|x| x.is_finite()));
        }
    }
}
