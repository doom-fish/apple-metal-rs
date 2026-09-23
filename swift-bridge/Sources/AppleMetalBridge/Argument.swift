import Foundation
import Metal
import ObjectiveC

private let amArgumentDescriptorStride = 6
private var amArgumentEncoderStateKey: UInt8 = 0
private var amArgumentBufferResourcesKey: UInt8 = 0
private var amCommandEncoderArgumentBuffersKey: UInt8 = 0
private let amArgumentResourceGraphLock = NSLock()

enum AMArgumentBindingKind: Int, Hashable {
    case buffer = 1
    case texture = 2
    case sampler = 3
    case constant = 4
}

struct AMArgumentBindingRange {
    let start: Int
    let last: Int
    let kind: AMArgumentBindingKind
}

private final class AMArgumentEncoderState: NSObject {
    let layout: [AMArgumentBindingRange]?
    var argumentBuffer: MTLBuffer?
    var argumentBufferOffset = 0

    init(layout: [AMArgumentBindingRange]?) {
        self.layout = layout
    }
}

private struct AMArgumentBindingKey: Hashable {
    let argumentBufferOffset: Int
    let kind: AMArgumentBindingKind
    let index: Int
}

private final class AMArgumentBufferResources: NSObject {
    private let lock = NSLock()
    private var bindings = [AMArgumentBindingKey: AnyObject]()

    func bind(
        _ resource: AnyObject,
        argumentBufferOffset: Int,
        kind: AMArgumentBindingKind,
        index: Int
    ) {
        lock.lock()
        bindings[
            AMArgumentBindingKey(
                argumentBufferOffset: argumentBufferOffset,
                kind: kind,
                index: index
            )
        ] = resource
        lock.unlock()
    }

    func allResources() -> [AnyObject] {
        lock.lock()
        let resources = Array(bindings.values)
        lock.unlock()
        return resources
    }
}

private final class AMCommandEncoderArgumentBuffers: NSObject {
    private let lock = NSLock()
    private var buffers = [MTLBuffer]()

    func add(_ buffer: MTLBuffer) {
        lock.lock()
        if !buffers.contains(where: { ($0 as AnyObject) === (buffer as AnyObject) }) {
            buffers.append(buffer)
        }
        lock.unlock()
    }

    func allBuffers() -> [MTLBuffer] {
        lock.lock()
        let result = buffers
        lock.unlock()
        return result
    }
}

func amRegisterArgumentEncoder(
    _ encoder: MTLArgumentEncoder,
    layout: [AMArgumentBindingRange]?
) {
    objc_setAssociatedObject(
        encoder as AnyObject,
        &amArgumentEncoderStateKey,
        AMArgumentEncoderState(layout: layout),
        .OBJC_ASSOCIATION_RETAIN_NONATOMIC
    )
}

private func amArgumentEncoderState(_ encoder: MTLArgumentEncoder) -> AMArgumentEncoderState {
    if let state = objc_getAssociatedObject(
        encoder as AnyObject,
        &amArgumentEncoderStateKey
    ) as? AMArgumentEncoderState {
        return state
    }
    let state = AMArgumentEncoderState(layout: nil)
    objc_setAssociatedObject(
        encoder as AnyObject,
        &amArgumentEncoderStateKey,
        state,
        .OBJC_ASSOCIATION_RETAIN_NONATOMIC
    )
    return state
}

private func amArgumentBufferResources(_ buffer: MTLBuffer) -> AMArgumentBufferResources {
    let object = buffer as AnyObject
    objc_sync_enter(object)
    defer { objc_sync_exit(object) }
    if let resources = objc_getAssociatedObject(
        object,
        &amArgumentBufferResourcesKey
    ) as? AMArgumentBufferResources {
        return resources
    }
    let resources = AMArgumentBufferResources()
    objc_setAssociatedObject(
        object,
        &amArgumentBufferResourcesKey,
        resources,
        .OBJC_ASSOCIATION_RETAIN_NONATOMIC
    )
    return resources
}

