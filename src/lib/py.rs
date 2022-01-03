use pyo3::Python;
use pyo3::prelude::*;

// get data from python file
pub fn get_data<T>(code: String, file: String, name: String, variable: Option<String>, function: Option<String>) -> Result<T, String> 
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

        if variable.is_none() && function.is_none() {
            return Err("No variable or function specified".to_string());
        } else if variable.is_some() && function.is_some() {
            return Err("Only one of variable or function can be specified".to_string());
        } else if variable.is_some() {
            let py_var: PyResult<T> = match py_mod.getattr(&variable.unwrap()) {
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
        } else if function.is_some() {
            let py_func: PyResult<T> = match py_mod.getattr(&function) {
                Ok(f) => f.extract(),
                Err(e) => {
                    return Result::Err(e.to_string());
                }
            };
    
            match py_func {
                Ok(f) => Result::Ok(f),
                Err(e) => {
                    return Result::Err(e.to_string());
                }
            }
        } else {
            return Err("No variable or function specified".to_string());
        }
    })
}

// call python function
pub fn call_func(code: String, file: String, name: String, function: String) -> Result<(), String> {
    Python::with_gil(|py| {
        let py_mod = PyModule::from_code(py, &code, &file, &name).unwrap();
        let py_func = py_mod.getattr(function).unwrap();
        match py_func.call0() {
            Ok(_) => Result::Ok(()),
            Err(e) => Result::Err(e.to_string())
        }
    })
}

// function to convert struct to python variable code
pub fn struct_to_py(struct_name: String, struct_contents: indexmap::IndexMap<String, String>) -> String {
    let mut code = String::new();
    code.push_str(&format!("{} = {}", struct_name, "{}"));
    code.push_str("\n");
    for data in struct_contents {

        match data.1.parse::<i128>() {
            Ok(i) => {
                code.push_str(&format!("{}.{} = {}", struct_name, data.0, i));
                code.push_str("\n");
                continue;
            },
            Err(_) => ()
        }

        match data.1.parse::<f64>() {
            Ok(f) => {
                code.push_str(&format!("{}.{} = {}", struct_name, data.0, f));
                code.push_str("\n");
                continue;
            },
            Err(_) => ()
        }

        match data.1.parse::<bool>() {
            Ok(b) => {
                let b = if b { "True" } else { "False" };
                code.push_str(&format!("{}.{} = {}", struct_name, data.0, b));
                code.push_str("\n");
                continue;
            },
            Err(_) => ()
        }

        code.push_str(&format!("{}.{} = '{}'", struct_name, data.0, data.1));
        code.push_str("\n");
    }
    code
}