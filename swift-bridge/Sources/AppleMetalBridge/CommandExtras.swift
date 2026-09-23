import Foundation
import Metal

@_cdecl("ametal_command_queue_new_command_buffer_with_unretained_references")
public func ametal_command_queue_new_command_buffer_with_unretained_references(
    _ handle: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let queue: MTLCommandQueue = am_borrow(handle),
          let commandBuffer = queue.makeCommandBufferWithUnretainedReferences()
    else { return nil }
    return am_retain(commandBuffer as AnyObject)
}

@_cdecl("ametal_command_buffer_enqueue")
public func ametal_command_buffer_enqueue(_ handle: UnsafeMutableRawPointer?) {
    guard let commandBuffer: MTLCommandBuffer = am_borrow(handle) else { return }
    commandBuffer.enqueue()
}

@_cdecl("ametal_command_buffer_wait_until_scheduled")
public func ametal_command_buffer_wait_until_scheduled(_ handle: UnsafeMutableRawPointer?) {
    guard let commandBuffer: MTLCommandBuffer = am_borrow(handle) else { return }
    commandBuffer.waitUntilScheduled()
}

@_cdecl("ametal_command_buffer_status")
public func ametal_command_buffer_status(_ handle: UnsafeMutableRawPointer?) -> Int {
    guard let commandBuffer: MTLCommandBuffer = am_borrow(handle) else { return 0 }
    return Int(commandBuffer.status.rawValue)
}

@_cdecl("ametal_command_buffer_error_message")
public func ametal_command_buffer_error_message(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let commandBuffer: MTLCommandBuffer = am_borrow(handle) else { return nil }
    return am_copy_string(commandBuffer.error?.localizedDescription)
}

@_cdecl("ametal_command_buffer_add_handler")
public func ametal_command_buffer_add_handler(
    _ handle: UnsafeMutableRawPointer?,
    _ completed: Bool,
    _ context: UnsafeMutableRawPointer?,
    _ callback: (@convention(c) (UnsafeMutableRawPointer?, Int, UnsafePointer<CChar>?) -> Void)?,
    _ release: (@convention(c) (UnsafeMutableRawPointer?) -> Void)?
) -> Bool {
    guard let owner = AMCallbackContextOwner(context, release) else { return false }
    guard let callback,
          let commandBuffer: MTLCommandBuffer = am_borrow(handle),
          commandBuffer.status == .notEnqueued || commandBuffer.status == .enqueued
    else { return false }
    let handler: MTLCommandBufferHandler = { buffer in
        let message = buffer.error.flatMap { strdup($0.localizedDescription) }
        callback(owner.context, Int(bitPattern: buffer.status.rawValue), message)
        free(message)
    }
    if completed {
        commandBuffer.addCompletedHandler(handler)
    } else {
        commandBuffer.addScheduledHandler(handler)
    }
    return true
}

@_cdecl("ametal_command_buffer_new_blit_command_encoder")
public func ametal_command_buffer_new_blit_command_encoder(
    _ handle: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let commandBuffer: MTLCommandBuffer = am_borrow(handle),
          let encoder = commandBuffer.makeBlitCommandEncoder()
    else { return nil }
    return am_retain(encoder as AnyObject)
}

@_cdecl("ametal_command_buffer_new_compute_command_encoder")
public func ametal_command_buffer_new_compute_command_encoder(
    _ handle: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let commandBuffer: MTLCommandBuffer = am_borrow(handle),
          let encoder = commandBuffer.makeComputeCommandEncoder()
    else { return nil }
    return am_retain(encoder as AnyObject)
}

@_cdecl("ametal_command_buffer_new_render_command_encoder")
public func ametal_command_buffer_new_render_command_encoder(
    _ handle: UnsafeMutableRawPointer?,
    _ textureHandle: UnsafeMutableRawPointer?,
    _ loadAction: Int,
    _ storeAction: Int,
    _ clearR: Double,
    _ clearG: Double,
    _ clearB: Double,
    _ clearA: Double
) -> UnsafeMutableRawPointer? {
    guard let commandBuffer: MTLCommandBuffer = am_borrow(handle),
          let texture: MTLTexture = am_borrow(textureHandle)
    else { return nil }

    let descriptor = MTLRenderPassDescriptor()
    let attachment = descriptor.colorAttachments[0]!
    attachment.texture = texture
    attachment.loadAction = MTLLoadAction(rawValue: UInt(loadAction)) ?? .dontCare
    attachment.storeAction = MTLStoreAction(rawValue: UInt(storeAction)) ?? .dontCare
    attachment.clearColor = MTLClearColor(red: clearR, green: clearG, blue: clearB, alpha: clearA)

    guard let encoder = commandBuffer.makeRenderCommandEncoder(descriptor: descriptor) else {
        return nil
    }
    return am_retain(encoder as AnyObject)
}

