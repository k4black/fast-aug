use rand::seq::SliceRandom;

/// Parameters for augmentation
#[derive(Clone)]
pub struct TextAugmentParameters {
    /// Probability of augmentation of single element
    /// Not true probability, but expected percent of elements to be augmented
    /// If not set fall back to default value of 0.3
    pub p: f32,
    /// Minimum elements for augmentation
    /// If not set - no limit
    pub min_elements: Option<usize>,
    /// Maximum elements for augmentation
    /// If not set - no limit
    pub max_elements: Option<usize>,
}

impl Default for TextAugmentParameters {
    fn default() -> Self {
        TextAugmentParameters {
            p: 0.3,
            min_elements: None,
            max_elements: None,
        }
    }
}

impl TextAugmentParameters {
    /// Create new AugmentParameters
    /// # Arguments
    /// * `p` - Probability of augmentation of single element (expected percent of elements to be augmented)
    /// * `min_elements` - Minimum elements for augmentation
    /// * `max_elements` - Maximum elements for augmentation
    pub fn new(p: f32, min_elements: Option<usize>, max_elements: Option<usize>) -> Self {
        TextAugmentParameters {
            p,
            min_elements,
            max_elements,
        }
    }

    /// Calculate number of elements to be augmented from input size
    ///
    /// # Arguments
    /// * `input_size` - Size of input in elements (words, chars, etc.)
    ///
    /// # Examples
    /// ```rust
    /// use fast_aug::text::TextAugmentParameters;
    ///
    /// let params = TextAugmentParameters{p: 0.5, min_elements: None, max_elements: None};
    /// // Apply only p
    /// assert_eq!(params.num_elements(10), 5);
    ///
    /// let params = TextAugmentParameters{p: 0.5, min_elements: Some(3), max_elements: Some(7)};
    /// // Apply min_elements
    /// assert_eq!(params.num_elements(4), 3);
    /// // Apply min_elements, but reach input_size, so return input_size
    /// assert_eq!(params.num_elements(1), 1);
    /// // Apply max_elements
    /// assert_eq!(params.num_elements(100), 7);
    /// ```
    pub fn num_elements(&self, input_size: usize) -> usize {
        if input_size == 0 || self.p <= 0.0 {
            return 0;
        }

        let prob_num_elements = f32::ceil(self.p * input_size as f32) as usize;

        if let Some(min_elements) = self.min_elements {
            if prob_num_elements < min_elements {
                if input_size < min_elements {
                    return input_size;
                }
                return min_elements;
            }
        }
        if let Some(max_elements) = self.max_elements {
            if prob_num_elements > max_elements {
                return max_elements;
            }
        }
        prob_num_elements
    }

    /// Select random elements to augment.
    /// Returns a vector of indexes of elements to be augmented.
    /// Automatically shuffled.
    ///
    /// # Arguments
    /// * `element_indexes` - A vector of indexes of elements to be augmented.
    pub fn select_random_element_indexes(
        &self,
        element_indexes: Vec<usize>,
    ) -> Vec<usize> {
        let mut rng = rand::thread_rng();

        // Calculate number of elements to be augmented using p, min_elements, max_elements
        let num_elements = self.num_elements(element_indexes.len());

        // If the number of requested elements is larger than available,
        // return the whole array to avoid panicking.
        if num_elements >= element_indexes.len() {
            return element_indexes;
        }

        // Randomly select indexes from the input vector
        let mut selected_elements: Vec<usize> = element_indexes
            .choose_multiple(&mut rng, num_elements)
            .cloned()
            .collect();

        selected_elements
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::*;

    #[test_case(0.0, 10, 0)]
    #[test_case(0.5, 10, 5)]
    #[test_case(1.0, 10, 10)]
    #[test_case(0.5, 0, 0)]
    #[test_case(0.5, 1, 1)]
    fn test_num_elements_only_p(p: f32, input_size: usize, expected: usize) {
        let params = TextAugmentParameters::new(p, None, None);
        assert_eq!(params.num_elements(input_size), expected);
    }

    #[test]
    fn test_num_elements_defaults() {
        let params = TextAugmentParameters::default();
        assert_eq!(params.num_elements(10), 3);
    }

    #[test_case(0.5, 10, 5, 5)]
    #[test_case(0.5, 10, 7, 7)]
    #[test_case(0.5, 10, 10, 10)]
    #[test_case(0.5, 10, 1, 5)]
    #[test_case(0.5, 10, 0, 5)]
    fn test_num_elements_min_limit(p: f32, input_size: usize, min_elements: usize, expected: usize) {
        let params = TextAugmentParameters::new(p, Some(min_elements), None);
        assert_eq!(params.num_elements(input_size), expected);
    }

    #[test]
    fn test_num_elements_min_limit_more_than_input() {
        let params = TextAugmentParameters::new(0.5, Some(10), None);
        assert_eq!(params.num_elements(5), 5);
    }

    #[test_case(0.5, 10, 5, 5)]
    #[test_case(0.5, 10, 7, 5)]
    #[test_case(0.5, 10, 10, 5)]
    #[test_case(0.5, 10, 1, 1)]
    #[test_case(0.5, 10, 0, 0)]
    fn test_num_elements_max_limit(p: f32, input_size: usize, max_elements: usize, expected: usize) {
        let params = TextAugmentParameters::new(p, None, Some(max_elements));
        assert_eq!(params.num_elements(input_size), expected);
    }

    // TODO: test min > max
    #[test_case(0.5, 10, 5, 5, 5)]
    #[test_case(0.5, 10, 3, 3, 3)]
    #[test_case(0.5, 10, 7, 7, 7)]
    #[test_case(0.5, 10, 3, 7, 5)]
    #[test_case(0.5, 10, 0, 3, 3)]
    #[test_case(0.5, 10, 6, 10, 6)]
    fn test_num_elements_min_max_limit(p: f32, input_size: usize, min_elements: usize, max_elements: usize, expected: usize) {
        let params = TextAugmentParameters::new(p, Some(min_elements), Some(max_elements));
        assert_eq!(params.num_elements(input_size), expected);
    }

    #[test_case(0.5, 10, 5)]
    #[test_case(0.7, 10, 7)]
    #[test_case(0.3, 10, 3)]
    #[test_case(0.5, 0, 0)]
    #[test_case(0.5, 100, 50)]
    fn test_select_random_element_indexes(p: f32, input_size: usize, expected_len: usize) {
        let params = TextAugmentParameters::new(p, None, None);
        let element_indexes = (0..input_size).collect::<Vec<usize>>();
        let selected_indexes = params.select_random_element_indexes(element_indexes);
        assert_eq!(selected_indexes.len(), expected_len);
    }

    #[test_case(0.5, 10, 5)]
    #[test_case(0.7, 10, 7)]
    #[test_case(0.3, 10, 3)]
    #[test_case(0.5, 0, 0)]
    #[test_case(0.5, 100, 50)]
    fn test_select_random_element_indexes_shuffle(p: f32, input_size: usize, expected_len: usize) {
        let params = TextAugmentParameters::new(p, None, None);
        let element_indexes = (0..input_size).collect::<Vec<usize>>();
        let selected_indexes = params.select_random_element_indexes(element_indexes);
        assert_eq!(selected_indexes.len(), expected_len);
    }
}
