#![doc = include_str!("../README.md")]
//!
//! ---
//!
//! # API Documentation

#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    clippy::missing_safety_doc,
    clippy::too_many_lines,
    clippy::doc_markdown,
    clippy::module_name_repetitions,
    non_camel_case_types,
    clippy::upper_case_acronyms,
    clippy::duplicated_attributes,
    clippy::missing_const_for_fn,
    clippy::ptr_as_ptr,
    clippy::items_after_statements
)]

use core::ffi::c_void;
use core::ptr;
use std::ffi::CString;

// ---- FFI ----

type id = *mut c_void;
type SEL = *const c_void;
type Class = *const c_void;

#[link(name = "Metal", kind = "framework")]
#[link(name = "Foundation", kind = "framework")]
#[link(name = "objc")]
extern "C" {
    fn MTLCreateSystemDefaultDevice() -> id;
    fn objc_msgSend();
    fn sel_registerName(name: *const i8) -> SEL;
    fn objc_getClass(name: *const i8) -> Class;
    fn objc_release(obj: id);
}

unsafe fn sel(name: &str) -> SEL {
    let c = CString::new(name).expect("selector name has no NUL");
    sel_registerName(c.as_ptr())
}

unsafe fn class_named(name: &str) -> Class {
    let c = CString::new(name).expect("class name has no NUL");
    objc_getClass(c.as_ptr())
}

// `objc_msgSend` is variadic; we cast it to the concrete signature per call.
type MsgSend0 = unsafe extern "C" fn(id, SEL) -> id;
type MsgSendUsize = unsafe extern "C" fn(id, SEL) -> usize;
type MsgSendU32 = unsafe extern "C" fn(id, SEL) -> u32;
type MsgSendTextureFromIOSurface =
    unsafe extern "C" fn(id, SEL, id, *mut c_void, usize) -> id;
type MsgSendTextureDescriptorInit =
    unsafe extern "C" fn(id, SEL, u32, usize, usize, bool) -> id;
type MsgSendSetUsage = unsafe extern "C" fn(id, SEL, usize);
type MsgSendSetStorage = unsafe extern "C" fn(id, SEL, usize);

/// Common `MTLPixelFormat` constants.
pub mod pixel_format {
    pub const A8UNORM: u32 = 1;
    pub const R8UNORM: u32 = 10;
    pub const R8SNORM: u32 = 12;
    pub const R8UINT: u32 = 13;
    pub const R8SINT: u32 = 14;
    pub const R16UNORM: u32 = 20;
    pub const R16SNORM: u32 = 22;
    pub const R16UINT: u32 = 23;
    pub const R16SINT: u32 = 24;
    pub const R16FLOAT: u32 = 25;
    pub const RG8UNORM: u32 = 30;
    pub const RG8SNORM: u32 = 32;
    pub const RG8UINT: u32 = 33;
    pub const RG8SINT: u32 = 34;
    pub const RGBA8UNORM: u32 = 70;
    pub const RGBA8UNORM_SRGB: u32 = 71;
    pub const RGBA8SNORM: u32 = 72;
    pub const RGBA8UINT: u32 = 73;
    pub const RGBA8SINT: u32 = 74;
    pub const BGRA8UNORM: u32 = 80;
    pub const BGRA8UNORM_SRGB: u32 = 81;
    pub const R32FLOAT: u32 = 55;
    pub const RG16FLOAT: u32 = 65;
    pub const RGBA16FLOAT: u32 = 115;
    pub const RGBA32FLOAT: u32 = 125;
    pub const DEPTH32FLOAT: u32 = 252;
    pub const STENCIL8: u32 = 253;
    pub const BGRA10_XR: u32 = 552;
    pub const BGR10_XR: u32 = 554;
}

const MTL_TEXTURE_USAGE_SHADER_READ: usize = 0x01;
const MTL_TEXTURE_USAGE_SHADER_WRITE: usize = 0x02;
const MTL_STORAGE_MODE_SHARED: usize = 0;

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
    ptr: id,
}

unsafe impl Send for MetalDevice {}
unsafe impl Sync for MetalDevice {}

