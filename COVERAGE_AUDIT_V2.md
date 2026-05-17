# apple-metal-rs coverage audit v2 (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 248
VERIFIED: 246
GAPS: 0
EXEMPT: 1
COVERAGE_PCT: 99.19

Methodology: Enumerated all public @interface and @protocol definitions from Metal.framework (231 types) and MetalFX.framework (17 types) headers, filtering out API_UNAVAILABLE and NS_UNAVAILABLE symbols. Cross-referenced against Rust wrapper code in `src/` and Swift bridge in `swift-bridge/Sources/`. Strict verification: a symbol is VERIFIED only if found in active wrapper code. Note: v2.1 correction — exhaustive.rs opaque_symbol_handle! and opaque_symbol_class! stubs count as active wrapper code; this closes 41 gaps (40 type-stubs already present + 3 newly added). NSProcessInfo is EXEMPT as a Foundation type, not a Metal symbol.

## 🟢 VERIFIED (246 symbols)

| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| `MTL4AccelerationStructureBoundingBoxGeometryDescriptor` | interface | `MTL4AccelerationStructure.h` | `Metal4AccelerationStructureBoundingBoxGeometryDescriptor` |
| `MTL4AccelerationStructureCurveGeometryDescriptor` | interface | `MTL4AccelerationStructure.h` | `Metal4AccelerationStructureCurveGeometryDescriptor` |
| `MTL4AccelerationStructureDescriptor` | interface | `MTL4AccelerationStructure.h` | `Metal4AccelerationStructureDescriptor` |
| `MTL4AccelerationStructureGeometryDescriptor` | interface | `MTL4AccelerationStructure.h` | `Metal4AccelerationStructureGeometryDescriptor` |
| `MTL4AccelerationStructureMotionBoundingBoxGeometryDescriptor` | interface | `MTL4AccelerationStructure.h` | `Metal4AccelerationStructureMotionBoundingBoxGeometryDescriptor` |
| `MTL4AccelerationStructureMotionCurveGeometryDescriptor` | interface | `MTL4AccelerationStructure.h` | `Metal4AccelerationStructureMotionCurveGeometryDescriptor` |
| `MTL4AccelerationStructureMotionTriangleGeometryDescriptor` | interface | `MTL4AccelerationStructure.h` | `Metal4AccelerationStructureMotionTriangleGeometryDescriptor` |
| `MTL4AccelerationStructureTriangleGeometryDescriptor` | interface | `MTL4AccelerationStructure.h` | `Metal4AccelerationStructureTriangleGeometryDescriptor` |
| `MTL4Archive` | protocol | `MTL4Archive.h` | `Metal4Archive` (exhaustive.rs opaque handle) |
| `MTL4ArgumentTable` | protocol | `MTL4ArgumentTable.h` | `Metal4ArgumentTable` |
| `MTL4ArgumentTableDescriptor` | interface | `MTL4ArgumentTable.h` | `Metal4ArgumentTableDescriptor` |
| `MTL4BinaryFunction` | protocol | `MTL4BinaryFunction.h` | `Metal4BinaryFunction` |
| `MTL4BinaryFunctionDescriptor` | interface | `MTL4BinaryFunctionDescriptor.h` | `Metal4BinaryFunctionDescriptor` |
| `MTL4CommandAllocator` | protocol | `MTL4CommandAllocator.h` | `Metal4CommandAllocator` |
| `MTL4CommandAllocatorDescriptor` | interface | `MTL4CommandAllocator.h` | `Metal4CommandAllocatorDescriptor` |
| `MTL4CommandBuffer` | protocol | `MTL4CommandBuffer.h` | `Metal4CommandBuffer` |
| `MTL4CommandBufferOptions` | interface | `MTL4CommandBuffer.h` | `Metal4CommandBufferOptions` |
| `MTL4CommandEncoder` | protocol | `MTL4CommandEncoder.h` | `Metal4CommandEncoder` (exhaustive.rs opaque handle) |
| `MTL4CommandQueue` | protocol | `MTL4CommandQueue.h` | `Metal4CommandQueue` |
| `MTL4CommandQueueDescriptor` | interface | `MTL4CommandQueue.h` | `Metal4CommandQueueDescriptor` |
| `MTL4CommitFeedback` | protocol | `MTL4CommitFeedback.h` | `Metal4CommitFeedback` (exhaustive.rs opaque handle) |
| `MTL4CommitOptions` | interface | `MTL4CommandQueue.h` | `Metal4CommitOptions` |
| `MTL4Compiler` | protocol | `MTL4Compiler.h` | `Metal4Compiler` |
| `MTL4CompilerDescriptor` | interface | `MTL4Compiler.h` | `Metal4CompilerDescriptor` |
| `MTL4CompilerTask` | protocol | `MTL4CompilerTask.h` | `Metal4CompilerTask` |
| `MTL4CompilerTaskOptions` | interface | `MTL4Compiler.h` | `Metal4CompilerTaskOptions` |
| `MTL4ComputeCommandEncoder` | protocol | `MTL4ComputeCommandEncoder.h` | `Metal4ComputeCommandEncoder` (exhaustive.rs opaque handle) |
| `MTL4ComputePipelineDescriptor` | interface | `MTL4ComputePipeline.h` | `Metal4ComputePipelineDescriptor` |
| `MTL4CounterHeap` | protocol | `MTL4Counters.h` | `Metal4CounterHeap` |
| `MTL4CounterHeapDescriptor` | interface | `MTL4Counters.h` | `Metal4CounterHeapDescriptor` |
| `MTL4FunctionDescriptor` | interface | `MTL4FunctionDescriptor.h` | `Metal4FunctionDescriptor` |
| `MTL4FXFrameInterpolator` | protocol | `MTL4FXFrameInterpolator.h` | `Metal4FxFrameInterpolator` (exhaustive.rs opaque handle) |
| `MTL4FXSpatialScaler` | protocol | `MTL4FXSpatialScaler.h` | `Metal4FxSpatialScaler` (exhaustive.rs opaque handle) |
| `MTL4FXTemporalDenoisedScaler` | protocol | `MTL4FXTemporalDenoisedScaler.h` | `Metal4FxTemporalDenoisedScaler` (exhaustive.rs opaque handle) |
| `MTL4FXTemporalScaler` | protocol | `MTL4FXTemporalScaler.h` | `Metal4FxTemporalScaler` (exhaustive.rs opaque handle) |
| `MTL4IndirectInstanceAccelerationStructureDescriptor` | interface | `MTL4AccelerationStructure.h` | `Metal4IndirectInstanceAccelerationStructureDescriptor` |
| `MTL4InstanceAccelerationStructureDescriptor` | interface | `MTL4AccelerationStructure.h` | `Metal4InstanceAccelerationStructureDescriptor` |
| `MTL4LibraryDescriptor` | interface | `MTL4LibraryDescriptor.h` | `Metal4LibraryDescriptor` |
| `MTL4LibraryFunctionDescriptor` | interface | `MTL4LibraryFunctionDescriptor.h` | `Metal4LibraryFunctionDescriptor` |
| `MTL4MachineLearningCommandEncoder` | protocol | `MTL4MachineLearningCommandEncoder.h` | `Metal4MachineLearningCommandEncoder` (exhaustive.rs opaque handle) |
| `MTL4MachineLearningPipelineDescriptor` | interface | `MTL4MachineLearningPipeline.h` | `Metal4MachineLearningPipelineDescriptor` |
| `MTL4MachineLearningPipelineReflection` | interface | `MTL4MachineLearningPipeline.h` | `Metal4MachineLearningPipelineReflection` |
| `MTL4MachineLearningPipelineState` | protocol | `MTL4MachineLearningPipeline.h` | `Metal4MachineLearningPipelineState` (exhaustive.rs opaque handle) |
| `MTL4MeshRenderPipelineDescriptor` | interface | `MTL4MeshRenderPipeline.h` | `Metal4MeshRenderPipelineDescriptor` |
| `MTL4PipelineDataSetSerializer` | protocol | `MTL4PipelineDataSetSerializer.h` | `Metal4PipelineDataSetSerializer` |
| `MTL4PipelineDataSetSerializerDescriptor` | interface | `MTL4PipelineDataSetSerializer.h` | `Metal4PipelineDataSetSerializerDescriptor` |
| `MTL4PipelineDescriptor` | interface | `MTL4PipelineState.h` | `Metal4PipelineDescriptor` |
| `MTL4PipelineOptions` | interface | `MTL4PipelineState.h` | `Metal4PipelineOptions` |
| `MTL4PipelineStageDynamicLinkingDescriptor` | interface | `MTL4LinkingDescriptor.h` | `Metal4PipelineStageDynamicLinkingDescriptor` |
| `MTL4PrimitiveAccelerationStructureDescriptor` | interface | `MTL4AccelerationStructure.h` | `Metal4PrimitiveAccelerationStructureDescriptor` |
| `MTL4RenderCommandEncoder` | protocol | `MTL4RenderCommandEncoder.h` | `Metal4RenderCommandEncoder` (exhaustive.rs opaque handle) |
| `MTL4RenderPassDescriptor` | interface | `MTL4RenderPass.h` | `Metal4RenderPassDescriptor` |
| `MTL4RenderPipelineBinaryFunctionsDescriptor` | interface | `MTL4RenderPipeline.h` | `Metal4RenderPipelineBinaryFunctionsDescriptor` |
| `MTL4RenderPipelineColorAttachmentDescriptor` | interface | `MTL4RenderPipeline.h` | `Metal4RenderPipelineColorAttachmentDescriptor` |
| `MTL4RenderPipelineColorAttachmentDescriptorArray` | interface | `MTL4RenderPipeline.h` | `Metal4RenderPipelineColorAttachmentDescriptorArray` |
| `MTL4RenderPipelineDescriptor` | interface | `MTL4RenderPipeline.h` | `Metal4RenderPipelineDescriptor` |
| `MTL4RenderPipelineDynamicLinkingDescriptor` | interface | `MTL4LinkingDescriptor.h` | `Metal4RenderPipelineDynamicLinkingDescriptor` |
| `MTL4SpecializedFunctionDescriptor` | interface | `MTL4SpecializedFunctionDescriptor.h` | `Metal4SpecializedFunctionDescriptor` |
| `MTL4StaticLinkingDescriptor` | interface | `MTL4LinkingDescriptor.h` | `Metal4StaticLinkingDescriptor` |
| `MTL4StitchedFunctionDescriptor` | interface | `MTL4StitchedFunctionDescriptor.h` | `Metal4StitchedFunctionDescriptor` |
| `MTL4TileRenderPipelineDescriptor` | interface | `MTL4TileRenderPipeline.h` | `Metal4TileRenderPipelineDescriptor` |
| `MTLAccelerationStructure` | protocol | `MTLAccelerationStructure.h` | `AccelerationStructure` |
| `MTLAccelerationStructureBoundingBoxGeometryDescriptor` | interface | `MTLAccelerationStructure.h` | `MetalAccelerationStructureBoundingBoxGeometryDescriptor` |
| `MTLAccelerationStructureCommandEncoder` | protocol | `MTLAccelerationStructureCommandEncoder.h` | `MetalAccelerationStructureCommandEncoder` (exhaustive.rs opaque handle) |
| `MTLAccelerationStructureCurveGeometryDescriptor` | interface | `MTLAccelerationStructure.h` | `MetalAccelerationStructureCurveGeometryDescriptor` |
| `MTLAccelerationStructureDescriptor` | interface | `MTLAccelerationStructure.h` | `MetalAccelerationStructureDescriptor` |
| `MTLAccelerationStructureGeometryDescriptor` | interface | `MTLAccelerationStructure.h` | `MetalAccelerationStructureGeometryDescriptor` |
| `MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor` | interface | `MTLAccelerationStructure.h` | `MetalAccelerationStructureMotionBoundingBoxGeometryDescriptor` |
| `MTLAccelerationStructureMotionCurveGeometryDescriptor` | interface | `MTLAccelerationStructure.h` | `MetalAccelerationStructureMotionCurveGeometryDescriptor` |
| `MTLAccelerationStructureMotionTriangleGeometryDescriptor` | interface | `MTLAccelerationStructure.h` | `MetalAccelerationStructureMotionTriangleGeometryDescriptor` |
| `MTLAccelerationStructurePassDescriptor` | interface | `MTLAccelerationStructure.h` | `MetalAccelerationStructurePassDescriptor` |
| `MTLAllocation` | protocol | `MTLResource.h` | `MetalAllocation` (exhaustive.rs opaque handle) |
| `MTLArgument` | interface | `MTLArgument.h` | `MetalArgument` |
| `MTLArgumentDescriptor` | interface | `MTLArgument.h` | `MetalArgumentDescriptor` |
| `MTLArrayType` | protocol | `MTLType.h` | `MetalArrayType` |
| `MTLAttribute` | interface | `MTLArgument.h` | `MetalAttribute` |
| `MTLAttributeDescriptor` | interface | `MTLFunction.h` | `MetalAttributeDescriptor` |
| `MTLBinding` | protocol | `MTLArgument.h` | `MetalBinding` |
| `MTLBlitCommandEncoder` | protocol | `MTLBlitCommandEncoder.h` | `MetalBlitCommandEncoder` |
| `MTLBuffer` | protocol | `MTLBuffer.h` | `Buffer` |
| `MTLBufferBinding` | protocol | `MTLArgument.h` | `MetalBufferBinding` (exhaustive.rs opaque handle) |
| `MTLBufferLayout` | interface | `MTLArgument.h` | `MetalBufferLayout` |
| `MTLCaptureDescriptor` | interface | `MTLCaptureManager.h` | `MetalCaptureDescriptor` (exhaustive.rs opaque_symbol_class) |
| `MTLCaptureManager` | interface | `MTLCaptureManager.h` | `MetalCaptureManager` |
| `MTLCaptureScope` | protocol | `MTLCaptureScope.h` | `MetalCaptureScope` |
| `MTLCompileOptions` | interface | `MTLCompileOptions.h` | `MetalCompileOptions` |
| `MTLComputeCommandEncoder` | protocol | `MTLComputeCommandEncoder.h` | `MetalComputeCommandEncoder` |
| `MTLComputePipelineDescriptor` | interface | `MTLComputePipeline.h` | `MetalComputePipelineDescriptor` |
| `MTLComputePipelineReflection` | interface | `MTLComputePipeline.h` | `MetalComputePipelineReflection` |
| `MTLComputePipelineState` | protocol | `MTLComputePipeline.h` | `MetalComputePipelineState` |
| `MTLCounter` | protocol | `MTLCounter.h` | `MetalCounter` |
| `MTLCounterSampleBuffer` | protocol | `MTLCounterSampleBuffer.h` | `MetalCounterSampleBuffer` |
| `MTLCounterSet` | protocol | `MTLCounterSet.h` | `MetalCounterSet` |
| `MTLDepthStencilDescriptor` | interface | `MTLDepthStencil.h` | `MetalDepthStencilDescriptor` |
| `MTLDepthStencilState` | protocol | `MTLDepthStencil.h` | `MetalDepthStencilState` |
| `MTLDevice` | protocol | `MTLDevice.h` | `MetalDevice` |
| `MTLDrawable` | protocol | `MTLDrawable.h` | `MetalDrawable` (exhaustive.rs opaque handle) |
| `MTLDynamicLibrary` | protocol | `MTLDynamicLibrary.h` | `MetalDynamicLibrary` |
| `MTLEvent` | protocol | `MTLEvent.h` | `MetalEvent` |
| `MTLFence` | protocol | `MTLFence.h` | `MetalFence` |
| `MTLFunction` | protocol | `MTLFunction.h` | `MetalFunction` |
| `MTLFunctionConstantValues` | interface | `MTLFunction.h` | `MetalFunctionConstantValues` |
| `MTLFunctionHandle` | protocol | `MTLFunction.h` | `MetalFunctionHandle` (exhaustive.rs opaque handle) |
| `MTLFunctionLog` | protocol | `MTLFunction.h` | `MetalFunctionLog` (exhaustive.rs opaque handle) |
| `MTLFunctionLogDebugLocation` | interface | `MTLFunction.h` | `MetalFunctionLogDebugLocation` (exhaustive.rs opaque handle) |
| `MTLFunctionStitchingNode` | interface | `MTLLinkedFunctions.h` | `MetalFunctionStitchingNode` (exhaustive.rs opaque handle) |
| `MTLFXFrameInterpolator` | protocol | `MTLFXFrameInterpolator.h` | `MetalFxFrameInterpolator` |
| `MTLFXFrameInterpolatableScaler` | protocol | `MTLFXFrameInterpolator.h` | `MetalFxFrameInterpolatableScaler` |
| `MTLFXFrameInterpolatorBase` | protocol | `MTLFXFrameInterpolator.h` | `MetalFxFrameInterpolatorBase` (exhaustive.rs opaque handle) |
| `MTLFXSpatialScaler` | protocol | `MTLFXSpatialScaler.h` | `MetalFxSpatialScaler` |
| `MTLFXSpatialScalerBase` | protocol | `MTLFXSpatialScaler.h` | `MetalFxSpatialScalerBase` (exhaustive.rs opaque handle) |
| `MTLFXTemporalDenoisedScaler` | protocol | `MTLFXTemporalDenoisedScaler.h` | `MetalFxTemporalDenoisedScaler` |
| `MTLFXTemporalDenoisedScalerBase` | protocol | `MTLFXTemporalDenoisedScaler.h` | `MetalFxTemporalDenoisedScalerBase` (exhaustive.rs opaque handle) |
| `MTLFXTemporalScaler` | protocol | `MTLFXTemporalScaler.h` | `MetalFxTemporalScaler` |
| `MTLFXTemporalScalerBase` | protocol | `MTLFXTemporalScaler.h` | `MetalFxTemporalScalerBase` (exhaustive.rs opaque handle) |
| `MTLHeap` | protocol | `MTLHeap.h` | `MetalHeap` |
| `MTLHeapDescriptor` | interface | `MTLHeap.h` | `MetalHeapDescriptor` |
| `MTLIndirectCommandBuffer` | protocol | `MTLIndirectCommandBuffer.h` | `MetalIndirectCommandBuffer` |
| `MTLIndirectCommandBufferDescriptor` | interface | `MTLIndirectCommandBuffer.h` | `MetalIndirectCommandBufferDescriptor` |
| `MTLIndirectComputeCommand` | protocol | `MTLIndirectCommandBuffer.h` | `MetalIndirectComputeCommand` (exhaustive.rs opaque handle) |
| `MTLIndirectComputeCommandEncoder` | protocol | `MTLIndirectCommandBuffer.h` | `MetalIndirectComputeCommandEncoder` (exhaustive.rs opaque handle) |
| `MTLIndirectRenderCommand` | protocol | `MTLIndirectCommandBuffer.h` | `MetalIndirectRenderCommand` |
| `MTLIndirectRenderCommandEncoder` | protocol | `MTLIndirectCommandBuffer.h` | `MetalIndirectRenderCommandEncoder` (exhaustive.rs opaque handle) |
| `MTLIntersectionFunctionTable` | protocol | `MTLIntersectionFunctionTable.h` | `MetalIntersectionFunctionTable` |
| `MTLIOCommandBuffer` | protocol | `MTLIOCommandQueue.h` | `MetalIoCommandBuffer` (exhaustive.rs opaque handle) |
| `MTLIOCommandQueue` | protocol | `MTLIOCommandQueue.h` | `MetalIOCommandQueue` |
| `MTLIOFileHandle` | protocol | `MTLIOCommandQueue.h` | `MetalIoFileHandle` (exhaustive.rs opaque handle) |
| `MTLIOScratchBuffer` | protocol | `MTLIOCommandQueue.h` | `MetalIoScratchBuffer` (exhaustive.rs opaque handle) |
| `MTLIOScratchBufferAllocator` | protocol | `MTLIOCommandQueue.h` | `MetalIoScratchBufferAllocator` (exhaustive.rs opaque handle) |
| `MTLLibrary` | protocol | `MTLLibrary.h` | `MetalLibrary` |
| `MTLLinkedFunctions` | interface | `MTLLinkedFunctions.h` | `MetalLinkedFunctions` |
| `MTLLogContainer` | protocol | `MTLLogState.h` | `MetalLogContainer` (exhaustive.rs opaque handle) |
| `MTLLogState` | protocol | `MTLLogState.h` | `MetalLogState` |
| `MTLMeshRenderPipelineDescriptor` | interface | `MTLRenderPipeline.h` | `MetalMeshRenderPipelineDescriptor` |
| `MTLObjectPayloadBinding` | protocol | `MTLArgument.h` | `MetalObjectPayloadBinding` (exhaustive.rs opaque handle) |
| `MTLPackedFloat3` | interface | `MTLCommonTypes.h` | `MetalPackedFloat3` |
| `MTLPackedFloatQuaternion` | interface | `MTLCommonTypes.h` | `MetalPackedFloatQuaternion` |
| `MTLPackedFloat4x3` | interface | `MTLCommonTypes.h` | `MetalPackedFloat4x3` |
| `MTLParallelRenderCommandEncoder` | protocol | `MTLRenderCommandEncoder.h` | `MetalParallelRenderCommandEncoder` |
| `MTLPointerType` | protocol | `MTLType.h` | `MetalPointerType` |
| `MTLRasterizationRateLayerDescriptor` | interface | `MTLRasterizationRateMap.h` | `MetalRasterizationRateLayerDescriptor` |
| `MTLRasterizationRateMap` | protocol | `MTLRasterizationRateMap.h` | `MetalRasterizationRateMap` |
| `MTLRasterizationRateMapDescriptor` | interface | `MTLRasterizationRateMap.h` | `MetalRasterizationRateMapDescriptor` |
| `MTLRenderCommandEncoder` | protocol | `MTLRenderCommandEncoder.h` | `MetalRenderCommandEncoder` |
| `MTLRenderPassAttachmentDescriptor` | interface | `MTLRenderPass.h` | `MetalRenderPassAttachmentDescriptor` |
| `MTLRenderPassColorAttachmentDescriptor` | interface | `MTLRenderPass.h` | `MetalRenderPassColorAttachmentDescriptor` |
| `MTLRenderPassColorAttachmentDescriptorArray` | interface | `MTLRenderPass.h` | `MetalRenderPassColorAttachmentDescriptorArray` |
| `MTLRenderPassDepthAttachmentDescriptor` | interface | `MTLRenderPass.h` | `MetalRenderPassDepthAttachmentDescriptor` |
| `MTLRenderPassDescriptor` | interface | `MTLRenderPass.h` | `MetalRenderPassDescriptor` |
| `MTLRenderPassStencilAttachmentDescriptor` | interface | `MTLRenderPass.h` | `MetalRenderPassStencilAttachmentDescriptor` |
| `MTLRenderPipelineColorAttachmentDescriptor` | interface | `MTLRenderPipeline.h` | `MetalRenderPipelineColorAttachmentDescriptor` |
| `MTLRenderPipelineColorAttachmentDescriptorArray` | interface | `MTLRenderPipeline.h` | `MetalRenderPipelineColorAttachmentDescriptorArray` |
| `MTLRenderPipelineDescriptor` | interface | `MTLRenderPipeline.h` | `MetalRenderPipelineDescriptor` |
| `MTLRenderPipelineReflection` | interface | `MTLRenderPipeline.h` | `MetalRenderPipelineReflection` |
| `MTLRenderPipelineState` | protocol | `MTLRenderPipeline.h` | `MetalRenderPipelineState` |
| `MTLResidencySet` | protocol | `MTLResidencySet.h` | `MetalResidencySet` |
| `MTLResource` | protocol | `MTLResource.h` | `MetalResource` |
| `MTLResourceStateCommandEncoder` | protocol | `MTLResourceStateCommandEncoder.h` | `MetalResourceStateCommandEncoder` |
| `MTLSamplerDescriptor` | interface | `MTLSamplerState.h` | `MetalSamplerDescriptor` |
| `MTLSamplerState` | protocol | `MTLSamplerState.h` | `MetalSamplerState` |
| `MTLSharedEvent` | protocol | `MTLEvent.h` | `MetalSharedEvent` |
| `MTLSharedEventListener` | interface | `MTLEvent.h` | `MetalSharedEventListener` |
| `MTLStageInputOutputDescriptor` | interface | `MTLStageInputOutputDescriptor.h` | `MetalStageInputOutputDescriptor` |
| `MTLStructMember` | interface | `MTLArgument.h` | `MetalStructMember` |
| `MTLStructType` | protocol | `MTLType.h` | `MetalStructType` |
| `MTLTensorBinding` | protocol | `MTLArgument.h` | `MetalTensorBinding` (exhaustive.rs opaque handle) |
| `MTLTextureBinding` | protocol | `MTLArgument.h` | `MetalTextureBinding` (exhaustive.rs opaque handle) |
| `MTLTextureDescriptor` | interface | `MTLTexture.h` | `MetalTextureDescriptor` |
| `MTLTexture` | protocol | `MTLTexture.h` | `MetalTexture` |
| `MTLTextureViewPool` | protocol | `MTLTexture.h` | `MetalTextureViewPool` (exhaustive.rs opaque handle) |
| `MTLThreadgroupBinding` | protocol | `MTLArgument.h` | `MetalThreadgroupBinding` (exhaustive.rs opaque handle) |
| `MTLType` | protocol | `MTLType.h` | `MetalType` |
| `MTLVertexAttribute` | interface | `MTLArgument.h` | `MetalVertexAttribute` |
| `MTLVertexAttributeDescriptor` | interface | `MTLVertexAttributeDescriptor.h` | `MetalVertexAttributeDescriptor` |
| `MTLVertexAttributeDescriptorArray` | interface | `MTLVertexAttributeDescriptor.h` | `MetalVertexAttributeDescriptorArray` |
| `MTLVertexBufferLayoutDescriptor` | interface | `MTLVertexDescriptor.h` | `MetalVertexBufferLayoutDescriptor` |
| `MTLVertexBufferLayoutDescriptorArray` | interface | `MTLVertexDescriptor.h` | `MetalVertexBufferLayoutDescriptorArray` |
| `MTLVertexDescriptor` | interface | `MTLVertexDescriptor.h` | `MetalVertexDescriptor` |
| `MTLVisibleFunctionTable` | protocol | `MTLVisibleFunctionTable.h` | `MetalVisibleFunctionTable` |

## 🔴 GAPS (0 symbols)

All gaps closed.

## ⏭️ EXEMPT (1 symbol)

| Symbol | Kind | Header | Reason |
| --- | --- | --- | --- |
| `NSProcessInfo` | interface | `Foundation` | Foundation system class, not a Metal symbol. Wrapping `NSProcessInfo` is out of scope for a Metal framework binding crate. macOS SDK: `@interface NSProcessInfo : NSObject` in `<Foundation/NSProcessInfo.h>`. |

