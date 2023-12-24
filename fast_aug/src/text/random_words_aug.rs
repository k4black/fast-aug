use std::collections::HashSet;
use std::sync::Arc;
use rand::prelude::{IteratorRandom, SliceRandom};
use rand::seq::SliceChooseIter;
use rand::thread_rng;
use crate::base::BaseAugmenter;
use crate::text::{Doc, TextAugmentParameters, Token, TokenType};
use super::base::{BaseTextAugmenter, TextAction};


pub struct RandomWordAugmenter {
    /// Action to augmentation, set of values {'substitute', 'swap', 'delete'}
    action: TextAction,
    /// Parameters to calculate number of words that will be augmented
    aug_params_word: TextAugmentParameters,
    /// Filter, Set of words that cannot be augmented
    stopwords: Option<HashSet<String>>,
}


impl RandomWordAugmenter {
    pub fn new(
        action: TextAction,
        aug_params_word: TextAugmentParameters,
        stopwords: Option<HashSet<String>>,
    ) -> Self {
        RandomWordAugmenter {
            action,
            aug_params_word,
            stopwords,
        }
    }
}


impl BaseTextAugmenter for RandomWordAugmenter {
    fn action(&self) -> &TextAction {
        &self.action
    }

    // Not applicable
    fn insert(&self, mut doc: Doc) -> Doc {
        panic!("Insert action is not applicable for RandomWordAugmenter");
    }

    fn delete(&self, mut doc: Doc) -> Doc {
        // Select random word tokens
        let word_tokens_indexes = doc.get_word_indexes(false, self.stopwords.as_ref());
        let selected_tokens_indexes = self.aug_params_word.select_random_element_indexes(word_tokens_indexes, false);

        // For all selected tokens set TokenType::Deleted
        let mut num_changes = 0;
        for index in selected_tokens_indexes {
            doc.tokens[index].change("", TokenType::Deleted);
            num_changes += 1;
        }
        doc.num_changes += num_changes;

        doc
    }

    fn substitute(&self, mut doc: Doc) -> Doc {
        panic!("Insert action is not applicable for RandomWordAugmenter");
    }

    fn swap(&self, mut doc: Doc) -> Doc {
        // Select random word tokens (shuffle selected tokens to make swaps)
        let word_tokens_indexes = doc.get_word_indexes(false, self.stopwords.as_ref());
        let selected_tokens_indexes = self.aug_params_word.select_random_element_indexes(word_tokens_indexes, true);

        // For all selected tokens swap pairs
        // As shuffled we can swap adjacent pairs (using chunks)
        let mut num_changes = 0;
        for idxes in selected_tokens_indexes.chunks(2) {
            let idx_a = idxes.first().unwrap();
            let idx_b = idxes.last().unwrap();
            doc.swap_tokens_by_index(*idx_a, *idx_b);
            num_changes += 1;
        }

        // If odd number of tokens, swap last with first
        if selected_tokens_indexes.len() % 2 != 0 {
            let last_idx = selected_tokens_indexes.last().unwrap();
            let first_idx = selected_tokens_indexes.first().unwrap();
            doc.swap_tokens_by_index(*last_idx, *first_idx);
            num_changes += 1;
        }
        doc.num_changes += num_changes;

        doc
    }
}


impl BaseAugmenter<String> for RandomWordAugmenter {
    fn augment(&self, input: String) -> String {
        BaseTextAugmenter::augment(self, input)
    }
}


#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::*;

    #[test_case(vec!["A", "B", "C", "D", "E"], 0.5, 3, 3)]
    #[test_case(vec!["A", "B", "C", "D", "E", "D"], 0.5, 3, 3)]
    #[test_case(vec!["\t", "B", " ", "D", "!"], 0.5, 1, 1)]
    #[test_case(vec!["\t", "B", " ", "D", "!"], 0.1, 1, 1)]
    #[test_case(vec!["\t", "B", " ", "D", "!"], 0.0, 0, 0)]
    #[test_case(vec!["\t", "!", " ", "-", "!"], 0.5, 0, 0)]
    fn test_delete(input_tokens: Vec<&str>, p: f32, expected_deleted_tokens: usize, expected_doc_changes: usize) {
        let mut doc = Doc::from_tokens(input_tokens);
        let params = TextAugmentParameters::new(p, None, None);
        let aug = RandomWordAugmenter::new(TextAction::Delete, params, None);

        let doc_tokens_before = doc.tokens.clone();

        doc = aug.delete(doc);

        let doc_tokens_after = doc.tokens.clone();

        if expected_doc_changes == 0 {
            assert_eq!(doc_tokens_before, doc_tokens_after);
        } else {
            assert_eq!(doc_tokens_before.len(), doc_tokens_after.len());
            assert_ne!(doc_tokens_before, doc_tokens_after);
            assert_eq!(doc.num_changes, expected_doc_changes);
            assert_eq!(doc_tokens_after.iter().filter(|token| token.kind() == &TokenType::Deleted).count(), expected_deleted_tokens);
        }
    }

    #[test_case(vec!["A", "B", "C", "D", "E"], 0.5, 3)]
    #[test_case(vec!["A", "B", "C", "D", "E", "D"], 0.5, 3)]
    #[test_case(vec!["\t", "B", " ", "D", "!"], 0.5, 0)]
    #[test_case(vec!["\t", "B", " ", "D", "!"], 0.1, 0)]
    fn test_swap(input_tokens: Vec<&str>, p: f32, expected_doc_changes: usize) {
        let mut doc = Doc::from_tokens(input_tokens);
        let params = TextAugmentParameters::new(p, None, None);
        let aug = RandomWordAugmenter::new(TextAction::Swap, params, None);

        let doc_tokens_before = doc.tokens.clone();

        doc = aug.swap(doc);

        let doc_tokens_after = doc.tokens.clone();

        if expected_doc_changes == 0 {
            assert_eq!(doc_tokens_before, doc_tokens_after);
        } else {
            assert_eq!(doc_tokens_before.len(), doc_tokens_after.len());
            assert_ne!(doc_tokens_before, doc_tokens_after);
            assert_eq!(doc.num_changes, expected_doc_changes);
        }
    }

}