private func amExistingArgumentBufferResources(
    _ buffer: MTLBuffer
) -> AMArgumentBufferResources? {
    objc_getAssociatedObject(
        buffer as AnyObject,
        &amArgumentBufferResourcesKey
    ) as? AMArgumentBufferResources
}

private func amCommandEncoderArgumentBuffers(
    _ encoder: MTLCommandEncoder
) -> AMCommandEncoderArgumentBuffers {
    let object = encoder as AnyObject
    if let buffers = objc_getAssociatedObject(
        object,
        &amCommandEncoderArgumentBuffersKey
    ) as? AMCommandEncoderArgumentBuffers {
        return buffers
    }
    let buffers = AMCommandEncoderArgumentBuffers()
    objc_setAssociatedObject(
        object,
        &amCommandEncoderArgumentBuffersKey,
        buffers,
        .OBJC_ASSOCIATION_RETAIN_NONATOMIC
    )
    return buffers
}

func amTrackArgumentBuffer(_ encoder: MTLCommandEncoder, buffer: MTLBuffer) {
    amCommandEncoderArgumentBuffers(encoder).add(buffer)
}

private func amTrackedArgumentResources(_ encoder: MTLCommandEncoder) -> [MTLResource] {
    var pendingBuffers = amCommandEncoderArgumentBuffers(encoder).allBuffers()
    var visitedBuffers = Set<ObjectIdentifier>()
    var visitedResources = Set<ObjectIdentifier>()
    var resources = [MTLResource]()

    while let buffer = pendingBuffers.popLast() {
        let bufferIdentifier = ObjectIdentifier(buffer as AnyObject)
        guard visitedBuffers.insert(bufferIdentifier).inserted,
              let retained = amExistingArgumentBufferResources(buffer)
        else { continue }
        for object in retained.allResources() {
            if let nestedBuffer = object as? MTLBuffer {
                pendingBuffers.append(nestedBuffer)
            }
            guard let resource = object as? MTLResource else { continue }
            let identifier = ObjectIdentifier(resource as AnyObject)
            if visitedResources.insert(identifier).inserted {
                resources.append(resource)
            }
        }
    }
    return resources
}

func amDeclareArgumentBufferResources(_ encoder: MTLComputeCommandEncoder) {
    for resource in amTrackedArgumentResources(encoder) {
        let usage: MTLResourceUsage = resource is MTLTexture
            ? [.read, .write, .sample]
            : [.read, .write]
        encoder.useResource(resource, usage: usage)
    }
}

func amDeclareArgumentBufferResources(_ encoder: MTLRenderCommandEncoder) {
    for resource in amTrackedArgumentResources(encoder) {
        let usage: MTLResourceUsage = resource is MTLTexture
            ? [.read, .write, .sample]
            : [.read, .write]
        encoder.useResource(resource, usage: usage, stages: .vertex)
    }
}

private func amArgumentEncoderAccepts(
    _ encoder: MTLArgumentEncoder,
    kind: AMArgumentBindingKind,
    index: Int
) -> Bool {
    guard index >= 0 else { return false }
    guard let layout = amArgumentEncoderState(encoder).layout else { return true }
    return layout.contains {
        index >= $0.start && index <= $0.last && $0.kind == kind
    }
}

private func amSupportsConstantArgumentDataType(_ dataType: MTLDataType) -> Bool {
    switch dataType.rawValue {
    case 3...56:
        return true
    case 81...88:
        if #available(macOS 12.0, *) {
            return true
        }
    case 121...124:
        if #available(macOS 14.0, *) {
            return true
        }
    default:
        break
    }
    return false
}

private func amRetainArgumentResource(
    _ resource: AnyObject,
    encoder: MTLArgumentEncoder,
    kind: AMArgumentBindingKind,
    index: Int
) -> Bool {
    let state = amArgumentEncoderState(encoder)
    guard let argumentBuffer = state.argumentBuffer else {
        return false
    }
    amArgumentResourceGraphLock.lock()
    defer { amArgumentResourceGraphLock.unlock() }
    if let buffer = resource as? MTLBuffer,
       amArgumentBufferReaches(buffer, target: argumentBuffer) {
        return false
    }
    if let texture = resource as? MTLTexture,
       let buffer = texture.buffer,
       amArgumentBufferReaches(buffer, target: argumentBuffer) {
        return false
    }
    amArgumentBufferResources(argumentBuffer).bind(
        resource,
        argumentBufferOffset: state.argumentBufferOffset,
        kind: kind,
        index: index
    )
    return true
}

