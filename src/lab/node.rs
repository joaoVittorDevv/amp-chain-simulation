use crate::lab::{
    LabError, ParameterMeta, VariantFactory, VariantMailbox, VariantRegistry, VariantSlot,
};
use std::sync::Arc;

/// Loading state for a lab DSP node.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeLoadState {
    /// No variant has been loaded.
    Empty,
    /// A variant is being prepared for installation.
    Loading { variant_id: String },
    /// A variant is active or pending pickup by the audio slot.
    Active { variant_id: String },
    /// The last load request failed.
    Failed { variant_id: String, error: String },
}

/// Runtime node for one component-lab category.
pub struct Node {
    id: String,
    category_id: String,
    slot: VariantSlot,
    active_variant_id: Option<String>,
    load_state: NodeLoadState,
}

impl Node {
    /// Create an empty node for a category slot.
    pub fn new(id: impl Into<String>, category_id: impl Into<String>) -> Self {
        Self::with_mailbox(id, category_id, VariantMailbox::new_arc())
    }

    /// Create a node sharing an existing mailbox.
    pub fn with_mailbox(
        id: impl Into<String>,
        category_id: impl Into<String>,
        mailbox: Arc<VariantMailbox>,
    ) -> Self {
        Self {
            id: id.into(),
            category_id: category_id.into(),
            slot: VariantSlot::with_mailbox(mailbox),
            active_variant_id: None,
            load_state: NodeLoadState::Empty,
        }
    }

    /// Stable node id.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Category this node belongs to.
    pub fn category_id(&self) -> &str {
        &self.category_id
    }

    /// Shared mailbox used to publish variants to the audio slot.
    pub fn mailbox(&self) -> Arc<VariantMailbox> {
        self.slot.mailbox()
    }

    /// Currently selected variant id, if any.
    pub fn active_variant_id(&self) -> Option<&str> {
        self.active_variant_id.as_deref()
    }

    /// Current loading state.
    pub fn load_state(&self) -> &NodeLoadState {
        &self.load_state
    }

    /// Request a variant switch from a direct factory.
    pub fn request_switch(
        &mut self,
        variant_id: impl Into<String>,
        factory: VariantFactory,
        sample_rate: f32,
    ) -> Result<(), LabError> {
        let variant_id = variant_id.into();
        self.load_state = NodeLoadState::Loading {
            variant_id: variant_id.clone(),
        };

        let variant = factory(sample_rate);
        self.slot.install(variant);
        self.active_variant_id = Some(variant_id.clone());
        self.load_state = NodeLoadState::Active { variant_id };
        Ok(())
    }

    /// Request a variant switch by implementation id from the registry.
    pub fn request_switch_from_registry(
        &mut self,
        variant_id: impl Into<String>,
        impl_id: &str,
        registry: &VariantRegistry,
        sample_rate: f32,
    ) -> Result<(), LabError> {
        let variant_id = variant_id.into();
        match registry.get(impl_id) {
            Some(factory) => self.request_switch(variant_id, factory, sample_rate),
            None => {
                let error = format!("variant implementation '{impl_id}' is not registered");
                eprintln!("[Lab] unknown variant impl '{impl_id}' for node '{}' — falling back to passthrough: {error}", self.id);
                self.slot.mailbox().clear();
                self.active_variant_id = Some(variant_id.clone());
                self.load_state = NodeLoadState::Active { variant_id };
                Ok(())
            }
        }
    }

    /// Clear the active variant and return this node to pass-through.
    pub fn clear(&mut self) {
        self.slot.mailbox().clear();
        self.active_variant_id = None;
        self.load_state = NodeLoadState::Empty;
    }

    /// Audio-thread safe point: consume pending mailbox updates.
    pub fn collect_mailbox(&mut self) {
        self.slot.collect_mailbox();
    }

    /// UI-thread cleanup: drop old variants away from the audio callback.
    pub fn collect_garbage(&self) {
        self.slot.collect_garbage();
    }

    /// Process the active variant in-place, or pass through if none is active.
    pub fn process_block(&mut self, buffer: *mut f32, length: usize) {
        self.slot.process_block(buffer, length);
    }

    /// Read a parameter value from the active variant.
    ///
    /// Not real-time safe: inspects the audio-thread-owned variant and must not
    /// be called from the audio callback while `process_block` runs.
    pub fn get_param(&self, id: &str) -> Option<f32> {
        self.slot.get_param(id)
    }

