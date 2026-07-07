use crate::lab::{
    Category, LabError, Node, PipelineConfig, SlotConfig, VariantFactory, VariantMailbox,
};
use std::collections::HashSet;
use std::sync::Arc;

/// Runtime slot for one component-lab category.
pub struct NodeSlot {
    category: Category,
    node: Node,
    active_variant_id: Option<String>,
    bypassed: bool,
}

impl NodeSlot {
    /// Create an empty slot for a category.
    pub fn new(category: Category) -> Self {
        let node_id = format!("node-{}", category.id);
        Self {
            node: Node::new(node_id, category.id.clone()),
            category,
            active_variant_id: None,
            bypassed: false,
        }
    }

    /// Category metadata for this slot.
    pub fn category(&self) -> &Category {
        &self.category
    }

    /// Active variant id for this slot, if any.
    pub fn active_variant_id(&self) -> Option<&str> {
        self.active_variant_id.as_deref()
    }

    /// Return whether this slot is bypassed.
    pub fn bypassed(&self) -> bool {
        self.bypassed
    }

    /// Borrow the node owned by this slot.
    pub fn node(&self) -> &Node {
        &self.node
    }

    /// Mutably borrow the node owned by this slot.
    pub fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}

/// Runtime pipeline manager ordered by category sort order.
pub struct PipelineManager {
    slots: Vec<NodeSlot>,
    master_gain_db: f32,
}

impl PipelineManager {
    /// Create an empty pipeline manager.
    pub fn new() -> Self {
        Self {
            slots: Vec::new(),
            master_gain_db: 0.0,
        }
    }

    /// Create slots from categories, sorted by `sort_order`.
    pub fn from_categories(categories: &[Category]) -> Result<Self, LabError> {
        let mut manager = Self::new();
        let mut categories = categories.to_vec();
        categories.sort_by(|left, right| {
            left.sort_order
                .cmp(&right.sort_order)
                .then_with(|| left.id.cmp(&right.id))
        });
        for category in categories {
            manager.register_category(category)?;
        }
        Ok(manager)
    }

    /// Restore a runtime pipeline from serialized config and category metadata.
    pub fn from_config(categories: &[Category], config: PipelineConfig) -> Result<Self, LabError> {
        let mut manager = Self::from_categories(categories)?;
        manager.master_gain_db = config.master_gain_db;
        for slot_config in config.slots {
            if let Some(slot) = manager.slot_mut(&slot_config.category_id) {
                slot.active_variant_id = slot_config.active_variant_id;
                slot.bypassed = slot_config.bypassed;
            }
        }
        manager.validate()?;
        Ok(manager)
    }

    /// Register one category slot. Duplicate category IDs are rejected.
    pub fn register_category(&mut self, category: Category) -> Result<(), LabError> {
        if self
            .slots
            .iter()
            .any(|slot| slot.category.id == category.id)
        {
            return Err(LabError::InvalidData(format!(
                "category '{}' is already registered",
                category.id
            )));
        }
        self.slots.push(NodeSlot::new(category));
        self.slots.sort_by(|left, right| {
            left.category
                .sort_order
                .cmp(&right.category.sort_order)
                .then_with(|| left.category.id.cmp(&right.category.id))
        });
        Ok(())
    }

    /// Ordered pipeline slots.
    pub fn slots(&self) -> &[NodeSlot] {
        &self.slots
    }

    /// Return a node by category id.
    pub fn get_node(&self, category_id: &str) -> Option<&Node> {
        self.slots
            .iter()
            .find(|slot| slot.category.id == category_id)
            .map(NodeSlot::node)
    }

    /// Return a mutable node by category id.
    pub fn get_node_mut(&mut self, category_id: &str) -> Option<&mut Node> {
        self.slots
            .iter_mut()
            .find(|slot| slot.category.id == category_id)
            .map(NodeSlot::node_mut)
    }

