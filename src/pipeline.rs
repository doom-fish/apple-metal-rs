use crate::{
    ffi, util::take_optional_string, ComputePipelineState, MetalDevice, MetalFunction,
    RenderPipelineState,
};

/// `MTLBlendFactor` enum values.
pub mod blend_factor {
    pub const ZERO: usize = 0;
    pub const ONE: usize = 1;
    pub const SOURCE_COLOR: usize = 2;
    pub const ONE_MINUS_SOURCE_COLOR: usize = 3;
    pub const SOURCE_ALPHA: usize = 4;
    pub const ONE_MINUS_SOURCE_ALPHA: usize = 5;
    pub const DESTINATION_COLOR: usize = 6;
    pub const ONE_MINUS_DESTINATION_COLOR: usize = 7;
    pub const DESTINATION_ALPHA: usize = 8;
    pub const ONE_MINUS_DESTINATION_ALPHA: usize = 9;
    pub const SOURCE_ALPHA_SATURATED: usize = 10;
    pub const BLEND_COLOR: usize = 11;
    pub const ONE_MINUS_BLEND_COLOR: usize = 12;
    pub const BLEND_ALPHA: usize = 13;
    pub const ONE_MINUS_BLEND_ALPHA: usize = 14;
    pub const SOURCE1_COLOR: usize = 15;
    pub const ONE_MINUS_SOURCE1_COLOR: usize = 16;
    pub const SOURCE1_ALPHA: usize = 17;
    pub const ONE_MINUS_SOURCE1_ALPHA: usize = 18;
    pub const UNSPECIALIZED: usize = 19;
}

/// `MTLBlendOperation` enum values.
pub mod blend_operation {
    pub const ADD: usize = 0;
    pub const SUBTRACT: usize = 1;
    pub const REVERSE_SUBTRACT: usize = 2;
    pub const MIN: usize = 3;
    pub const MAX: usize = 4;
    pub const UNSPECIALIZED: usize = 5;
}

/// `MTLColorWriteMask` bitmask values.
pub mod color_write_mask {
    pub const NONE: usize = 0;
    pub const RED: usize = 0x1 << 3;
    pub const GREEN: usize = 0x1 << 2;
    pub const BLUE: usize = 0x1 << 1;
    pub const ALPHA: usize = 0x1 << 0;
    pub const ALL: usize = 0xf;
    pub const UNSPECIALIZED: usize = 0x10;
}

/// Safe Rust description of `MTLComputePipelineDescriptor`.
#[derive(Clone, Copy)]
pub struct ComputePipelineDescriptor<'a> {
    pub label: Option<&'a str>,
    pub compute_function: &'a MetalFunction,
    pub thread_group_size_is_multiple_of_thread_execution_width: bool,
    pub max_total_threads_per_threadgroup: usize,
    pub support_indirect_command_buffers: bool,
}

impl<'a> ComputePipelineDescriptor<'a> {
    /// Create a descriptor for a single compute function with default tuning flags.
    #[must_use]
    pub const fn new(compute_function: &'a MetalFunction) -> Self {
        Self {
            label: None,
            compute_function,
            thread_group_size_is_multiple_of_thread_execution_width: false,
            max_total_threads_per_threadgroup: 0,
            support_indirect_command_buffers: false,
        }
    }
}

/// Safe Rust description of `MTLRenderPipelineColorAttachmentDescriptor`.
#[derive(Debug, Clone, Copy)]
pub struct RenderPipelineColorAttachmentDescriptor {
    pub pixel_format: usize,
    pub blending_enabled: bool,
    pub source_rgb_blend_factor: usize,
    pub destination_rgb_blend_factor: usize,
    pub rgb_blend_operation: usize,
    pub source_alpha_blend_factor: usize,
    pub destination_alpha_blend_factor: usize,
    pub alpha_blend_operation: usize,
    pub write_mask: usize,
}

impl Default for RenderPipelineColorAttachmentDescriptor {
    fn default() -> Self {
        Self {
            pixel_format: 0,
            blending_enabled: false,
            source_rgb_blend_factor: blend_factor::ONE,
            destination_rgb_blend_factor: blend_factor::ZERO,
            rgb_blend_operation: blend_operation::ADD,
            source_alpha_blend_factor: blend_factor::ONE,
            destination_alpha_blend_factor: blend_factor::ZERO,
            alpha_blend_operation: blend_operation::ADD,
            write_mask: color_write_mask::ALL,
        }
    }
}

impl RenderPipelineColorAttachmentDescriptor {
    /// Create a color attachment descriptor targeting the given pixel format.
    #[must_use]
    pub const fn new(pixel_format: usize) -> Self {
        Self {
            pixel_format,
            blending_enabled: false,
            source_rgb_blend_factor: blend_factor::ONE,
            destination_rgb_blend_factor: blend_factor::ZERO,
            rgb_blend_operation: blend_operation::ADD,
            source_alpha_blend_factor: blend_factor::ONE,
            destination_alpha_blend_factor: blend_factor::ZERO,
            alpha_blend_operation: blend_operation::ADD,
            write_mask: color_write_mask::ALL,
        }
    }

