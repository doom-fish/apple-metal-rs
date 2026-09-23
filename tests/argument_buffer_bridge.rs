mod common;

use apple_metal::{
    argument_buffers_tier, binding_access, pixel_format, resource_options, texture_type,
    ArgumentDescriptor, SamplerDescriptor, TextureDescriptor,
};

#[test]
#[allow(clippy::too_many_lines)]
fn argument_encoders_can_encode_function_and_descriptor_layouts() {
    let device = common::device();
    let (_library, _increment_fn, args_fn, _increment_pipeline) = common::compile_compute(&device);
    let args_pipeline = device
        .new_compute_pipeline_state(&args_fn)
        .expect("argument-buffer compute pipeline");

    let mut function_encoder = args_fn
        .new_argument_encoder(0)
        .expect("function argument encoder");
    assert!(function_encoder.encoded_length() > 0);
    assert!(function_encoder.alignment() > 0);

    let storage_buffer = device
        .new_buffer(
            4 * core::mem::size_of::<u32>(),
            resource_options::STORAGE_MODE_SHARED,
        )
        .expect("storage buffer");
    common::write_u32_words(&storage_buffer, &[0, 0, 0, 0]);

    let texture = device
        .new_texture(TextureDescriptor::new_2d(4, 4, pixel_format::BGRA8UNORM))
        .expect("argument texture");
    let upload = vec![0x33_u8; 4 * 4 * 4];
    unsafe {
        texture
            .replace_region_2d(&upload, 16, (0, 0), (4, 4), 0)
            .expect("upload argument texture");
    }

    let function_argument_buffer = device
        .new_buffer(
            function_encoder.encoded_length(),
            resource_options::STORAGE_MODE_SHARED,
        )
        .expect("function argument buffer");
    unsafe {
        let mut binding = function_encoder
            .bind_argument_buffer(&function_argument_buffer, 0)
            .expect("bind function argument buffer");
        binding
            .set_buffer_unchecked(&storage_buffer, 0, 0)
            .expect("bind storage buffer");
        binding
            .set_texture_unchecked(&texture, 1)
            .expect("bind texture");
    }

    let queue = device.new_command_queue().expect("command queue");
    let compute_command_buffer = queue.new_command_buffer().expect("compute command buffer");
    let mut compute_encoder = compute_command_buffer
        .new_compute_command_encoder()
        .expect("compute encoder");
    compute_encoder
        .set_compute_pipeline_state(&args_pipeline)
        .expect("bind argument pipeline");
    compute_encoder
        .set_buffer(&function_argument_buffer, 0, 0)
        .expect("bind argument buffer");
    compute_encoder
        .dispatch_threads((1, 1, 1), (1, 1, 1))
        .expect("dispatch argument kernel");
    compute_encoder.end_encoding().expect("end compute encoder");
    compute_command_buffer.commit().expect("commit compute");
    compute_command_buffer
        .wait_until_completed()
        .expect("complete compute");
    assert_eq!(common::read_u32_words(&storage_buffer, 1), vec![7]);
    drop(function_encoder);
    drop(function_argument_buffer);
    common::write_u32_words(&storage_buffer, &[0, 0, 0, 0]);

    let mut descriptor_encoder = device
        .new_argument_encoder_with_descriptors(&[
            ArgumentDescriptor::buffer(0, binding_access::READ_WRITE),
            ArgumentDescriptor::texture(1, texture_type::TYPE_2D, binding_access::READ_ONLY),
            ArgumentDescriptor::sampler(2),
        ])
        .expect("descriptor argument encoder");
    assert!(descriptor_encoder.encoded_length() > 0);
    assert!(descriptor_encoder.alignment() > 0);

    let descriptor_argument_buffer = device
        .new_buffer(
            descriptor_encoder.encoded_length(),
            resource_options::STORAGE_MODE_SHARED,
        )
        .expect("descriptor argument buffer");
    let sampler = device
        .new_sampler_state(&SamplerDescriptor::new())
        .expect("descriptor sampler state");
    unsafe {
        let mut binding = descriptor_encoder
            .bind_argument_buffer(&descriptor_argument_buffer, 0)
            .expect("bind descriptor argument buffer");
        binding
            .set_buffer(&storage_buffer, 0, 0)
            .expect("bind descriptor storage buffer");
        binding
            .set_texture(&texture, 1)
            .expect("bind descriptor texture");
        binding
            .set_sampler_state(&sampler, 2)
            .expect("bind descriptor sampler");
    }

    let retained_storage_pointer = storage_buffer.as_ptr();
    drop(descriptor_encoder);
    drop(storage_buffer);
    drop(texture);
    drop(sampler);

    let retained_command_buffer = queue
        .new_command_buffer()
        .expect("retained-resource command buffer");
    let mut retained_encoder = retained_command_buffer
        .new_compute_command_encoder()
        .expect("retained-resource encoder");
    retained_encoder
        .set_compute_pipeline_state(&args_pipeline)
        .expect("bind retained-resource pipeline");
    retained_encoder
        .set_buffer(&descriptor_argument_buffer, 0, 0)
        .expect("bind retained argument buffer");
    retained_encoder
        .dispatch_threads((1, 1, 1), (1, 1, 1))
        .expect("dispatch retained argument buffer");
    retained_encoder
        .end_encoding()
        .expect("end retained-resource encoder");
    retained_command_buffer
        .commit()
        .expect("commit retained-resource command");
    retained_command_buffer
        .wait_until_completed()
        .expect("complete retained-resource command");
    let retained_value = unsafe {
        apple_metal::ffi::ametal_buffer_contents(retained_storage_pointer)
            .cast::<u32>()
            .read()
    };
    assert_eq!(retained_value, 7);

    assert!(device.argument_buffers_support() <= argument_buffers_tier::TIER2);
}
