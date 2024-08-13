use fast_aug_rust::text::TextAugmentParameters;
use fast_aug_rust::text::{
    CharsRandomDeleteAugmenter, CharsRandomInsertAugmenter, CharsRandomSubstituteAugmenter, CharsRandomSwapAugmenter,
};
use fast_aug_rust::text::{
    WordsRandomDeleteAugmenter, WordsRandomInsertAugmenter, WordsRandomSubstituteAugmenter, WordsRandomSwapAugmenter,
};
use std::collections::HashSet;
use std::panic;
use std::sync::Arc;

use crate::base::{AugmenterTypes, PyBaseAugmenter};
use fast_aug_rust::models::text::AlphabetModel;
use pyo3::exceptions::{PyNotImplementedError, PyValueError};
use pyo3::types::{PyAny, PyList};
use pyo3::prelude::*;
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
    /// :param data: A String to augment
    /// :returns: Augmented data
    #[pyo3(text_signature = "(self, data: str)")]
    pub fn augment(mut self_: PyRefMut<'_, Self>, py: Python, data: &Bound<'_, PyAny>) -> PyResult<PyObject> {
        // Get base class
        let super_base = self_.as_mut();
        // Call base class method
        super_base.augment(py, data)
    }

    /// Augment data given a batch of data
    /// :param data: Vector of strings to augment
    /// :returns: Augmented data
    #[pyo3(text_signature = "(self, data: list[str])")]
    pub fn augment_batch(mut self_: PyRefMut<'_, Self>, py: Python, data: &Bound<'_, PyList>) -> PyResult<PyObject> {
        // Get base class
        let super_base = self_.as_mut();
        // Call base class method
        super_base.augment_batch(py, data)
    }
}

/// Randomly augment chars in the random words
///
/// :param word_params: The parameters for the word augmentation
///     - probability or (probability, min_elements, max_elements)
/// :param char_params: The parameters for the char augmentation
///     - probability or (probability, min_elements, max_elements)
/// :param locale: The locale string to use for alphabet
/// :param stopwords: Optional set of stopwords to ignore
#[pyclass(extends=PyBaseTextAugmenter)]
#[pyo3(name = "CharsRandomInsertAugmenter")]
pub struct PyCharsRandomInsertAugmenter;

#[pymethods]
impl PyCharsRandomInsertAugmenter {
    #[new]
    #[pyo3(
        signature = (word_params, char_params, locale, stopwords=None),
        text_signature = "(self, word_params: float | tuple[float, int | None, int | None], char_params: float | tuple[float, int | None, int | None], locale: str, stopwords: set[str] | None = None)"
    )]
    fn py_new(
        word_params: PyConvertTextAugmentParameters,
        char_params: PyConvertTextAugmentParameters,
        locale: String,
        stopwords: Option<HashSet<String>>,
    ) -> PyResult<PyClassInitializer<Self>> {
        let rng = SmallRng::from_entropy(); // TODO: make from seed

        // Parse locale, if error return PyValueError
        let alphabet_model = panic::catch_unwind(|| AlphabetModel::from_locale_str(&locale))
            .map_err(|_| PyValueError::new_err("Invalid locale"))?;

        // Create Rust object of AugmenterTypes
        let rust_augmenter = AugmenterTypes::Text(Arc::new(CharsRandomInsertAugmenter::new(
            word_params.into(),
            char_params.into(),
            alphabet_model,
            stopwords,
        )));

        // Create Python object with respective parent classes
        Ok(PyClassInitializer::from(PyBaseAugmenter {
            inner: rust_augmenter,
            rng,
        })
        .add_subclass(PyBaseTextAugmenter {})
        .add_subclass(PyCharsRandomInsertAugmenter {}))
    }
}

/// Randomly substitute chars in the random words
///
/// :param word_params: The parameters for the word augmentation
///    - probability or (probability, min_elements, max_elements)
/// :param char_params: The parameters for the char augmentation
///   - probability or (probability, min_elements, max_elements)
/// :param locale: The locale string to use for alphabet
/// :param stopwords: Optional set of stopwords to ignore
#[pyclass(extends=PyBaseTextAugmenter)]
#[pyo3(name = "CharsRandomSubstituteAugmenter")]
pub struct PyCharsRandomSubstituteAugmenter;

