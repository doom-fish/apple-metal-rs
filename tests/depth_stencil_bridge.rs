mod common;

use apple_metal::{
    compare_function, load_action, pixel_format, primitive_type, resource_options,
    stencil_operation, storage_mode, store_action, texture_usage, CommandBufferError,
    DepthStencilDescriptor, DepthStencilState, MetalDevice, MetalTexture,
    RenderPassDepthAttachment, RenderPassStencilAttachment,
    RenderPipelineColorAttachmentDescriptor, RenderPipelineDescriptor, StencilDescriptor,
    TextureDescriptor,
};

const fn replacing_stencil() -> StencilDescriptor {
    let mut stencil = StencilDescriptor::new();
    stencil.stencil_compare_function = compare_function::ALWAYS;
    stencil.stencil_failure_operation = stencil_operation::REPLACE;
    stencil.depth_failure_operation = stencil_operation::KEEP;
    stencil.depth_stencil_pass_operation = stencil_operation::REPLACE;
    stencil.read_mask = 0xff;
    stencil.write_mask = 0xff;
    stencil
}

fn depth_and_stencil_state(device: &MetalDevice) -> DepthStencilState {
    let mut descriptor = DepthStencilDescriptor::new();
    descriptor.depth_compare_function = compare_function::LESS_EQUAL;
    descriptor.depth_write_enabled = true;
    descriptor.front_face_stencil = Some(replacing_stencil());
    descriptor.back_face_stencil = Some(replacing_stencil());
    descriptor.label = Some("depth-stencil-bridge".to_string());
    device
        .new_depth_stencil_state(&descriptor)
        .expect("depth stencil state")
}

fn private_target(device: &MetalDevice, format: usize) -> MetalTexture {
    let mut descriptor = TextureDescriptor::new_2d(4, 4, format);
    descriptor.storage_mode = storage_mode::PRIVATE;
    descriptor.usage = texture_usage::RENDER_TARGET;
    device
        .new_texture(descriptor)
        .expect("private render target")
}

#[test]
fn depth_stencil_state_renders_with_depth_and_stencil_attachments() {
    let device = common::device();
    let library = device
        .new_library_with_source(common::RENDER_SRC)
        .expect("render MSL");
    let vertex = library
        .new_function("fullscreen_vertex")
        .expect("vertex function");
    let fragment = library
        .new_function("solid_fragment")
        .expect("fragment function");
    let colors = [RenderPipelineColorAttachmentDescriptor::new(
        pixel_format::BGRA8UNORM,
    )];
    let mut pipeline_descriptor = RenderPipelineDescriptor::new(&vertex, Some(&fragment), &colors);
    pipeline_descriptor.depth_attachment_pixel_format = pixel_format::DEPTH32FLOAT_STENCIL8;
    pipeline_descriptor.stencil_attachment_pixel_format = pixel_format::DEPTH32FLOAT_STENCIL8;
    let pipeline = device
        .new_render_pipeline_state_with_descriptor(&pipeline_descriptor)
        .expect("depth-stencil pipeline");

    let state = depth_and_stencil_state(&device);
    assert_eq!(state.label().as_deref(), Some("depth-stencil-bridge"));
    assert!(state.tests_depth());
    assert!(state.tests_stencil());

    let queue = device.new_command_queue().expect("command queue");
    let color = device
        .new_texture(common::shared_render_target(4, 4))
        .expect("color target");
    let depth_stencil = private_target(&device, pixel_format::DEPTH32FLOAT_STENCIL8);
    let vertex_buffer = device
        .new_buffer(16, resource_options::STORAGE_MODE_SHARED)
        .expect("vertex buffer");
    let command_buffer = queue.new_command_buffer().expect("command buffer");
    let mut encoder = command_buffer
        .new_render_command_encoder(
            &color,
            load_action::CLEAR,
            store_action::STORE,
            [0.0, 0.0, 0.0, 0.0],
            Some(RenderPassDepthAttachment {
                texture: &depth_stencil,
                load_action: load_action::CLEAR,
                store_action: store_action::DONT_CARE,
                clear_depth: 1.0,
            }),
            Some(RenderPassStencilAttachment {
                texture: &depth_stencil,
                load_action: load_action::CLEAR,
                store_action: store_action::DONT_CARE,
                clear_stencil: 0,
            }),
        )
        .expect("render encoder with depth and stencil");
    encoder
        .set_render_pipeline_state(&pipeline)
        .expect("bind depth-stencil pipeline");
    encoder
        .set_depth_stencil_state(&state)
        .expect("bind depth stencil state");
    encoder
        .set_vertex_buffer(&vertex_buffer, 0, 0)
        .expect("bind vertex buffer");
    encoder
        .draw_primitives(primitive_type::TRIANGLE, 0, 3)
        .expect("draw triangle");
    encoder.end_encoding().expect("end render encoder");
    command_buffer.commit().expect("commit render");
    command_buffer
        .wait_until_completed()
        .expect("complete render");

    let mut rendered = vec![0_u8; 4 * 4 * 4];
    unsafe {
        color
            .read_bytes_2d(&mut rendered, 16, (0, 0), (4, 4), 0)
            .expect("read color target");
    }
    assert!(rendered.chunks_exact(4).all(|pixel| pixel[3] == 0xff));
}

