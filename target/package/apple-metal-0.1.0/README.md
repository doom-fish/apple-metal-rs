# apple-metal-rs

Safe Rust bindings for Apple's [Metal](https://developer.apple.com/metal/)
framework on macOS.

The crate is intentionally tiny: it exposes the bits needed to obtain a
GPU device handle and bridge it with Apple's
[`IOSurface`](https://developer.apple.com/documentation/iosurface) for
zero-copy texture access. For higher-level rendering you can pair it
with the [`metal`](https://crates.io/crates/metal) crate or
`objc2-metal`.

## Quick start

```rust,no_run
use apple_metal::MetalDevice;

let device = MetalDevice::system_default().expect("no Metal-capable GPU");
println!("got Metal device {:p}", device.as_ptr());
```

### Zero-copy from IOSurface

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

## Status

v0.1 covers:

- `MetalDevice::system_default()` — `MTLCreateSystemDefaultDevice`
- `MetalTexture` with `width / height / pixel_format / as_ptr`
- `IOSurfaceMetalExt::create_metal_texture(device, plane_index)`
- Common `MTLPixelFormat` constants
- `is_ycbcr_biplanar(fourcc)` helper

Extracted from `apple-cf-rs` v0.1.1's `metal` feature so the Metal
surface area can grow on its own without bloating CoreFoundation.
