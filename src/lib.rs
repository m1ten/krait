use pyo3::prelude::*;
use std::io;
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
pub fn get(_py: Python, url: String, file: String) -> u64{
    let mut resp = reqwest::blocking::get(url).expect("Failed to get");
    let mut out = File::create(file).expect("failed to create file");
    io::copy(&mut resp, &mut out).expect("failed to copy")
}

#[pyfunction]
pub fn hello() {
    println!("Hello, Python!");
}

#[pymodule]
pub fn wix_py(_py: Python, m: &PyModule) -> PyResult<()> {

    let name: &str = "wix_py";
    let version: &str = "0.1.0";

    m.add_function(wrap_pyfunction!(cmd, m)?)?;
    m.add_function(wrap_pyfunction!(get, m)?)?;
    m.add_function(wrap_pyfunction!(hello, m)?)?;
    m.add("__name__", name)?;
    m.add("__version__", version)?;

    Ok(())
}