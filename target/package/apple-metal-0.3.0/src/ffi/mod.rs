//! Raw FFI declarations matching `swift-bridge/Sources/AppleMetalBridge/AppleMetal.swift`.

#![allow(missing_docs)]

use core::ffi::c_void;

extern "C" {
    pub fn am_device_system_default() -> *mut c_void;
    pub fn am_device_release(handle: *mut c_void);
    pub fn am_device_has_unified_memory(handle: *mut c_void) -> bool;
    pub fn am_device_recommended_max_working_set_size(handle: *mut c_void) -> u64;
    pub fn am_device_supports_family(handle: *mut c_void, family: i64) -> bool;

    pub fn am_device_new_buffer(
        device_handle: *mut c_void,
        length: usize,
        options: usize,
    ) -> *mut c_void;
    pub fn am_buffer_release(handle: *mut c_void);
    pub fn am_buffer_length(handle: *mut c_void) -> usize;
    pub fn am_buffer_contents(handle: *mut c_void) -> *mut c_void;

    pub fn am_device_new_texture_2d(
        device_handle: *mut c_void,
        pixel_format: usize,
        width: usize,
        height: usize,
        mipmapped: bool,
        usage: usize,
        storage_mode: usize,
    ) -> *mut c_void;
    pub fn am_texture_release(handle: *mut c_void);
    pub fn am_texture_width(handle: *mut c_void) -> usize;
    pub fn am_texture_height(handle: *mut c_void) -> usize;
    pub fn am_texture_pixel_format(handle: *mut c_void) -> usize;

    pub fn am_device_new_texture_from_iosurface(
        device_handle: *mut c_void,
        iosurface_ptr: *mut c_void,
        plane_index: usize,
        pixel_format: usize,
        width: usize,
        height: usize,
    ) -> *mut c_void;
}
