#![doc = include_str!("../README.md")]
//!
//! ---
//!
//! # API Documentation

#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::missing_const_for_fn)]

use core::ffi::c_void;
use core::ops::{Deref, DerefMut};
use core::ptr;
use std::sync::{Arc, Mutex, MutexGuard};

pub(crate) mod advanced;
pub(crate) mod argument;
pub(crate) mod command;
pub(crate) mod exhaustive;
/// Groups `Metal` framework constants for `ffi`.
pub mod ffi;
pub(crate) mod metalfx;
pub(crate) mod pipeline;
pub(crate) mod render;
pub(crate) mod state;
pub(crate) mod util;

/// Re-exports the `Metal` framework surface for this item.
pub use advanced::*;
/// Re-exports the `Metal` framework surface for this item.
pub use argument::*;
/// Re-exports the `Metal` framework surface for this item.
pub use command::*;
/// Re-exports the `Metal` framework surface for this item.
pub use exhaustive::*;
/// Re-exports the `Metal` framework surface for this item.
pub use metalfx::*;
/// Re-exports the `Metal` framework surface for this item.
pub use pipeline::*;
/// Re-exports the `Metal` framework surface for this item.
pub use render::*;
/// Re-exports the `Metal` framework surface for this item.
pub use state::*;

/// Common `MTLPixelFormat` constants.
pub mod pixel_format {
    /// Mirrors the `Metal` framework constant `A8UNORM`.
    pub const A8UNORM: usize = 1;
    /// Mirrors the `Metal` framework constant `R8UNORM`.
    pub const R8UNORM: usize = 10;
    /// Mirrors the `Metal` framework constant `R8SNORM`.
    pub const R8SNORM: usize = 12;
    /// Mirrors the `Metal` framework constant `R8UINT`.
    pub const R8UINT: usize = 13;
    /// Mirrors the `Metal` framework constant `R8SINT`.
    pub const R8SINT: usize = 14;
    /// Mirrors the `Metal` framework constant `R16UNORM`.
    pub const R16UNORM: usize = 20;
    /// Mirrors the `Metal` framework constant `R16SNORM`.
    pub const R16SNORM: usize = 22;
    /// Mirrors the `Metal` framework constant `R16UINT`.
    pub const R16UINT: usize = 23;
    /// Mirrors the `Metal` framework constant `R16SINT`.
    pub const R16SINT: usize = 24;
    /// Mirrors the `Metal` framework constant `R16FLOAT`.
    pub const R16FLOAT: usize = 25;
    /// Mirrors the `Metal` framework constant `RG8UNORM`.
    pub const RG8UNORM: usize = 30;
    /// Mirrors the `Metal` framework constant `RG8SNORM`.
    pub const RG8SNORM: usize = 32;
    /// Mirrors the `Metal` framework constant `RG8UINT`.
    pub const RG8UINT: usize = 33;
    /// Mirrors the `Metal` framework constant `RG8SINT`.
    pub const RG8SINT: usize = 34;
    /// Mirrors the `Metal` framework constant `RGBA8UNORM`.
    pub const RGBA8UNORM: usize = 70;
    /// Mirrors the `Metal` framework constant `RGBA8UNORM_SRGB`.
    pub const RGBA8UNORM_SRGB: usize = 71;
    /// Mirrors the `Metal` framework constant `RGBA8SNORM`.
    pub const RGBA8SNORM: usize = 72;
    /// Mirrors the `Metal` framework constant `RGBA8UINT`.
    pub const RGBA8UINT: usize = 73;
    /// Mirrors the `Metal` framework constant `RGBA8SINT`.
    pub const RGBA8SINT: usize = 74;
    /// Mirrors the `Metal` framework constant `BGRA8UNORM`.
    pub const BGRA8UNORM: usize = 80;
    /// Mirrors the `Metal` framework constant `BGRA8UNORM_SRGB`.
    pub const BGRA8UNORM_SRGB: usize = 81;
    /// Mirrors the `Metal` framework constant `R32FLOAT`.
    pub const R32FLOAT: usize = 55;
    /// Mirrors the `Metal` framework constant `RG16FLOAT`.
    pub const RG16FLOAT: usize = 65;
    /// Mirrors the `Metal` framework constant `RGBA16FLOAT`.
    pub const RGBA16FLOAT: usize = 115;
    /// Mirrors the `Metal` framework constant `RGBA32FLOAT`.
    pub const RGBA32FLOAT: usize = 125;
    /// Mirrors the `Metal` framework constant `DEPTH32FLOAT`.
    pub const DEPTH32FLOAT: usize = 252;
    /// Mirrors the `Metal` framework constant `STENCIL8`.
    pub const STENCIL8: usize = 253;
    /// Mirrors the `Metal` framework constant `BGRA10_XR`.
    pub const BGRA10_XR: usize = 552;
    /// Mirrors the `Metal` framework constant `BGR10_XR`.
    pub const BGR10_XR: usize = 554;
}