@_cdecl("ametal_command_buffer_encode_wait_for_event")
public func ametal_command_buffer_encode_wait_for_event(
    _ handle: UnsafeMutableRawPointer?,
    _ eventHandle: UnsafeMutableRawPointer?,
    _ value: UInt64
) {
    guard let commandBuffer: MTLCommandBuffer = am_borrow(handle),
          let event: MTLEvent = am_borrow(eventHandle)
    else { return }
    commandBuffer.encodeWaitForEvent(event, value: value)
}

@_cdecl("ametal_command_buffer_encode_signal_event")
public func ametal_command_buffer_encode_signal_event(
    _ handle: UnsafeMutableRawPointer?,
    _ eventHandle: UnsafeMutableRawPointer?,
    _ value: UInt64
) {
    guard let commandBuffer: MTLCommandBuffer = am_borrow(handle),
          let event: MTLEvent = am_borrow(eventHandle)
    else { return }
    commandBuffer.encodeSignalEvent(event, value: value)
}

@_cdecl("ametal_command_encoder_end_encoding")
public func ametal_command_encoder_end_encoding(_ handle: UnsafeMutableRawPointer?) {
    guard let encoder: MTLCommandEncoder = am_borrow(handle) else { return }
    encoder.endEncoding()
}

@_cdecl("ametal_blit_command_encoder_copy_buffer")
public func ametal_blit_command_encoder_copy_buffer(
    _ handle: UnsafeMutableRawPointer?,
    _ srcHandle: UnsafeMutableRawPointer?,
    _ srcOffset: Int,
    _ dstHandle: UnsafeMutableRawPointer?,
    _ dstOffset: Int,
    _ size: Int
) -> Bool {
    guard let encoder: MTLBlitCommandEncoder = am_borrow(handle),
          let src: MTLBuffer = am_borrow(srcHandle),
          let dst: MTLBuffer = am_borrow(dstHandle)
    else { return false }
    encoder.copy(from: src, sourceOffset: srcOffset, to: dst, destinationOffset: dstOffset, size: size)
    return true
}

@_cdecl("ametal_blit_command_encoder_fill_buffer")
public func ametal_blit_command_encoder_fill_buffer(
    _ handle: UnsafeMutableRawPointer?,
    _ bufferHandle: UnsafeMutableRawPointer?,
    _ location: Int,
    _ length: Int,
    _ value: UInt8
) -> Bool {
    guard let encoder: MTLBlitCommandEncoder = am_borrow(handle),
          let buffer: MTLBuffer = am_borrow(bufferHandle)
    else { return false }
    encoder.__fill(buffer, range: NSRange(location: location, length: length), value: value)
    return true
}

@_cdecl("ametal_blit_command_encoder_sample_counters")
public func ametal_blit_command_encoder_sample_counters(
    _ handle: UnsafeMutableRawPointer?,
    _ sampleBufferHandle: UnsafeMutableRawPointer?,
    _ sampleIndex: Int,
    _ barrier: Bool
) -> Bool {
    guard #available(macOS 10.15, *),
          let encoder: MTLBlitCommandEncoder = am_borrow(handle),
          let sampleBuffer: MTLCounterSampleBuffer = am_borrow(sampleBufferHandle)
    else { return false }
    encoder.sampleCounters(sampleBuffer: sampleBuffer, sampleIndex: sampleIndex, barrier: barrier)
    return true
}

@_cdecl("ametal_blit_command_encoder_update_fence")
public func ametal_blit_command_encoder_update_fence(
    _ handle: UnsafeMutableRawPointer?,
    _ fenceHandle: UnsafeMutableRawPointer?
) {
    guard let encoder: MTLBlitCommandEncoder = am_borrow(handle),
          let fence: MTLFence = am_borrow(fenceHandle)
    else { return }
    encoder.updateFence(fence)
}

@_cdecl("ametal_blit_command_encoder_wait_for_fence")
public func ametal_blit_command_encoder_wait_for_fence(
    _ handle: UnsafeMutableRawPointer?,
    _ fenceHandle: UnsafeMutableRawPointer?
) {
    guard let encoder: MTLBlitCommandEncoder = am_borrow(handle),
          let fence: MTLFence = am_borrow(fenceHandle)
    else { return }
    encoder.waitForFence(fence)
}

