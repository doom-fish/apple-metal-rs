# apple-metal-rs

Safe Rust bindings for Apple's [Metal](https://developer.apple.com/metal/)
framework on macOS, backed by a Swift bridge in the
`screencapturekit-rs` style.

`apple-metal` covers:

- device discovery and capability queries
- buffers, textures, texture views, buffer-backed textures, and `IOSurface`
  zero-copy interop
- command queues/buffers plus explicit blit, compute, and render encoders
- MSL compilation, functions, compute/render pipeline state, and
  descriptor-driven compute/render/tile pipeline creation
- depth/stencil state, sampler state, and descriptor-driven argument encoders
- `MetalFX` spatial / temporal scaler support plus the broader `MetalFX`
  base / denoised / frame-interpolator symbol families
- heaps, events, shared events, dynamic libraries, binary archives, indirect
  command buffers, acceleration-structure handles, visible / intersection
  function tables, counter sample buffers, log state, residency sets, and
  capture scopes
- exhaustive top-level symbol coverage for the audited macOS
  `Metal.framework` + `MetalFX.framework` headers, including descriptor,
  reflection, render-pass, resource-state, rasterization-rate, tensor, IO,
  and `MTL4*` / `MTL4FX*` families

See [`COVERAGE.md`](./COVERAGE.md) for the audited SDK matrix and the note on
which families are exercised by the focused integration tests versus the
broader audited symbol wrappers. `MetalPerformanceShaders` remains out of
scope for this crate.

## Quick start

```rust,no_run
use apple_metal::{resource_options, MetalDevice};

let device = MetalDevice::system_default().expect("no Metal-capable GPU");
println!("{} (registry id {})", device.name(), device.registry_id());

let _queue = device.new_command_queue().expect("command queue");
let buffer = device
    .new_buffer(4096, resource_options::STORAGE_MODE_SHARED)
    .expect("shared buffer");
println!("allocated {} bytes", buffer.length());

unsafe {
    buffer
        .write_bytes(0, b"CPU data")
        .expect("exclusive CPU write");
}
```

## Resource safety contracts

`MetalBuffer::map_read` and `map_write` return scoped guards and serialize CPU
mappings across clones of the same Rust handle. They are `unsafe` because the
caller must still exclude overlapping GPU access; managed readback also
requires a completed blit `synchronize_resource`. Private buffers reject CPU
mapping and can create a shared staging buffer for explicit blit transfers.

Command buffers share lifecycle state across clones. Encoding and submission
methods return `Result`, commit is rejected while an encoder is active, encoder
drop ends encoding exactly once, and repeated submission or post-completion
encoding is rejected. Encoders also reject waiting on a fence after updating
that same fence. The unretained-reference constructor is `unsafe`; regular
command buffers remain the default. Resources written into argument buffers are
retained by the native argument buffer, keyed by encoded offset and binding,
until that argument buffer is released, and are automatically declared with
`useResource` before dispatch or draw, including vertex-stage hazard tracking
for render encoders. Argument setters are available only through an unsafe
scoped destination binding that holds the buffer's CPU mapping lock.
Self-referential and nested retention cycles are rejected.
Descriptor layouts reject empty or unsupported native configurations before
calling Metal and do not impose an artificial binding-index ceiling.

Unsafe texture upload and readback return `TextureTransferError` after checked
format-layout, stride, byte-length, mip, slice, region, storage-mode, and native
integer validation. The caller must exclude overlapping GPU and CPU/native
aliases, including buffer mappings behind buffer-backed textures. Private
textures require a GPU staging path rather than CPU transfer methods.

### Zero-copy from `IOSurface`

With the default `iosurface` feature:

```rust,no_run
# #[cfg(feature = "iosurface")]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
use apple_metal::{IOSurfaceMetalExt, MetalDevice};
use apple_cf::iosurface::IOSurface;

let device = MetalDevice::system_default().unwrap();
let surface: IOSurface = todo!("get one from ScreenCaptureKit / AVFoundation / etc");
let texture = surface.create_metal_texture(&device, 0).unwrap();
println!("{}x{} MTLTexture", texture.width(), texture.height());
# Ok(()) }
# #[cfg(not(feature = "iosurface"))] fn main() {}
```

`create_metal_texture` uses the selected plane's actual width, height, and row
stride, including odd bi-planar dimensions, and returns `IOSurfaceMetalError`
for unsupported formats or incompatible layouts. Packed `l10r` surfaces are
reported as unsupported rather than mapped to a Metal format with an uncertain
storage representation.

## Examples

- `01_get_device` — create the default Metal device and print basic identity.
- `02_caps_buffer_texture` — inspect device capabilities, allocate buffers, and
  create textures.
- `03_command_buffer_blit` — submit a simple blit copy on the GPU.
- `04_compute_shader` — compile MSL source and dispatch a compute kernel.
- `05_render_and_explicit_encoders` — exercise explicit blit, compute, and
  render encoders in one program.
- `06_resources_and_archives` — use argument encoders, heaps, log state,
  dynamic libraries, and binary archives.
- `07_advanced_objects` — touch shared events, fences, counters, indirect
  command buffers, residency sets, and capture scopes.

Run one directly with:

```bash
cargo run --example 05_render_and_explicit_encoders
```

## Status

- Audited against the active Xcode Metal SDK headers
  (`MacOSX26.2.sdk/System/Library/Frameworks/Metal.framework/Headers`).
- `COVERAGE.md` tracks implemented, partial, and deferred Metal families.
- The crate continues to prefer safe, synchronous handle wrappers over raw
  Objective-C messaging from Rust.
