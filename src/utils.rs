pub(crate) fn read_wstring(text: *const u16) -> Vec<u8> {
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

#[cfg(test)]
mod tests {
    use utf16string::{WString, LE, BE};

    use super::*;
    
    #[test]
    fn test_read_wstring() {
        let text = WString::<LE>::from("Some text\0");
        let mut result = read_wstring(text.as_ptr().cast());
        result.push(0);
        result.push(0);
        assert_eq!(result, text.as_bytes());
        
        let text = WString::<BE>::from("Some text\0");
        let mut result = read_wstring(text.as_ptr().cast());
        result.push(0);
        result.push(0);
        assert_eq!(result, text.as_bytes());
    }
}