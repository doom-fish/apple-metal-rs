use core::ffi::{c_char, c_void};

extern "C" {
    pub fn am_new_class_instance(class_name: *const c_char) -> *mut c_void;
    pub fn am_copy_metal_string_constant(symbol_name: *const c_char) -> *mut c_char;
    pub fn am_copy_all_devices(out_count: *mut usize) -> *mut *mut c_void;
    pub fn am_copy_all_devices_with_observer(
        out_count: *mut usize,
        out_observer: *mut *mut c_void,
        callback: Option<unsafe extern "C" fn(*mut c_void, *const c_char, *mut c_void)>,
        user_data: *mut c_void,
    ) -> *mut *mut c_void;
    pub fn am_remove_device_observer(observer_handle: *mut c_void);
    pub fn am_io_compression_context_default_chunk_size() -> usize;
    pub fn am_io_create_compression_context(
        path: *const c_char,
        method: usize,
        chunk_size: usize,
    ) -> *mut c_void;
    pub fn am_io_compression_context_append_data(handle: *mut c_void, data: *const u8, size: usize);
    pub fn am_io_flush_and_destroy_compression_context(handle: *mut c_void) -> usize;
}
