use pyo3::prelude::*;

#[pyfunction]
fn add(a: usize, b: usize) -> PyResult<usize> {
    Ok(a + b)
}