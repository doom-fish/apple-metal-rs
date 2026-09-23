# Changelog

All notable changes to `apple-metal` are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.10.0] - Unreleased

### Security

- CPU texture transfers sized `BGRA10_XR` and `BGRA10_XR_sRGB` as 4 bytes per
  pixel instead of 8 (MTLPixelFormat.h lists them as 64-bit formats), so a
  `read_bytes_2d` that met its documented contract wrote past the caller's
  slice and `replace_region_2d` read past it. Both the Rust and the Swift
  validator now take their sizes from one table, `pixel_format::bytes_per_pixel`.
- Dropping a `MetalDeviceObserver` never called `MTLRemoveDeviceObserver`, so a
  device hot-plug afterwards called into freed user data. The observer is now
  removed on drop, and its closure lives in a reference-counted context that
  Metal's handler block keeps alive until Metal releases it.
- `MetalBuffer::new_texture_view_2d` passed its offset, row stride and height
  to Metal unchecked: views larger than their buffer were created (later
  transfers ran out of bounds) and a misaligned offset aborted the process.
- apple-metal 0.8 and 0.9 linked a static `AppleMetalBridge` with identical
  `am_*` symbols but different ABIs and no `links` key, so a graph containing
  both failed with duplicate symbols or bound one version's calls to the
  other's code. See Changed for the new bridge names.

### Fixed

- The 18 macOS 15 bridge exports (residency sets, shader log state, command
  queues with a log state) were annotated `@available(macOS 15.0, *)`, which
  makes their availability guard statically true while Package.swift targets
  macOS 11. The guards are now real run-time checks, and `new_log_state` and
  `new_residency_set` return "... requires macOS 15.0 or later" on older
  systems.
- Process aborts reachable from safe code:
  - trapping `UInt(x)`/`Int(x)` conversions of caller integers in the Swift
    bridge (texture descriptors, heap and counter storage modes, log level,
    capture destination, counter sampling point, view, pipeline, archive and
    MetalFX pixel formats, load/store actions, primitive type);
  - `CounterSampleBuffer::resolve_range` and `IndirectCommandBuffer::reset_range`
    building `location..<(location + length)` from unchecked ranges;
  - `MetalDevice::new_texture` and `Heap::new_texture` handing Metal unknown
    pixel formats, unknown or atomic usage bits, extents beyond 16384 (2048 for
    3D textures and array layers), unsupported sample counts, or formats the
    device lacks (for example `Depth24Unorm_Stencil8` on Apple GPUs). Metal's
    descriptor validation aborts on these even without the debug layer;
  - `MetalTexture::new_view` with a format of a different size;
  - heap buffers and textures whose storage or CPU cache mode differs from the
    heap, and a force-unwrapped render pass attachment.
- `MetalRasterizationRateLayerDescriptor::new` called the `init` the SDK marks
  `API_UNAVAILABLE`.
- `Heap::new_buffer` lacked the `isize::MAX` guard of `MetalDevice::new_buffer`,
  and invalid heap and counter storage modes no longer fall back to shared.
- COVERAGE.md said 430 audited symbols where COVERAGE_AUDIT.md lists 431. Both
  now say that VERIFIED only means a named Rust item exists, and that the
  Metal 4 (`MTL4*`) and `MTLTensor` families are opaque handles without
  constructors or methods.
- The README states the macOS 11 minimum and which features need macOS 13 or 15.

### Changed

- **BREAKING:** the package declares `links = "apple_metal_bridge"`, so a
  build can contain only one apple-metal release from 0.10 on. The Swift
  module and static library are now `AppleMetalSwiftBridge` and every bridge
  export, including the raw `apple_metal::ffi` functions, is named `ametal_*`
  (was `am_*`), so this release shares no symbol with 0.8 or 0.9.
- **BREAKING:** `copy_all_devices_with_observer` is safe and takes a
  `FnMut(MetalDevice, &str) + Send + 'static` closure instead of an
  `extern "C"` callback and user-data pointer. The closure receives an owned
  device and the notification name, and panics are contained.
  `MetalDeviceObserver::remove` is idempotent; the type no longer has
  `from_raw` or `label`.
- **BREAKING:** `MetalBuffer::new_texture_view_2d` returns
  `Result<MetalTexture, TextureViewError>`, and the view takes the buffer's
  storage, CPU cache and hazard tracking modes instead of always being shared.
