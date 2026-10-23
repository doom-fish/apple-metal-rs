// MTLDevice — handle to a GPU.

import Foundation
import Metal

@_cdecl("ametal_device_system_default")
public func ametal_device_system_default() -> UnsafeMutableRawPointer? {
    guard let device = MTLCreateSystemDefaultDevice() else { return nil }
    return am_retain(device as AnyObject)
}

@_cdecl("ametal_device_release")
public func ametal_device_release(_ handle: UnsafeMutableRawPointer?) { am_release(handle) }

@_cdecl("ametal_device_has_unified_memory")
public func ametal_device_has_unified_memory(_ handle: UnsafeMutableRawPointer?) -> Bool {
    guard let dev: MTLDevice = am_borrow(handle) else { return false }
    return dev.hasUnifiedMemory
}

@_cdecl("ametal_device_recommended_max_working_set_size")
public func ametal_device_recommended_max_working_set_size(_ handle: UnsafeMutableRawPointer?) -> UInt64 {
    guard let dev: MTLDevice = am_borrow(handle) else { return 0 }
    return dev.recommendedMaxWorkingSetSize
}

@_cdecl("ametal_device_supports_family")
public func ametal_device_supports_family(_ handle: UnsafeMutableRawPointer?, _ family: Int64) -> Bool {
    guard let dev: MTLDevice = am_borrow(handle),
          let fam = MTLGPUFamily(rawValue: Int(family))
    else { return false }
    return dev.supportsFamily(fam)
}

@_cdecl("ametal_device_is_removable")
public func ametal_device_is_removable(_ handle: UnsafeMutableRawPointer?) -> Bool {
    guard let dev: MTLDevice = am_borrow(handle) else { return false }
    return dev.isRemovable
}

@_cdecl("ametal_device_is_low_power")
public func ametal_device_is_low_power(_ handle: UnsafeMutableRawPointer?) -> Bool {
    guard let dev: MTLDevice = am_borrow(handle) else { return false }
    return dev.isLowPower
}

@_cdecl("ametal_device_is_headless")
public func ametal_device_is_headless(_ handle: UnsafeMutableRawPointer?) -> Bool {
    guard let dev: MTLDevice = am_borrow(handle) else { return false }
    return dev.isHeadless
}
