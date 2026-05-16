use std::ffi::{CStr, CString};

pub fn c_string(value: &str) -> Result<CString, String> {
    CString::new(value).map_err(|error| error.to_string())
}

pub unsafe fn take_optional_string(ptr: *mut core::ffi::c_char) -> Option<String> {
    if ptr.is_null() {
        return None;
    }

    let value = CStr::from_ptr(ptr).to_string_lossy().into_owned();
    libc::free(ptr.cast());
    Some(value)
}

pub unsafe fn take_string(ptr: *mut core::ffi::c_char) -> String {
    take_optional_string(ptr).unwrap_or_default()
}
