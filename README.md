# apple-metal-rs

Safe Rust bindings for Apple's [Metal](https://developer.apple.com/metal/)
framework on macOS, backed by a Swift bridge in the
`screencapturekit-rs` style.

`apple-metal` covers:

- device discovery and capability queries
- buffers (including buffers created from bytes), 1D/2D/3D/array/cube/multisample
  textures, texture views, buffer-backed textures, and `IOSurface` zero-copy
  interop
- `pixel_format::bytes_per_pixel`, a byte-size table for every `MTLPixelFormat`
  in the SDK, used by the crate's own CPU transfer and texture-view checks
- command queues/buffers with scheduled/completed handlers, plus explicit blit,
  compute, and render encoders
- MSL compilation, functions, compute/render pipeline state, and
  descriptor-driven compute/render/tile pipeline creation
- depth/stencil state, sampler state, and descriptor-driven argument encoders
- `MetalFX` spatial / temporal scaler support plus the broader `MetalFX`
  base / denoised / frame-interpolator symbol families
- heaps, events, shared events with listener notifications, dynamic
  libraries, binary archives, indirect
  command buffers, acceleration-structure handles, visible / intersection
  function tables, counter sample buffers, log state, residency sets, and
  capture scopes
- a named Rust item for every audited top-level `Metal.framework` +
  `MetalFX.framework` symbol; most of those are opaque handles, `new()`-only
  classes, or raw integer values without methods

Metal 4 is not wrapped: the `MTL4*` families and `MTLTensor` are opaque handles
without constructors or methods. See [`COVERAGE.md`](./COVERAGE.md) for what the
audit counts and which families are functional. `MetalPerformanceShaders`
remains out of scope for this crate.

## Requirements

- macOS 11 or later and a Swift toolchain (Xcode or the Command Line Tools) to
  build the bridge.
- Residency sets and shader log state need macOS 15: `new_residency_set` and
  `new_log_state` return an error on older systems. The `MetalFX` scalers need
  macOS 13, and their constructors return `None` before that.

Only one `apple-metal` release from 0.10 on can be linked into a binary: the
package declares `links = "apple_metal_bridge"`, so Cargo rejects a graph with
two of them. Its Swift bridge (`AppleMetalSwiftBridge`, C symbols `ametal_*`)
shares no symbol with the `AppleMetalBridge` of apple-metal 0.9 and earlier, so
an older copy pulled in by another crate still links, but its types don't
interoperate with this one.

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
integer validation. Pixel sizes come from `pixel_format::bytes_per_pixel`, which
is checked against the SDK's `MTLPixelFormat.h` (`BGRA10_XR` is 8 bytes). The caller must exclude overlapping GPU and CPU/native
aliases, including buffer mappings behind buffer-backed textures. Private
textures require a GPU staging path rather than CPU transfer methods.

`MetalBuffer::new_texture_view_2d` returns `TextureViewError` unless the view
fits inside the buffer and its offset and row stride meet the device's linear
texture alignment. `MetalDevice::new_texture` returns `None` for descriptors
Metal would abort on (unknown formats or usage bits, extents beyond 16384, or
2048 for 3D textures and array layers, unsupported sample counts or formats).

Command-buffer handlers, shared-event notifications and the device observer
take `Send + 'static` Rust closures. Metal owns them until it releases its
block, so they may run after the Rust wrapper is dropped; panics are contained.
A handler whose command buffer is released without being committed receives
`CommandBufferError::NotExecuted`. Dropping a `MetalDeviceObserver` removes the
observer before its closure is freed.

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

- The symbol audit was generated against
  `MacOSX26.2.sdk/System/Library/Frameworks/Metal.framework/Headers`; the
  pixel-format table was checked against `MacOSX26.5.sdk`.
- `COVERAGE.md` tracks implemented, partial, and deferred Metal families.
- The crate continues to prefer safe, synchronous handle wrappers over raw
  Objective-C messaging from Rust.
