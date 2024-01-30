use super::base::{BaseTextAugmenter, TextAction};
use super::utils::{Doc, TextAugmentParameters, TokenType};
use crate::base::BaseAugmenter;
use crate::text::Token;
use rand::prelude::SliceRandom;
use std::collections::HashSet;

pub struct WordsRandomAugmenter {
    /// Action to augmentation, set of values {'insert', 'substitute', 'swap', 'delete'}
    action: TextAction,
    /// Parameters to calculate number of words that will be augmented
    aug_params_word: TextAugmentParameters,
    /// Filter, Set of words that cannot be augmented
    stopwords: Option<HashSet<String>>,
    /// For substitute and insert actions, set of words that can be used to replace or insert
    /// If None, insert or substitute can not be used
    vocabulary: Option<Vec<String>>,
}

impl WordsRandomAugmenter {
    pub fn new(
        action: TextAction,
        aug_params_word: TextAugmentParameters,
        stopwords: Option<HashSet<String>>,
        vocabulary: Option<Vec<String>>,
    ) -> Self {
        // Check vocabulary is not None if action is insert or substitute
        if action == TextAction::Insert || action == TextAction::Substitute {
            assert!(
                vocabulary.is_some(),
                "Vocabulary must be provided for insert or substitute actions"
            );
        }

        WordsRandomAugmenter {
            action,
            aug_params_word,
            stopwords,
            vocabulary,
        }
    }

    fn insert(&self, mut doc: Doc, rng: &mut dyn rand::RngCore) -> Doc {
        // Select random places to insert tokens from 0 (before the first token) to len inclusive (after last token)
        let num_tokens_to_insert = self.aug_params_word.num_elements(doc.tokens.len());
        let mut selected_places_to_insert_indexes =
            self.select_random_element_indexes(rng, (0..=doc.tokens.len()).collect(), num_tokens_to_insert);
        selected_places_to_insert_indexes.sort();

        // Select random words from vocabulary - with replacement
        let mut tokens_to_insert: Vec<Token> = (0..num_tokens_to_insert)
            .map(|_| {
                self.vocabulary
                    .as_ref()
                    .unwrap()
                    .choose(rng)
                    .map(|word| Token::from_str(word))
                    .unwrap()
            })
            .collect();
        assert_eq!(selected_places_to_insert_indexes.len(), tokens_to_insert.len());

        // Insert tokens in selected places
        // Go through all positions to insert, if have original doc token insert it else insert new token
        let mut new_tokens: Vec<Token> = Vec::with_capacity(doc.tokens.len() + tokens_to_insert.len());
        let mut current_doc_index = 0;
        for place_to_insert_index in selected_places_to_insert_indexes {
            // Add tokens until current place
            while current_doc_index < place_to_insert_index {
                new_tokens.push(doc.tokens[current_doc_index].clone());
                current_doc_index += 1;
            }
            // If current place < than current doc index, add inserted token
            if place_to_insert_index <= current_doc_index {
                new_tokens.push(tokens_to_insert.pop().unwrap());
            }
        }
        // Insert remaining tokens
        while current_doc_index < doc.tokens.len() {
            new_tokens.push(doc.tokens[current_doc_index].clone());
            current_doc_index += 1;
        }
        doc.num_changes = num_tokens_to_insert;
        doc.tokens = new_tokens;

        doc
    }

    fn substitute(&self, mut doc: Doc, rng: &mut dyn rand::RngCore) -> Doc {
        // Select random word tokens
        let word_tokens_indexes = doc.get_word_indexes(false, self.stopwords.as_ref());
        let num_tokens_to_change = self.aug_params_word.num_elements(word_tokens_indexes.len());
        let selected_tokens_indexes =
            self.select_random_element_indexes(rng, word_tokens_indexes, num_tokens_to_change);

        // Select random words from vocabulary - with replacement
        let tokens_to_insert: Vec<Token> = (0..num_tokens_to_change)
            .map(|_| {
                self.vocabulary
                    .as_ref()
                    .unwrap()
                    .choose(rng)
                    .map(|word| Token::from_str(word))
                    .unwrap()
            })
            .collect();
        assert_eq!(selected_tokens_indexes.len(), tokens_to_insert.len());

        // Substitute tokens in selected places
        for (index, token) in selected_tokens_indexes.into_iter().zip(tokens_to_insert.into_iter()) {
            doc.tokens[index] = token;
            doc.num_changes += 1;
        }

        doc
    }

