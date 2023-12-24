// use std::collections::HashMap;
// use crate::text::base::TextAugmenter;
// use self::super::doc::Doc;
//
// pub struct SpellingErrorAugmenter {
//     grammar_dict: HashMap<String, Vec<String>>,
// }
//
// impl TextAugmenter for SpellingErrorAugmenter {
//     fn augment_inner(&self, input: Doc) -> Doc {
//         let mut output = input.clone();
//         for token in output.tokens.iter_mut() {
//             if token.token_type() == TokenType::Word {
//                 let token_str = token.token();
//                 if let Some(replacements) = self.grammar_dict.get(token_str) {
//                     let replacement = replacements.choose(&mut rand::thread_rng()).unwrap();
//                     token.set_token(replacement);
//                 }
//             }
//         }
//         output
//     }
// }