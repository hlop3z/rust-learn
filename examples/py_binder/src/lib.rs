use pyo3::prelude::*;

/// A simple Rust function to add two numbers
#[pyfunction]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Create a Python module
#[pymodule]
fn rust_python_binding(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    Ok(())
}