#[pymethods]
impl PyCharsRandomSubstituteAugmenter {
    #[new]
    #[pyo3(
        signature = (word_params, char_params, locale, stopwords=None),
        text_signature = "(self, word_params: float | tuple[float, int | None, int | None], char_params: float | tuple[float, int | None, int | None], locale: str, stopwords: set[str] | None = None)"
    )]
    fn py_new(
        word_params: PyConvertTextAugmentParameters,
        char_params: PyConvertTextAugmentParameters,
        locale: String,
        stopwords: Option<HashSet<String>>,
    ) -> PyResult<PyClassInitializer<Self>> {
        let rng = SmallRng::from_entropy(); // TODO: make from seed

        // Parse locale, if error return PyValueError
        let alphabet_model = panic::catch_unwind(|| AlphabetModel::from_locale_str(&locale))
            .map_err(|_| PyValueError::new_err("Invalid locale"))?;

        // Create Rust object of AugmenterTypes
        let rust_augmenter = AugmenterTypes::Text(Arc::new(CharsRandomSubstituteAugmenter::new(
            word_params.into(),
            char_params.into(),
            alphabet_model,
            stopwords,
        )));

        // Create Python object with respective parent classes
        Ok(PyClassInitializer::from(PyBaseAugmenter {
            inner: rust_augmenter,
            rng,
        })
        .add_subclass(PyBaseTextAugmenter {})
        .add_subclass(PyCharsRandomSubstituteAugmenter {}))
    }
}

/// Randomly swap chars in the random words
///
/// :param word_params: The parameters for the word augmentation
///   - probability or (probability, min_elements, max_elements)
/// :param char_params: The parameters for the char augmentation
///  - probability or (probability, min_elements, max_elements)
/// :param stopwords: Optional set of stopwords to ignore
#[pyclass(extends=PyBaseTextAugmenter)]
#[pyo3(name = "CharsRandomSwapAugmenter")]
pub struct PyCharsRandomSwapAugmenter;

#[pymethods]
impl PyCharsRandomSwapAugmenter {
    #[new]
    #[pyo3(
        signature = (word_params, char_params, stopwords=None),
        text_signature = "(self, word_params: float | tuple[float, int | None, int | None], char_params: float | tuple[float, int | None, int | None], stopwords: set[str] | None = None)"
    )]
    fn py_new(
        word_params: PyConvertTextAugmentParameters,
        char_params: PyConvertTextAugmentParameters,
        stopwords: Option<HashSet<String>>,
    ) -> PyResult<PyClassInitializer<Self>> {
        let rng = SmallRng::from_entropy(); // TODO: make from seed

        // Create Rust object of AugmenterTypes
        let rust_augmenter = AugmenterTypes::Text(Arc::new(CharsRandomSwapAugmenter::new(
            word_params.into(),
            char_params.into(),
            stopwords,
        )));

        // Create Python object with respective parent classes
        Ok(PyClassInitializer::from(PyBaseAugmenter {
            inner: rust_augmenter,
            rng,
        })
        .add_subclass(PyBaseTextAugmenter {})
        .add_subclass(PyCharsRandomSwapAugmenter {}))
    }
}

/// Randomly delete chars in the random words
///
/// :param word_params: The parameters for the word augmentation
///  - probability or (probability, min_elements, max_elements)
/// :param char_params: The parameters for the char augmentation
///  - probability or (probability, min_elements, max_elements)
/// :param stopwords: Optional set of stopwords to ignore
#[pyclass(extends=PyBaseTextAugmenter)]
#[pyo3(name = "CharsRandomDeleteAugmenter")]
pub struct PyCharsRandomDeleteAugmenter;