    fn delete(&self, mut doc: Doc, rng: &mut dyn rand::RngCore) -> Doc {
        // Select random word tokens
        let word_tokens_indexes = doc.get_word_indexes(false, self.stopwords.as_ref());
        let num_tokens_to_change = self.aug_params_word.num_elements(word_tokens_indexes.len());
        let selected_tokens_indexes =
            self.select_random_element_indexes(rng, word_tokens_indexes, num_tokens_to_change);

        // For all selected tokens set TokenType::Deleted
        for index in selected_tokens_indexes {
            doc.tokens[index].change("", TokenType::Deleted);
            doc.num_changes += 1;
        }

        doc
    }

    fn swap(&self, mut doc: Doc, rng: &mut dyn rand::RngCore) -> Doc {
        // Select random word tokens (shuffle selected tokens to make swaps)
        let word_tokens_indexes = doc.get_word_indexes(false, self.stopwords.as_ref());
        let num_tokens_to_change = self.aug_params_word.num_elements(word_tokens_indexes.len());
        let selected_tokens_indexes =
            self.select_random_element_indexes(rng, word_tokens_indexes, num_tokens_to_change);

        // For all selected tokens swap pairs
        // As shuffled we can swap adjacent pairs (using chunks)
        for idxes in selected_tokens_indexes.chunks(2) {
            let idx_a = idxes.first().unwrap();
            let idx_b = idxes.last().unwrap();
            doc.swap_tokens_by_index(*idx_a, *idx_b);
            doc.num_changes += 1;
        }

        // If odd number of tokens, swap last with first
        if selected_tokens_indexes.len() % 2 != 0 {
            let last_idx = selected_tokens_indexes.last().unwrap();
            let first_idx = selected_tokens_indexes.first().unwrap();
            doc.swap_tokens_by_index(*last_idx, *first_idx);
            doc.num_changes += 1;
        }

        doc
    }
}

impl BaseTextAugmenter for WordsRandomAugmenter {}

