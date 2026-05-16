# apple-metal-rs

Safe Rust bindings for Apple's [Metal](https://developer.apple.com/metal/)
framework on macOS, backed by a Swift bridge in the
`screencapturekit-rs` style.

`apple-metal` 0.6.0 now covers:

- device discovery and capability queries
- buffers, textures, texture views, buffer-backed textures, and `IOSurface`
  zero-copy interop
- command queues/buffers plus explicit blit, compute, and render encoders
- MSL compilation, functions, compute pipeline state, and render pipeline state
- heaps, events, shared events, dynamic libraries, binary archives, argument
  encoders, indirect command buffers, acceleration-structure handles, visible /
  intersection function tables, counter sample buffers, log state, residency
  sets, and capture scopes

See [`COVERAGE.md`](./COVERAGE.md) for the audited SDK matrix and the deferred
families that are still intentionally out of scope for this crate release.
`MetalFX` and `MetalPerformanceShaders` are separate frameworks and are not part
of `apple-metal`.

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
```

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
