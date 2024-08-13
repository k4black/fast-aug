use pyo3::prelude::*;
use pyo3::{pymodule, PyResult, Python};

/// Models Classes Module
#[pymodule]
pub fn models(_py: Python, _m: &Bound<'_, PyModule>) -> PyResult<()> {
    // m.add_class::<ChanceAugmenter>()?;
    // m.add_class::<SelectorAugmenter>()?;
    // m.add_class::<SequentialAugmenter>()?;

    Ok(())
}
