#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
#[pyfunction]
fn detect_face_py(image: String) -> PyResult<bool> {
    match super::super::vision::detect_face(&image) {
        Ok(result) => Ok(result),
        Err(_) => Ok(false),
    }
}

#[cfg(feature = "python")]
#[pyfunction]
fn recommend_product_py(user_id: String) -> PyResult<Vec<String>> {
    Ok(super::super::ai::recommend_product(&user_id))
}

#[cfg(feature = "python")]
#[pymodule]
fn ar_shopping(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(detect_face_py, m)?)?;
    m.add_function(wrap_pyfunction!(recommend_product_py, m)?)?;
    Ok(())
}
