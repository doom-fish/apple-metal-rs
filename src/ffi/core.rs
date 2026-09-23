//! Raw FFI declarations matching `swift-bridge/Sources/AppleMetalBridge/AppleMetal.swift`.

#![allow(missing_docs)]

use core::ffi::c_void;

extern "C" {
    /// Calls the `Metal` framework counterpart for `ametal_device_system_default`.
    pub fn ametal_device_system_default() -> *mut c_void;
    /// Calls the `Metal` framework counterpart for `ametal_device_release`.
    pub fn ametal_device_release(handle: *mut c_void);
    /// Calls the `Metal` framework counterpart for `ametal_device_has_unified_memory`.
    pub fn ametal_device_has_unified_memory(handle: *mut c_void) -> bool;
    /// Calls the `Metal` framework counterpart for `ametal_device_recommended_max_working_set_size`.
    pub fn ametal_device_recommended_max_working_set_size(handle: *mut c_void) -> u64;
    /// Calls the `Metal` framework counterpart for `ametal_device_supports_family`.
    pub fn ametal_device_supports_family(handle: *mut c_void, family: i64) -> bool;

    /// Calls the `Metal` framework counterpart for `ametal_device_new_buffer`.
    pub fn ametal_device_new_buffer(
        device_handle: *mut c_void,
        length: usize,
        options: usize,
    ) -> *mut c_void;
    /// Calls the `Metal` framework counterpart for `ametal_buffer_release`.
    pub fn ametal_buffer_release(handle: *mut c_void);
    /// Calls the `Metal` framework counterpart for `ametal_buffer_length`.
    pub fn ametal_buffer_length(handle: *mut c_void) -> usize;
    /// Calls the `Metal` framework counterpart for `ametal_buffer_storage_mode`.
    pub fn ametal_buffer_storage_mode(handle: *mut c_void) -> usize;
    /// Calls the `Metal` framework counterpart for `ametal_buffer_contents`.
    pub fn ametal_buffer_contents(handle: *mut c_void) -> *mut c_void;
    /// Calls the `Metal` framework counterpart for `ametal_buffer_new_staging_buffer`.
    pub fn ametal_buffer_new_staging_buffer(handle: *mut c_void) -> *mut c_void;

    /// Calls the `Metal` framework counterpart for `ametal_device_new_texture_2d`.
    pub fn ametal_device_new_texture_2d(
        device_handle: *mut c_void,
        pixel_format: usize,
        width: usize,
        height: usize,
        mipmapped: bool,
        usage: usize,
        storage_mode: usize,
    ) -> *mut c_void;
    /// Calls the `Metal` framework counterpart for `ametal_texture_release`.
    pub fn ametal_texture_release(handle: *mut c_void);
    /// Calls the `Metal` framework counterpart for `ametal_texture_width`.
    pub fn ametal_texture_width(handle: *mut c_void) -> usize;
    /// Calls the `Metal` framework counterpart for `ametal_texture_height`.
    pub fn ametal_texture_height(handle: *mut c_void) -> usize;
    /// Calls the `Metal` framework counterpart for `ametal_texture_pixel_format`.
    pub fn ametal_texture_pixel_format(handle: *mut c_void) -> usize;
    /// Calls the `Metal` framework counterpart for `ametal_texture_type`.
    pub fn ametal_texture_type(handle: *mut c_void) -> usize;

    /// Calls the `Metal` framework counterpart for `ametal_device_new_texture_from_iosurface`.
    pub fn ametal_device_new_texture_from_iosurface(
        device_handle: *mut c_void,
        iosurface_ptr: *mut c_void,
        plane_index: usize,
        pixel_format: usize,
        width: usize,
        height: usize,
    ) -> *mut c_void;

    /// Calls the `Metal` framework counterpart for `ametal_device_new_command_queue`.
    pub fn ametal_device_new_command_queue(device_handle: *mut c_void) -> *mut c_void;
    /// Calls the `Metal` framework counterpart for `ametal_command_queue_release`.
    pub fn ametal_command_queue_release(handle: *mut c_void);
    /// Calls the `Metal` framework counterpart for `ametal_command_queue_new_command_buffer`.
    pub fn ametal_command_queue_new_command_buffer(handle: *mut c_void) -> *mut c_void;
    /// Calls the `Metal` framework counterpart for `ametal_command_buffer_release`.
    pub fn ametal_command_buffer_release(handle: *mut c_void);
    /// Calls the `Metal` framework counterpart for `ametal_command_buffer_commit`.
    pub fn ametal_command_buffer_commit(handle: *mut c_void);
    /// Calls the `Metal` framework counterpart for `ametal_command_buffer_wait_until_completed`.
    pub fn ametal_command_buffer_wait_until_completed(handle: *mut c_void);
    /// Calls the `Metal` framework counterpart for `ametal_command_buffer_blit_copy_buffer`.
    pub fn ametal_command_buffer_blit_copy_buffer(
        cb_handle: *mut c_void,
        src_handle: *mut c_void,
        src_offset: usize,
        dst_handle: *mut c_void,
        dst_offset: usize,
        size: usize,
    ) -> bool;

    // ---- Library + Function + ComputePipelineState + dispatch (v0.5) ----
    /// Calls the `Metal` framework counterpart for `ametal_device_new_library_with_source`.
    pub fn ametal_device_new_library_with_source(
        device_handle: *mut c_void,
        source: *const core::ffi::c_char,
        out_error_message: *mut *mut core::ffi::c_char,
    ) -> *mut c_void;
    /// Calls the `Metal` framework counterpart for `ametal_library_release`.
    pub fn ametal_library_release(handle: *mut c_void);
    /// Calls the `Metal` framework counterpart for `ametal_library_new_function`.
    pub fn ametal_library_new_function(
        lib_handle: *mut c_void,
        name: *const core::ffi::c_char,
    ) -> *mut c_void;
    /// Calls the `Metal` framework counterpart for `ametal_function_release`.
    pub fn ametal_function_release(handle: *mut c_void);
    /// Calls the `Metal` framework counterpart for `ametal_device_new_compute_pipeline_state`.
    pub fn ametal_device_new_compute_pipeline_state(
        device_handle: *mut c_void,
        fn_handle: *mut c_void,
        out_error_message: *mut *mut core::ffi::c_char,
    ) -> *mut c_void;
    /// Calls the `Metal` framework counterpart for `ametal_compute_pipeline_state_release`.
    pub fn ametal_compute_pipeline_state_release(handle: *mut c_void);
    /// Calls the `Metal` framework counterpart for `ametal_command_buffer_dispatch_compute_1d`.
    pub fn ametal_command_buffer_dispatch_compute_1d(
        cb_handle: *mut c_void,
        pso_handle: *mut c_void,
        buffers: *const *mut c_void,
        buffer_count: usize,
        threadgroups: usize,
        threads_per_group: usize,
    ) -> bool;
}
