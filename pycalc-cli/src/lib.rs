/*
Calculator functions to later import into a Python Fire CLI.
*/

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn multiply_as_string(a: i64, b: i64) -> PyResult<String> {
    Ok((a * b).to_string())
}

/// Formats adding two numbers as string.
#[pyfunction]
fn add_as_string(a: i64, b: i64) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// Formats subtracting two numbers as string.
#[pyfunction]
fn subtract_as_string(a: i64, b: i64) -> PyResult<String> {
    Ok((a - b).to_string())
}

/// Formats dividing two numbers as string, with a check for division by zero.
#[pyfunction]
fn divide_as_string(a: i64, b: i64) -> PyResult<String> {
    if b == 0 {
        return Err(PyErr::new::<pyo3::exceptions::PyZeroDivisionError, _>("division by zero"));
    }
    Ok((a / b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn libpycalc_cli(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(multiply_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(add_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(subtract_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(divide_as_string, m)?)?;
    Ok(())
}