- **BREAKING:** `TextureDescriptor` has new public fields `texture_type`,
  `depth`, `array_length` and `sample_count`, so struct literals need them
  (for example `..TextureDescriptor::new_2d(width, height, format)`). It now
  derives `PartialEq`, `Eq` and `Hash`.
- **BREAKING:** `IndirectCommandBuffer::reset_range` returns
  `Result<(), CommandBufferError>` and checks the range against the command
  count.
- **BREAKING:** `CommandBufferError` has a `NotExecuted { status }` variant.
- **BREAKING:** `MetalRasterizationRateLayerDescriptor::new` is replaced by
  `with_sample_count(horizontal, vertical)`.
- `CounterSampleBuffer::resolve_range` returns `None` for empty, reversed or
  out-of-range ranges.
- `MetalTexture::new_view` only creates views that Metal's validation accepts:
  the texture's own format or its sRGB/linear twin, or, when the texture has
  `texture_usage::PIXEL_FORMAT_VIEW`, a colour format of the same size or the
  stencil view of a depth/stencil format.
- Requires `apple-cf >=0.11, <0.12` and `doom-fish-utils >=0.4.1, <0.5`, and
  `rust-version` is 1.82 (was 1.76).

### Added

- `pixel_format::bytes_per_pixel` (also re-exported as
  `apple_metal::bytes_per_pixel`): bytes per pixel for every uncompressed
  `MTLPixelFormat`, including depth-only and stencil-only formats, and `None`
  for block-compressed, 4:2:2, combined depth/stencil, invalid, unspecialized
  and unknown values. It is checked against the macOS 26.5 SDK header, and
  `tests/pixel_format_table.rs` re-checks it against the installed SDK.
- `pixel_format` constants for all 140 SDK formats, and
  `texture_usage::PIXEL_FORMAT_VIEW`.
- `CommandBuffer::add_scheduled_handler` and `add_completed_handler`, taking
  `FnOnce(Result<(), CommandBufferError>) + Send + 'static`. They must be added
  before commit. Handlers still run after the Rust wrapper is dropped; when an
  uncommitted buffer is released they receive `NotExecuted`.
- `Event::notify_listener(listener, value, handler)` for `MTLSharedEvent`
  notifications.
- `MetalDevice::new_buffer_with_bytes` for shared or managed buffers.
- `TextureDescriptor::with_texture_type`, `with_depth`, `with_array_length` and
  `with_sample_count`, for 1D, 3D, array, cube, cube-array and multisample
  textures on devices and heaps.
- `TextureViewError`.

### Removed

- `MetalDeviceObserverCallback` and the raw-callback form of
  `copy_all_devices_with_observer`.
- `MetalRasterizationRateLayerDescriptor::new`.
- The `am_device_new_texture_2d` and `am_heap_new_texture_2d` raw exports;
  use `ametal_device_new_texture` and `ametal_heap_new_texture`.

## [0.9.0] - 2026-09-07

### Changed (breaking)

- Replaced raw `MetalBuffer::contents` access with unsafe scoped read/write
  mappings shared across buffer clones. `write_bytes` and `read_bytes` now
  return typed errors, private buffers reject CPU access, managed writes are
  reported on guard drop, and private allocations can create shared staging
  buffers for blit transfers.
- Made unretained-reference command-buffer creation unsafe. Regular command
  buffers now share lifecycle state across clones, reject repeated commit,
  active-encoder commit, and post-completion encoding, while encoder drop ends
  encoding exactly once. Illegal wait-after-update fence ordering is rejected.
  Command and encoder mutations now return `CommandBufferError`.
- Argument encoders now validate active destination binding, alignment,
  checked encoded range, storage mode, resource offsets, and descriptor
  index/type. Destination binding is an unsafe scoped guard that holds the
  buffer's CPU mapping lock; function-derived layouts require explicit unsafe
  setters. Empty or unsupported descriptor configurations are rejected before
  the native call, and binding indexes are limited only by native integer
  representation.
  Resources bound indirectly through argument buffers are retained by the
  native argument buffer for each encoded offset and binding and automatically
  declared to command encoders before dispatch or draw, with render-stage
  hazard tracking. Self-referential and nested buffer/texture retention cycles
  are rejected.
