use pyo3::Python;
use pyo3::prelude::*;
use crate as wix;

// get variable from python
pub fn get_variable<T>(code: String, file: String, name: String, variable: String) -> Result<T, String> 
where T: core::str::FromStr {
    Python::with_gil(|py| -> Result<T, String> {

        let py_mod = match PyModule::from_code(py, &code, &file, &name) {
            Ok(m) => m,
            Err(e) => {
                return Result::Err("error".to_string());
            }
        };

        let py_var = match py_mod.getattr(&variable) {
            Ok(v) => v,
            Err(e) => {
                return Result::Err("variable not found".to_string());
            }
        };

        match py_var.to_string().parse::<T>() {
            Ok(v) => Result::Ok(v),
            Err(e) => {
                return Result::Err("type error".to_string());
            }
        }
    })
}