/// `MTLStorageMode` enum values — memory residency hints.
pub mod storage_mode {
    /// Mirrors the `Metal` framework constant `SHARED`.
    pub const SHARED: usize = 0;
    /// Mirrors the `Metal` framework constant `MANAGED`.
    pub const MANAGED: usize = 1;
    /// Mirrors the `Metal` framework constant `PRIVATE`.
    pub const PRIVATE: usize = 2;
    /// Mirrors the `Metal` framework constant `MEMORYLESS`.
    pub const MEMORYLESS: usize = 3;
}

/// `MTLCPUCacheMode` enum values.
pub mod cpu_cache_mode {
    /// Mirrors the `Metal` framework constant `DEFAULT_CACHE`.
    pub const DEFAULT_CACHE: usize = 0;
    /// Mirrors the `Metal` framework constant `WRITE_COMBINED`.
    pub const WRITE_COMBINED: usize = 1;
}

/// `MTLHazardTrackingMode` enum values.
pub mod hazard_tracking_mode {
    /// Mirrors the `Metal` framework constant `DEFAULT`.
    pub const DEFAULT: usize = 0;
    /// Mirrors the `Metal` framework constant `UNTRACKED`.
    pub const UNTRACKED: usize = 1;
    /// Mirrors the `Metal` framework constant `TRACKED`.
    pub const TRACKED: usize = 2;
}

/// `MTLResourceOptions` bitmask values.
pub mod resource_options {
    /// Mirrors the `Metal` framework constant `CPU_CACHE_MODE_DEFAULT`.
    pub const CPU_CACHE_MODE_DEFAULT: usize = 0;
    /// Mirrors the `Metal` framework constant `CPU_CACHE_MODE_WRITE_COMBINED`.
    pub const CPU_CACHE_MODE_WRITE_COMBINED: usize = 1;
    /// Mirrors the `Metal` framework constant `STORAGE_MODE_SHARED`.
    pub const STORAGE_MODE_SHARED: usize = 0;
    /// Mirrors the `Metal` framework constant `STORAGE_MODE_MANAGED`.
    pub const STORAGE_MODE_MANAGED: usize = 1 << 4;
    /// Mirrors the `Metal` framework constant `STORAGE_MODE_PRIVATE`.
    pub const STORAGE_MODE_PRIVATE: usize = 2 << 4;
    /// Mirrors the `Metal` framework constant `HAZARD_TRACKING_MODE_DEFAULT`.
    pub const HAZARD_TRACKING_MODE_DEFAULT: usize = 0;
    /// Mirrors the `Metal` framework constant `HAZARD_TRACKING_MODE_UNTRACKED`.
    pub const HAZARD_TRACKING_MODE_UNTRACKED: usize = 1 << 8;
    /// Mirrors the `Metal` framework constant `HAZARD_TRACKING_MODE_TRACKED`.
    pub const HAZARD_TRACKING_MODE_TRACKED: usize = 2 << 8;
}

/// `MTLTextureUsage` bitmask.
pub mod texture_usage {
    /// Mirrors the `Metal` framework constant `SHADER_READ`.
    pub const SHADER_READ: usize = 0x01;
    /// Mirrors the `Metal` framework constant `SHADER_WRITE`.
    pub const SHADER_WRITE: usize = 0x02;
    /// Mirrors the `Metal` framework constant `RENDER_TARGET`.
    pub const RENDER_TARGET: usize = 0x04;
}

/// `MTLGPUFamily` — feature-family identifiers.
pub mod gpu_family {
    /// Mirrors the `Metal` framework constant `APPLE1`.
    pub const APPLE1: i64 = 1001;
    /// Mirrors the `Metal` framework constant `APPLE2`.
    pub const APPLE2: i64 = 1002;
    /// Mirrors the `Metal` framework constant `APPLE3`.
    pub const APPLE3: i64 = 1003;
    /// Mirrors the `Metal` framework constant `APPLE4`.
    pub const APPLE4: i64 = 1004;
    /// Mirrors the `Metal` framework constant `APPLE5`.
    pub const APPLE5: i64 = 1005;
    /// Mirrors the `Metal` framework constant `APPLE6`.
    pub const APPLE6: i64 = 1006;
    /// Mirrors the `Metal` framework constant `APPLE7`.
    pub const APPLE7: i64 = 1007;
    /// Mirrors the `Metal` framework constant `APPLE8`.
    pub const APPLE8: i64 = 1008;
    /// Mirrors the `Metal` framework constant `APPLE9`.
    pub const APPLE9: i64 = 1009;
    /// Mirrors the `Metal` framework constant `MAC1`.
    pub const MAC1: i64 = 2001;
    /// Mirrors the `Metal` framework constant `MAC2`.
    pub const MAC2: i64 = 2002;
    /// Mirrors the `Metal` framework constant `COMMON1`.
    pub const COMMON1: i64 = 3001;
    /// Mirrors the `Metal` framework constant `COMMON2`.
    pub const COMMON2: i64 = 3002;
    /// Mirrors the `Metal` framework constant `COMMON3`.
    pub const COMMON3: i64 = 3003;
    /// Mirrors the `Metal` framework constant `METAL3`.
    pub const METAL3: i64 = 5001;
}

