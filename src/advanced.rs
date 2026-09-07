use crate::{
    ffi,
    util::{c_string, take_optional_string, take_string},
    ArgumentEncoder, CommandQueue, ComputePipelineState, MetalBuffer, MetalBufferAccessError,
    MetalDevice, MetalFunction, MetalTexture, TextureDescriptor,
};
use core::ffi::c_void;
use core::ops::Range;
use std::path::Path;

macro_rules! opaque_handle {
    ($(#[$meta:meta])* pub struct $name:ident;) => {
        $(#[$meta])*
/// Mirrors the `Metal` framework counterpart for this type.
        pub struct $name {
            ptr: *mut c_void,
        }

        // SAFETY: Metal ObjC objects use atomic reference counting and are safe
        // to move across threads.  All `&self` methods on these types either read
        // immutable state or call ObjC methods documented as thread-safe by Apple.
        unsafe impl Send for $name {}
        unsafe impl Sync for $name {}

        impl Drop for $name {
            fn drop(&mut self) {
                if !self.ptr.is_null() {
                    // SAFETY: `ptr` is a non-null, +1-retained ObjC handle
                    // exclusively owned by this struct.  Setting it to null
                    // immediately prevents any subsequent release.
                    unsafe { ffi::am_object_release(self.ptr) };
                    self.ptr = core::ptr::null_mut();
                }
            }
        }

        impl $name {
/// Mirrors the `Metal` framework constant `fn`.
            #[must_use]
            pub const fn as_ptr(&self) -> *mut c_void {
                self.ptr
            }

            fn wrap(ptr: *mut c_void) -> Option<Self> {
                if ptr.is_null() {
                    None
                } else {
                    Some(Self { ptr })
                }
            }
        }
    };
}

/// `MTLIndirectCommandType` bit values.
pub mod indirect_command_type {
    /// Mirrors the `Metal` framework constant `DRAW`.
    pub const DRAW: usize = 1 << 0;
    /// Mirrors the `Metal` framework constant `DRAW_INDEXED`.
    pub const DRAW_INDEXED: usize = 1 << 1;
    /// Mirrors the `Metal` framework constant `CONCURRENT_DISPATCH`.
    pub const CONCURRENT_DISPATCH: usize = 1 << 5;
    /// Mirrors the `Metal` framework constant `CONCURRENT_DISPATCH_THREADS`.
    pub const CONCURRENT_DISPATCH_THREADS: usize = 1 << 6;
}

/// `MTLCounterSamplingPoint` enum values.
pub mod counter_sampling_point {
    /// Mirrors the `Metal` framework constant `AT_STAGE_BOUNDARY`.
    pub const AT_STAGE_BOUNDARY: usize = 0;
    /// Mirrors the `Metal` framework constant `AT_DRAW_BOUNDARY`.
    pub const AT_DRAW_BOUNDARY: usize = 1;
    /// Mirrors the `Metal` framework constant `AT_DISPATCH_BOUNDARY`.
    pub const AT_DISPATCH_BOUNDARY: usize = 2;
    /// Mirrors the `Metal` framework constant `AT_TILE_DISPATCH_BOUNDARY`.
    pub const AT_TILE_DISPATCH_BOUNDARY: usize = 3;
    /// Mirrors the `Metal` framework constant `AT_BLIT_BOUNDARY`.
    pub const AT_BLIT_BOUNDARY: usize = 4;
}

/// `MTLLogLevel` enum values.
pub mod log_level {
    /// Mirrors the `Metal` framework constant `UNDEFINED`.
    pub const UNDEFINED: usize = 0;
    /// Mirrors the `Metal` framework constant `DEBUG`.
    pub const DEBUG: usize = 1;
    /// Mirrors the `Metal` framework constant `INFO`.
    pub const INFO: usize = 2;
    /// Mirrors the `Metal` framework constant `NOTICE`.
    pub const NOTICE: usize = 3;
    /// Mirrors the `Metal` framework constant `ERROR`.
    pub const ERROR: usize = 4;
    /// Mirrors the `Metal` framework constant `FAULT`.
    pub const FAULT: usize = 5;
}

/// `MTLPurgeableState` enum values.
pub mod purgeable_state {
    /// Mirrors the `Metal` framework constant `KEEP_CURRENT`.
    pub const KEEP_CURRENT: usize = 1;
    /// Mirrors the `Metal` framework constant `NON_VOLATILE`.
    pub const NON_VOLATILE: usize = 2;
    /// Mirrors the `Metal` framework constant `VOLATILE`.
    pub const VOLATILE: usize = 3;
    /// Mirrors the `Metal` framework constant `EMPTY`.
    pub const EMPTY: usize = 4;
}

/// `MTLCaptureDestination` enum values.
pub mod capture_destination {
    /// Mirrors the `Metal` framework constant `DEVELOPER_TOOLS`.
    pub const DEVELOPER_TOOLS: usize = 1;
    /// Mirrors the `Metal` framework constant `GPU_TRACE_DOCUMENT`.
    pub const GPU_TRACE_DOCUMENT: usize = 2;
}

/// `MTLIntersectionFunctionSignature` bit values.
pub mod intersection_function_signature {
    /// Mirrors the `Metal` framework constant `NONE`.
    pub const NONE: usize = 0;
    /// Mirrors the `Metal` framework constant `INSTANCING`.
    pub const INSTANCING: usize = 1 << 0;
    /// Mirrors the `Metal` framework constant `TRIANGLE_DATA`.
    pub const TRIANGLE_DATA: usize = 1 << 1;
    /// Mirrors the `Metal` framework constant `WORLD_SPACE_DATA`.
    pub const WORLD_SPACE_DATA: usize = 1 << 2;
}

opaque_handle!(
    /// Apple's `id<MTLHeap>` — shared GPU allocation arena.
    pub struct Heap;
);
opaque_handle!(
    /// Apple's `id<MTLEvent>` backed by `MTLSharedEvent`.
    pub struct Event;
);
opaque_handle!(
    /// Apple's `id<MTLFence>` — intra-queue synchronization primitive.
    pub struct Fence;
);
opaque_handle!(
    /// Apple's `id<MTLDynamicLibrary>` — device-linked Metal code bundle.
    pub struct DynamicLibrary;
);
opaque_handle!(
    /// Apple's `id<MTLBinaryArchive>` — persistent pipeline cache.
    pub struct BinaryArchive;
);
opaque_handle!(
    /// Apple's `id<MTLIndirectCommandBuffer>` — stores GPU-executable commands.
    pub struct IndirectCommandBuffer;
);
opaque_handle!(
    /// Apple's `id<MTLAccelerationStructure>` — storage for ray tracing data.
    pub struct AccelerationStructure;
);
opaque_handle!(
    /// Apple's `id<MTLIntersectionFunctionTable>` — table of ray intersection functions.
    pub struct IntersectionFunctionTable;
);
opaque_handle!(
    /// Apple's `id<MTLVisibleFunctionTable>` — table of callable function handles.
    pub struct VisibleFunctionTable;
);
opaque_handle!(
    /// Apple's `id<MTLCounterSampleBuffer>` — storage for GPU counter samples.
    pub struct CounterSampleBuffer;
);
opaque_handle!(
    /// Apple's `id<MTLLogState>` — shader logging configuration.
    pub struct LogState;
);
opaque_handle!(
    /// Apple's `id<MTLResidencySet>` — explicit residency tracking set.
    pub struct ResidencySet;
);
opaque_handle!(
    /// Apple's `MTLCaptureManager` singleton.
    pub struct CaptureManager;
);
opaque_handle!(
    /// Apple's `id<MTLCaptureScope>` — named capture region.
    pub struct CaptureScope;
);

impl MetalDevice {
    /// Human-readable device name.
    #[must_use]
    pub fn name(&self) -> String {
        unsafe { take_string(ffi::am_device_name(self.as_ptr())) }
    }

    /// Global `IORegistry` identifier for the device.
    #[must_use]
    pub fn registry_id(&self) -> u64 {
        unsafe { ffi::am_device_registry_id(self.as_ptr()) }
    }

    /// Whether this device supports Metal dynamic libraries.
    #[must_use]
    pub fn supports_dynamic_libraries(&self) -> bool {
        unsafe { ffi::am_device_supports_dynamic_libraries(self.as_ptr()) }
    }

    /// Whether this device supports render-stage dynamic libraries.
    #[must_use]
    pub fn supports_render_dynamic_libraries(&self) -> bool {
        unsafe { ffi::am_device_supports_render_dynamic_libraries(self.as_ptr()) }
    }

    /// Whether this device supports compute ray tracing.
    #[must_use]
    pub fn supports_raytracing(&self) -> bool {
        unsafe { ffi::am_device_supports_raytracing(self.as_ptr()) }
    }

    /// Query support for a hardware counter sampling point.
    #[must_use]
    pub fn supports_counter_sampling(&self, sampling_point: usize) -> bool {
        unsafe { ffi::am_device_supports_counter_sampling(self.as_ptr(), sampling_point) }
    }

    /// Return the names of all counter sets exposed by this device.
    #[must_use]
    pub fn counter_set_names(&self) -> Vec<String> {
        let count = unsafe { ffi::am_device_counter_set_count(self.as_ptr()) };
        (0..count)
            .filter_map(|index| unsafe {
                take_optional_string(ffi::am_device_counter_set_name_at(self.as_ptr(), index))
            })
            .collect()
    }

    /// Create a command queue with an explicit maximum in-flight command-buffer count.
    #[must_use]
    pub fn new_command_queue_with_max_command_buffer_count(
        &self,
        max_command_buffer_count: usize,
    ) -> Option<CommandQueue> {
        let ptr = unsafe {
            ffi::am_device_new_command_queue_with_max_command_buffer_count(
                self.as_ptr(),
                max_command_buffer_count,
            )
        };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { CommandQueue::from_retained_ptr(ptr) })
        }
    }

    /// Create a command queue that uses `log_state` for shader logging.
    #[must_use]
    pub fn new_command_queue_with_log_state(
        &self,
        max_command_buffer_count: usize,
        log_state: &LogState,
    ) -> Option<CommandQueue> {
        let ptr = unsafe {
            ffi::am_device_new_command_queue_with_log_state(
                self.as_ptr(),
                max_command_buffer_count,
                log_state.as_ptr(),
            )
        };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { CommandQueue::from_retained_ptr(ptr) })
        }
    }

    /// Create a heap with the requested size and storage mode.
    #[must_use]
    pub fn new_heap(&self, size: usize, storage_mode: usize) -> Option<Heap> {
        Heap::wrap(unsafe { ffi::am_device_new_heap(self.as_ptr(), size, storage_mode) })
    }

    /// Create a new fence.
    #[must_use]
    pub fn new_fence(&self) -> Option<Fence> {
        Fence::wrap(unsafe { ffi::am_device_new_fence(self.as_ptr()) })
    }

    /// Create a new shared event.
    #[must_use]
    pub fn new_shared_event(&self) -> Option<Event> {
        Event::wrap(unsafe { ffi::am_device_new_shared_event(self.as_ptr()) })
    }

    /// Compile `source` as a Metal dynamic library with the given `install_name`.
    ///
    /// # Errors
    ///
    /// Returns Metal's localized compiler or linker error on failure.
    pub fn new_dynamic_library_with_source(
        &self,
        source: &str,
        install_name: &str,
    ) -> Result<DynamicLibrary, String> {
        let source = c_string(source)?;
        let install_name = c_string(install_name)?;
        let mut err: *mut core::ffi::c_char = core::ptr::null_mut();
        let ptr = unsafe {
            ffi::am_device_new_dynamic_library_with_source(
                self.as_ptr(),
                source.as_ptr(),
                install_name.as_ptr(),
                &mut err,
            )
        };
        DynamicLibrary::wrap(ptr).ok_or_else(|| unsafe {
            take_optional_string(err)
                .unwrap_or_else(|| "MTLDevice.makeDynamicLibrary(source:) returned nil".to_string())
        })
    }

    /// Load a serialized Metal dynamic library from `path`.
    ///
    /// # Errors
    ///
    /// Returns Metal's localized file or linker error on failure.
    pub fn load_dynamic_library(&self, path: &Path) -> Result<DynamicLibrary, String> {
        let path = c_string(path.to_string_lossy().as_ref())?;
        let mut err: *mut core::ffi::c_char = core::ptr::null_mut();
        let ptr = unsafe {
            ffi::am_device_new_dynamic_library_with_url(self.as_ptr(), path.as_ptr(), &mut err)
        };
        DynamicLibrary::wrap(ptr).ok_or_else(|| unsafe {
            take_optional_string(err)
                .unwrap_or_else(|| "MTLDevice.makeDynamicLibrary(URL:) returned nil".to_string())
        })
    }

    /// Create a binary archive, optionally loading it from `path` first.
    ///
    /// # Errors
    ///
    /// Returns Metal's localized archive creation error on failure.
    pub fn new_binary_archive(&self, path: Option<&Path>) -> Result<BinaryArchive, String> {
        let owned_path = path
            .map(|path| c_string(path.to_string_lossy().as_ref()))
            .transpose()?;
        let raw_path = owned_path
            .as_ref()
            .map_or(core::ptr::null(), |path| path.as_c_str().as_ptr());
        let mut err: *mut core::ffi::c_char = core::ptr::null_mut();
        let ptr = unsafe { ffi::am_device_new_binary_archive(self.as_ptr(), raw_path, &mut err) };
        BinaryArchive::wrap(ptr).ok_or_else(|| unsafe {
            take_optional_string(err)
                .unwrap_or_else(|| "MTLDevice.makeBinaryArchive returned nil".to_string())
        })
    }

    /// Create a new indirect command buffer.
    #[must_use]
    pub fn new_indirect_command_buffer(
        &self,
        command_types: usize,
        max_command_count: usize,
        max_vertex_buffer_bind_count: usize,
        max_fragment_buffer_bind_count: usize,
        max_kernel_buffer_bind_count: usize,
        options: usize,
    ) -> Option<IndirectCommandBuffer> {
        IndirectCommandBuffer::wrap(unsafe {
            ffi::am_device_new_indirect_command_buffer(
                self.as_ptr(),
                command_types,
                max_command_count,
                max_vertex_buffer_bind_count,
                max_fragment_buffer_bind_count,
                max_kernel_buffer_bind_count,
                options,
            )
        })
    }

    /// Allocate storage for a ray-tracing acceleration structure.
    #[must_use]
    pub fn new_acceleration_structure_with_size(
        &self,
        size: usize,
    ) -> Option<AccelerationStructure> {
        AccelerationStructure::wrap(unsafe {
            ffi::am_device_new_acceleration_structure_with_size(self.as_ptr(), size)
        })
    }

    /// Create a counter sample buffer for the named counter set.
    ///
    /// # Errors
    ///
    /// Returns Metal's localized counter-sample-buffer error on failure.
    pub fn new_counter_sample_buffer(
        &self,
        counter_set_name: &str,
        sample_count: usize,
        storage_mode: usize,
        label: Option<&str>,
    ) -> Result<CounterSampleBuffer, String> {
        let counter_set_name = c_string(counter_set_name)?;
        let label = label.map(c_string).transpose()?;
        let raw_label = label
            .as_ref()
            .map_or(core::ptr::null(), |label| label.as_c_str().as_ptr());
        let mut err: *mut core::ffi::c_char = core::ptr::null_mut();
        let ptr = unsafe {
            ffi::am_device_new_counter_sample_buffer(
                self.as_ptr(),
                counter_set_name.as_ptr(),
                sample_count,
                storage_mode,
                raw_label,
                &mut err,
            )
        };
        CounterSampleBuffer::wrap(ptr).ok_or_else(|| unsafe {
            take_optional_string(err)
                .unwrap_or_else(|| "MTLDevice.makeCounterSampleBuffer returned nil".to_string())
        })
    }

    /// Create a shader log state.
    ///
    /// # Errors
    ///
    /// Returns Metal's localized log-state creation error on failure.
    pub fn new_log_state(&self, level: usize, buffer_size: isize) -> Result<LogState, String> {
        let mut err: *mut core::ffi::c_char = core::ptr::null_mut();
        let ptr =
            unsafe { ffi::am_device_new_log_state(self.as_ptr(), level, buffer_size, &mut err) };
        LogState::wrap(ptr).ok_or_else(|| unsafe {
            take_optional_string(err)
                .unwrap_or_else(|| "MTLDevice.makeLogState returned nil".to_string())
        })
    }

    /// Create a residency set.
    ///
    /// # Errors
    ///
    /// Returns Metal's localized residency-set creation error on failure.
    pub fn new_residency_set(
        &self,
        label: Option<&str>,
        initial_capacity: usize,
    ) -> Result<ResidencySet, String> {
        let label = label.map(c_string).transpose()?;
        let raw_label = label
            .as_ref()
            .map_or(core::ptr::null(), |label| label.as_c_str().as_ptr());
        let mut err: *mut core::ffi::c_char = core::ptr::null_mut();
        let ptr = unsafe {
            ffi::am_device_new_residency_set(self.as_ptr(), raw_label, initial_capacity, &mut err)
        };
        ResidencySet::wrap(ptr).ok_or_else(|| unsafe {
            take_optional_string(err)
                .unwrap_or_else(|| "MTLDevice.makeResidencySet returned nil".to_string())
        })
    }
}

