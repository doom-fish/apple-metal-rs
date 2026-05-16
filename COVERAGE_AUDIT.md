# apple-metal-rs coverage audit (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 430
VERIFIED: 48
GAPS: 377
EXEMPT: 5
COVERAGE_PCT: 11.29%

Scope: top-level public symbols from `Metal.framework` + `MetalFX.framework` headers, filtered for macOS availability. Deprecated top-level APIs remain listed as EXEMPT per the audit instructions.

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| `MTLAccelerationStructure` | protocol | `MTLAccelerationStructure.h` | `AccelerationStructure; MetalDevice::new_acceleration_structure_with_size; Heap::new_acceleration_structure_with_size; ComputeCommandEncoder::set_acceleration_structure` |
| `MTLArgumentEncoder` | protocol | `MTLArgumentEncoder.h` | `ArgumentEncoder; MetalFunction::new_argument_encoder` |
| `MTLBinaryArchive` | protocol | `MTLBinaryArchive.h` | `BinaryArchive; MetalDevice::new_binary_archive` |
| `MTLBlitCommandEncoder` | protocol | `MTLBlitCommandEncoder.h` | `BlitCommandEncoder; CommandBuffer::new_blit_command_encoder; CommandBuffer::blit_copy_buffer` |
| `MTLBuffer` | protocol | `MTLBuffer.h` | `MetalBuffer; MetalDevice::new_buffer; Heap::new_buffer` |
| `MTLCPUCacheMode` | enum | `MTLResource.h` | `cpu_cache_mode module` |
| `MTLCaptureDestination` | enum | `MTLCaptureManager.h` | `capture_destination module; CaptureManager::supports_destination` |
| `MTLCaptureManager` | interface | `MTLCaptureManager.h` | `CaptureManager::shared; CaptureManager::new_capture_scope_with_*` |
| `MTLCaptureScope` | protocol | `MTLCaptureScope.h` | `CaptureScope; CaptureManager::new_capture_scope_with_*` |
| `MTLCommandBuffer` | protocol | `MTLCommandBuffer.h` | `CommandBuffer; CommandQueue::{new_command_buffer,new_command_buffer_with_unretained_references}` |
| `MTLCommandBufferStatus` | enum | `MTLCommandBuffer.h` | `command_buffer_status module; CommandBuffer::status` |
| `MTLCommandQueue` | protocol | `MTLCommandQueue.h` | `CommandQueue; MetalDevice::{new_command_queue,new_command_queue_with_max_command_buffer_count,new_command_queue_with_log_state}` |
| `MTLComputeCommandEncoder` | protocol | `MTLComputeCommandEncoder.h` | `ComputeCommandEncoder; CommandBuffer::new_compute_command_encoder; CommandBuffer::dispatch_compute_1d` |
| `MTLComputePipelineState` | protocol | `MTLComputePipeline.h` | `ComputePipelineState; MetalDevice::new_compute_pipeline_state` |
| `MTLCounterSampleBuffer` | protocol | `MTLCounters.h` | `CounterSampleBuffer; MetalDevice::new_counter_sample_buffer; BlitCommandEncoder::sample_counters` |
| `MTLCounterSamplingPoint` | enum | `MTLDevice.h` | `counter_sampling_point module; MetalDevice::supports_counter_sampling` |
| `MTLCounterSet` | protocol | `MTLCounters.h` | `MetalDevice::counter_set_names; MetalDevice::new_counter_sample_buffer` |
| `MTLCreateSystemDefaultDevice` | function | `MTLDevice.h` | `MetalDevice::system_default` |
| `MTLDevice` | protocol | `MTLDevice.h` | `MetalDevice` |
| `MTLDynamicLibrary` | protocol | `MTLDynamicLibrary.h` | `DynamicLibrary; MetalDevice::{new_dynamic_library_with_source,load_dynamic_library}` |
| `MTLEvent` | protocol | `MTLEvent.h` | `Event; CommandBuffer::{encode_wait_for_event,encode_signal_event}` |
| `MTLFence` | protocol | `MTLFence.h` | `Fence; MetalDevice::new_fence; encoder wait/update methods` |
| `MTLFunction` | protocol | `MTLLibrary.h` | `MetalFunction; MetalLibrary::new_function` |
| `MTLGPUFamily` | enum | `MTLDevice.h` | `gpu_family module; MetalDevice::supports_family` |
| `MTLHazardTrackingMode` | enum | `MTLResource.h` | `hazard_tracking_mode module` |
| `MTLHeap` | protocol | `MTLHeap.h` | `Heap; MetalDevice::new_heap` |
| `MTLIndirectCommandBuffer` | protocol | `MTLIndirectCommandBuffer.h` | `IndirectCommandBuffer; MetalDevice::new_indirect_command_buffer` |
| `MTLIndirectCommandType` | enum | `MTLIndirectCommandBuffer.h` | `indirect_command_type module; MetalDevice::new_indirect_command_buffer` |
| `MTLIntersectionFunctionSignature` | enum | `MTLIntersectionFunctionTable.h` | `intersection_function_signature module; IntersectionFunctionTable::set_opaque_triangle_intersection_function` |
| `MTLIntersectionFunctionTable` | protocol | `MTLIntersectionFunctionTable.h` | `IntersectionFunctionTable; ComputeCommandEncoder::set_intersection_function_table; ComputePipelineState::new_intersection_function_table` |
| `MTLLibrary` | protocol | `MTLLibrary.h` | `MetalLibrary; MetalDevice::new_library_with_source` |
| `MTLLoadAction` | enum | `MTLRenderPass.h` | `load_action module; CommandBuffer::new_render_command_encoder` |
| `MTLLogLevel` | enum | `MTLLogState.h` | `log_level module; MetalDevice::new_log_state` |
| `MTLLogState` | protocol | `MTLLogState.h` | `LogState; MetalDevice::new_log_state; MetalDevice::new_command_queue_with_log_state` |
| `MTLPixelFormat` | enum | `MTLPixelFormat.h` | `pixel_format module; TextureDescriptor; MetalTexture::pixel_format` |
| `MTLPrimitiveType` | enum | `MTLRenderCommandEncoder.h` | `primitive_type module; RenderCommandEncoder::draw_primitives` |
| `MTLPurgeableState` | enum | `MTLResource.h` | `purgeable_state module; Heap::set_purgeable_state` |
| `MTLRenderCommandEncoder` | protocol | `MTLRenderCommandEncoder.h` | `RenderCommandEncoder; CommandBuffer::new_render_command_encoder` |
| `MTLRenderPipelineState` | protocol | `MTLRenderPipeline.h` | `RenderPipelineState; MetalDevice::new_render_pipeline_state` |
| `MTLResidencySet` | protocol | `MTLResidencySet.h` | `ResidencySet; MetalDevice::new_residency_set; CommandQueue::{add_residency_set,remove_residency_set}` |
| `MTLResourceOptions` | enum | `MTLResource.h` | `resource_options module; MetalDevice::new_buffer; Heap::new_buffer` |
| `MTLSharedEvent` | protocol | `MTLEvent.h` | `Event; MetalDevice::new_shared_event` |
| `MTLStorageMode` | enum | `MTLResource.h` | `storage_mode module; TextureDescriptor; MetalDevice::new_heap` |
| `MTLStoreAction` | enum | `MTLRenderPass.h` | `store_action module; CommandBuffer::new_render_command_encoder` |
| `MTLTexture` | protocol | `MTLTexture.h` | `MetalTexture; MetalDevice::new_texture; IOSurfaceMetalExt::create_metal_texture` |
| `MTLTextureDescriptor` | interface | `MTLTexture.h` | `TextureDescriptor; MetalDevice::new_texture; Heap::new_texture` |
| `MTLTextureUsage` | enum | `MTLTexture.h` | `texture_usage module; TextureDescriptor; MetalTexture::usage` |
| `MTLVisibleFunctionTable` | protocol | `MTLVisibleFunctionTable.h` | `VisibleFunctionTable; ComputeCommandEncoder::set_visible_function_table; ComputePipelineState::new_visible_function_table` |

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |
| `MTL4AccelerationStructureBoundingBoxGeometryDescriptor` | interface | `MTL4AccelerationStructure.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4AccelerationStructureCurveGeometryDescriptor` | interface | `MTL4AccelerationStructure.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4AccelerationStructureDescriptor` | interface | `MTL4AccelerationStructure.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4AccelerationStructureGeometryDescriptor` | interface | `MTL4AccelerationStructure.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4AccelerationStructureMotionBoundingBoxGeometryDescriptor` | interface | `MTL4AccelerationStructure.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4AccelerationStructureMotionCurveGeometryDescriptor` | interface | `MTL4AccelerationStructure.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4AccelerationStructureMotionTriangleGeometryDescriptor` | interface | `MTL4AccelerationStructure.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4AccelerationStructureTriangleGeometryDescriptor` | interface | `MTL4AccelerationStructure.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4AlphaToCoverageState` | enum | `MTL4PipelineState.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4AlphaToOneState` | enum | `MTL4PipelineState.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4Archive` | protocol | `MTL4Archive.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4ArgumentTable` | protocol | `MTL4ArgumentTable.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4ArgumentTableDescriptor` | interface | `MTL4ArgumentTable.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4BinaryFunction` | protocol | `MTL4BinaryFunction.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4BinaryFunctionDescriptor` | interface | `MTL4BinaryFunctionDescriptor.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4BinaryFunctionOptions` | enum | `MTL4BinaryFunctionDescriptor.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4BlendState` | enum | `MTL4PipelineState.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4CommandAllocator` | protocol | `MTL4CommandAllocator.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4CommandAllocatorDescriptor` | interface | `MTL4CommandAllocator.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4CommandBuffer` | protocol | `MTL4CommandBuffer.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4CommandBufferOptions` | interface | `MTL4CommandBuffer.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4CommandEncoder` | protocol | `MTL4CommandEncoder.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4CommandQueue` | protocol | `MTL4CommandQueue.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4CommandQueueDescriptor` | interface | `MTL4CommandQueue.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4CommandQueueError` | enum | `MTL4CommandQueue.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4CommandQueueErrorDomain` | const | `MTL4CommandQueue.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4CommitFeedback` | protocol | `MTL4CommitFeedback.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4CommitOptions` | interface | `MTL4CommandQueue.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4Compiler` | protocol | `MTL4Compiler.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4CompilerDescriptor` | interface | `MTL4Compiler.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4CompilerTask` | protocol | `MTL4CompilerTask.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4CompilerTaskOptions` | interface | `MTL4Compiler.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4CompilerTaskStatus` | enum | `MTL4CompilerTask.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4ComputeCommandEncoder` | protocol | `MTL4ComputeCommandEncoder.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4ComputePipelineDescriptor` | interface | `MTL4ComputePipeline.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4CounterHeap` | protocol | `MTL4Counters.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4CounterHeapDescriptor` | interface | `MTL4Counters.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4CounterHeapType` | enum | `MTL4Counters.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4FXFrameInterpolator` | protocol | `MTL4FXFrameInterpolator.h` | MetalFX Metal 4 surface is out of scope for apple-metal 0.6.0. |
| `MTL4FXSpatialScaler` | protocol | `MTL4FXSpatialScaler.h` | MetalFX Metal 4 surface is out of scope for apple-metal 0.6.0. |
| `MTL4FXTemporalDenoisedScaler` | protocol | `MTL4FXTemporalDenoisedScaler.h` | MetalFX Metal 4 surface is out of scope for apple-metal 0.6.0. |
| `MTL4FXTemporalScaler` | protocol | `MTL4FXTemporalScaler.h` | MetalFX Metal 4 surface is out of scope for apple-metal 0.6.0. |
| `MTL4FunctionDescriptor` | interface | `MTL4FunctionDescriptor.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4IndirectCommandBufferSupportState` | enum | `MTL4PipelineState.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4IndirectInstanceAccelerationStructureDescriptor` | interface | `MTL4AccelerationStructure.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4InstanceAccelerationStructureDescriptor` | interface | `MTL4AccelerationStructure.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4LibraryDescriptor` | interface | `MTL4LibraryDescriptor.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4LibraryFunctionDescriptor` | interface | `MTL4LibraryFunctionDescriptor.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4LogicalToPhysicalColorAttachmentMappingState` | enum | `MTL4RenderPipeline.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4MachineLearningCommandEncoder` | protocol | `MTL4MachineLearningCommandEncoder.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4MachineLearningPipelineDescriptor` | interface | `MTL4MachineLearningPipeline.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4MachineLearningPipelineReflection` | interface | `MTL4MachineLearningPipeline.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4MachineLearningPipelineState` | protocol | `MTL4MachineLearningPipeline.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4MeshRenderPipelineDescriptor` | interface | `MTL4MeshRenderPipeline.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4PipelineDataSetSerializer` | protocol | `MTL4PipelineDataSetSerializer.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4PipelineDataSetSerializerConfiguration` | enum | `MTL4PipelineDataSetSerializer.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4PipelineDataSetSerializerDescriptor` | interface | `MTL4PipelineDataSetSerializer.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4PipelineDescriptor` | interface | `MTL4PipelineState.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4PipelineOptions` | interface | `MTL4PipelineState.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4PipelineStageDynamicLinkingDescriptor` | interface | `MTL4LinkingDescriptor.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4PrimitiveAccelerationStructureDescriptor` | interface | `MTL4AccelerationStructure.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4RenderCommandEncoder` | protocol | `MTL4RenderCommandEncoder.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4RenderEncoderOptions` | enum | `MTL4RenderCommandEncoder.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4RenderPassDescriptor` | interface | `MTL4RenderPass.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4RenderPipelineBinaryFunctionsDescriptor` | interface | `MTL4RenderPipeline.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4RenderPipelineColorAttachmentDescriptor` | interface | `MTL4RenderPipeline.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4RenderPipelineColorAttachmentDescriptorArray` | interface | `MTL4RenderPipeline.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4RenderPipelineDescriptor` | interface | `MTL4RenderPipeline.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4RenderPipelineDynamicLinkingDescriptor` | interface | `MTL4LinkingDescriptor.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4ShaderReflection` | enum | `MTL4PipelineState.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4SpecializedFunctionDescriptor` | interface | `MTL4SpecializedFunctionDescriptor.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4StaticLinkingDescriptor` | interface | `MTL4LinkingDescriptor.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4StitchedFunctionDescriptor` | interface | `MTL4StitchedFunctionDescriptor.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4TileRenderPipelineDescriptor` | interface | `MTL4TileRenderPipeline.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4TimestampGranularity` | enum | `MTL4Counters.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTL4VisibilityOptions` | enum | `MTL4CommandEncoder.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTLAccelerationStructureBoundingBoxGeometryDescriptor` | interface | `MTLAccelerationStructure.h` | Ray-tracing build/update/descriptor surface is deferred; crate only wraps handles and basic binding. |
| `MTLAccelerationStructureCommandEncoder` | protocol | `MTLAccelerationStructureCommandEncoder.h` | Ray-tracing build/update/descriptor surface is deferred; crate only wraps handles and basic binding. |
| `MTLAccelerationStructureCurveGeometryDescriptor` | interface | `MTLAccelerationStructure.h` | Ray-tracing build/update/descriptor surface is deferred; crate only wraps handles and basic binding. |
| `MTLAccelerationStructureDescriptor` | interface | `MTLAccelerationStructure.h` | Ray-tracing build/update/descriptor surface is deferred; crate only wraps handles and basic binding. |
| `MTLAccelerationStructureGeometryDescriptor` | interface | `MTLAccelerationStructure.h` | Ray-tracing build/update/descriptor surface is deferred; crate only wraps handles and basic binding. |
| `MTLAccelerationStructureInstanceDescriptorType` | enum | `MTLAccelerationStructure.h` | Ray-tracing build/update/descriptor surface is deferred; crate only wraps handles and basic binding. |
| `MTLAccelerationStructureInstanceOptions` | enum | `MTLAccelerationStructure.h` | Ray-tracing build/update/descriptor surface is deferred; crate only wraps handles and basic binding. |
| `MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor` | interface | `MTLAccelerationStructure.h` | Ray-tracing build/update/descriptor surface is deferred; crate only wraps handles and basic binding. |
| `MTLAccelerationStructureMotionCurveGeometryDescriptor` | interface | `MTLAccelerationStructure.h` | Ray-tracing build/update/descriptor surface is deferred; crate only wraps handles and basic binding. |
| `MTLAccelerationStructureMotionTriangleGeometryDescriptor` | interface | `MTLAccelerationStructure.h` | Ray-tracing build/update/descriptor surface is deferred; crate only wraps handles and basic binding. |
| `MTLAccelerationStructurePassDescriptor` | interface | `MTLAccelerationStructureCommandEncoder.h` | Ray-tracing build/update/descriptor surface is deferred; crate only wraps handles and basic binding. |
| `MTLAccelerationStructurePassSampleBufferAttachmentDescriptor` | interface | `MTLAccelerationStructureCommandEncoder.h` | Ray-tracing build/update/descriptor surface is deferred; crate only wraps handles and basic binding. |
| `MTLAccelerationStructurePassSampleBufferAttachmentDescriptorArray` | interface | `MTLAccelerationStructureCommandEncoder.h` | Ray-tracing build/update/descriptor surface is deferred; crate only wraps handles and basic binding. |
| `MTLAccelerationStructureRefitOptions` | enum | `MTLAccelerationStructure.h` | Ray-tracing build/update/descriptor surface is deferred; crate only wraps handles and basic binding. |
| `MTLAccelerationStructureTriangleGeometryDescriptor` | interface | `MTLAccelerationStructure.h` | Ray-tracing build/update/descriptor surface is deferred; crate only wraps handles and basic binding. |
| `MTLAccelerationStructureUsage` | enum | `MTLAccelerationStructure.h` | Ray-tracing build/update/descriptor surface is deferred; crate only wraps handles and basic binding. |
| `MTLAllocation` | protocol | `MTLAllocation.h` | Base-resource / newer utility family is not wrapped in 0.6.0. |
| `MTLArchitecture` | interface | `MTLDevice.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLArgumentBuffersTier` | enum | `MTLDevice.h` | Argument/reflection metadata surface is not wrapped yet. |
| `MTLArgumentDescriptor` | interface | `MTLDevice.h` | Argument/reflection metadata surface is not wrapped yet. |
| `MTLArrayType` | interface | `MTLArgument.h` | Argument/reflection metadata surface is not wrapped yet. |
| `MTLAttribute` | interface | `MTLLibrary.h` | Vertex/stage-input descriptor and reflection metadata are not wrapped yet. |
| `MTLAttributeDescriptor` | interface | `MTLStageInputOutputDescriptor.h` | Vertex/stage-input descriptor and reflection metadata are not wrapped yet. |
| `MTLAttributeDescriptorArray` | interface | `MTLStageInputOutputDescriptor.h` | Vertex/stage-input descriptor and reflection metadata are not wrapped yet. |
| `MTLAttributeFormat` | enum | `MTLStageInputOutputDescriptor.h` | Vertex/stage-input descriptor and reflection metadata are not wrapped yet. |
| `MTLAutoreleasedComputePipelineReflection` | typedef | `MTLLibrary.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLAutoreleasedRenderPipelineReflection` | typedef | `MTLLibrary.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLBarrierScope` | enum | `MTLCommandEncoder.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLBinaryArchiveDescriptor` | interface | `MTLBinaryArchive.h` | Descriptor/configuration surface is kept internal to the Swift bridge. |
| `MTLBinaryArchiveDomain` | const | `MTLBinaryArchive.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLBinaryArchiveError` | enum | `MTLBinaryArchive.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLBinding` | protocol | `MTLArgument.h` | Argument/reflection metadata surface is not wrapped yet. |
| `MTLBindingAccess` | enum | `MTLArgument.h` | Argument/reflection metadata surface is not wrapped yet. |
| `MTLBindingType` | enum | `MTLArgument.h` | Argument/reflection metadata surface is not wrapped yet. |
| `MTLBlendFactor` | enum | `MTLRenderPipeline.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLBlendOperation` | enum | `MTLRenderPipeline.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLBlitOption` | enum | `MTLBlitCommandEncoder.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLBlitPassDescriptor` | interface | `MTLBlitPass.h` | Pass-descriptor family is not exposed; crate keeps a simplified headless path. |
| `MTLBlitPassSampleBufferAttachmentDescriptor` | interface | `MTLBlitPass.h` | Pass-descriptor family is not exposed; crate keeps a simplified headless path. |
| `MTLBlitPassSampleBufferAttachmentDescriptorArray` | interface | `MTLBlitPass.h` | Pass-descriptor family is not exposed; crate keeps a simplified headless path. |
| `MTLBufferBinding` | protocol | `MTLArgument.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLBufferLayoutDescriptor` | interface | `MTLStageInputOutputDescriptor.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLBufferLayoutDescriptorArray` | interface | `MTLStageInputOutputDescriptor.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLBufferSparseTier` | enum | `MTLResource.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLCaptureError` | enum | `MTLCaptureManager.h` | Capture configuration beyond scopes and destination checks is not wrapped. |
| `MTLCaptureErrorDomain` | const | `MTLCaptureManager.h` | Capture configuration beyond scopes and destination checks is not wrapped. |
| `MTLColorWriteMask` | enum | `MTLRenderPipeline.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLCommandBufferDescriptor` | interface | `MTLCommandBuffer.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLCommandBufferEncoderInfo` | protocol | `MTLCommandBuffer.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLCommandBufferEncoderInfoErrorKey` | const | `MTLCommandBuffer.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLCommandBufferError` | enum | `MTLCommandBuffer.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLCommandBufferErrorDomain` | const | `MTLCommandBuffer.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLCommandBufferErrorOption` | enum | `MTLCommandBuffer.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLCommandEncoder` | protocol | `MTLCommandEncoder.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLCommandEncoderErrorState` | enum | `MTLCommandBuffer.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLCommandQueueDescriptor` | interface | `MTLCommandQueue.h` | Descriptor/configuration surface is kept internal to the Swift bridge. |
| `MTLCommonCounter` | typedef | `MTLCounters.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLCommonCounterClipperInvocations` | const | `MTLCounters.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLCommonCounterClipperPrimitivesOut` | const | `MTLCounters.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLCommonCounterComputeKernelInvocations` | const | `MTLCounters.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLCommonCounterFragmentCycles` | const | `MTLCounters.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLCommonCounterFragmentInvocations` | const | `MTLCounters.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLCommonCounterFragmentsPassed` | const | `MTLCounters.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLCommonCounterPostTessellationVertexCycles` | const | `MTLCounters.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLCommonCounterPostTessellationVertexInvocations` | const | `MTLCounters.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLCommonCounterRenderTargetWriteCycles` | const | `MTLCounters.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLCommonCounterSet` | typedef | `MTLCounters.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLCommonCounterSetStageUtilization` | const | `MTLCounters.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLCommonCounterSetStatistic` | const | `MTLCounters.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLCommonCounterSetTimestamp` | const | `MTLCounters.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLCommonCounterTessellationCycles` | const | `MTLCounters.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLCommonCounterTessellationInputPatches` | const | `MTLCounters.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLCommonCounterTimestamp` | const | `MTLCounters.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLCommonCounterTotalCycles` | const | `MTLCounters.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLCommonCounterVertexCycles` | const | `MTLCounters.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLCommonCounterVertexInvocations` | const | `MTLCounters.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLCompareFunction` | enum | `MTLDepthStencil.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLCompileOptions` | interface | `MTLLibrary.h` | Descriptor/configuration surface is kept internal to the Swift bridge. |
| `MTLCompileSymbolVisibility` | enum | `MTLLibrary.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLComputePassDescriptor` | interface | `MTLComputePass.h` | Pass-descriptor family is not exposed; crate keeps a simplified headless path. |
| `MTLComputePassSampleBufferAttachmentDescriptor` | interface | `MTLComputePass.h` | Pass-descriptor family is not exposed; crate keeps a simplified headless path. |
| `MTLComputePassSampleBufferAttachmentDescriptorArray` | interface | `MTLComputePass.h` | Pass-descriptor family is not exposed; crate keeps a simplified headless path. |
| `MTLComputePipelineDescriptor` | interface | `MTLComputePipeline.h` | Only simple pipeline-state helpers are public; descriptor/reflection surface is deferred. |
| `MTLComputePipelineReflection` | interface | `MTLComputePipeline.h` | Only simple pipeline-state helpers are public; descriptor/reflection surface is deferred. |
| `MTLCoordinate2D` | typedef | `MTLTypes.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLCopyAllDevices` | function | `MTLDevice.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLCopyAllDevicesWithObserver` | function | `MTLDevice.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLCounter` | protocol | `MTLCounters.h` | Counter objects beyond sample buffers and counter-set names are not wrapped. |
| `MTLCounterErrorDomain` | const | `MTLCounters.h` | Counter objects beyond sample buffers and counter-set names are not wrapped. |
| `MTLCounterSampleBufferDescriptor` | interface | `MTLCounters.h` | Counter objects beyond sample buffers and counter-set names are not wrapped. |
| `MTLCounterSampleBufferError` | enum | `MTLCounters.h` | Counter objects beyond sample buffers and counter-set names are not wrapped. |
| `MTLCullMode` | enum | `MTLRenderCommandEncoder.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLCurveBasis` | enum | `MTLAccelerationStructure.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLCurveEndCaps` | enum | `MTLAccelerationStructure.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLCurveType` | enum | `MTLAccelerationStructure.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLDataType` | enum | `MTLDataType.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLDepthClipMode` | enum | `MTLRenderCommandEncoder.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLDepthStencilDescriptor` | interface | `MTLDepthStencil.h` | Depth/stencil descriptor/state family is deferred in 0.6.0. |
| `MTLDepthStencilState` | protocol | `MTLDepthStencil.h` | Depth/stencil descriptor/state family is deferred in 0.6.0. |
| `MTLDeviceLocation` | enum | `MTLDevice.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLDeviceNotificationName` | typedef | `MTLDevice.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLDeviceRemovalRequestedNotification` | const | `MTLDevice.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLDeviceWasAddedNotification` | const | `MTLDevice.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLDeviceWasRemovedNotification` | const | `MTLDevice.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLDispatchType` | enum | `MTLCommandBuffer.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLDrawable` | protocol | `MTLDrawable.h` | Drawable/presentation APIs are deferred. |
| `MTLDynamicLibraryDomain` | const | `MTLDynamicLibrary.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLDynamicLibraryError` | enum | `MTLDynamicLibrary.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLFXFrameInterpolatableScaler` | protocol | `MTLFXTemporalScaler.h` | MetalFX.framework is explicitly out of scope for apple-metal 0.6.0. |
| `MTLFXFrameInterpolator` | protocol | `MTLFXFrameInterpolator.h` | MetalFX.framework is explicitly out of scope for apple-metal 0.6.0. |
| `MTLFXFrameInterpolatorBase` | protocol | `MTLFXFrameInterpolator.h` | MetalFX.framework is explicitly out of scope for apple-metal 0.6.0. |
| `MTLFXFrameInterpolatorDescriptor` | interface | `MTLFXFrameInterpolator.h` | MetalFX.framework is explicitly out of scope for apple-metal 0.6.0. |
| `MTLFXSpatialScaler` | protocol | `MTLFXSpatialScaler.h` | MetalFX.framework is explicitly out of scope for apple-metal 0.6.0. |
| `MTLFXSpatialScalerBase` | protocol | `MTLFXSpatialScaler.h` | MetalFX.framework is explicitly out of scope for apple-metal 0.6.0. |
| `MTLFXSpatialScalerColorProcessingMode` | enum | `MTLFXSpatialScaler.h` | MetalFX.framework is explicitly out of scope for apple-metal 0.6.0. |
| `MTLFXSpatialScalerDescriptor` | interface | `MTLFXSpatialScaler.h` | MetalFX.framework is explicitly out of scope for apple-metal 0.6.0. |
| `MTLFXTemporalDenoisedScaler` | protocol | `MTLFXTemporalDenoisedScaler.h` | MetalFX.framework is explicitly out of scope for apple-metal 0.6.0. |
| `MTLFXTemporalDenoisedScalerBase` | protocol | `MTLFXTemporalDenoisedScaler.h` | MetalFX.framework is explicitly out of scope for apple-metal 0.6.0. |
| `MTLFXTemporalDenoisedScalerDescriptor` | interface | `MTLFXTemporalDenoisedScaler.h` | MetalFX.framework is explicitly out of scope for apple-metal 0.6.0. |
| `MTLFXTemporalScaler` | protocol | `MTLFXTemporalScaler.h` | MetalFX.framework is explicitly out of scope for apple-metal 0.6.0. |
| `MTLFXTemporalScalerBase` | protocol | `MTLFXTemporalScaler.h` | MetalFX.framework is explicitly out of scope for apple-metal 0.6.0. |
| `MTLFXTemporalScalerDescriptor` | interface | `MTLFXTemporalScaler.h` | MetalFX.framework is explicitly out of scope for apple-metal 0.6.0. |
| `MTLFunctionConstant` | interface | `MTLLibrary.h` | Advanced function specialization/linking/logging surface is deferred. |
| `MTLFunctionConstantValues` | interface | `MTLFunctionConstantValues.h` | Advanced function specialization/linking/logging surface is deferred. |
| `MTLFunctionDescriptor` | interface | `MTLFunctionDescriptor.h` | Advanced function specialization/linking/logging surface is deferred. |
| `MTLFunctionHandle` | protocol | `MTLFunctionHandle.h` | Advanced function specialization/linking/logging surface is deferred. |
| `MTLFunctionLog` | protocol | `MTLFunctionLog.h` | Advanced function specialization/linking/logging surface is deferred. |
| `MTLFunctionLogDebugLocation` | protocol | `MTLFunctionLog.h` | Advanced function specialization/linking/logging surface is deferred. |
| `MTLFunctionLogType` | enum | `MTLFunctionLog.h` | Advanced function specialization/linking/logging surface is deferred. |
| `MTLFunctionOptions` | enum | `MTLFunctionDescriptor.h` | Advanced function specialization/linking/logging surface is deferred. |
| `MTLFunctionReflection` | interface | `MTLLibrary.h` | Advanced function specialization/linking/logging surface is deferred. |
| `MTLFunctionStitchingAttribute` | protocol | `MTLFunctionStitching.h` | Advanced function specialization/linking/logging surface is deferred. |
| `MTLFunctionStitchingAttributeAlwaysInline` | interface | `MTLFunctionStitching.h` | Advanced function specialization/linking/logging surface is deferred. |
| `MTLFunctionStitchingFunctionNode` | interface | `MTLFunctionStitching.h` | Advanced function specialization/linking/logging surface is deferred. |
| `MTLFunctionStitchingGraph` | interface | `MTLFunctionStitching.h` | Advanced function specialization/linking/logging surface is deferred. |
| `MTLFunctionStitchingInputNode` | interface | `MTLFunctionStitching.h` | Advanced function specialization/linking/logging surface is deferred. |
| `MTLFunctionStitchingNode` | protocol | `MTLFunctionStitching.h` | Advanced function specialization/linking/logging surface is deferred. |
| `MTLFunctionType` | enum | `MTLLibrary.h` | Advanced function specialization/linking/logging surface is deferred. |
| `MTLGPUAddress` | struct | `MTL4BufferRange.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTLHeapDescriptor` | interface | `MTLHeap.h` | Descriptor/configuration surface is kept internal to the Swift bridge. |
| `MTLHeapType` | enum | `MTLHeap.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLIOCommandBuffer` | protocol | `MTLIOCommandBuffer.h` | GPU file-I/O surface is deferred. |
| `MTLIOCommandQueue` | protocol | `MTLIOCommandQueue.h` | GPU file-I/O surface is deferred. |
| `MTLIOCommandQueueDescriptor` | interface | `MTLIOCommandQueue.h` | GPU file-I/O surface is deferred. |
| `MTLIOCommandQueueType` | enum | `MTLIOCommandQueue.h` | GPU file-I/O surface is deferred. |
| `MTLIOCompressionContext` | typedef | `MTLIOCompressor.h` | GPU file-I/O surface is deferred. |
| `MTLIOCompressionContextAppendData` | function | `MTLIOCompressor.h` | GPU file-I/O surface is deferred. |
| `MTLIOCompressionContextDefaultChunkSize` | function | `MTLIOCompressor.h` | GPU file-I/O surface is deferred. |
| `MTLIOCompressionMethod` | enum | `MTLDevice.h` | GPU file-I/O surface is deferred. |
| `MTLIOCompressionStatus` | enum | `MTLIOCompressor.h` | GPU file-I/O surface is deferred. |
| `MTLIOCreateCompressionContext` | function | `MTLIOCompressor.h` | GPU file-I/O surface is deferred. |
| `MTLIOErrorDomain` | const | `MTLIOCommandQueue.h` | GPU file-I/O surface is deferred. |
| `MTLIOFileHandle` | protocol | `MTLIOCommandQueue.h` | GPU file-I/O surface is deferred. |
| `MTLIOFlushAndDestroyCompressionContext` | function | `MTLIOCompressor.h` | GPU file-I/O surface is deferred. |
| `MTLIOPriority` | enum | `MTLIOCommandQueue.h` | GPU file-I/O surface is deferred. |
| `MTLIOScratchBuffer` | protocol | `MTLIOCommandQueue.h` | GPU file-I/O surface is deferred. |
| `MTLIOScratchBufferAllocator` | protocol | `MTLIOCommandQueue.h` | GPU file-I/O surface is deferred. |
| `MTLIOStatus` | enum | `MTLIOCommandBuffer.h` | GPU file-I/O surface is deferred. |
| `MTLIndexType` | enum | `MTLArgument.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLIndirectCommandBufferDescriptor` | interface | `MTLIndirectCommandBuffer.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLIndirectComputeCommand` | protocol | `MTLIndirectCommandEncoder.h` | Indirect command recording is deferred; crate only allocates/resets indirect command buffers. |
| `MTLIndirectInstanceAccelerationStructureDescriptor` | interface | `MTLAccelerationStructure.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLIndirectRenderCommand` | protocol | `MTLIndirectCommandEncoder.h` | Indirect command recording is deferred; crate only allocates/resets indirect command buffers. |
| `MTLInstanceAccelerationStructureDescriptor` | interface | `MTLAccelerationStructure.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLIntersectionFunctionDescriptor` | interface | `MTLFunctionDescriptor.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLIntersectionFunctionTableDescriptor` | interface | `MTLIntersectionFunctionTable.h` | Advanced function specialization/linking/logging surface is deferred. |
| `MTLLanguageVersion` | enum | `MTLLibrary.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLLibraryError` | enum | `MTLLibrary.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLLibraryErrorDomain` | const | `MTLLibrary.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLLibraryOptimizationLevel` | enum | `MTLLibrary.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLLibraryType` | enum | `MTLLibrary.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLLinkedFunctions` | interface | `MTLLinkedFunctions.h` | Advanced function specialization/linking/logging surface is deferred. |
| `MTLLogContainer` | protocol | `MTLFunctionLog.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLLogStateDescriptor` | interface | `MTLLogState.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLLogStateError` | enum | `MTLLogState.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLLogStateErrorDomain` | const | `MTLLogState.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLLogicalToPhysicalColorAttachmentMap` | interface | `MTLRenderPipeline.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLMathFloatingPointFunctions` | enum | `MTLLibrary.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLMathMode` | enum | `MTLLibrary.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLMatrixLayout` | enum | `MTLAccelerationStructure.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLMeshRenderPipelineDescriptor` | interface | `MTLRenderPipeline.h` | Only simple pipeline-state helpers are public; descriptor/reflection surface is deferred. |
| `MTLMotionBorderMode` | enum | `MTLAccelerationStructure.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLMotionKeyframeData` | interface | `MTLAccelerationStructure.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLMultisampleDepthResolveFilter` | enum | `MTLRenderPass.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLMultisampleStencilResolveFilter` | enum | `MTLRenderPass.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLMutability` | enum | `MTLPipeline.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLNewComputePipelineStateCompletionHandler` | typedef | `MTLLibrary.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLNewComputePipelineStateWithReflectionCompletionHandler` | typedef | `MTLLibrary.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLNewDynamicLibraryCompletionHandler` | typedef | `MTLLibrary.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLNewLibraryCompletionHandler` | typedef | `MTLLibrary.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLNewRenderPipelineStateCompletionHandler` | typedef | `MTLLibrary.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLNewRenderPipelineStateWithReflectionCompletionHandler` | typedef | `MTLLibrary.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLObjectPayloadBinding` | protocol | `MTLArgument.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLOrigin` | struct | `MTLTypes.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLPackedFloat3` | typedef | `MTLAccelerationStructureTypes.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLPackedFloat4x3` | struct | `MTLAccelerationStructure.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLPackedFloatQuaternion` | typedef | `MTLAccelerationStructureTypes.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLParallelRenderCommandEncoder` | protocol | `MTLParallelRenderCommandEncoder.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLPatchType` | enum | `MTLLibrary.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLPipelineBufferDescriptor` | interface | `MTLPipeline.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLPipelineBufferDescriptorArray` | interface | `MTLPipeline.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLPipelineOption` | enum | `MTLDevice.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLPointerType` | interface | `MTLArgument.h` | Argument/reflection metadata surface is not wrapped yet. |
| `MTLPrimitiveAccelerationStructureDescriptor` | interface | `MTLAccelerationStructure.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLPrimitiveTopologyClass` | enum | `MTLRenderPipeline.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLRasterizationRateLayerArray` | interface | `MTLRasterizationRate.h` | Rasterization-rate map surface is deferred. |
| `MTLRasterizationRateLayerDescriptor` | interface | `MTLRasterizationRate.h` | Rasterization-rate map surface is deferred. |
| `MTLRasterizationRateMap` | protocol | `MTLRasterizationRate.h` | Rasterization-rate map surface is deferred. |
| `MTLRasterizationRateMapDescriptor` | interface | `MTLRasterizationRate.h` | Rasterization-rate map surface is deferred. |
| `MTLRasterizationRateSampleArray` | interface | `MTLRasterizationRate.h` | Rasterization-rate map surface is deferred. |
| `MTLReadWriteTextureTier` | enum | `MTLDevice.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLRegion` | struct | `MTL4CommandQueue.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTLRemoveDeviceObserver` | function | `MTLDevice.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLRenderPassAttachmentDescriptor` | interface | `MTLRenderPass.h` | Pass-descriptor family is not exposed; crate keeps a simplified headless path. |
| `MTLRenderPassColorAttachmentDescriptor` | interface | `MTLRenderPass.h` | Pass-descriptor family is not exposed; crate keeps a simplified headless path. |
| `MTLRenderPassColorAttachmentDescriptorArray` | interface | `MTLRenderPass.h` | Pass-descriptor family is not exposed; crate keeps a simplified headless path. |
| `MTLRenderPassDepthAttachmentDescriptor` | interface | `MTLRenderPass.h` | Pass-descriptor family is not exposed; crate keeps a simplified headless path. |
| `MTLRenderPassDescriptor` | interface | `MTLRenderPass.h` | Pass-descriptor family is not exposed; crate keeps a simplified headless path. |
| `MTLRenderPassSampleBufferAttachmentDescriptor` | interface | `MTLRenderPass.h` | Pass-descriptor family is not exposed; crate keeps a simplified headless path. |
| `MTLRenderPassSampleBufferAttachmentDescriptorArray` | interface | `MTLRenderPass.h` | Pass-descriptor family is not exposed; crate keeps a simplified headless path. |
| `MTLRenderPassStencilAttachmentDescriptor` | interface | `MTLRenderPass.h` | Pass-descriptor family is not exposed; crate keeps a simplified headless path. |
| `MTLRenderPipelineColorAttachmentDescriptor` | interface | `MTLRenderPipeline.h` | Only simple pipeline-state helpers are public; descriptor/reflection surface is deferred. |
| `MTLRenderPipelineColorAttachmentDescriptorArray` | interface | `MTLRenderPipeline.h` | Only simple pipeline-state helpers are public; descriptor/reflection surface is deferred. |
| `MTLRenderPipelineDescriptor` | interface | `MTLRenderPipeline.h` | Only simple pipeline-state helpers are public; descriptor/reflection surface is deferred. |
| `MTLRenderPipelineFunctionsDescriptor` | interface | `MTLRenderPipeline.h` | Only simple pipeline-state helpers are public; descriptor/reflection surface is deferred. |
| `MTLRenderPipelineReflection` | interface | `MTLRenderPipeline.h` | Only simple pipeline-state helpers are public; descriptor/reflection surface is deferred. |
| `MTLRenderStages` | enum | `MTLRenderCommandEncoder.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLResidencySetDescriptor` | interface | `MTLResidencySet.h` | Descriptor/configuration surface is kept internal to the Swift bridge. |
| `MTLResource` | protocol | `MTLResource.h` | Base-resource / newer utility family is not wrapped in 0.6.0. |
| `MTLResourceID` | struct | `MTLTypes.h` | Base-resource / newer utility family is not wrapped in 0.6.0. |
| `MTLResourceStateCommandEncoder` | protocol | `MTLResourceStateCommandEncoder.h` | Resource-state and sparse-resource management are deferred. |
| `MTLResourceStatePassDescriptor` | interface | `MTLResourceStatePass.h` | Resource-state and sparse-resource management are deferred. |
| `MTLResourceStatePassSampleBufferAttachmentDescriptor` | interface | `MTLResourceStatePass.h` | Resource-state and sparse-resource management are deferred. |
| `MTLResourceStatePassSampleBufferAttachmentDescriptorArray` | interface | `MTLResourceStatePass.h` | Resource-state and sparse-resource management are deferred. |
| `MTLResourceUsage` | enum | `MTLCommandEncoder.h` | Base-resource / newer utility family is not wrapped in 0.6.0. |
| `MTLResourceViewPool` | protocol | `MTLResourceViewPool.h` | Base-resource / newer utility family is not wrapped in 0.6.0. |
| `MTLResourceViewPoolDescriptor` | interface | `MTLResourceViewPool.h` | Base-resource / newer utility family is not wrapped in 0.6.0. |
| `MTLSamplerAddressMode` | enum | `MTLSampler.h` | Sampler descriptor/state family is deferred in 0.6.0. |
| `MTLSamplerBorderColor` | enum | `MTLSampler.h` | Sampler descriptor/state family is deferred in 0.6.0. |
| `MTLSamplerDescriptor` | interface | `MTLSampler.h` | Sampler descriptor/state family is deferred in 0.6.0. |
| `MTLSamplerMinMagFilter` | enum | `MTLSampler.h` | Sampler descriptor/state family is deferred in 0.6.0. |
| `MTLSamplerMipFilter` | enum | `MTLSampler.h` | Sampler descriptor/state family is deferred in 0.6.0. |
| `MTLSamplerReductionMode` | enum | `MTLSampler.h` | Sampler descriptor/state family is deferred in 0.6.0. |
| `MTLSamplerState` | protocol | `MTLSampler.h` | Sampler descriptor/state family is deferred in 0.6.0. |
| `MTLShaderValidation` | enum | `MTLPipeline.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLSharedEventHandle` | interface | `MTLEvent.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLSharedEventListener` | interface | `MTLEvent.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLSharedTextureHandle` | interface | `MTLTexture.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLSparsePageSize` | enum | `MTLResource.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLSparseTextureMappingMode` | struct | `MTL4CommandQueue.h` | Metal 4 family is intentionally deferred in 0.6.0. |
| `MTLSparseTextureRegionAlignmentMode` | enum | `MTLDevice.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLStageInputOutputDescriptor` | interface | `MTLStageInputOutputDescriptor.h` | Vertex/stage-input descriptor and reflection metadata are not wrapped yet. |
| `MTLStages` | enum | `MTLCommandEncoder.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLStencilDescriptor` | interface | `MTLDepthStencil.h` | Depth/stencil descriptor/state family is deferred in 0.6.0. |
| `MTLStencilOperation` | enum | `MTLDepthStencil.h` | Depth/stencil descriptor/state family is deferred in 0.6.0. |
| `MTLStepFunction` | enum | `MTLStageInputOutputDescriptor.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLStitchedLibraryDescriptor` | interface | `MTLFunctionStitching.h` | Advanced function specialization/linking/logging surface is deferred. |
| `MTLStitchedLibraryOptions` | enum | `MTLFunctionStitching.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLStoreActionOptions` | enum | `MTLRenderPass.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLStructMember` | interface | `MTLArgument.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLStructType` | interface | `MTLArgument.h` | Argument/reflection metadata surface is not wrapped yet. |
| `MTLTensorBinding` | protocol | `MTLArgument.h` | Base-resource / newer utility family is not wrapped in 0.6.0. |
| `MTLTensorDataType` | enum | `MTLTensor.h` | Base-resource / newer utility family is not wrapped in 0.6.0. |
| `MTLTensorDescriptor` | interface | `MTLTensor.h` | Base-resource / newer utility family is not wrapped in 0.6.0. |
| `MTLTensorDomain` | const | `MTLTensor.h` | Base-resource / newer utility family is not wrapped in 0.6.0. |
| `MTLTensorError` | enum | `MTLTensor.h` | Base-resource / newer utility family is not wrapped in 0.6.0. |
| `MTLTensorExtents` | interface | `MTLTensor.h` | Base-resource / newer utility family is not wrapped in 0.6.0. |
| `MTLTensorReferenceType` | interface | `MTLArgument.h` | Base-resource / newer utility family is not wrapped in 0.6.0. |
| `MTLTensorUsage` | enum | `MTLTensor.h` | Base-resource / newer utility family is not wrapped in 0.6.0. |
| `MTLTessellationControlPointIndexType` | enum | `MTLRenderPipeline.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLTessellationFactorFormat` | enum | `MTLRenderPipeline.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLTessellationFactorStepFunction` | enum | `MTLRenderPipeline.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLTessellationPartitionMode` | enum | `MTLRenderPipeline.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLTextureBinding` | protocol | `MTLArgument.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLTextureCompressionType` | enum | `MTLTexture.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLTextureReferenceType` | interface | `MTLArgument.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLTextureSparseTier` | enum | `MTLResource.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLTextureSwizzle` | enum | `MTLTexture.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLTextureType` | enum | `MTLTexture.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLTextureViewDescriptor` | interface | `MTLTexture.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLTextureViewPool` | protocol | `MTLTextureViewPool.h` | Base-resource / newer utility family is not wrapped in 0.6.0. |
| `MTLThreadgroupBinding` | protocol | `MTLArgument.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLTileRenderPipelineColorAttachmentDescriptor` | interface | `MTLRenderPipeline.h` | Only simple pipeline-state helpers are public; descriptor/reflection surface is deferred. |
| `MTLTileRenderPipelineColorAttachmentDescriptorArray` | interface | `MTLRenderPipeline.h` | Only simple pipeline-state helpers are public; descriptor/reflection surface is deferred. |
| `MTLTileRenderPipelineDescriptor` | interface | `MTLRenderPipeline.h` | Only simple pipeline-state helpers are public; descriptor/reflection surface is deferred. |
| `MTLTimestamp` | typedef | `MTLDevice.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLTransformType` | enum | `MTLAccelerationStructure.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLTriangleFillMode` | enum | `MTLRenderCommandEncoder.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLType` | interface | `MTLArgument.h` | Argument/reflection metadata surface is not wrapped yet. |
| `MTLVertexAttribute` | interface | `MTLLibrary.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLVertexAttributeDescriptor` | interface | `MTLVertexDescriptor.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLVertexAttributeDescriptorArray` | interface | `MTLVertexDescriptor.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLVertexBufferLayoutDescriptor` | interface | `MTLVertexDescriptor.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLVertexBufferLayoutDescriptorArray` | interface | `MTLVertexDescriptor.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLVertexDescriptor` | interface | `MTLVertexDescriptor.h` | Vertex/stage-input descriptor and reflection metadata are not wrapped yet. |
| `MTLVertexFormat` | enum | `MTLVertexDescriptor.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLVertexStepFunction` | enum | `MTLVertexDescriptor.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLVisibilityResultMode` | enum | `MTLRenderCommandEncoder.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLVisibilityResultType` | enum | `MTLRenderPass.h` | No public Rust wrapper in apple-metal 0.6.0. |
| `MTLVisibleFunctionTableDescriptor` | interface | `MTLVisibleFunctionTable.h` | Advanced function specialization/linking/logging surface is deferred. |
| `MTLWinding` | enum | `MTLRenderCommandEncoder.h` | No public Rust wrapper in apple-metal 0.6.0. |

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| `MTLArgument` | interface | `MTLArgument.h` | Deprecated top-level API; audit instructions treat deprecated symbols as exempt. | `API_DEPRECATED_WITH_REPLACEMENT("MTLBinding", macos(10.11, 13.0), ios(8.0, 16.0)) @interface MTLArgument : NSObject` |
| `MTLArgumentAccess` | typedef | `MTLArgument.h` | Deprecated top-level API; audit instructions treat deprecated symbols as exempt. | `API_DEPRECATED_WITH_REPLACEMENT("MTLBindingAccess", macos(10.11, 14.0), ios(8.0, 17.0)); typedef MTLBindingAccess MTLArgumentAccess API_DEPRECATED_WITH_REPLACEMENT("MTLBindingAccess", macos(10.11, 14.0), ios(8.0, 17.0));` |
| `MTLArgumentType` | enum | `MTLArgument.h` | Deprecated top-level API; audit instructions treat deprecated symbols as exempt. | `API_DEPRECATED_WITH_REPLACEMENT("MTLBindingType", macos(10.11, 13.0), ios(8.0, 16.0));` |
| `MTLAutoreleasedArgument` | typedef | `MTLLibrary.h` | Deprecated top-level API; audit instructions treat deprecated symbols as exempt. | `API_DEPRECATED("Use MTLBinding and cast to specific Binding (MTLTextureBinding, MTLBufferBinding, .etc) instead", macos(10.11, 13.0), ios(8.0, 16.0)); typedef __autoreleasing MTLArgument *__nullable MTLAutoreleasedArgument API_DEPRECATED("Use MTLBinding and cast to specific Binding (MTLTextureBinding, MTLBufferBinding, .etc) instead", macos(10.11, 13.0), ios(8.0, 16.0));` |
| `MTLFeatureSet` | enum | `MTLDevice.h` | Deprecated top-level API; audit instructions treat deprecated symbols as exempt. | `API_DEPRECATED("Use MTLGPUFamily instead", macos(10.11, 13.0), ios(8.0, 16.0), tvos(9.0, 16.0));` |

