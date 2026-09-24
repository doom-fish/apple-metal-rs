import Foundation
import Metal

@inline(__always)
public func am_copy_string(_ value: String?) -> UnsafeMutablePointer<CChar>? {
    guard let value else { return nil }
    return strdup(value)
}

@inline(__always)
public func am_store_error(
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ error: Error
) {
    outErrorMessage?.pointee = strdup((error as NSError).localizedDescription)
}

@inline(__always)
public func am_store_error_message(
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ message: String
) {
    outErrorMessage?.pointee = strdup(message)
}

@inline(__always)
public func am_url(from path: UnsafePointer<CChar>?) -> URL? {
    guard let path else { return nil }
    return URL(fileURLWithPath: String(cString: path))
}

@inline(__always)
public func am_copy_data(_ data: Data?) -> UnsafeMutableRawPointer? {
    guard let data, !data.isEmpty, let buffer = malloc(data.count) else { return nil }
    data.copyBytes(to: buffer.assumingMemoryBound(to: UInt8.self), count: data.count)
    return buffer
}

func amTextureDescriptor(
    textureType: Int,
    pixelFormat: Int,
    width: Int,
    height: Int,
    depth: Int,
    mipmapped: Bool,
    arrayLength: Int,
    sampleCount: Int,
    usage: Int,
    storageMode: Int
) -> MTLTextureDescriptor? {
    guard width > 0,
          height > 0,
          depth > 0,
          arrayLength > 0,
          sampleCount > 0,
          let rawType = UInt(exactly: textureType),
          let type = MTLTextureType(rawValue: rawType),
          let rawFormat = UInt(exactly: pixelFormat),
          rawFormat != 263,
          let format = MTLPixelFormat(rawValue: rawFormat),
          format != .invalid,
          let rawUsage = UInt(exactly: usage),
          rawUsage & ~UInt(0x17) == 0,
          let rawStorage = UInt(exactly: storageMode),
          let storage = MTLStorageMode(rawValue: rawStorage),
          storage == .shared || storage == .managed || storage == .private || storage == .memoryless
    else { return nil }

    let planar = width <= 16_384 && height <= 16_384
    let shapeIsValid: Bool
    switch type {
    case .type1D, .type1DArray:
        shapeIsValid = height == 1 && depth == 1 && !mipmapped && planar
    case .type2D, .type2DArray, .type2DMultisample, .type2DMultisampleArray:
        shapeIsValid = depth == 1 && planar
    case .typeCube, .typeCubeArray:
        shapeIsValid = width == height && depth == 1 && planar
    case .type3D:
        shapeIsValid = width <= 2_048 && height <= 2_048 && depth <= 2_048
    default:
        shapeIsValid = false
    }
    let layersAreValid: Bool
    switch type {
    case .type1DArray, .type2DArray, .type2DMultisampleArray:
        layersAreValid = arrayLength <= 2_048
    case .typeCubeArray:
        layersAreValid = arrayLength <= 2_048 / 6
    default:
        layersAreValid = arrayLength == 1
    }
    let isMultisample = type == .type2DMultisample || type == .type2DMultisampleArray
    guard shapeIsValid,
          layersAreValid,
          isMultisample ? [2, 4, 8].contains(sampleCount) && !mipmapped : sampleCount == 1
    else { return nil }

    let descriptor = MTLTextureDescriptor()
    descriptor.textureType = type
    descriptor.pixelFormat = format
    descriptor.width = width
    descriptor.height = height
    descriptor.depth = depth
    descriptor.arrayLength = arrayLength
    descriptor.sampleCount = sampleCount
    if mipmapped {
        let largest = type == .type3D ? max(width, height, depth) : max(width, height)
        descriptor.mipmapLevelCount = Int.bitWidth - largest.leadingZeroBitCount
    }
    descriptor.usage = MTLTextureUsage(rawValue: rawUsage)
    descriptor.storageMode = storage
    return descriptor
}

@_cdecl("ametal_object_release")
public func ametal_object_release(_ handle: UnsafeMutableRawPointer?) {
    am_release(handle)
}

@_cdecl("ametal_object_copy_label")
public func ametal_object_copy_label(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    if let value: MTLRenderPipelineState = am_borrow(handle) {
        return am_copy_string(value.label)
    }
    if let value: MTLComputePipelineState = am_borrow(handle) {
        return am_copy_string(value.label)
    }
    if let value: MTLDepthStencilState = am_borrow(handle) {
        return am_copy_string(value.label)
    }
    if let value: MTLSamplerState = am_borrow(handle) {
        return am_copy_string(value.label)
    }
    if let value: MTLCommandQueue = am_borrow(handle) {
        return am_copy_string(value.label)
    }
    if let value: MTLBinaryArchive = am_borrow(handle) {
        return am_copy_string(value.label)
    }
    if let value: MTLCaptureScope = am_borrow(handle) {
        return am_copy_string(value.label)
    }
    if let value: MTLResource = am_borrow(handle) {
        return am_copy_string(value.label)
    }
    return nil
}

func amDeviceSupportsTexture(_ device: MTLDevice, _ descriptor: MTLTextureDescriptor) -> Bool {
    if descriptor.sampleCount > 1, !device.supportsTextureSampleCount(descriptor.sampleCount) {
        return false
    }
    return amDeviceSupportsPixelFormat(device, descriptor.pixelFormat)
}

func amDeviceSupportsPixelFormat(_ device: MTLDevice, _ format: MTLPixelFormat) -> Bool {
    switch format.rawValue {
    case 255, 262:
        return device.isDepth24Stencil8PixelFormatSupported
    case 130...135, 140...143, 150...153:
        return device.supportsBCTextureCompression
    case 222...236:
        return device.supportsFamily(.apple6)
    case 552...555:
        return device.supportsFamily(.apple3)
    case 11, 31, 40...43, 160...218:
        return device.supportsFamily(.apple2)
    default:
        return true
    }
}
