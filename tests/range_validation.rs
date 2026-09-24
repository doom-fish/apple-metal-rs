mod common;

use apple_metal::{
    indirect_command_type, pixel_format, resource_options, storage_mode, texture_usage,
    CommandBufferError, MetalDevice, MetalRasterizationRateLayerDescriptor, TextureDescriptor,
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
    if common::heap_resources_work(&device) {
        assert!(heap
            .new_buffer(256, resource_options::STORAGE_MODE_SHARED)
            .is_some());
    }
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
fn texture_views_follow_metals_reinterpretation_rules() {
    let Some(device) = device() else { return };
    let plain = device
        .new_texture(TextureDescriptor::new_2d(4, 4, pixel_format::RGBA8UNORM))
        .expect("texture");
    for format in [pixel_format::RGBA8UNORM, pixel_format::RGBA8UNORM_SRGB] {
        let view = plain.new_view(format).expect("same format or sRGB twin");
        assert_eq!(view.pixel_format(), format);
    }
    for format in [
        pixel_format::BGRA8UNORM,
        pixel_format::R32FLOAT,
        pixel_format::RGBA16FLOAT,
        pixel_format::R8UNORM,
        pixel_format::BC1_RGBA,
        pixel_format::DEPTH32FLOAT,
        pixel_format::INVALID,
        9_999,
        usize::MAX,
    ] {
        assert!(plain.new_view(format).is_none(), "{format}");
    }

    let mut reinterpretable = TextureDescriptor::new_2d(4, 4, pixel_format::RGBA8UNORM);
    reinterpretable.usage |= texture_usage::PIXEL_FORMAT_VIEW;
    let reinterpretable = device
        .new_texture(reinterpretable)
        .expect("pixel-format-view texture");
    for format in [
        pixel_format::BGRA8UNORM,
        pixel_format::R32FLOAT,
        pixel_format::RGBA8UNORM_SRGB,
    ] {
        let view = reinterpretable.new_view(format).expect("same-size view");
        assert_eq!(view.pixel_format(), format);
    }
    for format in [
        pixel_format::RGBA16FLOAT,
        pixel_format::BC1_RGBA,
        pixel_format::DEPTH32FLOAT,
    ] {
        assert!(reinterpretable.new_view(format).is_none(), "{format}");
    }

    let mut depth_stencil = TextureDescriptor::new_2d(4, 4, pixel_format::DEPTH32FLOAT_STENCIL8);
    depth_stencil.storage_mode = storage_mode::PRIVATE;
    depth_stencil.usage = texture_usage::SHADER_READ | texture_usage::RENDER_TARGET;
    if let Some(texture) = device.new_texture(depth_stencil) {
        assert!(texture.new_view(pixel_format::X32_STENCIL8).is_none());
    }
    depth_stencil.usage |= texture_usage::PIXEL_FORMAT_VIEW;
    if let Some(texture) = device.new_texture(depth_stencil) {
        let stencil = texture
            .new_view(pixel_format::X32_STENCIL8)
            .expect("stencil view");
        assert_eq!(stencil.pixel_format(), pixel_format::X32_STENCIL8);
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

#[test]
fn intersection_function_tables_check_index_and_signature() {
    let Some(device) = device() else { return };
    if !device.supports_raytracing() {
        eprintln!("skipping: no ray tracing");
        return;
    }
    let library = device
        .new_library_with_source(
            "#include <metal_stdlib>\nusing namespace metal;\nkernel void k(device uint *o [[buffer(0)]], uint i [[thread_position_in_grid]]) { o[i] = i; }",
        )
        .expect("compute MSL");
    let function = library.new_function("k").expect("kernel");
    let pipeline = device
        .new_compute_pipeline_state(&function)
        .expect("compute pipeline");
    let table = pipeline
        .new_intersection_function_table(2)
        .expect("intersection function table");
    assert_eq!(table.function_count(), 2);
    table
        .set_opaque_triangle_intersection_function(
            apple_metal::intersection_function_signature::TRIANGLE_DATA,
            1,
        )
        .expect("index inside the table");
    assert_eq!(
        table.set_opaque_triangle_intersection_function(0, 2),
        Err(apple_metal::FunctionTableError::IndexOutOfRange {
            index: 2,
            function_count: 2,
        })
    );
    assert_eq!(
        table.set_opaque_triangle_intersection_function(1 << 10, 0),
        Err(apple_metal::FunctionTableError::UnknownSignature { signature: 1 << 10 })
    );
}

#[test]
fn metalfx_scalers_refuse_formats_and_sizes_that_abort() {
    let Some(device) = device() else { return };
    if !apple_metal::SpatialScalerDescriptor::supports_device(&device) {
        eprintln!("skipping: no MetalFX spatial scaler");
        return;
    }
    let valid = apple_metal::SpatialScalerDescriptor::new(
        pixel_format::BGRA8UNORM,
        pixel_format::BGRA8UNORM,
        64,
        64,
        128,
        128,
    );
    assert!(device.new_spatial_scaler(&valid).is_some());
    let mut depth_output = valid;
    depth_output.output_texture_format = pixel_format::DEPTH32FLOAT;
    assert!(device.new_spatial_scaler(&depth_output).is_none());
    let mut unknown = valid;
    unknown.color_texture_format = 9_999;
    assert!(device.new_spatial_scaler(&unknown).is_none());
    let mut huge = valid;
    huge.output_width = 1 << 20;
    assert!(device.new_spatial_scaler(&huge).is_none());
    let mut empty = valid;
    empty.input_width = 0;
    assert!(device.new_spatial_scaler(&empty).is_none());

    if !apple_metal::TemporalScalerDescriptor::supports_device(&device) {
        return;
    }
    let temporal = apple_metal::TemporalScalerDescriptor::new(
        pixel_format::BGRA8UNORM,
        pixel_format::DEPTH32FLOAT,
        pixel_format::RG16FLOAT,
        pixel_format::BGRA8UNORM,
        (64, 64),
        (128, 128),
    );
    assert!(device.new_temporal_scaler(&temporal).is_some());
    let mut depth_as_color = temporal;
    depth_as_color.color_texture_format = pixel_format::DEPTH32FLOAT;
    assert!(device.new_temporal_scaler(&depth_as_color).is_none());
    let mut compressed_motion = temporal;
    compressed_motion.motion_texture_format = pixel_format::BC1_RGBA;
    assert!(device.new_temporal_scaler(&compressed_motion).is_none());
}