    /// Set the active variant for a category using a compiled factory.
    pub fn request_switch(
        &mut self,
        category_id: &str,
        variant_id: &str,
        factory: VariantFactory,
        sample_rate: f32,
    ) -> Result<(), LabError> {
        if self.amp_switch_would_conflict(category_id) {
            return Err(LabError::InvalidData(
                "amp-modeler and amp-capture cannot both be active".to_string(),
            ));
        }
        let slot = self.slot_mut(category_id).ok_or_else(|| {
            LabError::InvalidData(format!("category '{category_id}' is not registered"))
        })?;
        slot.node.request_switch(variant_id, factory, sample_rate)?;
        slot.active_variant_id = Some(variant_id.to_string());
        Ok(())
    }

    /// Set a parameter on a category's active variant.
    ///
    /// Any pending mailbox variant is collected first so the value lands on the
    /// live runtime. Unknown parameter ids are ignored. Not real-time safe:
    /// intended for UI and snapshot-restore paths, never the audio thread.
    pub fn set_node_param(
        &mut self,
        category_id: &str,
        param_id: &str,
        value: f32,
    ) -> Result<(), LabError> {
        let node = self.get_node_mut(category_id).ok_or_else(|| {
            LabError::InvalidData(format!("category '{category_id}' is not registered"))
        })?;
        node.collect_mailbox();
        node.set_param(param_id, value);
        Ok(())
    }

    /// Clear a category slot.
    pub fn clear_category(&mut self, category_id: &str) -> Result<(), LabError> {
        let slot = self.slot_mut(category_id).ok_or_else(|| {
            LabError::InvalidData(format!("category '{category_id}' is not registered"))
        })?;
        slot.node.clear();
        slot.active_variant_id = None;
        Ok(())
    }

    /// Set bypass for a category slot.
    pub fn set_bypass(&mut self, category_id: &str, bypassed: bool) -> Result<(), LabError> {
        let slot = self.slot_mut(category_id).ok_or_else(|| {
            LabError::InvalidData(format!("category '{category_id}' is not registered"))
        })?;
        let previous = slot.bypassed;
        slot.bypassed = bypassed;
        if let Err(err) = self.validate() {
            if let Some(slot) = self.slot_mut(category_id) {
                slot.bypassed = previous;
            }
            return Err(err);
        }
        Ok(())
    }

    /// Convert current runtime slot state into serializable config.
    pub fn to_config(&self, name: impl Into<String>) -> PipelineConfig {
        PipelineConfig {
            name: name.into(),
            version: "1.0".to_string(),
            slots: self
                .slots
                .iter()
                .map(|slot| SlotConfig {
                    category_id: slot.category.id.clone(),
                    active_variant_id: slot.active_variant_id.clone(),
                    bypassed: slot.bypassed,
                })
                .collect(),
            master_gain_db: self.master_gain_db,
        }
    }

    /// Validate one slot per category and amp-modeler/amp-capture exclusivity.
    pub fn validate(&self) -> Result<(), LabError> {
        let mut seen = HashSet::new();
        for slot in &self.slots {
            if !seen.insert(slot.category.id.as_str()) {
                return Err(LabError::InvalidData(format!(
                    "category '{}' has more than one slot",
                    slot.category.id
                )));
            }
        }

        let amp_modeler_active = self.slot_active("amp-modeler");
        let amp_capture_active = self.slot_active("amp-capture");
        if amp_modeler_active && amp_capture_active {
            return Err(LabError::InvalidData(
                "amp-modeler and amp-capture cannot both be active".to_string(),
            ));
        }
        Ok(())
    }

    /// Process all non-bypassed slots in order.
    pub fn process_block(&mut self, buffer: *mut f32, length: usize) {
        for slot in &mut self.slots {
            slot.node.collect_mailbox();
            if !slot.bypassed {
                slot.node.process_block(buffer, length);
            }
        }
    }

    /// Drop parked old variants for every slot on the caller thread.
    pub fn collect_garbage(&self) {
        for slot in &self.slots {
            slot.node.collect_garbage();
        }
    }

    /// Clone mailbox handles for UI-thread garbage collection.
    pub fn mailboxes(&self) -> Vec<Arc<VariantMailbox>> {
        self.slots.iter().map(|slot| slot.node.mailbox()).collect()
    }

