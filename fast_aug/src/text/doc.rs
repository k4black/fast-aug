use std::collections::HashSet;
use super::token::{Token, TokenType};
use unicode_segmentation::UnicodeSegmentation;


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
    /// use fast_aug::text::doc::Doc;
    /// use fast_aug::text::token::Token;
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
    /// use fast_aug::text::doc::Doc;
    /// use fast_aug::text::token::Token;
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

    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
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

    #[test_case("Hello, world!", vec!["Hello", ",", " ", "world", "!"] ; "basic latin script")]
    #[test_case("    Some\t\t    spaces", vec!["    ", "Some", "\t", "\t", "    ", "spaces"] ; "complicated spaces")]
    #[test_case("Hello\u{200A}world", vec!["Hello", "\u{200A}", "world"] ; "unicode spaces")]
    #[test_case(".!@#$%^&*()_+", vec![".", "!", "@", "#", "$", "%", "^", "&", "*", "(", ")", "_", "+"] ; "special characters")]
    #[test_case("😛🙈😆", vec!["😛", "🙈", "😆"] ; "emoji")]
    #[test_case(":clown_face: :see_no_evil: :laughing:", vec![":", "clown_face", ":", " ", ":", "see_no_evil", ":", " ", ":", "laughing", ":"] ; "emoji aliases")]
    fn test_docs_spesial_cases(input_text: &str, expected_tokens: Vec<&str>) {
        let doc = Doc::new(input_text);
        let expected_tokens = expected_tokens.iter().map(|&token| Token::from_str(token)).collect::<Vec<Token>>();
        assert_eq!(doc.tokens.len(), expected_tokens.len());
        assert_eq!(doc.tokens, expected_tokens);
    }

    // from https://generator.lorem-ipsum.info/
    #[test_case(
        "Lorem ipsum dolor sit amet, soluta commune ponderum mea ad, te qui erant vulputate, ne sed noster verear. No illud abhorreant vel, quo no amet liber tantas.",
        vec!["Lorem", " ", "ipsum", " ", "dolor", " ", "sit", " ", "amet", ",", " ", "soluta", " ", "commune", " ", "ponderum", " ", "mea", " ", "ad", ",", " ", "te", " ", "qui", " ", "erant", " ", "vulputate", ",", " ", "ne", " ", "sed", " ", "noster", " ", "verear", ".", " ", "No", " ", "illud", " ", "abhorreant", " ", "vel", ",", " ", "quo", " ", "no", " ", "amet", " ", "liber", " ", "tantas", "."]
        ; "latin script"
    )]
    #[test_case(
        "Лорем ипсум долор сит амет, пер цлита поссит ех, ат мунере фабулас петентиум сит. Иус цу цибо саперет сцрипсерит, нец виси муциус лабитур ид. Ет хис нонумес нолуиссе дигниссим.",
        vec!["Лорем", " ", "ипсум", " ", "долор", " ", "сит", " ", "амет", ",", " ", "пер", " ", "цлита", " ", "поссит", " ", "ех", ",", " ", "ат", " ", "мунере", " ", "фабулас", " ", "петентиум", " ", "сит", ".", " ", "Иус", " ", "цу", " ", "цибо", " ", "саперет", " ", "сцрипсерит", ",", " ", "нец", " ", "виси", " ", "муциус", " ", "лабитур", " ", "ид", ".", " ", "Ет", " ", "хис", " ", "нонумес", " ", "нолуиссе", " ", "дигниссим", "."]
        ; "cyrillic script"
    )]
    #[test_case(
        "국민경제의 발전을 위한 중요정책의 수립에 관하여 대통령의 자문에 응하기 위하여 국민경제자문회의를 둘 수 있다.",
        vec!["국민경제의", " ", "발전을", " ", "위한", " ", "중요정책의", " ", "수립에", " ", "관하여", " ", "대통령의", " ", "자문에", " ", "응하기", " ", "위하여", " ", "국민경제자문회의를", " ", "둘", " ", "수", " ", "있다", "."]
        ; "korean script"
    )]
    #[test_case(
        "Λορεμ ιπσθμ δολορ σιτ αμετ, μει ιδ νοvθμ φαβελλασ πετεντιθμ vελ νε, ατ νισλ σονετ οπορτερε εθμ. Αλιι δοcτθσ μει ιδ, νο αθτεμ αθδιρε ιντερεσσετ μελ, δοcενδι cομμθνε οπορτεατ τε cθμ.",
        vec!["Λορεμ", " ", "ιπσθμ", " ", "δολορ", " ", "σιτ", " ", "αμετ", ",", " ", "μει", " ", "ιδ", " ", "νοvθμ", " ", "φαβελλασ", " ", "πετεντιθμ", " ", "vελ", " ", "νε", ",", " ", "ατ", " ", "νισλ", " ", "σονετ", " ", "οπορτερε", " ", "εθμ", ".", " ", "Αλιι", " ", "δοcτθσ", " ", "μει", " ", "ιδ", ",", " ", "νο", " ", "αθτεμ", " ", "αθδιρε", " ", "ιντερεσσετ", " ", "μελ", ",", " ", "δοcενδι", " ", "cομμθνε", " ", "οπορτεατ", " ", "τε", " ", "cθμ", "."]
        ; "greek script"
    )]
    // TODO: check japanese script - why "セムレ" is not split?
    #[test_case(
        "旅ロ京青利セムレ弱改フヨス波府かばぼ意送でぼ調掲察たス日西重ケアナ住橋ユムミク順待ふかんぼ人奨貯鏡すびそ。",
        vec!["旅", "ロ", "京", "青", "利", "セムレ", "弱", "改", "フヨス", "波", "府", "か", "ば", "ぼ", "意", "送", "で", "ぼ", "調", "掲", "察", "た", "ス", "日", "西", "重", "ケアナ", "住", "橋", "ユムミク", "順", "待", "ふ", "か", "ん", "ぼ", "人", "奨", "貯", "鏡", "す", "び", "そ", "。"]
        ; "japanese script"
    )]
    #[test_case(
        "側経意責家方家閉討店暖育田庁載社転線宇。得君新術治温抗添代話考振投員殴大闘北裁。品間識部案代学凰処済準世一戸刻法分。悼測済諏計飯利安凶断理資沢同岩面文認革。内警格化再薬方久化体教御決数詭芸得筆代。",
        vec!["側", "経", "意", "責", "家", "方", "家", "閉", "討", "店", "暖", "育", "田", "庁", "載", "社", "転", "線", "宇", "。", "得", "君", "新", "術", "治", "温", "抗", "添", "代", "話", "考", "振", "投", "員", "殴", "大", "闘", "北", "裁", "。", "品", "間", "識", "部", "案", "代", "学", "凰", "処", "済", "準", "世", "一", "戸", "刻", "法", "分", "。", "悼", "測", "済", "諏", "計", "飯", "利", "安", "凶", "断", "理", "資", "沢", "同", "岩", "面", "文", "認", "革", "。", "内", "警", "格", "化", "再", "薬", "方", "久", "化", "体", "教", "御", "決", "数", "詭", "芸", "得", "筆", "代", "。"]
        ; "chinese script"
    )]
    #[test_case(
        "पढाए हिंदी रहारुप अनुवाद कार्यलय मुख्य संस्था सोफ़तवेर निरपेक्ष उनका आपके बाटते आशाआपस मुख्यतह उशकी करता। शुरुआत संस्था कुशलता मेंभटृ अनुवाद गएआप विशेष सकते परिभाषित लाभान्वित प्रति देकर समजते दिशामे प्राप्त जैसे वर्णन संस्थान निर्माता प्रव्रुति भाति चुनने उपलब्ध बेंगलूर अर्थपुर्ण",
        vec!["पढाए", " ", "हिंदी", " ", "रहारुप", " ", "अनुवाद", " ", "कार्यलय", " ", "मुख्य", " ", "संस्था", " ", "सोफ़तवेर", " ", "निरपेक्ष", " ", "उनका", " ", "आपके", " ", "बाटते", " ", "आशाआपस", " ", "मुख्यतह", " ", "उशकी", " ", "करता", "।", " ", "शुरुआत", " ", "संस्था", " ", "कुशलता", " ", "मेंभटृ", " ", "अनुवाद", " ", "गएआप", " ", "विशेष", " ", "सकते", " ", "परिभाषित", " ", "लाभान्वित", " ", "प्रति", " ", "देकर", " ", "समजते", " ", "दिशामे", " ", "प्राप्त", " ", "जैसे", " ", "वर्णन", " ", "संस्थान", " ", "निर्माता", " ", "प्रव्रुति", " ", "भाति", " ", "चुनने", " ", "उपलब्ध", " ", "बेंगलूर", " ", "अर्थपुर्ण"]
        ; "hindi script"
    )]
    #[test_case(
        "լոռեմ իպսում դոլոռ սիթ ամեթ, լաբոռե մոդեռաթիուս եթ հաս, պեռ ոմնիս լաթինե դիսպութաթիոնի աթ, վիս ֆեուգաիթ ծիվիբուս եխ. վիվենդում լաբոռամուս ելաբոռառեթ նամ ին.",
        vec!["լոռեմ", " ", "իպսում", " ", "դոլոռ", " ", "սիթ", " ", "ամեթ", ",", " ", "լաբոռե", " ", "մոդեռաթիուս", " ", "եթ", " ", "հաս", ",", " ", "պեռ", " ", "ոմնիս", " ", "լաթինե", " ", "դիսպութաթիոնի", " ", "աթ", ",", " ", "վիս", " ", "ֆեուգաիթ", " ", "ծիվիբուս", " ", "եխ", ".", " ", "վիվենդում", " ", "լաբոռամուս", " ", "ելաբոռառեթ", " ", "նամ", " ", "ին", "."]
        ; "armenian script"
    )]
    #[test_case(
        "غينيا واستمر العصبة ضرب قد. وباءت الأمريكي الأوربيين هو به،, هو العالم، الثقيلة بال. مع وايرلندا الأوروبيّون كان, قد بحق أسابيع العظمى واعتلاء. انه كل وإقامة المواد.",
        vec!["غينيا", " ", "واستمر", " ", "العصبة", " ", "ضرب", " ", "قد", ".", " ", "وباءت", " ", "الأمريكي", " ", "الأوربيين", " ", "هو", " ", "به", "،", ",", " ", "هو", " ", "العالم", "،", " ", "الثقيلة", " ", "بال", ".", " ", "مع", " ", "وايرلندا", " ", "الأوروبيّون", " ", "كان", ",", " ", "قد", " ", "بحق", " ", "أسابيع", " ", "العظمى", " ", "واعتلاء", ".", " ", "انه", " ", "كل", " ", "وإقامة", " ", "المواد", "."]
        ; "arabic script"
    )]
    #[test_case(
        "כדי יסוד מונחים מועמדים של, דת דפים מאמרשיחהצפה זאת. אתה דת שונה כלשהו, גם אחר ליום בשפות, או ניווט פולנית לחיבור ארץ. ויש בקלות ואמנות אירועים או, אל אינו כלכלה שתי.",
        vec!["כדי", " ", "יסוד", " ", "מונחים", " ", "מועמדים", " ", "של", ",", " ", "דת", " ", "דפים", " ", "מאמרשיחהצפה", " ", "זאת", ".", " ", "אתה", " ", "דת", " ", "שונה", " ", "כלשהו", ",", " ", "גם", " ", "אחר", " ", "ליום", " ", "בשפות", ",", " ", "או", " ", "ניווט", " ", "פולנית", " ", "לחיבור", " ", "ארץ", ".", " ", "ויש", " ", "בקלות", " ", "ואמנות", " ", "אירועים", " ", "או", ",", " ", "אל", " ", "אינו", " ", "כלכלה", " ", "שתי", "."]
        ; "hebrew script"
    )]
    fn test_multilingual_lorem_ipsum(input_text: &str, expected_tokens: Vec<&str>) {
        let doc = Doc::new(input_text);
        let expected_tokens = expected_tokens.iter().map(|&token| Token::from_str(token)).collect::<Vec<Token>>();
        assert_eq!(doc.tokens.len(), expected_tokens.len());
        assert_eq!(doc.tokens, expected_tokens);
    }

    #[test_case(vec!["Hello", ",", " ", "world", "!"] ; "latin script")]
    #[test_case(vec!["    ", "Some", "\t", "\t", "    ", "spaces"] ; "complicated spaces")]
    fn test_docs_from_tokens(tokens: Vec<&str>) {
        let doc = Doc::from_tokens(tokens.clone());
        let expected_tokens = tokens.iter().map(|&token| Token::from_str(token)).collect::<Vec<Token>>();
        assert_eq!(doc.tokens.len(), tokens.len());
        assert_eq!(doc.tokens, expected_tokens);
    }

    #[test_case("Hello, world!" ; "latin script")]
    #[test_case("    Some\t\t    spaces" ; "complicated spaces")]
    #[test_case("Hello\u{200A}world" ; "unicode spaces")]
    #[test_case("Лорем ипсум, юсто дицтас еи." ; "cyrillic script")]
    #[test_case("下姐，做兒采。" ; "chinese script")]
    #[test_case("قد فاتّبع وإعلان حدى. نقطة سقوط" ; "arabic script")]
    #[test_case(".!@#$%^&*()_+" ; "special characters")]
    fn test_to_string(text: &str) {
        let text_copy = text.to_string();
        let doc = Doc::new(text);
        assert_eq!(doc.to_string(), text_copy);
    }

    #[test_case(vec!["A", ",", " ", "B", "!"], false, 2 ; "only words")]
    #[test_case(vec!["A", ",", " ", "B", "!"], true, 4 ; "words with special chars")]
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

    #[test_case(vec!["A", "B", "C", "D"], false, vec!["A"], vec![1, 2, 3] ; "only words filter single word")]
    #[test_case(vec!["A", "B", "C", "D"], false, vec![], vec![0, 1, 2, 3] ; "only words filter empty")]
    #[test_case(vec!["A", ",", " ", "B", "C", "!"], true, vec!["A", "B"], vec![1, 4, 5] ; "words with special chars filter multiple")]
    #[test_case(vec!["A", ",", " ", "B", "C", "!"], false, vec!["A", "B"], vec![4] ; "filter existing")]
    #[test_case(vec!["A", ",", " ", "B", "C", "!"], false, vec!["C", "E", "R"], vec![0, 3] ; "filter not existing")]
    #[test_case(vec!["A", "B", "C", "D"], true, vec!["A", "B", "C", "D", "F"], vec![] ; "all words filter all")]
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
