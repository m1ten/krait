use pyo3::Python;
use pyo3::prelude::*;
use crate as wix;

// get variable from python
pub fn get_variable<T>(code: String, file: String, name: String, variable: String) -> Result<T, String> 
where
    T: for<'p> FromPyObject<'p>
{
    Python::with_gil(|py| -> Result<T, String> {

        let py_mod = match PyModule::from_code(py, &code, &file, &name) {
            Ok(m) => m,
            Err(e) => {
                return Result::Err(e.to_string());
            }
        };

        let py_var: PyResult<T> = match py_mod.getattr(&variable) {
            Ok(v) => v.extract(),
            Err(e) => {
                return Result::Err(e.to_string());
            }
        };

        match py_var {
            Ok(v) => Result::Ok(v),
            Err(e) => {
                return Result::Err(e.to_string());
            }
        }
    })
}

// function to convert struct to python code
pub fn struct_to_code(struct_name: String, struct_contents: Vec<[String; 2]>) -> String {
    let mut code = String::new();
    code.push_str(&format!("{} = {}", struct_name, "{}"));
    code.push_str("\n");
    for data in struct_contents {
        // write code to check type of data[1]
        code.push_str(&format!("{}.{} = {}", struct_name, data[0], data[1]));
        code.push_str("\n");
    }
    code
}