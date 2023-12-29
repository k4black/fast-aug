use std::sync::Arc;
use rand::{Rng, thread_rng};
use rand::distributions::{Distribution, WeightedIndex};
use crate::base::BaseAugmenter;

pub struct SelectorAugmenter<T> {
    /// The augmenters to choose one from
    /// Added Send + Sync for multi-threading safety
    augmenters: Vec<Arc<dyn BaseAugmenter<T> + Send + Sync>>,
    /// Optional weights for each augmenter
    /// If None, uniform probability is used
    weights: Option<Vec<f64>>,
}

impl<T> SelectorAugmenter<T> {
    pub fn new(augmenters: Vec<Arc<dyn BaseAugmenter<T> + Send + Sync>>, weights: Option<Vec<f64>>) -> Self {
        SelectorAugmenter { augmenters, weights }
    }
}

impl<T> BaseAugmenter<T> for SelectorAugmenter<T> {
    fn augment(&self, input: T) -> T {
        let mut rng = thread_rng();
        if let Some(weights) = &self.weights {
            let augmenter_index = WeightedIndex::new(weights).unwrap().sample(&mut rng);
            self.augmenters[augmenter_index].augment(input)
        } else {
            let augmenter_index = rng.gen_range(0..self.augmenters.len());
            self.augmenters[augmenter_index].augment(input)
        }

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
    fn test_selector_augmenter_works() {
        let augmenter1 = Arc::new(DummyMultiplyAugmenter);
        let augmenter2 = Arc::new(DummyAddAugmenter);
        let selector_augmenter = SelectorAugmenter::new(vec![augmenter1, augmenter2], None);

        let output = selector_augmenter.augment(1);

        assert_eq!(output, 2)
    }

    fn test_uniform_selection() {
        let augmenter1 = Arc::new(DummyMultiplyAugmenter);
        let augmenter2 = Arc::new(DummyAddAugmenter);
        let selector_augmenter = SelectorAugmenter::new(vec![augmenter1, augmenter2], None);

        // Calculate the number of times each augmenter is selected
        let mut num_augmenter1 = 0;
        let mut num_augmenter2 = 0;
        for _ in 0..1000 {
            let output = selector_augmenter.augment(10);
            if output == 20 {
                num_augmenter1 += 1;
            } else if output == 11 {
                num_augmenter2 += 1;
            }
        }

        // Allow for some variance in the number of changes due to randomness
        assert!((num_augmenter1 as f64 - num_augmenter2 as f64).abs() / 1000.0 < 0.1);
    }

    #[test_case(vec![0.0, 1.0] ; "0.0 and 1.0 weights")]
    #[test_case(vec![0.1, 0.9] ; "0.1 and 0.9 weights")]
    #[test_case(vec![0.5, 0.5] ; "0.5 and 0.5 weights")]
    #[test_case(vec![0.9, 0.1] ; "0.9 and 0.1 weights")]
    #[test_case(vec![1.0, 0.0] ; "1.0 and 0.0 weights")]
    #[test_case(vec![100.0, 100.0] ; "100.0 and 100.0 weights")]
    fn test_weighted_selection(weights: Vec<f64>) {
        let weights_normalized = weights.iter().map(|w| w / weights.iter().sum::<f64>()).collect::<Vec<f64>>();

        let augmenter1 = Arc::new(DummyMultiplyAugmenter);
        let augmenter2 = Arc::new(DummyAddAugmenter);
        let selector_augmenter = SelectorAugmenter::new(vec![augmenter1, augmenter2], Some(weights));

        // Calculate the number of times each augmenter is selected
        let mut num_augmenter1 = 0;
        let mut num_augmenter2 = 0;
        for _ in 0..1000 {
            let output = selector_augmenter.augment(10);
            if output == 20 {
                num_augmenter1 += 1;
            } else if output == 11 {
                num_augmenter2 += 1;
            }
        }

        // Allow for some variance in the number of changes due to randomness
        // Test only the first, as the second is the complement
        assert!((num_augmenter1 as f64 / 1000.0 - weights_normalized[0]).abs() < 0.1);
    }

    #[test_case(None)]
    #[test_case(Some(vec![0.5]))]
    #[test_case(Some(vec![1.0]))]
    #[test_case(Some(vec![100.0]))]
    fn test_only_one_augmenter(weights: Option<Vec<f64>>) {
        let augmenter1 = Arc::new(DummyMultiplyAugmenter);
        let selector_augmenter = SelectorAugmenter::new(vec![augmenter1], weights);

        let output = selector_augmenter.augment(1);

        assert_eq!(output, 2)
    }
}
