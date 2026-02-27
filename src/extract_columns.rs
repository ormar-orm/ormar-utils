use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::HashSet;

/// Extract prefixed columns from a database row into a model dict.
#[pyfunction]
pub fn extract_prefixed_columns(
    _py: Python<'_>,
    column_mappings: Vec<(String, String)>,
    selected_columns: HashSet<String>,
    row: &Bound<'_, PyAny>,
    column_prefix: String,
    item: &Bound<'_, PyDict>,
) -> PyResult<PyObject> {
    for (col_name, alias) in &column_mappings {
        if item.contains(alias.as_str())? {
            continue;
        }
        if !selected_columns.contains(alias.as_str()) {
            continue;
        }
        let prefixed = format!("{}{}", column_prefix, col_name);
        let value = row.get_item(&prefixed)?;
        item.set_item(alias.as_str(), value)?;
    }
    Ok(item.as_any().clone().unbind())
}
