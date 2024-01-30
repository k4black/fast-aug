pub trait BaseAugmenter<T, K> {
    /// Public method to augment an input
    /// 1. Convert input to inner type (K)
    /// 2. Augment using inner type
    /// 3. Convert output to outer type (T)
    fn augment(&self, input: T, rng: &mut dyn rand::RngCore) -> T {
        let input = self.convert_to_inner(input);
        let output = self.augment_inner(input, rng);
        self.convert_to_outer(output)
    }

    fn augment_batch(&self, inputs: Vec<T>, rng: &mut dyn rand::RngCore) -> Vec<T> {
        // TODO: parallelize, not just loop
        // If less than X elements, just augment sequentially
        // Otherwise, parallelize
        inputs.into_iter().map(|input| self.augment(input, rng)).collect()
    }

    /// "Private" method to augment an input of inner type (K)
    fn augment_inner(&self, input: K, rng: &mut dyn rand::RngCore) -> K;

    fn convert_to_inner(&self, input: T) -> K;
    fn convert_to_outer(&self, input: K) -> T;
}
