use apple_metal::{
    pixel_format, resource_options, storage_mode, MetalBuffer, MetalDevice, TextureViewError,
};

fn offset_alignment(buffer: &MetalBuffer, format: usize) -> usize {
    match buffer.new_texture_view_2d(format, 1, 1, 4096, 1).err() {
        Some(TextureViewError::Misaligned {
            field: "offset",
            value: 1,
            alignment,
        }) => alignment,
        other => panic!("expected an offset alignment error, got {other:?}"),
    }
}

#[test]
fn texture_views_check_their_layout_against_the_buffer() {
    let Some(device) = MetalDevice::system_default() else {
        eprintln!("skipping: no Metal device");
        return;
    };
    let buffer = device
        .new_buffer(4096, resource_options::STORAGE_MODE_SHARED)
        .expect("shared buffer");
    let alignment = offset_alignment(&buffer, pixel_format::RGBA8UNORM);
    assert!(alignment >= 4 && alignment.is_power_of_two());
    let row = 64_usize.div_ceil(alignment) * alignment;

    let view = buffer
        .new_texture_view_2d(pixel_format::RGBA8UNORM, 16, 4, row, 0)
        .expect("view inside the buffer");
    assert_eq!((view.width(), view.height()), (16, 4));
    assert_eq!(view.pixel_format(), pixel_format::RGBA8UNORM);
    assert_eq!(view.storage_mode(), storage_mode::SHARED);

    let rows_that_fit = (4096 - alignment) / row;
    buffer
        .new_texture_view_2d(pixel_format::RGBA8UNORM, 16, rows_that_fit, row, alignment)
        .expect("offset view inside the buffer");

    let too_tall = 4096 / row + 1;
    assert_eq!(
        buffer
            .new_texture_view_2d(pixel_format::RGBA8UNORM, 16, too_tall, row, 0)
            .err(),
        Some(TextureViewError::OutOfBounds {
            required: row * too_tall,
            buffer_length: 4096,
        })
    );
    assert_eq!(
        buffer
            .new_texture_view_2d(pixel_format::RGBA8UNORM, 16, 4, row, 4096 - row)
            .err(),
        Some(TextureViewError::OutOfBounds {
            required: 4096 - row + row * 4,
            buffer_length: 4096,
        })
    );
    assert_eq!(
        buffer
            .new_texture_view_2d(pixel_format::RGBA8UNORM, 16, 4, 60, 0)
            .err(),
        Some(TextureViewError::BytesPerRowTooSmall {
            bytes_per_row: 60,
            minimum: 64,
        })
    );
    assert_eq!(
        buffer
            .new_texture_view_2d(pixel_format::RGBA8UNORM, 1, 1, 66, 0)
            .err(),
        Some(TextureViewError::Misaligned {
            field: "bytes_per_row",
            value: 66,
            alignment: 4,
        })
    );
    if alignment > 4 {
        assert_eq!(
            buffer
                .new_texture_view_2d(pixel_format::RGBA8UNORM, 1, 1, row + 4, 0)
                .err(),
            Some(TextureViewError::Misaligned {
                field: "bytes_per_row",
                value: row + 4,
                alignment,
            })
        );
    }
    assert_eq!(
        buffer
            .new_texture_view_2d(pixel_format::BGRA10_XR, 16, 4, 64, 0)
            .err(),
        Some(TextureViewError::BytesPerRowTooSmall {
            bytes_per_row: 64,
            minimum: 128,
        })
    );
}

#[test]
fn texture_views_reject_formats_and_values_metal_cannot_take() {
    let Some(device) = MetalDevice::system_default() else {
        eprintln!("skipping: no Metal device");
        return;
    };
    let buffer = device
        .new_buffer(4096, resource_options::STORAGE_MODE_SHARED)
        .expect("shared buffer");
    for format in [
        pixel_format::INVALID,
        pixel_format::DEPTH32FLOAT,
        pixel_format::STENCIL8,
        pixel_format::DEPTH32FLOAT_STENCIL8,
        pixel_format::BC1_RGBA,
        pixel_format::ASTC_4X4_LDR,
        pixel_format::GBGR422,
        9999,
        usize::MAX,
    ] {
        assert_eq!(
            buffer.new_texture_view_2d(format, 4, 4, 256, 0).err(),
            Some(TextureViewError::UnsupportedPixelFormat {
                pixel_format: format
            })
        );
    }
    assert_eq!(
        buffer
            .new_texture_view_2d(pixel_format::RGBA8UNORM, 0, 4, 256, 0)
            .err(),
        Some(TextureViewError::EmptyRegion)
    );
    assert_eq!(
        buffer
            .new_texture_view_2d(pixel_format::RGBA8UNORM, 4, 4, 256, usize::MAX)
            .err(),
        Some(TextureViewError::IntegerOutOfRange {
            field: "offset",
            value: usize::MAX,
        })
    );
    assert_eq!(
        buffer
            .new_texture_view_2d(pixel_format::RGBA8UNORM, isize::MAX as usize, 1, 256, 0)
            .err(),
        Some(TextureViewError::LayoutOverflow)
    );
}

#[test]
fn texture_views_inherit_the_buffer_storage_mode() {
    let Some(device) = MetalDevice::system_default() else {
        eprintln!("skipping: no Metal device");
        return;
    };
    let private = device
        .new_buffer(4096, resource_options::STORAGE_MODE_PRIVATE)
        .expect("private buffer");
    let alignment = offset_alignment(&private, pixel_format::RGBA8UNORM);
    let row = 64_usize.div_ceil(alignment) * alignment;
    let view = private
        .new_texture_view_2d(pixel_format::RGBA8UNORM, 16, 4, row, 0)
        .expect("private view");
    assert_eq!(view.storage_mode(), storage_mode::PRIVATE);

    if let Some(managed) = device.new_buffer(4096, resource_options::STORAGE_MODE_MANAGED) {
        if managed.storage_mode() == storage_mode::MANAGED {
            let view = managed
                .new_texture_view_2d(pixel_format::RGBA8UNORM, 16, 4, row, 0)
                .expect("managed view");
            assert_eq!(view.storage_mode(), storage_mode::MANAGED);
        }
    }
}
