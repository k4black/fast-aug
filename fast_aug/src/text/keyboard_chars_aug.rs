#![allow(dead_code, unused_variables, unreachable_code, unused_imports)]

use super::base::{BaseTextAugmenter, TextAction};
use super::doc::Doc;
use super::parameters::TextAugmentParameters;
use crate::base::BaseAugmenter;
use std::collections::HashSet;

pub struct KeyboardCharsAugmenter {
    /// Action to augmentation, set of values {'substitute', 'swap', 'delete'}
    action: TextAction,
    /// Parameters to calculate number of words that will be augmented
    aug_params_word: TextAugmentParameters,
    /// Parameters to calculate number of chars that will be augmented in each word
    aug_params_char: TextAugmentParameters,
    /// Filter, Set of words that cannot be augmented
    stopwords: Option<HashSet<String>>,
}

impl KeyboardCharsAugmenter {
    pub fn new(
        action: TextAction,
        aug_params_word: TextAugmentParameters,
        aug_params_char: TextAugmentParameters,
        stopwords: Option<HashSet<String>>,
    ) -> Self {
        KeyboardCharsAugmenter {
            action,
            aug_params_word,
            aug_params_char,
            stopwords,
        }
    }

    fn substitute(&self, mut _doc: Doc, _rng: &mut dyn rand::RngCore) -> Doc {
        todo!();
    }

    fn swap(&self, mut doc: Doc, rng: &mut dyn rand::RngCore) -> Doc {
        // TODO: adjacent, middle, random swaps (now only random)
        // Select random word tokens
        let word_tokens_indexes = doc.get_word_indexes(false, self.stopwords.as_ref());
        let num_tokens_to_change = self.aug_params_word.num_elements(word_tokens_indexes.len());
        let selected_tokens_indexes =
            self.select_random_element_indexes(rng, word_tokens_indexes, num_tokens_to_change);

        // For all selected tokens select random chars and swap them
        for token_index in selected_tokens_indexes {
            todo!();

            doc.num_changes += 1;
        }

        doc
    }
}

impl BaseTextAugmenter for KeyboardCharsAugmenter {}

impl BaseAugmenter<String, Doc> for KeyboardCharsAugmenter {
    fn augment_inner(&self, input: Doc, rng: &mut dyn rand::RngCore) -> Doc {
        match self.action {
            TextAction::Substitute => self.substitute(input, rng),
            TextAction::Swap => self.swap(input, rng),
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
    use crate::text::random_words_aug::RandomWordsAugmenter;
    use test_case::test_case;
}
