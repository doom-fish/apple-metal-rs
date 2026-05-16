# COVERAGE

Audit target: `MacOSX26.2.sdk/System/Library/Frameworks/Metal.framework/Headers`
plus the wrapped `MetalFX.framework` scaler headers from the active Xcode
toolchain (`xcrun --sdk macosx --show-sdk-path`).

Scope notes:

- This document covers **Metal.framework** plus the `MetalFX.framework`
  spatial/temporal scaler subset wrapped by `apple-metal`.
- `MetalPerformanceShaders.framework`, `MTL4*`, `MTL4FX*`, and the remaining
  `MetalFX` families are intentionally out of scope for this release.
- Status is tracked per logical API row so implemented subsets of larger headers
  can be called out without pretending that an entire header family is complete.

Legend:

- ✅ implemented
- 🟡 partial
- ⏭️ skipped / deferred

## Implemented rows

| API row | Status | Rust surface | Notes |
| --- | --- | --- | --- |
| System device discovery and core capability queries (`MTLCreateSystemDefaultDevice`, `MTLDevice.name`, `registryID`, unified-memory / working-set / family support, dynamic-library / raytracing / counter-sampling feature gates, counter-set names) | ✅ | `MetalDevice::system_default`, `name`, `registry_id`, `has_unified_memory`, `recommended_max_working_set_size`, `supports_family`, `supports_dynamic_libraries`, `supports_render_dynamic_libraries`, `supports_raytracing`, `supports_counter_sampling`, `counter_set_names` | Backed by the Swift bridge in `Device.swift` + `Advanced.swift`. |
| Core resource / queue / pipeline construction (`makeBuffer`, `makeTexture`, `makeCommandQueue`, `makeLibrary(source:)`, `makeComputePipelineState`, `makeRenderPipelineState`) | ✅ | `MetalDevice::new_buffer`, `new_texture`, `new_command_queue`, `new_library_with_source`, `new_compute_pipeline_state`, `new_render_pipeline_state` | Covers the creation path used by all examples/tests. |
| Public pipeline-descriptor surface (`MTLComputePipelineDescriptor`, `MTLRenderPipelineDescriptor`, `MTLRenderPipelineColorAttachmentDescriptor`, `MTLTileRenderPipelineDescriptor`) | ✅ | `ComputePipelineDescriptor`, `RenderPipelineColorAttachmentDescriptor`, `RenderPipelineDescriptor`, `TileRenderPipelineColorAttachmentDescriptor`, `TileRenderPipelineDescriptor`, and `MetalDevice::new_*_with_descriptor` | Exposes safe descriptor-driven compute/render/tile pipeline creation without dropping down to raw Objective-C or Swift. |
| Depth/stencil state (`MTLCompareFunction`, `MTLStencilOperation`, `MTLStencilDescriptor`, `MTLDepthStencilDescriptor`, `MTLDepthStencilState`) | ✅ | `compare_function`, `stencil_operation`, `StencilDescriptor`, `DepthStencilDescriptor`, `DepthStencilState`, `MetalDevice::new_depth_stencil_state`, `RenderCommandEncoder::set_depth_stencil_state` | Public safe descriptors/state wrappers are smoke-tested end-to-end. |
| Sampler state (`MTLSampler*`, `MTLSamplerDescriptor`, `MTLSamplerState`) | ✅ | `sampler_*` modules, `SamplerDescriptor`, `SamplerState`, `MetalDevice::new_sampler_state`, `ComputeCommandEncoder::set_sampler_state`, `RenderCommandEncoder::set_fragment_sampler_state`, `ArgumentEncoder::set_sampler_state` | Covers creation plus compute/render/argument-encoder bindings. |
| Argument-buffer descriptors (`MTLArgumentBuffersTier`, `MTLBindingAccess`, `MTLTextureType`, `MTLArgumentDescriptor`) | ✅ | `argument_buffers_tier`, `binding_access`, `texture_type`, `ArgumentDescriptor`, `MetalDevice::argument_buffers_support`, `new_argument_encoder_with_descriptors` | Adds public descriptor-driven argument-encoder construction alongside the function-derived encoder path. |
| Advanced object construction (`makeCommandQueue(maxCount/logState)`, `makeHeap`, `makeFence`, `makeSharedEvent`, `makeDynamicLibrary`, `makeBinaryArchive`, `makeIndirectCommandBuffer`, `makeAccelerationStructure`, `makeCounterSampleBuffer`, `newLogState`, `newResidencySet`) | ✅ | `MetalDevice::new_command_queue_with_max_command_buffer_count`, `new_command_queue_with_log_state`, `new_heap`, `new_fence`, `new_shared_event`, `new_dynamic_library_with_source`, `load_dynamic_library`, `new_binary_archive`, `new_indirect_command_buffer`, `new_acceleration_structure_with_size`, `new_counter_sample_buffer`, `new_log_state`, `new_residency_set` | Availability-gated in Swift where the OS requires it. |
| Library/function lookup (`MTLLibrary.makeFunction`) | ✅ | `MetalLibrary::new_function` | Source-based library compilation is covered end-to-end in examples/tests. |
| Command queue + command buffer creation (`makeCommandBuffer`, `makeCommandBufferWithUnretainedReferences`) | ✅ | `CommandQueue::new_command_buffer`, `new_command_buffer_with_unretained_references` | Both retained and unretained command-buffer creation paths are covered. |
| Command-buffer lifecycle/state (`enqueue`, `commit`, `waitUntilScheduled`, `waitUntilCompleted`, `status`, `error`) | ✅ | `CommandBuffer::enqueue`, `commit`, `wait_until_scheduled`, `wait_until_completed`, `status`, `error` | `command_buffer_status::*` constants are exposed for state checks. |
| Command-buffer event sync (`encodeWaitForEvent`, `encodeSignalEvent`) | ✅ | `CommandBuffer::encode_wait_for_event`, `encode_signal_event` | Shared-event signaling is smoke-tested. |
| Blit encoder basics (`copyFromBuffer`, `fillBuffer`, fence wait/update, counter sampling, `endEncoding`) | ✅ | `BlitCommandEncoder::{copy_buffer, fill_buffer, wait_for_fence, update_fence, sample_counters, end_encoding}` | Enough to cover common copy/fill/counter workflows. |
| Compute encoder basics (pipeline/buffer/texture/sampler binding, visible/intersection tables, acceleration-structure binding, threadgroup dispatch, fence wait/update, `endEncoding`) | ✅ | `ComputeCommandEncoder::{set_compute_pipeline_state, set_buffer, set_texture, set_sampler_state, set_visible_function_table, set_intersection_function_table, set_acceleration_structure, dispatch_threadgroups, dispatch_threads, wait_for_fence, update_fence, end_encoding}` | The explicit encoder path is exercised in examples and `public_api_smoke`. |
| Render pipeline creation (`makeRenderPipelineState`) | ✅ | `MetalDevice::new_render_pipeline_state`, `RenderPipelineState::label` | Covers a simple single-color render pipeline. |
| Render encoder basics (single color attachment, pipeline bind, vertex-buffer bind, fragment sampler bind, depth/stencil bind, `drawPrimitives`, fence wait/update, `endEncoding`) | ✅ | `CommandBuffer::new_render_command_encoder`, `RenderCommandEncoder::{set_render_pipeline_state, set_depth_stencil_state, set_fragment_sampler_state, set_vertex_buffer, draw_primitives, wait_for_fence, update_fence, end_encoding}` | Enough for headless render-to-texture smoke coverage with public sampler/depth state. |
| Buffer basics (`length`, `contents`, `didModifyRange`) | ✅ | `MetalBuffer::{length, contents, write_bytes, did_modify_range}` | Shared-memory upload / readback is validated in tests/examples. |
| Texture basics (`width`, `height`, `depth`, `mipmapLevelCount`, `arrayLength`, `pixelFormat`, `usage`, `storageMode`) | ✅ | `MetalTexture::{width, height, depth, mipmap_level_count, array_length, pixel_format, usage, storage_mode}` | Basic metadata is exposed for allocated and IOSurface-backed textures. |
| 2D texture I/O and view creation (`replaceRegion`, `getBytes`, `newTextureView`) | ✅ | `MetalTexture::{replace_region_2d, read_bytes_2d, new_view}` | The crate can populate, read back, and create same-format views. |
| Buffer-backed 2D textures (`MTLBuffer.makeTexture`) | ✅ | `MetalBuffer::new_texture_view_2d` | Used in examples/tests for linear-data-backed textures. |
| Heap basics (`size`, `usedSize`, `currentAllocatedSize`, `maxAvailableSize`, basic resource allocation, purgeable state) | ✅ | `Heap::{size, used_size, current_allocated_size, max_available_size, new_buffer, new_texture, new_acceleration_structure_with_size, set_purgeable_state}` | Heap allocation + basic bookkeeping is exposed. |
| Shared events (`signaledValue`, `notify/wait` equivalent via polling helper + command-buffer encode paths) | ✅ | `Event::{signaled_value, set_signaled_value, wait_until_signaled_value}` | Synchronous wait helper is implemented in the Swift bridge. |
| Dynamic libraries (`makeDynamicLibrary`, serialize/load, install name) | ✅ | `DynamicLibrary::{install_name, serialize_to_file}` plus `MetalDevice::{new_dynamic_library_with_source, load_dynamic_library}` | Availability-gated to supported macOS releases. |
| Binary archives (`addComputePipelineFunctions`, `addRenderPipelineFunctions`, serialize/load) | ✅ | `BinaryArchive::{add_compute_function, add_render_functions, serialize_to_file}` plus `MetalDevice::new_binary_archive` | Archive round-trip is covered in example 06. |
| Argument encoders (`makeArgumentEncoder`, descriptor-driven encoders, encoded length / alignment, set buffer / texture / sampler / argument buffer) | ✅ | `MetalFunction::new_argument_encoder`, `MetalDevice::new_argument_encoder_with_descriptors`, `ArgumentEncoder::{encoded_length, alignment, set_argument_buffer, set_buffer, set_texture, set_sampler_state}` | Covers both function-derived and descriptor-driven argument-buffer setup. |
| MetalFX scaler subset (`MTLFXSpatialScaler*`, `MTLFXTemporalScaler*`, `MTLFXFrameInterpolatableScaler`) | ✅ | `SpatialScalerDescriptor`, `SpatialScaler`, `TemporalScalerDescriptor`, `TemporalScaler`, `FrameInterpolatableScaler`, and their support/configuration helpers | Covers device support queries, texture-usage queries, per-frame configuration, and encode entry points for the spatial/temporal scaler subset. |
| Indirect command-buffer allocation/reset/size | ✅ | `IndirectCommandBuffer::{size, reset_range}` plus `MetalDevice::new_indirect_command_buffer` | Allocation surface is wrapped even though command recording is deferred. |
| Counter sample buffers (`sampleCount`, resolve range) | ✅ | `CounterSampleBuffer::{sample_count, resolve_range}` | Creation, sampling, and resolve paths are available when the device supports them. |
| Log state + queue creation (`MTLLogState`, queue descriptor log-state path) | ✅ | `MetalDevice::new_log_state`, `new_command_queue_with_log_state` | Safe creation path only; log contents remain SDK-managed. |
| Residency sets (`add/remove/contains/commit/request/end`) | ✅ | `ResidencySet::{add_buffer, add_texture, add_heap, remove_buffer, remove_texture, remove_heap, remove_all_allocations, contains_buffer, contains_texture, allocation_count, commit, request_residency, end_residency}` plus `CommandQueue::{add_residency_set, remove_residency_set}` | Enough for residency bookkeeping and queue association. |
| Capture scopes (`MTLCaptureManager.shared`, destination support, capture scopes for device/queue, `begin`/`end`) | ✅ | `CaptureManager::{shared, supports_destination, is_capturing, new_capture_scope_with_device, new_capture_scope_with_command_queue}` and `CaptureScope::{begin, end}` | Headless-safe scope control is exposed. |
| IOSurface zero-copy texture interop (crate extension, not a Metal header family) | ✅ | `IOSurfaceMetalExt::create_metal_texture` | Preserved from earlier releases under the default `iosurface` feature. |

