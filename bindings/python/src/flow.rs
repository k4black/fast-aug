use fast_aug_rust::flow::{ChanceAugmenter, SelectorAugmenter, SequentialAugmenter};
use std::sync::Arc;

use crate::base::{AugmenterTypes, PyBaseAugmenter};

use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;

use pyo3::types::PyList;
use rand::rngs::SmallRng;
use rand::SeedableRng;

/// Given other augmenter apply it with a given probability
/// :param augmenter: The augmenter to apply with a given probability
/// :param probability: The probability of applying the augmenter
#[pyclass(extends=PyBaseAugmenter)]
#[pyo3(name = "ChanceAugmenter")]
pub struct PyChanceAugmenter;

#[pymethods]
impl PyChanceAugmenter {
    #[new]
    #[pyo3(text_signature = "(self, augmenter: BaseAugmenter, probability: float)")]
    fn py_new(augmenter: &PyBaseAugmenter, probability: f64) -> PyResult<PyClassInitializer<Self>> {
        let rng = SmallRng::from_entropy();

        // Process parameters
        if !(0.0..=1.0).contains(&probability) {
            return Err(PyValueError::new_err("probability must be between 0 and 1"));
        }

        // Create Rust object of AugmenterTypes
        // TODO: other types than String
        let rust_augmenter = match &augmenter.inner {
            AugmenterTypes::Text(augmenter) => {
                AugmenterTypes::Text(Arc::new(ChanceAugmenter::new(augmenter.clone(), probability)))
            }
            _ => return Err(PyTypeError::new_err("augmenter must be a text augmenter")),
        };

        // Create Python object with respective parent classes
        Ok(PyClassInitializer::from(PyBaseAugmenter {
            inner: rust_augmenter,
            rng,
        })
        .add_subclass(PyChanceAugmenter {}))
    }

    /// Augment the data
    /// :param data: The String data to augment
    /// :return: The augmented data
    #[pyo3(text_signature = "(self, data: str)")]
    fn augment(mut self_: PyRefMut<'_, Self>, data: String) -> PyResult<String> {
        // Get base class
        let super_base = self_.as_mut();
        // Get inner Rust object
        let rust_augmenter = match &super_base.inner {
            AugmenterTypes::Text(augmenter) => augmenter,
            _ => std::panic!("Augmenter is not a TextAugmenter"),
        };
        // Call original augment function
        Ok(rust_augmenter.augment(data, &mut super_base.rng))
    }

