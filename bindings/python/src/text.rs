use fast_aug_rust::text::CharsRandomAugmenter;
use fast_aug_rust::text::TextAction;
use fast_aug_rust::text::TextAugmentParameters;
use fast_aug_rust::text::WordsRandomAugmenter;
use std::collections::HashSet;
use std::panic;
use std::sync::Arc;

use crate::base::{AugmenterTypes, PyBaseAugmenter};
use fast_aug_rust::models::text::AlphabetModel;
use pyo3::exceptions::{PyNotImplementedError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyAny;
use rand::rngs::SmallRng;
use rand::SeedableRng;

#[derive(FromPyObject)]
enum PyConvertTextAugmentParameters {
    Float(f32),
    Tuple(f32, Option<usize>, Option<usize>),
}

#[allow(clippy::from_over_into)]
impl Into<TextAugmentParameters> for PyConvertTextAugmentParameters {
    fn into(self) -> TextAugmentParameters {
        match self {
            PyConvertTextAugmentParameters::Float(p) => TextAugmentParameters::new(p, None, None),
            PyConvertTextAugmentParameters::Tuple(p, min_elements, max_elements) => {
                TextAugmentParameters::new(p, min_elements, max_elements)
            }
        }
    }
}

#[pyclass]
#[pyo3(name = "TextAction")]
#[derive(Clone, Copy)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
enum PyTextAction {
    INSERT = 0,
    SUBSTITUTE = 1,
    SWAP = 2,
    DELETE = 3,
}

#[derive(FromPyObject)]
enum PyConvertTextAction {
    String(String),
    TextAction(PyTextAction),
}

#[allow(clippy::from_over_into)]
impl Into<TextAction> for PyConvertTextAction {
    fn into(self) -> TextAction {
        match self {
            PyConvertTextAction::String(action) => match action.to_lowercase().as_str() {
                "insert" => TextAction::Insert,
                "substitute" => TextAction::Substitute,
                "swap" => TextAction::Swap,
                "delete" => TextAction::Delete,
                _ => panic!("Action not implemented"),
            },
            PyConvertTextAction::TextAction(action) => match action {
                PyTextAction::INSERT => TextAction::Insert,
                PyTextAction::SUBSTITUTE => TextAction::Substitute,
                PyTextAction::SWAP => TextAction::Swap,
                PyTextAction::DELETE => TextAction::Delete,
            },
        }
    }
}

/// Abstract Base Class for Text Augmentation
#[pyclass(extends=PyBaseAugmenter, subclass)]
#[pyo3(name = "BaseTextAugmenter")]
pub struct PyBaseTextAugmenter;

#[pymethods]
impl PyBaseTextAugmenter {
    #[new]
    #[pyo3(text_signature = "(self)")]
    fn py_new() -> PyResult<(Self, PyBaseAugmenter)> {
        Err(PyNotImplementedError::new_err("Not implemented"))
    }

    /// Augment the data
    /// :param data: The single String to augment
    /// :return: The augmented data
    #[pyo3(text_signature = "(self, data: str)")]
    fn augment(&self, _data: &PyAny) -> PyResult<PyObject> {
        Err(PyNotImplementedError::new_err("Not implemented"))
    }

    /// Augment data given a batch of data
    /// :param data: Vector of Strings to augment
    /// :returns: Augmented data
    #[pyo3(text_signature = "(self, data: list[Any])")]
    fn augment_batch(&self, _data: Vec<&PyAny>) -> PyResult<Vec<PyObject>> {
        Err(PyNotImplementedError::new_err("Not implemented"))
    }
}

/// Randomly augment chars in the words
/// :param action: The action to perform - insert, substitute, swap, delete
/// :param aug_params_word: The parameters for the word augmentation
///     - probability or (probability, min_elements, max_elements)
/// :param aug_params_char: The parameters for the char augmentation
///     - probability or (probability, min_elements, max_elements)
/// :param stopwords: The set of stopwords to ignore
/// :param locale: The locale string to use for alphabet, optional. Required for insert and substitute
#[pyclass(extends=PyBaseTextAugmenter)]
#[pyo3(name = "CharsRandomAugmenter")]
pub struct PyCharsRandomAugmenter;

#[pymethods]
impl PyCharsRandomAugmenter {
    #[new]
    #[pyo3(
        text_signature = "(self, action: str | TextAction, aug_params_word: float | tuple[float, int | None, int | None] | None = None, aug_params_char: float | tuple[float, int | None, int | None] | None = None, stopwords: set[str] | None = None, locale: str | None = None)"
    )]
    fn py_new(
        action: PyConvertTextAction,
        aug_params_word: Option<PyConvertTextAugmentParameters>,
        aug_params_char: Option<PyConvertTextAugmentParameters>,
        stopwords: Option<HashSet<String>>,
        locale: Option<String>,
    ) -> PyResult<PyClassInitializer<Self>> {
        let rng = SmallRng::from_entropy(); // TODO: make from seed

        // Process parameters
        let action = panic::catch_unwind(|| action.into());
        if action.is_err() {
            return Err(PyErr::new::<PyValueError, _>("Action not implemented"));
        }
        let action = action.unwrap();
        let aug_params_word = match aug_params_word {
            Some(p) => p.into(),
            None => TextAugmentParameters::default(),
        };
        let aug_params_char = match aug_params_char {
            Some(p) => p.into(),
            None => TextAugmentParameters::default(),
        };

        // Get alphabet model
        let alphabet_model = locale.map(|locale| AlphabetModel::from_locale_str(&locale));

        match action {
            TextAction::Insert | TextAction::Substitute => {
                if alphabet_model.is_none() {
                    return Err(PyErr::new::<PyValueError, _>(
                        "Locale must be provided for insert and substitute",
                    ));
                }
            }
            _ => (),
        };

        // Create Rust object of AugmenterTypes
        let rust_augmenter = AugmenterTypes::Text(Arc::new(CharsRandomAugmenter::new(
            action,
            aug_params_word,
            aug_params_char,
            stopwords,
            alphabet_model,
            false,
        )));

        // Create Python object with respective parent classes
        Ok(PyClassInitializer::from(PyBaseAugmenter {
            inner: rust_augmenter,
            rng,
        })
        .add_subclass(PyBaseTextAugmenter {})
        .add_subclass(PyCharsRandomAugmenter {}))
    }

    /// Augment the data
    /// :param data: A String to augment
    /// :return: The augmented data
    #[pyo3(text_signature = "(self, data: str)")]
    fn augment(self_: PyRefMut<'_, Self>, data: String) -> PyResult<String> {
        // Get base class
        let mut super_text = self_.into_super();
        let super_base = super_text.as_mut();
        // Get inner Rust object
        let rust_augmenter = match &super_base.inner {
            AugmenterTypes::Text(augmenter) => augmenter,
            _ => panic!("Augmenter is not a TextAugmenter"),
        };
        // Call original augment function
        Ok(rust_augmenter.augment(data, &mut super_base.rng))
    }

    /// Augment the data given a batch
    /// :param data: The list of Strings to augment
    /// :return: The augmented data
    #[pyo3(text_signature = "(self, data: list[str])")]
    fn augment_batch(self_: PyRefMut<'_, Self>, data: Vec<String>) -> PyResult<Vec<String>> {
        // Get base class
        let mut super_text = self_.into_super();
        let super_base = super_text.as_mut();
        // Get inner Rust object
        let rust_augmenter = match &super_base.inner {
            AugmenterTypes::Text(augmenter) => augmenter,
            _ => panic!("Augmenter is not a TextAugmenter"),
        };
        // Call original augment function
        // TODO: make batch augmenter, not just a loop
        let mut augmented_data = Vec::with_capacity(data.len());
        for d in data {
            augmented_data.push(rust_augmenter.augment(d, &mut super_base.rng));
        }
        Ok(augmented_data)
    }
}

