use core::ffi::{c_char, c_void};

extern "C" {
    /// Calls the `Metal` framework counterpart for `ametal_new_class_instance`.
    pub fn ametal_new_class_instance(class_name: *const c_char) -> *mut c_void;
    /// Calls the `Metal` framework counterpart for `ametal_copy_metal_string_constant`.
    pub fn ametal_copy_metal_string_constant(symbol_name: *const c_char) -> *mut c_char;
    /// Calls the `Metal` framework counterpart for `ametal_copy_all_devices`.
    pub fn ametal_copy_all_devices(out_count: *mut usize) -> *mut *mut c_void;
    /// Calls the `Metal` framework counterpart for `ametal_copy_all_devices_with_observer`.
    pub fn ametal_copy_all_devices_with_observer(
        out_count: *mut usize,
        out_observer: *mut *mut c_void,
        callback: Option<unsafe extern "C" fn(*mut c_void, *const c_char, *mut c_void)>,
        context: *mut c_void,
        release: Option<unsafe extern "C" fn(*mut c_void)>,
    ) -> *mut *mut c_void;
    /// Calls the `Metal` framework counterpart for `ametal_remove_device_observer`.
    pub fn ametal_remove_device_observer(observer_handle: *mut c_void);
    /// Calls the `Metal` framework counterpart for `ametal_io_compression_context_default_chunk_size`.
    pub fn ametal_io_compression_context_default_chunk_size() -> usize;
    /// Calls the `Metal` framework counterpart for `ametal_io_create_compression_context`.
    pub fn ametal_io_create_compression_context(
        path: *const c_char,
        method: usize,
        chunk_size: usize,
    ) -> *mut c_void;
    /// Calls the `Metal` framework counterpart for `ametal_io_compression_context_append_data`.
    pub fn ametal_io_compression_context_append_data(
        handle: *mut c_void,
        data: *const u8,
        size: usize,
    );
    /// Calls the `Metal` framework counterpart for `ametal_io_flush_and_destroy_compression_context`.
    pub fn ametal_io_flush_and_destroy_compression_context(handle: *mut c_void) -> usize;
    pub fn ametal_rasterization_rate_layer_descriptor_new(
        horizontal: usize,
        vertical: usize,
    ) -> *mut c_void;
}