impl BaseAugmenter<String, Doc> for WordsRandomAugmenter {
    #[allow(unreachable_patterns)]
    fn augment_inner(&self, input: Doc, rng: &mut dyn rand::RngCore) -> Doc {
        match self.action {
            TextAction::Insert => self.insert(input, rng),
            TextAction::Substitute => self.substitute(input, rng),
            TextAction::Delete => self.delete(input, rng),
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
    use test_case::test_case;

    #[test_case(vec!["A", "B", "C", "D", "E"], vec!["A", "T", "K"], 0.5, 8, 3 ; "round 2.5 as +3")]
    #[test_case(vec!["A", "B", "C", "D", "E", "D"], vec!["A", "T", "K"], 0.5, 9, 3 ; "3 of 6")]
    #[test_case(vec!["\t", "B", " ", "D", "!"], vec!["A", "T", "K"], 0.2, 6, 1 ; "1 word")]
    #[test_case(vec!["\t", "B", " ", "D", "!"], vec!["A", "T", "K"], 0.1, 6, 1 ; "round 0.05 as 1 word")]
    #[test_case(vec!["\t", "B", " ", "D", "!"], vec!["A", "T", "K"], 0.0, 5, 0 ; "insert probability=0")]
    fn test_insert(
        input_tokens: Vec<&str>,
        vocab: Vec<&str>,
        p: f32,
        expected_len: usize,
        expected_doc_changes: usize,
    ) {
        let mut doc = Doc::from_tokens(input_tokens);
        let params = TextAugmentParameters::new(p, None, None);
        let vocab = vocab.into_iter().map(|s| s.to_string()).collect();
        let aug = WordsRandomAugmenter::new(TextAction::Insert, params, None, Some(vocab));

        let doc_tokens_before = doc.tokens.clone();

        doc = aug.insert(doc, &mut rand::thread_rng());

        let doc_tokens_after = doc.tokens.clone();

        assert_eq!(doc_tokens_after.len(), expected_len);
        if expected_doc_changes == 0 {
            assert_eq!(doc_tokens_before, doc_tokens_after);
        } else {
            assert_ne!(doc_tokens_before, doc_tokens_after);
            assert_eq!(doc.num_changes, expected_doc_changes);
        }
    }

    #[test_case(vec!["A", "B", "C", "D", "E"], vec!["A", "T", "K"], 0.5, 3 ; "round 2.5 as 3 of 5")]
    #[test_case(vec!["A", "B", "C", "D", "E", "D"], vec!["A", "T", "K"], 0.5, 3 ; "3 of 6")]
    #[test_case(vec!["\t", "B", " ", "D", "!"], vec!["A", "T", "K"], 0.5, 1 ; "1 word")]
    #[test_case(vec!["\t", "B", " ", "D", "!"], vec!["A", "T", "K"], 0.1, 1 ; "round 0.05 as 1 word")]
    #[test_case(vec!["\t", "B", " ", "D", "!"], vec!["A", "T", "K"], 0.0, 0 ; "substitute probability=0")]
    #[test_case(vec!["\t", "!", " ", "-", "!"], vec!["A", "T", "K"], 0.5, 0 ; "no words in input")]
    fn test_substitute(input_tokens: Vec<&str>, vocab: Vec<&str>, p: f32, expected_doc_changes: usize) {
        let mut doc = Doc::from_tokens(input_tokens);
        let params = TextAugmentParameters::new(p, None, None);
        let vocab = vocab.into_iter().map(|s| s.to_string()).collect();
        let aug = WordsRandomAugmenter::new(TextAction::Substitute, params, None, Some(vocab));

        let doc_tokens_before = doc.tokens.clone();

        doc = aug.substitute(doc, &mut rand::thread_rng());

        let doc_tokens_after = doc.tokens.clone();

        if expected_doc_changes == 0 {
            assert_eq!(doc_tokens_before, doc_tokens_after);
        } else {
            assert_eq!(doc_tokens_before.len(), doc_tokens_after.len());
            assert_ne!(doc_tokens_before, doc_tokens_after);
            assert_eq!(doc.num_changes, expected_doc_changes);
        }
    }

    #[test_case(vec!["A", "B", "C", "D", "E"], 0.5, 3, 3 ; "round 2.5 as 3 of 5")]
    #[test_case(vec!["A", "B", "C", "D", "E", "D"], 0.5, 3, 3 ; "3 of 6")]
    #[test_case(vec!["\t", "B", " ", "D", "!"], 0.5, 1, 1 ; "1 word")]
    #[test_case(vec!["\t", "B", " ", "D", "!"], 0.1, 1, 1 ; "round 0.05 as 1 word")]
    #[test_case(vec!["\t", "B", " ", "D", "!"], 0.0, 0, 0 ; "delete probability=0")]
    #[test_case(vec!["\t", "!", " ", "-", "!"], 0.5, 0, 0 ; "no words in input")]
    fn test_delete(input_tokens: Vec<&str>, p: f32, expected_deleted_tokens: usize, expected_doc_changes: usize) {
        let mut doc = Doc::from_tokens(input_tokens);
        let params = TextAugmentParameters::new(p, None, None);
        let aug = WordsRandomAugmenter::new(TextAction::Delete, params, None, None);

        let doc_tokens_before = doc.tokens.clone();

        doc = aug.delete(doc, &mut rand::thread_rng());

        let doc_tokens_after = doc.tokens.clone();

        if expected_doc_changes == 0 {
            assert_eq!(doc_tokens_before, doc_tokens_after);
        } else {
            assert_eq!(doc_tokens_before.len(), doc_tokens_after.len());
            assert_ne!(doc_tokens_before, doc_tokens_after);
            assert_eq!(doc.num_changes, expected_doc_changes);
            assert_eq!(
                doc_tokens_after
                    .iter()
                    .filter(|token| token.kind() == &TokenType::Deleted)
                    .count(),
                expected_deleted_tokens
            );
        }
    }

    #[test_case(vec!["A", "B", "C", "D", "E"], 0.5, 3 ; "round 2.5 as 3 of 5")]
    #[test_case(vec!["A", "B", "C", "D", "E", "D"], 0.5, 3 ; "3 of 6 words")]
    #[test_case(vec!["\t", "B", " ", "D", "!"], 0.5, 0 ; "1 word no swaps")]
    #[test_case(vec!["\t", "B", " ", "D", "!"], 0.1, 0  ; "round 0.2 as 1 no swaps")]
    fn test_swap(input_tokens: Vec<&str>, p: f32, expected_doc_changes: usize) {
        let mut doc = Doc::from_tokens(input_tokens);
        let params = TextAugmentParameters::new(p, None, None);
        let aug = WordsRandomAugmenter::new(TextAction::Swap, params, None, None);

        let doc_tokens_before = doc.tokens.clone();

        doc = aug.swap(doc, &mut rand::thread_rng());

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
