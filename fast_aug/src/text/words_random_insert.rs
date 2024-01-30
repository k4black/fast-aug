use super::base::BaseTextAugmenter;
use super::utils::{Doc, TextAugmentParameters};
use crate::base::BaseAugmenter;
use crate::text::Token;
use rand::prelude::SliceRandom;
use std::collections::HashSet;

/// Augmenter that substitutes random words with random words from vocabulary
///
/// # Examples
/// ```rust
/// use fast_aug::base::BaseAugmenter;
/// use fast_aug::text::{WordsRandomInsertAugmenter, TextAugmentParameters};
///
/// let rng = &mut rand::thread_rng();
/// let augmenter = WordsRandomInsertAugmenter::new(
///     TextAugmentParameters::new(0.5, None, None),
///     vec!["A", "B", "C", "D", "E"].into_iter().map(|s| s.to_string()).collect(),
///     None,
/// );
/// augmenter.augment("Some text!".to_string(), rng);
/// augmenter.augment_batch(vec!["Some text!".to_string()], rng);
/// ```
pub struct WordsRandomInsertAugmenter {
    /// Parameters to calculate number of words that will be augmented
    word_params: TextAugmentParameters,
    /// Set of words that can be used to replace or insert
    vocabulary: Vec<String>,
    /// Filter, Set of words that cannot be augmented
    #[allow(dead_code)]
    stopwords: Option<HashSet<String>>,
}

impl WordsRandomInsertAugmenter {
    pub fn new(
        word_params: TextAugmentParameters,
        vocabulary: Vec<String>,
        stopwords: Option<HashSet<String>>,
    ) -> Self {
        WordsRandomInsertAugmenter {
            word_params,
            vocabulary,
            stopwords,
        }
    }
}

impl BaseTextAugmenter for WordsRandomInsertAugmenter {}

impl BaseAugmenter<String, Doc> for WordsRandomInsertAugmenter {
    fn augment_inner(&self, mut input: Doc, rng: &mut dyn rand::RngCore) -> Doc {
        // Select random places to insert tokens from 0 (before the first token) to len inclusive (after last token)
        let num_tokens_to_insert = self.word_params.num_elements(input.tokens.len());
        let mut selected_places_to_insert_indexes =
            self.select_random_element_indexes(rng, (0..=input.tokens.len()).collect(), num_tokens_to_insert);
        selected_places_to_insert_indexes.sort();

        // Select random words from vocabulary - with replacement
        let mut tokens_to_insert: Vec<Token> = (0..num_tokens_to_insert)
            .map(|_| self.vocabulary.choose(rng).map(|word| Token::from_str(word)).unwrap())
            .collect();
        assert_eq!(selected_places_to_insert_indexes.len(), tokens_to_insert.len());

        // Insert tokens in selected places
        // Go through all positions to insert, if have original doc token insert it else insert new token
        let mut new_tokens: Vec<Token> = Vec::with_capacity(input.tokens.len() + tokens_to_insert.len());
        let mut current_doc_index = 0;
        for place_to_insert_index in selected_places_to_insert_indexes {
            // Add tokens until current place
            while current_doc_index < place_to_insert_index {
                new_tokens.push(input.tokens[current_doc_index].clone());
                current_doc_index += 1;
            }
            // If current place < than current doc index, add inserted token
            if place_to_insert_index <= current_doc_index {
                new_tokens.push(tokens_to_insert.pop().unwrap());
            }
        }
        // Insert remaining tokens
        while current_doc_index < input.tokens.len() {
            new_tokens.push(input.tokens[current_doc_index].clone());
            current_doc_index += 1;
        }
        input.num_changes = num_tokens_to_insert;
        input.tokens = new_tokens;

        input
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
        let aug = WordsRandomInsertAugmenter::new(params, vocab, None);

        let doc_tokens_before = doc.tokens.clone();

        doc = aug.augment_inner(doc, &mut rand::thread_rng());

        let doc_tokens_after = doc.tokens.clone();

        assert_eq!(doc_tokens_after.len(), expected_len);
        if expected_doc_changes == 0 {
            assert_eq!(doc_tokens_before, doc_tokens_after);
        } else {
            assert_ne!(doc_tokens_before, doc_tokens_after);
            assert_eq!(doc.num_changes, expected_doc_changes);
        }
    }
}
