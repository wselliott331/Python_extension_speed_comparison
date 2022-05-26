
use pyo3::prelude::*;

/// Calculate's nth fibonacci number. 
#[pyfunction]
///#[pyo3(name = "fibonacci")]
fn fibonacci(n: i64) -> i64 {
    if n <= 2{
        return 1;
    } else {
        return fibonacci(n-1) + fibonacci(n-2);
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn rustinacci(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fibonacci, m)?)?;
    Ok(())
}