#[pymethods]
impl PyCharsRandomDeleteAugmenter {
    #[new]
    #[pyo3(
        signature = (word_params, char_params, stopwords=None),
        text_signature = "(self, word_params: float | tuple[float, int | None, int | None], char_params: float | tuple[float, int | None, int | None], stopwords: set[str] | None = None)"
    )]
    fn py_new(
        word_params: PyConvertTextAugmentParameters,
        char_params: PyConvertTextAugmentParameters,
        stopwords: Option<HashSet<String>>,
    ) -> PyResult<PyClassInitializer<Self>> {
        let rng = SmallRng::from_entropy(); // TODO: make from seed

        // Create Rust object of AugmenterTypes
        let rust_augmenter = AugmenterTypes::Text(Arc::new(CharsRandomDeleteAugmenter::new(
            word_params.into(),
            char_params.into(),
            stopwords,
        )));

        // Create Python object with respective parent classes
        Ok(PyClassInitializer::from(PyBaseAugmenter {
            inner: rust_augmenter,
            rng,
        })
        .add_subclass(PyBaseTextAugmenter {})
        .add_subclass(PyCharsRandomDeleteAugmenter {}))
    }
}

/// Randomly insert words from the given vocabulary
///
/// :param word_params: The parameters for the word augmentation
///     - probability or (probability, min_elements, max_elements)
/// :param vocabulary: The vocabulary to use for insertion
/// :param stopwords: Optional set of stopwords to ignore
#[pyclass(extends=PyBaseTextAugmenter)]
#[pyo3(name = "WordsRandomInsertAugmenter")]
pub struct PyWordsRandomInsertAugmenter;

#[pymethods]
impl PyWordsRandomInsertAugmenter {
    #[new]
    #[pyo3(
        signature = (word_params, vocabulary, stopwords=None),
        text_signature = "(self, word_params: float | tuple[float, int | None, int | None], vocabulary: list[str], stopwords: set[str] | None = None)"
    )]
    fn py_new(
        word_params: PyConvertTextAugmentParameters,
        vocabulary: Vec<String>,
        stopwords: Option<HashSet<String>>,
    ) -> PyResult<PyClassInitializer<Self>> {
        let rng = SmallRng::from_entropy(); // TODO: make from seed

        // Check if vocabulary is empty
        if vocabulary.is_empty() {
            return Err(PyValueError::new_err("Vocabulary cannot be empty"));
        }

        // Create Rust object of AugmenterTypes
        let rust_augmenter = AugmenterTypes::Text(Arc::new(WordsRandomInsertAugmenter::new(
            word_params.into(),
            vocabulary,
            stopwords,
        )));

        // Create Python object with respective parent classes
        Ok(PyClassInitializer::from(PyBaseAugmenter {
            inner: rust_augmenter,
            rng,
        })
        .add_subclass(PyBaseTextAugmenter {})
        .add_subclass(PyWordsRandomInsertAugmenter {}))
    }
}

/// Randomly substitute words from the given vocabulary
///
/// :param word_params: The parameters for the word augmentation
///    - probability or (probability, min_elements, max_elements)
/// :param vocabulary: The vocabulary to use for substitution
/// :param stopwords: Optional set of stopwords to ignore
#[pyclass(extends=PyBaseTextAugmenter)]
#[pyo3(name = "WordsRandomSubstituteAugmenter")]
pub struct PyWordsRandomSubstituteAugmenter;

#[pymethods]
impl PyWordsRandomSubstituteAugmenter {
    #[new]
    #[pyo3(
        signature = (word_params, vocabulary, stopwords=None),
        text_signature = "(self, word_params: float | tuple[float, int | None, int | None], vocabulary: list[str], stopwords: set[str] | None = None)"
    )]
    fn py_new(
        word_params: PyConvertTextAugmentParameters,
        vocabulary: Vec<String>,
        stopwords: Option<HashSet<String>>,
    ) -> PyResult<PyClassInitializer<Self>> {
        let rng = SmallRng::from_entropy(); // TODO: make from seed

        // Check if vocabulary is empty
        if vocabulary.is_empty() {
            return Err(PyValueError::new_err("Vocabulary cannot be empty"));
        }

        // Create Rust object of AugmenterTypes
        let rust_augmenter = AugmenterTypes::Text(Arc::new(WordsRandomSubstituteAugmenter::new(
            word_params.into(),
            vocabulary,
            stopwords,
        )));

        // Create Python object with respective parent classes
        Ok(PyClassInitializer::from(PyBaseAugmenter {
            inner: rust_augmenter,
            rng,
        })
        .add_subclass(PyBaseTextAugmenter {})
        .add_subclass(PyWordsRandomSubstituteAugmenter {}))
    }
}