#[test]
#[allow(clippy::too_many_lines)]
fn encoders_refuse_states_and_pipelines_their_pass_cannot_serve() {
    let device = common::device();
    let (library, vertex, fragment, pipeline) = common::compile_render(&device);
    drop(library);

    let mut depth_only = DepthStencilDescriptor::new();
    depth_only.depth_compare_function = compare_function::LESS;
    let depth_state = device
        .new_depth_stencil_state(&depth_only)
        .expect("depth-only state");
    assert!(depth_state.tests_depth());
    assert!(!depth_state.tests_stencil());

    let mut stencil_only = DepthStencilDescriptor::new();
    stencil_only.back_face_stencil = Some(replacing_stencil());
    let stencil_state = device
        .new_depth_stencil_state(&stencil_only)
        .expect("stencil-only state");
    assert!(!stencil_state.tests_depth());
    assert!(stencil_state.tests_stencil());

    let mut write_only = DepthStencilDescriptor::new();
    write_only.depth_write_enabled = true;
    assert!(device
        .new_depth_stencil_state(&write_only)
        .expect("depth-write state")
        .tests_depth());

    let mut untouched = DepthStencilDescriptor::new();
    untouched.front_face_stencil = Some(StencilDescriptor::new());
    let passthrough = device
        .new_depth_stencil_state(&untouched)
        .expect("pass-through state");
    assert!(!passthrough.tests_depth());
    assert!(!passthrough.tests_stencil());

    let colors = [RenderPipelineColorAttachmentDescriptor::new(
        pixel_format::BGRA8UNORM,
    )];
    let mut depth_pipeline_descriptor =
        RenderPipelineDescriptor::new(&vertex, Some(&fragment), &colors);
    depth_pipeline_descriptor.depth_attachment_pixel_format = pixel_format::DEPTH32FLOAT;
    let depth_pipeline = device
        .new_render_pipeline_state_with_descriptor(&depth_pipeline_descriptor)
        .expect("depth pipeline");
    let rgba_pipeline = device
        .new_render_pipeline_state(&vertex, &fragment, pixel_format::RGBA8UNORM, 1)
        .expect("RGBA pipeline");

    let queue = device.new_command_queue().expect("command queue");
    let color = device
        .new_texture(common::shared_render_target(4, 4))
        .expect("color target");
    let vertex_buffer = device
        .new_buffer(16, resource_options::STORAGE_MODE_SHARED)
        .expect("vertex buffer");
    let command_buffer = queue.new_command_buffer().expect("command buffer");
    let mut encoder = command_buffer
        .new_render_command_encoder(
            &color,
            load_action::CLEAR,
            store_action::STORE,
            [0.0, 0.0, 0.0, 1.0],
            None,
            None,
        )
        .expect("color-only render encoder");

    assert_eq!(
        encoder.set_depth_stencil_state(&depth_state),
        Err(CommandBufferError::MissingAttachment {
            attachment: "depth"
        })
    );
    assert_eq!(
        encoder.set_depth_stencil_state(&stencil_state),
        Err(CommandBufferError::MissingAttachment {
            attachment: "stencil"
        })
    );
    encoder
        .set_depth_stencil_state(&passthrough)
        .expect("pass-through state on a color-only pass");
    assert_eq!(
        encoder.draw_primitives(primitive_type::TRIANGLE, 0, 3),
        Err(CommandBufferError::MissingPipelineState)
    );
    assert_eq!(
        encoder.set_render_pipeline_state(&depth_pipeline),
        Err(CommandBufferError::IncompatiblePipelineState)
    );
    assert_eq!(
        encoder.set_render_pipeline_state(&rgba_pipeline),
        Err(CommandBufferError::IncompatiblePipelineState)
    );
    encoder
        .set_render_pipeline_state(&pipeline)
        .expect("matching pipeline");
    encoder
        .set_vertex_buffer(&vertex_buffer, 0, 0)
        .expect("bind vertex buffer");
    assert_eq!(
        encoder.draw_primitives(99, 0, 3),
        Err(CommandBufferError::InvalidPrimitiveType { primitive_type: 99 })
    );
    encoder
        .draw_primitives(primitive_type::TRIANGLE, 0, 3)
        .expect("draw triangle");
    encoder.end_encoding().expect("end render encoder");
    command_buffer.commit().expect("commit render");
    command_buffer
        .wait_until_completed()
        .expect("complete render");
}

