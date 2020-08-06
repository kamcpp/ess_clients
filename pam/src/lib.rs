use std::os::raw::c_char;
use std::ffi::{CString, CStr};

#[no_mangle]
pub extern "C" fn new_identity_verify_request(c_username: *const c_char) -> *const c_char {
    let username = unsafe { CStr::from_ptr(c_username).to_string_lossy().into_owned() };
    let greeting = format!("Hello {}", username);
    return CString::new(greeting).unwrap().into_raw();
}
