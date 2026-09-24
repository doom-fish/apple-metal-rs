mod common;

use apple_metal::{
    binding_access, pixel_format, resource_options, storage_mode, texture_type, ArgumentDescriptor,
    ArgumentEncoderError, CommandBufferError, MetalBufferAccessError, TextureDescriptor,
    TextureTransferError,
};

#[test]
fn private_buffers_reject_cpu_access_and_offer_staging() {
    let device = common::device();
    let private = device
        .new_buffer(64, resource_options::STORAGE_MODE_PRIVATE)
        .expect("private buffer");

    assert!(!private.is_cpu_accessible());
    assert!(matches!(
        unsafe { private.map_write() },
        Err(MetalBufferAccessError::CpuInaccessibleStorage {
            storage_mode: storage_mode::PRIVATE
        })
    ));
    assert!(matches!(
        unsafe { private.write_bytes(0, b"private") },
        Err(MetalBufferAccessError::CpuInaccessibleStorage {
            storage_mode: storage_mode::PRIVATE
        })
    ));

    let staging = private.new_staging_buffer().expect("staging buffer");
    assert_eq!(staging.length(), private.length());
    assert_eq!(staging.storage_mode(), storage_mode::SHARED);
    assert!(staging.is_cpu_accessible());
}

#[test]
fn command_lifecycle_is_shared_across_clones() {
    let device = common::device();
    let queue = device.new_command_queue().expect("command queue");
    let command_buffer = queue.new_command_buffer().expect("command buffer");
    let clone = command_buffer.clone();

    let encoder = command_buffer
        .new_blit_command_encoder()
        .expect("blit encoder");
    assert!(matches!(
        clone.commit(),
        Err(CommandBufferError::ActiveEncoder)
    ));

    drop(encoder);
    clone.commit().expect("commit after encoder drop");
    assert!(matches!(
        command_buffer.commit(),
        Err(CommandBufferError::InvalidState { .. })
    ));
    clone
        .wait_until_completed()
        .expect("complete command buffer");
    assert_eq!(
        clone.status(),
        apple_metal::command_buffer_status::COMPLETED
    );
    command_buffer
        .wait_until_completed()
        .expect("repeat completed wait");
    assert!(matches!(
        command_buffer.new_blit_command_encoder(),
        Err(CommandBufferError::InvalidState { .. })
    ));
}

#[test]
fn function_tables_and_acceleration_structures_follow_device_support() {
    let device = common::device();
    let (_library, _increment, _args, pipeline) = common::compile_compute(&device);
    assert_eq!(
        pipeline.new_visible_function_table(1).is_some(),
        device.supports_function_pointers()
    );
    assert_eq!(
        pipeline.new_intersection_function_table(1).is_some(),
        device.supports_raytracing()
    );
    assert_eq!(
        device.new_acceleration_structure_with_size(256).is_some(),
        device.supports_raytracing()
    );
    if let Some(heap) = device.new_heap(1 << 20, storage_mode::PRIVATE) {
        if !device.supports_raytracing() {
            assert!(heap.new_acceleration_structure_with_size(256).is_none());
        } else if common::heap_resources_work(&device) {
            assert!(heap.new_acceleration_structure_with_size(256).is_some());
        }
    }
}

#[test]
fn encoder_rejects_wait_after_updating_the_same_fence() {
    let device = common::device();
    let Some(fence) = device.new_fence() else {
        return;
    };
    let queue = device.new_command_queue().expect("command queue");
    let command_buffer = queue.new_command_buffer().expect("command buffer");
    let mut encoder = command_buffer
        .new_blit_command_encoder()
        .expect("blit encoder");

    encoder.update_fence(&fence).expect("update fence");
    assert!(matches!(
        encoder.wait_for_fence(&fence),
        Err(CommandBufferError::FenceWaitAfterUpdate)
    ));
    encoder.end_encoding().expect("end blit encoder");
    command_buffer.commit().expect("commit command buffer");
    command_buffer
        .wait_until_completed()
        .expect("complete command buffer");
}

