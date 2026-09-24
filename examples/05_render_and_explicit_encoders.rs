#![allow(clippy::too_many_lines)]

#[path = "common/mod.rs"]
mod common;

use apple_metal::{
    load_action, pixel_format, primitive_type, resource_options, store_action, MetalDevice,
    TextureDescriptor,
};

fn main() {
    let device = MetalDevice::system_default().expect("Metal device available");
    println!(
        "device: {} (registry id {})",
        device.name(),
        device.registry_id()
    );

    let queue = device.new_command_queue().expect("command queue");
    let status_buffer = unsafe {
        queue
            .new_command_buffer_with_unretained_references()
            .expect("scratch command buffer")
    };
    println!("scratch command buffer status={}", status_buffer.status());

    let src = device
        .new_buffer(64, resource_options::STORAGE_MODE_SHARED)
        .expect("source buffer");
    let dst = device
        .new_buffer(64, resource_options::STORAGE_MODE_SHARED)
        .expect("destination buffer");
    let blit_cb = queue.new_command_buffer().expect("blit command buffer");
    let mut blit = blit_cb
        .new_blit_command_encoder()
        .expect("blit command encoder");
    blit.fill_buffer(&src, 0..64, b'Z').expect("fill source");
    blit.copy_buffer(&src, 0, &dst, 0, 64)
        .expect("copy buffers");
    blit.end_encoding().expect("end blit encoder");
    blit_cb.commit().expect("commit blit");
    blit_cb.wait_until_completed().expect("complete blit");
    let copied = {
        let mapping = unsafe { dst.map_read().expect("map destination") };
        let copied = mapping[..8].to_vec();
        drop(mapping);
        copied
    };
    println!("blit copied bytes: {copied:?}");

    let library = device
        .new_library_with_source(common::COMPUTE_SRC)
        .expect("compile compute library");
    let increment = library
        .new_function("increment")
        .expect("increment function");
    let pipeline = device
        .new_compute_pipeline_state(&increment)
        .expect("compute pipeline");

    let buffer = device
        .new_buffer(16, resource_options::STORAGE_MODE_SHARED)
        .expect("compute buffer");
    common::write_u32_words(&buffer, &[10, 20, 30, 40]);
    let compute_cb = queue.new_command_buffer().expect("compute command buffer");
    let mut compute = compute_cb
        .new_compute_command_encoder()
        .expect("compute command encoder");
    compute
        .set_compute_pipeline_state(&pipeline)
        .expect("bind compute pipeline");
    compute
        .set_buffer(&buffer, 0, 0)
        .expect("bind compute buffer");
    compute
        .dispatch_threads((4, 1, 1), (1, 1, 1))
        .expect("dispatch compute");
    compute.end_encoding().expect("end compute encoder");
    compute_cb.commit().expect("commit compute");
    compute_cb.wait_until_completed().expect("complete compute");
    println!("compute output: {:?}", common::read_u32_words(&buffer, 4));

    let render_library = device
        .new_library_with_source(common::RENDER_SRC)
        .expect("compile render library");
    let vertex = render_library
        .new_function("fullscreen_vertex")
        .expect("vertex function");
    let fragment = render_library
        .new_function("solid_fragment")
        .expect("fragment function");
    let render_pipeline = device
        .new_render_pipeline_state(&vertex, &fragment, pixel_format::BGRA8UNORM, 1)
        .expect("render pipeline");
    println!("render pipeline label: {:?}", render_pipeline.label());

    let render_target = device
        .new_texture(common::shared_render_target(4, 4))
        .expect("render target");
    let vertex_buffer = device
        .new_buffer(16, resource_options::STORAGE_MODE_SHARED)
        .expect("vertex buffer");
    let render_cb = queue.new_command_buffer().expect("render command buffer");
    let mut render = render_cb
        .new_render_command_encoder(
            &render_target,
            load_action::CLEAR,
            store_action::STORE,
            [0.0, 0.0, 0.0, 1.0],
            None,
            None,
        )
        .expect("render command encoder");
    render
        .set_render_pipeline_state(&render_pipeline)
        .expect("bind render pipeline");
    render
        .set_vertex_buffer(&vertex_buffer, 0, 0)
        .expect("bind vertex buffer");
    render
        .draw_primitives(primitive_type::TRIANGLE, 0, 3)
        .expect("draw triangle");
    render.end_encoding().expect("end render encoder");
    render_cb.commit().expect("commit render");
    render_cb.wait_until_completed().expect("complete render");

    let mut rendered = vec![0_u8; 4 * 4 * 4];
    unsafe {
        render_target
            .read_bytes_2d(&mut rendered, 16, (0, 0), (4, 4), 0)
            .expect("read render target");
    }
    println!("first rendered pixel: {:?}", &rendered[..4]);

    let shared_texture = device
        .new_texture(TextureDescriptor::new_2d(4, 4, pixel_format::BGRA8UNORM))
        .expect("shared texture");
    let upload = vec![0x22_u8; 4 * 4 * 4];
    unsafe {
        shared_texture
            .replace_region_2d(&upload, 16, (0, 0), (4, 4), 0)
            .expect("upload texture");
    }
    let mut download = vec![0_u8; upload.len()];
    unsafe {
        shared_texture
            .read_bytes_2d(&mut download, 16, (0, 0), (4, 4), 0)
            .expect("read texture");
    }
    let view = shared_texture
        .new_view(pixel_format::BGRA8UNORM)
        .expect("texture view");
    println!(
        "texture {}x{} usage={} storage_mode={} view_width={}",
        shared_texture.width(),
        shared_texture.height(),
        shared_texture.usage(),
        shared_texture.storage_mode(),
        view.width(),
    );
}