// ---- Device ----

/// Apple's `id<MTLDevice>` — handle to a Metal GPU.
pub struct MetalDevice {
    ptr: *mut c_void,
    drop_on_release: bool,
}

// SAFETY: `id<MTLDevice>` is thread-safe: creation, capability queries, and
// resource allocation all synchronize internally via ObjC ARC + Metal's own locks.
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
            Some(unsafe { Self::from_retained_ptr(p) })
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
        if length > isize::MAX as usize {
            return None;
        }
        let p = unsafe { ffi::am_device_new_buffer(self.ptr, length, options) };
        if p.is_null() {
            None
        } else {
            Some(unsafe { MetalBuffer::from_retained_ptr(p) })
        }
    }

    /// Allocate a fresh `MTLTexture` matching `descriptor`.
    #[must_use]
    pub fn new_texture(&self, descriptor: TextureDescriptor) -> Option<MetalTexture> {
        if [
            descriptor.pixel_format,
            descriptor.width,
            descriptor.height,
            descriptor.usage,
            descriptor.storage_mode,
        ]
        .into_iter()
        .any(|value| value > isize::MAX as usize)
        {
            return None;
        }
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

    /// Create a new `MTLCommandQueue` to schedule GPU work.
    #[must_use]
    pub fn new_command_queue(&self) -> Option<CommandQueue> {
        let p = unsafe { ffi::am_device_new_command_queue(self.ptr) };
        if p.is_null() {
            None
        } else {
            Some(CommandQueue { ptr: p })
        }
    }

    /// Compile a Metal Shading Language source string into a runtime
    /// `MTLLibrary`. On error, returns the localized Metal compiler
    /// diagnostic.
    ///
    /// # Errors
    ///
    /// Returns the Metal compiler's localized error string on failure.
    pub fn new_library_with_source(&self, source: &str) -> Result<MetalLibrary, String> {
        let csrc = std::ffi::CString::new(source).map_err(|e| e.to_string())?;
        let mut err_msg: *mut core::ffi::c_char = core::ptr::null_mut();
        let p = unsafe {
            ffi::am_device_new_library_with_source(self.ptr, csrc.as_ptr(), &mut err_msg)
        };
        if p.is_null() {
            let msg = if err_msg.is_null() {
                "MTLDevice.makeLibrary returned nil".to_string()
            } else {
                let s = unsafe { std::ffi::CStr::from_ptr(err_msg) }
                    .to_string_lossy()
                    .into_owned();
                unsafe { libc::free(err_msg.cast()) };
                s
            };
            Err(msg)
        } else {
            Ok(MetalLibrary { ptr: p })
        }
    }

    /// Compile a kernel into a `MTLComputePipelineState` ready for
    /// dispatch on a command buffer.
    ///
    /// # Errors
    ///
    /// Returns the Metal pipeline compiler's localized error string
    /// on failure.
    pub fn new_compute_pipeline_state(
        &self,
        function: &MetalFunction,
    ) -> Result<ComputePipelineState, String> {
        let mut err_msg: *mut core::ffi::c_char = core::ptr::null_mut();
        let p = unsafe {
            ffi::am_device_new_compute_pipeline_state(self.ptr, function.ptr, &mut err_msg)
        };
        if p.is_null() {
            let msg = if err_msg.is_null() {
                "MTLDevice.makeComputePipelineState returned nil".to_string()
            } else {
                let s = unsafe { std::ffi::CStr::from_ptr(err_msg) }
                    .to_string_lossy()
                    .into_owned();
                unsafe { libc::free(err_msg.cast()) };
                s
            };
            Err(msg)
        } else {
            Ok(ComputePipelineState { ptr: p })
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

// ---- Command queue + command buffer ----

/// Apple's `id<MTLCommandQueue>` — schedules GPU work.
pub struct CommandQueue {
    ptr: *mut c_void,
}

// SAFETY: `id<MTLCommandQueue>` is documented by Apple as thread-safe; multiple
// threads may independently create command buffers from the same queue.
unsafe impl Send for CommandQueue {}
unsafe impl Sync for CommandQueue {}

impl Drop for CommandQueue {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::am_command_queue_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl CommandQueue {
    /// Create a new command buffer for recording GPU commands.
    #[must_use]
    pub fn new_command_buffer(&self) -> Option<CommandBuffer> {
        let p = unsafe { ffi::am_command_queue_new_command_buffer(self.ptr) };
        if p.is_null() {
            None
        } else {
            Some(unsafe { CommandBuffer::from_retained_ptr(p) })
        }
    }

    /// Raw `id<MTLCommandQueue>` pointer.
    #[must_use]
    pub const fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

/// Apple's `id<MTLCommandBuffer>` — a recorded batch of GPU commands.
#[derive(Clone)]
pub struct CommandBuffer {
    pub(crate) inner: Arc<CommandBufferInner>,
}

pub(crate) struct CommandBufferInner {
    pub(crate) ptr: *mut c_void,
    pub(crate) state: Mutex<CommandBufferState>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CommandBufferPhase {
    Recording,
    Enqueued,
    Committed,
    Completed,
    Error,
}

pub(crate) struct CommandBufferState {
    pub(crate) phase: CommandBufferPhase,
    pub(crate) active_encoder: bool,
}

// SAFETY: all command-buffer state transitions and native mutations exposed by
// this crate are serialized by `state`.
unsafe impl Send for CommandBufferInner {}
unsafe impl Sync for CommandBufferInner {}

impl Drop for CommandBufferInner {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::am_command_buffer_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl CommandBuffer {
    /// Borrowed raw `id<MTLCommandBuffer>` pointer.
    ///
    /// The pointer remains valid only while at least one clone of this wrapper
    /// is alive. Calling lifecycle or encoding methods through the pointer can
    /// bypass this crate's state validation.
    #[must_use]
    pub fn as_ptr(&self) -> *mut c_void {
        self.inner.ptr
    }
}

// ---- Library + Function + ComputePipelineState ----

/// Apple's `id<MTLLibrary>` — compiled MSL source.
pub struct MetalLibrary {
    ptr: *mut c_void,
}

// SAFETY: `id<MTLLibrary>` is immutable after creation; all its methods are
// thread-safe per Apple documentation.
unsafe impl Send for MetalLibrary {}
unsafe impl Sync for MetalLibrary {}

impl Drop for MetalLibrary {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::am_library_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl MetalLibrary {
    /// Look up a kernel function by its source name.
    #[must_use]
    pub fn new_function(&self, name: &str) -> Option<MetalFunction> {
        let cname = std::ffi::CString::new(name).ok()?;
        let p = unsafe { ffi::am_library_new_function(self.ptr, cname.as_ptr()) };
        if p.is_null() {
            None
        } else {
            Some(MetalFunction { ptr: p })
        }
    }

    /// Raw `id<MTLLibrary>` pointer.
    #[must_use]
    pub const fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

/// Apple's `id<MTLFunction>` — a single compiled shader entry point.
pub struct MetalFunction {
    ptr: *mut c_void,
}

// SAFETY: `id<MTLFunction>` is immutable after creation and its handle is safe
// to share across threads.
unsafe impl Send for MetalFunction {}
unsafe impl Sync for MetalFunction {}

impl Drop for MetalFunction {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::am_function_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl MetalFunction {
    /// Raw `id<MTLFunction>` pointer.
    #[must_use]
    pub const fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

/// Apple's `id<MTLComputePipelineState>` — a compiled compute kernel.
pub struct ComputePipelineState {
    ptr: *mut c_void,
}

// SAFETY: `id<MTLComputePipelineState>` is immutable after creation and
// thread-safe per Apple documentation.
unsafe impl Send for ComputePipelineState {}
unsafe impl Sync for ComputePipelineState {}

impl Drop for ComputePipelineState {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::am_compute_pipeline_state_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl ComputePipelineState {
    /// Raw `id<MTLComputePipelineState>` pointer.
    #[must_use]
    pub const fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }

    pub(crate) const unsafe fn from_retained_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

// ---- Buffer ----

/// Errors returned by CPU access to a [`MetalBuffer`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MetalBufferAccessError {
    /// The buffer's storage mode does not expose CPU-addressable bytes.
    CpuInaccessibleStorage { storage_mode: usize },
    /// Metal did not provide a CPU mapping for an otherwise accessible buffer.
    MappingUnavailable,
    /// Another mapping panicked while holding the allocation's mapping lock.
    MappingLockPoisoned,
    /// The requested byte range is outside the allocation.
    RangeOutOfBounds {
        offset: usize,
        length: usize,
        buffer_length: usize,
    },
    /// The supplied range has its end before its start.
    InvalidRange,
    /// The operation requires managed storage.
    ManagedStorageRequired { storage_mode: usize },
}

impl core::fmt::Display for MetalBufferAccessError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::CpuInaccessibleStorage { storage_mode } => {
                write!(
                    formatter,
                    "storage mode {storage_mode} is not CPU-addressable"
                )
            }
            Self::MappingUnavailable => formatter.write_str("Metal did not provide a CPU mapping"),
            Self::MappingLockPoisoned => formatter.write_str("buffer mapping lock is poisoned"),
            Self::RangeOutOfBounds {
                offset,
                length,
                buffer_length,
            } => write!(
                formatter,
                "byte range {offset}..{} exceeds buffer length {buffer_length}",
                offset.saturating_add(*length)
            ),
            Self::InvalidRange => formatter.write_str("range end precedes range start"),
            Self::ManagedStorageRequired { storage_mode } => {
                write!(
                    formatter,
                    "managed storage required, got mode {storage_mode}"
                )
            }
        }
    }
}

impl std::error::Error for MetalBufferAccessError {}

/// Scoped read-only CPU mapping of a [`MetalBuffer`].
pub struct MetalBufferReadMapping<'a> {
    pointer: core::ptr::NonNull<u8>,
    length: usize,
    _mapping_lock: MutexGuard<'a, ()>,
}

impl Deref for MetalBufferReadMapping<'_> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        unsafe { core::slice::from_raw_parts(self.pointer.as_ptr(), self.length) }
    }
}

/// Scoped writable CPU mapping of a [`MetalBuffer`].
pub struct MetalBufferWriteMapping<'a> {
    buffer: &'a MetalBuffer,
    pointer: core::ptr::NonNull<u8>,
    length: usize,
    storage_mode: usize,
    _mapping_lock: MutexGuard<'a, ()>,
}

impl Deref for MetalBufferWriteMapping<'_> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        unsafe { core::slice::from_raw_parts(self.pointer.as_ptr(), self.length) }
    }
}

impl DerefMut for MetalBufferWriteMapping<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { core::slice::from_raw_parts_mut(self.pointer.as_ptr(), self.length) }
    }
}

