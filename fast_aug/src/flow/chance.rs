use std::sync::Arc;
use rand::{Rng, thread_rng};
use crate::base::BaseAugmenter;


pub struct ChanceAugmenter<T> {
    /// The augmenter to apply with a given probability
    /// Added Send + Sync for multi-threading safety
    augmenter: Arc<dyn BaseAugmenter<T> + Send + Sync>,
    /// The probability of applying the augmenter
    probability: f64,
}


impl<T> ChanceAugmenter<T> {
    pub fn new(augmenter: Arc<dyn BaseAugmenter<T> + Send + Sync>, probability: f64) -> Self {
        ChanceAugmenter { augmenter, probability }
    }
}

impl<T> BaseAugmenter<T> for ChanceAugmenter<T> {
    fn augment(&self, input: T) -> T {
        if thread_rng().gen_bool(self.probability) {
            self.augmenter.augment(input)
        } else {
            input
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

    #[test]
    fn test_chance_augmenter_works_1_prob() {
        let augmenter = Arc::new(DummyMultiplyAugmenter);
        let chance_augmenter = ChanceAugmenter::new(augmenter, 1.0);

        let output = chance_augmenter.augment(1);

        assert_eq!(output, 2);
    }

    #[test]
    fn test_chance_augmenter_works_0_prob() {
        let augmenter = Arc::new(DummyMultiplyAugmenter);
        let chance_augmenter = ChanceAugmenter::new(augmenter, 0.0);

        let output = chance_augmenter.augment(1);

        assert_eq!(output, 1);
    }

    #[test_case(0.0, 1, 1 ; "0.0 probability")]
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
            let output = chance_augmenter.augment(input);
            num_changes += (output != input) as usize;
        }

        // Allow for some variance in the number of changes due to randomness
        assert!((num_changes as f64 / 1000.0 - p).abs() < 0.1);
    }
}