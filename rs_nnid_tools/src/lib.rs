use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn mod11_test(numbers: Vec<usize>, factors: Vec<usize>) -> PyResult<bool> {
    let mut mod_11: usize = 0;

    for (numb, fac) in numbers.iter().zip(factors.iter()){
        mod_11 += numb * fac % 11;
    }         // Calculate the product of the pairs


    Ok(mod_11%11==0)
}

/// A Python module implemented in Rust.
#[pymodule]
fn rs_nnid_tools(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(mod11_test, m)?)?;
    Ok(())
}