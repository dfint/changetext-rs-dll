use pyo3::{prelude::*, types::PyBytes, types::PyTuple};
use utf16string::{LE, WString};

fn main() {
    pyo3::prepare_freethreaded_python();
    
    let py_app = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/changetext.py"));
    
    let from_python = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let changetext: Py<PyAny> = PyModule::from_code(py, py_app, "", "")?
            .getattr("ChangeText")?
            .into();
        
        
        let hello = WString::<LE>::from("hello");
        let b1 = hello.as_bytes();
        let py_bytes = PyBytes::new_with(py, b1.len(), |bytes: &mut [u8]| {
            bytes.copy_from_slice(b1);
            Ok(())
        })?;
        
        let args = PyTuple::new(py, &[py_bytes]);
        changetext.call1(py, args)
    });
    
    println!("py: {:?}", from_python);
    
}
