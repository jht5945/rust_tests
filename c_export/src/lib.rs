extern crate libc;

use std::ffi::CStr;
use libc::c_char;

#[no_mangle]
pub extern "C" fn count_substrings(value: *const c_char, substr: *const c_char) -> i32 {
    let c_value = unsafe { CStr::from_ptr(value) };
    let c_substr = unsafe { CStr::from_ptr(substr) };
    match c_value.to_str() {
        Ok(value) => match c_substr.to_str() {
            Ok(substr) => value.match_indices(substr).count() as i32,
            Err(_) => -1,
        },
        Err(_) => -1,
    }
}
