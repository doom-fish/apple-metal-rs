# COVERAGE

Audit target: `MacOSX26.2.sdk/System/Library/Frameworks/Metal.framework/Headers`
plus the wrapped `MetalFX.framework` headers. `COVERAGE_AUDIT.md` was generated
against that SDK and has not been regenerated for the installed 26.5 SDK.

## What the numbers measure

`COVERAGE_AUDIT.md` lists **431** audited top-level macOS `Metal.framework` +
`MetalFX.framework` symbols and marks all 431 as VERIFIED. VERIFIED only means
that a Rust item with the symbol's name exists. It does not mean the symbol's
properties or methods are wrapped:

- 139 Objective-C classes (the `opaque_symbol_class!` wrappers in
  `src/exhaustive.rs`) only have `new()`, which instantiates the class through
  `NSClassFromString`, plus `label()`. None of their properties or methods are
  bound.
- 60 protocols are opaque handles with `as_ptr`, `from_raw` and `label` only.
  `MetalRasterizationRateLayerDescriptor` is a handle with one constructor,
  `with_sample_count`.
- 95 enums and option sets are raw integer wrappers, and 32 string constants
  are resolved at run time.
- **Metal 4 is not wrapped.** The `MTL4*` families (41 descriptor classes, 20
  protocols including `MTL4CommandQueue`, `MTL4CommandBuffer`,
  `MTL4CommandAllocator`, `MTL4ArgumentTable` and `MTL4Compiler`, and 14 enums)
  and `MTLTensor`, `MTLTensorDescriptor`, `MTLTensorExtents` and
  `MTLTensorBinding` are opaque handles without constructors or methods (the
  descriptor classes only have `new()`). `MetalTensor` exists so sibling crates
  can pass tensor handles through FFI, and `MetalDevice::new_tensor` creates
  one from a checked `TensorDescriptor`; the tensor itself still has no
  methods. The Metal 4 method surface is out of scope for this release.

The working surface is the hand-written API: devices, buffers (including
buffers created from bytes), textures (1D, 2D, 3D, array, cube and multisample
descriptors, buffer-backed views, checked CPU transfers, IOSurface interop),
command queues and buffers with scheduled/completed handlers and encoding by
foreign code under the lifecycle lock, blit, compute and render encoders
(render passes with depth and stencil attachments), compute/render/tile pipelines, depth/stencil and sampler
state, argument encoders, heaps, events and shared-event notifications, fences,
dynamic libraries, binary archives, indirect command buffers,
acceleration-structure handles, function tables, counter sample buffers, log
state and residency sets (macOS 15+), capture scopes, the MetalFX spatial and
temporal scalers (macOS 13+) and the device observer.

`MetalPerformanceShaders.framework` is out of scope for this crate.

`pixel_format::bytes_per_pixel` was checked against the installed
`MacOSX26.5.sdk` `MTLPixelFormat.h`, and `tests/pixel_format_table.rs` repeats
that check against whatever SDK is installed.

## Coverage summary

`apple-metal` `0.10.0` keeps the smoke-tested, fully exercised core runtime
workflows established from `0.6.0` through `0.6.3` — device discovery, buffers, textures,
command queues/buffers, explicit blit/compute/render encoders, public
pipeline descriptors, depth/stencil state, sampler state, argument encoders,
heaps, events, dynamic libraries, binary archives, indirect command buffers,
acceleration-structure handles, capture scopes, residency sets, and the
spatial / temporal scaler path.

The current package retains the focused integration coverage for the split
bridge areas and the completed *top-level symbol* audit from `0.6.2`,
including the descriptor, reflection, render-pass, resource-state,
rasterization-rate, function-stitching, tensor, IO, MetalFX base / denoised /
frame-interpolator, and `MTL4*` / `MTL4FX*` families as named Rust items
(mostly without methods, see above):

- opaque handle wrappers for protocol/object families;
- constructible `Type::new()` wrappers for descriptor / Objective-C class
  families via the Swift bridge's Objective-C runtime path;
- raw-value wrapper types for enums, options, and newer Metal 4 state enums;
- Rust value types for audited C structs / typedefs such as `MetalOrigin`,
  `MetalRegion`, `MetalCoordinate2D`, `MetalResourceId`,
  `MetalPackedFloat3`, `MetalPackedFloatQuaternion`, `MetalPackedFloat4x3`,
  and `MetalGpuAddress`;
- bridge-backed helpers for exported Metal string constants, device
  enumeration / observation, and IO compression contexts.

The result is a Rust item for every audited top-level symbol, without dropping
down to raw Objective-C messaging from Rust; see "What the numbers measure"
above for how little of that surface is functional beyond the core runtime.

## Validation hooks

The wrapped surface is validated by:

- examples `01_get_device` through `07_advanced_objects`
- `tests/public_api_smoke.rs`
- `tests/exhaustive_symbols.rs`
- `tests/depth_stencil_bridge.rs`
- `tests/sampler_bridge.rs`
- `tests/argument_buffer_bridge.rs`
- `tests/heap_bridge.rs`
- `tests/event_bridge.rs`
- `tests/fence_bridge.rs`
- `tests/pixel_format_table.rs`
- `tests/texture_view_bridge.rs`
- `tests/texture_descriptor.rs`
- `tests/command_handlers.rs`
- `tests/device_observer.rs`
- `tests/range_validation.rs`
- `tests/os_availability.rs`
- `tests/safety_contracts.rs`
- `tests/foreign_encoding.rs`
- `tests/dispatch_validation.rs`
- `tests/tensor.rs`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
- `for ex in examples/*.rs; do cargo run --example "$(basename "$ex" .rs)"; done`
