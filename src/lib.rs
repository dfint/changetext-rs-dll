use pyo3::{prelude::*, types::PyBytes};
use utf16string::{LE, WString};
mod utils;

#[no_mangle]
pub extern "C" fn Init() -> u64 {
    pyo3::prepare_freethreaded_python();
    
    return 0;
}

#[no_mangle]
pub extern "C" fn Changetext(text: *const u16) -> *const u16 {
    let py_app = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/changetext.py"));
    
    let text = WString::<LE>::from_utf16le(utils::read_wstring(text)).expect("Conversion error");
    
    let mut result_text = String::new();
    
    Python::with_gil(|py| -> PyResult<()> {
        let changetext: Py<PyAny> = PyModule::from_code(py, py_app, "", "")?
            .getattr("ChangeText")?
            .into();
        
        let py_bytes = PyBytes::new(py, text.as_bytes());
        
        let result = changetext.call1(py, (py_bytes,))?;
        let result: Vec<u8> = result.extract(py)?;
        
        let result = WString::from_utf16le(result).expect("UTF-16 decode error");
        result_text.clone_from(&result.to_utf8());
        
        Ok(())
    }).expect("Something went wrong");
    
    return result_text.as_ptr().cast();
}
