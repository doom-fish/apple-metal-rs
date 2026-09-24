import Foundation
import Metal

@_cdecl("ametal_device_new_render_pipeline_state")
public func ametal_device_new_render_pipeline_state(
    _ deviceHandle: UnsafeMutableRawPointer?,
    _ vertexHandle: UnsafeMutableRawPointer?,
    _ fragmentHandle: UnsafeMutableRawPointer?,
    _ colorPixelFormat: Int,
    _ sampleCount: Int,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let device: MTLDevice = am_borrow(deviceHandle),
          let vertex: MTLFunction = am_borrow(vertexHandle),
          let fragment: MTLFunction = am_borrow(fragmentHandle)
    else { return nil }

    let descriptor = MTLRenderPipelineDescriptor()
    descriptor.vertexFunction = vertex
    descriptor.fragmentFunction = fragment
    descriptor.colorAttachments[0].pixelFormat = UInt(exactly: colorPixelFormat).flatMap(MTLPixelFormat.init(rawValue:)) ?? .invalid
    descriptor.sampleCount = sampleCount
    guard amDeviceSupportsPixelFormat(device, descriptor.colorAttachments[0].pixelFormat) else {
        am_store_error_message(outErrorMessage, "the device does not support the color pixel format")
        return nil
    }

    do {
        let pipeline = try device.makeRenderPipelineState(descriptor: descriptor)
        return am_retain(pipeline as AnyObject)
    } catch {
        am_store_error(outErrorMessage, error)
        return nil
    }
}