impl CommandQueue {
    /// Add `residency_set` to the queue-wide residency list.
    pub fn add_residency_set(&self, residency_set: &ResidencySet) {
        unsafe { ffi::am_command_queue_add_residency_set(self.as_ptr(), residency_set.as_ptr()) };
    }

    /// Remove `residency_set` from the queue-wide residency list.
    pub fn remove_residency_set(&self, residency_set: &ResidencySet) {
        unsafe {
            ffi::am_command_queue_remove_residency_set(self.as_ptr(), residency_set.as_ptr());
        };
    }
}

impl MetalBuffer {
    /// Notify Metal that CPU writes modified the given managed-memory byte range.
    ///
    /// # Errors
    ///
    /// Returns an error for non-managed storage or an invalid byte range.
    pub fn did_modify_range(&self, range: Range<usize>) -> Result<(), MetalBufferAccessError> {
        if range.start > range.end {
            return Err(MetalBufferAccessError::InvalidRange);
        }
        let storage_mode = self.storage_mode();
        if storage_mode != crate::storage_mode::MANAGED {
            return Err(MetalBufferAccessError::ManagedStorageRequired { storage_mode });
        }
        let length = range.end - range.start;
        self.checked_range_end(range.start, length)?;
        unsafe {
            ffi::am_buffer_did_modify_range(self.as_ptr(), range.start, length);
        };
        Ok(())
    }

