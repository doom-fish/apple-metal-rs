#![allow(clippy::module_name_repetitions, clippy::too_many_lines, clippy::type_complexity)]

use crate::{
    ffi, util::{c_string, take_optional_string}, ComputePipelineState, DynamicLibrary,
    MetalDevice, MetalLibrary, RenderPipelineState,
};
use core::ffi::{c_char, c_void};
use core::ptr;
use std::path::Path;

macro_rules! opaque_symbol_handle {
    ($(#[$meta:meta])* pub struct $name:ident;) => {
        $(#[$meta])*
        pub struct $name {
            ptr: *mut c_void,
        }

        unsafe impl Send for $name {}
        unsafe impl Sync for $name {}

        impl Drop for $name {
            fn drop(&mut self) {
                if !self.ptr.is_null() {
                    unsafe { ffi::am_object_release(self.ptr) };
                    self.ptr = ptr::null_mut();
                }
            }
        }

        impl $name {
            #[must_use]
            pub const fn as_ptr(&self) -> *mut c_void {
                self.ptr
            }

            #[allow(clippy::missing_safety_doc)]
            #[must_use]
            pub unsafe fn from_raw(ptr: *mut c_void) -> Self {
                Self { ptr }
            }

            #[allow(dead_code)]
            fn wrap(ptr: *mut c_void) -> Option<Self> {
                if ptr.is_null() {
                    None
                } else {
                    Some(Self { ptr })
                }
            }

            #[must_use]
            pub fn label(&self) -> Option<String> {
                unsafe { take_optional_string(ffi::am_object_copy_label(self.ptr)) }
            }
        }
    };
}

macro_rules! opaque_symbol_class {
    ($(#[$meta:meta])* pub struct $name:ident => $objc:literal;) => {
        opaque_symbol_handle!(
            $(#[$meta])*
            pub struct $name;
        );

        impl $name {
            #[must_use]
            pub fn new() -> Option<Self> {
                Self::wrap(unsafe {
                    ffi::am_new_class_instance(concat!($objc, "\0").as_ptr().cast())
                })
            }
        }
    };
}

macro_rules! raw_value_type {
    ($(#[$meta:meta])* pub struct $name:ident($ty:ty);) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
        #[repr(transparent)]
        pub struct $name(pub $ty);

        impl $name {
            #[must_use]
            pub const fn from_raw(raw: $ty) -> Self {
                Self(raw)
            }

            #[must_use]
            pub const fn as_raw(self) -> $ty {
                self.0
            }
        }
    };
}

macro_rules! metal_string_constant {
    ($(#[$meta:meta])* pub fn $name:ident => $symbol:literal;) => {
        $(#[$meta])*
        #[must_use]
        pub fn $name() -> Option<String> {
            unsafe {
                take_optional_string(
                    ffi::am_copy_metal_string_constant(concat!($symbol, "\0").as_ptr().cast()),
                )
            }
        }
    };
}

unsafe fn take_device_array(ptr: *mut *mut c_void, count: usize) -> Vec<MetalDevice> {
    if ptr.is_null() || count == 0 {
        return Vec::new();
    }

    let slice = core::slice::from_raw_parts(ptr, count);
    let values = slice
        .iter()
        .copied()
        .map(|device| unsafe { MetalDevice::from_retained_ptr(device) })
        .collect();
    libc::free(ptr.cast());
    values
}

pub type MetalCommonCounter = String;
pub type MetalCommonCounterSet = String;
pub type MetalDeviceNotificationName = String;
pub type MetalAutoreleasedArgument = MetalArgument;
pub type MetalArgumentType = MetalBindingType;
pub type MetalAutoreleasedComputePipelineReflection = MetalComputePipelineReflection;
pub type MetalAutoreleasedRenderPipelineReflection = MetalRenderPipelineReflection;
pub type MetalNewLibraryCompletionHandler = Box<dyn FnMut(Result<MetalLibrary, String>) + Send + 'static>;
pub type MetalNewDynamicLibraryCompletionHandler = Box<dyn FnMut(Result<DynamicLibrary, String>) + Send + 'static>;
pub type MetalNewComputePipelineStateCompletionHandler =
    Box<dyn FnMut(Result<ComputePipelineState, String>) + Send + 'static>;
pub type MetalNewComputePipelineStateWithReflectionCompletionHandler =
    Box<dyn FnMut(Result<(ComputePipelineState, MetalComputePipelineReflection), String>) + Send + 'static>;
pub type MetalNewRenderPipelineStateCompletionHandler =
    Box<dyn FnMut(Result<RenderPipelineState, String>) + Send + 'static>;
pub type MetalNewRenderPipelineStateWithReflectionCompletionHandler =
    Box<dyn FnMut(Result<(RenderPipelineState, MetalRenderPipelineReflection), String>) + Send + 'static>;
pub type MetalTimestamp = u64;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct MetalCoordinate2D {
    pub x: f32,
    pub y: f32,
}

impl MetalCoordinate2D {
    #[must_use]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct MetalSize {
    pub width: usize,
    pub height: usize,
    pub depth: usize,
}

impl MetalSize {
    #[must_use]
    pub const fn new(width: usize, height: usize, depth: usize) -> Self {
        Self { width, height, depth }
    }
}

raw_value_type!(pub struct MetalGpuAddress(u64););

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct MetalOrigin {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl MetalOrigin {
    #[must_use]
    pub const fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct MetalRegion {
    pub origin: MetalOrigin,
    pub size: MetalSize,
}

impl MetalRegion {
    #[must_use]
    pub const fn new(origin: MetalOrigin, size: MetalSize) -> Self {
        Self { origin, size }
    }

    #[must_use]
    pub const fn new_1d(x: usize, width: usize) -> Self {
        Self {
            origin: MetalOrigin::new(x, 0, 0),
            size: MetalSize::new(width, 1, 1),
        }
    }

    #[must_use]
    pub const fn new_2d(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self {
            origin: MetalOrigin::new(x, y, 0),
            size: MetalSize::new(width, height, 1),
        }
    }

    #[must_use]
    pub const fn new_3d(
        x: usize,
        y: usize,
        z: usize,
        width: usize,
        height: usize,
        depth: usize,
    ) -> Self {
        Self {
            origin: MetalOrigin::new(x, y, z),
            size: MetalSize::new(width, height, depth),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct MetalResourceId {
    pub value: u64,
}

impl MetalResourceId {
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self { value }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct MetalPackedFloat3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl MetalPackedFloat3 {
    #[must_use]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MetalPackedFloatQuaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Default for MetalPackedFloatQuaternion {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }
}

impl MetalPackedFloatQuaternion {
    #[must_use]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct MetalPackedFloat4x3 {
    pub columns: [MetalPackedFloat3; 4],
}

impl MetalPackedFloat4x3 {
    #[must_use]
    pub const fn new(
        column0: MetalPackedFloat3,
        column1: MetalPackedFloat3,
        column2: MetalPackedFloat3,
        column3: MetalPackedFloat3,
    ) -> Self {
        Self {
            columns: [column0, column1, column2, column3],
        }
    }
}

raw_value_type!(pub struct MetalSparseTextureMappingMode(usize););

opaque_symbol_handle!(pub struct MetalDeviceObserver;);

pub type MetalDeviceObserverCallback =
    unsafe extern "C" fn(device: *mut c_void, notification_name: *const c_char, user_data: *mut c_void);

impl MetalDeviceObserver {
    pub fn remove(&self) {
        unsafe { ffi::am_remove_device_observer(self.as_ptr()) };
    }
}

#[must_use]
pub fn copy_all_devices() -> Vec<MetalDevice> {
    let mut count = 0;
    let ptr = unsafe { ffi::am_copy_all_devices(&mut count) };
    unsafe { take_device_array(ptr, count) }
}

#[allow(clippy::missing_safety_doc)]
pub unsafe fn copy_all_devices_with_observer(
    callback: Option<MetalDeviceObserverCallback>,
    user_data: *mut c_void,
) -> (Vec<MetalDevice>, Option<MetalDeviceObserver>) {
    let mut count = 0;
    let mut observer = ptr::null_mut();
    let ptr = ffi::am_copy_all_devices_with_observer(&mut count, &mut observer, callback, user_data);
    (take_device_array(ptr, count), MetalDeviceObserver::wrap(observer))
}

pub fn remove_device_observer(observer: &MetalDeviceObserver) {
    observer.remove();
}

pub struct MetalIoCompressionContext {
    ptr: *mut c_void,
}

impl Drop for MetalIoCompressionContext {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::am_io_flush_and_destroy_compression_context(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl MetalIoCompressionContext {
    #[must_use]
    pub const fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }

    #[allow(clippy::missing_safety_doc)]
    #[must_use]
    pub unsafe fn from_raw(ptr: *mut c_void) -> Self {
        Self { ptr }
    }

    pub fn append_data(&self, data: &[u8]) {
        unsafe {
            ffi::am_io_compression_context_append_data(self.ptr, data.as_ptr(), data.len());
        }
    }

    #[must_use]
    pub fn flush_and_destroy(mut self) -> MetalIoCompressionStatus {
        let status = unsafe { ffi::am_io_flush_and_destroy_compression_context(self.ptr) };
        self.ptr = ptr::null_mut();
        MetalIoCompressionStatus::from_raw(status)
    }
}

#[must_use]
pub fn io_compression_context_default_chunk_size() -> usize {
    unsafe { ffi::am_io_compression_context_default_chunk_size() }
}

#[must_use]
pub fn create_io_compression_context(
    path: &Path,
    method: MetalIoCompressionMethod,
    chunk_size: Option<usize>,
) -> Option<MetalIoCompressionContext> {
    let path = c_string(path.to_string_lossy().as_ref()).ok()?;
    let chunk_size = chunk_size.unwrap_or_else(io_compression_context_default_chunk_size);
    let ptr = unsafe {
        ffi::am_io_create_compression_context(path.as_ptr(), method.as_raw(), chunk_size)
    };
    if ptr.is_null() {
        None
    } else {
        Some(unsafe { MetalIoCompressionContext::from_raw(ptr) })
    }
}

metal_string_constant!(pub fn metal4_command_queue_error_domain => "MTL4CommandQueueErrorDomain";);
metal_string_constant!(pub fn metal_binary_archive_domain => "MTLBinaryArchiveDomain";);
metal_string_constant!(pub fn metal_capture_error_domain => "MTLCaptureErrorDomain";);
metal_string_constant!(pub fn metal_command_buffer_encoder_info_error_key => "MTLCommandBufferEncoderInfoErrorKey";);
metal_string_constant!(pub fn metal_command_buffer_error_domain => "MTLCommandBufferErrorDomain";);
metal_string_constant!(pub fn metal_common_counter_clipper_invocations => "MTLCommonCounterClipperInvocations";);
metal_string_constant!(pub fn metal_common_counter_clipper_primitives_out => "MTLCommonCounterClipperPrimitivesOut";);
metal_string_constant!(pub fn metal_common_counter_compute_kernel_invocations => "MTLCommonCounterComputeKernelInvocations";);
metal_string_constant!(pub fn metal_common_counter_fragment_cycles => "MTLCommonCounterFragmentCycles";);
metal_string_constant!(pub fn metal_common_counter_fragment_invocations => "MTLCommonCounterFragmentInvocations";);
metal_string_constant!(pub fn metal_common_counter_fragments_passed => "MTLCommonCounterFragmentsPassed";);
metal_string_constant!(pub fn metal_common_counter_post_tessellation_vertex_cycles => "MTLCommonCounterPostTessellationVertexCycles";);
metal_string_constant!(pub fn metal_common_counter_post_tessellation_vertex_invocations => "MTLCommonCounterPostTessellationVertexInvocations";);
metal_string_constant!(pub fn metal_common_counter_render_target_write_cycles => "MTLCommonCounterRenderTargetWriteCycles";);
metal_string_constant!(pub fn metal_common_counter_set_stage_utilization => "MTLCommonCounterSetStageUtilization";);
metal_string_constant!(pub fn metal_common_counter_set_statistic => "MTLCommonCounterSetStatistic";);
metal_string_constant!(pub fn metal_common_counter_set_timestamp => "MTLCommonCounterSetTimestamp";);
metal_string_constant!(pub fn metal_common_counter_tessellation_cycles => "MTLCommonCounterTessellationCycles";);
metal_string_constant!(pub fn metal_common_counter_tessellation_input_patches => "MTLCommonCounterTessellationInputPatches";);
metal_string_constant!(pub fn metal_common_counter_timestamp => "MTLCommonCounterTimestamp";);
metal_string_constant!(pub fn metal_common_counter_total_cycles => "MTLCommonCounterTotalCycles";);
metal_string_constant!(pub fn metal_common_counter_vertex_cycles => "MTLCommonCounterVertexCycles";);
metal_string_constant!(pub fn metal_common_counter_vertex_invocations => "MTLCommonCounterVertexInvocations";);
metal_string_constant!(pub fn metal_counter_error_domain => "MTLCounterErrorDomain";);
metal_string_constant!(pub fn metal_device_removal_requested_notification => "MTLDeviceRemovalRequestedNotification";);
metal_string_constant!(pub fn metal_device_was_added_notification => "MTLDeviceWasAddedNotification";);
metal_string_constant!(pub fn metal_device_was_removed_notification => "MTLDeviceWasRemovedNotification";);
metal_string_constant!(pub fn metal_dynamic_library_domain => "MTLDynamicLibraryDomain";);
metal_string_constant!(pub fn metal_io_error_domain => "MTLIOErrorDomain";);
metal_string_constant!(pub fn metal_library_error_domain => "MTLLibraryErrorDomain";);
metal_string_constant!(pub fn metal_log_state_error_domain => "MTLLogStateErrorDomain";);
metal_string_constant!(pub fn metal_tensor_domain => "MTLTensorDomain";);

raw_value_type!(pub struct Metal4AlphaToCoverageState(usize););
raw_value_type!(pub struct Metal4AlphaToOneState(usize););
raw_value_type!(pub struct Metal4BinaryFunctionOptions(usize););
raw_value_type!(pub struct Metal4BlendState(usize););
raw_value_type!(pub struct Metal4CommandQueueError(usize););
raw_value_type!(pub struct Metal4CompilerTaskStatus(usize););
raw_value_type!(pub struct Metal4CounterHeapType(usize););
raw_value_type!(pub struct Metal4IndirectCommandBufferSupportState(usize););
raw_value_type!(pub struct Metal4LogicalToPhysicalColorAttachmentMappingState(usize););
raw_value_type!(pub struct Metal4PipelineDataSetSerializerConfiguration(usize););
raw_value_type!(pub struct Metal4RenderEncoderOptions(usize););
raw_value_type!(pub struct Metal4ShaderReflection(usize););
raw_value_type!(pub struct Metal4TimestampGranularity(usize););
raw_value_type!(pub struct Metal4VisibilityOptions(usize););
raw_value_type!(pub struct MetalAccelerationStructureInstanceDescriptorType(usize););
raw_value_type!(pub struct MetalAccelerationStructureInstanceOptions(usize););
raw_value_type!(pub struct MetalAccelerationStructureRefitOptions(usize););
raw_value_type!(pub struct MetalAccelerationStructureUsage(usize););
raw_value_type!(pub struct MetalArgumentAccess(usize););
raw_value_type!(pub struct MetalAttributeFormat(usize););
raw_value_type!(pub struct MetalBarrierScope(usize););
raw_value_type!(pub struct MetalBinaryArchiveError(usize););
raw_value_type!(pub struct MetalBindingType(usize););
raw_value_type!(pub struct MetalBlitOption(usize););
raw_value_type!(pub struct MetalBufferSparseTier(usize););
raw_value_type!(pub struct MetalCaptureError(usize););
raw_value_type!(pub struct MetalCommandBufferError(usize););
raw_value_type!(pub struct MetalCommandBufferErrorOption(usize););
raw_value_type!(pub struct MetalCommandEncoderErrorState(usize););
raw_value_type!(pub struct MetalCompileSymbolVisibility(usize););
raw_value_type!(pub struct MetalCounterSampleBufferError(usize););
raw_value_type!(pub struct MetalCullMode(usize););
raw_value_type!(pub struct MetalCurveBasis(usize););
raw_value_type!(pub struct MetalCurveEndCaps(usize););
raw_value_type!(pub struct MetalCurveType(usize););
raw_value_type!(pub struct MetalDataType(usize););
raw_value_type!(pub struct MetalDepthClipMode(usize););
raw_value_type!(pub struct MetalDeviceLocation(usize););
raw_value_type!(pub struct MetalDispatchType(usize););
raw_value_type!(pub struct MetalDynamicLibraryError(usize););
raw_value_type!(pub struct MetalFeatureSet(usize););
raw_value_type!(pub struct MetalFunctionLogType(usize););
raw_value_type!(pub struct MetalFunctionOptions(usize););
raw_value_type!(pub struct MetalFunctionType(usize););
raw_value_type!(pub struct MetalHeapType(usize););
raw_value_type!(pub struct MetalIndexType(usize););
raw_value_type!(pub struct MetalIoCommandQueueType(usize););
raw_value_type!(pub struct MetalIoCompressionMethod(usize););
raw_value_type!(pub struct MetalIoCompressionStatus(usize););
raw_value_type!(pub struct MetalIoPriority(usize););
raw_value_type!(pub struct MetalIoStatus(usize););
raw_value_type!(pub struct MetalLanguageVersion(usize););
raw_value_type!(pub struct MetalLibraryError(usize););
raw_value_type!(pub struct MetalLibraryOptimizationLevel(usize););
raw_value_type!(pub struct MetalLibraryType(usize););
raw_value_type!(pub struct MetalLogStateError(usize););
raw_value_type!(pub struct MetalMathFloatingPointFunctions(usize););
raw_value_type!(pub struct MetalMathMode(usize););
raw_value_type!(pub struct MetalMatrixLayout(usize););
raw_value_type!(pub struct MetalMotionBorderMode(usize););
raw_value_type!(pub struct MetalMultisampleDepthResolveFilter(usize););
raw_value_type!(pub struct MetalMultisampleStencilResolveFilter(usize););
raw_value_type!(pub struct MetalMutability(usize););
raw_value_type!(pub struct MetalPatchType(usize););
raw_value_type!(pub struct MetalPipelineOption(usize););
raw_value_type!(pub struct MetalPrimitiveTopologyClass(usize););
raw_value_type!(pub struct MetalReadWriteTextureTier(usize););
raw_value_type!(pub struct MetalRenderStages(usize););
raw_value_type!(pub struct MetalResourceUsage(usize););
raw_value_type!(pub struct MetalShaderValidation(usize););
raw_value_type!(pub struct MetalSparsePageSize(usize););
raw_value_type!(pub struct MetalSparseTextureRegionAlignmentMode(usize););
raw_value_type!(pub struct MetalStages(usize););
raw_value_type!(pub struct MetalStepFunction(usize););
raw_value_type!(pub struct MetalStitchedLibraryOptions(usize););
raw_value_type!(pub struct MetalStoreActionOptions(usize););
raw_value_type!(pub struct MetalTensorDataType(usize););
raw_value_type!(pub struct MetalTensorError(usize););
raw_value_type!(pub struct MetalTensorUsage(usize););
raw_value_type!(pub struct MetalTessellationControlPointIndexType(usize););
raw_value_type!(pub struct MetalTessellationFactorFormat(usize););
raw_value_type!(pub struct MetalTessellationFactorStepFunction(usize););
raw_value_type!(pub struct MetalTessellationPartitionMode(usize););
raw_value_type!(pub struct MetalTextureCompressionType(usize););
raw_value_type!(pub struct MetalTextureSparseTier(usize););
raw_value_type!(pub struct MetalTextureSwizzle(usize););
raw_value_type!(pub struct MetalTransformType(usize););
raw_value_type!(pub struct MetalTriangleFillMode(usize););
raw_value_type!(pub struct MetalVertexFormat(usize););
raw_value_type!(pub struct MetalVertexStepFunction(usize););
raw_value_type!(pub struct MetalVisibilityResultMode(usize););
raw_value_type!(pub struct MetalVisibilityResultType(usize););
raw_value_type!(pub struct MetalWinding(usize););
opaque_symbol_handle!(pub struct Metal4Archive;);
opaque_symbol_handle!(pub struct Metal4ArgumentTable;);
opaque_symbol_handle!(pub struct Metal4BinaryFunction;);
opaque_symbol_handle!(pub struct Metal4CommandAllocator;);
opaque_symbol_handle!(pub struct Metal4CommandBuffer;);
opaque_symbol_handle!(pub struct Metal4CommandEncoder;);
opaque_symbol_handle!(pub struct Metal4CommandQueue;);
opaque_symbol_handle!(pub struct Metal4CommitFeedback;);
opaque_symbol_handle!(pub struct Metal4Compiler;);
opaque_symbol_handle!(pub struct Metal4CompilerTask;);
opaque_symbol_handle!(pub struct Metal4ComputeCommandEncoder;);
opaque_symbol_handle!(pub struct Metal4CounterHeap;);
opaque_symbol_handle!(pub struct Metal4FxFrameInterpolator;);
opaque_symbol_handle!(pub struct Metal4FxSpatialScaler;);
opaque_symbol_handle!(pub struct Metal4FxTemporalDenoisedScaler;);
opaque_symbol_handle!(pub struct Metal4FxTemporalScaler;);
opaque_symbol_handle!(pub struct Metal4MachineLearningCommandEncoder;);
opaque_symbol_handle!(pub struct Metal4MachineLearningPipelineState;);
opaque_symbol_handle!(pub struct Metal4PipelineDataSetSerializer;);
opaque_symbol_handle!(pub struct Metal4RenderCommandEncoder;);
opaque_symbol_handle!(pub struct MetalAccelerationStructureCommandEncoder;);
opaque_symbol_handle!(pub struct MetalAllocation;);
opaque_symbol_handle!(pub struct MetalBinding;);
opaque_symbol_handle!(pub struct MetalBufferBinding;);
opaque_symbol_handle!(pub struct MetalCommandBufferEncoderInfo;);
opaque_symbol_handle!(pub struct MetalCommandEncoder;);
opaque_symbol_handle!(pub struct MetalCounter;);
opaque_symbol_handle!(pub struct MetalDrawable;);
opaque_symbol_handle!(pub struct MetalFunctionHandle;);
opaque_symbol_handle!(pub struct MetalFunctionLog;);
opaque_symbol_handle!(pub struct MetalFunctionLogDebugLocation;);
opaque_symbol_handle!(pub struct MetalFunctionStitchingAttribute;);
opaque_symbol_handle!(pub struct MetalFunctionStitchingNode;);
opaque_symbol_handle!(pub struct MetalFxFrameInterpolator;);
opaque_symbol_handle!(pub struct MetalFxFrameInterpolatorBase;);
opaque_symbol_handle!(pub struct MetalFxSpatialScalerBase;);
opaque_symbol_handle!(pub struct MetalFxTemporalDenoisedScaler;);
opaque_symbol_handle!(pub struct MetalFxTemporalDenoisedScalerBase;);
opaque_symbol_handle!(pub struct MetalFxTemporalScalerBase;);
opaque_symbol_handle!(pub struct MetalIndirectComputeCommand;);
opaque_symbol_handle!(pub struct MetalIndirectRenderCommand;);
opaque_symbol_handle!(pub struct MetalIoCommandBuffer;);
opaque_symbol_handle!(pub struct MetalIoCommandQueue;);
opaque_symbol_handle!(pub struct MetalIoFileHandle;);
opaque_symbol_handle!(pub struct MetalIoScratchBuffer;);
opaque_symbol_handle!(pub struct MetalIoScratchBufferAllocator;);
opaque_symbol_handle!(pub struct MetalLogContainer;);
opaque_symbol_handle!(pub struct MetalObjectPayloadBinding;);
opaque_symbol_handle!(pub struct MetalParallelRenderCommandEncoder;);
opaque_symbol_handle!(pub struct MetalRasterizationRateMap;);
opaque_symbol_handle!(pub struct MetalResource;);
opaque_symbol_handle!(pub struct MetalResourceStateCommandEncoder;);
opaque_symbol_handle!(pub struct MetalResourceViewPool;);
opaque_symbol_handle!(pub struct MetalTensorBinding;);
opaque_symbol_handle!(pub struct MetalTextureBinding;);
opaque_symbol_handle!(pub struct MetalTextureViewPool;);
opaque_symbol_handle!(pub struct MetalThreadgroupBinding;);
opaque_symbol_class!(pub struct Metal4AccelerationStructureBoundingBoxGeometryDescriptor => "MTL4AccelerationStructureBoundingBoxGeometryDescriptor";);
opaque_symbol_class!(pub struct Metal4AccelerationStructureCurveGeometryDescriptor => "MTL4AccelerationStructureCurveGeometryDescriptor";);
opaque_symbol_class!(pub struct Metal4AccelerationStructureDescriptor => "MTL4AccelerationStructureDescriptor";);
opaque_symbol_class!(pub struct Metal4AccelerationStructureGeometryDescriptor => "MTL4AccelerationStructureGeometryDescriptor";);
opaque_symbol_class!(pub struct Metal4AccelerationStructureMotionBoundingBoxGeometryDescriptor => "MTL4AccelerationStructureMotionBoundingBoxGeometryDescriptor";);
opaque_symbol_class!(pub struct Metal4AccelerationStructureMotionCurveGeometryDescriptor => "MTL4AccelerationStructureMotionCurveGeometryDescriptor";);
opaque_symbol_class!(pub struct Metal4AccelerationStructureMotionTriangleGeometryDescriptor => "MTL4AccelerationStructureMotionTriangleGeometryDescriptor";);
opaque_symbol_class!(pub struct Metal4AccelerationStructureTriangleGeometryDescriptor => "MTL4AccelerationStructureTriangleGeometryDescriptor";);
opaque_symbol_class!(pub struct Metal4ArgumentTableDescriptor => "MTL4ArgumentTableDescriptor";);
opaque_symbol_class!(pub struct Metal4BinaryFunctionDescriptor => "MTL4BinaryFunctionDescriptor";);
opaque_symbol_class!(pub struct Metal4CommandAllocatorDescriptor => "MTL4CommandAllocatorDescriptor";);
opaque_symbol_class!(pub struct Metal4CommandBufferOptions => "MTL4CommandBufferOptions";);
opaque_symbol_class!(pub struct Metal4CommandQueueDescriptor => "MTL4CommandQueueDescriptor";);
opaque_symbol_class!(pub struct Metal4CommitOptions => "MTL4CommitOptions";);
opaque_symbol_class!(pub struct Metal4CompilerDescriptor => "MTL4CompilerDescriptor";);
opaque_symbol_class!(pub struct Metal4CompilerTaskOptions => "MTL4CompilerTaskOptions";);
opaque_symbol_class!(pub struct Metal4ComputePipelineDescriptor => "MTL4ComputePipelineDescriptor";);
opaque_symbol_class!(pub struct Metal4CounterHeapDescriptor => "MTL4CounterHeapDescriptor";);
opaque_symbol_class!(pub struct Metal4FunctionDescriptor => "MTL4FunctionDescriptor";);
opaque_symbol_class!(pub struct Metal4IndirectInstanceAccelerationStructureDescriptor => "MTL4IndirectInstanceAccelerationStructureDescriptor";);
opaque_symbol_class!(pub struct Metal4InstanceAccelerationStructureDescriptor => "MTL4InstanceAccelerationStructureDescriptor";);
opaque_symbol_class!(pub struct Metal4LibraryDescriptor => "MTL4LibraryDescriptor";);
opaque_symbol_class!(pub struct Metal4LibraryFunctionDescriptor => "MTL4LibraryFunctionDescriptor";);
opaque_symbol_class!(pub struct Metal4MachineLearningPipelineDescriptor => "MTL4MachineLearningPipelineDescriptor";);
opaque_symbol_class!(pub struct Metal4MachineLearningPipelineReflection => "MTL4MachineLearningPipelineReflection";);
opaque_symbol_class!(pub struct Metal4MeshRenderPipelineDescriptor => "MTL4MeshRenderPipelineDescriptor";);
opaque_symbol_class!(pub struct Metal4PipelineDataSetSerializerDescriptor => "MTL4PipelineDataSetSerializerDescriptor";);
opaque_symbol_class!(pub struct Metal4PipelineDescriptor => "MTL4PipelineDescriptor";);
opaque_symbol_class!(pub struct Metal4PipelineOptions => "MTL4PipelineOptions";);
opaque_symbol_class!(pub struct Metal4PipelineStageDynamicLinkingDescriptor => "MTL4PipelineStageDynamicLinkingDescriptor";);
opaque_symbol_class!(pub struct Metal4PrimitiveAccelerationStructureDescriptor => "MTL4PrimitiveAccelerationStructureDescriptor";);
opaque_symbol_class!(pub struct Metal4RenderPassDescriptor => "MTL4RenderPassDescriptor";);
opaque_symbol_class!(pub struct Metal4RenderPipelineBinaryFunctionsDescriptor => "MTL4RenderPipelineBinaryFunctionsDescriptor";);
opaque_symbol_class!(pub struct Metal4RenderPipelineColorAttachmentDescriptor => "MTL4RenderPipelineColorAttachmentDescriptor";);
opaque_symbol_class!(pub struct Metal4RenderPipelineColorAttachmentDescriptorArray => "MTL4RenderPipelineColorAttachmentDescriptorArray";);
opaque_symbol_class!(pub struct Metal4RenderPipelineDescriptor => "MTL4RenderPipelineDescriptor";);
opaque_symbol_class!(pub struct Metal4RenderPipelineDynamicLinkingDescriptor => "MTL4RenderPipelineDynamicLinkingDescriptor";);
opaque_symbol_class!(pub struct Metal4SpecializedFunctionDescriptor => "MTL4SpecializedFunctionDescriptor";);
opaque_symbol_class!(pub struct Metal4StaticLinkingDescriptor => "MTL4StaticLinkingDescriptor";);
opaque_symbol_class!(pub struct Metal4StitchedFunctionDescriptor => "MTL4StitchedFunctionDescriptor";);
opaque_symbol_class!(pub struct Metal4TileRenderPipelineDescriptor => "MTL4TileRenderPipelineDescriptor";);
opaque_symbol_class!(pub struct MetalAccelerationStructureBoundingBoxGeometryDescriptor => "MTLAccelerationStructureBoundingBoxGeometryDescriptor";);
opaque_symbol_class!(pub struct MetalAccelerationStructureCurveGeometryDescriptor => "MTLAccelerationStructureCurveGeometryDescriptor";);
opaque_symbol_class!(pub struct MetalAccelerationStructureDescriptor => "MTLAccelerationStructureDescriptor";);
opaque_symbol_class!(pub struct MetalAccelerationStructureGeometryDescriptor => "MTLAccelerationStructureGeometryDescriptor";);
opaque_symbol_class!(pub struct MetalAccelerationStructureMotionBoundingBoxGeometryDescriptor => "MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor";);
opaque_symbol_class!(pub struct MetalAccelerationStructureMotionCurveGeometryDescriptor => "MTLAccelerationStructureMotionCurveGeometryDescriptor";);
opaque_symbol_class!(pub struct MetalAccelerationStructureMotionTriangleGeometryDescriptor => "MTLAccelerationStructureMotionTriangleGeometryDescriptor";);
opaque_symbol_class!(pub struct MetalAccelerationStructurePassDescriptor => "MTLAccelerationStructurePassDescriptor";);
opaque_symbol_class!(pub struct MetalAccelerationStructurePassSampleBufferAttachmentDescriptor => "MTLAccelerationStructurePassSampleBufferAttachmentDescriptor";);
opaque_symbol_class!(pub struct MetalAccelerationStructurePassSampleBufferAttachmentDescriptorArray => "MTLAccelerationStructurePassSampleBufferAttachmentDescriptorArray";);
opaque_symbol_class!(pub struct MetalAccelerationStructureTriangleGeometryDescriptor => "MTLAccelerationStructureTriangleGeometryDescriptor";);
opaque_symbol_class!(pub struct MetalArchitecture => "MTLArchitecture";);
opaque_symbol_class!(pub struct MetalArgument => "MTLArgument";);
opaque_symbol_class!(pub struct MetalArrayType => "MTLArrayType";);
opaque_symbol_class!(pub struct MetalAttribute => "MTLAttribute";);
opaque_symbol_class!(pub struct MetalAttributeDescriptor => "MTLAttributeDescriptor";);
opaque_symbol_class!(pub struct MetalAttributeDescriptorArray => "MTLAttributeDescriptorArray";);
opaque_symbol_class!(pub struct MetalBinaryArchiveDescriptor => "MTLBinaryArchiveDescriptor";);
opaque_symbol_class!(pub struct MetalBlitPassDescriptor => "MTLBlitPassDescriptor";);
opaque_symbol_class!(pub struct MetalBlitPassSampleBufferAttachmentDescriptor => "MTLBlitPassSampleBufferAttachmentDescriptor";);
opaque_symbol_class!(pub struct MetalBlitPassSampleBufferAttachmentDescriptorArray => "MTLBlitPassSampleBufferAttachmentDescriptorArray";);
opaque_symbol_class!(pub struct MetalBufferLayoutDescriptor => "MTLBufferLayoutDescriptor";);
opaque_symbol_class!(pub struct MetalBufferLayoutDescriptorArray => "MTLBufferLayoutDescriptorArray";);
opaque_symbol_class!(pub struct MetalCommandBufferDescriptor => "MTLCommandBufferDescriptor";);
opaque_symbol_class!(pub struct MetalCommandQueueDescriptor => "MTLCommandQueueDescriptor";);
opaque_symbol_class!(pub struct MetalCompileOptions => "MTLCompileOptions";);
opaque_symbol_class!(pub struct MetalComputePassDescriptor => "MTLComputePassDescriptor";);
opaque_symbol_class!(pub struct MetalComputePassSampleBufferAttachmentDescriptor => "MTLComputePassSampleBufferAttachmentDescriptor";);
opaque_symbol_class!(pub struct MetalComputePassSampleBufferAttachmentDescriptorArray => "MTLComputePassSampleBufferAttachmentDescriptorArray";);
opaque_symbol_class!(pub struct MetalComputePipelineReflection => "MTLComputePipelineReflection";);
opaque_symbol_class!(pub struct MetalCounterSampleBufferDescriptor => "MTLCounterSampleBufferDescriptor";);
opaque_symbol_class!(pub struct MetalFunctionConstant => "MTLFunctionConstant";);
opaque_symbol_class!(pub struct MetalFunctionConstantValues => "MTLFunctionConstantValues";);
opaque_symbol_class!(pub struct MetalFunctionDescriptor => "MTLFunctionDescriptor";);
opaque_symbol_class!(pub struct MetalFunctionReflection => "MTLFunctionReflection";);
opaque_symbol_class!(pub struct MetalFunctionStitchingAttributeAlwaysInline => "MTLFunctionStitchingAttributeAlwaysInline";);
opaque_symbol_class!(pub struct MetalFunctionStitchingFunctionNode => "MTLFunctionStitchingFunctionNode";);
opaque_symbol_class!(pub struct MetalFunctionStitchingGraph => "MTLFunctionStitchingGraph";);
opaque_symbol_class!(pub struct MetalFunctionStitchingInputNode => "MTLFunctionStitchingInputNode";);
opaque_symbol_class!(pub struct MetalFxFrameInterpolatorDescriptor => "MTLFXFrameInterpolatorDescriptor";);
opaque_symbol_class!(pub struct MetalFxTemporalDenoisedScalerDescriptor => "MTLFXTemporalDenoisedScalerDescriptor";);
opaque_symbol_class!(pub struct MetalHeapDescriptor => "MTLHeapDescriptor";);
opaque_symbol_class!(pub struct MetalIndirectCommandBufferDescriptor => "MTLIndirectCommandBufferDescriptor";);
opaque_symbol_class!(pub struct MetalIndirectInstanceAccelerationStructureDescriptor => "MTLIndirectInstanceAccelerationStructureDescriptor";);
opaque_symbol_class!(pub struct MetalInstanceAccelerationStructureDescriptor => "MTLInstanceAccelerationStructureDescriptor";);
opaque_symbol_class!(pub struct MetalIntersectionFunctionDescriptor => "MTLIntersectionFunctionDescriptor";);
opaque_symbol_class!(pub struct MetalIntersectionFunctionTableDescriptor => "MTLIntersectionFunctionTableDescriptor";);
opaque_symbol_class!(pub struct MetalIoCommandQueueDescriptor => "MTLIOCommandQueueDescriptor";);
opaque_symbol_class!(pub struct MetalLinkedFunctions => "MTLLinkedFunctions";);
opaque_symbol_class!(pub struct MetalLogStateDescriptor => "MTLLogStateDescriptor";);
opaque_symbol_class!(pub struct MetalLogicalToPhysicalColorAttachmentMap => "MTLLogicalToPhysicalColorAttachmentMap";);
opaque_symbol_class!(pub struct MetalMeshRenderPipelineDescriptor => "MTLMeshRenderPipelineDescriptor";);
opaque_symbol_class!(pub struct MetalMotionKeyframeData => "MTLMotionKeyframeData";);
opaque_symbol_class!(pub struct MetalPipelineBufferDescriptor => "MTLPipelineBufferDescriptor";);
opaque_symbol_class!(pub struct MetalPipelineBufferDescriptorArray => "MTLPipelineBufferDescriptorArray";);
opaque_symbol_class!(pub struct MetalPointerType => "MTLPointerType";);
opaque_symbol_class!(pub struct MetalPrimitiveAccelerationStructureDescriptor => "MTLPrimitiveAccelerationStructureDescriptor";);
opaque_symbol_class!(pub struct MetalRasterizationRateLayerArray => "MTLRasterizationRateLayerArray";);
opaque_symbol_class!(pub struct MetalRasterizationRateLayerDescriptor => "MTLRasterizationRateLayerDescriptor";);
opaque_symbol_class!(pub struct MetalRasterizationRateMapDescriptor => "MTLRasterizationRateMapDescriptor";);
opaque_symbol_class!(pub struct MetalRasterizationRateSampleArray => "MTLRasterizationRateSampleArray";);
opaque_symbol_class!(pub struct MetalRenderPassAttachmentDescriptor => "MTLRenderPassAttachmentDescriptor";);
opaque_symbol_class!(pub struct MetalRenderPassColorAttachmentDescriptor => "MTLRenderPassColorAttachmentDescriptor";);
opaque_symbol_class!(pub struct MetalRenderPassColorAttachmentDescriptorArray => "MTLRenderPassColorAttachmentDescriptorArray";);
opaque_symbol_class!(pub struct MetalRenderPassDepthAttachmentDescriptor => "MTLRenderPassDepthAttachmentDescriptor";);
opaque_symbol_class!(pub struct MetalRenderPassDescriptor => "MTLRenderPassDescriptor";);
opaque_symbol_class!(pub struct MetalRenderPassSampleBufferAttachmentDescriptor => "MTLRenderPassSampleBufferAttachmentDescriptor";);
opaque_symbol_class!(pub struct MetalRenderPassSampleBufferAttachmentDescriptorArray => "MTLRenderPassSampleBufferAttachmentDescriptorArray";);
opaque_symbol_class!(pub struct MetalRenderPassStencilAttachmentDescriptor => "MTLRenderPassStencilAttachmentDescriptor";);
opaque_symbol_class!(pub struct MetalRenderPipelineColorAttachmentDescriptorArray => "MTLRenderPipelineColorAttachmentDescriptorArray";);
opaque_symbol_class!(pub struct MetalRenderPipelineFunctionsDescriptor => "MTLRenderPipelineFunctionsDescriptor";);
opaque_symbol_class!(pub struct MetalRenderPipelineReflection => "MTLRenderPipelineReflection";);
opaque_symbol_class!(pub struct MetalResidencySetDescriptor => "MTLResidencySetDescriptor";);
opaque_symbol_class!(pub struct MetalResourceStatePassDescriptor => "MTLResourceStatePassDescriptor";);
opaque_symbol_class!(pub struct MetalResourceStatePassSampleBufferAttachmentDescriptor => "MTLResourceStatePassSampleBufferAttachmentDescriptor";);
opaque_symbol_class!(pub struct MetalResourceStatePassSampleBufferAttachmentDescriptorArray => "MTLResourceStatePassSampleBufferAttachmentDescriptorArray";);
opaque_symbol_class!(pub struct MetalResourceViewPoolDescriptor => "MTLResourceViewPoolDescriptor";);
opaque_symbol_class!(pub struct MetalSharedEventHandle => "MTLSharedEventHandle";);
opaque_symbol_class!(pub struct MetalSharedEventListener => "MTLSharedEventListener";);
opaque_symbol_class!(pub struct MetalSharedTextureHandle => "MTLSharedTextureHandle";);
opaque_symbol_class!(pub struct MetalStageInputOutputDescriptor => "MTLStageInputOutputDescriptor";);
opaque_symbol_class!(pub struct MetalStitchedLibraryDescriptor => "MTLStitchedLibraryDescriptor";);
opaque_symbol_class!(pub struct MetalStructMember => "MTLStructMember";);
opaque_symbol_class!(pub struct MetalStructType => "MTLStructType";);
opaque_symbol_class!(pub struct MetalTensorDescriptor => "MTLTensorDescriptor";);
opaque_symbol_class!(pub struct MetalTensorExtents => "MTLTensorExtents";);
opaque_symbol_class!(pub struct MetalTensorReferenceType => "MTLTensorReferenceType";);
opaque_symbol_class!(pub struct MetalTextureReferenceType => "MTLTextureReferenceType";);
opaque_symbol_class!(pub struct MetalTextureViewDescriptor => "MTLTextureViewDescriptor";);
opaque_symbol_class!(pub struct MetalTileRenderPipelineColorAttachmentDescriptorArray => "MTLTileRenderPipelineColorAttachmentDescriptorArray";);
opaque_symbol_class!(pub struct MetalType => "MTLType";);
opaque_symbol_class!(pub struct MetalVertexAttribute => "MTLVertexAttribute";);
opaque_symbol_class!(pub struct MetalVertexAttributeDescriptor => "MTLVertexAttributeDescriptor";);
opaque_symbol_class!(pub struct MetalVertexAttributeDescriptorArray => "MTLVertexAttributeDescriptorArray";);
opaque_symbol_class!(pub struct MetalVertexBufferLayoutDescriptor => "MTLVertexBufferLayoutDescriptor";);
opaque_symbol_class!(pub struct MetalVertexBufferLayoutDescriptorArray => "MTLVertexBufferLayoutDescriptorArray";);
opaque_symbol_class!(pub struct MetalVertexDescriptor => "MTLVertexDescriptor";);
opaque_symbol_class!(pub struct MetalVisibleFunctionTableDescriptor => "MTLVisibleFunctionTableDescriptor";);