#[test]
#[allow(clippy::too_many_lines)]
fn argument_encoder_rejects_invalid_binding_state_and_layout() {
    let device = common::device();
    assert!(matches!(
        device.new_argument_encoder_with_descriptors(&[]),
        Err(ArgumentEncoderError::EmptyDescriptorSet)
    ));
    assert!(matches!(
        device.new_argument_encoder_with_descriptors(&[ArgumentDescriptor::constant(1, 0, 0)]),
        Err(ArgumentEncoderError::UnsupportedDataType { data_type: 1 })
    ));

    let mut encoder = device
        .new_argument_encoder_with_descriptors(&[
            ArgumentDescriptor::buffer(0, binding_access::READ_WRITE),
            ArgumentDescriptor::texture(1, texture_type::TYPE_2D, binding_access::READ_ONLY),
        ])
        .expect("argument encoder");
    let storage = device
        .new_buffer(16, resource_options::STORAGE_MODE_SHARED)
        .expect("storage buffer");
    let texture = device
        .new_texture(TextureDescriptor::new_2d(4, 4, pixel_format::RGBA8UNORM))
        .expect("texture");

    let private_argument_buffer = device
        .new_buffer(
            encoder.encoded_length(),
            resource_options::STORAGE_MODE_PRIVATE,
        )
        .expect("private argument buffer");
    assert!(matches!(
        unsafe { encoder.bind_argument_buffer(&private_argument_buffer, 0) },
        Err(ArgumentEncoderError::CpuInaccessibleArgumentBuffer {
            storage_mode: storage_mode::PRIVATE
        })
    ));

    if encoder.alignment() > 1 {
        let padded = device
            .new_buffer(
                encoder.encoded_length() + encoder.alignment(),
                resource_options::STORAGE_MODE_SHARED,
            )
            .expect("padded argument buffer");
        assert!(matches!(
            unsafe { encoder.bind_argument_buffer(&padded, 1) },
            Err(ArgumentEncoderError::MisalignedArgumentBufferOffset { .. })
        ));
    }

    let short = device
        .new_buffer(
            encoder.encoded_length().saturating_sub(1),
            resource_options::STORAGE_MODE_SHARED,
        )
        .expect("short argument buffer");
    assert!(matches!(
        unsafe { encoder.bind_argument_buffer(&short, 0) },
        Err(ArgumentEncoderError::ArgumentBufferRangeOutOfBounds { .. })
    ));

    let argument_buffer = device
        .new_buffer(
            encoder.encoded_length(),
            resource_options::STORAGE_MODE_SHARED,
        )
        .expect("argument buffer");
    {
        let mut binding = unsafe {
            encoder
                .bind_argument_buffer(&argument_buffer, 0)
                .expect("bind argument buffer")
        };
        assert!(matches!(
            binding.set_texture(&texture, 0),
            Err(ArgumentEncoderError::BindingTypeMismatch { .. })
        ));
        assert!(matches!(
            binding.set_buffer(&storage, 0, 7),
            Err(ArgumentEncoderError::InvalidBindingIndex { index: 7 })
        ));
        drop(binding);
    }

    let (_library, _increment, args, _pipeline) = common::compile_compute(&device);
    let mut function_encoder = args.new_argument_encoder(0).expect("function encoder");
    let function_argument_buffer = device
        .new_buffer(
            function_encoder.encoded_length(),
            resource_options::STORAGE_MODE_SHARED,
        )
        .expect("function argument buffer");
    let mut binding = unsafe {
        function_encoder
            .bind_argument_buffer(&function_argument_buffer, 0)
            .expect("bind function argument buffer")
    };
    assert!(matches!(
        binding.set_buffer(&storage, 0, 0),
        Err(ArgumentEncoderError::LayoutUnavailable)
    ));
    drop(binding);

    let mut high_index_encoder = device
        .new_argument_encoder_with_descriptors(&[ArgumentDescriptor::buffer(
            1_024,
            binding_access::READ_WRITE,
        )])
        .expect("high-index argument encoder");
    let high_index_argument_buffer = device
        .new_buffer(
            high_index_encoder.encoded_length(),
            resource_options::STORAGE_MODE_SHARED,
        )
        .expect("high-index argument buffer");
    let mut high_index_binding = unsafe {
        high_index_encoder
            .bind_argument_buffer(&high_index_argument_buffer, 0)
            .expect("bind high-index argument buffer")
    };
    high_index_binding
        .set_buffer(&storage, 0, 1_024)
        .expect("bind high-index resource");
    drop(high_index_binding);
}

