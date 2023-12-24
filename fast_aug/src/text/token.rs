use regex::Regex;

/// Token types
///
/// TokenType::Word    -> Any Word token
/// TokenType::Space   -> Any Space token (continuous whitespace counts as 1 token)
/// TokenType::Special -> Any Special token (non alphanumeric chars, digits, #!.$~, etc.)
/// TokenType::Deleted -> Any Deleted token (token that was deleted by some augmenter, but keep for optimization)
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TokenType {
    Word,
    Space,
    Special,
    Deleted,
}

/// Struct that stores token, it's type and it's lexicographic length
///
/// Note: lexicographic length != bytes length, as a lot of non-english chars encode to more than 1 byte
/// TODO: Make lazy token ownership, while not edited - just store the string slice
#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    kind: TokenType,
    token: String,
    token_len: usize,
}

impl Token {
    /// Create a new Token from a string slice.
    /// Automatically classifies the token on words, spaces, and special symbols.
    ///
    /// # Arguments
    /// * `token` - A string slice that holds the token content.
    ///
    /// # Examples
    /// ```rust
    /// use fast_aug::text::Token;
    /// use fast_aug::text::TokenType;
    ///
    /// let token = Token::from_str("Hello");
    /// assert_eq!(token.kind(), &TokenType::Word);
    /// let token = Token::from_str("    ");
    /// assert_eq!(token.kind(), &TokenType::Space);
    /// let token = Token::from_str("!@#$%^&*()");
    /// assert_eq!(token.kind(), &TokenType::Special);
    pub fn from_str(token: &str) -> Self {
        let token_len = token.chars().count();
        let kind = Token::classify_token_by_any_chars(&token);
        Token {
            kind,
            token: token.to_string(),
            token_len,
        }
    }

    /// Create a new Token from a string slice and token type.
    pub fn new(token: &str, kind: TokenType) -> Self {
        let token_len = token.chars().count();
        Token {
            kind,
            token: token.to_string(),
            token_len,
        }
    }

    /// Change token content
    pub fn change(&mut self, token: &str, kind: TokenType) {
        self.token = token.to_string();
        self.token_len = token.chars().count();
        self.kind = kind;
    }

    /// Classify token on TokenTypes. Check any char is:
    ///   empty -> Deleted
    ///   any alphabetic -> Word
    ///   any whitespace -> Space
    ///   else -> Special
    /// Medium speed
    fn classify_token_by_any_chars(token: &str) -> TokenType {
        if token.is_empty() {
            TokenType::Deleted
        } else if token.chars().any(|c| c.is_alphabetic()) {
            TokenType::Word
        } else if token.chars().any(|c| c.is_whitespace()) {
            TokenType::Space
        } else {
            TokenType::Special
        }
    }

    /// Classify token on TokenTypes. Check first char is:
    ///   empty -> Deleted
    ///   whitespace -> Space
    ///   only special -> Special
    ///   else -> Word
    /// Fastest speed
    #[allow(dead_code, unused_variables, unreachable_code)]
    fn classify_token_by_first_chart(token: &str) -> TokenType {
        panic!("Not implemented");
        match token.chars().next() {
            None => TokenType::Deleted,
            Some(c) if c.is_alphabetic() => TokenType::Word,
            Some(c) if c.is_whitespace() => TokenType::Space,
            _ => TokenType::Special,
        }
    }

    /// Classify token on TokenTypes. Check token matches:
    ///   empty -> Deleted
    ///   word regex -> Word
    ///   space regex -> Space
    ///   else -> Special
    /// Slowest speed
    #[allow(dead_code, unused_variables, unreachable_code)]
    fn classify_token_by_regex(token: &str, re_word: &Regex, re_space: &Regex) -> TokenType {
        panic!("Not implemented");
        if token.is_empty() {
            TokenType::Deleted
        } else if re_word.is_match(token) {
            TokenType::Word
        } else if re_space.is_match(token) {
            TokenType::Space
        } else {
            TokenType::Special
        }
    }

    /// Get respective TokenType
    pub fn kind(&self) -> &TokenType {
        &self.kind
    }

    /// Get token content
    pub fn token(&self) -> &String {
        &self.token
    }

    /// Get lexicographic length
    pub fn utf8_len(&self) -> usize {
        self.token_len
    }

    /// Get bytes length
    pub fn byte_len(&self) -> usize {
        self.token.len()
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::*;

    #[test_case("6", TokenType::Word, 1, 1 ; "single char")]
    #[test_case("4Kadf3321s", TokenType::Word, 10, 10 ; "multiple chars")]
    #[test_case("نشأت", TokenType::Word, 4, 8 ; "non-english chars taking 2 bytes")]
    #[test_case("假", TokenType::Word, 1, 3 ; "non-english chars taking 3 bytes")]
    fn test_token_interface(token: &str, kind: TokenType, utf8_len: usize, byte_len: usize) {
        let kind_clone = kind.clone();
        let token_obj = Token::new(token, kind);
        assert_eq!(token_obj.kind(), &kind_clone);
        assert_eq!(token_obj.token(), &token);
        assert_eq!(token_obj.utf8_len(), utf8_len);
        assert_eq!(token_obj.byte_len(), byte_len);
    }

    #[test_case("123", TokenType::Word, "456", TokenType::Word ; "digits to digits")]
    #[test_case("hello", TokenType::Word, "world", TokenType::Word ; "alphabetic to alphabetic")]
    #[test_case("don't", TokenType::Word, "", TokenType::Deleted ; "word with apostrophe to empty")]
    #[test_case("!", TokenType::Special, "word", TokenType::Word ; "special to alphabetic")]
    fn test_change_token(token: &str, kind: TokenType, new_token: &str, new_kind: TokenType) {
        let target_token = Token::new(new_token, new_kind.clone());
        let mut token_obj = Token::new(token, kind);
        token_obj.change(new_token, new_kind);
        assert_eq!(token_obj, target_token);
    }


    #[test_case("6", TokenType::Special ; "single digit")]
    #[test_case("123", TokenType::Special ; "only digits")]
    #[test_case("hello", TokenType::Word ; "only alphabetic")]
    #[test_case("don't", TokenType::Word ; "word with apostrophe")]
    #[test_case("'cause", TokenType::Word ; "word with starting apostrophe")]
    #[test_case("привет", TokenType::Word ; "only cyrillic")]
    #[test_case("نشأت", TokenType::Word ; "only arabic")]
    #[test_case("假", TokenType::Word ; "only chinese")]
    #[test_case(" ", TokenType::Space ; "only whitespace")]
    #[test_case("", TokenType::Deleted ; "empty")]
    #[test_case("!", TokenType::Special ; "single special")]
    #[test_case("&!*", TokenType::Special ; "multiple special")]
    #[test_case("\t", TokenType::Space ; "tab")]
    #[test_case("\n", TokenType::Space ; "newline")]
    fn test_token_classification(token: &str, kind: TokenType) {
        // Using method
        let kind_all = Token::classify_token_by_any_chars(token);
        assert_eq!(kind_all, kind);

        // Using Token constructor
        let token_obj = Token::from_str(token);
        assert_eq!(token_obj.kind(), &kind);
    }

}
