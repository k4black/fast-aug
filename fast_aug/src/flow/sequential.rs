use crate::base::BaseAugmenter;
use std::sync::Arc;

pub struct SequentialAugmenter<T, K> {
    /// The augmenters to apply in sequence
    /// Added Send + Sync for multi-threading safety
    augmenters: Vec<Arc<dyn BaseAugmenter<T, K> + Send + Sync>>,
}

impl<T, K> SequentialAugmenter<T, K> {
    pub fn new(augmenters: Vec<Arc<dyn BaseAugmenter<T, K> + Send + Sync>>) -> Self {
        if augmenters.is_empty() {
            panic!("SequentialAugmenter must have at least one augmenter");
        }
        SequentialAugmenter { augmenters }
    }
}

impl<T, K> BaseAugmenter<T, K> for SequentialAugmenter<T, K> {
    fn augment_inner(&self, input: K, rng: &mut dyn rand::RngCore) -> K {
        self.augmenters
            .iter()
            .fold(input, |acc, augmenter| augmenter.augment_inner(acc, rng))
    }

    fn convert_to_inner(&self, input: T) -> K {
        self.augmenters[0].convert_to_inner(input)
    }

    fn convert_to_outer(&self, input: K) -> T {
        self.augmenters[0].convert_to_outer(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    struct DummyMultiplyAugmenter;

    impl BaseAugmenter<i32, i32> for DummyMultiplyAugmenter {
        fn augment_inner(&self, input: i32, _rng: &mut dyn rand::RngCore) -> i32 {
            input * 2
        }
        fn convert_to_inner(&self, input: i32) -> i32 {
            input
        }
        fn convert_to_outer(&self, input: i32) -> i32 {
            input
        }
    }

    struct DummyAddAugmenter;

    impl BaseAugmenter<i32, i32> for DummyAddAugmenter {
        fn augment_inner(&self, input: i32, _rng: &mut dyn rand::RngCore) -> i32 {
            input + 1
        }
        fn convert_to_inner(&self, input: i32) -> i32 {
            input
        }
        fn convert_to_outer(&self, input: i32) -> i32 {
            input
        }
    }

    #[test]
    fn test_sequential_augmenter_works() {
        let augmenter1 = Arc::new(DummyMultiplyAugmenter);
        let augmenter2 = Arc::new(DummyAddAugmenter);
        let sequential_augmenter = SequentialAugmenter::new(vec![augmenter1, augmenter2]);

        let output = sequential_augmenter.augment(1, &mut rand::thread_rng());

        assert_eq!(output, 3);
    }

    #[test]
    fn test_sequential_augmenter_works_with_multiple_augmenters() {
        let augmenter1 = Arc::new(DummyMultiplyAugmenter);
        let augmenter2 = Arc::new(DummyAddAugmenter);
        let augmenter3 = Arc::new(DummyMultiplyAugmenter);
        let sequential_augmenter = SequentialAugmenter::new(vec![augmenter1, augmenter2, augmenter3]);

        let output = sequential_augmenter.augment(1, &mut rand::thread_rng());

        assert_eq!(output, 6);
    }
}
