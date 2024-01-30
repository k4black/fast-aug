use super::base::BaseTextAugmenter;
use super::utils::{Doc, TextAugmentParameters, TokenType};
use crate::base::BaseAugmenter;
use std::collections::HashSet;

/// Augmenter that deletes random words in text
///
/// # Examples
/// ```rust
/// use fast_aug::base::BaseAugmenter;
/// use fast_aug::text::{WordsRandomDeleteAugmenter, TextAugmentParameters};
///
/// let rng = &mut rand::thread_rng();
/// let augmenter = WordsRandomDeleteAugmenter::new(
///     TextAugmentParameters::new(0.5, None, None),
///     None,
/// );
/// augmenter.augment("Some text!".to_string(), rng);
/// augmenter.augment_batch(vec!["Some text!".to_string()], rng);
/// ```
pub struct WordsRandomDeleteAugmenter {
    /// Parameters to calculate number of words that will be augmented
    word_params: TextAugmentParameters,
    /// Filter, Set of words that cannot be augmented
    stopwords: Option<HashSet<String>>,
}

impl WordsRandomDeleteAugmenter {
    pub fn new(word_params: TextAugmentParameters, stopwords: Option<HashSet<String>>) -> Self {
        WordsRandomDeleteAugmenter { word_params, stopwords }
    }
}

impl BaseTextAugmenter for WordsRandomDeleteAugmenter {}

impl BaseAugmenter<String, Doc> for WordsRandomDeleteAugmenter {
    fn augment_inner(&self, mut input: Doc, rng: &mut dyn rand::RngCore) -> Doc {
        // Select random word tokens
        let word_tokens_indexes = input.get_word_indexes(false, self.stopwords.as_ref());
        let num_tokens_to_change = self.word_params.num_elements(word_tokens_indexes.len());
        let selected_tokens_indexes =
            self.select_random_element_indexes(rng, word_tokens_indexes, num_tokens_to_change);

        // For all selected tokens set TokenType::Deleted
        for index in selected_tokens_indexes {
            input.tokens[index].change("", TokenType::Deleted);
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

    #[test_case(vec!["A", "B", "C", "D", "E"], 0.5, 3, 3 ; "round 2.5 as 3 of 5")]
    #[test_case(vec!["A", "B", "C", "D", "E", "D"], 0.5, 3, 3 ; "3 of 6")]
    #[test_case(vec!["\t", "B", " ", "D", "!"], 0.5, 1, 1 ; "1 word")]
    #[test_case(vec!["\t", "B", " ", "D", "!"], 0.1, 1, 1 ; "round 0.05 as 1 word")]
    #[test_case(vec!["\t", "B", " ", "D", "!"], 0.0, 0, 0 ; "delete probability=0")]
    #[test_case(vec!["\t", "!", " ", "-", "!"], 0.5, 0, 0 ; "no words in input")]
    fn test_delete(input_tokens: Vec<&str>, p: f32, expected_deleted_tokens: usize, expected_doc_changes: usize) {
        let mut doc = Doc::from_tokens(input_tokens);
        let params = TextAugmentParameters::new(p, None, None);
        let aug = WordsRandomDeleteAugmenter::new(params, None);

        let doc_tokens_before = doc.tokens.clone();

        doc = aug.augment_inner(doc, &mut rand::thread_rng());

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
}
