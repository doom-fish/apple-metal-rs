// Apple Metal bridge — @_cdecl wrappers around Apple's Swift Metal
// API. Avoids hand-rolled objc_msgSend gymnastics in Rust.
//
// All "handle" pointers exchanged across this FFI are Unmanaged-toOpaque
// retained pointers — callers pair every am_*_release with the
// returning constructor.

import Foundation
import Metal
#if canImport(IOSurface)
import IOSurface
#endif

// ---- Device ----

@_cdecl("am_device_system_default")
public func am_device_system_default() -> UnsafeMutableRawPointer? {
    guard let device = MTLCreateSystemDefaultDevice() else { return nil }
    return Unmanaged.passRetained(device as AnyObject).toOpaque()
}

@_cdecl("am_device_release")
public func am_device_release(_ handle: UnsafeMutableRawPointer?) {
    guard let handle = handle else { return }
    Unmanaged<AnyObject>.fromOpaque(handle).release()
}

@_cdecl("am_device_has_unified_memory")
public func am_device_has_unified_memory(_ handle: UnsafeMutableRawPointer?) -> Bool {
    guard let handle = handle,
          let dev = Unmanaged<AnyObject>.fromOpaque(handle).takeUnretainedValue() as? MTLDevice
    else { return false }
    return dev.hasUnifiedMemory
}

@_cdecl("am_device_recommended_max_working_set_size")
public func am_device_recommended_max_working_set_size(_ handle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let handle = handle,
          let dev = Unmanaged<AnyObject>.fromOpaque(handle).takeUnretainedValue() as? MTLDevice
    else { return 0 }
    return dev.recommendedMaxWorkingSetSize
}

@_cdecl("am_device_supports_family")
public func am_device_supports_family(_ handle: UnsafeMutableRawPointer?, _ family: Int64) -> Bool {
    guard let handle = handle,
          let dev = Unmanaged<AnyObject>.fromOpaque(handle).takeUnretainedValue() as? MTLDevice,
          let fam = MTLGPUFamily(rawValue: Int(family))
    else { return false }
    return dev.supportsFamily(fam)
}

// ---- Buffer ----

@_cdecl("am_device_new_buffer")
public func am_device_new_buffer(
    _ device_handle: UnsafeMutableRawPointer?,
    _ length: Int,
    _ options: UInt
) -> UnsafeMutableRawPointer? {
    guard let device_handle = device_handle,
          let dev = Unmanaged<AnyObject>.fromOpaque(device_handle).takeUnretainedValue() as? MTLDevice,
          let buf = dev.makeBuffer(length: length, options: MTLResourceOptions(rawValue: options))
    else { return nil }
    return Unmanaged.passRetained(buf as AnyObject).toOpaque()
}

@_cdecl("am_buffer_release")
public func am_buffer_release(_ handle: UnsafeMutableRawPointer?) {
    guard let handle = handle else { return }
    Unmanaged<AnyObject>.fromOpaque(handle).release()
}

@_cdecl("am_buffer_length")
public func am_buffer_length(_ handle: UnsafeMutableRawPointer?) -> Int {
    guard let handle = handle,
          let buf = Unmanaged<AnyObject>.fromOpaque(handle).takeUnretainedValue() as? MTLBuffer
    else { return 0 }
    return buf.length
}

@_cdecl("am_buffer_contents")
public func am_buffer_contents(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let handle = handle,
          let buf = Unmanaged<AnyObject>.fromOpaque(handle).takeUnretainedValue() as? MTLBuffer
    else { return nil }
    return buf.contents()
}

// ---- Texture (2D, from descriptor) ----

@_cdecl("am_device_new_texture_2d")
public func am_device_new_texture_2d(
    _ device_handle: UnsafeMutableRawPointer?,
    _ pixel_format: UInt,
    _ width: Int,
    _ height: Int,
    _ mipmapped: Bool,
    _ usage: UInt,
    _ storage_mode: UInt
) -> UnsafeMutableRawPointer? {
    guard let device_handle = device_handle,
          let dev = Unmanaged<AnyObject>.fromOpaque(device_handle).takeUnretainedValue() as? MTLDevice,
          let pf = MTLPixelFormat(rawValue: pixel_format)
    else { return nil }
    let desc = MTLTextureDescriptor.texture2DDescriptor(
        pixelFormat: pf, width: width, height: height, mipmapped: mipmapped)
    desc.usage = MTLTextureUsage(rawValue: usage)
    if let sm = MTLStorageMode(rawValue: storage_mode) {
        desc.storageMode = sm
    }
    guard let tex = dev.makeTexture(descriptor: desc) else { return nil }
    return Unmanaged.passRetained(tex as AnyObject).toOpaque()
}

@_cdecl("am_texture_release")
public func am_texture_release(_ handle: UnsafeMutableRawPointer?) {
    guard let handle = handle else { return }
    Unmanaged<AnyObject>.fromOpaque(handle).release()
}

@_cdecl("am_texture_width")
public func am_texture_width(_ handle: UnsafeMutableRawPointer?) -> Int {
    guard let handle = handle,
          let t = Unmanaged<AnyObject>.fromOpaque(handle).takeUnretainedValue() as? MTLTexture
    else { return 0 }
    return t.width
}

