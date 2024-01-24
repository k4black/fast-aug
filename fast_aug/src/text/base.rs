use super::doc::Doc;
use super::token::Token;
use crate::base::BaseAugmenter;
use rand::prelude::IteratorRandom;
use rand::seq::SliceRandom;
use std::collections::HashSet;

/// Actions enum, which action BaseTextAugmenter will perform
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TextAction {
    Insert,
    Delete,
    Substitute,
    Swap,
}

impl std::fmt::Display for TextAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextAction::Insert => write!(f, "Insert"),
            TextAction::Delete => write!(f, "Delete"),
            TextAction::Substitute => write!(f, "Substitute"),
            TextAction::Swap => write!(f, "Swap"),
        }
    }
}

pub trait BaseTextAugmenter: BaseAugmenter<String, Doc> {
    /// Select random word tokens to augment given a number of elements
    /// Returns a vector of tuples (index, &mut token)
    /// TODO: optimize this function
    fn select_word_tokens_to_augment<'a>(
        &self,
        rng: &mut dyn rand::RngCore,
        candidate_tokens: &'a mut Vec<(usize, &'a mut Token)>,
        num_elements: usize,
        stopwords: Option<&HashSet<String>>,
    ) -> Vec<(usize, &'a mut Token)> {
        let mut filtered_tokens: Vec<(usize, &mut Token)> = Vec::with_capacity(candidate_tokens.len());

        for (idx, token) in candidate_tokens {
            let token_str = token.token();

            // Skip stopwords
            if let Some(stopwords) = stopwords {
                if stopwords.contains(token_str) {
                    continue;
                }
            }

            filtered_tokens.push((*idx, token));
        }

        // Return all if len<num_elements
        if filtered_tokens.len() <= num_elements {
            return filtered_tokens;
        }

        // Return random subset of num_elements len
        filtered_tokens.into_iter().choose_multiple(rng, num_elements)
    }

    /// Select random elements to augment.
    /// Returns a vector of indexes of elements to be augmented.
    /// Automatically shuffled.
    ///
    /// # Arguments
    /// * `element_indexes` - A vector of indexes of elements to be augmented.
    fn select_random_element_indexes(
        &self,
        rng: &mut dyn rand::RngCore,
        element_indexes: Vec<usize>,
        num_elements: usize,
    ) -> Vec<usize> {
        // If the number of requested elements is larger than available,
        // return the whole array to avoid panicking.
        if num_elements >= element_indexes.len() {
            return element_indexes;
        }

        // Randomly select indexes from the input vector
        let selected_elements: Vec<usize> = element_indexes.choose_multiple(rng, num_elements).cloned().collect();

        selected_elements
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    // #[test_case(0.5, 10, 5)]
    // #[test_case(0.7, 10, 7)]
    // #[test_case(0.3, 10, 3)]
    // #[test_case(0.5, 0, 0)]
    // #[test_case(0.5, 100, 50)]
    // fn test_select_random_element_indexes(p: f32, input_size: usize, expected_len: usize) {
    //     let params = TextAugmentParameters::new(p, None, None);
    //     let element_indexes = (0..input_size).collect::<Vec<usize>>();
    //     let selected_indexes = params.select_random_element_indexes(element_indexes);
    //     assert_eq!(selected_indexes.len(), expected_len);
    // }
    //
    // #[test_case(0.5, 10, 5)]
    // #[test_case(0.7, 10, 7)]
    // #[test_case(0.3, 10, 3)]
    // #[test_case(0.5, 0, 0)]
    // #[test_case(0.5, 100, 50)]
    // fn test_select_random_element_indexes_shuffle(p: f32, input_size: usize, expected_len: usize) {
    //     let params = TextAugmentParameters::new(p, None, None);
    //     let element_indexes = (0..input_size).collect::<Vec<usize>>();
    //     let selected_indexes = params.select_random_element_indexes(element_indexes);
    //     assert_eq!(selected_indexes.len(), expected_len);
    // }
}