impl Drop for MetalBufferWriteMapping<'_> {
    fn drop(&mut self) {
        if self.storage_mode == storage_mode::MANAGED {
            unsafe {
                ffi::am_buffer_did_modify_range(self.buffer.as_ptr(), 0, self.length);
            }
        }
    }
}

/// Apple's `id<MTLBuffer>` — a GPU-visible byte buffer.
#[derive(Clone)]
pub struct MetalBuffer {
    inner: Arc<MetalBufferInner>,
}

struct MetalBufferInner {
    ptr: *mut c_void,
    mapping_lock: Mutex<()>,
}

// SAFETY: immutable resource queries are thread-safe and scoped CPU mappings
// are serialized across clones by `mapping_lock`.
unsafe impl Send for MetalBufferInner {}
unsafe impl Sync for MetalBufferInner {}

impl Drop for MetalBufferInner {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::am_buffer_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

#[allow(clippy::missing_errors_doc)]
impl MetalBuffer {
    /// Buffer length in bytes.
    #[must_use]
    pub fn length(&self) -> usize {
        unsafe { ffi::am_buffer_length(self.as_ptr()) }
    }

    /// `MTLStorageMode` enum value.
    #[must_use]
    pub fn storage_mode(&self) -> usize {
        unsafe { ffi::am_buffer_storage_mode(self.as_ptr()) }
    }

