use crate::lab::DspVariant;
use std::collections::HashMap;

/// Factory function that creates a compiled DSP variant for a sample rate.
pub type VariantFactory = fn(sample_rate: f32) -> Box<dyn DspVariant>;

/// Registry mapping variant implementation ids to compiled DSP factories.
#[derive(Default)]
pub struct VariantRegistry {
    factories: HashMap<String, VariantFactory>,
}

impl VariantRegistry {
    /// Create an empty variant registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a factory by implementation id.
    pub fn register(&mut self, id: &str, factory: VariantFactory) {
        self.factories.insert(id.to_string(), factory);
    }

    /// Look up a factory by implementation id.
    pub fn get(&self, id: &str) -> Option<VariantFactory> {
        self.factories.get(id).copied()
    }

    /// Return registered implementation ids in deterministic order.
    pub fn ids(&self) -> Vec<&str> {
        let mut ids: Vec<_> = self.factories.keys().map(String::as_str).collect();
        ids.sort_unstable();
        ids
    }

    /// Return the number of registered factories.
    pub fn len(&self) -> usize {
        self.factories.len()
    }

    /// Return whether no factories are registered.
    pub fn is_empty(&self) -> bool {
        self.factories.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::{VariantFactory, VariantRegistry};
    use crate::lab::DspVariant;

    struct TestVariant {
        param_ids: [&'static str; 1],
    }

    impl DspVariant for TestVariant {
        fn process_block(&mut self, _buffer: *mut f32, _length: usize) {}

        fn param_count(&self) -> usize {
            self.param_ids.len()
        }

        fn param_ids(&self) -> &[&str] {
            &self.param_ids
        }

        fn latency(&self) -> usize {
            0
        }
    }

    fn factory(_sample_rate: f32) -> Box<dyn DspVariant> {
        Box::new(TestVariant {
            param_ids: ["gain"],
        })
    }

    #[test]
    fn registry_returns_registered_factory() {
        let mut registry = VariantRegistry::new();
        registry.register("test", factory as VariantFactory);

        let created = registry.get("test").expect("factory")(48_000.0);

        assert_eq!(created.param_count(), 1);
        assert_eq!(created.param_ids(), &["gain"]);
        assert!(registry.get("missing").is_none());
    }
}
