use std::collections::HashSet;
use rand::thread_rng;
use super::token::Token;
use unicode_segmentation::UnicodeSegmentation;
use crate::text::TokenType;


/// Doc struct holds content as a list of tokens.
/// TODO: Lazy loading of tokens while not requested.
pub struct Doc {
    pub tokens: Vec<Token>,
    pub num_changes: usize,
}

impl Doc {
    /// Create a new Doc from a string slice.
    /// Automatically tokenizes the text on word boundaries (words, spaces, and special symbols).
    /// See https://www.unicode.org/reports/tr29/#Word_Boundaries
    ///
    /// # Arguments
    /// * `text` - A string slice that holds the text to be tokenized.
    ///
    /// # Examples
    /// ```rust
    /// use fast_aug::text::Doc;
    /// use fast_aug::text::Token;
    ///
    /// let doc = Doc::new("Hello,  world!");
    /// let expected_tokens = vec!["Hello", ",", "  ", "world", "!"].iter().map(|&token| Token::from_str(token)).collect::<Vec<Token>>();
    /// assert_eq!(doc.tokens, expected_tokens);
    /// ```
    pub fn new(text: &str) -> Self {
        let tokens = Doc::tokenize(text);
        Doc {
            tokens,
            num_changes: 0,
        }
    }

    /// Create a new Doc from a list of tokens.
    /// Select token type automatically.
    ///
    /// # Arguments
    /// * `tokens` - A vector of string slices that holds the tokens.
    ///
    /// # Examples
    /// ```rust
    /// use fast_aug::text::Doc;
    /// use fast_aug::text::Token;
    ///
    /// let doc = Doc::from_tokens(vec!["Hello", ",", "  ", "world", "!"]);
    /// let expected_tokens = vec!["Hello", ",", "  ", "world", "!"].iter().map(|&token| Token::from_str(token)).collect::<Vec<Token>>();
    /// assert_eq!(doc.tokens, expected_tokens);
    /// ```
    pub fn from_tokens(tokens: Vec<&str>) -> Self {
        let tokens = tokens.iter().map(|&token| Token::from_str(token)).collect::<Vec<Token>>();
        Doc {
            tokens,
            num_changes: 0,
        }
    }

    /// Tokenize a string slice on word boundaries (words, spaces, and special symbols).
    /// Use "Unicode Standard Annex #29" https://www.unicode.org/reports/tr29/#Word_Boundaries
    fn tokenize(text: &str) -> Vec<Token> {
        UnicodeSegmentation::split_word_bounds(text)
            .map(Token::from_str)
            .collect()
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    /// Convert Doc to string
    pub fn to_string(&self) -> String {
        self.tokens.iter().map(|token| token.token().as_str()).collect::<Vec<&str>>().join("")
    }

    /// Calculate number of word tokens
    ///
    /// # Arguments
    /// * `include_special_char` - Include Special tokens in count
    pub fn get_word_tokens_count(&self, include_special_char: bool) -> usize {
        let mut count = 0;
        for token in self.tokens.iter() {
            let token_type = token.kind();
            match (token_type, include_special_char) {
                (TokenType::Word, _) => count += 1,
                (TokenType::Special, true) => count += 1,
                (_, _) => (),
            }
        }
        count
    }

    /// Get only WordTokens original indexes
    ///
    /// # Arguments
    /// * `include_special_char` - Include Special tokens in count
    /// * `stopwords` - A HashSet of stopwords to be skipped
    pub fn get_word_indexes(&mut self, include_special_char: bool, stopwords: Option<&HashSet<String>>) -> Vec<usize> {
        let mut word_indexes = Vec::with_capacity(self.tokens.len());

        for (idx, token) in self.tokens.iter().enumerate() {
            let token_type = token.kind();
            match (token_type, include_special_char) {
                (TokenType::Word, _) => {
                    if let Some(stopwords) = stopwords {
                        if stopwords.contains(token.token()) {
                            continue;
                        }
                    }
                    word_indexes.push(idx);
                },
                (TokenType::Special, true) => word_indexes.push(idx),
                (_, _) => (),
            }
        }

        word_indexes
    }

    /// Swap two tokens in Doc - in-place
    ///
    /// # Arguments
    /// * `idx_a` - Index of first token
    /// * `idx_b` - Index of second token
    pub fn swap_tokens_by_index(&mut self, idx_a: usize, idx_b: usize) {
        self.tokens.swap(idx_a, idx_b);
    }
}


#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::*;

    #[test_case("Hello, world!", vec!["Hello", ",", " ", "world", "!"])]
    #[test_case("    Some\t\t    spaces", vec!["    ", "Some", "\t", "\t", "    ", "spaces"])]
    #[test_case("Hello\u{200A}world", vec!["Hello", "\u{200A}", "world"])]
    #[test_case("Лорем ипсум, юсто дицтас еи.", vec!["Лорем", " ", "ипсум", ",", " ", "юсто", " ", "дицтас", " ", "еи", "."])]
    #[test_case("下姐，做兒采。", vec!["下", "姐", "，", "做", "兒", "采", "。"])]
    #[test_case("قد فاتّبع وإعلان حدى. نقطة سقوط", vec!["قد", " ", "فاتّبع", " ", "وإعلان", " ", "حدى", ".", " ", "نقطة", " ", "سقوط"])]
    #[test_case(".!@#$%^&*()_+", vec![".", "!", "@", "#", "$", "%", "^", "&", "*", "(", ")", "_", "+"])]
    fn test_docs_tokenizes_text(text: &str, tokens: Vec<&str>) {
        let doc = Doc::new(text);
        let expected_tokens = tokens.iter().map(|&token| Token::from_str(token)).collect::<Vec<Token>>();
        assert_eq!(doc.tokens.len(), tokens.len());
        assert_eq!(doc.tokens, expected_tokens);
    }