    /// Set a parameter value on the active variant.
    ///
    /// Not real-time safe: mutates the audio-thread-owned variant and must not
    /// be called from the audio callback while `process_block` runs.
    pub fn set_param(&mut self, id: &str, value: f32) -> bool {
        self.slot.set_param(id, value)
    }

    /// Metadata for the active variant's parameters.
    pub fn param_metadata(&self) -> Vec<ParameterMeta> {
        self.slot.param_metadata()
    }
}

#[cfg(test)]
mod tests {
    use super::{Node, NodeLoadState};
    use crate::lab::{DspVariant, VariantFactory, VariantRegistry};

    struct OffsetVariant {
        offset: f32,
    }

    impl DspVariant for OffsetVariant {
        fn process_block(&mut self, buffer: *mut f32, length: usize) {
            let samples = unsafe { std::slice::from_raw_parts_mut(buffer, length) };
            for sample in samples {
                *sample += self.offset;
            }
        }

        fn param_count(&self) -> usize {
            0
        }

        fn param_ids(&self) -> &[&str] {
            &[]
        }

        fn latency(&self) -> usize {
            0
        }
    }

    fn factory(_sample_rate: f32) -> Box<dyn DspVariant> {
        Box::new(OffsetVariant { offset: 1.0 })
    }

    #[test]
    fn node_switch_tracks_active_variant_and_processes_after_collect() {
        let mut node = Node::new("node-eq", "eq");
        node.request_switch("variant-eq", factory as VariantFactory, 48_000.0)
            .expect("switch");
        node.collect_mailbox();
        let mut buffer = [0.0, 1.0];

        node.process_block(buffer.as_mut_ptr(), buffer.len());

        assert_eq!(node.active_variant_id(), Some("variant-eq"));
        assert_eq!(
            node.load_state(),
            &NodeLoadState::Active {
                variant_id: "variant-eq".to_string()
            }
        );
        assert_eq!(buffer, [1.0, 2.0]);
    }

    #[test]
    fn node_falls_back_to_passthrough_on_unknown_impl() {
        let registry = VariantRegistry::new();
        let mut node = Node::new("node-eq", "eq");

        node.request_switch_from_registry("variant-missing", "missing_impl", &registry, 48_000.0)
            .expect("unknown impl should fall back to passthrough, not error");
        node.collect_mailbox();

        let mut buffer = [1.0, 2.0];
        node.process_block(buffer.as_mut_ptr(), buffer.len());

        assert_eq!(buffer, [1.0, 2.0]);
        assert_eq!(node.active_variant_id(), Some("variant-missing"));
        assert_eq!(
            node.load_state(),
            &NodeLoadState::Active {
                variant_id: "variant-missing".to_string()
            }
        );
    }

    #[test]
    fn node_switching_to_unknown_impl_replaces_active_variant_with_passthrough() {
        let mut registry = VariantRegistry::new();
        registry.register("ok_impl", factory as VariantFactory);
        let mut node = Node::new("node-eq", "eq");
        node.request_switch_from_registry("variant-eq", "ok_impl", &registry, 48_000.0)
            .expect("first switch");
        node.collect_mailbox();

        node.request_switch_from_registry("variant-missing", "missing_impl", &registry, 48_000.0)
            .expect("unknown impl should fall back to passthrough, not error");
        node.collect_mailbox();

        let mut buffer = [1.0, 2.0];
        node.process_block(buffer.as_mut_ptr(), buffer.len());

        assert_eq!(buffer, [1.0, 2.0]);
        assert_eq!(node.active_variant_id(), Some("variant-missing"));
        assert_eq!(
            node.load_state(),
            &NodeLoadState::Active {
                variant_id: "variant-missing".to_string()
            }
        );
    }

    #[test]
    fn node_clear_is_passthrough() {
        let mut node = Node::new("node-eq", "eq");
        node.request_switch("variant-eq", factory as VariantFactory, 48_000.0)
            .expect("switch");
        node.collect_mailbox();
        node.clear();
        node.collect_mailbox();
        let mut buffer = [1.0, 2.0];

        node.process_block(buffer.as_mut_ptr(), buffer.len());

        assert_eq!(buffer, [1.0, 2.0]);
        assert_eq!(node.active_variant_id(), None);
        assert_eq!(node.load_state(), &NodeLoadState::Empty);
    }
}