## Partial rows

| API row | Status | Rust surface | What's still missing |
| --- | --- | --- | --- |
| `TextureDescriptor` coverage | 🟡 | `TextureDescriptor::new_2d`, `render_target_2d` plus public fields | No dedicated helpers yet for 1D/3D/cube/array/sparse descriptors, texture-type-specific validation, or descriptor convenience builders beyond the common 2D cases. |
| Ray-tracing support (`MTLAccelerationStructure*`, `MTLVisibleFunctionTable`, `MTLIntersectionFunctionTable`) | 🟡 | `new_acceleration_structure_with_size`, compute binding helpers, `new_visible_function_table`, `new_intersection_function_table`, `set_opaque_triangle_intersection_function` | The crate exposes handle allocation/binding only; build/update/copy encoders, geometry descriptors, instance descriptors, compaction, and acceleration-structure command encoders are still deferred. |
| Pipeline-descriptor coverage | 🟡 | `ComputePipelineDescriptor`, `RenderPipelineDescriptor`, `TileRenderPipelineDescriptor`, and their color-attachment helpers | Reflection objects, vertex descriptors, pipeline options, async compilation APIs, and descriptor-array wrappers remain deferred. |
| Render encoding coverage | 🟡 | Single color-attachment render pass + pipeline/vertex-buffer bind + fragment sampler + depth/stencil bind + `draw_primitives` | No depth/stencil attachments on the headless helper, viewport/scissor, fragment buffers/textures, indexed/instanced draws, visibility result buffers, render-pass descriptor graph, or parallel render encoders yet. |
| MetalFX coverage | 🟡 | Spatial/temporal scaler descriptors + concrete scaler handles + support/configuration helpers | Base-protocol property getters, frame interpolation, temporal-denoised scalers, Metal4FX, and the broader MetalFX surface remain deferred. |
| Heap coverage | 🟡 | Basic heap creation/allocation + purgeable state | No dedicated heap descriptor wrapper, sparse heaps, aliasing helpers, or residency preference controls beyond the SDK defaults. |
| Capture coverage | 🟡 | Capture-manager singleton, destination support, capture scopes | File/trace-output configuration and explicit `startCapture` / `stopCapture` session management are not wrapped yet. |

