use std::collections::HashSet;
use std::sync::Arc;
use rand;
use fast_aug_rust::text::{BaseTextAugmenter, TextAction};
use fast_aug_rust::text::RandomCharsAugmenter;
use fast_aug_rust::text::RandomWordsAugmenter;
use fast_aug_rust::text::TextAugmentParameters;

use pyo3::prelude::*;
use pyo3::wrap_pymodule;
use pyo3::types::PyList;
use pyo3::types::PyAny;
use pyo3::exceptions::{PyNotImplementedError, PyValueError};
use rand::rngs::SmallRng;
use rand::SeedableRng;
use fast_aug_rust::BaseAugmenter;
use crate::base::PyBaseAugmenter;


#[derive(FromPyObject)]
struct PyTextAugmentParameters(f32, usize, usize);

impl Into<TextAugmentParameters> for PyTextAugmentParameters {
    fn into(self) -> TextAugmentParameters {
        TextAugmentParameters {
            p: self.0,
            min_elements: Some(self.1),
            max_elements: Some(self.2),
        }
    }
}


#[pyclass]
#[pyo3(name = "TextAction")]
#[derive(Clone)]
enum PyTextAction {
    Insert,
    Substitute,
    Swap,
    Delete,
}

impl Into<TextAction> for PyTextAction {
    fn into(self) -> TextAction {
        match self {
            PyTextAction::Insert => TextAction::Insert,
            PyTextAction::Substitute => TextAction::Substitute,
            PyTextAction::Swap => TextAction::Swap,
            PyTextAction::Delete => TextAction::Delete,
        }
    }
}


#[pyclass(extends=PyBaseAugmenter, subclass)]
#[pyo3(name = "BaseTextAugmenter")]
pub struct PyBaseTextAugmenter;

#[pymethods]
impl PyBaseTextAugmenter {
    #[new]
    fn new() -> (Self, PyBaseAugmenter) {
        (PyBaseTextAugmenter {}, PyBaseAugmenter {})
    }

    fn augment(&self, data: &PyAny) -> PyResult<PyObject> {
        Err(PyNotImplementedError::new_err("Not implemented"))
    }
}


#[pyclass(extends=PyBaseTextAugmenter)]
#[pyo3(name = "RandomCharsAugmenter")]
pub struct PyRandomCharsAugmenter {
    inner: RandomCharsAugmenter,
    rng: SmallRng,
}

#[pymethods]
impl PyRandomCharsAugmenter {
    #[new]
    fn new(
        action: PyTextAction,
        aug_params_word: PyTextAugmentParameters,
        aug_params_char: PyTextAugmentParameters,
        stopwords: Option<HashSet<String>>,
    ) -> PyClassInitializer<Self> {
        let rng = SmallRng::from_entropy();  // TODO: make from seed

        PyClassInitializer::from(
            PyBaseTextAugmenter::new()
        ).add_subclass(
            PyRandomCharsAugmenter {
                inner: RandomCharsAugmenter::new(
                    action.into(),
                    aug_params_word.into(),
                    aug_params_char.into(),
                    stopwords,
                ),
                rng,
            }
        )

    }

    fn augment(&mut self, data: String) -> PyResult<String> {
        Ok(self.inner.augment(data, &mut self.rng))
    }
}


#[pyclass(extends=PyBaseTextAugmenter)]
#[pyo3(name = "RandomWordsAugmenter")]
pub struct PyRandomWordsAugmenter {
    inner: RandomWordsAugmenter,
    rng: SmallRng,
}

#[pymethods]
impl PyRandomWordsAugmenter {
    #[new]
    fn new(
        action: PyTextAction,
        aug_params_word: PyTextAugmentParameters,
        stopwords: Option<HashSet<String>>,
    ) -> PyClassInitializer<Self> {
        let rng = SmallRng::from_entropy();  // TODO: make from seed

        PyClassInitializer::from(
            PyBaseTextAugmenter::new()
        ).add_subclass(
            PyRandomWordsAugmenter {
                inner: RandomWordsAugmenter::new(
                    action.into(),
                    aug_params_word.into(),
                    stopwords,
                ),
                rng,
            }
        )

    }

    fn augment(&mut self, data: String) -> PyResult<String> {
        Ok(self.inner.augment(data, &mut self.rng))
    }
}


/// Text Augmentation Classes Module
#[pymodule]
pub fn text(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyBaseTextAugmenter>()?;
    m.add_class::<PyTextAction>()?;
    m.add_class::<PyRandomCharsAugmenter>()?;
    m.add_class::<PyRandomWordsAugmenter>()?;

    // Manually construct the __all__ list
    let all = PyList::new(py, &["BaseTextAugmenter", "TextAction", "RandomCharsAugmenter", "RandomWordsAugmenter"]);
    m.add("__all__", all)?;

    Ok(())
}
