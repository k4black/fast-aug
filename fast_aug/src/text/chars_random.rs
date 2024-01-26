use super::base::{BaseTextAugmenter, TextAction};
use super::utils::{Doc, TextAugmentParameters};
use crate::base::BaseAugmenter;
use std::collections::HashSet;

pub struct CharsRandomAugmenter {
    /// Action to augmentation, set of values {'substitute', 'swap', 'delete'}
    action: TextAction,
    /// Parameters to calculate number of words that will be augmented
    aug_params_word: TextAugmentParameters,
    /// Parameters to calculate number of chars that will be augmented in each word
    aug_params_char: TextAugmentParameters,
    /// Filter, Set of words that cannot be augmented
    stopwords: Option<HashSet<String>>,
}

impl CharsRandomAugmenter {
    pub fn new(
        action: TextAction,
        aug_params_word: TextAugmentParameters,
        aug_params_char: TextAugmentParameters,
        stopwords: Option<HashSet<String>>,
    ) -> Self {
        CharsRandomAugmenter {
            action,
            aug_params_word,
            aug_params_char,
            stopwords,
        }
    }

    fn delete(&self, mut doc: Doc, rng: &mut dyn rand::RngCore) -> Doc {
        // Select random word tokens
        let word_tokens_indexes = doc.get_word_indexes(false, self.stopwords.as_ref());
        let num_tokens_to_change = self.aug_params_word.num_elements(word_tokens_indexes.len());
        let selected_tokens_indexes =
            self.select_random_element_indexes(rng, word_tokens_indexes, num_tokens_to_change);

        // For all selected tokens select random chars and remove them
        for token_index in selected_tokens_indexes {
            let token = &mut doc.tokens[token_index];
            let num_chars_to_change = self.aug_params_char.num_elements(token.utf8_len());

            let selected_chars_indexes =
                self.select_random_element_indexes(rng, (0..token.utf8_len()).collect(), num_chars_to_change);
            let mut new_token = String::with_capacity(token.utf8_len());
            for (idx, char) in token.token().char_indices() {
                if !selected_chars_indexes.contains(&idx) {
                    new_token.push(char);
                }
            }
            token.change(&new_token, *token.kind());

            doc.num_changes += 1;
        }

        doc
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
            let token = &mut doc.tokens[token_index];
            let num_chars_to_change = self.aug_params_char.num_elements(token.utf8_len());

            let selected_chars_indexes =
                self.select_random_element_indexes(rng, (0..token.utf8_len()).collect(), num_chars_to_change);
            let mut chars = token.token().chars().collect::<Vec<char>>();
            selected_chars_indexes.chunks(2).for_each(|chunk| {
                if chunk.len() == 2 {
                    chars.swap(chunk[0], chunk[1]);
                }
            });
            let new_token = chars.iter().collect::<String>();
            token.change(&new_token, *token.kind());

            doc.num_changes += 1;
        }

        doc
    }
}

impl BaseTextAugmenter for CharsRandomAugmenter {}

impl BaseAugmenter<String, Doc> for CharsRandomAugmenter {
    fn augment_inner(&self, input: Doc, rng: &mut dyn rand::RngCore) -> Doc {
        match self.action {
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

    #[test_case(vec!["AAAA", "BBBB", "CCCC", "DDDD", "EEEE"], 0.5, 0.5, 3, 3 ; "round 2.5 as 3 words round 2.5 as 3 chars each")]
    #[test_case(vec!["AAAA", "BBBB", "CCCC", "DDDD", "EEEE"], 0.0, 0.5, 0, 0 ; "delete chars in 0 words - no changes")]
    #[test_case(vec!["AAAA", "BBBB", "CCCC", "DDDD", "EEEE"], 0.5, 0.0, 0, 0 ; "delete 0 chars - no changes")]
    fn test_delete(
        input_tokens: Vec<&str>,
        words_p: f32,
        chars_p: f32,
        expected_changed_words: usize,
        expected_doc_changes: usize,
    ) {
        let mut doc = Doc::from_tokens(input_tokens);
        let words_params = TextAugmentParameters::new(words_p, None, None);
        let chars_params = TextAugmentParameters::new(chars_p, None, None);
        let aug = CharsRandomAugmenter::new(TextAction::Delete, words_params, chars_params, None);

        let doc_tokens_before = doc.tokens.clone();

        doc = aug.delete(doc, &mut rand::thread_rng());

        let doc_tokens_after = doc.tokens.clone();

        if expected_doc_changes == 0 {
            assert_eq!(doc_tokens_before, doc_tokens_after);
        } else {
            assert_eq!(doc_tokens_before.len(), doc_tokens_after.len());
            assert_ne!(doc_tokens_before, doc_tokens_after);
            assert_eq!(doc.num_changes, expected_doc_changes);
        }

        let mut num_changed_words = 0;
        for (token_before, token_after) in doc_tokens_before.iter().zip(doc_tokens_after.iter()) {
            if token_before.token() != token_after.token() {
                assert!(token_before.token().len() > token_after.token().len());
                num_changed_words += 1;
            }
        }
        assert_eq!(num_changed_words, expected_changed_words);
    }

    #[test_case(vec!["ABCD", "EFGH", "IJKL", "MNOP", "QRST"], 0.5, 0.5, 3 ; "round 2.5 as 3 words, round 2.5 as 3 chars each")]
    #[test_case(vec!["ABCD", "EFGH", "IJKL", "MNOP", "QRST"], 0.0, 0.5, 0 ; "swap chars in 0 words - no changes")]
    #[test_case(vec!["ABCD", "EFGH", "IJKL", "MNOP", "QRST"], 0.5, 0.0, 0 ; "swap 0 chars - no changes")]
    fn test_swap(input_tokens: Vec<&str>, words_p: f32, chars_p: f32, expected_doc_changes: usize) {
        let mut doc = Doc::from_tokens(input_tokens);
        let words_params = TextAugmentParameters::new(words_p, None, None);
        let chars_params = TextAugmentParameters::new(chars_p, None, None);
        let aug = CharsRandomAugmenter::new(TextAction::Swap, words_params, chars_params, None);

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

        let mut num_changed_words = 0;
        for (token_before, token_after) in doc_tokens_before.iter().zip(doc_tokens_after.iter()) {
            if token_before.token() != token_after.token() {
                assert_eq!(token_before.token().len(), token_after.token().len());
                assert_ne!(token_before.token(), token_after.token());
                num_changed_words += 1;
            }
        }
        assert_eq!(num_changed_words, expected_doc_changes);
    }

    #[test_case("It’s ” #NBAwards" ; "utf8 len not equal bytes len")]
    fn test_swap_bugs(text: &str) {
        let mut doc = Doc::new(text);
        let words_params = TextAugmentParameters::new(1.0, None, None);
        let chars_params = TextAugmentParameters::new(0.3, None, None);
        let aug = CharsRandomAugmenter::new(TextAction::Swap, words_params, chars_params, None);

        let doc_tokens_before = doc.tokens.clone();

        doc = aug.swap(doc, &mut rand::thread_rng());

        let doc_tokens_after = doc.tokens.clone();

        assert_eq!(doc_tokens_before.len(), doc_tokens_after.len());
        assert_ne!(doc_tokens_before, doc_tokens_after);
    }
}