    /// Augment the data given a batch
    /// :param data: The String data to augment
    /// :return: The augmented data
    #[pyo3(text_signature = "(self, data: list[str])")]
    fn augment_batch(mut self_: PyRefMut<'_, Self>, data: Vec<String>) -> PyResult<Vec<String>> {
        // Get base class
        let super_base = self_.as_mut();
        // Get inner Rust object
        let rust_augmenter = match &super_base.inner {
            AugmenterTypes::Text(augmenter) => augmenter,
            _ => std::panic!("Augmenter is not a TextAugmenter"),
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

/// Given a list of augmenters, apply one of them randomly
/// :param augmenters: The list of augmenters to choose from
/// :param weights: Optional weights for each augmenter
#[pyclass(extends=PyBaseAugmenter)]
#[pyo3(name = "SelectorAugmenter")]
pub struct PySelectorAugmenter;

#[pymethods]
impl PySelectorAugmenter {
    #[new]
    #[pyo3(text_signature = "(self, augmenters: list[BaseAugmenter], weights: list[float] | None = None)")]
    fn py_new(augmenters: &PyList, weights: Option<Vec<f32>>) -> PyResult<PyClassInitializer<Self>> {
        let rng = SmallRng::from_entropy();

        // Process parameters
        if augmenters.len() <= 1 {
            return Err(PyValueError::new_err("augmenters must have at least 2 augmenters"));
        }
        if let Some(weights) = &weights {
            if weights.len() != augmenters.len() {
                return Err(PyValueError::new_err("weights must be the same length as augmenters"));
            }
        }
        let augmenters = augmenters
            .iter()
            .map(|x| x.extract::<PyBaseAugmenter>())
            .collect::<Result<Vec<_>, _>>();
        let augmenters = match augmenters {
            Ok(augmenters) => augmenters,
            Err(_) => return Err(PyTypeError::new_err("augmenters must be a list of BaseAugmenter")),
        };

        // Create Rust object of AugmenterTypes
        // TODO: other types than String
        let rust_augmenter = match augmenters[0].inner {
            AugmenterTypes::Text(_) => {
                let augmenters = augmenters
                    .into_iter()
                    .map(|x| match x.inner {
                        AugmenterTypes::Text(augmenter) => augmenter,
                        _ => panic!("Augmenter is not a TextAugmenter"),
                    })
                    .collect::<Vec<_>>();
                AugmenterTypes::Text(Arc::new(SelectorAugmenter::new(augmenters, weights)))
            }
            _ => return Err(PyTypeError::new_err("augmenters must be a list of text augmenters")),
        };

        // Create Python object with respective parent classes
        Ok(PyClassInitializer::from(PyBaseAugmenter {
            inner: rust_augmenter,
            rng,
        })
        .add_subclass(PySelectorAugmenter {}))
    }

    /// Augment the data
    /// :param data: The String data to augment
    /// :return: The augmented data
    #[pyo3(text_signature = "(self, data: str)")]
    fn augment(mut self_: PyRefMut<'_, Self>, data: String) -> PyResult<String> {
        // Get base class
        let super_base = self_.as_mut();
        // Get inner Rust object
        let rust_augmenter = match &super_base.inner {
            AugmenterTypes::Text(augmenter) => augmenter,
            _ => std::panic!("Augmenter is not a TextAugmenter"),
        };
        // Call original augment function
        Ok(rust_augmenter.augment(data, &mut super_base.rng))
    }

    /// Augment the data given a batch
    /// :param data: The String data to augment
    /// :return: The augmented data
    #[pyo3(text_signature = "(self, data: list[str])")]
    fn augment_batch(mut self_: PyRefMut<'_, Self>, data: Vec<String>) -> PyResult<Vec<String>> {
        // Get base class
        let super_base = self_.as_mut();
        // Get inner Rust object
        let rust_augmenter = match &super_base.inner {
            AugmenterTypes::Text(augmenter) => augmenter,
            _ => std::panic!("Augmenter is not a TextAugmenter"),
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

/// Given a list of augmenters, apply them sequentially
/// :param augmenters: The list of augmenters to apply sequentially
#[pyclass(extends=PyBaseAugmenter)]
#[pyo3(name = "SequentialAugmenter")]
pub struct PySequentialAugmenter;

#[pymethods]
impl PySequentialAugmenter {
    #[new]
    #[pyo3(text_signature = "(self, augmenters: list[BaseAugmenter])")]
    fn py_new(augmenters: &PyList) -> PyResult<PyClassInitializer<Self>> {
        let rng = SmallRng::from_entropy();

        // Process parameters
        if augmenters.len() <= 1 {
            return Err(PyValueError::new_err("augmenters must have at least 2 augmenters"));
        }
        let augmenters = augmenters
            .iter()
            .map(|x| x.extract::<PyBaseAugmenter>())
            .collect::<Result<Vec<_>, _>>();
        let augmenters = match augmenters {
            Ok(augmenters) => augmenters,
            Err(_) => return Err(PyTypeError::new_err("augmenters must be a list of BaseAugmenter")),
        };

        // Create Rust object of AugmenterTypes
        let rust_augmenter = match augmenters[0].inner {
            AugmenterTypes::Text(_) => {
                let augmenters = augmenters
                    .into_iter()
                    .map(|x| match x.inner {
                        AugmenterTypes::Text(augmenter) => augmenter,
                        _ => panic!("Augmenter is not a TextAugmenter"),
                    })
                    .collect::<Vec<_>>();
                AugmenterTypes::Text(Arc::new(SequentialAugmenter::new(augmenters)))
            }
            _ => return Err(PyTypeError::new_err("augmenters must be a list of text augmenters")),
        };

        // Create Python object with respective parent classes
        Ok(PyClassInitializer::from(PyBaseAugmenter {
            inner: rust_augmenter,
            rng,
        })
        .add_subclass(PySequentialAugmenter {}))
    }

    /// Augment the data
    /// :param data: The String data to augment
    /// :return: The augmented data
    #[pyo3(text_signature = "(self, data: str)")]
    fn augment(mut self_: PyRefMut<'_, Self>, data: String) -> PyResult<String> {
        // Get base class
        let super_base = self_.as_mut();
        // Get inner Rust object
        let rust_augmenter = match &super_base.inner {
            AugmenterTypes::Text(augmenter) => augmenter,
            _ => std::panic!("Augmenter is not a TextAugmenter"),
        };
        // Call original augment function
        Ok(rust_augmenter.augment(data, &mut super_base.rng))
    }

    /// Augment the data given a batch
    /// :param data: The String data to augment
    /// :return: The augmented data
    #[pyo3(text_signature = "(self, data: list[str])")]
    fn augment_batch(mut self_: PyRefMut<'_, Self>, data: Vec<String>) -> PyResult<Vec<String>> {
        // Get base class
        let super_base = self_.as_mut();
        // Get inner Rust object
        let rust_augmenter = match &super_base.inner {
            AugmenterTypes::Text(augmenter) => augmenter,
            _ => std::panic!("Augmenter is not a TextAugmenter"),
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

/// Flow Module - Pipelines, Random Selection, etc.
#[pymodule]
pub fn flow(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyChanceAugmenter>()?;
    m.add_class::<PySelectorAugmenter>()?;
    m.add_class::<PySequentialAugmenter>()?;

    Ok(())
}