/// Randomly augment the words
/// :param action: The action to perform - insert, substitute, swap, delete
/// :param aug_params_word: The parameters for the word augmentation
///     - probability or (probability, min_elements, max_elements)
/// :param stopwords: The set of stopwords to ignore
/// :param vocab: The optional vocabulary to use for insert and substitute
#[pyclass(extends=PyBaseTextAugmenter)]
#[pyo3(name = "WordsRandomAugmenter")]
pub struct PyWordsRandomAugmenter;

#[pymethods]
impl PyWordsRandomAugmenter {
    #[new]
    #[pyo3(
        text_signature = "(self, action: str | TextAction, aug_params_word: float | tuple[float, int | None, int | None] | None = None, stopwords: set[str] | None = None, vocabulary: list[str] | None = None)"
    )]
    fn py_new(
        action: PyConvertTextAction,
        aug_params_word: Option<PyConvertTextAugmentParameters>,
        stopwords: Option<HashSet<String>>,
        vocabulary: Option<Vec<String>>,
    ) -> PyResult<PyClassInitializer<Self>> {
        let rng = SmallRng::from_entropy(); // TODO: make from seed

        // Process parameters
        let action = panic::catch_unwind(|| action.into());
        if action.is_err() {
            return Err(PyErr::new::<PyValueError, _>("Action not implemented"));
        }
        let action = action.unwrap();
        let aug_params_word = match aug_params_word {
            Some(p) => p.into(),
            None => TextAugmentParameters::default(),
        };

        match action {
            TextAction::Insert | TextAction::Substitute => {
                if vocabulary.is_none() {
                    return Err(PyErr::new::<PyValueError, _>(
                        "Vocabulary must be provided for insert and substitute",
                    ));
                }
            }
            _ => (),
        };

        // Create Rust object of AugmenterTypes
        let rust_augmenter = AugmenterTypes::Text(Arc::new(WordsRandomAugmenter::new(
            action,
            aug_params_word,
            stopwords,
            vocabulary,
        )));

        // Create Python object with respective parent classes
        Ok(PyClassInitializer::from(PyBaseAugmenter {
            inner: rust_augmenter,
            rng,
        })
        .add_subclass(PyBaseTextAugmenter {})
        .add_subclass(PyWordsRandomAugmenter {}))
    }

    /// Augment the data
    /// :param data: A String to augment
    /// :return: The augmented data
    #[pyo3(text_signature = "(self, data: str)")]
    fn augment(self_: PyRefMut<'_, Self>, data: String) -> PyResult<String> {
        // Get base class
        let mut super_text = self_.into_super();
        let super_base = super_text.as_mut();
        // Get inner Rust object
        let rust_augmenter = match &super_base.inner {
            AugmenterTypes::Text(augmenter) => augmenter,
            _ => panic!("Augmenter is not a TextAugmenter"),
        };
        // Call original augment function
        Ok(rust_augmenter.augment(data, &mut super_base.rng))
    }

    /// Augment the data given a batch
    /// :param data: The list of Strings to augment
    /// :return: The augmented data
    #[pyo3(text_signature = "(self, data: list[str])")]
    fn augment_batch(self_: PyRefMut<'_, Self>, data: Vec<String>) -> PyResult<Vec<String>> {
        // Get base class
        let mut super_text = self_.into_super();
        let super_base = super_text.as_mut();
        // Get inner Rust object
        let rust_augmenter = match &super_base.inner {
            AugmenterTypes::Text(augmenter) => augmenter,
            _ => panic!("Augmenter is not a TextAugmenter"),
        };
        // Call original augment function
        // TODO: make batch augmenter, not just a loop
        let mut augmented_data = Vec::with_capacity(data.len());
        for d in data {
            augmented_data.push(rust_augmenter.augment(d, &mut super_base.rng));
        }
        Ok(augmented_data)
    }
}

/// Text Augmentation Module
#[pymodule]
pub fn text(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyBaseTextAugmenter>()?;
    m.add_class::<PyTextAction>()?;
    m.add_class::<PyCharsRandomAugmenter>()?;
    m.add_class::<PyWordsRandomAugmenter>()?;

    Ok(())
}
