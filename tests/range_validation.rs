use apple_metal::{
    indirect_command_type, pixel_format, resource_options, storage_mode, CommandBufferError,
    MetalDevice, MetalRasterizationRateLayerDescriptor, TextureDescriptor,
};

fn device() -> Option<MetalDevice> {
    let device = MetalDevice::system_default();
    if device.is_none() {
        eprintln!("skipping: no Metal device");
    }
    device
}

#[test]
fn indirect_command_ranges_are_checked_against_the_command_count() {
    let Some(device) = device() else { return };
    let Some(indirect) = device.new_indirect_command_buffer(
        indirect_command_type::CONCURRENT_DISPATCH,
        4,
        0,
        0,
        4,
        resource_options::STORAGE_MODE_PRIVATE,
    ) else {
        eprintln!("skipping: no indirect command buffer support");
        return;
    };
    assert_eq!(indirect.size(), 4);
    indirect.reset_range(0..4).expect("whole range");
    indirect.reset_range(2..2).expect("empty range");
    assert_eq!(
        indirect.reset_range(1..5),
        Err(CommandBufferError::RangeOutOfBounds {
            resource: "indirect command buffer",
            offset: 1,
            length: 4,
            resource_length: 4,
        })
    );
    let reversed = core::ops::Range { start: 3, end: 2 };
    assert_eq!(
        indirect.reset_range(reversed),
        Err(CommandBufferError::InvalidRange)
    );
    assert!(matches!(
        indirect.reset_range(1..usize::MAX),
        Err(CommandBufferError::RangeOutOfBounds { .. })
    ));
    assert!(device
        .new_indirect_command_buffer(
            indirect_command_type::CONCURRENT_DISPATCH,
            0,
            0,
            0,
            4,
            resource_options::STORAGE_MODE_PRIVATE,
        )
        .is_none());
}

#[test]
fn counter_ranges_outside_the_sample_buffer_resolve_to_none() {
    let Some(device) = device() else { return };
    let Some(counter_set) = device.counter_set_names().into_iter().next() else {
        eprintln!("skipping: no counter sets");
        return;
    };
    let Ok(samples) = device.new_counter_sample_buffer(&counter_set, 4, storage_mode::SHARED, None)
    else {
        eprintln!("skipping: counter sample buffer unavailable");
        return;
    };
    assert_eq!(samples.sample_count(), 4);
    assert!(samples.resolve_range(0..5).is_none());
    assert!(samples.resolve_range(4..8).is_none());
    assert!(samples.resolve_range(2..2).is_none());
    assert!(samples.resolve_range(1..usize::MAX).is_none());
    let reversed = core::ops::Range { start: 3, end: 1 };
    assert!(samples.resolve_range(reversed).is_none());
    assert!(device
        .new_counter_sample_buffer(&counter_set, 4, usize::MAX, None)
        .is_err());
    assert!(device
        .new_counter_sample_buffer(&counter_set, usize::MAX, storage_mode::SHARED, None)
        .is_err());
}

#[test]
fn heap_buffers_respect_native_int_and_heap_modes() {
    let Some(device) = device() else { return };
    let Some(heap) = device.new_heap(1 << 20, storage_mode::SHARED) else {
        eprintln!("skipping: no shared heap");
        return;
    };
    assert!(heap
        .new_buffer(256, resource_options::STORAGE_MODE_SHARED)
        .is_some());
    assert!(heap
        .new_buffer(usize::MAX, resource_options::STORAGE_MODE_SHARED)
        .is_none());
    assert!(heap
        .new_buffer(256, resource_options::STORAGE_MODE_PRIVATE)
        .is_none());
    assert!(heap
        .new_buffer(256, resource_options::CPU_CACHE_MODE_WRITE_COMBINED)
        .is_none());
    assert!(heap
        .new_buffer(0, resource_options::STORAGE_MODE_SHARED)
        .is_none());
    assert!(device.new_heap(1 << 20, usize::MAX).is_none());
    assert!(device.new_heap(1 << 20, 7).is_none());
    assert!(device.new_heap(usize::MAX, storage_mode::SHARED).is_none());
}

#[test]
fn texture_views_keep_the_pixel_size() {
    let Some(device) = device() else { return };
    let texture = device
        .new_texture(TextureDescriptor::new_2d(4, 4, pixel_format::RGBA8UNORM))
        .expect("texture");
    for format in [
        pixel_format::RGBA8UNORM,
        pixel_format::RGBA8UNORM_SRGB,
        pixel_format::BGRA8UNORM,
    ] {
        let view = texture.new_view(format).expect("same-size view");
        assert_eq!(view.pixel_format(), format);
    }
    for format in [
        pixel_format::RGBA16FLOAT,
        pixel_format::R8UNORM,
        pixel_format::BC1_RGBA,
        pixel_format::DEPTH32FLOAT,
        pixel_format::INVALID,
        9_999,
        usize::MAX,
    ] {
        assert!(texture.new_view(format).is_none(), "{format}");
    }
}

#[test]
fn rasterization_rate_layers_use_the_sample_count_initializer() {
    if device().is_none() {
        return;
    }
    assert!(MetalRasterizationRateLayerDescriptor::with_sample_count(4, 2).is_some());
    assert!(MetalRasterizationRateLayerDescriptor::with_sample_count(0, 2).is_none());
    assert!(MetalRasterizationRateLayerDescriptor::with_sample_count(2, 0).is_none());
    assert!(MetalRasterizationRateLayerDescriptor::with_sample_count(usize::MAX, 1).is_none());
    assert!(MetalRasterizationRateLayerDescriptor::with_sample_count(16_385, 1).is_none());
}
