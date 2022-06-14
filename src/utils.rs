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
    fn test_normal() {
        let mut text = WString::<LE>::from("Some text\0");
        let result = read_wstring(text.as_ptr().cast());
        text.pop();  // remove trailing zero character
        assert_eq!(result, text.as_bytes());
        
        let mut text = WString::<BE>::from("Some text\0");
        let result = read_wstring(text.as_ptr().cast());
        text.pop();  // remove trailing zero character
        assert_eq!(result, text.as_bytes());
    }
    
    #[test]
    fn test_with_garbage() {
        let text_with_garbage = WString::<LE>::from("Some text\0Some garbage");
        let clean_text = WString::<LE>::from("Some text");
        let result = read_wstring(text_with_garbage.as_ptr().cast());
        assert_eq!(result, clean_text.as_bytes())
    }
    
    #[test]
    fn test_empty() {
        let bytes = b"\0\0";
        assert!(read_wstring(bytes.as_ptr().cast()).is_empty());
    }
}