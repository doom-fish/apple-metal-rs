# Changelog

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
