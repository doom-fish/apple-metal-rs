mod common;

use apple_metal::{
    pixel_format, resource_options, storage_mode, texture_type, texture_usage, MetalDevice,
    TextureDescriptor,
};

fn device() -> Option<MetalDevice> {
    let device = MetalDevice::system_default();
    if device.is_none() {
        eprintln!("skipping: no Metal device");
    }
    device
}

const fn private(mut descriptor: TextureDescriptor) -> TextureDescriptor {
    descriptor.storage_mode = storage_mode::PRIVATE;
    descriptor
}

#[test]
fn new_2d_keeps_the_single_layer_defaults() {
    let descriptor = TextureDescriptor::new_2d(8, 4, pixel_format::RGBA8UNORM);
    assert_eq!(descriptor.texture_type, texture_type::TYPE_2D);
    assert_eq!(descriptor.depth, 1);
    assert_eq!(descriptor.array_length, 1);
    assert_eq!(descriptor.sample_count, 1);
    assert_eq!(
        descriptor
            .with_depth(3)
            .with_array_length(2)
            .with_sample_count(4),
        TextureDescriptor {
            depth: 3,
            array_length: 2,
            sample_count: 4,
            ..descriptor
        }
    );
    let render_target = TextureDescriptor::render_target_2d(8, 4, pixel_format::BGRA8UNORM);
    assert_eq!(render_target.texture_type, texture_type::TYPE_2D);
    assert_eq!(
        (
            render_target.depth,
            render_target.array_length,
            render_target.sample_count
        ),
        (1, 1, 1)
    );
}

#[test]
fn creates_3d_array_cube_and_multisample_textures() {
    let Some(device) = device() else { return };

    let volume = device
        .new_texture(
            TextureDescriptor::new_2d(8, 4, pixel_format::RGBA8UNORM)
                .with_texture_type(texture_type::TYPE_3D)
                .with_depth(2),
        )
        .expect("3D texture");
    assert_eq!(volume.texture_type(), texture_type::TYPE_3D);
    assert_eq!((volume.width(), volume.height(), volume.depth()), (8, 4, 2));

    let mut mipmapped_volume = TextureDescriptor::new_2d(4, 4, pixel_format::RGBA8UNORM)
        .with_texture_type(texture_type::TYPE_3D)
        .with_depth(16);
    mipmapped_volume.mipmapped = true;
    let mipmapped_volume = device
        .new_texture(mipmapped_volume)
        .expect("mipmapped 3D texture");
    assert_eq!(mipmapped_volume.mipmap_level_count(), 5);

    let layers = device
        .new_texture(
            TextureDescriptor::new_2d(4, 4, pixel_format::RGBA8UNORM)
                .with_texture_type(texture_type::TYPE_2D_ARRAY)
                .with_array_length(3),
        )
        .expect("2D array texture");
    assert_eq!(layers.texture_type(), texture_type::TYPE_2D_ARRAY);
    assert_eq!(layers.array_length(), 3);

    let cube = device
        .new_texture(
            TextureDescriptor::new_2d(16, 16, pixel_format::RGBA8UNORM)
                .with_texture_type(texture_type::CUBE),
        )
        .expect("cube texture");
    assert_eq!(cube.texture_type(), texture_type::CUBE);

    let cube_array = device.new_texture(
        TextureDescriptor::new_2d(16, 16, pixel_format::RGBA8UNORM)
            .with_texture_type(texture_type::CUBE_ARRAY)
            .with_array_length(2),
    );
    if let Some(cube_array) = cube_array {
        assert_eq!(cube_array.texture_type(), texture_type::CUBE_ARRAY);
        assert_eq!(cube_array.array_length(), 2);
    }

    let mut multisample = private(
        TextureDescriptor::new_2d(16, 16, pixel_format::BGRA8UNORM)
            .with_texture_type(texture_type::TYPE_2D_MULTISAMPLE)
            .with_sample_count(4),
    );
    multisample.usage = texture_usage::RENDER_TARGET;
    let multisample = device
        .new_texture(multisample)
        .expect("4x multisample texture");
    assert_eq!(
        multisample.texture_type(),
        texture_type::TYPE_2D_MULTISAMPLE
    );

    let one_dimensional = device
        .new_texture(
            TextureDescriptor::new_2d(32, 1, pixel_format::R8UNORM)
                .with_texture_type(texture_type::TYPE_1D),
        )
        .expect("1D texture");
    assert_eq!(one_dimensional.texture_type(), texture_type::TYPE_1D);
}

#[test]
fn rejects_descriptors_metal_cannot_build() {
    let Some(device) = device() else { return };
    let base = TextureDescriptor::new_2d(16, 16, pixel_format::RGBA8UNORM);
    let mut mipmapped_multisample = private(
        base.with_texture_type(texture_type::TYPE_2D_MULTISAMPLE)
            .with_sample_count(4),
    );
    mipmapped_multisample.mipmapped = true;
    let mut mipmapped_line = TextureDescriptor::new_2d(16, 1, pixel_format::R8UNORM)
        .with_texture_type(texture_type::TYPE_1D);
    mipmapped_line.mipmapped = true;
    let mut invalid_format = base;
    invalid_format.pixel_format = pixel_format::INVALID;
    let mut unknown_storage = base;
    unknown_storage.storage_mode = 7;
    let mut huge = base;
    huge.width = usize::MAX;

    for (label, descriptor) in [
        ("2D with depth", base.with_depth(2)),
        ("2D with layers", base.with_array_length(2)),
        ("2D with samples", base.with_sample_count(4)),
        ("zero depth", base.with_depth(0)),
        ("zero layers", base.with_array_length(0)),
        ("zero samples", base.with_sample_count(0)),
        (
            "non-square cube",
            TextureDescriptor::new_2d(16, 8, pixel_format::RGBA8UNORM)
                .with_texture_type(texture_type::CUBE),
        ),
        (
            "3D with layers",
            base.with_texture_type(texture_type::TYPE_3D)
                .with_array_length(2),
        ),
        (
            "single-sample multisample",
            private(base.with_texture_type(texture_type::TYPE_2D_MULTISAMPLE)),
        ),
        ("mipmapped multisample", mipmapped_multisample),
        ("tall 1D", base.with_texture_type(texture_type::TYPE_1D)),
        ("mipmapped 1D", mipmapped_line),
        (
            "texture buffer",
            base.with_texture_type(texture_type::TEXTURE_BUFFER),
        ),
        ("unknown type", base.with_texture_type(99)),
        ("oversized type", base.with_texture_type(usize::MAX)),
        ("invalid format", invalid_format),
        ("unknown storage", unknown_storage),
        ("huge width", huge),
    ] {
        assert!(device.new_texture(descriptor).is_none(), "{label}");
    }
}