private func amArgumentBufferReaches(_ start: MTLBuffer, target: MTLBuffer) -> Bool {
    var pending = [start]
    var visited = Set<ObjectIdentifier>()
    let targetIdentifier = ObjectIdentifier(target as AnyObject)

    while let buffer = pending.popLast() {
        let identifier = ObjectIdentifier(buffer as AnyObject)
        if identifier == targetIdentifier {
            return true
        }
        guard visited.insert(identifier).inserted,
              let resources = amExistingArgumentBufferResources(buffer)
        else { continue }
        for resource in resources.allResources() {
            if let nestedBuffer = resource as? MTLBuffer {
                pending.append(nestedBuffer)
            } else if let texture = resource as? MTLTexture,
                      let backingBuffer = texture.buffer {
                pending.append(backingBuffer)
            }
        }
    }
    return false
}

@_cdecl("ametal_device_argument_buffers_support")
public func ametal_device_argument_buffers_support(_ handle: UnsafeMutableRawPointer?) -> Int {
    guard let device: MTLDevice = am_borrow(handle) else { return 0 }
    return Int(device.argumentBuffersSupport.rawValue)
}

@_cdecl("ametal_device_new_argument_encoder_with_descriptors")
public func ametal_device_new_argument_encoder_with_descriptors(
    _ handle: UnsafeMutableRawPointer?,
    _ descriptors: UnsafePointer<UInt>?,
    _ descriptorCount: Int
) -> UnsafeMutableRawPointer? {
    guard descriptorCount > 0,
          descriptorCount <= Int.max / amArgumentDescriptorStride,
          let device: MTLDevice = am_borrow(handle),
          let descriptors
    else { return nil }

    var arguments = [MTLArgumentDescriptor]()
    var layout = [AMArgumentBindingRange]()
    arguments.reserveCapacity(descriptorCount)

    for descriptorIndex in 0..<descriptorCount {
        let base = descriptorIndex * amArgumentDescriptorStride
        guard let dataType = MTLDataType(rawValue: descriptors[base]),
              let index = Int(exactly: descriptors[base + 1]),
              let arrayLength = Int(exactly: descriptors[base + 2]),
              let access = MTLBindingAccess(rawValue: descriptors[base + 3]),
              let textureType = MTLTextureType(rawValue: descriptors[base + 4]),
              let constantBlockAlignment = Int(exactly: descriptors[base + 5]),
              index >= 0,
              arrayLength >= 0,
              constantBlockAlignment >= 0
        else { return nil }

        let kind: AMArgumentBindingKind
        switch dataType {
        case .pointer:
            guard arrayLength == 0, constantBlockAlignment == 0 else { return nil }
            kind = .buffer
        case .texture:
            guard constantBlockAlignment == 0 else { return nil }
            kind = .texture
        case .sampler:
            guard access.rawValue == 0, constantBlockAlignment == 0 else { return nil }
            kind = .sampler
        default:
            guard amSupportsConstantArgumentDataType(dataType),
                  access.rawValue == 0,
                  constantBlockAlignment == 0
                    || constantBlockAlignment.nonzeroBitCount == 1
            else { return nil }
            kind = .constant
        }
        let occupiedBindingCount = max(1, arrayLength)
        let (last, overflow) = index.addingReportingOverflow(occupiedBindingCount - 1)
        guard !overflow else { return nil }
        layout.append(AMArgumentBindingRange(start: index, last: last, kind: kind))

        let descriptor = MTLArgumentDescriptor()
        descriptor.dataType = dataType
        descriptor.index = index
        descriptor.arrayLength = arrayLength
        descriptor.access = access
        descriptor.textureType = textureType
        descriptor.constantBlockAlignment = constantBlockAlignment
        arguments.append(descriptor)
    }

    layout.sort { $0.start < $1.start }
    for index in 1..<layout.count {
        guard layout[index].start > layout[index - 1].last else { return nil }
    }

    guard let encoder = device.makeArgumentEncoder(arguments: arguments) else {
        return nil
    }
    amRegisterArgumentEncoder(encoder, layout: layout)
    return am_retain(encoder as AnyObject)
}

