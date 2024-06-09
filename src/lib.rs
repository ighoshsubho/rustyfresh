use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::collections::HashMap;

#[pyfunction]
pub fn mean(data: Vec<f64>) -> f64 {
    let sum: f64 = data.iter().sum();
    sum / data.len() as f64
}

#[pyfunction]
pub fn variance(data: Vec<f64>) -> f64 {
    let mean_val = mean(data.clone());
    let var_sum: f64 = data.iter().map(|x| (x - mean_val).powi(2)).sum();
    var_sum / (data.len() as f64 - 1.0)
}


#[pyfunction]
pub fn skewness(data: Vec<f64>) -> f64 {
    let mean_val = mean(data.clone());
    let variance_val = variance(data.clone());
    let n = data.len() as f64;
    let skew_sum: f64 = data.iter().map(|x| (x - mean_val).powi(3)).sum();
    skew_sum / (n * variance_val.sqrt().powi(3))
}


#[pyfunction]
pub fn extract_features(data: Vec<f64>) -> HashMap<String, f64> {
    let mut features = HashMap::new();
    features.insert("mean".to_string(), mean(data.clone()));
    features.insert("variance".to_string(), variance(data.clone()));
    features.insert("skewness".to_string(), skewness(data.clone()));
    features
}

#[pymodule]
fn rustyfresh(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(mean, m)?)?;
    m.add_function(wrap_pyfunction!(variance, m)?)?;
    m.add_function(wrap_pyfunction!(skewness, m)?)?;
    m.add_function(wrap_pyfunction!(extract_features, m)?)?;
    Ok(())
}