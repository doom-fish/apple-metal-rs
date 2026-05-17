# Changelog

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