    const fn as_words(self) -> [usize; 9] {
        [
            self.pixel_format,
            self.blending_enabled as usize,
            self.source_rgb_blend_factor,
            self.destination_rgb_blend_factor,
            self.rgb_blend_operation,
            self.source_alpha_blend_factor,
            self.destination_alpha_blend_factor,
            self.alpha_blend_operation,
            self.write_mask,
        ]
    }
}

/// Safe Rust description of `MTLRenderPipelineDescriptor`.
#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Copy)]
pub struct RenderPipelineDescriptor<'a> {
    pub label: Option<&'a str>,
    pub vertex_function: &'a MetalFunction,
    pub fragment_function: Option<&'a MetalFunction>,
    pub color_attachments: &'a [RenderPipelineColorAttachmentDescriptor],
    pub raster_sample_count: usize,
    pub alpha_to_coverage_enabled: bool,
    pub alpha_to_one_enabled: bool,
    pub rasterization_enabled: bool,
    pub support_indirect_command_buffers: bool,
    pub depth_attachment_pixel_format: usize,
    pub stencil_attachment_pixel_format: usize,
}

impl<'a> RenderPipelineDescriptor<'a> {
    /// Create a descriptor for a simple render pipeline with the given attachment list.
    #[must_use]
    pub const fn new(
        vertex_function: &'a MetalFunction,
        fragment_function: Option<&'a MetalFunction>,
        color_attachments: &'a [RenderPipelineColorAttachmentDescriptor],
    ) -> Self {
        Self {
            label: None,
            vertex_function,
            fragment_function,
            color_attachments,
            raster_sample_count: 1,
            alpha_to_coverage_enabled: false,
            alpha_to_one_enabled: false,
            rasterization_enabled: true,
            support_indirect_command_buffers: false,
            depth_attachment_pixel_format: 0,
            stencil_attachment_pixel_format: 0,
        }
    }
}

/// Safe Rust description of `MTLTileRenderPipelineColorAttachmentDescriptor`.
#[derive(Debug, Clone, Copy)]
pub struct TileRenderPipelineColorAttachmentDescriptor {
    pub pixel_format: usize,
}

impl TileRenderPipelineColorAttachmentDescriptor {
    /// Create a tile color attachment descriptor for the given pixel format.
    #[must_use]
    pub const fn new(pixel_format: usize) -> Self {
        Self { pixel_format }
    }
}

/// Safe Rust description of `MTLTileRenderPipelineDescriptor`.
#[derive(Clone, Copy)]
pub struct TileRenderPipelineDescriptor<'a> {
    pub label: Option<&'a str>,
    pub tile_function: &'a MetalFunction,
    pub color_attachments: &'a [TileRenderPipelineColorAttachmentDescriptor],
    pub raster_sample_count: usize,
    pub threadgroup_size_matches_tile_size: bool,
    pub max_total_threads_per_threadgroup: usize,
}

impl<'a> TileRenderPipelineDescriptor<'a> {
    /// Create a tile-pipeline descriptor with the given tile function and attachments.
    #[must_use]
    pub const fn new(
        tile_function: &'a MetalFunction,
        color_attachments: &'a [TileRenderPipelineColorAttachmentDescriptor],
    ) -> Self {
        Self {
            label: None,
            tile_function,
            color_attachments,
            raster_sample_count: 1,
            threadgroup_size_matches_tile_size: false,
            max_total_threads_per_threadgroup: 0,
        }
    }
}

fn flatten_render_color_attachments(
    color_attachments: &[RenderPipelineColorAttachmentDescriptor],
) -> Result<Vec<usize>, String> {
    if color_attachments.len() > 8 {
        return Err("Metal render pipelines support at most 8 color attachments".to_string());
    }

    let mut flat = Vec::with_capacity(color_attachments.len().saturating_mul(9));
    for attachment in color_attachments {
        flat.extend_from_slice(&attachment.as_words());
    }
    Ok(flat)
}

fn flatten_tile_color_attachments(
    color_attachments: &[TileRenderPipelineColorAttachmentDescriptor],
) -> Result<Vec<usize>, String> {
    if color_attachments.len() > 8 {
        return Err("Metal tile pipelines support at most 8 color attachments".to_string());
    }

    let mut flat = Vec::with_capacity(color_attachments.len());
    for attachment in color_attachments {
        flat.push(attachment.pixel_format);
    }
    Ok(flat)
}

