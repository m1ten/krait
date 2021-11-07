use pyo3::Python;
use pyo3::prelude::*;
use std::fs::File;

#[pyfunction]
pub fn cmd(cmd: String, args: Vec<String>) -> PyResult<String> {
    let child = std::process::Command::new(cmd)
        .args(args)
        .stdout(std::process::Stdio::piped())
        .spawn()?;

    let output = child.wait_with_output()?;

    Ok(String::from_utf8(output.stdout).unwrap())
}

#[pyfunction]
pub fn get(_py: Python, url: String, file: String) -> u64 {
    let mut resp = reqwest::blocking::get(url).expect("Failed to get");
    let mut out = File::create(file).expect("failed to create file");
    std::io::copy(&mut resp, &mut out).expect("failed to copy")
}

#[pyfunction]
pub fn hello() {
    println!("Hello, Python!");
}

pub fn wix(m: &PyModule) -> &PyModule
{
    let name: &str = "wix";
    let version: &str = "0.1.0";

    m.add_function(wrap_pyfunction!(cmd, m).unwrap()).unwrap();
    m.add_function(wrap_pyfunction!(get, m).unwrap()).unwrap();
    m.add_function(wrap_pyfunction!(hello, m).unwrap()).unwrap();
    m.add("__name__", name).unwrap();
    m.add("__version__", version).unwrap();

    m
}

// get variable from python file
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