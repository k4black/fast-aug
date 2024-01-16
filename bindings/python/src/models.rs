use pyo3::{pymodule, PyResult, Python};
use pyo3::types::PyList;
use pyo3::prelude::PyModule;


/// Models Classes Module
#[pymodule]
pub fn models(py: Python, m: &PyModule) -> PyResult<()> {
    // m.add_class::<ChanceAugmenter>()?;
    // m.add_class::<SelectorAugmenter>()?;
    // m.add_class::<SequentialAugmenter>()?;

    // Manually construct the __all__ list
    // let all = PyList::new(py, &["BaseTextAugmenter", "TextAction", "RandomCharsAugmenter", "RandomWordsAugmenter"]);
    // m.add("__all__", all)?;

    Ok(())
}
