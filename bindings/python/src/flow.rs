use std::sync::Arc;
use fast_aug_rust::flow::ChanceAugmenter as ChanceAugmenterRust;
use fast_aug_rust::flow::SelectorAugmenter as SelectorAugmenterRust;
use fast_aug_rust::flow::SequentialAugmenter as SequentialAugmenterRust;

use pyo3::prelude::*;
use pyo3::types::PyList;
use pyo3::wrap_pymodule;
use pyo3::types::PyAny;
use pyo3::exceptions::PyNotImplementedError;


// #[pyclass]
// pub struct ChanceAugmenter {
//     inner: ChanceAugmenterRust<T, K>,
// }
//
// #[pymethods]
// impl ChanceAugmenter {
//     #[new]
//     fn new() -> Self {
//         ChanceAugmenter {
//             inner: ChanceAugmenterRust::new(Arc::new(()), 0.0),
//         }
//     }
// }


/// Flow Classes Module
#[pymodule]
pub fn flow(py: Python, m: &PyModule) -> PyResult<()> {
    // m.add_class::<ChanceAugmenter>()?;
    // m.add_class::<SelectorAugmenter>()?;
    // m.add_class::<SequentialAugmenter>()?;

    // Manually construct the __all__ list
    let all = PyList::new(py, &["ChanceAugmenter", "SelectorAugmenter", "SequentialAugmenter"]);
    m.add("__all__", all)?;

    Ok(())
}