/// Randomly swap words
///
/// :param word_params: The parameters for the word augmentation
///    - probability or (probability, min_elements, max_elements)
/// :param stopwords: Optional set of stopwords to ignore
#[pyclass(extends=PyBaseTextAugmenter)]
#[pyo3(name = "WordsRandomSwapAugmenter")]
pub struct PyWordsRandomSwapAugmenter;

#[pymethods]
impl PyWordsRandomSwapAugmenter {
    #[new]
    #[pyo3(
        signature = (word_params, stopwords=None),
        text_signature = "(self, word_params: float | tuple[float, int | None, int | None], stopwords: set[str] | None = None)"
    )]
    fn py_new(
        word_params: PyConvertTextAugmentParameters,
        stopwords: Option<HashSet<String>>,
    ) -> PyResult<PyClassInitializer<Self>> {
        let rng = SmallRng::from_entropy(); // TODO: make from seed

        // Create Rust object of AugmenterTypes
        let rust_augmenter =
            AugmenterTypes::Text(Arc::new(WordsRandomSwapAugmenter::new(word_params.into(), stopwords)));

        // Create Python object with respective parent classes
        Ok(PyClassInitializer::from(PyBaseAugmenter {
            inner: rust_augmenter,
            rng,
        })
        .add_subclass(PyBaseTextAugmenter {})
        .add_subclass(PyWordsRandomSwapAugmenter {}))
    }
}

/// Randomly delete words
///
/// :param word_params: The parameters for the word augmentation
///   - probability or (probability, min_elements, max_elements)
/// :param stopwords: Optional set of stopwords to ignore
#[pyclass(extends=PyBaseTextAugmenter)]
#[pyo3(name = "WordsRandomDeleteAugmenter")]
pub struct PyWordsRandomDeleteAugmenter;

#[pymethods]
impl PyWordsRandomDeleteAugmenter {
    #[new]
    #[pyo3(
        signature = (word_params, stopwords=None),
        text_signature = "(self, word_params: float | tuple[float, int | None, int | None], stopwords: set[str] | None = None)"
    )]
    fn py_new(
        word_params: PyConvertTextAugmentParameters,
        stopwords: Option<HashSet<String>>,
    ) -> PyResult<PyClassInitializer<Self>> {
        let rng = SmallRng::from_entropy(); // TODO: make from seed

        // Create Rust object of AugmenterTypes
        let rust_augmenter =
            AugmenterTypes::Text(Arc::new(WordsRandomDeleteAugmenter::new(word_params.into(), stopwords)));

        // Create Python object with respective parent classes
        Ok(PyClassInitializer::from(PyBaseAugmenter {
            inner: rust_augmenter,
            rng,
        })
        .add_subclass(PyBaseTextAugmenter {})
        .add_subclass(PyWordsRandomDeleteAugmenter {}))
    }
}

/// Text Augmentation Module
#[pymodule]
pub fn text(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyBaseTextAugmenter>()?;
    m.add_class::<PyCharsRandomInsertAugmenter>()?;
    m.add_class::<PyCharsRandomSubstituteAugmenter>()?;
    m.add_class::<PyCharsRandomSwapAugmenter>()?;
    m.add_class::<PyCharsRandomDeleteAugmenter>()?;
    m.add_class::<PyWordsRandomInsertAugmenter>()?;
    m.add_class::<PyWordsRandomSubstituteAugmenter>()?;
    m.add_class::<PyWordsRandomSwapAugmenter>()?;
    m.add_class::<PyWordsRandomDeleteAugmenter>()?;

    Ok(())
}
