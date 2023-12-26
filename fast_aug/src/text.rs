mod base;
pub mod doc;
pub mod token;
mod spelling;
mod parameters;
mod random_words_aug;
mod random_chars_aug;

pub use base::{BaseTextAugmenter, TextAction};
pub use random_words_aug::RandomWordsAugmenter;
pub use random_chars_aug::RandomCharsAugmenter;
pub use parameters::TextAugmentParameters;
