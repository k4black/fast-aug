use std::sync::Arc;
use crate::base::BaseAugmenter;

pub struct SequentialAugmenter<T> {
    /// The augmenters to apply in sequence
    /// Added Send + Sync for multi-threading safety
    augmenters: Vec<Arc<dyn BaseAugmenter<T> + Send + Sync>>,
}

impl<T> SequentialAugmenter<T> {
    pub fn new(augmenters: Vec<Arc<dyn BaseAugmenter<T> + Send + Sync>>) -> Self {
        SequentialAugmenter { augmenters }
    }
}

impl<T> BaseAugmenter<T> for SequentialAugmenter<T> {
    fn augment(&self, input: T) -> T {
        self.augmenters.iter().fold(input, |acc, augmenter| augmenter.augment(acc))
    }
}


#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::*;

    struct DummyMultiplyAugmenter;

    impl BaseAugmenter<i32> for DummyMultiplyAugmenter {
        fn augment(&self, input: i32) -> i32 {
            input * 2
        }
    }

    struct DummyAddAugmenter;

    impl BaseAugmenter<i32> for DummyAddAugmenter {
        fn augment(&self, input: i32) -> i32 {
            input + 1
        }
    }

    #[test]
    fn test_sequential_augmenter_works() {
        let augmenter1 = Arc::new(DummyMultiplyAugmenter);
        let augmenter2 = Arc::new(DummyAddAugmenter);
        let sequential_augmenter = SequentialAugmenter::new(vec![augmenter1, augmenter2]);

        let output = sequential_augmenter.augment(1);

        assert_eq!(output, 3);
    }

    #[test]
    fn test_sequential_augmenter_works_with_no_augmenters() {
        let sequential_augmenter = SequentialAugmenter::new(vec![]);

        let output = sequential_augmenter.augment(1);

        assert_eq!(output, 1);
    }

    #[test]
    fn test_sequential_augmenter_works_with_multiple_augmenters() {
        let augmenter1 = Arc::new(DummyMultiplyAugmenter);
        let augmenter2 = Arc::new(DummyAddAugmenter);
        let augmenter3 = Arc::new(DummyMultiplyAugmenter);
        let sequential_augmenter = SequentialAugmenter::new(vec![augmenter1, augmenter2, augmenter3]);

        let output = sequential_augmenter.augment(1);

        assert_eq!(output, 6);
    }
}