    #[test_case(vec!["Hello", ",", " ", "world", "!"])]
    #[test_case(vec!["    ", "Some", "\t", "\t", "    ", "spaces"])]
    fn test_docs_from_tokens(tokens: Vec<&str>) {
        let doc = Doc::from_tokens(tokens.clone());
        let expected_tokens = tokens.iter().map(|&token| Token::from_str(token)).collect::<Vec<Token>>();
        assert_eq!(doc.tokens.len(), tokens.len());
        assert_eq!(doc.tokens, expected_tokens);
    }

    #[test_case("Hello, world!")]
    #[test_case("    Some\t\t    spaces")]
    #[test_case("Hello\u{200A}world")]
    #[test_case("Лорем ипсум, юсто дицтас еи.")]
    #[test_case("下姐，做兒采。")]
    #[test_case("قد فاتّبع وإعلان حدى. نقطة سقوط")]
    #[test_case(".!@#$%^&*()_+")]
    fn test_to_string(text: &str) {
        let text_copy = text.to_string();
        let doc = Doc::new(text);
        assert_eq!(doc.to_string(), text_copy);
    }

    #[test_case(vec!["Hello", ",", " ", "world", "!"], false, 2)]
    #[test_case(vec!["Hello", ",", " ", "world", "!"], true, 4)]
    #[test_case(vec!["下", "姐", "，", "做", "兒", "采", "。"], false, 5)]
    #[test_case(vec!["下", "姐", "，", "做", "兒", "采", "。"], true, 7)]
    #[test_case(vec!["قد", " ", "فاتّبع", " ", "وإعلان", " ", "حدى", ".", " ", "نقطة", " ", "سقوط"], false, 6)]
    #[test_case(vec!["قد", " ", "فاتّبع", " ", "وإعلان", " ", "حدى", ".", " ", "نقطة", " ", "سقوط"], true, 7)]
    fn test_word_tokens_count(tokens: Vec<&str>, include_special_char: bool, expected: usize) {
        let doc = Doc::from_tokens(tokens);
        assert_eq!(doc.get_word_tokens_count(include_special_char), expected);
    }

    #[test_case(vec!["A", "B", "C", "D"], false, vec![0, 1, 2, 3])]
    #[test_case(vec!["A", "B", "C", "D"], true, vec![0, 1, 2, 3])]
    #[test_case(vec!["A", ",", " ", "B", "C", "!"], false, vec![0, 3, 4])]
    #[test_case(vec!["A", ",", " ", "B", "C", "!"], true, vec![0, 1, 3, 4, 5])]
    fn test_get_word_indexes(tokens: Vec<&str>, include_special_char: bool, expected: Vec<usize>) {
        let mut doc = Doc::from_tokens(tokens);
        let word_tokens = doc.get_word_indexes(include_special_char, None);
        assert_eq!(word_tokens, expected);
    }

    #[test_case(vec!["A", "B", "C", "D"], false, vec!["A"], vec![1, 2, 3])]
    #[test_case(vec!["A", "B", "C", "D"], false, vec![], vec![0, 1, 2, 3])]
    #[test_case(vec!["A", ",", " ", "B", "C", "!"], true, vec!["A", "B"], vec![1, 4, 5])]
    #[test_case(vec!["A", ",", " ", "B", "C", "!"], false, vec!["A", "B"], vec![4])]
    #[test_case(vec!["A", "B", "C", "D"], true, vec!["A", "B", "C", "D"], vec![])]
    fn test_get_word_indexes_without_stopwords(tokens: Vec<&str>, include_special_char: bool, stopwords: Vec<&str>, expected: Vec<usize>) {
        let stopwords = stopwords.iter().map(|&token| token.to_string()).collect::<HashSet<String>>();
        let mut doc = Doc::from_tokens(tokens);
        let word_tokens = doc.get_word_indexes(include_special_char, Some(&stopwords));
        assert_eq!(word_tokens, expected);
    }

    #[test_case("A B, C D", 0, 2, "B A, C D")]
    #[test_case("A B, C D", 0, 1, " AB, C D")]
    #[test_case("A B, C D", 2, 3, "A ,B C D")]
    #[test_case("A B, C D", 7, 1, "ADB, C  ")]
    fn test_swap_tokens_by_index(text: &str, idx_a: usize, idx_b: usize, expected: &str) {
        let mut doc = Doc::new(text);
        doc.swap_tokens_by_index(idx_a, idx_b);
        assert_eq!(doc.to_string(), expected);
    }
}
