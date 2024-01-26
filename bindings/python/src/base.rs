use std::sync::Arc;

use fast_aug_rust::text::Doc;
use fast_aug_rust::BaseAugmenter;
use pyo3::exceptions::PyNotImplementedError;
use pyo3::prelude::*;
use pyo3::types::PyAny;

use rand::rngs::SmallRng;

#[derive(Clone)]
#[allow(dead_code)]
pub(crate) enum AugmenterTypes {
    Int(Arc<dyn BaseAugmenter<i32, i32> + Send + Sync>),
    Text(Arc<dyn BaseAugmenter<String, Doc> + Send + Sync>),
}

/// Abstract Base Class for Augmentation
#[pyclass(subclass)]
#[pyo3(name = "BaseAugmenter")]
#[derive(Clone)]
pub struct PyBaseAugmenter {
    pub(crate) inner: AugmenterTypes,
    pub(crate) rng: SmallRng,
}

#[pymethods]
impl PyBaseAugmenter {
    #[new]
    #[pyo3(text_signature = "(self)")]
    fn py_new() -> PyResult<Self> {
        Err(PyNotImplementedError::new_err("Not implemented"))
    }

    /// Augment data
    /// :param data: Data to augment - single data point
    /// :returns: Augmented data
    #[pyo3(text_signature = "(self, data: Any)")]
    fn augment(&self, _data: &PyAny) -> PyResult<PyObject> {
        Err(PyNotImplementedError::new_err("Not implemented"))
    }

    /// Augment data given a batch of data
    /// :param data: Data to augment - vector of data points
    /// :returns: Augmented data
    #[pyo3(text_signature = "(self, data: list[Any])")]
    fn augment_batch(&self, _data: Vec<&PyAny>) -> PyResult<PyObject> {
        Err(PyNotImplementedError::new_err("Not implemented"))
    }
}

/// Base Classes Module
#[pymodule]
pub fn base(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyBaseAugmenter>()?;
    Ok(())
}
