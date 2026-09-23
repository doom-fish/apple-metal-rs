use core::ffi::c_void;

extern "C" {
    /// Calls the `Metal` framework counterpart for `ametal_device_new_render_pipeline_state`.
    pub fn ametal_device_new_render_pipeline_state(
        device_handle: *mut c_void,
        vertex_handle: *mut c_void,
        fragment_handle: *mut c_void,
        color_pixel_format: usize,
        sample_count: usize,
        out_error_message: *mut *mut core::ffi::c_char,
    ) -> *mut c_void;

    /// Calls the `Metal` framework counterpart for `ametal_object_copy_label`.
    pub fn ametal_object_copy_label(handle: *mut c_void) -> *mut core::ffi::c_char;
}