    /// Create a 2D texture view that shares this buffer's storage.
    #[must_use]
    pub fn new_texture_view_2d(
        &self,
        pixel_format: usize,
        width: usize,
        height: usize,
        bytes_per_row: usize,
        offset: usize,
    ) -> Option<MetalTexture> {
        let ptr = unsafe {
            ffi::am_buffer_new_texture_view_2d(
                self.as_ptr(),
                pixel_format,
                width,
                height,
                bytes_per_row,
                offset,
            )
        };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { MetalTexture::from_raw(ptr) })
        }
    }
}

/// Errors returned by CPU texture uploads and readback.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TextureTransferError {
    /// The texture format does not have a supported CPU byte layout.
    UnsupportedPixelFormat { pixel_format: usize },
    /// The texture storage mode cannot be accessed through CPU transfer APIs.
    CpuInaccessibleStorage { storage_mode: usize },
    /// The requested mipmap level does not exist.
    InvalidMipmapLevel {
        mipmap_level: usize,
        mipmap_level_count: usize,
    },
    /// The requested array slice does not exist.
    InvalidSlice { slice: usize, array_length: usize },
    /// A zero-sized transfer region was requested.
    EmptyRegion,
    /// A two-dimensional transfer was requested for a depth texture.
    UnsupportedDepth { depth: usize },
    /// The texture kind is not compatible with a two-dimensional CPU transfer.
    UnsupportedTextureType { texture_type: usize },
    /// The region exceeds the selected mip level.
    RegionOutOfBounds {
        origin: (usize, usize),
        size: (usize, usize),
        mip_size: (usize, usize),
    },
    /// A compressed-format coordinate is not block aligned.
    BlockMisaligned {
        field: &'static str,
        value: usize,
        block_size: usize,
    },
    /// The row stride is shorter than the format requires.
    BytesPerRowTooSmall {
        bytes_per_row: usize,
        minimum: usize,
    },
    /// The row stride is not aligned to a complete format block.
    BytesPerRowMisaligned {
        bytes_per_row: usize,
        bytes_per_block: usize,
    },
    /// Checked layout arithmetic overflowed.
    LayoutOverflow,
    /// A value cannot be represented by the native API.
    IntegerOutOfRange { field: &'static str, value: usize },
    /// The source or destination byte slice is too short.
    BufferTooShort { actual: usize, required: usize },
    /// The native bridge rejected a validated transfer.
    NativeRejected,
}

