use std::collections::HashSet;
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyTuple};


/// Converts a Python iterable into a Rust HashSet<String>
fn iterable_to_hashset(iterable: &PyAny) -> PyResult<HashSet<String>> {
    let mut set = HashSet::new();
    for item in iterable.iter()? {
        let item_str = item?.extract::<String>()?;
        set.insert(item_str);
    }
    Ok(set)
}
