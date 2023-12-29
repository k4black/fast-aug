use std::collections::HashSet;
use crate::base::BaseAugmenter;
use super::doc::Doc;
use super::parameters::TextAugmentParameters;
use super::base::{BaseTextAugmenter, TextAction};


pub struct RandomCharsAugmenter {
    /// Action to augmentation, set of values {'substitute', 'swap', 'delete'}
    action: TextAction,
    /// Parameters to calculate number of words that will be augmented
    aug_params_word: TextAugmentParameters,
    /// Parameters to calculate number of chars that will be augmented in each word
    aug_params_char: TextAugmentParameters,
    /// Filter, Set of words that cannot be augmented
    stopwords: Option<HashSet<String>>,
}


impl RandomCharsAugmenter {
    pub fn new(
        action: TextAction,
        aug_params_word: TextAugmentParameters,
        aug_params_char: TextAugmentParameters,
        stopwords: Option<HashSet<String>>,
    ) -> Self {
        RandomCharsAugmenter {
            action,
            aug_params_word,
            aug_params_char,
            stopwords,
        }
    }
}


impl BaseTextAugmenter for RandomCharsAugmenter {
    fn action(&self) -> &TextAction {
        &self.action
    }

    // Not applicable
    fn insert(&self, _doc: Doc) -> Doc {
        // TODO: chars insertion using alphabet
        panic!("Insert action is not applicable for RandomWordAugmenter");
    }

    fn delete(&self, mut doc: Doc) -> Doc {
        // Select random word tokens
        let word_tokens_indexes = doc.get_word_indexes(false, self.stopwords.as_ref());
        let selected_tokens_indexes = self.aug_params_word.select_random_element_indexes(word_tokens_indexes);

        // For all selected tokens select random chars and remove them
        let mut num_changes = 0;
        for token_index in selected_tokens_indexes {
            let token = &mut doc.tokens[token_index];

            let mut selected_chars_indexes = self.aug_params_char.select_random_element_indexes((0..token.token().len()).collect());
            let mut new_token = String::with_capacity(token.token().len());
            for (idx, char) in token.token().char_indices() {
                if !selected_chars_indexes.contains(&idx) {
                    new_token.push(char);
                }
            }
            token.change(&new_token, *token.kind());

            num_changes += 1;
        }
        doc.num_changes += num_changes;

        doc
    }

    fn substitute(&self, _doc: Doc) -> Doc {
        // TODO: chars substitution using alphabet
        panic!("Insert action is not applicable for RandomWordAugmenter");
    }

    fn swap(&self, mut doc: Doc) -> Doc {
        // TODO: adjacent, middle, random swaps (now only random)
        // Select random word tokens
        let word_tokens_indexes = doc.get_word_indexes(false, self.stopwords.as_ref());
        let selected_tokens_indexes = self.aug_params_word.select_random_element_indexes(word_tokens_indexes);

        // For all selected tokens select random chars and swap them
        let mut num_changes = 0;
        for token_index in selected_tokens_indexes {
            let token = &mut doc.tokens[token_index];

            let selected_chars_indexes = self.aug_params_char.select_random_element_indexes((0..token.token().len()).collect());
            let mut chars = token.token().chars().collect::<Vec<char>>();
            selected_chars_indexes.chunks(2).for_each(|chunk| {
                if chunk.len() == 2 {
                    chars.swap(chunk[0], chunk[1]);
                }
            });
            let new_token = chars.iter().collect::<String>();
            token.change(&new_token, *token.kind());

            num_changes += 1;
        }
        doc.num_changes += num_changes;

        doc
    }
}


impl BaseAugmenter<String> for RandomCharsAugmenter {
    fn augment(&self, input: String) -> String {
        BaseTextAugmenter::augment(self, input)
    }
}


#[cfg(test)]
mod tests {
    use test_case::test_case;
    use crate::text::random_words_aug::RandomWordsAugmenter;
    use super::*;

    #[test_case(vec!["AAAA", "BBBB", "CCCC", "DDDD", "EEEE"], 0.5, 0.5, 3, 3 ; "round 2.5 as 3 words round 2.5 as 3 chars each")]
    #[test_case(vec!["AAAA", "BBBB", "CCCC", "DDDD", "EEEE"], 0.0, 0.5, 0, 0 ; "delete chars in 0 words - no changes")]
    #[test_case(vec!["AAAA", "BBBB", "CCCC", "DDDD", "EEEE"], 0.5, 0.0, 0, 0 ; "delete 0 chars - no changes")]
    fn test_delete(input_tokens: Vec<&str>, words_p: f32, chars_p: f32, expected_changed_words: usize, expected_doc_changes: usize) {
        let mut doc = Doc::from_tokens(input_tokens);
        let words_params = TextAugmentParameters::new(words_p, None, None);
        let chars_params = TextAugmentParameters::new(chars_p, None, None);
        let aug = RandomCharsAugmenter::new(TextAction::Delete, words_params, chars_params, None);

        let doc_tokens_before = doc.tokens.clone();

        doc = aug.delete(doc);

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
        let aug = RandomCharsAugmenter::new(TextAction::Swap, words_params, chars_params, None);

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
}