#[test]
fn argument_buffer_retention_rejects_cycles() {
    let device = common::device();
    let descriptors = [ArgumentDescriptor::buffer(0, binding_access::READ_WRITE)];
    let mut first_encoder = device
        .new_argument_encoder_with_descriptors(&descriptors)
        .expect("first argument encoder");
    let mut second_encoder = device
        .new_argument_encoder_with_descriptors(&descriptors)
        .expect("second argument encoder");
    let length = first_encoder
        .encoded_length()
        .max(second_encoder.encoded_length());
    let first = device
        .new_buffer(length, resource_options::STORAGE_MODE_SHARED)
        .expect("first argument buffer");
    let second = device
        .new_buffer(length, resource_options::STORAGE_MODE_SHARED)
        .expect("second argument buffer");

    {
        let mut binding = unsafe {
            first_encoder
                .bind_argument_buffer(&first, 0)
                .expect("bind first argument buffer")
        };
        assert!(matches!(
            binding.set_buffer(&first, 0, 0),
            Err(ArgumentEncoderError::NativeRejected { .. })
        ));
        binding
            .set_buffer(&second, 0, 0)
            .expect("retain second argument buffer");
        drop(binding);
    }

    let mut binding = unsafe {
        second_encoder
            .bind_argument_buffer(&second, 0)
            .expect("bind second argument buffer")
    };
    assert!(matches!(
        binding.set_buffer(&first, 0, 0),
        Err(ArgumentEncoderError::NativeRejected { .. })
    ));
    drop(binding);
}

#[test]
fn texture_transfers_reject_invalid_layouts_and_storage() {
    let device = common::device();
    let texture = device
        .new_texture(TextureDescriptor::new_2d(4, 4, pixel_format::RGBA8UNORM))
        .expect("shared texture");
    let bytes = vec![0_u8; 64];

    assert!(matches!(
        unsafe { texture.replace_region_2d(&bytes, 15, (0, 0), (4, 4), 0) },
        Err(TextureTransferError::BytesPerRowTooSmall { minimum: 16, .. })
    ));
    assert!(matches!(
        unsafe { texture.replace_region_2d(&bytes, usize::MAX, (0, 0), (4, 4), 0) },
        Err(TextureTransferError::IntegerOutOfRange {
            field: "bytes_per_row",
            ..
        })
    ));
    assert!(matches!(
        unsafe { texture.replace_region_2d(&bytes, 16, (0, 0), (4, 4), 1) },
        Err(TextureTransferError::InvalidMipmapLevel { .. })
    ));
    assert!(matches!(
        unsafe { texture.replace_region_2d_at_slice(&bytes, 16, (0, 0), (4, 4), 0, 1) },
        Err(TextureTransferError::InvalidSlice { .. })
    ));

    let mut private_descriptor = TextureDescriptor::new_2d(4, 4, pixel_format::RGBA8UNORM);
    private_descriptor.storage_mode = storage_mode::PRIVATE;
    let private = device
        .new_texture(private_descriptor)
        .expect("private texture");
    assert!(matches!(
        unsafe { private.replace_region_2d(&bytes, 16, (0, 0), (4, 4), 0) },
        Err(TextureTransferError::CpuInaccessibleStorage {
            storage_mode: storage_mode::PRIVATE
        })
    ));
}

#[cfg(feature = "iosurface")]
#[test]
fn iosurface_texture_uses_actual_odd_chroma_plane_dimensions() {
    use apple_cf::iosurface::{IOSurface, PlaneProperties};
    use apple_metal::IOSurfaceMetalExt;

    let width = 5;
    let height = 3;
    let plane0_bytes_per_row = 16;
    let plane1_bytes_per_row = 16;
    let plane0_size = plane0_bytes_per_row * height;
    let plane1_width = 3;
    let plane1_height = 2;
    let plane1_size = plane1_bytes_per_row * plane1_height;
    let surface = IOSurface::create_with_properties(
        width,
        height,
        u32::from_be_bytes(*b"420v"),
        1,
        plane0_bytes_per_row,
        plane0_size + plane1_size,
        Some(&[
            PlaneProperties {
                width,
                height,
                bytes_per_row: plane0_bytes_per_row,
                bytes_per_element: 1,
                offset: 0,
                size: plane0_size,
            },
            PlaneProperties {
                width: plane1_width,
                height: plane1_height,
                bytes_per_row: plane1_bytes_per_row,
                bytes_per_element: 2,
                offset: plane0_size,
                size: plane1_size,
            },
        ]),
    )
    .expect("odd bi-planar IOSurface");

    let texture = surface
        .create_metal_texture(&common::device(), 1)
        .expect("chroma texture");
    assert_eq!(texture.width(), plane1_width);
    assert_eq!(texture.height(), plane1_height);
    assert_eq!(texture.pixel_format(), pixel_format::RG8UNORM);
}