impl core::fmt::Display for TextureTransferError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::UnsupportedPixelFormat { pixel_format } => {
                write!(
                    formatter,
                    "pixel format {pixel_format} has no supported CPU layout"
                )
            }
            Self::CpuInaccessibleStorage { storage_mode } => {
                write!(
                    formatter,
                    "storage mode {storage_mode} is not CPU-accessible"
                )
            }
            Self::InvalidMipmapLevel {
                mipmap_level,
                mipmap_level_count,
            } => write!(
                formatter,
                "mipmap level {mipmap_level} is outside 0..{mipmap_level_count}"
            ),
            Self::InvalidSlice {
                slice,
                array_length,
            } => write!(
                formatter,
                "texture slice {slice} is outside 0..{array_length}"
            ),
            Self::EmptyRegion => formatter.write_str("texture transfer region is empty"),
            Self::UnsupportedDepth { depth } => {
                write!(
                    formatter,
                    "2D texture transfer does not support depth {depth}"
                )
            }
            Self::UnsupportedTextureType { texture_type } => {
                write!(
                    formatter,
                    "texture type {texture_type} is not a supported 2D transfer"
                )
            }
            Self::RegionOutOfBounds {
                origin,
                size,
                mip_size,
            } => write!(
                formatter,
                "region {origin:?} + {size:?} exceeds mip dimensions {mip_size:?}"
            ),
            Self::BlockMisaligned {
                field,
                value,
                block_size,
            } => write!(
                formatter,
                "{field} value {value} is not aligned to block size {block_size}"
            ),
            Self::BytesPerRowTooSmall {
                bytes_per_row,
                minimum,
            } => write!(
                formatter,
                "bytes_per_row {bytes_per_row} is smaller than required {minimum}"
            ),
            Self::BytesPerRowMisaligned {
                bytes_per_row,
                bytes_per_block,
            } => write!(
                formatter,
                "bytes_per_row {bytes_per_row} is not aligned to {bytes_per_block}-byte blocks"
            ),
            Self::LayoutOverflow => formatter.write_str("texture byte layout overflowed"),
            Self::IntegerOutOfRange { field, value } => {
                write!(formatter, "{field} value {value} exceeds native Int")
            }
            Self::BufferTooShort { actual, required } => write!(
                formatter,
                "byte slice length {actual} is shorter than required {required}"
            ),
            Self::NativeRejected => formatter.write_str("Metal rejected the texture transfer"),
        }
    }
}

impl std::error::Error for TextureTransferError {}

#[derive(Clone, Copy)]
struct TextureTransferMetadata {
    width: usize,
    height: usize,
    depth: usize,
    mipmap_level_count: usize,
    array_length: usize,
    pixel_format: usize,
    texture_type: usize,
    storage_mode: usize,
}

#[derive(Clone, Copy)]
struct PixelFormatLayout {
    block_width: usize,
    block_height: usize,
    bytes_per_block: usize,
}

#[allow(clippy::missing_errors_doc)]
impl MetalTexture {
    /// Texture depth in pixels.
    #[must_use]
    pub fn depth(&self) -> usize {
        unsafe { ffi::am_texture_depth(self.as_ptr()) }
    }

    /// Number of mipmap levels.
    #[must_use]
    pub fn mipmap_level_count(&self) -> usize {
        unsafe { ffi::am_texture_mipmap_level_count(self.as_ptr()) }
    }

    /// Number of array slices.
    #[must_use]
    pub fn array_length(&self) -> usize {
        unsafe { ffi::am_texture_array_length(self.as_ptr()) }
    }

    /// `MTLTextureUsage` bitmask.
    #[must_use]
    pub fn usage(&self) -> usize {
        unsafe { ffi::am_texture_usage(self.as_ptr()) }
    }

    /// `MTLStorageMode` enum value.
    #[must_use]
    pub fn storage_mode(&self) -> usize {
        unsafe { ffi::am_texture_storage_mode(self.as_ptr()) }
    }

    /// Upload bytes into slice zero of a 2D texture region.
    ///
    /// # Safety
    ///
    /// No GPU access or CPU/native alias access may overlap this transfer. For
    /// buffer-backed textures, this includes mappings of the backing buffer.
    pub unsafe fn replace_region_2d(
        &self,
        bytes: &[u8],
        bytes_per_row: usize,
        origin: (usize, usize),
        size: (usize, usize),
        mipmap_level: usize,
    ) -> Result<(), TextureTransferError> {
        unsafe {
            self.replace_region_2d_at_slice(bytes, bytes_per_row, origin, size, mipmap_level, 0)
        }
    }

    /// Upload bytes into a selected array slice of a 2D texture region.
    ///
    /// # Safety
    ///
    /// No GPU access or CPU/native alias access may overlap this transfer. For
    /// buffer-backed textures, this includes mappings of the backing buffer.
    pub unsafe fn replace_region_2d_at_slice(
        &self,
        bytes: &[u8],
        bytes_per_row: usize,
        origin: (usize, usize),
        size: (usize, usize),
        mipmap_level: usize,
        slice: usize,
    ) -> Result<(), TextureTransferError> {
        validate_texture_transfer(
            self.transfer_metadata(),
            bytes.len(),
            bytes_per_row,
            origin,
            size,
            mipmap_level,
            slice,
        )?;
        let accepted = unsafe {
            ffi::am_texture_replace_region_2d(
                self.as_ptr(),
                origin.0,
                origin.1,
                size.0,
                size.1,
                mipmap_level,
                slice,
                bytes.as_ptr(),
                bytes.len(),
                bytes_per_row,
            )
        };
        if accepted {
            Ok(())
        } else {
            Err(TextureTransferError::NativeRejected)
        }
    }

