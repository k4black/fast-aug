use std::collections::HashSet;
use crate::base::BaseAugmenter;
use super::doc::Doc;
use super::token::TokenType;
use super::parameters::TextAugmentParameters;
use super::base::{BaseTextAugmenter, TextAction};


pub struct RandomWordsAugmenter {
    /// Action to augmentation, set of values {'substitute', 'swap', 'delete'}
    action: TextAction,
    /// Parameters to calculate number of words that will be augmented
    aug_params_word: TextAugmentParameters,
    /// Filter, Set of words that cannot be augmented
    stopwords: Option<HashSet<String>>,
}


impl RandomWordsAugmenter {
    pub fn new(
        action: TextAction,
        aug_params_word: TextAugmentParameters,
        stopwords: Option<HashSet<String>>,
    ) -> Self {
        RandomWordsAugmenter {
            action,
            aug_params_word,
            stopwords,
        }
    }

    fn delete(&self, mut doc: Doc) -> Doc {
        // Select random word tokens
        let word_tokens_indexes = doc.get_word_indexes(false, self.stopwords.as_ref());
        let selected_tokens_indexes = self.aug_params_word.select_random_element_indexes(word_tokens_indexes);

        // For all selected tokens set TokenType::Deleted
        let mut num_changes = 0;
        for index in selected_tokens_indexes {
            doc.tokens[index].change("", TokenType::Deleted);
            num_changes += 1;
        }
        doc.num_changes += num_changes;

        doc
    }

    fn swap(&self, mut doc: Doc) -> Doc {
        // Select random word tokens (shuffle selected tokens to make swaps)
        let word_tokens_indexes = doc.get_word_indexes(false, self.stopwords.as_ref());
        let selected_tokens_indexes = self.aug_params_word.select_random_element_indexes(word_tokens_indexes);

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


impl BaseTextAugmenter for RandomWordsAugmenter{}


impl BaseAugmenter<String,Doc> for RandomWordsAugmenter {
    fn augment_inner(&self, input: Doc) -> Doc {
        match self.action {
            TextAction::Delete => self.delete(input),
            TextAction::Swap => self.swap(input),
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
    use test_case::test_case;
    use super::*;

    #[test_case(vec!["A", "B", "C", "D", "E"], 0.5, 3, 3 ; "round 2.5 as 3 of 5")]
    #[test_case(vec!["A", "B", "C", "D", "E", "D"], 0.5, 3, 3 ; "3 of 6")]
    #[test_case(vec!["\t", "B", " ", "D", "!"], 0.5, 1, 1 ; "1 word")]
    #[test_case(vec!["\t", "B", " ", "D", "!"], 0.1, 1, 1 ; "round 0.05 as 1 word")]
    #[test_case(vec!["\t", "B", " ", "D", "!"], 0.0, 0, 0 ; "delete probability=0")]
    #[test_case(vec!["\t", "!", " ", "-", "!"], 0.5, 0, 0 ; "no words in input")]
    fn test_delete(input_tokens: Vec<&str>, p: f32, expected_deleted_tokens: usize, expected_doc_changes: usize) {
        let mut doc = Doc::from_tokens(input_tokens);
        let params = TextAugmentParameters::new(p, None, None);
        let aug = RandomWordsAugmenter::new(TextAction::Delete, params, None);

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

    #[test_case(vec!["A", "B", "C", "D", "E"], 0.5, 3 ; "round 2.5 as 3 of 5")]
    #[test_case(vec!["A", "B", "C", "D", "E", "D"], 0.5, 3 ; "3 of 6 words")]
    #[test_case(vec!["\t", "B", " ", "D", "!"], 0.5, 0 ; "1 word no swaps")]
    #[test_case(vec!["\t", "B", " ", "D", "!"], 0.1, 0  ; "round 0.2 as 1 no swaps")]
    fn test_swap(input_tokens: Vec<&str>, p: f32, expected_doc_changes: usize) {
        let mut doc = Doc::from_tokens(input_tokens);
        let params = TextAugmentParameters::new(p, None, None);
        let aug = RandomWordsAugmenter::new(TextAction::Swap, params, None);

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