@_cdecl("ametal_argument_encoder_encoded_length")
public func ametal_argument_encoder_encoded_length(_ handle: UnsafeMutableRawPointer?) -> Int {
    guard let encoder: MTLArgumentEncoder = am_borrow(handle) else { return 0 }
    return encoder.encodedLength
}

@_cdecl("ametal_argument_encoder_alignment")
public func ametal_argument_encoder_alignment(_ handle: UnsafeMutableRawPointer?) -> Int {
    guard let encoder: MTLArgumentEncoder = am_borrow(handle) else { return 0 }
    return encoder.alignment
}

@_cdecl("ametal_argument_encoder_set_argument_buffer")
public func ametal_argument_encoder_set_argument_buffer(
    _ handle: UnsafeMutableRawPointer?,
    _ bufferHandle: UnsafeMutableRawPointer?,
    _ offset: Int
) -> Bool {
    guard offset >= 0,
          let encoder: MTLArgumentEncoder = am_borrow(handle),
          let buffer: MTLBuffer = am_borrow(bufferHandle),
          buffer.storageMode == .shared || buffer.storageMode == .managed,
          encoder.alignment > 0,
          offset.isMultiple(of: encoder.alignment)
    else { return false }
    let (end, overflow) = offset.addingReportingOverflow(encoder.encodedLength)
    guard !overflow, end <= buffer.length else { return false }
    encoder.setArgumentBuffer(buffer, offset: offset)
    let state = amArgumentEncoderState(encoder)
    state.argumentBuffer = buffer
    state.argumentBufferOffset = offset
    return true
}

@_cdecl("ametal_argument_encoder_set_buffer")
public func ametal_argument_encoder_set_buffer(
    _ handle: UnsafeMutableRawPointer?,
    _ bufferHandle: UnsafeMutableRawPointer?,
    _ offset: Int,
    _ index: Int
) -> Bool {
    guard offset >= 0,
          let encoder: MTLArgumentEncoder = am_borrow(handle),
          let buffer: MTLBuffer = am_borrow(bufferHandle),
          offset <= buffer.length,
          amArgumentEncoderAccepts(encoder, kind: .buffer, index: index),
          amRetainArgumentResource(
              buffer as AnyObject,
              encoder: encoder,
              kind: .buffer,
              index: index
          )
    else { return false }
    encoder.setBuffer(buffer, offset: offset, index: index)
    return true
}

@_cdecl("ametal_argument_encoder_set_texture")
public func ametal_argument_encoder_set_texture(
    _ handle: UnsafeMutableRawPointer?,
    _ textureHandle: UnsafeMutableRawPointer?,
    _ index: Int
) -> Bool {
    guard let encoder: MTLArgumentEncoder = am_borrow(handle),
          let texture: MTLTexture = am_borrow(textureHandle),
          amArgumentEncoderAccepts(encoder, kind: .texture, index: index),
          amRetainArgumentResource(
              texture as AnyObject,
              encoder: encoder,
              kind: .texture,
              index: index
          )
    else { return false }
    encoder.setTexture(texture, index: index)
    return true
}

@_cdecl("ametal_argument_encoder_set_sampler_state")
public func ametal_argument_encoder_set_sampler_state(
    _ handle: UnsafeMutableRawPointer?,
    _ samplerHandle: UnsafeMutableRawPointer?,
    _ index: Int
) -> Bool {
    guard let encoder: MTLArgumentEncoder = am_borrow(handle),
          let sampler: MTLSamplerState = am_borrow(samplerHandle),
          amArgumentEncoderAccepts(encoder, kind: .sampler, index: index),
          amRetainArgumentResource(
              sampler as AnyObject,
              encoder: encoder,
              kind: .sampler,
              index: index
          )
    else { return false }
    encoder.setSamplerState(sampler, index: index)
    return true
}