    /// Read slice zero of a 2D texture region into `out`.
    ///
    /// # Safety
    ///
    /// No GPU write or CPU/native alias mutation may overlap this transfer.
    /// Managed GPU writes require a completed resource synchronization first.
    pub unsafe fn read_bytes_2d(
        &self,
        out: &mut [u8],
        bytes_per_row: usize,
        origin: (usize, usize),
        size: (usize, usize),
        mipmap_level: usize,
    ) -> Result<(), TextureTransferError> {
        unsafe { self.read_bytes_2d_at_slice(out, bytes_per_row, origin, size, mipmap_level, 0) }
    }

    /// Read a selected array slice of a 2D texture region into `out`.
    ///
    /// # Safety
    ///
    /// No GPU write or CPU/native alias mutation may overlap this transfer.
    /// Managed GPU writes require a completed resource synchronization first.
    pub unsafe fn read_bytes_2d_at_slice(
        &self,
        out: &mut [u8],
        bytes_per_row: usize,
        origin: (usize, usize),
        size: (usize, usize),
        mipmap_level: usize,
        slice: usize,
    ) -> Result<(), TextureTransferError> {
        validate_texture_transfer(
            self.transfer_metadata(),
            out.len(),
            bytes_per_row,
            origin,
            size,
            mipmap_level,
            slice,
        )?;
        let accepted = unsafe {
            ffi::am_texture_get_bytes_2d(
                self.as_ptr(),
                out.as_mut_ptr(),
                out.len(),
                bytes_per_row,
                origin.0,
                origin.1,
                size.0,
                size.1,
                mipmap_level,
                slice,
            )
        };
        if accepted {
            Ok(())
        } else {
            Err(TextureTransferError::NativeRejected)
        }
    }

    /// Create a texture view with a compatible pixel format.
    #[must_use]
    pub fn new_view(&self, pixel_format: usize) -> Option<Self> {
        let ptr = unsafe { ffi::am_texture_new_view(self.as_ptr(), pixel_format) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { Self::from_raw(ptr) })
        }
    }

    fn transfer_metadata(&self) -> TextureTransferMetadata {
        TextureTransferMetadata {
            width: self.width(),
            height: self.height(),
            depth: self.depth(),
            mipmap_level_count: self.mipmap_level_count(),
            array_length: self.array_length(),
            pixel_format: self.pixel_format(),
            texture_type: self.texture_type(),
            storage_mode: self.storage_mode(),
        }
    }
}

#[allow(clippy::too_many_lines)]
fn validate_texture_transfer(
    metadata: TextureTransferMetadata,
    byte_length: usize,
    bytes_per_row: usize,
    origin: (usize, usize),
    size: (usize, usize),
    mipmap_level: usize,
    slice: usize,
) -> Result<(), TextureTransferError> {
    if !matches!(
        metadata.storage_mode,
        crate::storage_mode::SHARED | crate::storage_mode::MANAGED
    ) {
        return Err(TextureTransferError::CpuInaccessibleStorage {
            storage_mode: metadata.storage_mode,
        });
    }
    if metadata.depth != 1 {
        return Err(TextureTransferError::UnsupportedDepth {
            depth: metadata.depth,
        });
    }
    if mipmap_level >= metadata.mipmap_level_count {
        return Err(TextureTransferError::InvalidMipmapLevel {
            mipmap_level,
            mipmap_level_count: metadata.mipmap_level_count,
        });
    }
    let array_length = texture_slice_count(metadata)?;
    if slice >= array_length {
        return Err(TextureTransferError::InvalidSlice {
            slice,
            array_length,
        });
    }
    if size.0 == 0 || size.1 == 0 {
        return Err(TextureTransferError::EmptyRegion);
    }
    for (field, value) in [
        ("bytes_per_row", bytes_per_row),
        ("origin.x", origin.0),
        ("origin.y", origin.1),
        ("size.width", size.0),
        ("size.height", size.1),
        ("mipmap_level", mipmap_level),
        ("slice", slice),
        ("byte_length", byte_length),
    ] {
        ensure_texture_native_int(value, field)?;
    }

    let mip_size = (
        mip_dimension(metadata.width, mipmap_level),
        mip_dimension(metadata.height, mipmap_level),
    );
    let end_x = origin
        .0
        .checked_add(size.0)
        .ok_or(TextureTransferError::RegionOutOfBounds {
            origin,
            size,
            mip_size,
        })?;
    let end_y = origin
        .1
        .checked_add(size.1)
        .ok_or(TextureTransferError::RegionOutOfBounds {
            origin,
            size,
            mip_size,
        })?;
    if end_x > mip_size.0 || end_y > mip_size.1 {
        return Err(TextureTransferError::RegionOutOfBounds {
            origin,
            size,
            mip_size,
        });
    }

    let layout = pixel_format_layout(metadata.pixel_format).ok_or(
        TextureTransferError::UnsupportedPixelFormat {
            pixel_format: metadata.pixel_format,
        },
    )?;
    validate_block_alignment("origin.x", origin.0, layout.block_width)?;
    validate_block_alignment("origin.y", origin.1, layout.block_height)?;
    if end_x != mip_size.0 {
        validate_block_alignment("size.width", size.0, layout.block_width)?;
    }
    if end_y != mip_size.1 {
        validate_block_alignment("size.height", size.1, layout.block_height)?;
    }

    let blocks_per_row = checked_div_ceil(size.0, layout.block_width)?;
    let block_rows = checked_div_ceil(size.1, layout.block_height)?;
    let minimum_row_bytes = blocks_per_row
        .checked_mul(layout.bytes_per_block)
        .ok_or(TextureTransferError::LayoutOverflow)?;
    if bytes_per_row < minimum_row_bytes {
        return Err(TextureTransferError::BytesPerRowTooSmall {
            bytes_per_row,
            minimum: minimum_row_bytes,
        });
    }
    if bytes_per_row % layout.bytes_per_block != 0 {
        return Err(TextureTransferError::BytesPerRowMisaligned {
            bytes_per_row,
            bytes_per_block: layout.bytes_per_block,
        });
    }
    let preceding_rows = bytes_per_row
        .checked_mul(block_rows - 1)
        .ok_or(TextureTransferError::LayoutOverflow)?;
    let required = preceding_rows
        .checked_add(minimum_row_bytes)
        .ok_or(TextureTransferError::LayoutOverflow)?;
    ensure_texture_native_int(required, "required byte length")?;
    if byte_length < required {
        return Err(TextureTransferError::BufferTooShort {
            actual: byte_length,
            required,
        });
    }
    Ok(())
}