impl MetalDevice {
    /// Compile a compute pipeline from a public `MTLComputePipelineDescriptor` wrapper.
    ///
    /// # Errors
    ///
    /// Returns Metal's localized pipeline compiler error on failure.
    pub fn new_compute_pipeline_state_with_descriptor(
        &self,
        descriptor: &ComputePipelineDescriptor<'_>,
    ) -> Result<ComputePipelineState, String> {
        let label = descriptor
            .label
            .and_then(|value| std::ffi::CString::new(value).ok());
        let label_ptr = label
            .as_deref()
            .map_or(core::ptr::null(), core::ffi::CStr::as_ptr);
        let mut err: *mut core::ffi::c_char = core::ptr::null_mut();
        let ptr = unsafe {
            ffi::am_device_new_compute_pipeline_state_with_descriptor(
                self.as_ptr(),
                descriptor.compute_function.as_ptr(),
                label_ptr,
                descriptor.thread_group_size_is_multiple_of_thread_execution_width,
                descriptor.max_total_threads_per_threadgroup,
                descriptor.support_indirect_command_buffers,
                &mut err,
            )
        };
        if ptr.is_null() {
            Err(unsafe {
                take_optional_string(err).unwrap_or_else(|| {
                    "MTLDevice.makeComputePipelineState(descriptor:) returned nil".to_string()
                })
            })
        } else {
            Ok(unsafe { ComputePipelineState::from_retained_ptr(ptr) })
        }
    }

    /// Compile a render pipeline from a public `MTLRenderPipelineDescriptor` wrapper.
    ///
    /// # Errors
    ///
    /// Returns Metal's localized pipeline compiler error on failure.
    pub fn new_render_pipeline_state_with_descriptor(
        &self,
        descriptor: &RenderPipelineDescriptor<'_>,
    ) -> Result<RenderPipelineState, String> {
        let color_attachments = flatten_render_color_attachments(descriptor.color_attachments)?;
        let label = descriptor
            .label
            .and_then(|value| std::ffi::CString::new(value).ok());
        let label_ptr = label
            .as_deref()
            .map_or(core::ptr::null(), core::ffi::CStr::as_ptr);
        let mut err: *mut core::ffi::c_char = core::ptr::null_mut();
        let ptr = unsafe {
            ffi::am_device_new_render_pipeline_state_with_descriptor(
                self.as_ptr(),
                descriptor.vertex_function.as_ptr(),
                descriptor
                    .fragment_function
                    .map_or(core::ptr::null_mut(), MetalFunction::as_ptr),
                label_ptr,
                descriptor.raster_sample_count,
                descriptor.alpha_to_coverage_enabled,
                descriptor.alpha_to_one_enabled,
                descriptor.rasterization_enabled,
                descriptor.support_indirect_command_buffers,
                descriptor.depth_attachment_pixel_format,
                descriptor.stencil_attachment_pixel_format,
                color_attachments.as_ptr(),
                descriptor.color_attachments.len(),
                &mut err,
            )
        };
        if ptr.is_null() {
            Err(unsafe {
                take_optional_string(err).unwrap_or_else(|| {
                    "MTLDevice.makeRenderPipelineState(descriptor:) returned nil".to_string()
                })
            })
        } else {
            Ok(unsafe { RenderPipelineState::from_retained_ptr(ptr) })
        }
    }

    /// Compile a tile render pipeline from a public `MTLTileRenderPipelineDescriptor` wrapper.
    ///
    /// # Errors
    ///
    /// Returns Metal's localized pipeline compiler error on failure.
    pub fn new_tile_render_pipeline_state(
        &self,
        descriptor: &TileRenderPipelineDescriptor<'_>,
    ) -> Result<RenderPipelineState, String> {
        let color_attachments = flatten_tile_color_attachments(descriptor.color_attachments)?;
        let label = descriptor
            .label
            .and_then(|value| std::ffi::CString::new(value).ok());
        let label_ptr = label
            .as_deref()
            .map_or(core::ptr::null(), core::ffi::CStr::as_ptr);
        let mut err: *mut core::ffi::c_char = core::ptr::null_mut();
        let ptr = unsafe {
            ffi::am_device_new_tile_render_pipeline_state(
                self.as_ptr(),
                descriptor.tile_function.as_ptr(),
                label_ptr,
                descriptor.raster_sample_count,
                descriptor.threadgroup_size_matches_tile_size,
                descriptor.max_total_threads_per_threadgroup,
                color_attachments.as_ptr(),
                descriptor.color_attachments.len(),
                &mut err,
            )
        };
        if ptr.is_null() {
            Err(unsafe {
                take_optional_string(err).unwrap_or_else(|| {
                    "MTLDevice.makeRenderPipelineState(tileDescriptor:) returned nil".to_string()
                })
            })
        } else {
            Ok(unsafe { RenderPipelineState::from_retained_ptr(ptr) })
        }
    }
}
