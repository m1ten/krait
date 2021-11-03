use pyo3::Python;
use pyo3::prelude::*;
use crate as wix;

// get variable from python
pub fn get_variable<T>(code: String, file: String, name: String, variable: String) -> Result<T, ()> 
where
    T: for<'p> FromPyObject<'p>
{
    Python::with_gil(|py| -> Result<T, ()> {

        let py_mod = match PyModule::from_code(py, &code, &file, &name) {
            Ok(m) => m,
            Err(e) => {
                return Result::Err(());
            }
        };

        let py_var: PyResult<T> = match py_mod.getattr(&variable) {
            Ok(v) => v.extract(),
            Err(e) => {
                return Result::Err(());
            }
        };

        match py_var {
            Ok(v) => Result::Ok(v),
            Err(e) => {
                return Result::Err(());
            }
        }
    })
}