fn pixel_format_layout(pixel_format: usize) -> Option<PixelFormatLayout> {
    use crate::pixel_format;

    let bytes_per_block = match pixel_format {
        pixel_format::A8UNORM
        | pixel_format::R8UNORM
        | pixel_format::R8SNORM
        | pixel_format::R8UINT
        | pixel_format::R8SINT => 1,
        pixel_format::R16UNORM
        | pixel_format::R16SNORM
        | pixel_format::R16UINT
        | pixel_format::R16SINT
        | pixel_format::R16FLOAT
        | pixel_format::RG8UNORM
        | pixel_format::RG8SNORM
        | pixel_format::RG8UINT
        | pixel_format::RG8SINT => 2,
        pixel_format::R32FLOAT
        | pixel_format::RG16FLOAT
        | pixel_format::RGBA8UNORM
        | pixel_format::RGBA8UNORM_SRGB
        | pixel_format::RGBA8SNORM
        | pixel_format::RGBA8UINT
        | pixel_format::RGBA8SINT
        | pixel_format::BGRA8UNORM
        | pixel_format::BGRA8UNORM_SRGB
        | pixel_format::BGRA10_XR
        | pixel_format::BGR10_XR => 4,
        pixel_format::RGBA16FLOAT => 8,
        pixel_format::RGBA32FLOAT => 16,
        _ => return None,
    };
    Some(PixelFormatLayout {
        block_width: 1,
        block_height: 1,
        bytes_per_block,
    })
}

fn texture_slice_count(metadata: TextureTransferMetadata) -> Result<usize, TextureTransferError> {
    match metadata.texture_type {
        crate::texture_type::TYPE_2D => Ok(1),
        crate::texture_type::TYPE_2D_ARRAY => Ok(metadata.array_length.max(1)),
        crate::texture_type::CUBE => Ok(6),
        crate::texture_type::CUBE_ARRAY => metadata
            .array_length
            .max(1)
            .checked_mul(6)
            .ok_or(TextureTransferError::LayoutOverflow),
        texture_type => Err(TextureTransferError::UnsupportedTextureType { texture_type }),
    }
}

fn mip_dimension(base: usize, mipmap_level: usize) -> usize {
    base.checked_shr(u32::try_from(mipmap_level).unwrap_or(u32::MAX))
        .unwrap_or(0)
        .max(1)
}

fn checked_div_ceil(value: usize, divisor: usize) -> Result<usize, TextureTransferError> {
    value
        .checked_add(divisor - 1)
        .map(|adjusted| adjusted / divisor)
        .ok_or(TextureTransferError::LayoutOverflow)
}

fn validate_block_alignment(
    field: &'static str,
    value: usize,
    block_size: usize,
) -> Result<(), TextureTransferError> {
    if value % block_size == 0 {
        Ok(())
    } else {
        Err(TextureTransferError::BlockMisaligned {
            field,
            value,
            block_size,
        })
    }
}

fn ensure_texture_native_int(
    value: usize,
    field: &'static str,
) -> Result<(), TextureTransferError> {
    if isize::try_from(value).is_ok() {
        Ok(())
    } else {
        Err(TextureTransferError::IntegerOutOfRange { field, value })
    }
}

impl ComputePipelineState {
    /// Thread execution width for this compute pipeline.
    #[must_use]
    pub fn thread_execution_width(&self) -> usize {
        unsafe { ffi::am_compute_pipeline_state_thread_execution_width(self.as_ptr()) }
    }

    /// Maximum threads per threadgroup.
    #[must_use]
    pub fn max_total_threads_per_threadgroup(&self) -> usize {
        unsafe { ffi::am_compute_pipeline_state_max_total_threads_per_threadgroup(self.as_ptr()) }
    }

    /// Allocate a visible function table for this pipeline.
    #[must_use]
    pub fn new_visible_function_table(
        &self,
        function_count: usize,
    ) -> Option<VisibleFunctionTable> {
        VisibleFunctionTable::wrap(unsafe {
            ffi::am_compute_pipeline_state_new_visible_function_table(self.as_ptr(), function_count)
        })
    }

    /// Allocate an intersection function table for this pipeline.
    #[must_use]
    pub fn new_intersection_function_table(
        &self,
        function_count: usize,
    ) -> Option<IntersectionFunctionTable> {
        IntersectionFunctionTable::wrap(unsafe {
            ffi::am_compute_pipeline_state_new_intersection_function_table(
                self.as_ptr(),
                function_count,
            )
        })
    }
}

impl MetalFunction {
    /// Create an argument encoder for the argument buffer bound at `buffer_index`.
    #[must_use]
    pub fn new_argument_encoder(&self, buffer_index: usize) -> Option<ArgumentEncoder> {
        let ptr = unsafe { ffi::am_function_new_argument_encoder(self.as_ptr(), buffer_index) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { ArgumentEncoder::from_function_ptr(ptr) })
        }
    }
}

impl Heap {
    /// Heap size in bytes.
    #[must_use]
    pub fn size(&self) -> usize {
        unsafe { ffi::am_heap_size(self.as_ptr()) }
    }

    /// Bytes currently used by heap-backed resources.
    #[must_use]
    pub fn used_size(&self) -> usize {
        unsafe { ffi::am_heap_used_size(self.as_ptr()) }
    }

    /// Current heap allocation size in bytes.
    #[must_use]
    pub fn current_allocated_size(&self) -> usize {
        unsafe { ffi::am_heap_current_allocated_size(self.as_ptr()) }
    }

    /// Largest allocatable block in the heap for the given alignment.
    #[must_use]
    pub fn max_available_size(&self, alignment: usize) -> usize {
        unsafe { ffi::am_heap_max_available_size(self.as_ptr(), alignment) }
    }

