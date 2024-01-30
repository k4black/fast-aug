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
/// use fast_aug::text::{WordsRandomSubstituteAugmenter, TextAugmentParameters};
///
/// let rng = &mut rand::thread_rng();
/// let augmenter = WordsRandomSubstituteAugmenter::new(
///     TextAugmentParameters::new(0.5, None, None),
///     vec!["A", "B", "C", "D", "E"].into_iter().map(|s| s.to_string()).collect(),
///     None,
/// );
/// augmenter.augment("Some text!".to_string(), rng);
/// augmenter.augment_batch(vec!["Some text!".to_string()], rng);
/// ```
pub struct WordsRandomSubstituteAugmenter {
    /// Parameters to calculate number of words that will be augmented
    word_params: TextAugmentParameters,
    /// Set of words that can be used to replace or insert
    vocabulary: Vec<String>,
    /// Filter, Set of words that cannot be augmented
    stopwords: Option<HashSet<String>>,
}

impl WordsRandomSubstituteAugmenter {
    pub fn new(
        word_params: TextAugmentParameters,
        vocabulary: Vec<String>,
        stopwords: Option<HashSet<String>>,
    ) -> Self {
        WordsRandomSubstituteAugmenter {
            word_params,
            vocabulary,
            stopwords,
        }
    }
}

impl BaseTextAugmenter for WordsRandomSubstituteAugmenter {}

impl BaseAugmenter<String, Doc> for WordsRandomSubstituteAugmenter {
    fn augment_inner(&self, mut input: Doc, rng: &mut dyn rand::RngCore) -> Doc {
        // Select random word tokens
        let word_tokens_indexes = input.get_word_indexes(false, self.stopwords.as_ref());
        let num_tokens_to_change = self.word_params.num_elements(word_tokens_indexes.len());
        let selected_tokens_indexes =
            self.select_random_element_indexes(rng, word_tokens_indexes, num_tokens_to_change);

        // Select random words from vocabulary - with replacement
        let tokens_to_insert: Vec<Token> = (0..num_tokens_to_change)
            .map(|_| self.vocabulary.choose(rng).map(|word| Token::from_str(word)).unwrap())
            .collect();
        assert_eq!(selected_tokens_indexes.len(), tokens_to_insert.len());

        // Substitute tokens in selected places
        for (index, token) in selected_tokens_indexes.into_iter().zip(tokens_to_insert.into_iter()) {
            input.tokens[index] = token;
            input.num_changes += 1;
        }

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
        let aug = WordsRandomSubstituteAugmenter::new(params, vocab, None);

        let doc_tokens_before = doc.tokens.clone();

        doc = aug.augment_inner(doc, &mut rand::thread_rng());

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
