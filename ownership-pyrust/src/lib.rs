use pyo3::prelude::*;
use pyo3::pyclass;

#[pyclass]
struct NumberList {
    numbers: Vec<i32>,
}

#[pymethods]
impl NumberList {
    #[new]
    fn new_obj() -> Self {
        NumberList { numbers: Vec::new() }
    }

    fn add(&mut self, value: i32) {
        self.numbers.push(value);
    }

    fn length(&self) -> usize {
        self.numbers.len()
    }

    fn clear_list(&mut self) {
        self.numbers.clear();
    }
}

#[pymodule]
fn libownership_pyrust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<NumberList>()?;
    Ok(())
}
