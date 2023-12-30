mod base;
pub mod doc;
pub mod token;
mod parameters;
mod random_words_aug;
mod random_chars_aug;
mod tf_idf_words_aug;
mod spelling_words_aug;
mod embeddings_words_aug;

pub use base::{BaseTextAugmenter, TextAction};
pub use random_words_aug::RandomWordsAugmenter;
pub use random_chars_aug::RandomCharsAugmenter;
pub use tf_idf_words_aug::TfIdfWordsAugmenter;
pub use spelling_words_aug::SpellingWordsAugmenter;
pub use parameters::TextAugmentParameters;