@_cdecl("ametal_blit_command_encoder_synchronize_resource")
public func ametal_blit_command_encoder_synchronize_resource(
    _ handle: UnsafeMutableRawPointer?,
    _ resourceHandle: UnsafeMutableRawPointer?
) -> Bool {
    guard let encoder: MTLBlitCommandEncoder = am_borrow(handle),
          let resource: MTLResource = am_borrow(resourceHandle),
          resource.storageMode == .managed
    else { return false }
    encoder.synchronize(resource: resource)
    return true
}

@_cdecl("ametal_compute_command_encoder_set_pipeline_state")
public func ametal_compute_command_encoder_set_pipeline_state(
    _ handle: UnsafeMutableRawPointer?,
    _ pipelineHandle: UnsafeMutableRawPointer?
) {
    guard let encoder: MTLComputeCommandEncoder = am_borrow(handle),
          let pipeline: MTLComputePipelineState = am_borrow(pipelineHandle)
    else { return }
    encoder.setComputePipelineState(pipeline)
}

@_cdecl("ametal_compute_command_encoder_set_buffer")
public func ametal_compute_command_encoder_set_buffer(
    _ handle: UnsafeMutableRawPointer?,
    _ bufferHandle: UnsafeMutableRawPointer?,
    _ offset: Int,
    _ index: Int
) {
    guard let encoder: MTLComputeCommandEncoder = am_borrow(handle),
          let buffer: MTLBuffer = am_borrow(bufferHandle)
    else { return }
    encoder.setBuffer(buffer, offset: offset, index: index)
    amTrackArgumentBuffer(encoder, buffer: buffer)
}

@_cdecl("ametal_compute_command_encoder_set_texture")
public func ametal_compute_command_encoder_set_texture(
    _ handle: UnsafeMutableRawPointer?,
    _ textureHandle: UnsafeMutableRawPointer?,
    _ index: Int
) {
    guard let encoder: MTLComputeCommandEncoder = am_borrow(handle),
          let texture: MTLTexture = am_borrow(textureHandle)
    else { return }
    encoder.setTexture(texture, index: index)
}

@_cdecl("ametal_compute_command_encoder_set_visible_function_table")
public func ametal_compute_command_encoder_set_visible_function_table(
    _ handle: UnsafeMutableRawPointer?,
    _ tableHandle: UnsafeMutableRawPointer?,
    _ index: Int
) {
    guard #available(macOS 11.0, *),
          let encoder: MTLComputeCommandEncoder = am_borrow(handle),
          let table: MTLVisibleFunctionTable = am_borrow(tableHandle)
    else { return }
    encoder.setVisibleFunctionTable(table, bufferIndex: index)
}

@_cdecl("ametal_compute_command_encoder_set_intersection_function_table")
public func ametal_compute_command_encoder_set_intersection_function_table(
    _ handle: UnsafeMutableRawPointer?,
    _ tableHandle: UnsafeMutableRawPointer?,
    _ index: Int
) {
    guard #available(macOS 11.0, *),
          let encoder: MTLComputeCommandEncoder = am_borrow(handle),
          let table: MTLIntersectionFunctionTable = am_borrow(tableHandle)
    else { return }
    encoder.setIntersectionFunctionTable(table, bufferIndex: index)
}

@_cdecl("ametal_compute_command_encoder_set_acceleration_structure")
public func ametal_compute_command_encoder_set_acceleration_structure(
    _ handle: UnsafeMutableRawPointer?,
    _ accelerationStructureHandle: UnsafeMutableRawPointer?,
    _ index: Int
) {
    guard #available(macOS 11.0, *),
          let encoder: MTLComputeCommandEncoder = am_borrow(handle),
          let accelerationStructure: MTLAccelerationStructure = am_borrow(accelerationStructureHandle)
    else { return }
    encoder.setAccelerationStructure(accelerationStructure, bufferIndex: index)
}

@_cdecl("ametal_compute_command_encoder_dispatch_threadgroups")
public func ametal_compute_command_encoder_dispatch_threadgroups(
    _ handle: UnsafeMutableRawPointer?,
    _ tgW: Int,
    _ tgH: Int,
    _ tgD: Int,
    _ threadsW: Int,
    _ threadsH: Int,
    _ threadsD: Int
) {
    guard let encoder: MTLComputeCommandEncoder = am_borrow(handle) else { return }
    amDeclareArgumentBufferResources(encoder)
    encoder.dispatchThreadgroups(
        MTLSize(width: tgW, height: tgH, depth: tgD),
        threadsPerThreadgroup: MTLSize(width: threadsW, height: threadsH, depth: threadsD)
    )
}

