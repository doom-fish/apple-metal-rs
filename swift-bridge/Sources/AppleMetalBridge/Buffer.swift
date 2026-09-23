// MTLBuffer — generic GPU memory.

import Foundation
import Metal

@_cdecl("ametal_device_new_buffer")
public func ametal_device_new_buffer(
    _ device_handle: UnsafeMutableRawPointer?,
    _ length: Int,
    _ options: UInt
) -> UnsafeMutableRawPointer? {
    guard length >= 0,
          let dev: MTLDevice = am_borrow(device_handle),
          let buf = dev.makeBuffer(length: length, options: MTLResourceOptions(rawValue: options))
    else { return nil }
    return am_retain(buf as AnyObject)
}

@_cdecl("ametal_device_new_buffer_with_bytes")
public func ametal_device_new_buffer_with_bytes(
    _ device_handle: UnsafeMutableRawPointer?,
    _ bytes: UnsafeRawPointer?,
    _ length: Int,
    _ options: UInt
) -> UnsafeMutableRawPointer? {
    let storageMode = (options & 0xF0) >> 4
    guard length > 0,
          let bytes,
          storageMode == MTLStorageMode.shared.rawValue || storageMode == MTLStorageMode.managed.rawValue,
          let dev: MTLDevice = am_borrow(device_handle),
          let buf = dev.makeBuffer(bytes: bytes, length: length, options: MTLResourceOptions(rawValue: options))
    else { return nil }
    return am_retain(buf as AnyObject)
}

@_cdecl("ametal_buffer_release")
public func ametal_buffer_release(_ handle: UnsafeMutableRawPointer?) { am_release(handle) }

@_cdecl("ametal_buffer_length")
public func ametal_buffer_length(_ handle: UnsafeMutableRawPointer?) -> Int {
    guard let buf: MTLBuffer = am_borrow(handle) else { return 0 }
    return buf.length
}

@_cdecl("ametal_buffer_storage_mode")
public func ametal_buffer_storage_mode(_ handle: UnsafeMutableRawPointer?) -> Int {
    guard let buf: MTLBuffer = am_borrow(handle) else { return 0 }
    return Int(bitPattern: buf.storageMode.rawValue)
}

@_cdecl("ametal_buffer_contents")
public func ametal_buffer_contents(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let buf: MTLBuffer = am_borrow(handle),
          buf.storageMode == .shared || buf.storageMode == .managed
    else { return nil }
    return buf.contents()
}

@_cdecl("ametal_buffer_new_staging_buffer")
public func ametal_buffer_new_staging_buffer(
    _ handle: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let buf: MTLBuffer = am_borrow(handle),
          let staging = buf.device.makeBuffer(length: buf.length, options: .storageModeShared)
    else { return nil }
    return am_retain(staging as AnyObject)
}
