use std::collections::HashSet;
use crate::base::BaseAugmenter;
use super::doc::Doc;
use super::parameters::TextAugmentParameters;
use super::base::{BaseTextAugmenter, TextAction};


pub struct TfIdfWordsAugmenter {
    /// Action to augmentation, set of values {'substitute', 'swap', 'delete'}
    action: TextAction,
    /// Parameters to calculate number of words that will be augmented
    aug_params_word: TextAugmentParameters,
    /// Filter, Set of words that cannot be augmented
    stopwords: Option<HashSet<String>>,
    /// top k similar words to substitute
    top_k: usize,
}


impl TfIdfWordsAugmenter {
    pub fn new(
        aug_params_word: TextAugmentParameters,
        stopwords: Option<HashSet<String>>,
        top_k: usize,
    ) -> Self {
        TfIdfWordsAugmenter {
            action: TextAction::Substitute,
            aug_params_word,
            stopwords,
            top_k,
        }
    }

    fn substitute(&self, mut doc: Doc) -> Doc {
        // Select random word tokens
        let word_tokens_indexes = doc.get_word_indexes(false, self.stopwords.as_ref());
        let selected_tokens_indexes = self.aug_params_word.select_random_element_indexes(word_tokens_indexes);

        // For all selected tokens randomly select one word with closest tf-idf and substitute
        for index in selected_tokens_indexes {
            todo!();
            doc.num_changes += 1;
        }

        doc
    }
}


impl BaseTextAugmenter for TfIdfWordsAugmenter{}


impl BaseAugmenter<String,Doc> for TfIdfWordsAugmenter {
    fn augment_inner(&self, input: Doc) -> Doc {
        match self.action {
            TextAction::Substitute => self.substitute(input),
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



}