#[test]
fn limits_and_unknown_values_return_none_instead_of_aborting() {
    let Some(device) = device() else { return };
    let base = TextureDescriptor::new_2d(16, 16, pixel_format::R8UNORM);
    let mut unknown_format = base;
    unknown_format.pixel_format = 9_999;
    let mut unspecialized = base;
    unspecialized.pixel_format = pixel_format::UNSPECIALIZED;
    let mut atomic = base;
    atomic.usage = 0x20;
    let mut unknown_usage = base;
    unknown_usage.usage = 0x8000_0000;
    let mut wide = base;
    wide.width = 16_385;
    let mut wide_line = TextureDescriptor::new_2d(16_385, 1, pixel_format::R8UNORM)
        .with_texture_type(texture_type::TYPE_1D);
    wide_line.storage_mode = storage_mode::PRIVATE;
    let mut odd_samples = private(
        base.with_texture_type(texture_type::TYPE_2D_MULTISAMPLE)
            .with_sample_count(3),
    );
    odd_samples.usage = texture_usage::RENDER_TARGET;

    for (label, descriptor) in [
        ("unknown format", unknown_format),
        ("unspecialized format", unspecialized),
        ("atomic usage", atomic),
        ("unknown usage", unknown_usage),
        ("2D wider than 16384", wide),
        ("1D wider than 16384", wide_line),
        (
            "3D deeper than 2048",
            private(
                base.with_texture_type(texture_type::TYPE_3D)
                    .with_depth(2_049),
            ),
        ),
        (
            "2D array with 2049 layers",
            private(
                base.with_texture_type(texture_type::TYPE_2D_ARRAY)
                    .with_array_length(2_049),
            ),
        ),
        (
            "cube array with 342 cubes",
            private(
                base.with_texture_type(texture_type::CUBE_ARRAY)
                    .with_array_length(342),
            ),
        ),
        ("three samples", odd_samples),
    ] {
        assert!(device.new_texture(descriptor).is_none(), "{label}");
    }

    let at_limits = private(
        base.with_texture_type(texture_type::TYPE_2D_ARRAY)
            .with_array_length(2_048),
    );
    assert!(device.new_texture(at_limits).is_some());
    let cube_array_limit = private(
        base.with_texture_type(texture_type::CUBE_ARRAY)
            .with_array_length(341),
    );
    assert!(device.new_texture(cube_array_limit).is_some());

    let mut depth_stencil = private(base);
    depth_stencil.pixel_format = pixel_format::DEPTH24UNORM_STENCIL8;
    depth_stencil.usage = texture_usage::RENDER_TARGET;
    let _ = device.new_texture(depth_stencil);
}

#[test]
fn heap_textures_accept_every_descriptor_shape_matching_the_heap() {
    let Some(device) = device() else { return };
    let Some(heap) = device.new_heap(1 << 22, storage_mode::PRIVATE) else {
        eprintln!("skipping: no private heap");
        return;
    };
    if common::heap_resources_work(&device) {
        let layers = heap
            .new_texture(private(
                TextureDescriptor::new_2d(8, 8, pixel_format::RGBA8UNORM)
                    .with_texture_type(texture_type::TYPE_2D_ARRAY)
                    .with_array_length(2),
            ))
            .expect("heap array texture");
        assert_eq!(layers.array_length(), 2);
        assert_eq!(layers.storage_mode(), storage_mode::PRIVATE);
    }

    let shared = TextureDescriptor::new_2d(8, 8, pixel_format::RGBA8UNORM);
    assert!(heap.new_texture(shared).is_none());
    assert!(heap.new_texture(private(shared.with_depth(2))).is_none());
}

#[test]
fn buffers_can_be_created_from_bytes() {
    let Some(device) = device() else { return };
    let bytes: Vec<u8> = (0..=255).collect();
    let buffer = device
        .new_buffer_with_bytes(&bytes, resource_options::STORAGE_MODE_SHARED)
        .expect("buffer with bytes");
    assert_eq!(buffer.length(), bytes.len());
    let mut copy = vec![0_u8; bytes.len()];
    unsafe { buffer.read_bytes(0, &mut copy) }.expect("read back");
    assert_eq!(copy, bytes);

    assert!(device
        .new_buffer_with_bytes(&[], resource_options::STORAGE_MODE_SHARED)
        .is_none());
    assert!(device
        .new_buffer_with_bytes(&bytes, resource_options::STORAGE_MODE_PRIVATE)
        .is_none());
    assert!(device.new_buffer_with_bytes(&bytes, 3 << 4).is_none());
    assert!(device.new_buffer_with_bytes(&bytes, usize::MAX).is_none());
}
