use std::sync::Arc;
use fast_aug_rust::base::BaseAugmenter as BaseAugmenterRust;

use pyo3::prelude::*;
use pyo3::wrap_pymodule;
use pyo3::types::{PyAny, PyType};
use pyo3::exceptions::{PyNotImplementedError, PyTypeError, PyValueError};
use rand::rngs::SmallRng;
use fast_aug_rust::BaseAugmenter;
use fast_aug_rust::text::Doc;


// struct PythonAugmenter {
//     python_augmenter: PyAny,
// }
//
// impl PythonAugmenter {
//     fn new(py: Python, python_augmenter: PyAny) -> Self {
//         // Validate that the python_augmenter is a subclass of python BaseAugmenter (PyBaseAugmenter)
//         if !python_augmenter.is_instance(PyType::new::<PyBaseAugmenter>(py)) {
//             panic!("python_augmenter must be a subclass of BaseAugmenter");
//         }
//
//         // Create a new instance of the python_augmenter
//         PythonAugmenter {
//             python_augmenter,
//         }
//     }
// }
//
//
// impl BaseAugmenterRust<T, K> for PythonAugmenter {
//     fn augment_inner(&self, input: K, rng: &mut dyn rand::RngCore) -> K {
//         self.python_augmenter.call_method1("augment", (input, rng)).unwrap()
//     }
//
//     fn convert_to_inner(&self, input: T) -> K {
//         self.python_augmenter.call_method1("convert_to_inner", input).unwrap()
//     }
//
//     fn convert_to_outer(&self, input: K) -> T {
//         self.python_augmenter.call_method1("convert_to_outer", input).unwrap()
//     }
// }


#[derive(Clone)]
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
    #[pyo3(text_signature = "(self)" )]
    pub fn py_new() -> PyResult<Self> {
        Err(PyNotImplementedError::new_err("Not implemented"))
    }

    /// Augment data
    /// :param data: Data to augment
    /// :returns: Augmented data
    #[pyo3(text_signature = "(self, data: Any) -> Any")]
    pub fn augment(&self, data: &PyAny) -> PyResult<PyObject> {
        Err(PyNotImplementedError::new_err("Not implemented"))
    }
}


/// Base Classes Module
#[pymodule]
pub fn base(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyBaseAugmenter>()?;
    Ok(())
}
