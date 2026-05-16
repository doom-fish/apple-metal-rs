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
    clippy::ptr_as_ptr
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
    pub const BGRA8UNORM: u32 = 80;
    pub const RGBA8UNORM: u32 = 70;
    pub const R8UNORM: u32 = 10;
    pub const RG8UNORM: u32 = 30;
    pub const BGRA10_XR: u32 = 552;
    pub const BGR10_XR: u32 = 554;
}

const MTL_TEXTURE_USAGE_SHADER_READ: usize = 0x01;
const MTL_TEXTURE_USAGE_SHADER_WRITE: usize = 0x02;
const MTL_STORAGE_MODE_SHARED: usize = 0;

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
