use crate::lab::{DspVariant, ParameterMeta};
use arc_swap::ArcSwapOption;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

type SharedVariant = Box<dyn DspVariant>;

/// Lock-free hand-off mailbox for DSP variants.
pub struct VariantMailbox {
    inner: ArcSwapOption<SharedVariant>,
    trash: ArcSwapOption<SharedVariant>,
    clear_requested: AtomicBool,
}

// `VariantMailbox` is shared across UI/worker/audio threads, but it only moves
// variants through atomic ArcSwap slots. DSP mutation remains owned by
// `VariantSlot::process_block(&mut self)` on the audio thread.
unsafe impl Send for VariantMailbox {}
unsafe impl Sync for VariantMailbox {}

impl Default for VariantMailbox {
    fn default() -> Self {
        Self::new()
    }
}

impl VariantMailbox {
    /// Create an empty variant mailbox.
    pub fn new() -> Self {
        Self {
            inner: ArcSwapOption::empty(),
            trash: ArcSwapOption::empty(),
            clear_requested: AtomicBool::new(false),
        }
    }

    /// Create a shareable mailbox handle.
    pub fn new_arc() -> Arc<Self> {
        Arc::new(Self::new())
    }

    /// Publish a newly-created DSP variant for the audio slot to consume.
    pub fn install(&self, variant: Box<dyn DspVariant>) {
        self.collect_garbage();
        self.clear_requested.store(false, Ordering::Release);
        self.inner.store(Some(Arc::new(variant)));
    }

    /// Convenience processing path for tests and non-audio callers.
    ///
    /// The real-time path should use [`VariantSlot::process_block`], where the
    /// active runtime is already owned by the audio thread.
    pub fn process_block(&self, buffer: *mut f32, length: usize) {
        let Some(variant_arc) = self.inner.swap(None) else {
            return;
        };

        match Arc::try_unwrap(variant_arc) {
            Ok(mut variant) => {
                variant.process_block(buffer, length);
                self.inner.store(Some(Arc::new(variant)));
            }
            Err(variant_arc) => {
                debug_assert!(
                    false,
                    "variant unexpectedly shared during mailbox processing"
                );
                self.inner.store(Some(variant_arc));
            }
        }
    }

    /// Request clearing the active variant and discard any pending install.
    pub fn clear(&self) {
        if let Some(pending) = self.inner.swap(None) {
            self.trash.store(Some(pending));
        }
        self.clear_requested.store(true, Ordering::Release);
    }

    /// Drop any parked variant on the caller's thread.
    pub fn collect_garbage(&self) {
        self.trash.store(None);
    }
}

/// Audio-thread-owned active variant slot.
pub struct VariantSlot {
    mailbox: Arc<VariantMailbox>,
    current: Option<Arc<SharedVariant>>,
}

// The audio engine moves `VariantSlot` with its owning processor, but processing
// is single-writer through `&mut self`. Shared Arcs are only used for lock-free
// handoff and are never processed concurrently.
unsafe impl Send for VariantSlot {}

impl Default for VariantSlot {
    fn default() -> Self {
        Self::new()
    }
}

impl VariantSlot {
    /// Create a slot with a fresh mailbox.
    pub fn new() -> Self {
        Self::with_mailbox(VariantMailbox::new_arc())
    }

    /// Create a slot using an externally shared mailbox.
    pub fn with_mailbox(mailbox: Arc<VariantMailbox>) -> Self {
        Self {
            mailbox,
            current: None,
        }
    }

    /// Shared mailbox handle used by UI/worker threads.
    pub fn mailbox(&self) -> Arc<VariantMailbox> {
        self.mailbox.clone()
    }

    /// Publish a variant through this slot's mailbox.
    pub fn install(&self, variant: Box<dyn DspVariant>) {
        self.mailbox.install(variant);
    }

    /// Process the current variant in-place, or pass through when empty.
    pub fn process_block(&mut self, buffer: *mut f32, length: usize) {
        let Some(variant_arc) = self.current.as_mut() else {
            return;
        };

        match Arc::get_mut(variant_arc) {
            Some(variant) => variant.process_block(buffer, length),
            None => {
                debug_assert!(
                    false,
                    "variant runtime Arc unexpectedly shared; get_mut failed"
                );
            }
        }
    }

