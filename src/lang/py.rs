use pyo3::Python;
use pyo3::prelude::*;
use crate as wix;

// get variable from python
pub fn get_info(code: String, file: String, name: String) -> wix::Package {
    Python::with_gil(|py| -> wix::Package {
        let py_mod = PyModule::from_code(py, &code, &file, &name).unwrap();
        let py_class = py_mod.getattr(&name).unwrap();
        
        wix::Package {
            name: py_class.getattr("name").unwrap().extract::<String>().unwrap(),
            version: py_class.getattr("version").unwrap().extract::<String>().unwrap(),
            url: py_class.getattr("url").unwrap().extract::<String>().unwrap(),
        }
        
    })
}