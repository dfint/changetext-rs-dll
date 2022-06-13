use pyo3::{prelude::*, types::PyBytes};
use utf16string::{LE, WString};

fn main() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    
    let py_app = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/changetext.py"));
    
    Python::with_gil(|py| -> PyResult<()> {
        let changetext: Py<PyAny> = PyModule::from_code(py, py_app, "", "")?
            .getattr("ChangeText")?
            .into();
        
        let hello = WString::<LE>::from("hello");
        let b1 = hello.as_bytes();
        let py_bytes = PyBytes::new_with(py, b1.len(), |bytes: &mut [u8]| {
            bytes.copy_from_slice(b1);
            Ok(())
        })?;
        
        let result = changetext.call1(py, (py_bytes,))?;
        let result: Vec<u8> = result.extract(py)?;
        
        let result = WString::from_utf16le(result).expect("UTF-16 decode error");
        let result = result.to_utf8();
        println!("Result: {result}");
        Ok(())
    })
}