impl Drop for MetalDevice {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { objc_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl MetalDevice {
    /// Return the system's default Metal device.
    #[must_use]
    pub fn system_default() -> Option<Self> {
        let p = unsafe { MTLCreateSystemDefaultDevice() };
        if p.is_null() {
            None
        } else {
            Some(Self { ptr: p })
        }
    }

    /// Raw `id<MTLDevice>` pointer — for interop with other Metal-using
    /// Rust crates (`metal`, `objc2-metal`, ...).
    #[must_use]
    pub const fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }

    /// Wrap a raw `id<MTLDevice>` pointer **without** taking ownership.
    /// The returned handle will NOT release the underlying object on
    /// drop, so the caller is responsible for keeping the original
    /// owner alive for as long as this borrowed handle is used.
    ///
    /// Use this for short-lived bridging between crates (e.g. when
    /// `screencapturekit::metal::MetalDevice::as_apple_metal()` wants
    /// to expose its device to apple-metal helpers).
    ///
    /// # Safety
    ///
    /// `ptr` must be a valid `id<MTLDevice>` whose lifetime is managed
    /// by some other owner.
    #[must_use]
    pub unsafe fn from_raw_borrowed(ptr: *mut c_void) -> ManuallyDropDevice {
        ManuallyDropDevice {
            inner: core::mem::ManuallyDrop::new(Self { ptr }),
        }
    }
}

/// Borrowed [`MetalDevice`] that does not release on drop. Deref to
/// [`MetalDevice`] for use with anything that takes `&MetalDevice`.
pub struct ManuallyDropDevice {
    inner: core::mem::ManuallyDrop<MetalDevice>,
}

impl core::ops::Deref for ManuallyDropDevice {
    type Target = MetalDevice;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl MetalDevice {
    /// True if the GPU uses unified memory (Apple Silicon) — i.e.
    /// CPU and GPU share the same physical RAM. Wraps
    /// `[MTLDevice hasUnifiedMemory]`.
    #[must_use]
    pub fn has_unified_memory(&self) -> bool {
        unsafe {
            type M = unsafe extern "C" fn(id, SEL) -> bool;
            let m: M = core::mem::transmute(objc_msgSend as *const c_void);
            m(self.ptr, sel("hasUnifiedMemory"))
        }
    }

    /// Recommended maximum working-set size in bytes. Wraps
    /// `[MTLDevice recommendedMaxWorkingSetSize]`.
    #[must_use]
    pub fn recommended_max_working_set_size(&self) -> u64 {
        unsafe {
            type M = unsafe extern "C" fn(id, SEL) -> u64;
            let m: M = core::mem::transmute(objc_msgSend as *const c_void);
            m(self.ptr, sel("recommendedMaxWorkingSetSize"))
        }
    }

    /// True if this device supports the requested feature family —
    /// see [`gpu_family`]. Wraps `[MTLDevice supportsFamily:]`.
    #[must_use]
    pub fn supports_family(&self, family: i64) -> bool {
        unsafe {
            type M = unsafe extern "C" fn(id, SEL, i64) -> bool;
            let m: M = core::mem::transmute(objc_msgSend as *const c_void);
            m(self.ptr, sel("supportsFamily:"), family)
        }
    }

    /// Allocate a GPU-visible buffer of `length` bytes, zero-filled.
    /// `options` is an `MTLResourceOptions` bitmask (see
    /// [`resource_options`]). Wraps
    /// `[MTLDevice newBufferWithLength:options:]`.
    #[must_use]
    pub fn new_buffer(&self, length: usize, options: usize) -> Option<MetalBuffer> {
        unsafe {
            type M = unsafe extern "C" fn(id, SEL, usize, usize) -> id;
            let m: M = core::mem::transmute(objc_msgSend as *const c_void);
            let buf = m(self.ptr, sel("newBufferWithLength:options:"), length, options);
            if buf.is_null() {
                None
            } else {
                Some(MetalBuffer { ptr: buf })
            }
        }
    }
}

// ---- Buffer ----

/// Apple's `id<MTLBuffer>` — a GPU-visible byte buffer.
pub struct MetalBuffer {
    ptr: id,
}

unsafe impl Send for MetalBuffer {}
unsafe impl Sync for MetalBuffer {}

impl Drop for MetalBuffer {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { objc_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl MetalBuffer {
    /// Buffer length in bytes. Wraps `[MTLBuffer length]`.
    #[must_use]
    pub fn length(&self) -> usize {
        unsafe {
            type M = unsafe extern "C" fn(id, SEL) -> usize;
            let m: M = core::mem::transmute(objc_msgSend as *const c_void);
            m(self.ptr, sel("length"))
        }
    }