    /// Whether this buffer's storage mode permits CPU mapping.
    #[must_use]
    pub fn is_cpu_accessible(&self) -> bool {
        matches!(
            self.storage_mode(),
            storage_mode::SHARED | storage_mode::MANAGED
        )
    }

    /// Create shared staging storage with the same length as this buffer.
    ///
    /// Use a blit encoder to copy between the staging allocation and private
    /// storage before mapping the staging buffer.
    #[must_use]
    pub fn new_staging_buffer(&self) -> Option<Self> {
        let pointer = unsafe { ffi::am_buffer_new_staging_buffer(self.as_ptr()) };
        if pointer.is_null() {
            None
        } else {
            Some(unsafe { Self::from_retained_ptr(pointer) })
        }
    }

    /// Map this buffer for scoped CPU reads.
    ///
    /// Clones of this Rust handle share a mapping lock. Independently-created
    /// native aliases are outside that lock.
    ///
    /// # Safety
    ///
    /// The caller must ensure that no GPU write or CPU/native alias mutation
    /// overlaps the mapping. This includes texture views backed by this buffer.
    /// For managed storage, GPU writes must first be made visible with a
    /// completed blit-encoder resource synchronization.
    pub unsafe fn map_read(&self) -> Result<MetalBufferReadMapping<'_>, MetalBufferAccessError> {
        let storage_mode = self.ensure_cpu_accessible()?;
        let mapping_lock = self.lock_mapping()?;
        let pointer = core::ptr::NonNull::new(ffi::am_buffer_contents(self.as_ptr()).cast::<u8>())
            .ok_or(MetalBufferAccessError::MappingUnavailable)?;
        let _ = storage_mode;
        Ok(MetalBufferReadMapping {
            pointer,
            length: self.length(),
            _mapping_lock: mapping_lock,
        })
    }

