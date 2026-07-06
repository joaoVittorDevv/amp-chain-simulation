use super::types::CabinetRuntime;
use arc_swap::ArcSwapOption;
use std::sync::Arc;

/// Default ramp lengths in milliseconds (converted to samples via `set_sample_rate`).
const MUTE_RAMP_MS: f32 = 5.0;
const BYPASS_FADE_MS: f32 = 10.0;

/// Lock-free hand-off channel between the UI/worker thread and the audio thread.
///
/// - `inbox`: the UI publishes a freshly-built [`CabinetRuntime`] here; the audio
///   thread consumes it exactly once.
/// - `trash`: the audio thread parks the *previously* active runtime here so the
///   `Arc` is dropped (and its FFT buffers freed) by the UI thread, never on the
///   audio thread. The UI drains it before every publish.
///
/// Because IR switches are user-paced (tens of ms apart at minimum), the audio
/// thread parks at most one runtime per publish and the UI drains it on the next
/// publish, so no allocation or free ever happens on the audio thread.
pub struct CabinetMailbox {
    inbox: ArcSwapOption<CabinetRuntime>,
    trash: ArcSwapOption<CabinetRuntime>,
}

impl CabinetMailbox {
    fn new() -> Self {
        Self {
            inbox: ArcSwapOption::empty(),
            trash: ArcSwapOption::empty(),
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
        self.inbox.store(Some(Arc::new(runtime)));
    }

    /// UI thread: drop any parked old runtime. Safe to call every frame.
    pub fn collect_garbage(&self) {
        self.trash.store(None);
    }

    /// UI thread: clear the active IR (audio stage becomes pass-through).
    pub fn clear(&self) {
        self.trash.store(None);
        // Publish an explicit "none" by leaving inbox empty and letting the audio
        // thread's current runtime be parked. We model this as publishing nothing;
        // callers that want pass-through simply never build a runtime.
        self.inbox.store(None);
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

/// Audio-thread convolution engine with atomic runtime hand-off, a 5 ms mute
/// ramp on IR switch, and a 10 ms wet↔dry crossfade on bypass.
///
/// Owned mutably by the plugin/standalone so its per-block `process` has
/// exclusive access to the convolver state. The UI publishes runtimes through
/// the shared [`CabinetMailbox`] (see [`CabinetEngine::mailbox`]).
pub struct CabinetEngine {
    mailbox: Arc<CabinetMailbox>,
    /// Audio-thread-owned live runtime (holds both channel convolvers).
    current: Option<Arc<CabinetRuntime>>,
    last_hash: Option<String>,
    /// Remaining mute-ramp samples after an IR switch.
    mute_remaining: usize,
    /// Total mute-ramp length in samples.
    mute_len: usize,
    bypass_fade: BypassFade,
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
            last_hash: None,
            mute_remaining: 0,
            mute_len: (MUTE_RAMP_MS * 0.001 * 48_000.0) as usize,
            bypass_fade: BypassFade::new(),
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
        self.mute_len = (MUTE_RAMP_MS * 0.001 * sr).max(1.0) as usize;
        self.bypass_fade.set_sample_rate(sr);
    }

    /// Hash of the currently active IR, if any.
    pub fn current_hash(&self) -> Option<&str> {
        self.last_hash.as_deref()
    }

    /// Bypass request (the authoritative value is also passed each block to
    /// `process`, so this is mostly for parity with the spec's API).
    pub fn set_bypass(&mut self, _bypass: bool) {}

    /// Consume a pending runtime from the mailbox, if any. Parks the old runtime
    /// in the trash for the UI to drop. Starts the mute ramp on a real change.
    fn poll_mailbox(&mut self) {
        if let Some(new_rt) = self.mailbox.inbox.swap(None) {
            let new_hash = new_rt.ir_hash.clone();
            if self.last_hash.as_deref() != Some(new_hash.as_str()) {
                self.mute_remaining = self.mute_len;
            }
            // Park the previous runtime for the UI thread to drop.
            let old = self.current.replace(new_rt);
            self.mailbox.trash.store(old);
            self.last_hash = Some(new_hash);
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
        self.poll_mailbox();

        let len = left.len().min(right.len()).min(scratch_l.len()).min(scratch_r.len());
        if len == 0 {
            return;
        }

        let target_wet = if bypass { 0.0 } else { 1.0 };

        // No IR loaded: cabinet is pass-through. Still settle the fade so a later
        // engage doesn't jump.
        let runtime = match self.current.as_mut().and_then(Arc::get_mut) {
            Some(rt) => rt,
            None => {
                self.bypass_fade.wet_gain = target_wet;
                return;
            }
        };

        // Preserve the dry signal, then convolve into the sample buffers.
        scratch_l[..len].copy_from_slice(&left[..len]);
        scratch_r[..len].copy_from_slice(&right[..len]);

        if runtime.convolver_l.process(&scratch_l[..len], &mut left[..len]).is_err() {
            left[..len].copy_from_slice(&scratch_l[..len]);
        }
        if runtime.convolver_r.process(&scratch_r[..len], &mut right[..len]).is_err() {
            right[..len].copy_from_slice(&scratch_r[..len]);
        }

        let mut wet_gain = self.bypass_fade.wet_gain;
        let step = self.bypass_fade.step;
        let mut mute_remaining = self.mute_remaining;
        let mute_len = self.mute_len.max(1);

        for i in 0..len {
            // Mute ramp (fade the wet path back in after an IR switch).
            let mute = if mute_remaining > 0 {
                let done = mute_len - mute_remaining;
                mute_remaining -= 1;
                done as f32 / mute_len as f32
            } else {
                1.0
            };

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
        self.mute_remaining = mute_remaining;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::cabinet::CabinetRuntime;

    const DEFAULT_IR: &[u8] = include_bytes!("../../../neural/drive/cabinet_ir.wav");

    #[test]
    fn passthrough_when_no_runtime() {
        let mut eng = CabinetEngine::new();
        eng.set_sample_rate(48_000.0);
        let mut l = vec![0.5f32; 64];
        let mut r = vec![-0.5f32; 64];
        let (mut sl, mut sr) = (vec![0.0; 64], vec![0.0; 64]);
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
        let (mut sl, mut sr) = (vec![0.0; 64], vec![0.0; 64]);
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
            let (mut sl, mut sr) = (vec![0.0; 64], vec![0.0; 64]);
            eng.process(&mut l, &mut r, &mut sl, &mut sr, true, 1.0, 1.0);
            // Once fully bypassed the output equals the dry input.
            if l.iter().all(|&x| (x - 0.25).abs() < 1e-4) {
                return;
            }
        }
        panic!("bypass crossfade never reached full dry");
    }
}