    /// Allocate a buffer from this heap.
    #[must_use]
    pub fn new_buffer(&self, length: usize, options: usize) -> Option<MetalBuffer> {
        let ptr = unsafe { ffi::am_heap_new_buffer(self.as_ptr(), length, options) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { MetalBuffer::from_retained_ptr(ptr) })
        }
    }

    /// Allocate a texture from this heap.
    #[must_use]
    pub fn new_texture(&self, descriptor: TextureDescriptor) -> Option<MetalTexture> {
        let ptr = unsafe {
            ffi::am_heap_new_texture_2d(
                self.as_ptr(),
                descriptor.pixel_format,
                descriptor.width,
                descriptor.height,
                descriptor.mipmapped,
                descriptor.usage,
                descriptor.storage_mode,
            )
        };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { MetalTexture::from_raw(ptr) })
        }
    }

    /// Allocate acceleration-structure storage from this heap.
    #[must_use]
    pub fn new_acceleration_structure_with_size(
        &self,
        size: usize,
    ) -> Option<AccelerationStructure> {
        AccelerationStructure::wrap(unsafe {
            ffi::am_heap_new_acceleration_structure_with_size(self.as_ptr(), size)
        })
    }

    /// Set the heap purgeable state.
    #[must_use]
    pub fn set_purgeable_state(&self, state: usize) -> usize {
        unsafe { ffi::am_heap_set_purgeable_state(self.as_ptr(), state) }
    }
}

impl Event {
    /// Current `signaledValue`.
    #[must_use]
    pub fn signaled_value(&self) -> u64 {
        unsafe { ffi::am_event_signaled_value(self.as_ptr()) }
    }

    /// Update the event's `signaledValue`.
    pub fn set_signaled_value(&self, value: u64) {
        unsafe { ffi::am_event_set_signaled_value(self.as_ptr(), value) };
    }

    /// Wait until the event reaches at least `value`.
    #[must_use]
    pub fn wait_until_signaled_value(&self, value: u64, timeout_ms: u64) -> bool {
        unsafe { ffi::am_event_wait_until_signaled_value(self.as_ptr(), value, timeout_ms) }
    }
}

impl DynamicLibrary {
    /// Install name embedded into the dynamic library.
    #[must_use]
    pub fn install_name(&self) -> String {
        unsafe { take_string(ffi::am_dynamic_library_install_name(self.as_ptr())) }
    }

    /// Serialize this dynamic library to `path`.
    ///
    /// # Errors
    ///
    /// Returns Metal's localized serialization error on failure.
    pub fn serialize_to_file(&self, path: &Path) -> Result<(), String> {
        let path = c_string(path.to_string_lossy().as_ref())?;
        let mut err: *mut core::ffi::c_char = core::ptr::null_mut();
        let ok = unsafe {
            ffi::am_dynamic_library_serialize_to_url(self.as_ptr(), path.as_ptr(), &mut err)
        };
        if ok {
            Ok(())
        } else {
            Err(unsafe {
                take_optional_string(err)
                    .unwrap_or_else(|| "MTLDynamicLibrary.serialize(to:) failed".to_string())
            })
        }
    }
}

impl BinaryArchive {
    /// Add a compute pipeline descriptor built from `function` to the archive.
    ///
    /// # Errors
    ///
    /// Returns Metal's localized archive error on failure.
    pub fn add_compute_function(&self, function: &MetalFunction) -> Result<(), String> {
        let mut err: *mut core::ffi::c_char = core::ptr::null_mut();
        let ok = unsafe {
            ffi::am_binary_archive_add_compute_function(self.as_ptr(), function.as_ptr(), &mut err)
        };
        if ok {
            Ok(())
        } else {
            Err(unsafe {
                take_optional_string(err).unwrap_or_else(|| {
                    "MTLBinaryArchive.addComputePipelineFunctions failed".to_string()
                })
            })
        }
    }

    /// Add a render pipeline descriptor built from `vertex` and `fragment` to the archive.
    ///
    /// # Errors
    ///
    /// Returns Metal's localized archive error on failure.
    pub fn add_render_functions(
        &self,
        vertex: &MetalFunction,
        fragment: &MetalFunction,
        color_pixel_format: usize,
        sample_count: usize,
    ) -> Result<(), String> {
        let mut err: *mut core::ffi::c_char = core::ptr::null_mut();
        let ok = unsafe {
            ffi::am_binary_archive_add_render_functions(
                self.as_ptr(),
                vertex.as_ptr(),
                fragment.as_ptr(),
                color_pixel_format,
                sample_count,
                &mut err,
            )
        };
        if ok {
            Ok(())
        } else {
            Err(unsafe {
                take_optional_string(err).unwrap_or_else(|| {
                    "MTLBinaryArchive.addRenderPipelineFunctions failed".to_string()
                })
            })
        }
    }

    /// Serialize the archive to `path`.
    ///
    /// # Errors
    ///
    /// Returns Metal's localized serialization error on failure.
    pub fn serialize_to_file(&self, path: &Path) -> Result<(), String> {
        let path = c_string(path.to_string_lossy().as_ref())?;
        let mut err: *mut core::ffi::c_char = core::ptr::null_mut();
        let ok = unsafe {
            ffi::am_binary_archive_serialize_to_url(self.as_ptr(), path.as_ptr(), &mut err)
        };
        if ok {
            Ok(())
        } else {
            Err(unsafe {
                take_optional_string(err)
                    .unwrap_or_else(|| "MTLBinaryArchive.serialize(to:) failed".to_string())
            })
        }
    }
}

impl IndirectCommandBuffer {
    /// Size of the indirect command buffer in bytes.
    #[must_use]
    pub fn size(&self) -> usize {
        unsafe { ffi::am_indirect_command_buffer_size(self.as_ptr()) }
    }

    /// Reset commands in `range` back to empty state.
    pub fn reset_range(&self, range: Range<usize>) {
        unsafe {
            ffi::am_indirect_command_buffer_reset_range(
                self.as_ptr(),
                range.start,
                range.end.saturating_sub(range.start),
            );
        };
    }
}

impl AccelerationStructure {
    /// Allocated storage size in bytes.
    #[must_use]
    pub fn size(&self) -> usize {
        unsafe { ffi::am_acceleration_structure_size(self.as_ptr()) }
    }
}

impl IntersectionFunctionTable {
    /// Populate `index` with the built-in opaque triangle intersection function.
    pub fn set_opaque_triangle_intersection_function(&self, signature: usize, index: usize) {
        unsafe {
            ffi::am_intersection_function_table_set_opaque_triangle(
                self.as_ptr(),
                signature,
                index,
            );
        };
    }
}

impl CounterSampleBuffer {
    /// Number of samples available in the buffer.
    #[must_use]
    pub fn sample_count(&self) -> usize {
        unsafe { ffi::am_counter_sample_buffer_sample_count(self.as_ptr()) }
    }