@_cdecl("ametal_compute_command_encoder_dispatch_threads")
public func ametal_compute_command_encoder_dispatch_threads(
    _ handle: UnsafeMutableRawPointer?,
    _ gridW: Int,
    _ gridH: Int,
    _ gridD: Int,
    _ threadsW: Int,
    _ threadsH: Int,
    _ threadsD: Int
) {
    guard let encoder: MTLComputeCommandEncoder = am_borrow(handle) else { return }
    amDeclareArgumentBufferResources(encoder)
    encoder.dispatchThreads(
        MTLSize(width: gridW, height: gridH, depth: gridD),
        threadsPerThreadgroup: MTLSize(width: threadsW, height: threadsH, depth: threadsD)
    )
}

@_cdecl("ametal_compute_command_encoder_update_fence")
public func ametal_compute_command_encoder_update_fence(
    _ handle: UnsafeMutableRawPointer?,
    _ fenceHandle: UnsafeMutableRawPointer?
) {
    guard let encoder: MTLComputeCommandEncoder = am_borrow(handle),
          let fence: MTLFence = am_borrow(fenceHandle)
    else { return }
    encoder.updateFence(fence)
}

@_cdecl("ametal_compute_command_encoder_wait_for_fence")
public func ametal_compute_command_encoder_wait_for_fence(
    _ handle: UnsafeMutableRawPointer?,
    _ fenceHandle: UnsafeMutableRawPointer?
) {
    guard let encoder: MTLComputeCommandEncoder = am_borrow(handle),
          let fence: MTLFence = am_borrow(fenceHandle)
    else { return }
    encoder.waitForFence(fence)
}

@_cdecl("ametal_render_command_encoder_set_render_pipeline_state")
public func ametal_render_command_encoder_set_render_pipeline_state(
    _ handle: UnsafeMutableRawPointer?,
    _ pipelineHandle: UnsafeMutableRawPointer?
) {
    guard let encoder: MTLRenderCommandEncoder = am_borrow(handle),
          let pipeline: MTLRenderPipelineState = am_borrow(pipelineHandle)
    else { return }
    encoder.setRenderPipelineState(pipeline)
}

@_cdecl("ametal_render_command_encoder_set_vertex_buffer")
public func ametal_render_command_encoder_set_vertex_buffer(
    _ handle: UnsafeMutableRawPointer?,
    _ bufferHandle: UnsafeMutableRawPointer?,
    _ offset: Int,
    _ index: Int
) {
    guard let encoder: MTLRenderCommandEncoder = am_borrow(handle),
          let buffer: MTLBuffer = am_borrow(bufferHandle)
    else { return }
    encoder.setVertexBuffer(buffer, offset: offset, index: index)
    amTrackArgumentBuffer(encoder, buffer: buffer)
}

@_cdecl("ametal_render_command_encoder_draw_primitives")
public func ametal_render_command_encoder_draw_primitives(
    _ handle: UnsafeMutableRawPointer?,
    _ primitiveType: Int,
    _ vertexStart: Int,
    _ vertexCount: Int
) {
    guard let encoder: MTLRenderCommandEncoder = am_borrow(handle) else { return }
    amDeclareArgumentBufferResources(encoder)
    encoder.drawPrimitives(
        type: MTLPrimitiveType(rawValue: UInt(primitiveType)) ?? .point,
        vertexStart: vertexStart,
        vertexCount: vertexCount
    )
}

@_cdecl("ametal_render_command_encoder_update_fence")
public func ametal_render_command_encoder_update_fence(
    _ handle: UnsafeMutableRawPointer?,
    _ fenceHandle: UnsafeMutableRawPointer?
) {
    guard let encoder: MTLRenderCommandEncoder = am_borrow(handle),
          let fence: MTLFence = am_borrow(fenceHandle)
    else { return }
    if #available(macOS 10.13, *) {
        encoder.updateFence(fence, after: .fragment)
    }
}

@_cdecl("ametal_render_command_encoder_wait_for_fence")
public func ametal_render_command_encoder_wait_for_fence(
    _ handle: UnsafeMutableRawPointer?,
    _ fenceHandle: UnsafeMutableRawPointer?
) {
    guard let encoder: MTLRenderCommandEncoder = am_borrow(handle),
          let fence: MTLFence = am_borrow(fenceHandle)
    else { return }
    if #available(macOS 10.13, *) {
        encoder.waitForFence(fence, before: .vertex)
    }
}
