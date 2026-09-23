// MTLTexture — 2D GPU image + IOSurface-backed textures.

import Foundation
import Metal
#if canImport(IOSurface)
import IOSurface
#endif

@_cdecl("ametal_device_new_texture")
public func ametal_device_new_texture(
    _ device_handle: UnsafeMutableRawPointer?,
    _ texture_type: Int,
    _ pixel_format: Int,
    _ width: Int,
    _ height: Int,
    _ depth: Int,
    _ mipmapped: Bool,
    _ array_length: Int,
    _ sample_count: Int,
    _ usage: Int,
    _ storage_mode: Int
) -> UnsafeMutableRawPointer? {
    guard let dev: MTLDevice = am_borrow(device_handle),
          let desc = amTextureDescriptor(
              textureType: texture_type,
              pixelFormat: pixel_format,
              width: width,
              height: height,
              depth: depth,
              mipmapped: mipmapped,
              arrayLength: array_length,
              sampleCount: sample_count,
              usage: usage,
              storageMode: storage_mode
          ),
          amDeviceSupportsTexture(dev, desc),
          let tex = dev.makeTexture(descriptor: desc)
    else { return nil }
    return am_retain(tex as AnyObject)
}

@_cdecl("ametal_texture_release")
public func ametal_texture_release(_ handle: UnsafeMutableRawPointer?) { am_release(handle) }

@_cdecl("ametal_texture_width")
public func ametal_texture_width(_ handle: UnsafeMutableRawPointer?) -> Int {
    guard let t: MTLTexture = am_borrow(handle) else { return 0 }
    return t.width
}

@_cdecl("ametal_texture_height")
public func ametal_texture_height(_ handle: UnsafeMutableRawPointer?) -> Int {
    guard let t: MTLTexture = am_borrow(handle) else { return 0 }
    return t.height
}

@_cdecl("ametal_texture_pixel_format")
public func ametal_texture_pixel_format(_ handle: UnsafeMutableRawPointer?) -> Int {
    guard let t: MTLTexture = am_borrow(handle) else { return 0 }
    return Int(bitPattern: t.pixelFormat.rawValue)
}

@_cdecl("ametal_texture_type")
public func ametal_texture_type(_ handle: UnsafeMutableRawPointer?) -> Int {
    guard let t: MTLTexture = am_borrow(handle) else { return 0 }
    return Int(bitPattern: t.textureType.rawValue)
}

#if canImport(IOSurface)
@_cdecl("ametal_device_new_texture_from_iosurface")
public func ametal_device_new_texture_from_iosurface(
    _ device_handle: UnsafeMutableRawPointer?,
    _ iosurface_ptr: UnsafeMutableRawPointer?,
    _ plane_index: Int,
    _ pixel_format: Int,
    _ width: Int,
    _ height: Int
) -> UnsafeMutableRawPointer? {
    guard plane_index >= 0,
          pixel_format >= 0,
          width > 0,
          height > 0,
          let pixelFormat = MTLPixelFormat(rawValue: UInt(pixel_format)),
          pixelFormat != .invalid,
          let dev: MTLDevice = am_borrow(device_handle),
          let iosurface_ptr = iosurface_ptr
    else { return nil }
    let surface = Unmanaged<IOSurfaceRef>.fromOpaque(iosurface_ptr).takeUnretainedValue()
    let desc = MTLTextureDescriptor.texture2DDescriptor(
        pixelFormat: pixelFormat,
        width: width,
        height: height,
        mipmapped: false
    )
    desc.usage = [.shaderRead, .shaderWrite]
    desc.storageMode = .shared
    guard let tex = dev.makeTexture(descriptor: desc, iosurface: surface, plane: plane_index) else {
        return nil
    }
    return am_retain(tex as AnyObject)
}
#endif