## Deferred / skipped rows

| API row | Status | Reason |
| --- | --- | --- |
| Remaining `MTLArgument` reflection, `MTLBinding`, `MTLArrayType`, `MTLType`, `MTLVertexDescriptor`, and `MTLStageInputOutputDescriptor` families | ⏭️ | Descriptor-heavy vertex/input/reflection APIs beyond the public argument-descriptor subset are not wrapped yet. |
| `MTLRenderPass*`, `MTLBlitPass*`, `MTLComputePass*`, `MTLParallelRenderCommandEncoder`, `MTLDrawable` presentation APIs | ⏭️ | `0.6.1` still intentionally keeps the safe render path to a headless single-color render target helper. |
| `MTLIndirectCommandEncoder` command-recording APIs | ⏭️ | The crate allocates/resets indirect command buffers but does not yet expose per-command recording. |
| `MTLResourceStateCommandEncoder`, `MTLResourceStatePass*`, sparse/resource-view pool APIs | ⏭️ | Resource-state and sparse-resource management are deferred. |
| `MTLFunctionConstantValues`, `MTLFunctionDescriptor`, `MTLLinkedFunctions`, `MTLFunctionStitching`, `MTLFunctionHandle`, `MTLFunctionLog` | ⏭️ | Advanced function specialization/linking/logging families are not wrapped yet. |
| `MTLFXFrameInterpolator*`, `MTLFXTemporalDenoisedScaler*`, MetalFX base protocols, and `MTL4FX*` | ⏭️ | `0.6.1` only wraps the spatial/temporal scaler descriptor + concrete scaler subset of `MetalFX.framework`. |
| `MTLIOCommandQueue`, `MTLIOCommandBuffer`, `MTLIOCompressor` | ⏭️ | GPU file-I/O APIs are deferred. |
| `MTLRasterizationRate*`, `MTLDeviceCertification`, `MTLAllocation`, `MTLTensor`, `MTLResourceViewPool`, `MTLTextureViewPool`, other niche utility headers | ⏭️ | These newer or niche utility families are not part of the `0.6.1` safe surface. |
| `MTL4*` headers (`MTL4CommandQueue`, `MTL4CommandBuffer`, `MTL4RenderPipeline`, `MTL4ComputePipeline`, `MTL4MeshRenderPipeline`, `MTL4MachineLearning*`, etc.) | ⏭️ | Metal 4 is a separate, very large API family and is intentionally deferred from `0.6.1`. |

## Validation hooks

The implemented rows above are exercised by:

- examples `01_get_device` through `07_advanced_objects`
- `tests/public_api_smoke.rs`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
- `for ex in examples/*.rs; do cargo run --example "$(basename "$ex" .rs)"; done`