    /// Raw `void *` to the buffer's CPU-visible bytes. `None` for
    /// `MTLStorageMode::Private` (GPU-only) buffers. Wraps
    /// `[MTLBuffer contents]`.
    #[must_use]
    pub fn contents(&self) -> Option<*mut c_void> {
        let p = unsafe {
            type M = unsafe extern "C" fn(id, SEL) -> *mut c_void;
            let m: M = core::mem::transmute(objc_msgSend as *const c_void);
            m(self.ptr, sel("contents"))
        };
        if p.is_null() {
            None
        } else {
            Some(p)
        }
    }

    /// Copy `src` into this buffer at byte offset `0`. Returns the
    /// number of bytes actually written (`min(src.len(), length)`).
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

// ---- Texture descriptor builder ----

/// Configuration for `MTLDevice::new_texture`. Mirrors the most-used
/// `MTLTextureDescriptor` properties.
#[derive(Debug, Clone, Copy)]
pub struct TextureDescriptor {
    pub pixel_format: u32,
    pub width: usize,
    pub height: usize,
    pub mipmapped: bool,
    pub usage: usize,
    pub storage_mode: usize,
}

impl TextureDescriptor {
    /// Sensible defaults for a shader-read+write BGRA 2D texture in
    /// shared storage.
    #[must_use]
    pub const fn new_2d(width: usize, height: usize, pixel_format: u32) -> Self {
        Self {
            pixel_format,
            width,
            height,
            mipmapped: false,
            usage: MTL_TEXTURE_USAGE_SHADER_READ | MTL_TEXTURE_USAGE_SHADER_WRITE,
            storage_mode: MTL_STORAGE_MODE_SHARED,
        }
    }
}

impl MetalDevice {
    /// Allocate a fresh `MTLTexture` matching `descriptor`. Wraps
    /// `[MTLDevice newTextureWithDescriptor:]`.
    #[must_use]
    pub fn new_texture(&self, descriptor: TextureDescriptor) -> Option<MetalTexture> {
        unsafe {
            // Class method that returns an autoreleased MTLTextureDescriptor.
            let desc_class = class_named("MTLTextureDescriptor");
            let init: MsgSendTextureDescriptorInit =
                core::mem::transmute(objc_msgSend as *const c_void);
            let desc = init(
                desc_class.cast_mut(),
                sel("texture2DDescriptorWithPixelFormat:width:height:mipmapped:"),
                descriptor.pixel_format,
                descriptor.width,
                descriptor.height,
                descriptor.mipmapped,
            );
            if desc.is_null() {
                return None;
            }
            let set_usage: MsgSendSetUsage =
                core::mem::transmute(objc_msgSend as *const c_void);
            set_usage(desc, sel("setUsage:"), descriptor.usage);
            let set_storage: MsgSendSetStorage =
                core::mem::transmute(objc_msgSend as *const c_void);
            set_storage(desc, sel("setStorageMode:"), descriptor.storage_mode);

            type Mtex = unsafe extern "C" fn(id, SEL, id) -> id;
            let m: Mtex = core::mem::transmute(objc_msgSend as *const c_void);
            let tx = m(self.ptr, sel("newTextureWithDescriptor:"), desc);
            // desc is autoreleased — do NOT explicitly release.
            if tx.is_null() {
                None
            } else {
                Some(MetalTexture { ptr: tx })
            }
        }
    }
}

// ---- Texture ----

/// Apple's `id<MTLTexture>` — a GPU-resident 2D image.
pub struct MetalTexture {
    ptr: id,
}

unsafe impl Send for MetalTexture {}
unsafe impl Sync for MetalTexture {}

impl Drop for MetalTexture {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { objc_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl MetalTexture {
    /// Texture width in pixels.
    #[must_use]
    pub fn width(&self) -> usize {
        unsafe {
            let m: MsgSendUsize = core::mem::transmute(objc_msgSend as *const c_void);
            m(self.ptr, sel("width"))
        }
    }