#[test]
fn render_pass_attachments_are_checked() {
    let device = common::device();
    let queue = device.new_command_queue().expect("command queue");
    let color = device
        .new_texture(common::shared_render_target(4, 4))
        .expect("color target");
    let sampled_only = device
        .new_texture(TextureDescriptor::new_2d(4, 4, pixel_format::BGRA8UNORM))
        .expect("sampled texture");
    let depth = private_target(&device, pixel_format::DEPTH32FLOAT);
    let combined = private_target(&device, pixel_format::DEPTH32FLOAT_STENCIL8);
    let other_combined = private_target(&device, pixel_format::DEPTH32FLOAT_STENCIL8);
    let color_as_depth = private_target(&device, pixel_format::BGRA8UNORM);
    let depth_attachment = |texture| RenderPassDepthAttachment {
        texture,
        load_action: load_action::CLEAR,
        store_action: store_action::DONT_CARE,
        clear_depth: 1.0,
    };
    let stencil_attachment = |texture| RenderPassStencilAttachment {
        texture,
        load_action: load_action::CLEAR,
        store_action: store_action::DONT_CARE,
        clear_stencil: 0,
    };
    let command_buffer = queue.new_command_buffer().expect("command buffer");
    let attempt = |color: &MetalTexture,
                   store: usize,
                   depth: Option<RenderPassDepthAttachment<'_>>,
                   stencil: Option<RenderPassStencilAttachment<'_>>| {
        command_buffer
            .new_render_command_encoder(color, load_action::CLEAR, store, [0.0; 4], depth, stencil)
            .err()
    };

    assert_eq!(
        attempt(&sampled_only, store_action::STORE, None, None),
        Some(CommandBufferError::InvalidAttachment {
            attachment: "color"
        })
    );
    assert_eq!(
        attempt(&color, store_action::MULTISAMPLE_RESOLVE, None, None),
        Some(CommandBufferError::InvalidAttachment {
            attachment: "color"
        })
    );
    assert_eq!(
        attempt(
            &color,
            store_action::STORE,
            Some(depth_attachment(&color_as_depth)),
            None
        ),
        Some(CommandBufferError::InvalidAttachment {
            attachment: "depth"
        })
    );
    assert_eq!(
        attempt(
            &color,
            store_action::STORE,
            None,
            Some(stencil_attachment(&depth))
        ),
        Some(CommandBufferError::InvalidAttachment {
            attachment: "stencil"
        })
    );
    assert_eq!(
        attempt(
            &color,
            store_action::STORE,
            Some(depth_attachment(&combined)),
            Some(stencil_attachment(&other_combined))
        ),
        Some(CommandBufferError::InvalidAttachment {
            attachment: "stencil"
        })
    );
    let mut far = depth_attachment(&depth);
    far.clear_depth = 2.0;
    assert_eq!(
        attempt(&color, store_action::STORE, Some(far), None),
        Some(CommandBufferError::InvalidAttachment {
            attachment: "depth"
        })
    );
    let encoder = command_buffer
        .new_render_command_encoder(
            &color,
            load_action::CLEAR,
            store_action::STORE,
            [0.0; 4],
            Some(depth_attachment(&combined)),
            Some(stencil_attachment(&combined)),
        )
        .expect("combined depth-stencil attachment");
    encoder.end_encoding().expect("end render encoder");
}

#[test]
fn state_descriptors_with_unknown_values_are_refused() {
    let device = common::device();
    let mut unknown_compare = DepthStencilDescriptor::new();
    unknown_compare.depth_compare_function = 99;
    assert!(device.new_depth_stencil_state(&unknown_compare).is_none());
    let mut stencil = StencilDescriptor::new();
    stencil.depth_failure_operation = 99;
    let mut unknown_operation = DepthStencilDescriptor::new();
    unknown_operation.front_face_stencil = Some(stencil);
    assert!(device.new_depth_stencil_state(&unknown_operation).is_none());
}
