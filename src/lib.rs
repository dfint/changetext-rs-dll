use std::ptr::null;

#[no_mangle]
pub extern "C" fn Init() -> u64 {
    return 0;
}

#[no_mangle]
pub extern "C" fn Changetext(_text: *const u16) -> *const u16 {
    return null();
}