    /// Texture height in pixels.
    #[must_use]
    pub fn height(&self) -> usize {
        unsafe {
            let m: MsgSendUsize = core::mem::transmute(objc_msgSend as *const c_void);
            m(self.ptr, sel("height"))
        }
    }

    /// Underlying `MTLPixelFormat` enum value — see [`pixel_format`].
    #[must_use]
    pub fn pixel_format(&self) -> u32 {
        unsafe {
            let m: MsgSendU32 = core::mem::transmute(objc_msgSend as *const c_void);
            m(self.ptr, sel("pixelFormat"))
        }
    }

    /// Raw `id<MTLTexture>` pointer.
    #[must_use]
    pub const fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }

    /// Wrap a raw `id<MTLTexture>` pointer. The pointer is taken
    /// without retain; ensure the caller has already balanced any
    /// retain/release on it.
    ///
    /// # Safety
    ///
    /// `ptr` must be a valid `id<MTLTexture>` whose ownership the
    /// caller is transferring to the returned [`MetalTexture`].
    #[must_use]
    pub const unsafe fn from_raw(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

// ---- IOSurface extension trait ----

#[cfg(feature = "iosurface")]
#[cfg_attr(docsrs, doc(cfg(feature = "iosurface")))]
mod iosurface_ext {
    use super::{
        class_named, objc_msgSend, objc_release, pixel_format, sel, MetalDevice, MetalTexture,
        MsgSend0, MsgSendSetStorage, MsgSendSetUsage, MsgSendTextureDescriptorInit,
        MsgSendTextureFromIOSurface, MTL_STORAGE_MODE_SHARED, MTL_TEXTURE_USAGE_SHADER_READ,
        MTL_TEXTURE_USAGE_SHADER_WRITE,
    };
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

            unsafe {
                let desc_class = class_named("MTLTextureDescriptor");
                let m_alloc: MsgSend0 = core::mem::transmute(objc_msgSend as *const c_void);
                let raw_desc = m_alloc(desc_class.cast_mut(), sel("alloc"));
                let init: MsgSendTextureDescriptorInit =
                    core::mem::transmute(objc_msgSend as *const c_void);
                let desc = init(
                    raw_desc,
                    sel("texture2DDescriptorWithPixelFormat:width:height:mipmapped:"),
                    format,
                    width,
                    height,
                    false,
                );
                if desc.is_null() {
                    return None;
                }
                let set_usage: MsgSendSetUsage =
                    core::mem::transmute(objc_msgSend as *const c_void);
                set_usage(
                    desc,
                    sel("setUsage:"),
                    MTL_TEXTURE_USAGE_SHADER_READ | MTL_TEXTURE_USAGE_SHADER_WRITE,
                );
                let set_storage: MsgSendSetStorage =
                    core::mem::transmute(objc_msgSend as *const c_void);
                set_storage(desc, sel("setStorageMode:"), MTL_STORAGE_MODE_SHARED);

                let m_tx: MsgSendTextureFromIOSurface =
                    core::mem::transmute(objc_msgSend as *const c_void);
                let tx = m_tx(
                    device.as_ptr(),
                    sel("newTextureWithDescriptor:iosurface:plane:"),
                    desc,
                    self.as_ptr().cast::<c_void>(),
                    plane_index,
                );
                objc_release(desc);
                if tx.is_null() {
                    None
                } else {
                    Some(MetalTexture::from_raw(tx))
                }
            }
        }
    }

    /// Map an `IOSurface` `FourCC` + plane index to the matching `MTLPixelFormat`.
    fn pixel_format_for_fourcc(fourcc: u32, plane_index: usize) -> Option<u32> {
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
