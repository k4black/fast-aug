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
    pub fn augment(&mut self, py: Python, data: &PyAny) -> PyResult<PyObject> {
        // Match inner Rust object and extract respective data type
        match &self.inner {
            // String input
            AugmenterTypes::Text(augmenter) => {
                let data = data.extract::<String>().unwrap();
                let augmented_data = augmenter.augment(data, &mut self.rng);
                Ok(augmented_data.into_py(py))
            }
            // Not implemented for other types
            _ => Err(PyNotImplementedError::new_err("Not implemented")),
        }
    }

    /// Augment data given a batch of data
    /// :param data: Data to augment - vector of data points
    /// :returns: Augmented data
    #[pyo3(text_signature = "(self, data: list[Any])")]
    pub fn augment_batch(&mut self, py: Python, data: Vec<&PyAny>) -> PyResult<PyObject> {
        // Match inner Rust object and extract respective data type
        match &self.inner {
            // String input
            AugmenterTypes::Text(augmenter) => {
                let data = data
                    .iter()
                    .map(|x| x.extract::<String>().unwrap())
                    .collect::<Vec<String>>();
                let augmented_data = augmenter.augment_batch(data, &mut self.rng);
                Ok(augmented_data.into_py(py))
            }
            // Not implemented for other types
            _ => Err(PyNotImplementedError::new_err("Not implemented")),
        }
    }
}

/// Base Classes Module
#[pymodule]
pub fn base(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyBaseAugmenter>()?;
    Ok(())
}