    /// Map this buffer for scoped CPU writes.
    ///
    /// Managed mappings notify Metal of the modified allocation when the guard
    /// is dropped. Clones of this Rust handle share a mapping lock, but native
    /// aliases created outside this wrapper do not.
    ///
    /// # Safety
    ///
    /// The caller must ensure that no GPU access or CPU/native alias access
    /// overlaps the mapping and must not submit GPU work using the buffer until
    /// the mapping guard is dropped. This includes texture views backed by this
    /// buffer.
    pub unsafe fn map_write(&self) -> Result<MetalBufferWriteMapping<'_>, MetalBufferAccessError> {
        let storage_mode = self.ensure_cpu_accessible()?;
        let mapping_lock = self.lock_mapping()?;
        let pointer = core::ptr::NonNull::new(ffi::am_buffer_contents(self.as_ptr()).cast::<u8>())
            .ok_or(MetalBufferAccessError::MappingUnavailable)?;
        Ok(MetalBufferWriteMapping {
            buffer: self,
            pointer,
            length: self.length(),
            storage_mode,
            _mapping_lock: mapping_lock,
        })
    }

    /// Copy `src` into a CPU-visible byte range.
    ///
    /// # Safety
    ///
    /// The caller must uphold the same GPU exclusion requirements as
    /// [`Self::map_write`].
    pub unsafe fn write_bytes(
        &self,
        offset: usize,
        src: &[u8],
    ) -> Result<(), MetalBufferAccessError> {
        let end = self.checked_range_end(offset, src.len())?;
        let mut mapping = self.map_write()?;
        mapping[offset..end].copy_from_slice(src);
        drop(mapping);
        Ok(())
    }

    /// Copy a CPU-visible byte range into `destination`.
    ///
    /// # Safety
    ///
    /// The caller must uphold the same GPU exclusion and managed-storage
    /// synchronization requirements as [`Self::map_read`].
    pub unsafe fn read_bytes(
        &self,
        offset: usize,
        destination: &mut [u8],
    ) -> Result<(), MetalBufferAccessError> {
        let end = self.checked_range_end(offset, destination.len())?;
        let mapping = self.map_read()?;
        destination.copy_from_slice(&mapping[offset..end]);
        drop(mapping);
        Ok(())
    }

    /// Borrowed raw `id<MTLBuffer>` pointer.
    ///
    /// The pointer remains valid only while at least one clone of this wrapper
    /// is alive. CPU access through the raw object bypasses the mapping lock.
    #[must_use]
    pub fn as_ptr(&self) -> *mut c_void {
        self.inner.ptr
    }

    fn ensure_cpu_accessible(&self) -> Result<usize, MetalBufferAccessError> {
        let storage_mode = self.storage_mode();
        if matches!(storage_mode, storage_mode::SHARED | storage_mode::MANAGED) {
            Ok(storage_mode)
        } else {
            Err(MetalBufferAccessError::CpuInaccessibleStorage { storage_mode })
        }
    }

    pub(crate) fn checked_range_end(
        &self,
        offset: usize,
        length: usize,
    ) -> Result<usize, MetalBufferAccessError> {
        let end =
            offset
                .checked_add(length)
                .ok_or_else(|| MetalBufferAccessError::RangeOutOfBounds {
                    offset,
                    length,
                    buffer_length: self.length(),
                })?;
        let buffer_length = self.length();
        if end > buffer_length {
            Err(MetalBufferAccessError::RangeOutOfBounds {
                offset,
                length,
                buffer_length,
            })
        } else {
            Ok(end)
        }
    }

    pub(crate) fn lock_mapping(&self) -> Result<MutexGuard<'_, ()>, MetalBufferAccessError> {
        self.inner
            .mapping_lock
            .lock()
            .map_err(|_| MetalBufferAccessError::MappingLockPoisoned)
    }
}

// ---- Texture descriptor + texture ----

/// Configuration for `MetalDevice::new_texture`.
#[derive(Debug, Clone, Copy)]
pub struct TextureDescriptor {
    /// Mirrors the `Metal` framework property for `pixel_format`.
    pub pixel_format: usize,
    /// Mirrors the `Metal` framework property for `width`.
    pub width: usize,
    /// Mirrors the `Metal` framework property for `height`.
    pub height: usize,
    /// Mirrors the `Metal` framework property for `mipmapped`.
    pub mipmapped: bool,
    /// Mirrors the `Metal` framework property for `usage`.
    pub usage: usize,
    /// Mirrors the `Metal` framework property for `storage_mode`.
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

// SAFETY: `id<MTLTexture>` is a GPU resource handle.  ObjC ARC operations are
// atomic; descriptor queries are read-only and thread-safe.
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

    /// Underlying `MTLTextureType` enum value — see [`texture_type`].
    #[must_use]
    pub fn texture_type(&self) -> usize {
        unsafe { ffi::am_texture_type(self.ptr) }
    }

