#![doc = include_str!("../README.md")]
//!
//! ---
//!
//! # API Documentation

#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::missing_const_for_fn)]

use core::ffi::c_void;
use core::ptr;

pub mod ffi;

/// Common `MTLPixelFormat` constants.
pub mod pixel_format {
    pub const A8UNORM: usize = 1;
    pub const R8UNORM: usize = 10;
    pub const R8SNORM: usize = 12;
    pub const R8UINT: usize = 13;
    pub const R8SINT: usize = 14;
    pub const R16UNORM: usize = 20;
    pub const R16SNORM: usize = 22;
    pub const R16UINT: usize = 23;
    pub const R16SINT: usize = 24;
    pub const R16FLOAT: usize = 25;
    pub const RG8UNORM: usize = 30;
    pub const RG8SNORM: usize = 32;
    pub const RG8UINT: usize = 33;
    pub const RG8SINT: usize = 34;
    pub const RGBA8UNORM: usize = 70;
    pub const RGBA8UNORM_SRGB: usize = 71;
    pub const RGBA8SNORM: usize = 72;
    pub const RGBA8UINT: usize = 73;
    pub const RGBA8SINT: usize = 74;
    pub const BGRA8UNORM: usize = 80;
    pub const BGRA8UNORM_SRGB: usize = 81;
    pub const R32FLOAT: usize = 55;
    pub const RG16FLOAT: usize = 65;
    pub const RGBA16FLOAT: usize = 115;
    pub const RGBA32FLOAT: usize = 125;
    pub const DEPTH32FLOAT: usize = 252;
    pub const STENCIL8: usize = 253;
    pub const BGRA10_XR: usize = 552;
    pub const BGR10_XR: usize = 554;
}

/// `MTLStorageMode` enum values — memory residency hints.
pub mod storage_mode {
    pub const SHARED: usize = 0;
    pub const MANAGED: usize = 1;
    pub const PRIVATE: usize = 2;
    pub const MEMORYLESS: usize = 3;
}

/// `MTLResourceOptions` bitmask values.
pub mod resource_options {
    pub const CPU_CACHE_MODE_DEFAULT: usize = 0;
    pub const CPU_CACHE_MODE_WRITE_COMBINED: usize = 1;
    pub const STORAGE_MODE_SHARED: usize = 0;
    pub const STORAGE_MODE_MANAGED: usize = 1 << 4;
    pub const STORAGE_MODE_PRIVATE: usize = 2 << 4;
    pub const HAZARD_TRACKING_MODE_DEFAULT: usize = 0;
    pub const HAZARD_TRACKING_MODE_UNTRACKED: usize = 1 << 8;
    pub const HAZARD_TRACKING_MODE_TRACKED: usize = 2 << 8;
}

/// `MTLTextureUsage` bitmask.
pub mod texture_usage {
    pub const SHADER_READ: usize = 0x01;
    pub const SHADER_WRITE: usize = 0x02;
    pub const RENDER_TARGET: usize = 0x04;
}

/// `MTLGPUFamily` — feature-family identifiers.
pub mod gpu_family {
    pub const APPLE1: i64 = 1001;
    pub const APPLE2: i64 = 1002;
    pub const APPLE3: i64 = 1003;
    pub const APPLE4: i64 = 1004;
    pub const APPLE5: i64 = 1005;
    pub const APPLE6: i64 = 1006;
    pub const APPLE7: i64 = 1007;
    pub const APPLE8: i64 = 1008;
    pub const APPLE9: i64 = 1009;
    pub const MAC1: i64 = 2001;
    pub const MAC2: i64 = 2002;
    pub const COMMON1: i64 = 3001;
    pub const COMMON2: i64 = 3002;
    pub const COMMON3: i64 = 3003;
    pub const METAL3: i64 = 5001;
}

// ---- Device ----

/// Apple's `id<MTLDevice>` — handle to a Metal GPU.
pub struct MetalDevice {
    ptr: *mut c_void,
    drop_on_release: bool,
}

unsafe impl Send for MetalDevice {}
unsafe impl Sync for MetalDevice {}

