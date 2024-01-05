use std::collections::HashSet;
use icu_locid::Locale;
use icu_properties::{sets,exemplar_chars};
use icu_provider::DataLocale;


pub struct AlphabetModel {
    pub main: HashSet<char>,
    pub main_capitalized: HashSet<char>,
    pub index: HashSet<char>,
    pub auxiliary: HashSet<char>,
    pub punctuation: HashSet<char>,
    pub numbers: HashSet<char>,
}

impl AlphabetModel {

    pub fn new(
        main: HashSet<char>,
        main_capitalized: Option<HashSet<char>>,
        index: HashSet<char>,
        auxiliary: HashSet<char>,
        punctuation: HashSet<char>,
        numbers: HashSet<char>,
    ) -> Self {

        let main_capitalized = match main_capitalized {
            Some(main_capitalized) => main_capitalized,
            None => main.iter().map(|c| c.to_uppercase().next().unwrap()).collect(),
        };
        AlphabetModel {
            main,
            main_capitalized,
            index,
            auxiliary,
            punctuation,
            numbers,
        }
    }

    fn unicode_set_data_to_hashset(data: &sets::UnicodeSetData) -> HashSet<char> {
        HashSet::from_iter(
            data
                .as_code_point_inversion_list_string_list()
                .unwrap()
                .code_points()
                .iter_chars()
        )
    }

    /// Create alphabet using icu4x - icu_locid::Locale
    ///
    /// # Examples
    /// ```rust
    /// use icu_locid::Locale;
    /// use fast_aug::models::text::AlphabetModel;
    ///
    /// let locale = Locale::try_from_bytes(b"sr-Latn-ME").unwrap();
    /// let alphabet_model = AlphabetModel::from_locale(&locale);
    ///
    /// assert_eq!(alphabet_model.main.len(), 27);
    /// ```
    pub fn from_locale(locale: &Locale) -> Self {
        let data_locale: DataLocale = locale.clone().into();

        // TODO: validate local is valid

        let main = exemplar_chars::exemplars_main(&data_locale).expect("locale should be present");
        let index = exemplar_chars::exemplars_index(&data_locale).expect("locale should be present");
        let auxiliary = exemplar_chars::exemplars_auxiliary(&data_locale).expect("locale should be present");
        let punctuation = exemplar_chars::exemplars_punctuation(&data_locale).expect("locale should be present");
        let numbers = exemplar_chars::exemplars_numbers(&data_locale).expect("locale should be present");

        AlphabetModel::new(
            Self::unicode_set_data_to_hashset(&main),
            None,
            Self::unicode_set_data_to_hashset(&index),
            Self::unicode_set_data_to_hashset(&auxiliary),
            Self::unicode_set_data_to_hashset(&punctuation),
            Self::unicode_set_data_to_hashset(&numbers),
        )
    }

    /// Create alphabet using icu4x using language tag
    ///
    /// # Examples
    /// ```rust
    /// use fast_aug::models::text::AlphabetModel;
    ///
    /// let alphabet_model = AlphabetModel::from_locale_str(&"sr-Latn-ME");
    ///
    /// assert_eq!(alphabet_model.main.len(), 27);
    /// ```
    pub fn from_locale_str(locale_str: &str) -> Self {
        let locale = Locale::try_from_bytes(locale_str.as_bytes())
            .expect("language tag should be valid");
        Self::from_locale(&locale)
    }
}


#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::*;

    #[test_case("en")]
    #[test_case("de")]
    #[test_case("ja")]
    #[test_case("zh")]
    fn test_from_locale(locale_str: &str) {
        let locale = Locale::try_from_bytes(locale_str.as_bytes()).unwrap();

        let alphabet_model = AlphabetModel::from_locale(&locale);

        assert!(!alphabet_model.main.is_empty());
        assert!(!alphabet_model.main_capitalized.is_empty());
        assert!(!alphabet_model.index.is_empty());
        // assert!(alphabet_model.auxiliary.len() > 0);
        assert!(!alphabet_model.punctuation.is_empty());
        assert!(!alphabet_model.numbers.is_empty());
    }

    #[test_case("en")]
    #[test_case("de")]
    #[test_case("ja-JP")]
    #[test_case("zh")]
    #[test_case("sr-Latn-RS")]
    #[test_case("sr-Cyrl")]
    #[test_case("jv-Latn")]
    #[test_case("ca-ES-valencia")]
    #[test_case("en_US_POSIX")]
    #[test_case("uz_Latn_UZ")]
    fn test_from_locale_str(locale_str: &str) {
        let alphabet_model = AlphabetModel::from_locale_str(locale_str);

        assert!(!alphabet_model.main.is_empty());
        assert!(!alphabet_model.main_capitalized.is_empty());
        assert!(!alphabet_model.index.is_empty());
        // assert!(alphabet_model.auxiliary.len() > 0);
        assert!(!alphabet_model.punctuation.is_empty());
        assert!(!alphabet_model.numbers.is_empty());
    }

    #[test_case("invalid")]
    #[test_case("123")]
    #[test_case("!@#")]
    #[test_case("sr-Latn-")]
    #[test_case("sr-La!!!tn-RS")]
    fn test_from_locale_str_with_invalid_locale(locale_str: &str) {
        let result = std::panic::catch_unwind(|| {
            AlphabetModel::from_locale_str(locale_str);
        });
        assert!(result.is_err());
    }
}