- Texture upload/readback are now unsafe, return `TextureTransferError`, and
  validate native integer conversion, mip/slice/region bounds, supported pixel
  layout, row stride, byte length, and CPU-compatible storage before entering Metal.
  Their contract excludes overlapping GPU and CPU/native aliases, including
  mappings of buffer-backed texture storage.
- `IOSurfaceMetalExt::create_metal_texture` now returns
  `Result<_, IOSurfaceMetalError>`, uses selected-plane dimensions and row
  layout, handles odd bi-planar geometry, validates format compatibility, and
  rejects packed `l10r` surfaces instead of guessing a storage mapping.
- Raised in-family requirements to `apple-cf >=0.10, <0.11` and
  `doom-fish-utils >=0.4, <0.5`.

## [0.8.8] - 2026-06-06

- Removed the unsound `Sync` impl on `CommandBuffer` and an empty module map, and
  documented the `MetalBuffer` data-race hazard and `MetalTexture::from_raw`
  ownership.

## [0.8.7] - 2026-05-20

- Migrated local `take_string` body to call `doom_fish_utils::ffi_string::take_owned_cstring_c`. Centralises the duplicated FFI take-string pattern fleet-wide. No public API change.

## [0.8.6] - 2026-05-20

- Added pure-CPU doctests across descriptor and packed-vector types (`MetalCoordinate2D`, `MetalGpuAddress`, `MetalOrigin`, `MetalRegion`, `MetalResourceId`, `MetalSize`, and the `MetalPackedFloat*` family) so the public API is discoverable without a GPU.

## [0.8.5] - 2026-05-19

- Added an owned `MetalTensor` wrapper for the `MTLTensor` protocol so sibling crates can pass tensor handles across FFI without falling back to raw pointers.

## [0.8.4] - 2026-05-18

- Add one-line docs across the public safe and FFI surfaces, raising public-item rustdoc coverage to 100.0%.

## [0.8.3] - 2026-05-18

- Widen apple-cf version bound to `<0.10` so 0.9.x resolves.

## [0.8.2] - 2026-05-18

- Widen apple-cf version bound to `<0.9` so the 0.8.0 nested-CGRect dep resolves. No source changes.

## 0.8.1 — Quality pass: unsafe/Send+Sync hygiene

### Unsafe correctness

- **`opaque_handle!` macro** (advanced.rs): added `unsafe impl Send` and
  `unsafe impl Sync` for all 15 opaque-handle types it generates (`Heap`,
  `Event`, `Fence`, `DynamicLibrary`, `BinaryArchive`, `ArgumentEncoder`,
  `IndirectCommandBuffer`, `AccelerationStructure`,
  `IntersectionFunctionTable`, `VisibleFunctionTable`,
  `CounterSampleBuffer`, `LogState`, `ResidencySet`, `CaptureManager`,
  `CaptureScope`).  These types wrap Metal ObjC protocol objects whose
  reference counting is atomic and whose API is documented as thread-safe;
  the missing impls were an inconsistency relative to the equivalent types
  in `lib.rs` and `exhaustive.rs`.

- **`unsafe impl Send/Sync` in lib.rs and render.rs**: added `// SAFETY:`
  justification comments to all eight declarations explaining the Metal
  thread-safety guarantee.

- **`opaque_symbol_handle!` macro** (exhaustive.rs): added `// SAFETY:`
  comment to its `unsafe impl Send/Sync` block.

- **`take_device_array`** (exhaustive.rs): added `/// # Safety` doc
  explaining pointer, length, and allocator preconditions.

- **`copy_all_devices_with_observer`** (exhaustive.rs): replaced
  `#[allow(clippy::missing_safety_doc)]` with a proper `/// # Safety`
  section documenting callback and user-data lifetime requirements.

- **`opaque_symbol_handle!::from_raw`** (exhaustive.rs): replaced
  `#[allow(clippy::missing_safety_doc)]` with a `/// # Safety` doc
  describing the +1-retain ownership transfer contract.

- **`MetalIoCompressionContext::from_raw`** (exhaustive.rs): same — real
  safety doc replacing the `#[allow]` attribute.

## 0.8.0 — Gate macOS 15+/26+ Swift APIs behind `@available` / `#available`

