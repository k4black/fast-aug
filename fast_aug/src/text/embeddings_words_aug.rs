#![allow(dead_code, unused_variables, unreachable_code)]

use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use finalfusion::prelude::{Embeddings, ReadEmbeddings, ReadFastText, ReadWord2Vec, StorageWrap, VocabWrap};
use finalfusion::storage::NdArray;
use finalfusion::vocab::FastTextSubwordVocab;
use crate::base::BaseAugmenter;
use super::doc::Doc;
use super::parameters::TextAugmentParameters;
use super::base::{BaseTextAugmenter, TextAction};


pub struct EmbeddingsWordsAugmenter {
    /// Finalfusion embeddings
    embeddings: Arc<Embeddings<FastTextSubwordVocab, NdArray>>,
    /// Action to augmentation, set of values {'substitute', 'swap', 'delete'}
    action: TextAction,
    /// Parameters to calculate number of words that will be augmented
    aug_params_word: TextAugmentParameters,
    /// Filter, Set of words that cannot be augmented
    stopwords: Option<HashSet<String>>,
    /// top k similar words to substitute
    top_k: usize,
}


impl EmbeddingsWordsAugmenter {
    pub fn new(
        embeddings_path: &str,
        aug_params_word: TextAugmentParameters,
        stopwords: Option<HashSet<String>>,
        top_k: usize,
    ) -> Self {
        // .expect("Cannot open embeddings file")
        let mut reader = BufReader::new(File::open(embeddings_path).unwrap());

        // let embeddings_0 = Embeddings::read_embeddings(&mut reader).unwrap();
        let embeddings_1 = Embeddings::read_fasttext(&mut reader).unwrap();
        // let embeddings_2 = Embeddings::read_word2vec_binary(&mut reader).unwrap();

        EmbeddingsWordsAugmenter {
            embeddings: Arc::new(embeddings_1),
            action: TextAction::Substitute,
            aug_params_word,
            stopwords,
            top_k,
        }
    }

    fn substitute(&self, mut doc: Doc, rng: &mut dyn rand::RngCore) -> Doc {
        // Select random word tokens
        let word_tokens_indexes = doc.get_word_indexes(false, self.stopwords.as_ref());
        let num_tokens_to_change = self.aug_params_word.num_elements(word_tokens_indexes.len());
        let selected_tokens_indexes = self.select_random_element_indexes(rng, word_tokens_indexes, num_tokens_to_change);

        // For all selected tokens select some random similar word and substitute
        for index in selected_tokens_indexes {
            todo!();
            doc.num_changes += 1;
        }

        doc
    }
}


impl BaseTextAugmenter for EmbeddingsWordsAugmenter{}


impl BaseAugmenter<String,Doc> for EmbeddingsWordsAugmenter {
    fn augment_inner(&self, input: Doc, rng: &mut dyn rand::RngCore) -> Doc {
        match self.action {
            TextAction::Substitute => self.substitute(input, rng),
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

    // #[test_case("test_data/cc.en.100.bin" ; "fasttext")]
    // fn test_load_embeddings(embeddings_path: &str) {
    //     let embeddings_augmenter = EmbeddingsWordsAugmenter::new(
    //         embeddings_path,
    //         TextAugmentParameters::default(),
    //         None,
    //         5,
    //     );
    // }

}

