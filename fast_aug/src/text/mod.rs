mod base;
mod chars_keyboard;
mod chars_random;
mod utils;
mod words_random;
mod words_spelling;
mod words_tf_idf;

pub use base::{BaseTextAugmenter, TextAction};
pub use chars_keyboard::CharsKeyboardAugmenter;
pub use chars_random::CharsRandomAugmenter;
pub use utils::{Doc, TextAugmentParameters, Token, TokenType};
pub use words_random::WordsRandomAugmenter;
pub use words_spelling::WordsSpellingAugmenter;
pub use words_tf_idf::WordsTfIdfAugmenter;