@_cdecl("am_texture_height")
public func am_texture_height(_ handle: UnsafeMutableRawPointer?) -> Int {
    guard let handle = handle,
          let t = Unmanaged<AnyObject>.fromOpaque(handle).takeUnretainedValue() as? MTLTexture
    else { return 0 }
    return t.height
}

@_cdecl("am_texture_pixel_format")
public func am_texture_pixel_format(_ handle: UnsafeMutableRawPointer?) -> UInt {
    guard let handle = handle,
          let t = Unmanaged<AnyObject>.fromOpaque(handle).takeUnretainedValue() as? MTLTexture
    else { return 0 }
    return t.pixelFormat.rawValue
}

// ---- IOSurface → texture ----

#if canImport(IOSurface)
@_cdecl("am_device_new_texture_from_iosurface")
public func am_device_new_texture_from_iosurface(
    _ device_handle: UnsafeMutableRawPointer?,
    _ iosurface_ptr: UnsafeMutableRawPointer?,
    _ plane_index: Int,
    _ pixel_format: UInt,
    _ width: Int,
    _ height: Int
) -> UnsafeMutableRawPointer? {
    guard let device_handle = device_handle,
          let iosurface_ptr = iosurface_ptr,
          let dev = Unmanaged<AnyObject>.fromOpaque(device_handle).takeUnretainedValue() as? MTLDevice,
          let pf = MTLPixelFormat(rawValue: pixel_format)
    else { return nil }
    let surface = Unmanaged<IOSurfaceRef>.fromOpaque(iosurface_ptr).takeUnretainedValue()
    let desc = MTLTextureDescriptor.texture2DDescriptor(
        pixelFormat: pf, width: width, height: height, mipmapped: false)
    desc.usage = [.shaderRead, .shaderWrite]
    desc.storageMode = .shared
    guard let tex = dev.makeTexture(descriptor: desc, iosurface: surface, plane: plane_index)
    else { return nil }
    return Unmanaged.passRetained(tex as AnyObject).toOpaque()
}
#endif

// MARK: - Command queue + command buffer + blit (v0.4)

@_cdecl("am_device_new_command_queue")
public func am_device_new_command_queue(_ device_handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let device_handle = device_handle,
          let dev = Unmanaged<AnyObject>.fromOpaque(device_handle).takeUnretainedValue() as? MTLDevice,
          let queue = dev.makeCommandQueue()
    else { return nil }
    return Unmanaged.passRetained(queue as AnyObject).toOpaque()
}

@_cdecl("am_command_queue_release")
public func am_command_queue_release(_ handle: UnsafeMutableRawPointer?) {
    guard let handle = handle else { return }
    Unmanaged<AnyObject>.fromOpaque(handle).release()
}

@_cdecl("am_command_queue_new_command_buffer")
public func am_command_queue_new_command_buffer(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let handle = handle,
          let q = Unmanaged<AnyObject>.fromOpaque(handle).takeUnretainedValue() as? MTLCommandQueue,
          let cb = q.makeCommandBuffer()
    else { return nil }
    return Unmanaged.passRetained(cb as AnyObject).toOpaque()
}

@_cdecl("am_command_buffer_release")
public func am_command_buffer_release(_ handle: UnsafeMutableRawPointer?) {
    guard let handle = handle else { return }
    Unmanaged<AnyObject>.fromOpaque(handle).release()
}

@_cdecl("am_command_buffer_commit")
public func am_command_buffer_commit(_ handle: UnsafeMutableRawPointer?) {
    guard let handle = handle,
          let cb = Unmanaged<AnyObject>.fromOpaque(handle).takeUnretainedValue() as? MTLCommandBuffer
    else { return }
    cb.commit()
}

@_cdecl("am_command_buffer_wait_until_completed")
public func am_command_buffer_wait_until_completed(_ handle: UnsafeMutableRawPointer?) {
    guard let handle = handle,
          let cb = Unmanaged<AnyObject>.fromOpaque(handle).takeUnretainedValue() as? MTLCommandBuffer
    else { return }
    cb.waitUntilCompleted()
}

/// Blit a region from one buffer to another. Returns false on bad pointers.
@_cdecl("am_command_buffer_blit_copy_buffer")
public func am_command_buffer_blit_copy_buffer(
    _ cb_handle: UnsafeMutableRawPointer?,
    _ src_handle: UnsafeMutableRawPointer?,
    _ src_offset: Int,
    _ dst_handle: UnsafeMutableRawPointer?,
    _ dst_offset: Int,
    _ size: Int
) -> Bool {
    guard let cb_handle = cb_handle, let src_handle = src_handle, let dst_handle = dst_handle,
          let cb = Unmanaged<AnyObject>.fromOpaque(cb_handle).takeUnretainedValue() as? MTLCommandBuffer,
          let src = Unmanaged<AnyObject>.fromOpaque(src_handle).takeUnretainedValue() as? MTLBuffer,
          let dst = Unmanaged<AnyObject>.fromOpaque(dst_handle).takeUnretainedValue() as? MTLBuffer,
          let blit = cb.makeBlitCommandEncoder()
    else { return false }
    blit.copy(from: src, sourceOffset: src_offset,
              to: dst, destinationOffset: dst_offset,
              size: size)
    blit.endEncoding()
    return true
}