    /// Consume pending mailbox work at a safe block boundary.
    pub fn collect_mailbox(&mut self) {
        if self.mailbox.clear_requested.swap(false, Ordering::AcqRel) {
            if let Some(old) = self.current.take() {
                self.mailbox.trash.store(Some(old));
            }
            return;
        }

        if let Some(new_variant) = self.mailbox.inner.swap(None) {
            let old = self.current.replace(new_variant);
            if let Some(old) = old {
                self.mailbox.trash.store(Some(old));
            }
        }
    }

    /// Drop parked variants on the caller's thread.
    pub fn collect_garbage(&self) {
        self.mailbox.collect_garbage();
    }

    /// Return whether this slot currently has an active variant.
    pub fn has_current(&self) -> bool {
        self.current.is_some()
    }

    /// Read a parameter from the active variant.
    ///
    /// This inspects the audio-thread-owned variant and should not be called
    /// from the audio thread while `process_block` is running.
    pub fn get_param(&self, id: &str) -> Option<f32> {
        self.current
            .as_ref()
            .and_then(|variant| variant.get_param(id))
    }

    /// Set a parameter on the active variant.
    ///
    /// This mutates the audio-thread-owned variant and should not be called
    /// from the audio thread while `process_block` is running.
    pub fn set_param(&mut self, id: &str, value: f32) -> bool {
        let Some(variant_arc) = self.current.as_mut() else {
            return false;
        };

        match Arc::get_mut(variant_arc) {
            Some(variant) => variant.set_param(id, value),
            None => {
                debug_assert!(
                    false,
                    "variant runtime Arc unexpectedly shared; get_mut failed"
                );
                false
            }
        }
    }

    /// Return metadata for the active variant's parameters.
    ///
    /// This inspects the audio-thread-owned variant and should not be called
    /// from the audio thread while `process_block` is running.
    pub fn param_metadata(&self) -> Vec<ParameterMeta> {
        self.current
            .as_ref()
            .map(|variant| variant.param_metadata())
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::{VariantMailbox, VariantSlot};
    use crate::lab::DspVariant;

    struct GainVariant {
        gain: f32,
        ids: [&'static str; 1],
    }

    impl DspVariant for GainVariant {
        fn process_block(&mut self, buffer: *mut f32, length: usize) {
            let samples = unsafe { std::slice::from_raw_parts_mut(buffer, length) };
            for sample in samples {
                *sample *= self.gain;
            }
        }

        fn param_count(&self) -> usize {
            self.ids.len()
        }

        fn param_ids(&self) -> &[&str] {
            &self.ids
        }

        fn latency(&self) -> usize {
            0
        }
    }

    fn gain_variant(gain: f32) -> Box<dyn DspVariant> {
        Box::new(GainVariant {
            gain,
            ids: ["gain"],
        })
    }

    #[test]
    fn mailbox_processes_installed_variant() {
        let mailbox = VariantMailbox::new();
        mailbox.install(gain_variant(2.0));
        let mut buffer = [0.5, 1.0, -0.5];

        mailbox.process_block(buffer.as_mut_ptr(), buffer.len());

        assert_eq!(buffer, [1.0, 2.0, -1.0]);
    }

    #[test]
    fn slot_collects_mailbox_and_processes_current_variant() {
        let mut slot = VariantSlot::new();
        slot.install(gain_variant(0.5));
        let mut buffer = [2.0, 4.0];

        slot.collect_mailbox();
        slot.process_block(buffer.as_mut_ptr(), buffer.len());

        assert_eq!(buffer, [1.0, 2.0]);
        assert!(slot.has_current());
    }

    #[test]
    fn slot_clear_returns_to_passthrough() {
        let mut slot = VariantSlot::new();
        let mailbox = slot.mailbox();
        slot.install(gain_variant(0.5));
        slot.collect_mailbox();
        mailbox.clear();
        slot.collect_mailbox();

        let mut buffer = [2.0, 4.0];
        slot.process_block(buffer.as_mut_ptr(), buffer.len());

        assert_eq!(buffer, [2.0, 4.0]);
        assert!(!slot.has_current());
    }
}
