use crate::{
    ffi, texture_usage, util::take_optional_string, MetalDevice, MetalFunction, TextureDescriptor,
};
use core::ffi::c_void;

/// `MTLPrimitiveType` enum values.
pub mod primitive_type {
    /// Mirrors the `Metal` framework constant `POINT`.
    pub const POINT: usize = 0;
    /// Mirrors the `Metal` framework constant `LINE`.
    pub const LINE: usize = 1;
    /// Mirrors the `Metal` framework constant `LINE_STRIP`.
    pub const LINE_STRIP: usize = 2;
    /// Mirrors the `Metal` framework constant `TRIANGLE`.
    pub const TRIANGLE: usize = 3;
    /// Mirrors the `Metal` framework constant `TRIANGLE_STRIP`.
    pub const TRIANGLE_STRIP: usize = 4;
}

/// `MTLLoadAction` enum values.
pub mod load_action {
    /// Mirrors the `Metal` framework constant `DONT_CARE`.
    pub const DONT_CARE: usize = 0;
    /// Mirrors the `Metal` framework constant `LOAD`.
    pub const LOAD: usize = 1;
    /// Mirrors the `Metal` framework constant `CLEAR`.
    pub const CLEAR: usize = 2;
}

/// `MTLStoreAction` enum values.
pub mod store_action {
    /// Mirrors the `Metal` framework constant `DONT_CARE`.
    pub const DONT_CARE: usize = 0;
    /// Mirrors the `Metal` framework constant `STORE`.
    pub const STORE: usize = 1;
    /// Mirrors the `Metal` framework constant `MULTISAMPLE_RESOLVE`.
    pub const MULTISAMPLE_RESOLVE: usize = 2;
    /// Mirrors the `Metal` framework constant `STORE_AND_MULTISAMPLE_RESOLVE`.
    pub const STORE_AND_MULTISAMPLE_RESOLVE: usize = 3;
}

#[allow(clippy::redundant_pub_crate)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RenderTargetFormats {
    pub(crate) colors: [usize; 8],
    pub(crate) depth: usize,
    pub(crate) stencil: usize,
    pub(crate) sample_count: usize,
}

impl RenderTargetFormats {
    pub(crate) const EMPTY: Self = Self {
        colors: [crate::pixel_format::INVALID; 8],
        depth: crate::pixel_format::INVALID,
        stencil: crate::pixel_format::INVALID,
        sample_count: 1,
    };

    pub(crate) fn validate_pipeline(self) -> Result<Self, String> {
        use crate::pixel_format::{
            color_bytes_per_pixel, is_depth_attachment_format, is_stencil_attachment_format,
            DEPTH24UNORM_STENCIL8, DEPTH32FLOAT_STENCIL8, INVALID,
        };

        if let Some(format) = self
            .colors
            .iter()
            .copied()
            .find(|format| *format != INVALID && color_bytes_per_pixel(*format).is_none())
        {
            return Err(format!(
                "pixel format {format} cannot be a render pipeline color attachment"
            ));
        }
        if self.depth != INVALID && !is_depth_attachment_format(self.depth) {
            return Err(format!(
                "pixel format {} cannot be a depth attachment format",
                self.depth
            ));
        }
        if self.stencil != INVALID && !is_stencil_attachment_format(self.stencil) {
            return Err(format!(
                "pixel format {} cannot be a stencil attachment format",
                self.stencil
            ));
        }
        let combined = |format| matches!(format, DEPTH24UNORM_STENCIL8 | DEPTH32FLOAT_STENCIL8);
        if self.depth != INVALID
            && self.stencil != INVALID
            && (combined(self.depth) || combined(self.stencil))
            && self.depth != self.stencil
        {
            return Err(
                "a combined depth/stencil format must be used for both depth and stencil"
                    .to_string(),
            );
        }
        if !matches!(self.sample_count, 1 | 2 | 4 | 8) {
            return Err(format!(
                "raster sample count {} is not 1, 2, 4 or 8",
                self.sample_count
            ));
        }
        Ok(self)
    }
}

/// Apple's `id<MTLRenderPipelineState>` — a compiled render pipeline.
pub struct RenderPipelineState {
    ptr: *mut c_void,
    targets: RenderTargetFormats,
    drawable: bool,
}

// SAFETY: `id<MTLRenderPipelineState>` is immutable after creation and
// thread-safe per Apple documentation.
unsafe impl Send for RenderPipelineState {}
unsafe impl Sync for RenderPipelineState {}

impl Drop for RenderPipelineState {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::ametal_object_release(self.ptr) };
            self.ptr = core::ptr::null_mut();
        }
    }
}

impl RenderPipelineState {
    /// Mirrors the `Metal` framework constant `fn`.
    #[must_use]
    pub const fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }

    pub(crate) const unsafe fn from_retained_ptr(
        ptr: *mut c_void,
        targets: RenderTargetFormats,
        drawable: bool,
    ) -> Self {
        Self {
            ptr,
            targets,
            drawable,
        }
    }

    pub(crate) const fn targets(&self) -> RenderTargetFormats {
        self.targets
    }

    pub(crate) const fn is_drawable(&self) -> bool {
        self.drawable
    }

    /// Metal's label for this pipeline, if one was set.
    #[must_use]
    pub fn label(&self) -> Option<String> {
        unsafe { take_optional_string(ffi::ametal_object_copy_label(self.ptr)) }
    }
}

impl MetalDevice {
    /// Compile a render pipeline state from `vertex` and `fragment` functions.
    ///
    /// # Errors
    ///
    /// Returns Metal's localized pipeline compiler error on failure.
    pub fn new_render_pipeline_state(
        &self,
        vertex: &MetalFunction,
        fragment: &MetalFunction,
        color_pixel_format: usize,
        sample_count: usize,
    ) -> Result<RenderPipelineState, String> {
        let mut colors = [crate::pixel_format::INVALID; 8];
        colors[0] = color_pixel_format;
        let targets = RenderTargetFormats {
            colors,
            depth: crate::pixel_format::INVALID,
            stencil: crate::pixel_format::INVALID,
            sample_count,
        }
        .validate_pipeline()?;
        if color_pixel_format == crate::pixel_format::INVALID {
            return Err("a render pipeline needs a color attachment format".to_string());
        }
        let mut err: *mut core::ffi::c_char = core::ptr::null_mut();
        let ptr = unsafe {
            ffi::ametal_device_new_render_pipeline_state(
                self.as_ptr(),
                vertex.as_ptr(),
                fragment.as_ptr(),
                color_pixel_format,
                sample_count,
                &raw mut err,
            )
        };
        if ptr.is_null() {
            Err(unsafe {
                take_optional_string(err)
                    .unwrap_or_else(|| "MTLDevice.makeRenderPipelineState returned nil".to_string())
            })
        } else {
            Ok(unsafe { RenderPipelineState::from_retained_ptr(ptr, targets, true) })
        }
    }
}

impl TextureDescriptor {
    /// Sensible defaults for an offscreen 2D render target texture.
    #[must_use]
    pub const fn render_target_2d(width: usize, height: usize, pixel_format: usize) -> Self {
        Self {
            pixel_format,
            width,
            height,
            mipmapped: false,
            usage: texture_usage::RENDER_TARGET | texture_usage::SHADER_READ,
            storage_mode: crate::storage_mode::PRIVATE,
            texture_type: crate::texture_type::TYPE_2D,
            depth: 1,
            array_length: 1,
            sample_count: 1,
        }
    }
}