    /// Resolve raw counter bytes for `range`.
    #[must_use]
    pub fn resolve_range(&self, range: Range<usize>) -> Option<Vec<u8>> {
        let mut out_len = 0usize;
        let ptr = unsafe {
            ffi::am_counter_sample_buffer_resolve_range(
                self.as_ptr(),
                range.start,
                range.end.saturating_sub(range.start),
                &mut out_len,
            )
        };
        if ptr.is_null() {
            None
        } else {
            let bytes = unsafe { core::slice::from_raw_parts(ptr.cast::<u8>(), out_len) }.to_vec();
            unsafe { libc::free(ptr.cast()) };
            Some(bytes)
        }
    }
}

impl ResidencySet {
    /// Add `buffer` to the set.
    pub fn add_buffer(&self, buffer: &MetalBuffer) {
        unsafe { ffi::am_residency_set_add_buffer(self.as_ptr(), buffer.as_ptr()) };
    }

    /// Add `texture` to the set.
    pub fn add_texture(&self, texture: &MetalTexture) {
        unsafe { ffi::am_residency_set_add_texture(self.as_ptr(), texture.as_ptr()) };
    }

    /// Add `heap` to the set.
    pub fn add_heap(&self, heap: &Heap) {
        unsafe { ffi::am_residency_set_add_heap(self.as_ptr(), heap.as_ptr()) };
    }

    /// Remove `buffer` from the set.
    pub fn remove_buffer(&self, buffer: &MetalBuffer) {
        unsafe { ffi::am_residency_set_remove_buffer(self.as_ptr(), buffer.as_ptr()) };
    }

    /// Remove `texture` from the set.
    pub fn remove_texture(&self, texture: &MetalTexture) {
        unsafe { ffi::am_residency_set_remove_texture(self.as_ptr(), texture.as_ptr()) };
    }

    /// Remove `heap` from the set.
    pub fn remove_heap(&self, heap: &Heap) {
        unsafe { ffi::am_residency_set_remove_heap(self.as_ptr(), heap.as_ptr()) };
    }

    /// Remove all pending and committed allocations.
    pub fn remove_all_allocations(&self) {
        unsafe { ffi::am_residency_set_remove_all_allocations(self.as_ptr()) };
    }

    /// Whether the set currently contains `buffer`.
    #[must_use]
    pub fn contains_buffer(&self, buffer: &MetalBuffer) -> bool {
        unsafe { ffi::am_residency_set_contains_buffer(self.as_ptr(), buffer.as_ptr()) }
    }

    /// Whether the set currently contains `texture`.
    #[must_use]
    pub fn contains_texture(&self, texture: &MetalTexture) -> bool {
        unsafe { ffi::am_residency_set_contains_texture(self.as_ptr(), texture.as_ptr()) }
    }

    /// Number of allocations in the set.
    #[must_use]
    pub fn allocation_count(&self) -> usize {
        unsafe { ffi::am_residency_set_allocation_count(self.as_ptr()) }
    }

    /// Commit pending add/remove changes.
    pub fn commit(&self) {
        unsafe { ffi::am_residency_set_commit(self.as_ptr()) };
    }

    /// Request residency for all committed allocations.
    pub fn request_residency(&self) {
        unsafe { ffi::am_residency_set_request_residency(self.as_ptr()) };
    }

    /// End residency for all committed allocations.
    pub fn end_residency(&self) {
        unsafe { ffi::am_residency_set_end_residency(self.as_ptr()) };
    }
}

impl CaptureManager {
    /// Retrieve the process-global capture manager.
    #[must_use]
    pub fn shared() -> Option<Self> {
        Self::wrap(unsafe { ffi::am_capture_manager_shared() })
    }

    /// Query whether the given capture destination is supported.
    #[must_use]
    pub fn supports_destination(&self, destination: usize) -> bool {
        unsafe { ffi::am_capture_manager_supports_destination(self.as_ptr(), destination) }
    }

    /// Whether a capture is currently in progress.
    #[must_use]
    pub fn is_capturing(&self) -> bool {
        unsafe { ffi::am_capture_manager_is_capturing(self.as_ptr()) }
    }

    /// Create a capture scope that captures all command queues on `device`.
    #[must_use]
    pub fn new_capture_scope_with_device(&self, device: &MetalDevice) -> Option<CaptureScope> {
        CaptureScope::wrap(unsafe {
            ffi::am_capture_manager_new_scope_with_device(self.as_ptr(), device.as_ptr())
        })
    }

    /// Create a capture scope restricted to `command_queue`.
    #[must_use]
    pub fn new_capture_scope_with_command_queue(
        &self,
        command_queue: &CommandQueue,
    ) -> Option<CaptureScope> {
        CaptureScope::wrap(unsafe {
            ffi::am_capture_manager_new_scope_with_command_queue(
                self.as_ptr(),
                command_queue.as_ptr(),
            )
        })
    }
}

impl CaptureScope {
    /// Mark the start of the capture scope.
    pub fn begin(&self) {
        unsafe { ffi::am_capture_scope_begin(self.as_ptr()) };
    }

    /// Mark the end of the capture scope.
    pub fn end(&self) {
        unsafe { ffi::am_capture_scope_end(self.as_ptr()) };
    }
}

#[cfg(test)]
mod texture_transfer_tests {
    use super::*;

    fn rgba8_metadata() -> TextureTransferMetadata {
        TextureTransferMetadata {
            width: 4,
            height: 4,
            depth: 1,
            mipmap_level_count: 1,
            array_length: 1,
            pixel_format: crate::pixel_format::RGBA8UNORM,
            texture_type: crate::texture_type::TYPE_2D,
            storage_mode: crate::storage_mode::SHARED,
        }
    }

    #[test]
    fn rejects_short_rgba_row() {
        assert!(matches!(
            validate_texture_transfer(rgba8_metadata(), 64, 15, (0, 0), (4, 4), 0, 0),
            Err(TextureTransferError::BytesPerRowTooSmall { minimum: 16, .. })
        ));
    }

    #[test]
    fn rejects_stride_larger_than_native_int() {
        assert!(matches!(
            validate_texture_transfer(
                rgba8_metadata(),
                usize::MAX,
                usize::MAX,
                (0, 0),
                (4, 4),
                0,
                0,
            ),
            Err(TextureTransferError::IntegerOutOfRange {
                field: "bytes_per_row",
                ..
            })
        ));
    }

    #[test]
    fn rejects_unknown_pixel_format() {
        let mut metadata = rgba8_metadata();
        metadata.pixel_format = usize::MAX;
        assert!(matches!(
            validate_texture_transfer(metadata, 64, 16, (0, 0), (4, 4), 0, 0),
            Err(TextureTransferError::UnsupportedPixelFormat { .. })
        ));
    }

    #[test]
    fn accepts_final_row_without_trailing_stride_padding() {
        assert!(validate_texture_transfer(rgba8_metadata(), 80, 32, (0, 0), (4, 3), 0, 0).is_ok());
    }
}
