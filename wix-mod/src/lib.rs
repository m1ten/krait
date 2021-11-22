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

#[pymodule]
pub fn wix(py: Python, m: &PyModule) -> PyResult<()> {
    let name: &str = "wix";
    let version: &str = "0.1.0";

    m.add_wrapped(wrap_pyfunction!(hello))?;
    m.add_wrapped(wrap_pyfunction!(cmd))?;
    m.add_wrapped(wrap_pyfunction!(get))?;
    m.add("__name__", name)?;
    m.add("__version__", version)?;
    Ok(())
}