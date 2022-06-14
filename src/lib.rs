use pyo3::{prelude::*, types::PyBytes};
use utf16string::{LE, WString};
use std::{ptr::null};

#[no_mangle]
pub extern "C" fn Init() -> u64 {
    pyo3::prepare_freethreaded_python();
    
    return 0;
}

fn read_wstring(text: *const u16) -> Vec<u8> {
    let mut vector: Vec<u8> = Vec::new();
    
    unsafe {
        let mut idx: isize = 0;
        while *text.offset(idx) != 0 {
            let bytes = (*text.offset(idx)).to_le_bytes();
            vector.push(bytes[0]);
            vector.push(bytes[1]);
            idx += 1;
        } 
    }
    
    vector
}

#[no_mangle]
pub extern "C" fn Changetext(text: *const u16) -> *const u16 {
    let py_app = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/changetext.py"));
    
    let text = WString::<LE>::from_utf16le(read_wstring(text)).expect("Conversion error");
    
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