impl Drop for MetalDevice {
    fn drop(&mut self) {
        if self.drop_on_release && !self.ptr.is_null() {
            unsafe { ffi::am_device_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl MetalDevice {
    /// Return the system's default Metal device.
    #[must_use]
    pub fn system_default() -> Option<Self> {
        let p = unsafe { ffi::am_device_system_default() };
        if p.is_null() {
            None
        } else {
            Some(Self {
                ptr: p,
                drop_on_release: true,
            })
        }
    }

    /// Raw `id<MTLDevice>` pointer.
    #[must_use]
    pub const fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }

    /// True if the GPU uses unified memory (Apple Silicon).
    #[must_use]
    pub fn has_unified_memory(&self) -> bool {
        unsafe { ffi::am_device_has_unified_memory(self.ptr) }
    }

    /// Recommended maximum working-set size in bytes.
    #[must_use]
    pub fn recommended_max_working_set_size(&self) -> u64 {
        unsafe { ffi::am_device_recommended_max_working_set_size(self.ptr) }
    }

    /// True if this device supports the requested feature family —
    /// see [`gpu_family`].
    #[must_use]
    pub fn supports_family(&self, family: i64) -> bool {
        unsafe { ffi::am_device_supports_family(self.ptr, family) }
    }

    /// Allocate a GPU-visible buffer of `length` bytes.
    /// `options` is an `MTLResourceOptions` bitmask (see
    /// [`resource_options`]).
    #[must_use]
    pub fn new_buffer(&self, length: usize, options: usize) -> Option<MetalBuffer> {
        let p = unsafe { ffi::am_device_new_buffer(self.ptr, length, options) };
        if p.is_null() {
            None
        } else {
            Some(MetalBuffer { ptr: p })
        }
    }

    /// Allocate a fresh `MTLTexture` matching `descriptor`.
    #[must_use]
    pub fn new_texture(&self, descriptor: TextureDescriptor) -> Option<MetalTexture> {
        let p = unsafe {
            ffi::am_device_new_texture_2d(
                self.ptr,
                descriptor.pixel_format,
                descriptor.width,
                descriptor.height,
                descriptor.mipmapped,
                descriptor.usage,
                descriptor.storage_mode,
            )
        };
        if p.is_null() {
            None
        } else {
            Some(MetalTexture { ptr: p })
        }
    }

    /// Wrap a raw `id<MTLDevice>` pointer **without** taking ownership.
    /// The returned handle will NOT release the underlying object on
    /// drop.
    ///
    /// # Safety
    ///
    /// `ptr` must be a valid `id<MTLDevice>` whose lifetime is managed
    /// by some other owner.
    #[must_use]
    pub unsafe fn from_raw_borrowed(ptr: *mut c_void) -> ManuallyDropDevice {
        ManuallyDropDevice {
            inner: Self {
                ptr,
                drop_on_release: false,
            },
        }
    }
}

/// Borrowed [`MetalDevice`] that does not release on drop.
pub struct ManuallyDropDevice {
    inner: MetalDevice,
}

impl core::ops::Deref for ManuallyDropDevice {
    type Target = MetalDevice;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

// ---- Buffer ----

/// Apple's `id<MTLBuffer>` — a GPU-visible byte buffer.
pub struct MetalBuffer {
    ptr: *mut c_void,
}

unsafe impl Send for MetalBuffer {}
unsafe impl Sync for MetalBuffer {}

impl Drop for MetalBuffer {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::am_buffer_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl MetalBuffer {
    /// Buffer length in bytes.
    #[must_use]
    pub fn length(&self) -> usize {
        unsafe { ffi::am_buffer_length(self.ptr) }
    }

    /// Raw `void *` to the buffer's CPU-visible bytes. `None` for
    /// `MTLStorageMode::Private` (GPU-only) buffers.
    #[must_use]
    pub fn contents(&self) -> Option<*mut c_void> {
        let p = unsafe { ffi::am_buffer_contents(self.ptr) };
        if p.is_null() {
            None
        } else {
            Some(p)
        }
    }

    /// Copy `src` into this buffer at byte offset `0`. Returns the
    /// number of bytes actually written.
    #[must_use] 
    pub fn write_bytes(&self, src: &[u8]) -> usize {
        let Some(dst) = self.contents() else {
            return 0;
        };
        let n = core::cmp::min(src.len(), self.length());
        unsafe { core::ptr::copy_nonoverlapping(src.as_ptr(), dst.cast::<u8>(), n) };
        n
    }

    /// Raw `id<MTLBuffer>` pointer.
    #[must_use]
    pub const fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

// ---- Texture descriptor + texture ----

/// Configuration for `MetalDevice::new_texture`.
#[derive(Debug, Clone, Copy)]
pub struct TextureDescriptor {
    pub pixel_format: usize,
    pub width: usize,
    pub height: usize,
    pub mipmapped: bool,
    pub usage: usize,
    pub storage_mode: usize,
}

impl TextureDescriptor {
    /// Sensible defaults for a shader-read+write 2D texture in shared storage.
    #[must_use]
    pub const fn new_2d(width: usize, height: usize, pixel_format: usize) -> Self {
        Self {
            pixel_format,
            width,
            height,
            mipmapped: false,
            usage: texture_usage::SHADER_READ | texture_usage::SHADER_WRITE,
            storage_mode: storage_mode::SHARED,
        }
    }
}

/// Apple's `id<MTLTexture>` — a GPU-resident 2D image.
pub struct MetalTexture {
    ptr: *mut c_void,
}

unsafe impl Send for MetalTexture {}
unsafe impl Sync for MetalTexture {}

impl Drop for MetalTexture {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::am_texture_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl MetalTexture {
    /// Texture width in pixels.
    #[must_use]
    pub fn width(&self) -> usize {
        unsafe { ffi::am_texture_width(self.ptr) }
    }

    /// Texture height in pixels.
    #[must_use]
    pub fn height(&self) -> usize {
        unsafe { ffi::am_texture_height(self.ptr) }
    }

    /// Underlying `MTLPixelFormat` enum value — see [`pixel_format`].
    #[must_use]
    pub fn pixel_format(&self) -> usize {
        unsafe { ffi::am_texture_pixel_format(self.ptr) }
    }

    /// Raw `id<MTLTexture>` pointer.
    #[must_use]
    pub const fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }

    /// Wrap a raw `id<MTLTexture>` pointer. Pointer is taken without
    /// retain.
    ///
    /// # Safety
    ///
    /// `ptr` must be a valid `id<MTLTexture>` whose ownership the
    /// caller is transferring.
    #[must_use]
    pub const unsafe fn from_raw(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

// ---- IOSurface extension ----

#[cfg(feature = "iosurface")]
#[cfg_attr(docsrs, doc(cfg(feature = "iosurface")))]
mod iosurface_ext {
    use super::{ffi, pixel_format, MetalDevice, MetalTexture};
    use apple_cf::iosurface::IOSurface;
    use core::ffi::c_void;

    /// Add Metal interop methods to [`IOSurface`].
    pub trait IOSurfaceMetalExt {
        /// Wrap the given plane of this `IOSurface` as a zero-copy
        /// [`MetalTexture`] on the given device.
        fn create_metal_texture(
            &self,
            device: &MetalDevice,
            plane_index: usize,
        ) -> Option<MetalTexture>;
    }

    impl IOSurfaceMetalExt for IOSurface {
        fn create_metal_texture(
            &self,
            device: &MetalDevice,
            plane_index: usize,
        ) -> Option<MetalTexture> {
            let format = pixel_format_for_fourcc(self.pixel_format(), plane_index)?;
            let (width, height) = if plane_index == 0 {
                (self.width(), self.height())
            } else {
                (self.width() / 2, self.height() / 2)
            };
            let p = unsafe {
                ffi::am_device_new_texture_from_iosurface(
                    device.as_ptr(),
                    self.as_ptr().cast::<c_void>(),
                    plane_index,
                    format,
                    width,
                    height,
                )
            };
            if p.is_null() {
                None
            } else {
                Some(unsafe { MetalTexture::from_raw(p) })
            }
        }
    }

    fn pixel_format_for_fourcc(fourcc: u32, plane_index: usize) -> Option<usize> {
        const BGRA: u32 = u32::from_be_bytes(*b"BGRA");
        const L10R: u32 = u32::from_be_bytes(*b"l10r");
        const YUV420V: u32 = u32::from_be_bytes(*b"420v");
        const YUV420F: u32 = u32::from_be_bytes(*b"420f");

        match (fourcc, plane_index) {
            (BGRA, 0) => Some(pixel_format::BGRA8UNORM),
            (L10R, 0) => Some(pixel_format::BGRA10_XR),
            (YUV420V | YUV420F, 0) => Some(pixel_format::R8UNORM),
            (YUV420V | YUV420F, 1) => Some(pixel_format::RG8UNORM),
            _ => None,
        }
    }
}

#[cfg(feature = "iosurface")]
pub use iosurface_ext::IOSurfaceMetalExt;

/// True if `fourcc` identifies a YCbCr biplanar (`Y` + `CbCr`) format.
#[must_use]
pub const fn is_ycbcr_biplanar(fourcc: u32) -> bool {
    const YUV420V: u32 = u32::from_be_bytes(*b"420v");
    const YUV420F: u32 = u32::from_be_bytes(*b"420f");
    matches!(fourcc, YUV420V | YUV420F)
}
