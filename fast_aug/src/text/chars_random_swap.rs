use super::base::BaseTextAugmenter;
use super::utils::{Doc, TextAugmentParameters};
use crate::base::BaseAugmenter;
use std::collections::HashSet;

/// Augmenter that swaps random chars in random words in text
///
/// # Examples
/// ```rust
/// use fast_aug::base::BaseAugmenter;
/// use fast_aug::text::{CharsRandomSwapAugmenter, TextAugmentParameters};
///
/// let rng = &mut rand::thread_rng();
/// let augmenter = CharsRandomSwapAugmenter::new(
///     TextAugmentParameters::new(0.5, None, None),
///     TextAugmentParameters::new(0.5, None, None),
///     None,
/// );
/// augmenter.augment("Some text!".to_string(), rng);
/// augmenter.augment_batch(vec!["Some text!".to_string()], rng);
/// ```
pub struct CharsRandomSwapAugmenter {
    /// Parameters to calculate number of words that will be augmented
    word_params: TextAugmentParameters,
    /// Parameters to calculate number of chars that will be augmented in each word
    char_params: TextAugmentParameters,
    /// Filter, Set of words that cannot be augmented
    stopwords: Option<HashSet<String>>,
}

impl CharsRandomSwapAugmenter {
    pub fn new(
        word_params: TextAugmentParameters,
        char_params: TextAugmentParameters,
        stopwords: Option<HashSet<String>>,
    ) -> Self {
        CharsRandomSwapAugmenter {
            word_params,
            char_params,
            stopwords,
        }
    }
}

impl BaseTextAugmenter for CharsRandomSwapAugmenter {}

impl BaseAugmenter<String, Doc> for CharsRandomSwapAugmenter {
    fn augment_inner(&self, mut input: Doc, rng: &mut dyn rand::RngCore) -> Doc {
        // TODO: adjacent, middle, random swaps (now only random)
        // Select random word tokens
        let word_tokens_indexes = input.get_word_indexes(false, self.stopwords.as_ref());
        let num_tokens_to_change = self.word_params.num_elements(word_tokens_indexes.len());
        let selected_tokens_indexes =
            self.select_random_element_indexes(rng, word_tokens_indexes, num_tokens_to_change);

        // For all selected tokens select random chars and swap them
        for token_index in selected_tokens_indexes {
            let token = &mut input.tokens[token_index];
            let num_chars_to_change = self.char_params.num_elements(token.utf8_len());

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

    #[test_case(vec!["ABCD", "EFGH", "IJKL", "MNOP", "QRST"], 0.5, 0.5, 3 ; "round 2.5 as 3 words, round 2.5 as 3 chars each")]
    #[test_case(vec!["ABCD", "EFGH", "IJKL", "MNOP", "QRST"], 0.0, 0.5, 0 ; "swap chars in 0 words - no changes")]
    #[test_case(vec!["ABCD", "EFGH", "IJKL", "MNOP", "QRST"], 0.5, 0.0, 0 ; "swap 0 chars - no changes")]
    fn test_swap(input_tokens: Vec<&str>, words_p: f32, chars_p: f32, expected_doc_changes: usize) {
        let mut doc = Doc::from_tokens(input_tokens);
        let words_params = TextAugmentParameters::new(words_p, None, None);
        let chars_params = TextAugmentParameters::new(chars_p, None, None);
        let aug = CharsRandomSwapAugmenter::new(words_params, chars_params, None);

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
        let aug = CharsRandomSwapAugmenter::new(words_params, chars_params, None);

        let doc_tokens_before = doc.tokens.clone();

        doc = aug.augment_inner(doc, &mut rand::thread_rng());

        let doc_tokens_after = doc.tokens.clone();

        assert_eq!(doc_tokens_before.len(), doc_tokens_after.len());
        assert_ne!(doc_tokens_before, doc_tokens_after);
    }
}
