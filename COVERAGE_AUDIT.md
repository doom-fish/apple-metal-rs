# apple-metal-rs coverage audit (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 431
VERIFIED: 431
GAPS: 0
EXEMPT: 0
COVERAGE_PCT: 100.00

Scope: top-level public symbols from `Metal.framework` + `MetalFX.framework` headers, filtered for macOS availability.

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| `MTL4AccelerationStructureBoundingBoxGeometryDescriptor` | interface | `MTL4AccelerationStructure.h` | `Metal4AccelerationStructureBoundingBoxGeometryDescriptor::new` |
| `MTL4AccelerationStructureCurveGeometryDescriptor` | interface | `MTL4AccelerationStructure.h` | `Metal4AccelerationStructureCurveGeometryDescriptor::new` |
| `MTL4AccelerationStructureDescriptor` | interface | `MTL4AccelerationStructure.h` | `Metal4AccelerationStructureDescriptor::new` |
| `MTL4AccelerationStructureGeometryDescriptor` | interface | `MTL4AccelerationStructure.h` | `Metal4AccelerationStructureGeometryDescriptor::new` |
| `MTL4AccelerationStructureMotionBoundingBoxGeometryDescriptor` | interface | `MTL4AccelerationStructure.h` | `Metal4AccelerationStructureMotionBoundingBoxGeometryDescriptor::new` |
| `MTL4AccelerationStructureMotionCurveGeometryDescriptor` | interface | `MTL4AccelerationStructure.h` | `Metal4AccelerationStructureMotionCurveGeometryDescriptor::new` |
| `MTL4AccelerationStructureMotionTriangleGeometryDescriptor` | interface | `MTL4AccelerationStructure.h` | `Metal4AccelerationStructureMotionTriangleGeometryDescriptor::new` |
| `MTL4AccelerationStructureTriangleGeometryDescriptor` | interface | `MTL4AccelerationStructure.h` | `Metal4AccelerationStructureTriangleGeometryDescriptor::new` |
| `MTL4AlphaToCoverageState` | enum | `MTL4PipelineState.h` | `Metal4AlphaToCoverageState` |
| `MTL4AlphaToOneState` | enum | `MTL4PipelineState.h` | `Metal4AlphaToOneState` |
| `MTL4Archive` | protocol | `MTL4Archive.h` | `Metal4Archive` |
| `MTL4ArgumentTable` | protocol | `MTL4ArgumentTable.h` | `Metal4ArgumentTable` |
| `MTL4ArgumentTableDescriptor` | interface | `MTL4ArgumentTable.h` | `Metal4ArgumentTableDescriptor::new` |
| `MTL4BinaryFunction` | protocol | `MTL4BinaryFunction.h` | `Metal4BinaryFunction` |
| `MTL4BinaryFunctionDescriptor` | interface | `MTL4BinaryFunctionDescriptor.h` | `Metal4BinaryFunctionDescriptor::new` |
| `MTL4BinaryFunctionOptions` | enum | `MTL4BinaryFunctionDescriptor.h` | `Metal4BinaryFunctionOptions` |
| `MTL4BlendState` | enum | `MTL4PipelineState.h` | `Metal4BlendState` |
| `MTL4CommandAllocator` | protocol | `MTL4CommandAllocator.h` | `Metal4CommandAllocator` |
| `MTL4CommandAllocatorDescriptor` | interface | `MTL4CommandAllocator.h` | `Metal4CommandAllocatorDescriptor::new` |
| `MTL4CommandBuffer` | protocol | `MTL4CommandBuffer.h` | `Metal4CommandBuffer` |
| `MTL4CommandBufferOptions` | interface | `MTL4CommandBuffer.h` | `Metal4CommandBufferOptions::new` |
| `MTL4CommandEncoder` | protocol | `MTL4CommandEncoder.h` | `Metal4CommandEncoder` |
| `MTL4CommandQueue` | protocol | `MTL4CommandQueue.h` | `Metal4CommandQueue` |
| `MTL4CommandQueueDescriptor` | interface | `MTL4CommandQueue.h` | `Metal4CommandQueueDescriptor::new` |
| `MTL4CommandQueueError` | enum | `MTL4CommandQueue.h` | `Metal4CommandQueueError` |
| `MTL4CommandQueueErrorDomain` | const | `MTL4CommandQueue.h` | `metal4_command_queue_error_domain()` |
| `MTL4CommitFeedback` | protocol | `MTL4CommitFeedback.h` | `Metal4CommitFeedback` |
| `MTL4CommitOptions` | interface | `MTL4CommandQueue.h` | `Metal4CommitOptions::new` |
| `MTL4Compiler` | protocol | `MTL4Compiler.h` | `Metal4Compiler` |
| `MTL4CompilerDescriptor` | interface | `MTL4Compiler.h` | `Metal4CompilerDescriptor::new` |
| `MTL4CompilerTask` | protocol | `MTL4CompilerTask.h` | `Metal4CompilerTask` |
| `MTL4CompilerTaskOptions` | interface | `MTL4Compiler.h` | `Metal4CompilerTaskOptions::new` |
| `MTL4CompilerTaskStatus` | enum | `MTL4CompilerTask.h` | `Metal4CompilerTaskStatus` |
| `MTL4ComputeCommandEncoder` | protocol | `MTL4ComputeCommandEncoder.h` | `Metal4ComputeCommandEncoder` |
| `MTL4ComputePipelineDescriptor` | interface | `MTL4ComputePipeline.h` | `Metal4ComputePipelineDescriptor::new` |
| `MTL4CounterHeap` | protocol | `MTL4Counters.h` | `Metal4CounterHeap` |
| `MTL4CounterHeapDescriptor` | interface | `MTL4Counters.h` | `Metal4CounterHeapDescriptor::new` |
| `MTL4CounterHeapType` | enum | `MTL4Counters.h` | `Metal4CounterHeapType` |
| `MTL4FXFrameInterpolator` | protocol | `MTL4FXFrameInterpolator.h` | `Metal4FxFrameInterpolator` |
| `MTL4FXSpatialScaler` | protocol | `MTL4FXSpatialScaler.h` | `Metal4FxSpatialScaler` |
| `MTL4FXTemporalDenoisedScaler` | protocol | `MTL4FXTemporalDenoisedScaler.h` | `Metal4FxTemporalDenoisedScaler` |
| `MTL4FXTemporalScaler` | protocol | `MTL4FXTemporalScaler.h` | `Metal4FxTemporalScaler` |
| `MTL4FunctionDescriptor` | interface | `MTL4FunctionDescriptor.h` | `Metal4FunctionDescriptor::new` |
| `MTL4IndirectCommandBufferSupportState` | enum | `MTL4PipelineState.h` | `Metal4IndirectCommandBufferSupportState` |
| `MTL4IndirectInstanceAccelerationStructureDescriptor` | interface | `MTL4AccelerationStructure.h` | `Metal4IndirectInstanceAccelerationStructureDescriptor::new` |
| `MTL4InstanceAccelerationStructureDescriptor` | interface | `MTL4AccelerationStructure.h` | `Metal4InstanceAccelerationStructureDescriptor::new` |
| `MTL4LibraryDescriptor` | interface | `MTL4LibraryDescriptor.h` | `Metal4LibraryDescriptor::new` |
| `MTL4LibraryFunctionDescriptor` | interface | `MTL4LibraryFunctionDescriptor.h` | `Metal4LibraryFunctionDescriptor::new` |
| `MTL4LogicalToPhysicalColorAttachmentMappingState` | enum | `MTL4RenderPipeline.h` | `Metal4LogicalToPhysicalColorAttachmentMappingState` |
| `MTL4MachineLearningCommandEncoder` | protocol | `MTL4MachineLearningCommandEncoder.h` | `Metal4MachineLearningCommandEncoder` |
| `MTL4MachineLearningPipelineDescriptor` | interface | `MTL4MachineLearningPipeline.h` | `Metal4MachineLearningPipelineDescriptor::new` |
| `MTL4MachineLearningPipelineReflection` | interface | `MTL4MachineLearningPipeline.h` | `Metal4MachineLearningPipelineReflection::new` |
| `MTL4MachineLearningPipelineState` | protocol | `MTL4MachineLearningPipeline.h` | `Metal4MachineLearningPipelineState` |
| `MTL4MeshRenderPipelineDescriptor` | interface | `MTL4MeshRenderPipeline.h` | `Metal4MeshRenderPipelineDescriptor::new` |
| `MTL4PipelineDataSetSerializer` | protocol | `MTL4PipelineDataSetSerializer.h` | `Metal4PipelineDataSetSerializer` |
| `MTL4PipelineDataSetSerializerConfiguration` | enum | `MTL4PipelineDataSetSerializer.h` | `Metal4PipelineDataSetSerializerConfiguration` |
| `MTL4PipelineDataSetSerializerDescriptor` | interface | `MTL4PipelineDataSetSerializer.h` | `Metal4PipelineDataSetSerializerDescriptor::new` |
| `MTL4PipelineDescriptor` | interface | `MTL4PipelineState.h` | `Metal4PipelineDescriptor::new` |
| `MTL4PipelineOptions` | interface | `MTL4PipelineState.h` | `Metal4PipelineOptions::new` |
| `MTL4PipelineStageDynamicLinkingDescriptor` | interface | `MTL4LinkingDescriptor.h` | `Metal4PipelineStageDynamicLinkingDescriptor::new` |
| `MTL4PrimitiveAccelerationStructureDescriptor` | interface | `MTL4AccelerationStructure.h` | `Metal4PrimitiveAccelerationStructureDescriptor::new` |
| `MTL4RenderCommandEncoder` | protocol | `MTL4RenderCommandEncoder.h` | `Metal4RenderCommandEncoder` |
| `MTL4RenderEncoderOptions` | enum | `MTL4RenderCommandEncoder.h` | `Metal4RenderEncoderOptions` |
| `MTL4RenderPassDescriptor` | interface | `MTL4RenderPass.h` | `Metal4RenderPassDescriptor::new` |
| `MTL4RenderPipelineBinaryFunctionsDescriptor` | interface | `MTL4RenderPipeline.h` | `Metal4RenderPipelineBinaryFunctionsDescriptor::new` |
| `MTL4RenderPipelineColorAttachmentDescriptor` | interface | `MTL4RenderPipeline.h` | `Metal4RenderPipelineColorAttachmentDescriptor::new` |
| `MTL4RenderPipelineColorAttachmentDescriptorArray` | interface | `MTL4RenderPipeline.h` | `Metal4RenderPipelineColorAttachmentDescriptorArray::new` |
| `MTL4RenderPipelineDescriptor` | interface | `MTL4RenderPipeline.h` | `Metal4RenderPipelineDescriptor::new` |
| `MTL4RenderPipelineDynamicLinkingDescriptor` | interface | `MTL4LinkingDescriptor.h` | `Metal4RenderPipelineDynamicLinkingDescriptor::new` |
| `MTL4ShaderReflection` | enum | `MTL4PipelineState.h` | `Metal4ShaderReflection` |
| `MTL4SpecializedFunctionDescriptor` | interface | `MTL4SpecializedFunctionDescriptor.h` | `Metal4SpecializedFunctionDescriptor::new` |
| `MTL4StaticLinkingDescriptor` | interface | `MTL4LinkingDescriptor.h` | `Metal4StaticLinkingDescriptor::new` |
| `MTL4StitchedFunctionDescriptor` | interface | `MTL4StitchedFunctionDescriptor.h` | `Metal4StitchedFunctionDescriptor::new` |
| `MTL4TileRenderPipelineDescriptor` | interface | `MTL4TileRenderPipeline.h` | `Metal4TileRenderPipelineDescriptor::new` |
| `MTL4TimestampGranularity` | enum | `MTL4Counters.h` | `Metal4TimestampGranularity` |
| `MTL4VisibilityOptions` | enum | `MTL4CommandEncoder.h` | `Metal4VisibilityOptions` |
| `MTLAccelerationStructure` | protocol | `MTLAccelerationStructure.h` | `AccelerationStructure; MetalDevice::new_acceleration_structure_with_size; Heap::new_acceleration_structure_with_size; ComputeCommandEncoder::set_acceleration_structure` |
| `MTLAccelerationStructureBoundingBoxGeometryDescriptor` | interface | `MTLAccelerationStructure.h` | `MetalAccelerationStructureBoundingBoxGeometryDescriptor::new` |
| `MTLAccelerationStructureCommandEncoder` | protocol | `MTLAccelerationStructureCommandEncoder.h` | `MetalAccelerationStructureCommandEncoder` |
| `MTLAccelerationStructureCurveGeometryDescriptor` | interface | `MTLAccelerationStructure.h` | `MetalAccelerationStructureCurveGeometryDescriptor::new` |
| `MTLAccelerationStructureDescriptor` | interface | `MTLAccelerationStructure.h` | `MetalAccelerationStructureDescriptor::new` |
| `MTLAccelerationStructureGeometryDescriptor` | interface | `MTLAccelerationStructure.h` | `MetalAccelerationStructureGeometryDescriptor::new` |
| `MTLAccelerationStructureInstanceDescriptorType` | enum | `MTLAccelerationStructure.h` | `MetalAccelerationStructureInstanceDescriptorType` |
| `MTLAccelerationStructureInstanceOptions` | enum | `MTLAccelerationStructure.h` | `MetalAccelerationStructureInstanceOptions` |
| `MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor` | interface | `MTLAccelerationStructure.h` | `MetalAccelerationStructureMotionBoundingBoxGeometryDescriptor::new` |
| `MTLAccelerationStructureMotionCurveGeometryDescriptor` | interface | `MTLAccelerationStructure.h` | `MetalAccelerationStructureMotionCurveGeometryDescriptor::new` |
| `MTLAccelerationStructureMotionTriangleGeometryDescriptor` | interface | `MTLAccelerationStructure.h` | `MetalAccelerationStructureMotionTriangleGeometryDescriptor::new` |
| `MTLAccelerationStructurePassDescriptor` | interface | `MTLAccelerationStructureCommandEncoder.h` | `MetalAccelerationStructurePassDescriptor::new` |
| `MTLAccelerationStructurePassSampleBufferAttachmentDescriptor` | interface | `MTLAccelerationStructureCommandEncoder.h` | `MetalAccelerationStructurePassSampleBufferAttachmentDescriptor::new` |
| `MTLAccelerationStructurePassSampleBufferAttachmentDescriptorArray` | interface | `MTLAccelerationStructureCommandEncoder.h` | `MetalAccelerationStructurePassSampleBufferAttachmentDescriptorArray::new` |
| `MTLAccelerationStructureRefitOptions` | enum | `MTLAccelerationStructure.h` | `MetalAccelerationStructureRefitOptions` |
| `MTLAccelerationStructureTriangleGeometryDescriptor` | interface | `MTLAccelerationStructure.h` | `MetalAccelerationStructureTriangleGeometryDescriptor::new` |
| `MTLAccelerationStructureUsage` | enum | `MTLAccelerationStructure.h` | `MetalAccelerationStructureUsage` |
| `MTLAllocation` | protocol | `MTLAllocation.h` | `MetalAllocation` |
| `MTLArchitecture` | interface | `MTLDevice.h` | `MetalArchitecture::new` |
| `MTLArgument` | interface | `MTLArgument.h` | `MetalArgument::new` |
| `MTLArgumentAccess` | typedef | `MTLArgument.h` | `MetalArgumentAccess` |
| `MTLArgumentBuffersTier` | enum | `MTLDevice.h` | `argument_buffers_tier` module; `MetalDevice::argument_buffers_support` |
| `MTLArgumentDescriptor` | interface | `MTLDevice.h` | `ArgumentDescriptor`; `MetalDevice::new_argument_encoder_with_descriptors` |
| `MTLArgumentEncoder` | protocol | `MTLArgumentEncoder.h` | `ArgumentEncoder; MetalFunction::new_argument_encoder` |
| `MTLArgumentType` | enum | `MTLArgument.h` | `MetalArgumentType` |
| `MTLArrayType` | interface | `MTLArgument.h` | `MetalArrayType::new` |
| `MTLAttribute` | interface | `MTLLibrary.h` | `MetalAttribute::new` |
| `MTLAttributeDescriptor` | interface | `MTLStageInputOutputDescriptor.h` | `MetalAttributeDescriptor::new` |
| `MTLAttributeDescriptorArray` | interface | `MTLStageInputOutputDescriptor.h` | `MetalAttributeDescriptorArray::new` |
| `MTLAttributeFormat` | enum | `MTLStageInputOutputDescriptor.h` | `MetalAttributeFormat` |
| `MTLAutoreleasedArgument` | typedef | `MTLLibrary.h` | `MetalAutoreleasedArgument` |
| `MTLAutoreleasedComputePipelineReflection` | typedef | `MTLLibrary.h` | `MetalAutoreleasedComputePipelineReflection` |
| `MTLAutoreleasedRenderPipelineReflection` | typedef | `MTLLibrary.h` | `MetalAutoreleasedRenderPipelineReflection` |
| `MTLBarrierScope` | enum | `MTLCommandEncoder.h` | `MetalBarrierScope` |
| `MTLBinaryArchive` | protocol | `MTLBinaryArchive.h` | `BinaryArchive; MetalDevice::new_binary_archive` |
| `MTLBinaryArchiveDescriptor` | interface | `MTLBinaryArchive.h` | `MetalBinaryArchiveDescriptor::new` |
| `MTLBinaryArchiveDomain` | const | `MTLBinaryArchive.h` | `metal_binary_archive_domain()` |
| `MTLBinaryArchiveError` | enum | `MTLBinaryArchive.h` | `MetalBinaryArchiveError` |
| `MTLBinding` | protocol | `MTLArgument.h` | `MetalBinding` |
| `MTLBindingAccess` | enum | `MTLArgument.h` | `binding_access` module; `ArgumentDescriptor` |
| `MTLBindingType` | enum | `MTLArgument.h` | `MetalBindingType` |
| `MTLBlendFactor` | enum | `MTLRenderPipeline.h` | `blend_factor` module; `RenderPipelineColorAttachmentDescriptor` |
| `MTLBlendOperation` | enum | `MTLRenderPipeline.h` | `blend_operation` module; `RenderPipelineColorAttachmentDescriptor` |
| `MTLBlitCommandEncoder` | protocol | `MTLBlitCommandEncoder.h` | `BlitCommandEncoder; CommandBuffer::new_blit_command_encoder; CommandBuffer::blit_copy_buffer` |
| `MTLBlitOption` | enum | `MTLBlitCommandEncoder.h` | `MetalBlitOption` |
| `MTLBlitPassDescriptor` | interface | `MTLBlitPass.h` | `MetalBlitPassDescriptor::new` |
| `MTLBlitPassSampleBufferAttachmentDescriptor` | interface | `MTLBlitPass.h` | `MetalBlitPassSampleBufferAttachmentDescriptor::new` |
| `MTLBlitPassSampleBufferAttachmentDescriptorArray` | interface | `MTLBlitPass.h` | `MetalBlitPassSampleBufferAttachmentDescriptorArray::new` |
| `MTLBuffer` | protocol | `MTLBuffer.h` | `MetalBuffer; MetalDevice::new_buffer; Heap::new_buffer` |
| `MTLBufferBinding` | protocol | `MTLArgument.h` | `MetalBufferBinding` |
| `MTLBufferLayoutDescriptor` | interface | `MTLStageInputOutputDescriptor.h` | `MetalBufferLayoutDescriptor::new` |
| `MTLBufferLayoutDescriptorArray` | interface | `MTLStageInputOutputDescriptor.h` | `MetalBufferLayoutDescriptorArray::new` |
| `MTLBufferSparseTier` | enum | `MTLResource.h` | `MetalBufferSparseTier` |
| `MTLCPUCacheMode` | enum | `MTLResource.h` | `cpu_cache_mode module` |
| `MTLCaptureDestination` | enum | `MTLCaptureManager.h` | `capture_destination module; CaptureManager::supports_destination` |
| `MTLCaptureError` | enum | `MTLCaptureManager.h` | `MetalCaptureError` |
| `MTLCaptureErrorDomain` | const | `MTLCaptureManager.h` | `metal_capture_error_domain()` |
| `MTLCaptureManager` | interface | `MTLCaptureManager.h` | `CaptureManager::shared; CaptureManager::new_capture_scope_with_*` |
| `MTLCaptureScope` | protocol | `MTLCaptureScope.h` | `CaptureScope; CaptureManager::new_capture_scope_with_*` |
| `MTLColorWriteMask` | enum | `MTLRenderPipeline.h` | `color_write_mask` module; `RenderPipelineColorAttachmentDescriptor` |
| `MTLCommandBuffer` | protocol | `MTLCommandBuffer.h` | `CommandBuffer; CommandQueue::{new_command_buffer,new_command_buffer_with_unretained_references}` |
| `MTLCommandBufferDescriptor` | interface | `MTLCommandBuffer.h` | `MetalCommandBufferDescriptor::new` |
| `MTLCommandBufferEncoderInfo` | protocol | `MTLCommandBuffer.h` | `MetalCommandBufferEncoderInfo` |
| `MTLCommandBufferEncoderInfoErrorKey` | const | `MTLCommandBuffer.h` | `metal_command_buffer_encoder_info_error_key()` |
| `MTLCommandBufferError` | enum | `MTLCommandBuffer.h` | `MetalCommandBufferError` |
| `MTLCommandBufferErrorDomain` | const | `MTLCommandBuffer.h` | `metal_command_buffer_error_domain()` |
| `MTLCommandBufferErrorOption` | enum | `MTLCommandBuffer.h` | `MetalCommandBufferErrorOption` |
| `MTLCommandBufferStatus` | enum | `MTLCommandBuffer.h` | `command_buffer_status module; CommandBuffer::status` |
| `MTLCommandEncoder` | protocol | `MTLCommandEncoder.h` | `MetalCommandEncoder` |
| `MTLCommandEncoderErrorState` | enum | `MTLCommandBuffer.h` | `MetalCommandEncoderErrorState` |
| `MTLCommandQueue` | protocol | `MTLCommandQueue.h` | `CommandQueue; MetalDevice::{new_command_queue,new_command_queue_with_max_command_buffer_count,new_command_queue_with_log_state}` |
| `MTLCommandQueueDescriptor` | interface | `MTLCommandQueue.h` | `MetalCommandQueueDescriptor::new` |
| `MTLCommonCounter` | typedef | `MTLCounters.h` | `MetalCommonCounter`, `metal_common_counter_*()` |
| `MTLCommonCounterClipperInvocations` | const | `MTLCounters.h` | `metal_common_counter_clipper_invocations()` |
| `MTLCommonCounterClipperPrimitivesOut` | const | `MTLCounters.h` | `metal_common_counter_clipper_primitives_out()` |
| `MTLCommonCounterComputeKernelInvocations` | const | `MTLCounters.h` | `metal_common_counter_compute_kernel_invocations()` |
| `MTLCommonCounterFragmentCycles` | const | `MTLCounters.h` | `metal_common_counter_fragment_cycles()` |
| `MTLCommonCounterFragmentInvocations` | const | `MTLCounters.h` | `metal_common_counter_fragment_invocations()` |
| `MTLCommonCounterFragmentsPassed` | const | `MTLCounters.h` | `metal_common_counter_fragments_passed()` |
| `MTLCommonCounterPostTessellationVertexCycles` | const | `MTLCounters.h` | `metal_common_counter_post_tessellation_vertex_cycles()` |
| `MTLCommonCounterPostTessellationVertexInvocations` | const | `MTLCounters.h` | `metal_common_counter_post_tessellation_vertex_invocations()` |
| `MTLCommonCounterRenderTargetWriteCycles` | const | `MTLCounters.h` | `metal_common_counter_render_target_write_cycles()` |
| `MTLCommonCounterSet` | typedef | `MTLCounters.h` | `MetalCommonCounterSet`, `metal_common_counter_set_*()` |
| `MTLCommonCounterSetStageUtilization` | const | `MTLCounters.h` | `metal_common_counter_set_stage_utilization()` |
| `MTLCommonCounterSetStatistic` | const | `MTLCounters.h` | `metal_common_counter_set_statistic()` |
| `MTLCommonCounterSetTimestamp` | const | `MTLCounters.h` | `metal_common_counter_set_timestamp()` |
| `MTLCommonCounterTessellationCycles` | const | `MTLCounters.h` | `metal_common_counter_tessellation_cycles()` |
| `MTLCommonCounterTessellationInputPatches` | const | `MTLCounters.h` | `metal_common_counter_tessellation_input_patches()` |
| `MTLCommonCounterTimestamp` | const | `MTLCounters.h` | `metal_common_counter_timestamp()` |
| `MTLCommonCounterTotalCycles` | const | `MTLCounters.h` | `metal_common_counter_total_cycles()` |
| `MTLCommonCounterVertexCycles` | const | `MTLCounters.h` | `metal_common_counter_vertex_cycles()` |
| `MTLCommonCounterVertexInvocations` | const | `MTLCounters.h` | `metal_common_counter_vertex_invocations()` |
| `MTLCompareFunction` | enum | `MTLDepthStencil.h` | `compare_function` module; `StencilDescriptor`; `DepthStencilDescriptor`; `SamplerDescriptor` |
| `MTLCompileOptions` | interface | `MTLLibrary.h` | `MetalCompileOptions::new` |
| `MTLCompileSymbolVisibility` | enum | `MTLLibrary.h` | `MetalCompileSymbolVisibility` |
| `MTLComputeCommandEncoder` | protocol | `MTLComputeCommandEncoder.h` | `ComputeCommandEncoder; CommandBuffer::new_compute_command_encoder; CommandBuffer::dispatch_compute_1d` |
| `MTLComputePassDescriptor` | interface | `MTLComputePass.h` | `MetalComputePassDescriptor::new` |
| `MTLComputePassSampleBufferAttachmentDescriptor` | interface | `MTLComputePass.h` | `MetalComputePassSampleBufferAttachmentDescriptor::new` |
| `MTLComputePassSampleBufferAttachmentDescriptorArray` | interface | `MTLComputePass.h` | `MetalComputePassSampleBufferAttachmentDescriptorArray::new` |
| `MTLComputePipelineDescriptor` | interface | `MTLComputePipeline.h` | `ComputePipelineDescriptor`; `MetalDevice::new_compute_pipeline_state_with_descriptor` |
| `MTLComputePipelineReflection` | interface | `MTLComputePipeline.h` | `MetalComputePipelineReflection::new` |
| `MTLComputePipelineState` | protocol | `MTLComputePipeline.h` | `ComputePipelineState; MetalDevice::new_compute_pipeline_state` |
| `MTLCoordinate2D` | typedef | `MTLTypes.h` | `MetalCoordinate2D` |
| `MTLCopyAllDevices` | function | `MTLDevice.h` | `copy_all_devices` |
| `MTLCopyAllDevicesWithObserver` | function | `MTLDevice.h` | `copy_all_devices_with_observer`, `MetalDeviceObserver` |
| `MTLCounter` | protocol | `MTLCounters.h` | `MetalCounter` |
| `MTLCounterErrorDomain` | const | `MTLCounters.h` | `metal_counter_error_domain()` |
| `MTLCounterSampleBuffer` | protocol | `MTLCounters.h` | `CounterSampleBuffer; MetalDevice::new_counter_sample_buffer; BlitCommandEncoder::sample_counters` |
| `MTLCounterSampleBufferDescriptor` | interface | `MTLCounters.h` | `MetalCounterSampleBufferDescriptor::new` |
| `MTLCounterSampleBufferError` | enum | `MTLCounters.h` | `MetalCounterSampleBufferError` |
| `MTLCounterSamplingPoint` | enum | `MTLDevice.h` | `counter_sampling_point module; MetalDevice::supports_counter_sampling` |
| `MTLCounterSet` | protocol | `MTLCounters.h` | `MetalDevice::counter_set_names; MetalDevice::new_counter_sample_buffer` |
| `MTLCreateSystemDefaultDevice` | function | `MTLDevice.h` | `MetalDevice::system_default` |
| `MTLCullMode` | enum | `MTLRenderCommandEncoder.h` | `MetalCullMode` |
| `MTLCurveBasis` | enum | `MTLAccelerationStructure.h` | `MetalCurveBasis` |
| `MTLCurveEndCaps` | enum | `MTLAccelerationStructure.h` | `MetalCurveEndCaps` |
| `MTLCurveType` | enum | `MTLAccelerationStructure.h` | `MetalCurveType` |
| `MTLDataType` | enum | `MTLDataType.h` | `MetalDataType` |
| `MTLDepthClipMode` | enum | `MTLRenderCommandEncoder.h` | `MetalDepthClipMode` |
| `MTLDepthStencilDescriptor` | interface | `MTLDepthStencil.h` | `DepthStencilDescriptor`; `MetalDevice::new_depth_stencil_state` |
| `MTLDepthStencilState` | protocol | `MTLDepthStencil.h` | `DepthStencilState`; `MetalDevice::new_depth_stencil_state`; `RenderCommandEncoder::set_depth_stencil_state` |
| `MTLDevice` | protocol | `MTLDevice.h` | `MetalDevice` |
| `MTLDeviceLocation` | enum | `MTLDevice.h` | `MetalDeviceLocation` |
| `MTLDeviceNotificationName` | typedef | `MTLDevice.h` | `MetalDeviceNotificationName`, `metal_device_*_notification()` |
| `MTLDeviceRemovalRequestedNotification` | const | `MTLDevice.h` | `metal_device_removal_requested_notification()` |
| `MTLDeviceWasAddedNotification` | const | `MTLDevice.h` | `metal_device_was_added_notification()` |
| `MTLDeviceWasRemovedNotification` | const | `MTLDevice.h` | `metal_device_was_removed_notification()` |
| `MTLDispatchType` | enum | `MTLCommandBuffer.h` | `MetalDispatchType` |
| `MTLDrawable` | protocol | `MTLDrawable.h` | `MetalDrawable` |
| `MTLDynamicLibrary` | protocol | `MTLDynamicLibrary.h` | `DynamicLibrary; MetalDevice::{new_dynamic_library_with_source,load_dynamic_library}` |
| `MTLDynamicLibraryDomain` | const | `MTLDynamicLibrary.h` | `metal_dynamic_library_domain()` |
| `MTLDynamicLibraryError` | enum | `MTLDynamicLibrary.h` | `MetalDynamicLibraryError` |
| `MTLEvent` | protocol | `MTLEvent.h` | `Event; CommandBuffer::{encode_wait_for_event,encode_signal_event}` |
| `MTLFXFrameInterpolatableScaler` | protocol | `MTLFXTemporalScaler.h` | `FrameInterpolatableScaler` trait; `TemporalScaler` |
| `MTLFXFrameInterpolator` | protocol | `MTLFXFrameInterpolator.h` | `MetalFxFrameInterpolator` |
| `MTLFXFrameInterpolatorBase` | protocol | `MTLFXFrameInterpolator.h` | `MetalFxFrameInterpolatorBase` |
| `MTLFXFrameInterpolatorDescriptor` | interface | `MTLFXFrameInterpolator.h` | `MetalFxFrameInterpolatorDescriptor::new` |
| `MTLFXSpatialScaler` | protocol | `MTLFXSpatialScaler.h` | `SpatialScaler`; `MetalDevice::new_spatial_scaler` |
| `MTLFXSpatialScalerBase` | protocol | `MTLFXSpatialScaler.h` | `MetalFxSpatialScalerBase` |
| `MTLFXSpatialScalerColorProcessingMode` | enum | `MTLFXSpatialScaler.h` | `spatial_scaler_color_processing_mode` module; `SpatialScalerDescriptor` |
| `MTLFXSpatialScalerDescriptor` | interface | `MTLFXSpatialScaler.h` | `SpatialScalerDescriptor`; `MetalDevice::new_spatial_scaler` |
| `MTLFXTemporalDenoisedScaler` | protocol | `MTLFXTemporalDenoisedScaler.h` | `MetalFxTemporalDenoisedScaler` |
| `MTLFXTemporalDenoisedScalerBase` | protocol | `MTLFXTemporalDenoisedScaler.h` | `MetalFxTemporalDenoisedScalerBase` |
| `MTLFXTemporalDenoisedScalerDescriptor` | interface | `MTLFXTemporalDenoisedScaler.h` | `MetalFxTemporalDenoisedScalerDescriptor::new` |
| `MTLFXTemporalScaler` | protocol | `MTLFXTemporalScaler.h` | `TemporalScaler`; `MetalDevice::new_temporal_scaler` |
| `MTLFXTemporalScalerBase` | protocol | `MTLFXTemporalScaler.h` | `MetalFxTemporalScalerBase` |
| `MTLFXTemporalScalerDescriptor` | interface | `MTLFXTemporalScaler.h` | `TemporalScalerDescriptor`; `MetalDevice::new_temporal_scaler` |
| `MTLFeatureSet` | enum | `MTLDevice.h` | `MetalFeatureSet` |
| `MTLFence` | protocol | `MTLFence.h` | `Fence; MetalDevice::new_fence; encoder wait/update methods` |
| `MTLFunction` | protocol | `MTLLibrary.h` | `MetalFunction; MetalLibrary::new_function` |
| `MTLFunctionConstant` | interface | `MTLLibrary.h` | `MetalFunctionConstant::new` |
| `MTLFunctionConstantValues` | interface | `MTLFunctionConstantValues.h` | `MetalFunctionConstantValues::new` |
| `MTLFunctionDescriptor` | interface | `MTLFunctionDescriptor.h` | `MetalFunctionDescriptor::new` |
| `MTLFunctionHandle` | protocol | `MTLFunctionHandle.h` | `MetalFunctionHandle` |
| `MTLFunctionLog` | protocol | `MTLFunctionLog.h` | `MetalFunctionLog` |
| `MTLFunctionLogDebugLocation` | protocol | `MTLFunctionLog.h` | `MetalFunctionLogDebugLocation` |
| `MTLFunctionLogType` | enum | `MTLFunctionLog.h` | `MetalFunctionLogType` |
| `MTLFunctionOptions` | enum | `MTLFunctionDescriptor.h` | `MetalFunctionOptions` |
| `MTLFunctionReflection` | interface | `MTLLibrary.h` | `MetalFunctionReflection::new` |
| `MTLFunctionStitchingAttribute` | protocol | `MTLFunctionStitching.h` | `MetalFunctionStitchingAttribute` |
| `MTLFunctionStitchingAttributeAlwaysInline` | interface | `MTLFunctionStitching.h` | `MetalFunctionStitchingAttributeAlwaysInline::new` |
| `MTLFunctionStitchingFunctionNode` | interface | `MTLFunctionStitching.h` | `MetalFunctionStitchingFunctionNode::new` |
| `MTLFunctionStitchingGraph` | interface | `MTLFunctionStitching.h` | `MetalFunctionStitchingGraph::new` |
| `MTLFunctionStitchingInputNode` | interface | `MTLFunctionStitching.h` | `MetalFunctionStitchingInputNode::new` |
| `MTLFunctionStitchingNode` | protocol | `MTLFunctionStitching.h` | `MetalFunctionStitchingNode` |
| `MTLFunctionType` | enum | `MTLLibrary.h` | `MetalFunctionType` |
| `MTLGPUAddress` | struct | `MTL4BufferRange.h` | `MetalGpuAddress` |
| `MTLGPUFamily` | enum | `MTLDevice.h` | `gpu_family module; MetalDevice::supports_family` |
| `MTLHazardTrackingMode` | enum | `MTLResource.h` | `hazard_tracking_mode module` |
| `MTLHeap` | protocol | `MTLHeap.h` | `Heap; MetalDevice::new_heap` |
| `MTLHeapDescriptor` | interface | `MTLHeap.h` | `MetalHeapDescriptor::new` |
| `MTLHeapType` | enum | `MTLHeap.h` | `MetalHeapType` |
| `MTLIOCommandBuffer` | protocol | `MTLIOCommandBuffer.h` | `MetalIoCommandBuffer` |
| `MTLIOCommandQueue` | protocol | `MTLIOCommandQueue.h` | `MetalIoCommandQueue` |
| `MTLIOCommandQueueDescriptor` | interface | `MTLIOCommandQueue.h` | `MetalIoCommandQueueDescriptor::new` |
| `MTLIOCommandQueueType` | enum | `MTLIOCommandQueue.h` | `MetalIoCommandQueueType` |
| `MTLIOCompressionContext` | typedef | `MTLIOCompressor.h` | `MetalIoCompressionContext` |
| `MTLIOCompressionContextAppendData` | function | `MTLIOCompressor.h` | `MetalIoCompressionContext::append_data` |
| `MTLIOCompressionContextDefaultChunkSize` | function | `MTLIOCompressor.h` | `io_compression_context_default_chunk_size` |
| `MTLIOCompressionMethod` | enum | `MTLDevice.h` | `MetalIoCompressionMethod` |
| `MTLIOCompressionStatus` | enum | `MTLIOCompressor.h` | `MetalIoCompressionStatus` |
| `MTLIOCreateCompressionContext` | function | `MTLIOCompressor.h` | `create_io_compression_context`, `MetalIoCompressionContext` |
| `MTLIOErrorDomain` | const | `MTLIOCommandQueue.h` | `metal_io_error_domain()` |
| `MTLIOFileHandle` | protocol | `MTLIOCommandQueue.h` | `MetalIoFileHandle` |
| `MTLIOFlushAndDestroyCompressionContext` | function | `MTLIOCompressor.h` | `MetalIoCompressionContext::flush_and_destroy` |
| `MTLIOPriority` | enum | `MTLIOCommandQueue.h` | `MetalIoPriority` |
| `MTLIOScratchBuffer` | protocol | `MTLIOCommandQueue.h` | `MetalIoScratchBuffer` |
| `MTLIOScratchBufferAllocator` | protocol | `MTLIOCommandQueue.h` | `MetalIoScratchBufferAllocator` |
| `MTLIOStatus` | enum | `MTLIOCommandBuffer.h` | `MetalIoStatus` |
| `MTLIndexType` | enum | `MTLArgument.h` | `MetalIndexType` |
| `MTLIndirectCommandBuffer` | protocol | `MTLIndirectCommandBuffer.h` | `IndirectCommandBuffer; MetalDevice::new_indirect_command_buffer` |
| `MTLIndirectCommandBufferDescriptor` | interface | `MTLIndirectCommandBuffer.h` | `MetalIndirectCommandBufferDescriptor::new` |
| `MTLIndirectCommandType` | enum | `MTLIndirectCommandBuffer.h` | `indirect_command_type module; MetalDevice::new_indirect_command_buffer` |
| `MTLIndirectComputeCommand` | protocol | `MTLIndirectCommandEncoder.h` | `MetalIndirectComputeCommand` |
| `MTLIndirectInstanceAccelerationStructureDescriptor` | interface | `MTLAccelerationStructure.h` | `MetalIndirectInstanceAccelerationStructureDescriptor::new` |
| `MTLIndirectRenderCommand` | protocol | `MTLIndirectCommandEncoder.h` | `MetalIndirectRenderCommand` |
| `MTLInstanceAccelerationStructureDescriptor` | interface | `MTLAccelerationStructure.h` | `MetalInstanceAccelerationStructureDescriptor::new` |
| `MTLIntersectionFunctionDescriptor` | interface | `MTLFunctionDescriptor.h` | `MetalIntersectionFunctionDescriptor::new` |
| `MTLIntersectionFunctionSignature` | enum | `MTLIntersectionFunctionTable.h` | `intersection_function_signature module; IntersectionFunctionTable::set_opaque_triangle_intersection_function` |
| `MTLIntersectionFunctionTable` | protocol | `MTLIntersectionFunctionTable.h` | `IntersectionFunctionTable; ComputeCommandEncoder::set_intersection_function_table; ComputePipelineState::new_intersection_function_table` |
| `MTLIntersectionFunctionTableDescriptor` | interface | `MTLIntersectionFunctionTable.h` | `MetalIntersectionFunctionTableDescriptor::new` |
| `MTLLanguageVersion` | enum | `MTLLibrary.h` | `MetalLanguageVersion` |
| `MTLLibrary` | protocol | `MTLLibrary.h` | `MetalLibrary; MetalDevice::new_library_with_source` |
| `MTLLibraryError` | enum | `MTLLibrary.h` | `MetalLibraryError` |
| `MTLLibraryErrorDomain` | const | `MTLLibrary.h` | `metal_library_error_domain()` |
| `MTLLibraryOptimizationLevel` | enum | `MTLLibrary.h` | `MetalLibraryOptimizationLevel` |
| `MTLLibraryType` | enum | `MTLLibrary.h` | `MetalLibraryType` |
| `MTLLinkedFunctions` | interface | `MTLLinkedFunctions.h` | `MetalLinkedFunctions::new` |
| `MTLLoadAction` | enum | `MTLRenderPass.h` | `load_action module; CommandBuffer::new_render_command_encoder` |
| `MTLLogContainer` | protocol | `MTLFunctionLog.h` | `MetalLogContainer` |
| `MTLLogLevel` | enum | `MTLLogState.h` | `log_level module; MetalDevice::new_log_state` |
| `MTLLogState` | protocol | `MTLLogState.h` | `LogState; MetalDevice::new_log_state; MetalDevice::new_command_queue_with_log_state` |
| `MTLLogStateDescriptor` | interface | `MTLLogState.h` | `MetalLogStateDescriptor::new` |
| `MTLLogStateError` | enum | `MTLLogState.h` | `MetalLogStateError` |
| `MTLLogStateErrorDomain` | const | `MTLLogState.h` | `metal_log_state_error_domain()` |
| `MTLLogicalToPhysicalColorAttachmentMap` | interface | `MTLRenderPipeline.h` | `MetalLogicalToPhysicalColorAttachmentMap::new` |
| `MTLMathFloatingPointFunctions` | enum | `MTLLibrary.h` | `MetalMathFloatingPointFunctions` |
| `MTLMathMode` | enum | `MTLLibrary.h` | `MetalMathMode` |
| `MTLMatrixLayout` | enum | `MTLAccelerationStructure.h` | `MetalMatrixLayout` |
| `MTLMeshRenderPipelineDescriptor` | interface | `MTLRenderPipeline.h` | `MetalMeshRenderPipelineDescriptor::new` |
| `MTLMotionBorderMode` | enum | `MTLAccelerationStructure.h` | `MetalMotionBorderMode` |
| `MTLMotionKeyframeData` | interface | `MTLAccelerationStructure.h` | `MetalMotionKeyframeData::new` |
| `MTLMultisampleDepthResolveFilter` | enum | `MTLRenderPass.h` | `MetalMultisampleDepthResolveFilter` |
| `MTLMultisampleStencilResolveFilter` | enum | `MTLRenderPass.h` | `MetalMultisampleStencilResolveFilter` |
| `MTLMutability` | enum | `MTLPipeline.h` | `MetalMutability` |
| `MTLNewComputePipelineStateCompletionHandler` | typedef | `MTLLibrary.h` | `MetalNewComputePipelineStateCompletionHandler` |
| `MTLNewComputePipelineStateWithReflectionCompletionHandler` | typedef | `MTLLibrary.h` | `MetalNewComputePipelineStateWithReflectionCompletionHandler` |
| `MTLNewDynamicLibraryCompletionHandler` | typedef | `MTLLibrary.h` | `MetalNewDynamicLibraryCompletionHandler` |
| `MTLNewLibraryCompletionHandler` | typedef | `MTLLibrary.h` | `MetalNewLibraryCompletionHandler` |
| `MTLNewRenderPipelineStateCompletionHandler` | typedef | `MTLLibrary.h` | `MetalNewRenderPipelineStateCompletionHandler` |
| `MTLNewRenderPipelineStateWithReflectionCompletionHandler` | typedef | `MTLLibrary.h` | `MetalNewRenderPipelineStateWithReflectionCompletionHandler` |
| `MTLObjectPayloadBinding` | protocol | `MTLArgument.h` | `MetalObjectPayloadBinding` |
| `MTLOrigin` | struct | `MTLTypes.h` | `MetalOrigin` |
| `MTLPackedFloat3` | typedef | `MTLAccelerationStructureTypes.h` | `MetalPackedFloat3` |
| `MTLPackedFloat4x3` | struct | `MTLAccelerationStructure.h` | `MetalPackedFloat4x3` |
| `MTLPackedFloatQuaternion` | typedef | `MTLAccelerationStructureTypes.h` | `MetalPackedFloatQuaternion` |
| `MTLParallelRenderCommandEncoder` | protocol | `MTLParallelRenderCommandEncoder.h` | `MetalParallelRenderCommandEncoder` |
| `MTLPatchType` | enum | `MTLLibrary.h` | `MetalPatchType` |
| `MTLPipelineBufferDescriptor` | interface | `MTLPipeline.h` | `MetalPipelineBufferDescriptor::new` |
| `MTLPipelineBufferDescriptorArray` | interface | `MTLPipeline.h` | `MetalPipelineBufferDescriptorArray::new` |
| `MTLPipelineOption` | enum | `MTLDevice.h` | `MetalPipelineOption` |
| `MTLPixelFormat` | enum | `MTLPixelFormat.h` | `pixel_format module; TextureDescriptor; MetalTexture::pixel_format` |
| `MTLPointerType` | interface | `MTLArgument.h` | `MetalPointerType::new` |
| `MTLPrimitiveAccelerationStructureDescriptor` | interface | `MTLAccelerationStructure.h` | `MetalPrimitiveAccelerationStructureDescriptor::new` |
| `MTLPrimitiveTopologyClass` | enum | `MTLRenderPipeline.h` | `MetalPrimitiveTopologyClass` |
| `MTLPrimitiveType` | enum | `MTLRenderCommandEncoder.h` | `primitive_type module; RenderCommandEncoder::draw_primitives` |
| `MTLPurgeableState` | enum | `MTLResource.h` | `purgeable_state module; Heap::set_purgeable_state` |
| `MTLRasterizationRateLayerArray` | interface | `MTLRasterizationRate.h` | `MetalRasterizationRateLayerArray::new` |
| `MTLRasterizationRateLayerDescriptor` | interface | `MTLRasterizationRate.h` | `MetalRasterizationRateLayerDescriptor::new` |
| `MTLRasterizationRateMap` | protocol | `MTLRasterizationRate.h` | `MetalRasterizationRateMap` |
| `MTLRasterizationRateMapDescriptor` | interface | `MTLRasterizationRate.h` | `MetalRasterizationRateMapDescriptor::new` |
| `MTLRasterizationRateSampleArray` | interface | `MTLRasterizationRate.h` | `MetalRasterizationRateSampleArray::new` |
| `MTLReadWriteTextureTier` | enum | `MTLDevice.h` | `MetalReadWriteTextureTier` |
| `MTLRegion` | struct | `MTL4CommandQueue.h` | `MetalRegion` |
| `MTLRemoveDeviceObserver` | function | `MTLDevice.h` | `remove_device_observer`, `MetalDeviceObserver::remove` |
| `MTLRenderCommandEncoder` | protocol | `MTLRenderCommandEncoder.h` | `RenderCommandEncoder; CommandBuffer::new_render_command_encoder` |
| `MTLRenderPassAttachmentDescriptor` | interface | `MTLRenderPass.h` | `MetalRenderPassAttachmentDescriptor::new` |
| `MTLRenderPassColorAttachmentDescriptor` | interface | `MTLRenderPass.h` | `MetalRenderPassColorAttachmentDescriptor::new` |
| `MTLRenderPassColorAttachmentDescriptorArray` | interface | `MTLRenderPass.h` | `MetalRenderPassColorAttachmentDescriptorArray::new` |
| `MTLRenderPassDepthAttachmentDescriptor` | interface | `MTLRenderPass.h` | `MetalRenderPassDepthAttachmentDescriptor::new` |
| `MTLRenderPassDescriptor` | interface | `MTLRenderPass.h` | `MetalRenderPassDescriptor::new` |
| `MTLRenderPassSampleBufferAttachmentDescriptor` | interface | `MTLRenderPass.h` | `MetalRenderPassSampleBufferAttachmentDescriptor::new` |
| `MTLRenderPassSampleBufferAttachmentDescriptorArray` | interface | `MTLRenderPass.h` | `MetalRenderPassSampleBufferAttachmentDescriptorArray::new` |
| `MTLRenderPassStencilAttachmentDescriptor` | interface | `MTLRenderPass.h` | `MetalRenderPassStencilAttachmentDescriptor::new` |
| `MTLRenderPipelineColorAttachmentDescriptor` | interface | `MTLRenderPipeline.h` | `RenderPipelineColorAttachmentDescriptor`; `RenderPipelineDescriptor` |
| `MTLRenderPipelineColorAttachmentDescriptorArray` | interface | `MTLRenderPipeline.h` | `MetalRenderPipelineColorAttachmentDescriptorArray::new` |
| `MTLRenderPipelineDescriptor` | interface | `MTLRenderPipeline.h` | `RenderPipelineDescriptor`; `MetalDevice::new_render_pipeline_state_with_descriptor` |
| `MTLRenderPipelineFunctionsDescriptor` | interface | `MTLRenderPipeline.h` | `MetalRenderPipelineFunctionsDescriptor::new` |
| `MTLRenderPipelineReflection` | interface | `MTLRenderPipeline.h` | `MetalRenderPipelineReflection::new` |
| `MTLRenderPipelineState` | protocol | `MTLRenderPipeline.h` | `RenderPipelineState; MetalDevice::new_render_pipeline_state` |
| `MTLRenderStages` | enum | `MTLRenderCommandEncoder.h` | `MetalRenderStages` |
| `MTLResidencySet` | protocol | `MTLResidencySet.h` | `ResidencySet; MetalDevice::new_residency_set; CommandQueue::{add_residency_set,remove_residency_set}` |
| `MTLResidencySetDescriptor` | interface | `MTLResidencySet.h` | `MetalResidencySetDescriptor::new` |
| `MTLResource` | protocol | `MTLResource.h` | `MetalResource` |
| `MTLResourceID` | struct | `MTLTypes.h` | `MetalResourceId` |
| `MTLResourceOptions` | enum | `MTLResource.h` | `resource_options module; MetalDevice::new_buffer; Heap::new_buffer` |
| `MTLResourceStateCommandEncoder` | protocol | `MTLResourceStateCommandEncoder.h` | `MetalResourceStateCommandEncoder` |
| `MTLResourceStatePassDescriptor` | interface | `MTLResourceStatePass.h` | `MetalResourceStatePassDescriptor::new` |
| `MTLResourceStatePassSampleBufferAttachmentDescriptor` | interface | `MTLResourceStatePass.h` | `MetalResourceStatePassSampleBufferAttachmentDescriptor::new` |
| `MTLResourceStatePassSampleBufferAttachmentDescriptorArray` | interface | `MTLResourceStatePass.h` | `MetalResourceStatePassSampleBufferAttachmentDescriptorArray::new` |
| `MTLResourceUsage` | enum | `MTLCommandEncoder.h` | `MetalResourceUsage` |
| `MTLResourceViewPool` | protocol | `MTLResourceViewPool.h` | `MetalResourceViewPool` |
| `MTLResourceViewPoolDescriptor` | interface | `MTLResourceViewPool.h` | `MetalResourceViewPoolDescriptor::new` |
| `MTLSamplerAddressMode` | enum | `MTLSampler.h` | `sampler_address_mode` module; `SamplerDescriptor` |
| `MTLSamplerBorderColor` | enum | `MTLSampler.h` | `sampler_border_color` module; `SamplerDescriptor` |
| `MTLSamplerDescriptor` | interface | `MTLSampler.h` | `SamplerDescriptor`; `MetalDevice::new_sampler_state` |
| `MTLSamplerMinMagFilter` | enum | `MTLSampler.h` | `sampler_min_mag_filter` module; `SamplerDescriptor` |
| `MTLSamplerMipFilter` | enum | `MTLSampler.h` | `sampler_mip_filter` module; `SamplerDescriptor` |
| `MTLSamplerReductionMode` | enum | `MTLSampler.h` | `sampler_reduction_mode` module; `SamplerDescriptor` |
| `MTLSamplerState` | protocol | `MTLSampler.h` | `SamplerState`; `MetalDevice::new_sampler_state`; sampler binding helpers on compute/render/argument encoders |
| `MTLShaderValidation` | enum | `MTLPipeline.h` | `MetalShaderValidation` |
| `MTLSharedEvent` | protocol | `MTLEvent.h` | `Event; MetalDevice::new_shared_event` |
| `MTLSharedEventHandle` | interface | `MTLEvent.h` | `MetalSharedEventHandle::new` |
| `MTLSharedEventListener` | interface | `MTLEvent.h` | `MetalSharedEventListener::new` |
| `MTLSharedTextureHandle` | interface | `MTLTexture.h` | `MetalSharedTextureHandle::new` |
| `MTLSparsePageSize` | enum | `MTLResource.h` | `MetalSparsePageSize` |
| `MTLSparseTextureMappingMode` | struct | `MTL4CommandQueue.h` | `MetalSparseTextureMappingMode` |
| `MTLSparseTextureRegionAlignmentMode` | enum | `MTLDevice.h` | `MetalSparseTextureRegionAlignmentMode` |
| `MTLStageInputOutputDescriptor` | interface | `MTLStageInputOutputDescriptor.h` | `MetalStageInputOutputDescriptor::new` |
| `MTLStages` | enum | `MTLCommandEncoder.h` | `MetalStages` |
| `MTLStencilDescriptor` | interface | `MTLDepthStencil.h` | `StencilDescriptor`; `DepthStencilDescriptor` |
| `MTLStencilOperation` | enum | `MTLDepthStencil.h` | `stencil_operation` module; `StencilDescriptor` |
| `MTLStepFunction` | enum | `MTLStageInputOutputDescriptor.h` | `MetalStepFunction` |
| `MTLStitchedLibraryDescriptor` | interface | `MTLFunctionStitching.h` | `MetalStitchedLibraryDescriptor::new` |
| `MTLStitchedLibraryOptions` | enum | `MTLFunctionStitching.h` | `MetalStitchedLibraryOptions` |
| `MTLStorageMode` | enum | `MTLResource.h` | `storage_mode module; TextureDescriptor; MetalDevice::new_heap` |
| `MTLStoreAction` | enum | `MTLRenderPass.h` | `store_action module; CommandBuffer::new_render_command_encoder` |
| `MTLStoreActionOptions` | enum | `MTLRenderPass.h` | `MetalStoreActionOptions` |
| `MTLStructMember` | interface | `MTLArgument.h` | `MetalStructMember::new` |
| `MTLStructType` | interface | `MTLArgument.h` | `MetalStructType::new` |
| `MTLTensor` | protocol | `MTLTensor.h` | `MetalTensor` |
| `MTLTensorBinding` | protocol | `MTLArgument.h` | `MetalTensorBinding` |
| `MTLTensorDataType` | enum | `MTLTensor.h` | `MetalTensorDataType` |
| `MTLTensorDescriptor` | interface | `MTLTensor.h` | `MetalTensorDescriptor::new` |
| `MTLTensorDomain` | const | `MTLTensor.h` | `metal_tensor_domain()` |
| `MTLTensorError` | enum | `MTLTensor.h` | `MetalTensorError` |
| `MTLTensorExtents` | interface | `MTLTensor.h` | `MetalTensorExtents::new` |
| `MTLTensorReferenceType` | interface | `MTLArgument.h` | `MetalTensorReferenceType::new` |
| `MTLTensorUsage` | enum | `MTLTensor.h` | `MetalTensorUsage` |
| `MTLTessellationControlPointIndexType` | enum | `MTLRenderPipeline.h` | `MetalTessellationControlPointIndexType` |
| `MTLTessellationFactorFormat` | enum | `MTLRenderPipeline.h` | `MetalTessellationFactorFormat` |
| `MTLTessellationFactorStepFunction` | enum | `MTLRenderPipeline.h` | `MetalTessellationFactorStepFunction` |
| `MTLTessellationPartitionMode` | enum | `MTLRenderPipeline.h` | `MetalTessellationPartitionMode` |
| `MTLTexture` | protocol | `MTLTexture.h` | `MetalTexture; MetalDevice::new_texture; IOSurfaceMetalExt::create_metal_texture` |
| `MTLTextureBinding` | protocol | `MTLArgument.h` | `MetalTextureBinding` |
| `MTLTextureCompressionType` | enum | `MTLTexture.h` | `MetalTextureCompressionType` |
| `MTLTextureDescriptor` | interface | `MTLTexture.h` | `TextureDescriptor; MetalDevice::new_texture; Heap::new_texture` |
| `MTLTextureReferenceType` | interface | `MTLArgument.h` | `MetalTextureReferenceType::new` |
| `MTLTextureSparseTier` | enum | `MTLResource.h` | `MetalTextureSparseTier` |
| `MTLTextureSwizzle` | enum | `MTLTexture.h` | `MetalTextureSwizzle` |
| `MTLTextureType` | enum | `MTLTexture.h` | `texture_type` module; `ArgumentDescriptor` |
| `MTLTextureUsage` | enum | `MTLTexture.h` | `texture_usage module; TextureDescriptor; MetalTexture::usage` |
| `MTLTextureViewDescriptor` | interface | `MTLTexture.h` | `MetalTextureViewDescriptor::new` |
| `MTLTextureViewPool` | protocol | `MTLTextureViewPool.h` | `MetalTextureViewPool` |
| `MTLThreadgroupBinding` | protocol | `MTLArgument.h` | `MetalThreadgroupBinding` |
| `MTLTileRenderPipelineColorAttachmentDescriptor` | interface | `MTLRenderPipeline.h` | `TileRenderPipelineColorAttachmentDescriptor`; `TileRenderPipelineDescriptor` |
| `MTLTileRenderPipelineColorAttachmentDescriptorArray` | interface | `MTLRenderPipeline.h` | `MetalTileRenderPipelineColorAttachmentDescriptorArray::new` |
| `MTLTileRenderPipelineDescriptor` | interface | `MTLRenderPipeline.h` | `TileRenderPipelineDescriptor`; `MetalDevice::new_tile_render_pipeline_state` |
| `MTLTimestamp` | typedef | `MTLDevice.h` | `MetalTimestamp` |
| `MTLTransformType` | enum | `MTLAccelerationStructure.h` | `MetalTransformType` |
| `MTLTriangleFillMode` | enum | `MTLRenderCommandEncoder.h` | `MetalTriangleFillMode` |
| `MTLType` | interface | `MTLArgument.h` | `MetalType::new` |
| `MTLVertexAttribute` | interface | `MTLLibrary.h` | `MetalVertexAttribute::new` |
| `MTLVertexAttributeDescriptor` | interface | `MTLVertexDescriptor.h` | `MetalVertexAttributeDescriptor::new` |
| `MTLVertexAttributeDescriptorArray` | interface | `MTLVertexDescriptor.h` | `MetalVertexAttributeDescriptorArray::new` |
| `MTLVertexBufferLayoutDescriptor` | interface | `MTLVertexDescriptor.h` | `MetalVertexBufferLayoutDescriptor::new` |
| `MTLVertexBufferLayoutDescriptorArray` | interface | `MTLVertexDescriptor.h` | `MetalVertexBufferLayoutDescriptorArray::new` |
| `MTLVertexDescriptor` | interface | `MTLVertexDescriptor.h` | `MetalVertexDescriptor::new` |
| `MTLVertexFormat` | enum | `MTLVertexDescriptor.h` | `MetalVertexFormat` |
| `MTLVertexStepFunction` | enum | `MTLVertexDescriptor.h` | `MetalVertexStepFunction` |
| `MTLVisibilityResultMode` | enum | `MTLRenderCommandEncoder.h` | `MetalVisibilityResultMode` |
| `MTLVisibilityResultType` | enum | `MTLRenderPass.h` | `MetalVisibilityResultType` |
| `MTLVisibleFunctionTable` | protocol | `MTLVisibleFunctionTable.h` | `VisibleFunctionTable; ComputeCommandEncoder::set_visible_function_table; ComputePipelineState::new_visible_function_table` |
| `MTLVisibleFunctionTableDescriptor` | interface | `MTLVisibleFunctionTable.h` | `MetalVisibleFunctionTableDescriptor::new` |
| `MTLWinding` | enum | `MTLRenderCommandEncoder.h` | `MetalWinding` |