    /// Raw `id<MTLTexture>` pointer.
    #[must_use]
    pub const fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }

    /// Wrap a raw, **+1-retained** `id<MTLTexture>` pointer. Ownership is
    /// transferred to the returned wrapper and released once on drop (no extra
    /// retain is taken here).
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

impl MetalDevice {
    pub(crate) const unsafe fn from_retained_ptr(ptr: *mut c_void) -> Self {
        Self {
            ptr,
            drop_on_release: true,
        }
    }
}

impl CommandQueue {
    pub(crate) const unsafe fn from_retained_ptr(ptr: *mut c_void) -> Self {
        Self { ptr }
    }
}

impl CommandBuffer {
    pub(crate) unsafe fn from_retained_ptr(ptr: *mut c_void) -> Self {
        Self {
            inner: Arc::new(CommandBufferInner {
                ptr,
                state: Mutex::new(CommandBufferState {
                    phase: CommandBufferPhase::Recording,
                    active_encoder: false,
                }),
            }),
        }
    }
}

impl MetalBuffer {
    pub(crate) unsafe fn from_retained_ptr(ptr: *mut c_void) -> Self {
        Self {
            inner: Arc::new(MetalBufferInner {
                ptr,
                mapping_lock: Mutex::new(()),
            }),
        }
    }
}

// ---- IOSurface extension ----

#[cfg(feature = "iosurface")]
#[cfg_attr(docsrs, doc(cfg(feature = "iosurface")))]
mod iosurface_ext {
    use super::{ffi, pixel_format, MetalDevice, MetalTexture};
    use apple_cf::iosurface::IOSurface;
    use core::ffi::c_void;

    /// Errors returned while creating an `IOSurface`-backed texture.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum IOSurfaceMetalError {
        /// The requested plane does not exist for this surface.
        InvalidPlane {
            plane_index: usize,
            plane_count: usize,
        },
        /// The surface format and plane do not map to a supported Metal format.
        UnsupportedPixelFormat { fourcc: u32, plane_index: usize },
        /// The selected plane has zero dimensions or row stride.
        EmptyPlane {
            plane_index: usize,
            width: usize,
            height: usize,
            bytes_per_row: usize,
        },
        /// The plane row stride cannot contain its selected Metal format.
        IncompatiblePlaneLayout {
            plane_index: usize,
            bytes_per_row: usize,
            minimum_bytes_per_row: usize,
        },
        /// Plane metadata cannot be represented by the native bridge.
        IntegerOutOfRange { field: &'static str, value: usize },
        /// Metal rejected the validated `IOSurface` texture descriptor.
        NativeCreationFailed,
    }

