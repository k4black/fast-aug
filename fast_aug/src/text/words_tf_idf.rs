#![allow(dead_code, unused_variables, unreachable_code, unused_imports)]

use super::base::{BaseTextAugmenter, TextAction};
use super::utils::{Doc, TextAugmentParameters};
use crate::base::BaseAugmenter;
use std::collections::HashSet;

pub struct WordsTfIdfAugmenter {
    /// Action to augmentation, set of values {'substitute', 'swap', 'delete'}
    action: TextAction,
    /// Parameters to calculate number of words that will be augmented
    word_params: TextAugmentParameters,
    /// Filter, Set of words that cannot be augmented
    stopwords: Option<HashSet<String>>,
    /// top k similar words to substitute
    top_k: usize,
}

impl WordsTfIdfAugmenter {
    pub fn new(word_params: TextAugmentParameters, stopwords: Option<HashSet<String>>, top_k: usize) -> Self {
        WordsTfIdfAugmenter {
            action: TextAction::Substitute,
            word_params,
            stopwords,
            top_k,
        }
    }

    fn substitute(&self, mut doc: Doc, rng: &mut dyn rand::RngCore) -> Doc {
        // Select random word tokens
        let word_tokens_indexes = doc.get_word_indexes(false, self.stopwords.as_ref());
        let num_tokens_to_change = self.word_params.num_elements(word_tokens_indexes.len());
        let selected_tokens_indexes =
            self.select_random_element_indexes(rng, word_tokens_indexes, num_tokens_to_change);

        // For all selected tokens randomly select one word with closest tf-idf and substitute
        for index in selected_tokens_indexes {
            todo!();
            doc.num_changes += 1;
        }

        doc
    }
}

impl BaseTextAugmenter for WordsTfIdfAugmenter {}

impl BaseAugmenter<String, Doc> for WordsTfIdfAugmenter {
    fn augment_inner(&self, input: Doc, rng: &mut dyn rand::RngCore) -> Doc {
        match self.action {
            TextAction::Substitute => self.substitute(input, rng),
            _ => panic!("Action not implemented"),
        }
    }

    fn convert_to_inner(&self, input: String) -> Doc {
        Doc::new(&input)
    }

    fn convert_to_outer(&self, input: Doc) -> String {
        input.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
}
