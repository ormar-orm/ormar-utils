use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::HashMap;

/// Build a reverse mapping from alias -> field_name given a forward mapping
/// of field_name -> alias. If a field has no alias (alias == field_name),
/// it still gets an identity entry.
///
/// This is called once per model class and cached.
#[pyfunction]
pub fn build_reverse_alias_map<'py>(
    py: Python<'py>,
    field_alias_map: &Bound<'py, PyDict>,
) -> PyResult<Bound<'py, PyDict>> {
    let result = PyDict::new(py);
    for (key, value) in field_alias_map.iter() {
        let field_name: String = key.extract()?;
        let alias: String = value.extract()?;
        // Map alias -> field_name
        result.set_item(&alias, &field_name)?;
        // Also keep field_name -> field_name for identity lookups
        if alias != field_name {
            result.set_item(&field_name, &field_name)?;
        }
    }
    Ok(result)
}

/// Translate dict keys from field names to their database aliases.
/// Takes the dict to translate and a field_name->alias mapping.
/// Returns a new dict with aliased keys.
#[pyfunction]
pub fn translate_columns_to_aliases<'py>(
    py: Python<'py>,
    new_kwargs: &Bound<'py, PyDict>,
    field_to_alias: &Bound<'py, PyDict>,
) -> PyResult<Bound<'py, PyDict>> {
    let result = PyDict::new(py);
    // Pre-extract the mapping into a Rust HashMap for fast lookup
    let mut alias_map: HashMap<String, String> = HashMap::new();
    for (k, v) in field_to_alias.iter() {
        let key: String = k.extract()?;
        let val: String = v.extract()?;
        alias_map.insert(key, val);
    }

    for (key, value) in new_kwargs.iter() {
        let field_name: String = key.extract()?;
        if let Some(alias) = alias_map.get(&field_name) {
            result.set_item(alias, value)?;
        } else {
            result.set_item(key, value)?;
        }
    }
    Ok(result)
}

/// Translate dict keys from database aliases to field names.
/// Takes the dict to translate and an alias->field_name mapping.
/// Returns a new dict with field name keys.
#[pyfunction]
pub fn translate_aliases_to_columns<'py>(
    py: Python<'py>,
    new_kwargs: &Bound<'py, PyDict>,
    alias_to_field: &Bound<'py, PyDict>,
) -> PyResult<Bound<'py, PyDict>> {
    let result = PyDict::new(py);
    for (key, value) in new_kwargs.iter() {
        if let Some(field_name) = alias_to_field.get_item(&key)? {
            result.set_item(field_name, value)?;
        } else {
            result.set_item(key, value)?;
        }
    }
    Ok(result)
}