### Swift bridge compatibility

The Swift bridge previously compiled only on a macOS 26 machine because
`@_cdecl` thunks that use macOS 15+ types (`MTLLogState`,
`MTLLogStateDescriptor`, `MTLResidencySet`, `MTLResidencySetDescriptor`,
`MTLCommandQueueDescriptor.logState`) lacked the `@available(macOS 15.0, *)`
attribute on their function declarations.  Without that attribute the Swift
compiler resolves the type names at the deployment-target level (macOS 11 as
declared in `Package.swift`), which fails on any CI runner whose SDK pre-dates
macOS 15 (e.g., GitHub Actions `macos-14` with Xcode 15).

Every affected `@_cdecl` function already contained the correct
`guard #available(macOS 15.0, *)` runtime guard in its body, providing a safe
nil/0/false fallback on older operating systems.  This release adds the
matching compile-time `@available(macOS 15.0, *)` attribute above the
`@_cdecl` line on all 18 such functions, completing the two-layer guard
pattern required by the Swift compiler.

**Affected functions (all now `@available(macOS 15.0, *)`):**
- `am_device_new_command_queue_with_log_state`
- `am_device_new_log_state`
- `am_device_new_residency_set`
- `am_command_queue_add_residency_set`
- `am_command_queue_remove_residency_set`
- `am_residency_set_add_buffer`
- `am_residency_set_add_texture`
- `am_residency_set_add_heap`
- `am_residency_set_remove_buffer`
- `am_residency_set_remove_texture`
- `am_residency_set_remove_heap`
- `am_residency_set_remove_all_allocations`
- `am_residency_set_contains_buffer`
- `am_residency_set_contains_texture`
- `am_residency_set_allocation_count`
- `am_residency_set_commit`
- `am_residency_set_request_residency`
- `am_residency_set_end_residency`

**macOS 26+ sampler properties** (`MTLSamplerDescriptor.reductionMode` and
`lodBias`) remain guarded by the existing `if #available(macOS 26.0, *)` block
inside `am_device_new_sampler_state`; no function-level attribute is needed
there because only a portion of that function body requires the newer SDK.

**Behaviour on older OS versions is unchanged:** callers on macOS < 15 receive
`nil` / `false` / `0` from the guarded thunks rather than crashing.

## 0.7.0 — Close all audit-v2 gaps (0 remaining)

### Coverage

- Corrected COVERAGE_AUDIT_V2.md: 41 previously-reported GAPS were already
  wrapped as opaque handles in `exhaustive.rs`; the audit methodology was
  over-strict and did not recognise `opaque_symbol_handle!` stubs as active
  wrapper code. All 41 are now correctly marked 🟢 VERIFIED.
- Added three genuinely missing opaque-handle stubs:
  - `MetalCaptureDescriptor` (`MTLCaptureDescriptor`, `MTLCaptureManager.h`)
  - `MetalIndirectComputeCommandEncoder` (`MTLIndirectComputeCommandEncoder`,
    `MTLIndirectCommandBuffer.h`)
  - `MetalIndirectRenderCommandEncoder` (`MTLIndirectRenderCommandEncoder`,
    `MTLIndirectCommandBuffer.h`)
- EXEMPT 1 symbol: `NSProcessInfo` (Foundation class, out of scope for a
  Metal binding crate).
- Final audit: SDK_PUBLIC_SYMBOLS=248, VERIFIED=246, GAPS=0, EXEMPT=1,
  COVERAGE_PCT=99.19%.

### Fixes

- Widen `apple-cf` version constraint to `>=0.6.0, <0.8` to allow `apple-cf`
  v0.7.0.
- Add missing semicolon inside `unsafe` block in `argument.rs` (clippy
  `semicolon_if_nothing_returned`).
- Remove unnecessary `#` in raw string literals in `tests/common/mod.rs`
  (clippy `needless_raw_string_hashes`).

## 0.6.3 — Split integration coverage for bridge areas

- Added focused integration tests for depth/stencil state, sampler state,
  argument encoders, heaps, shared events, and fences under `tests/`.
- Kept the audited top-level Metal / MetalFX symbol surface unchanged while
  broadening runtime validation coverage beyond the original smoke test split.
- Bumped the crate to `0.6.3`.

