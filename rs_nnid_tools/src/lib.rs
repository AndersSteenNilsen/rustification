use pyo3::prelude::*;
use log::info;
use pyo3_log;

fn _mod11_test(numbers: &[usize], factors: &Vec<usize>) -> bool {
    let mut mod_11: usize = 0;

    for (numb, fac) in numbers.iter().zip(factors.iter()){
        mod_11 += (numb * fac) % 11;
    }         // Calculate the product of the pairs
    return mod_11%11==0;
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn mod11_test(numbers: Vec<usize>, factors: Vec<usize>) -> PyResult<bool> {
    let mut mod_11: usize = 0;

    for (numb, fac) in numbers.iter().zip(factors.iter()){
        mod_11 += (numb * fac) % 11;
    }         // Calculate the product of the pairs


    Ok(mod_11%11==0)
}

#[pyfunction]
fn log_hello() -> PyResult<bool> {
    info!("Hello Python, this is Rust");
    Ok(true)
} 


#[pyfunction]
fn nnid_tests(nnids: Vec<String>) -> PyResult<bool> {
    let controll_factors: Vec<Vec<usize>> =  vec![vec![3, 7, 6, 1, 8, 9, 4, 5, 2, 1], vec![5, 4, 3, 2, 7, 6, 5, 4, 3, 2, 1]];

    for nnid in nnids{
        //info!("NNID tests started");
        //info!("{}", nnid);
        let redix : u32 = 10;
        let nnid_a: Vec<usize> = nnid.chars().flat_map(|c| c.to_digit(redix).map(|d| d as usize)).collect();
        let test_1 = _mod11_test(&nnid_a[..10], &controll_factors[0]);
        if !test_1{
            return Ok(false)
        }
    }

    Ok(true)
}

/// A Python module implemented in Rust.
#[pymodule]
fn rs_nnid_tools(_py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();
    m.add_function(wrap_pyfunction!(mod11_test, m)?)?;
    m.add_function(wrap_pyfunction!(nnid_tests, m)?)?;
    m.add_function(wrap_pyfunction!(log_hello, m)?)?;
    Ok(())
}