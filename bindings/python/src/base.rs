use fast_aug_rust::base::BaseAugmenter as BaseAugmenterRust;

use pyo3::prelude::*;
use pyo3::wrap_pymodule;
use pyo3::types::PyAny;
use pyo3::exceptions::PyNotImplementedError;


#[pyclass(subclass)]
#[pyo3(name = "BaseAugmenter")]
pub struct PyBaseAugmenter;

#[pymethods]
impl PyBaseAugmenter {
    #[new]
    fn new() -> Self {
        PyBaseAugmenter {}
    }

    fn augment(&self, data: &PyAny) -> PyResult<PyObject> {
        Err(PyNotImplementedError::new_err("Not implemented"))
    }
}


/// Base Classes Module
#[pymodule]
pub fn base(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyBaseAugmenter>()?;
    Ok(())
}
