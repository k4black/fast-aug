use crate::base::BaseAugmenter;
use rand::Rng;
use std::sync::Arc;

pub struct ChanceAugmenter<T, K> {
    /// The augmenter to apply with a given probability
    /// Added Send + Sync for multi-threading safety
    augmenter: Arc<dyn BaseAugmenter<T, K> + Send + Sync>,
    /// The probability of applying the augmenter
    probability: f64,
}

impl<T, K> ChanceAugmenter<T, K> {
    pub fn new(augmenter: Arc<dyn BaseAugmenter<T, K> + Send + Sync>, probability: f64) -> Self {
        ChanceAugmenter { augmenter, probability }
    }
}

impl<T, K> BaseAugmenter<T, K> for ChanceAugmenter<T, K> {
    fn augment_inner(&self, input: K, rng: &mut dyn rand::RngCore) -> K {
        if rng.gen_bool(self.probability) {
            self.augmenter.augment_inner(input, rng)
        } else {
            input
        }
    }

    fn convert_to_inner(&self, input: T) -> K {
        self.augmenter.convert_to_inner(input)
    }

    fn convert_to_outer(&self, input: K) -> T {
        self.augmenter.convert_to_outer(input)
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

    #[test]
    fn test_chance_augmenter_works_1_prob() {
        let augmenter = Arc::new(DummyMultiplyAugmenter);
        let chance_augmenter = ChanceAugmenter::new(augmenter, 1.0);

        let output = chance_augmenter.augment(1, &mut rand::thread_rng());

        assert_eq!(output, 2);
    }

    #[test]
    fn test_chance_augmenter_works_0_prob() {
        let augmenter = Arc::new(DummyMultiplyAugmenter);
        let chance_augmenter = ChanceAugmenter::new(augmenter, 0.0);

        let output = chance_augmenter.augment(1, &mut rand::thread_rng());

        assert_eq!(output, 1);
    }

    #[test_case(0.0, 1, 2 ; "0.0 probability")]
    #[test_case(0.1, 1, 2 ; "0.1 probability")]
    #[test_case(0.5, 1, 2 ; "0.5 probability")]
    #[test_case(0.9, 1, 2 ; "0.9 probability")]
    #[test_case(1.0, 1, 2 ; "1.0 probability")]
    fn test_probability_works(p: f64, input: i32, expected_output: i32) {
        let augmenter = Arc::new(DummyMultiplyAugmenter);
        let chance_augmenter = ChanceAugmenter::new(augmenter, p);

        // Calculate the number of times the augmenter changes the input
        let mut num_changes = 0;
        for _ in 0..1000 {
            let output = chance_augmenter.augment(input, &mut rand::thread_rng());
            num_changes += (output == expected_output) as usize;
        }

        // Allow for some variance in the number of changes due to randomness
        assert!((num_changes as f64 / 1000.0 - p).abs() < 0.1);
    }
}