    impl core::fmt::Display for IOSurfaceMetalError {
        fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Self::InvalidPlane {
                    plane_index,
                    plane_count,
                } => write!(
                    formatter,
                    "IOSurface plane {plane_index} is outside the available count {plane_count}"
                ),
                Self::UnsupportedPixelFormat {
                    fourcc,
                    plane_index,
                } => write!(
                    formatter,
                    "IOSurface format {fourcc:#010x} plane {plane_index} is unsupported"
                ),
                Self::EmptyPlane {
                    plane_index,
                    width,
                    height,
                    bytes_per_row,
                } => write!(
                    formatter,
                    "IOSurface plane {plane_index} has invalid layout {width}x{height}, row {bytes_per_row}"
                ),
                Self::IncompatiblePlaneLayout {
                    plane_index,
                    bytes_per_row,
                    minimum_bytes_per_row,
                } => write!(
                    formatter,
                    "IOSurface plane {plane_index} row {bytes_per_row} is shorter than {minimum_bytes_per_row}"
                ),
                Self::IntegerOutOfRange { field, value } => {
                    write!(formatter, "{field} value {value} exceeds native Int")
                }
                Self::NativeCreationFailed => {
                    formatter.write_str("Metal could not create the IOSurface-backed texture")
                }
            }
        }
    }

    impl std::error::Error for IOSurfaceMetalError {}

    #[derive(Clone, Copy)]
    struct PlaneTextureFormat {
        pixel_format: usize,
        bytes_per_pixel: usize,
    }

    /// Add Metal interop methods to [`IOSurface`].
    pub trait IOSurfaceMetalExt {
        /// Wrap the given plane of this `IOSurface` as a zero-copy
        /// [`MetalTexture`] on the given device.
        ///
        /// The native `IOSurfaceRef` pointer is borrowed only for the duration
        /// of this call. The returned Metal texture retains its backing surface.
        ///
        /// # Errors
        ///
        /// Returns plane, format, row-layout, integer-conversion, or native
        /// texture-creation failures.
        fn create_metal_texture(
            &self,
            device: &MetalDevice,
            plane_index: usize,
        ) -> Result<MetalTexture, IOSurfaceMetalError>;
    }

    impl IOSurfaceMetalExt for IOSurface {
        fn create_metal_texture(
            &self,
            device: &MetalDevice,
            plane_index: usize,
        ) -> Result<MetalTexture, IOSurfaceMetalError> {
            let plane_count = self.plane_count();
            let (width, height, bytes_per_row) = if plane_count == 0 {
                if plane_index != 0 {
                    return Err(IOSurfaceMetalError::InvalidPlane {
                        plane_index,
                        plane_count: 1,
                    });
                }
                (self.width(), self.height(), self.bytes_per_row())
            } else {
                if plane_index >= plane_count {
                    return Err(IOSurfaceMetalError::InvalidPlane {
                        plane_index,
                        plane_count,
                    });
                }
                (
                    self.width_of_plane(plane_index),
                    self.height_of_plane(plane_index),
                    self.bytes_per_row_of_plane(plane_index),
                )
            };
            if width == 0 || height == 0 || bytes_per_row == 0 {
                return Err(IOSurfaceMetalError::EmptyPlane {
                    plane_index,
                    width,
                    height,
                    bytes_per_row,
                });
            }
            let format = pixel_format_for_fourcc(self.pixel_format(), plane_index)?;
            let minimum_bytes_per_row = width.checked_mul(format.bytes_per_pixel).ok_or(
                IOSurfaceMetalError::IntegerOutOfRange {
                    field: "minimum plane row bytes",
                    value: usize::MAX,
                },
            )?;
            if bytes_per_row < minimum_bytes_per_row || bytes_per_row % format.bytes_per_pixel != 0
            {
                return Err(IOSurfaceMetalError::IncompatiblePlaneLayout {
                    plane_index,
                    bytes_per_row,
                    minimum_bytes_per_row,
                });
            }
            for (field, value) in [
                ("plane index", plane_index),
                ("plane width", width),
                ("plane height", height),
                ("pixel format", format.pixel_format),
            ] {
                if value > isize::MAX as usize {
                    return Err(IOSurfaceMetalError::IntegerOutOfRange { field, value });
                }
            }
            let p = unsafe {
                ffi::am_device_new_texture_from_iosurface(
                    device.as_ptr(),
                    self.as_ptr().cast::<c_void>(),
                    plane_index,
                    format.pixel_format,
                    width,
                    height,
                )
            };
            if p.is_null() {
                Err(IOSurfaceMetalError::NativeCreationFailed)
            } else {
                Ok(unsafe { MetalTexture::from_raw(p) })
            }
        }
    }

    fn pixel_format_for_fourcc(
        fourcc: u32,
        plane_index: usize,
    ) -> Result<PlaneTextureFormat, IOSurfaceMetalError> {
        const BGRA: u32 = u32::from_be_bytes(*b"BGRA");
        const YUV420V: u32 = u32::from_be_bytes(*b"420v");
        const YUV420F: u32 = u32::from_be_bytes(*b"420f");

        match (fourcc, plane_index) {
            (BGRA, 0) => Ok(PlaneTextureFormat {
                pixel_format: pixel_format::BGRA8UNORM,
                bytes_per_pixel: 4,
            }),
            (YUV420V | YUV420F, 0) => Ok(PlaneTextureFormat {
                pixel_format: pixel_format::R8UNORM,
                bytes_per_pixel: 1,
            }),
            (YUV420V | YUV420F, 1) => Ok(PlaneTextureFormat {
                pixel_format: pixel_format::RG8UNORM,
                bytes_per_pixel: 2,
            }),
            _ => Err(IOSurfaceMetalError::UnsupportedPixelFormat {
                fourcc,
                plane_index,
            }),
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn packed_ten_bit_surface_is_not_guessed() {
            let fourcc = u32::from_be_bytes(*b"l10r");
            assert!(matches!(
                pixel_format_for_fourcc(fourcc, 0),
                Err(IOSurfaceMetalError::UnsupportedPixelFormat { .. })
            ));
        }

        #[test]
        fn odd_chroma_plane_uses_two_bytes_per_actual_plane_pixel() {
            let fourcc = u32::from_be_bytes(*b"420v");
            let format = pixel_format_for_fourcc(fourcc, 1).expect("chroma format");
            assert_eq!(format.pixel_format, pixel_format::RG8UNORM);
            assert_eq!(3 * format.bytes_per_pixel, 6);
        }
    }
}

/// Re-exports the `Metal` framework surface for this item.
#[cfg(feature = "iosurface")]
pub use iosurface_ext::{IOSurfaceMetalError, IOSurfaceMetalExt};

/// True if `fourcc` identifies a YCbCr biplanar (`Y` + `CbCr`) format.
#[must_use]
pub const fn is_ycbcr_biplanar(fourcc: u32) -> bool {
    const YUV420V: u32 = u32::from_be_bytes(*b"420v");
    const YUV420F: u32 = u32::from_be_bytes(*b"420f");
    matches!(fourcc, YUV420V | YUV420F)
}
