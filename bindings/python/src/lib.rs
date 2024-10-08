extern crate fast_aug as fast_aug_rust;

use pyo3::prelude::*;

use pyo3::wrap_pymodule;

mod base;
mod flow;
mod models;
mod text;

/// Main fast_aug Module
#[pymodule]
fn fast_aug(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(base::base))?;
    m.add_wrapped(wrap_pymodule!(flow::flow))?;
    m.add_wrapped(wrap_pymodule!(models::models))?;
    m.add_wrapped(wrap_pymodule!(text::text))?;

    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    Ok(())
}
