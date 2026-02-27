use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::{HashMap, HashSet};

/// Consolidates column alias translation and field filtering into a single pass.
#[pyfunction]
pub fn prepare_model_to_save(
    py: Python<'_>,
    new_kwargs: &Bound<'_, PyDict>,
    aliases_map: HashMap<String, String>,
    fields_to_keep: HashSet<String>,
) -> PyResult<PyObject> {
    let result = PyDict::new_bound(py);

    for (key, value) in new_kwargs.iter() {
        let key_str: String = key.extract()?;

        if !fields_to_keep.contains(&key_str) {
            continue;
        }

        let alias = aliases_map.get(&key_str).unwrap_or(&key_str);
        result.set_item(alias.as_str(), value)?;
    }

    Ok(result.into())
}
