use std::collections::HashSet;
use rand::prelude::IteratorRandom;
use rand::thread_rng;
use super::doc::Doc;
use crate::base::BaseAugmenter;
use crate::text::Token;

/// Actions enum, which action BaseTextAugmenter will perform
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TextAction {
    Insert,
    Delete,
    Substitute,
    Swap,
}


pub trait BaseTextAugmenter: BaseAugmenter<String> {
    fn action(&self) -> &TextAction;

    /// Public method to augment an input string
    /// 1. Convert input string to Doc
    /// 2. Perform augmentation on Doc
    /// 3. Convert augmented Doc to string
    fn augment(&self, input: String) -> String {
        let doc = Doc::new(&input);
        let output = match self.action() {
            TextAction::Insert => self.insert(doc),
            TextAction::Delete => self.delete(doc),
            TextAction::Substitute => self.substitute(doc),
            TextAction::Swap => self.swap(doc),
        };
        output.to_string()
    }
    
    fn insert(&self, doc: Doc) -> Doc;
    fn delete(&self, doc: Doc) -> Doc;
    fn substitute(&self, doc: Doc) -> Doc;
    fn swap(&self, doc: Doc) -> Doc;


    /// Select random word tokens to augment given a number of elements
    /// Returns a vector of tuples (index, &mut token)
    /// TODO: optimize this function
    fn select_word_tokens_to_augment<'a>(&self, candidate_tokens: &'a mut Vec<(usize, &'a mut Token)>, num_elements: usize, stopwords: Option<&HashSet<String>>) -> Vec<(usize, &'a mut Token)> {
        let mut rng = thread_rng();

        let mut filtered_tokens: Vec<(usize, &mut Token)> = Vec::with_capacity(candidate_tokens.len());

        for (idx, token) in candidate_tokens {
            let token_str = token.token();

            // Skip stopwords
            if let Some(stopwords) = stopwords {
                if stopwords.contains(token_str) {
                    continue;
                }
            }

            filtered_tokens.push((*idx, token));
        }

        // Return all if len<num_elements
        if filtered_tokens.len() <= num_elements {
            return filtered_tokens;
        }

        // Return random subset of num_elements len
        filtered_tokens
            .into_iter()
            .choose_multiple(&mut rng, num_elements)
    }
}