    fn slot_mut(&mut self, category_id: &str) -> Option<&mut NodeSlot> {
        self.slots
            .iter_mut()
            .find(|slot| slot.category.id == category_id)
    }

    fn slot_active(&self, category_id: &str) -> bool {
        self.slots.iter().any(|slot| {
            slot.category.id == category_id && slot.active_variant_id.is_some() && !slot.bypassed
        })
    }

    fn amp_switch_would_conflict(&self, category_id: &str) -> bool {
        match category_id {
            "amp-modeler" => self.slot_active("amp-capture"),
            "amp-capture" => self.slot_active("amp-modeler"),
            _ => false,
        }
    }
}

impl Default for PipelineManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::PipelineManager;
    use crate::lab::{Category, DspVariant, VariantFactory};

    fn category(id: &str, order: i64) -> Category {
        Category {
            id: id.to_string(),
            name: id.to_string(),
            description: None,
            sort_order: order,
        }
    }

    struct MultiplyVariant(f32);

    impl DspVariant for MultiplyVariant {
        fn process_block(&mut self, buffer: *mut f32, length: usize) {
            let samples = unsafe { std::slice::from_raw_parts_mut(buffer, length) };
            for sample in samples {
                *sample *= self.0;
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

    fn double_factory(_sample_rate: f32) -> Box<dyn DspVariant> {
        Box::new(MultiplyVariant(2.0))
    }

    #[test]
    fn pipeline_orders_slots_by_category_sort_order() {
        let manager = PipelineManager::from_categories(&[
            category("cab-sim", 20),
            category("eq", 10),
            category("output-stage", 30),
        ])
        .expect("pipeline");

        let ids: Vec<_> = manager
            .slots()
            .iter()
            .map(|slot| slot.category().id.as_str())
            .collect();

        assert_eq!(ids, ["eq", "cab-sim", "output-stage"]);
    }

    #[test]
    fn pipeline_rejects_duplicate_categories() {
        let mut manager = PipelineManager::new();
        manager
            .register_category(category("eq", 10))
            .expect("first category");
        let err = manager
            .register_category(category("eq", 20))
            .expect_err("duplicate category");

        assert!(err.to_string().contains("already registered"));
    }

    #[test]
    fn pipeline_enforces_amp_exclusivity() {
        let mut manager = PipelineManager::from_categories(&[
            category("amp-modeler", 10),
            category("amp-capture", 11),
        ])
        .expect("pipeline");

        manager
            .request_switch(
                "amp-modeler",
                "amp-model",
                double_factory as VariantFactory,
                48_000.0,
            )
            .expect("modeler switch");
        let err = manager
            .request_switch(
                "amp-capture",
                "amp-capture",
                double_factory as VariantFactory,
                48_000.0,
            )
            .expect_err("exclusive amp categories");

        assert!(err.to_string().contains("cannot both be active"));
    }

    #[test]
    fn pipeline_processes_non_bypassed_slots_in_order() {
        let mut manager =
            PipelineManager::from_categories(&[category("eq", 10)]).expect("pipeline");
        manager
            .request_switch("eq", "double", double_factory as VariantFactory, 48_000.0)
            .expect("switch");
        let mut buffer = [1.0, 2.0];

        manager.process_block(buffer.as_mut_ptr(), buffer.len());

        assert_eq!(buffer, [2.0, 4.0]);
    }

    #[test]
    fn pipeline_config_round_trips_slot_state() {
        let categories = [category("eq", 10), category("cab-sim", 20)];
        let mut manager = PipelineManager::from_categories(&categories).expect("pipeline");
        manager
            .request_switch("eq", "eq-a", double_factory as VariantFactory, 48_000.0)
            .expect("switch");
        manager.set_bypass("cab-sim", true).expect("bypass");
        let config = manager.to_config("default");

        let restored = PipelineManager::from_config(&categories, config).expect("restore");

        assert_eq!(restored.slots()[0].active_variant_id(), Some("eq-a"));
        assert!(restored.slots()[1].bypassed());
    }
}