## 0.6.2 — Exhaustive top-level Metal / MetalFX symbol coverage

- Completed the audited top-level symbol surface for the macOS `Metal.framework`
  and `MetalFX.framework` headers, closing every remaining gap from the
  coverage audit.
- Added exhaustive safe wrappers for the remaining descriptor, reflection,
  resource-state, rasterization-rate, tensor, IO, function-stitching,
  MetalFX base / denoised / frame-interpolator, and `MTL4*` / `MTL4FX*`
  families.
- Added bridge-backed runtime helpers for descriptor-class construction,
  device enumeration / observation, IO compression contexts, and exported
  Metal string constants.
- Added `tests/exhaustive_symbols.rs` to compile-smoke the full audited symbol
  surface.
- Bumped the crate to `0.6.2`.

## 0.6.1 — State descriptors, public pipeline descriptors, and MetalFX scalers

- Added safe wrappers for `MTLCompareFunction`, `MTLStencilOperation`,
  `MTLStencilDescriptor`, `MTLDepthStencilDescriptor`,
  `MTLDepthStencilState`, `MTLSamplerDescriptor`, and `MTLSamplerState`, plus
  encoder bindings for sampler and depth/stencil state.
- Added public argument-buffer descriptor coverage with
  `MTLArgumentBuffersTier`, `MTLBindingAccess`, `MTLTextureType`,
  `MTLArgumentDescriptor`, and descriptor-driven argument-encoder creation.
- Added public compute/render/tile pipeline descriptor wrappers, including
  blend/write-mask enums and descriptor-driven synchronous pipeline
  compilation helpers.
- Added limited `MetalFX` spatial and temporal scaler wrappers, linked the
  `MetalFX.framework`, and refreshed the README / coverage audit for the new
  surface.
- Bumped the crate to `0.6.1`.

## 0.6.0 — Wider Metal resource, command, and advanced-object coverage

- Added safe wrappers for richer `MTLDevice` capability queries, explicit
  command buffer lifecycle/state APIs, `MTLBlitCommandEncoder`,
  `MTLComputeCommandEncoder`, `MTLRenderCommandEncoder`, and
  `MTLRenderPipelineState`.
- Added advanced Metal object coverage for texture views, buffer-backed
  textures, heaps, fences, shared events, dynamic libraries, binary archives,
  argument encoders, indirect command buffers, acceleration-structure handles,
  visible/intersection function tables, counter sample buffers, log state,
  residency sets, and capture scopes.
- Split the Rust FFI and Swift bridge into `core`, `command`, `render`, and
  `advanced` areas following the `screencapturekit-rs` multi-file bridge
  pattern.
- Added examples `05_render_and_explicit_encoders`,
  `06_resources_and_archives`, and `07_advanced_objects`, plus the
  `tests/public_api_smoke.rs` integration smoke test.
- Added `COVERAGE.md`, refreshed the README, and bumped the crate to `0.6.0`.

## 0.5.0 — Compute pipeline + screencapturekit-style bridge split

- **`MetalLibrary`** — compile MSL source via
  `MetalDevice::new_library_with_source(...)`.
- **`MetalFunction`** — `library.new_function(name)`.
- **`ComputePipelineState`** —
  `device.new_compute_pipeline_state(&function)`.
- **`CommandBuffer::dispatch_compute_1d(&pso, &[&buffer, ...],
  threadgroups, threads_per_group)`** — record + dispatch a
  1-D compute kernel against a list of buffers bound at
  consecutive argument slots.
- Swift bridge restructured into 6 files following the
  `screencapturekit-rs` pattern (`Core.swift`, `Device.swift`,
  `Buffer.swift`, `Texture.swift`, `CommandQueue.swift`,
  `Compute.swift`) with shared `am_retain`/`am_release`/`am_borrow`
  helpers. No behaviour change for existing consumers.
- New example `04_compute_shader` runs an end-to-end GPU
  multiply-by-2 kernel.

## 0.1.0

- Initial release.
- Extracted from `apple-cf-rs v0.1.1`'s `metal` feature.
- `MetalDevice::system_default()`, `MetalTexture` getters,
  `IOSurfaceMetalExt::create_metal_texture` (under the default
  `iosurface` feature), `pixel_format` constants,
  `is_ycbcr_biplanar` helper.
