#![allow(clippy::missing_errors_doc)]

use crate::{ffi, storage_mode, MetalBuffer, MetalDevice, MetalTexture, SamplerState};
use core::ffi::c_void;
use std::sync::MutexGuard;

const DATA_TYPE_TEXTURE: usize = 58;
const DATA_TYPE_SAMPLER: usize = 59;
const DATA_TYPE_POINTER: usize = 60;
const DESCRIPTOR_WORD_COUNT: usize = 6;
const FIRST_CONSTANT_DATA_TYPE: usize = 3;
const LAST_CONSTANT_DATA_TYPE: usize = 56;
const FIRST_LONG_DATA_TYPE: usize = 81;
const LAST_LONG_DATA_TYPE: usize = 88;
const FIRST_BFLOAT_DATA_TYPE: usize = 121;
const LAST_BFLOAT_DATA_TYPE: usize = 124;

/// `MTLArgumentBuffersTier` enum values.
pub mod argument_buffers_tier {
    /// Mirrors the `Metal` framework constant `TIER1`.
    pub const TIER1: usize = 0;
    /// Mirrors the `Metal` framework constant `TIER2`.
    pub const TIER2: usize = 1;
}

/// `MTLBindingAccess` enum values.
pub mod binding_access {
    /// Mirrors the `Metal` framework constant `READ_ONLY`.
    pub const READ_ONLY: usize = 0;
    /// Mirrors the `Metal` framework constant `READ_WRITE`.
    pub const READ_WRITE: usize = 1;
    /// Mirrors the `Metal` framework constant `WRITE_ONLY`.
    pub const WRITE_ONLY: usize = 2;
}

/// `MTLTextureType` enum values.
pub mod texture_type {
    /// Mirrors the `Metal` framework constant `TYPE_1D`.
    pub const TYPE_1D: usize = 0;
    /// Mirrors the `Metal` framework constant `TYPE_1D_ARRAY`.
    pub const TYPE_1D_ARRAY: usize = 1;
    /// Mirrors the `Metal` framework constant `TYPE_2D`.
    pub const TYPE_2D: usize = 2;
    /// Mirrors the `Metal` framework constant `TYPE_2D_ARRAY`.
    pub const TYPE_2D_ARRAY: usize = 3;
    /// Mirrors the `Metal` framework constant `TYPE_2D_MULTISAMPLE`.
    pub const TYPE_2D_MULTISAMPLE: usize = 4;
    /// Mirrors the `Metal` framework constant `CUBE`.
    pub const CUBE: usize = 5;
    /// Mirrors the `Metal` framework constant `CUBE_ARRAY`.
    pub const CUBE_ARRAY: usize = 6;
    /// Mirrors the `Metal` framework constant `TYPE_3D`.
    pub const TYPE_3D: usize = 7;
    /// Mirrors the `Metal` framework constant `TYPE_2D_MULTISAMPLE_ARRAY`.
    pub const TYPE_2D_MULTISAMPLE_ARRAY: usize = 8;
    /// Mirrors the `Metal` framework constant `TEXTURE_BUFFER`.
    pub const TEXTURE_BUFFER: usize = 9;
}

/// Resource kind expected at an argument-buffer index.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArgumentBindingType {
    /// Buffer pointer binding.
    Buffer,
    /// Texture binding.
    Texture,
    /// Sampler binding.
    Sampler,
    /// Constant-data binding.
    Constant,
}

/// Errors returned while configuring an [`ArgumentEncoder`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArgumentEncoderError {
    /// The descriptor set is too large for the native ABI.
    DescriptorCountOutOfRange,
    /// At least one argument descriptor is required.
    EmptyDescriptorSet,
    /// The raw data type is not valid for descriptor-based argument encoding.
    UnsupportedDataType { data_type: usize },
    /// The raw binding-access value is invalid for this descriptor.
    InvalidAccess { access: usize },
    /// The raw texture-type value is invalid.
    InvalidTextureType { texture_type: usize },
    /// Buffer-pointer descriptors cannot declare an array length.
    BufferArrayUnsupported { array_length: usize },
    /// Constant-block alignment is invalid for this descriptor.
    InvalidConstantBlockAlignment { alignment: usize },
    /// A descriptor's binding range exceeds the supported index space.
    BindingRangeOutOfBounds { index: usize, array_length: usize },
    /// Two descriptors claim the same binding index.
    DuplicateBinding { index: usize },
    /// Metal could not create an argument encoder.
    NativeCreationFailed,
    /// Another CPU mapping panicked while holding the argument buffer lock.
    MappingLockPoisoned,
    /// The function-derived encoder does not expose type metadata.
    LayoutUnavailable,
    /// The binding index is not present in the encoder layout.
    InvalidBindingIndex { index: usize },
    /// The setter does not match the descriptor's resource type.
    BindingTypeMismatch {
        index: usize,
        expected: ArgumentBindingType,
        actual: ArgumentBindingType,
    },
    /// The destination argument-buffer offset is not suitably aligned.
    MisalignedArgumentBufferOffset { offset: usize, alignment: usize },
    /// The encoded argument range exceeds the destination buffer.
    ArgumentBufferRangeOutOfBounds {
        offset: usize,
        encoded_length: usize,
        buffer_length: usize,
    },
    /// The destination argument buffer is not CPU-addressable.
    CpuInaccessibleArgumentBuffer { storage_mode: usize },
    /// A referenced buffer offset exceeds that buffer's length.
    BufferOffsetOutOfBounds { offset: usize, buffer_length: usize },
    /// A value cannot be represented by the native API.
    IntegerOutOfRange { field: &'static str, value: usize },
    /// The native bridge rejected a validated setter.
    NativeRejected { operation: &'static str },
}

impl core::fmt::Display for ArgumentEncoderError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::DescriptorCountOutOfRange => {
                formatter.write_str("argument descriptor count exceeds native Int")
            }
            Self::EmptyDescriptorSet => {
                formatter.write_str("at least one argument descriptor is required")
            }
            Self::UnsupportedDataType { data_type } => {
                write!(
                    formatter,
                    "data type {data_type} is not valid for an argument descriptor"
                )
            }
            Self::InvalidAccess { access } => {
                write!(formatter, "binding access value {access} is invalid")
            }
            Self::InvalidTextureType { texture_type } => {
                write!(formatter, "texture type value {texture_type} is invalid")
            }
            Self::BufferArrayUnsupported { array_length } => write!(
                formatter,
                "buffer-pointer descriptor array length {array_length} is unsupported"
            ),
            Self::InvalidConstantBlockAlignment { alignment } => {
                write!(formatter, "constant-block alignment {alignment} is invalid")
            }
            Self::BindingRangeOutOfBounds {
                index,
                array_length,
            } => write!(
                formatter,
                "argument binding range {index}..{} is out of bounds",
                index.saturating_add(*array_length)
            ),
            Self::DuplicateBinding { index } => {
                write!(formatter, "argument binding index {index} is duplicated")
            }
            Self::NativeCreationFailed => {
                formatter.write_str("Metal could not create argument encoder")
            }
            Self::MappingLockPoisoned => {
                formatter.write_str("argument-buffer mapping lock is poisoned")
            }
            Self::LayoutUnavailable => formatter.write_str(
                "function-derived argument layout is unavailable; use an explicit unsafe setter",
            ),
            Self::InvalidBindingIndex { index } => {
                write!(formatter, "argument binding index {index} is not present")
            }
            Self::BindingTypeMismatch {
                index,
                expected,
                actual,
            } => write!(
                formatter,
                "argument binding {index} expects {expected:?}, not {actual:?}"
            ),
            Self::MisalignedArgumentBufferOffset { offset, alignment } => write!(
                formatter,
                "argument-buffer offset {offset} is not aligned to {alignment}"
            ),
            Self::ArgumentBufferRangeOutOfBounds {
                offset,
                encoded_length,
                buffer_length,
            } => write!(
                formatter,
                "encoded argument range {offset}..{} exceeds buffer length {buffer_length}",
                offset.saturating_add(*encoded_length)
            ),
            Self::CpuInaccessibleArgumentBuffer { storage_mode } => write!(
                formatter,
                "storage mode {storage_mode} cannot be used as an argument-encoder destination"
            ),
            Self::BufferOffsetOutOfBounds {
                offset,
                buffer_length,
            } => write!(
                formatter,
                "buffer offset {offset} exceeds buffer length {buffer_length}"
            ),
            Self::IntegerOutOfRange { field, value } => {
                write!(formatter, "{field} value {value} exceeds native Int")
            }
            Self::NativeRejected { operation } => write!(formatter, "Metal rejected {operation}"),
        }
    }
}

impl std::error::Error for ArgumentEncoderError {}

/// Safe Rust description of `MTLArgumentDescriptor`.
#[derive(Debug, Clone, Copy)]
pub struct ArgumentDescriptor {
    data_type: usize,
    index: usize,
    array_length: usize,
    access: usize,
    texture_type: usize,
    constant_block_alignment: usize,
}

impl ArgumentDescriptor {
    /// Describe a buffer pointer argument at `index`.
    #[must_use]
    pub const fn buffer(index: usize, access: usize) -> Self {
        Self {
            data_type: DATA_TYPE_POINTER,
            index,
            array_length: 0,
            access,
            texture_type: texture_type::TYPE_2D,
            constant_block_alignment: 0,
        }
    }

    /// Describe a texture argument at `index`.
    #[must_use]
    pub const fn texture(index: usize, texture_type: usize, access: usize) -> Self {
        Self {
            data_type: DATA_TYPE_TEXTURE,
            index,
            array_length: 0,
            access,
            texture_type,
            constant_block_alignment: 0,
        }
    }

    /// Describe a sampler argument at `index`.
    #[must_use]
    pub const fn sampler(index: usize) -> Self {
        Self {
            data_type: DATA_TYPE_SAMPLER,
            index,
            array_length: 0,
            access: binding_access::READ_ONLY,
            texture_type: texture_type::TYPE_2D,
            constant_block_alignment: 0,
        }
    }

    /// Describe a constant block argument using a raw `MTLDataType` value.
    #[must_use]
    pub const fn constant(data_type: usize, index: usize, array_length: usize) -> Self {
        Self {
            data_type,
            index,
            array_length,
            access: binding_access::READ_ONLY,
            texture_type: texture_type::TYPE_2D,
            constant_block_alignment: 0,
        }
    }

    /// Override the descriptor's array length.
    #[must_use]
    pub fn with_array_length(mut self, array_length: usize) -> Self {
        self.array_length = array_length;
        self
    }

    /// Override the descriptor's constant-block alignment.
    #[must_use]
    pub fn with_constant_block_alignment(mut self, alignment: usize) -> Self {
        self.constant_block_alignment = alignment;
        self
    }

    const fn as_words(self) -> [usize; DESCRIPTOR_WORD_COUNT] {
        [
            self.data_type,
            self.index,
            self.array_length,
            self.access,
            self.texture_type,
            self.constant_block_alignment,
        ]
    }

    const fn binding_type(self) -> ArgumentBindingType {
        match self.data_type {
            DATA_TYPE_POINTER => ArgumentBindingType::Buffer,
            DATA_TYPE_TEXTURE => ArgumentBindingType::Texture,
            DATA_TYPE_SAMPLER => ArgumentBindingType::Sampler,
            _ => ArgumentBindingType::Constant,
        }
    }
}

/// Apple's `id<MTLArgumentEncoder>` with an explicit active binding.
pub struct ArgumentEncoder {
    ptr: *mut c_void,
    layout: Option<Vec<ArgumentBindingRange>>,
}

#[derive(Clone, Copy)]
struct ArgumentBindingRange {
    start: usize,
    last: usize,
    binding_type: ArgumentBindingType,
}

/// Scoped active destination for argument-encoder setter operations.
pub struct ArgumentBufferBinding<'a> {
    encoder: &'a mut ArgumentEncoder,
    buffer: &'a MetalBuffer,
    offset: usize,
    encoded_length: usize,
    storage_mode: usize,
    _mapping_lock: MutexGuard<'a, ()>,
}

// SAFETY: the encoder may move between threads, but its mutating methods require
// exclusive access and it is intentionally not `Sync`.
unsafe impl Send for ArgumentEncoder {}

impl Drop for ArgumentEncoder {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::ametal_object_release(self.ptr) };
            self.ptr = core::ptr::null_mut();
        }
    }
}

impl MetalDevice {
    /// Create an argument encoder from explicit descriptors.
    ///
    /// # Errors
    ///
    /// Returns descriptor validation failures or native creation failure.
    pub fn new_argument_encoder_with_descriptors(
        &self,
        descriptors: &[ArgumentDescriptor],
    ) -> Result<ArgumentEncoder, ArgumentEncoderError> {
        let layout = build_layout(descriptors)?;
        let word_count = descriptors
            .len()
            .checked_mul(DESCRIPTOR_WORD_COUNT)
            .filter(|count| isize::try_from(*count).is_ok())
            .ok_or(ArgumentEncoderError::DescriptorCountOutOfRange)?;
        let mut words = Vec::with_capacity(word_count);
        for descriptor in descriptors {
            words.extend_from_slice(&descriptor.as_words());
        }
        let ptr = unsafe {
            ffi::ametal_device_new_argument_encoder_with_descriptors(
                self.as_ptr(),
                words.as_ptr(),
                descriptors.len(),
            )
        };
        if ptr.is_null() {
            Err(ArgumentEncoderError::NativeCreationFailed)
        } else {
            Ok(unsafe { ArgumentEncoder::from_descriptor_ptr(ptr, layout) })
        }
    }
}

impl ArgumentEncoder {
    /// Number of bytes required to encode the argument layout.
    #[must_use]
    pub fn encoded_length(&self) -> usize {
        unsafe { ffi::ametal_argument_encoder_encoded_length(self.as_ptr()) }
    }

    /// Required alignment for the encoded argument data.
    #[must_use]
    pub fn alignment(&self) -> usize {
        unsafe { ffi::ametal_argument_encoder_alignment(self.as_ptr()) }
    }

    /// Bind a destination argument buffer for a scoped sequence of setters.
    ///
    /// Managed storage is marked modified when the returned guard is dropped.
    ///
    /// # Safety
    ///
    /// The caller must exclude all GPU access to the encoded byte range until
    /// the returned binding guard is dropped, and must not submit GPU work that
    /// uses the argument buffer until then. Once a dispatch or draw references
    /// this argument buffer, its bindings must not change until that command
    /// buffer completes because resource declarations are captured while the
    /// command is encoded.
    pub unsafe fn bind_argument_buffer<'a>(
        &'a mut self,
        buffer: &'a MetalBuffer,
        offset: usize,
    ) -> Result<ArgumentBufferBinding<'a>, ArgumentEncoderError> {
        ensure_native_int(offset, "argument-buffer offset")?;
        let alignment = self.alignment();
        if alignment == 0 || offset % alignment != 0 {
            return Err(ArgumentEncoderError::MisalignedArgumentBufferOffset { offset, alignment });
        }
        let encoded_length = self.encoded_length();
        let end = offset.checked_add(encoded_length).ok_or_else(|| {
            ArgumentEncoderError::ArgumentBufferRangeOutOfBounds {
                offset,
                encoded_length,
                buffer_length: buffer.length(),
            }
        })?;
        if end > buffer.length() {
            return Err(ArgumentEncoderError::ArgumentBufferRangeOutOfBounds {
                offset,
                encoded_length,
                buffer_length: buffer.length(),
            });
        }
        let storage_mode = buffer.storage_mode();
        if !matches!(storage_mode, storage_mode::SHARED | storage_mode::MANAGED) {
            return Err(ArgumentEncoderError::CpuInaccessibleArgumentBuffer { storage_mode });
        }
        let mapping_lock = buffer
            .lock_mapping()
            .map_err(|_| ArgumentEncoderError::MappingLockPoisoned)?;
        let accepted = unsafe {
            ffi::ametal_argument_encoder_set_argument_buffer(self.as_ptr(), buffer.as_ptr(), offset)
        };
        if !accepted {
            return Err(ArgumentEncoderError::NativeRejected {
                operation: "argument-buffer binding",
            });
        }
        Ok(ArgumentBufferBinding {
            encoder: self,
            buffer,
            offset,
            encoded_length,
            storage_mode,
            _mapping_lock: mapping_lock,
        })
    }

    /// Borrowed raw `id<MTLArgumentEncoder>` pointer.
    ///
    /// The pointer is valid only while this wrapper is alive. Native setter
    /// calls through it bypass active-binding and layout validation.
    #[must_use]
    pub const fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }

    pub(crate) unsafe fn from_function_ptr(ptr: *mut c_void) -> Self {
        Self { ptr, layout: None }
    }

    unsafe fn from_descriptor_ptr(ptr: *mut c_void, layout: Vec<ArgumentBindingRange>) -> Self {
        Self {
            ptr,
            layout: Some(layout),
        }
    }

    fn validate_binding(
        &self,
        index: usize,
        actual: ArgumentBindingType,
    ) -> Result<(), ArgumentEncoderError> {
        validate_index(index)?;
        let layout = self
            .layout
            .as_ref()
            .ok_or(ArgumentEncoderError::LayoutUnavailable)?;
        let expected = layout
            .iter()
            .find(|range| index >= range.start && index <= range.last)
            .map(|range| range.binding_type)
            .ok_or(ArgumentEncoderError::InvalidBindingIndex { index })?;
        if expected == actual {
            Ok(())
        } else {
            Err(ArgumentEncoderError::BindingTypeMismatch {
                index,
                expected,
                actual,
            })
        }
    }
}

impl ArgumentBufferBinding<'_> {
    /// Encode a buffer binding at a descriptor-validated index.
    pub fn set_buffer(
        &mut self,
        buffer: &MetalBuffer,
        offset: usize,
        index: usize,
    ) -> Result<(), ArgumentEncoderError> {
        self.encoder
            .validate_binding(index, ArgumentBindingType::Buffer)?;
        unsafe { self.set_buffer_unchecked(buffer, offset, index) }
    }

    /// Encode a texture binding at a descriptor-validated index.
    pub fn set_texture(
        &mut self,
        texture: &MetalTexture,
        index: usize,
    ) -> Result<(), ArgumentEncoderError> {
        self.encoder
            .validate_binding(index, ArgumentBindingType::Texture)?;
        unsafe { self.set_texture_unchecked(texture, index) }
    }

    /// Encode a sampler binding at a descriptor-validated index.
    pub fn set_sampler_state(
        &mut self,
        sampler: &SamplerState,
        index: usize,
    ) -> Result<(), ArgumentEncoderError> {
        self.encoder
            .validate_binding(index, ArgumentBindingType::Sampler)?;
        unsafe { self.set_sampler_state_unchecked(sampler, index) }
    }

    /// Encode a buffer when the function-derived layout is known externally.
    ///
    /// # Safety
    ///
    /// `index` must identify a buffer binding in the actual function argument
    /// layout.
    pub unsafe fn set_buffer_unchecked(
        &mut self,
        buffer: &MetalBuffer,
        offset: usize,
        index: usize,
    ) -> Result<(), ArgumentEncoderError> {
        validate_index(index)?;
        ensure_native_int(offset, "buffer offset")?;
        if offset > buffer.length() {
            return Err(ArgumentEncoderError::BufferOffsetOutOfBounds {
                offset,
                buffer_length: buffer.length(),
            });
        }
        if ffi::ametal_argument_encoder_set_buffer(
            self.encoder.as_ptr(),
            buffer.as_ptr(),
            offset,
            index,
        ) {
            Ok(())
        } else {
            Err(ArgumentEncoderError::NativeRejected {
                operation: "argument buffer resource binding",
            })
        }
    }

    /// Encode a texture when the function-derived layout is known externally.
    ///
    /// # Safety
    ///
    /// `index` must identify a texture binding in the actual function argument
    /// layout.
    pub unsafe fn set_texture_unchecked(
        &mut self,
        texture: &MetalTexture,
        index: usize,
    ) -> Result<(), ArgumentEncoderError> {
        validate_index(index)?;
        if ffi::ametal_argument_encoder_set_texture(self.encoder.as_ptr(), texture.as_ptr(), index)
        {
            Ok(())
        } else {
            Err(ArgumentEncoderError::NativeRejected {
                operation: "argument texture binding",
            })
        }
    }

    /// Encode a sampler when the function-derived layout is known externally.
    ///
    /// # Safety
    ///
    /// `index` must identify a sampler binding in the actual function argument
    /// layout.
    pub unsafe fn set_sampler_state_unchecked(
        &mut self,
        sampler: &SamplerState,
        index: usize,
    ) -> Result<(), ArgumentEncoderError> {
        validate_index(index)?;
        if ffi::ametal_argument_encoder_set_sampler_state(
            self.encoder.as_ptr(),
            sampler.as_ptr(),
            index,
        ) {
            Ok(())
        } else {
            Err(ArgumentEncoderError::NativeRejected {
                operation: "argument sampler binding",
            })
        }
    }
}

impl Drop for ArgumentBufferBinding<'_> {
    fn drop(&mut self) {
        if self.storage_mode == storage_mode::MANAGED {
            unsafe {
                ffi::ametal_buffer_did_modify_range(
                    self.buffer.as_ptr(),
                    self.offset,
                    self.encoded_length,
                );
            }
        }
    }
}

fn build_layout(
    descriptors: &[ArgumentDescriptor],
) -> Result<Vec<ArgumentBindingRange>, ArgumentEncoderError> {
    if descriptors.is_empty() {
        return Err(ArgumentEncoderError::EmptyDescriptorSet);
    }
    if descriptors.len() > isize::MAX as usize {
        return Err(ArgumentEncoderError::DescriptorCountOutOfRange);
    }
    let mut layout = Vec::with_capacity(descriptors.len());
    for descriptor in descriptors {
        validate_descriptor(*descriptor)?;
        let occupied_binding_count = descriptor.array_length.max(1);
        let last = descriptor
            .index
            .checked_add(occupied_binding_count - 1)
            .filter(|last| isize::try_from(*last).is_ok())
            .ok_or(ArgumentEncoderError::BindingRangeOutOfBounds {
                index: descriptor.index,
                array_length: descriptor.array_length,
            })?;
        layout.push(ArgumentBindingRange {
            start: descriptor.index,
            last,
            binding_type: descriptor.binding_type(),
        });
    }
    layout.sort_unstable_by_key(|range| range.start);
    for ranges in layout.windows(2) {
        if ranges[1].start <= ranges[0].last {
            return Err(ArgumentEncoderError::DuplicateBinding {
                index: ranges[1].start,
            });
        }
    }
    Ok(layout)
}

fn validate_index(index: usize) -> Result<(), ArgumentEncoderError> {
    if isize::try_from(index).is_ok() {
        Ok(())
    } else {
        Err(ArgumentEncoderError::InvalidBindingIndex { index })
    }
}

fn validate_descriptor(descriptor: ArgumentDescriptor) -> Result<(), ArgumentEncoderError> {
    validate_index(descriptor.index)?;
    if descriptor.access > binding_access::WRITE_ONLY {
        return Err(ArgumentEncoderError::InvalidAccess {
            access: descriptor.access,
        });
    }
    match descriptor.binding_type() {
        ArgumentBindingType::Buffer => {
            if descriptor.array_length != 0 {
                return Err(ArgumentEncoderError::BufferArrayUnsupported {
                    array_length: descriptor.array_length,
                });
            }
            if descriptor.constant_block_alignment != 0 {
                return Err(ArgumentEncoderError::InvalidConstantBlockAlignment {
                    alignment: descriptor.constant_block_alignment,
                });
            }
        }
        ArgumentBindingType::Texture => {
            if descriptor.texture_type > texture_type::TEXTURE_BUFFER {
                return Err(ArgumentEncoderError::InvalidTextureType {
                    texture_type: descriptor.texture_type,
                });
            }
            if descriptor.constant_block_alignment != 0 {
                return Err(ArgumentEncoderError::InvalidConstantBlockAlignment {
                    alignment: descriptor.constant_block_alignment,
                });
            }
        }
        ArgumentBindingType::Sampler => {
            if descriptor.access != binding_access::READ_ONLY {
                return Err(ArgumentEncoderError::InvalidAccess {
                    access: descriptor.access,
                });
            }
            if descriptor.constant_block_alignment != 0 {
                return Err(ArgumentEncoderError::InvalidConstantBlockAlignment {
                    alignment: descriptor.constant_block_alignment,
                });
            }
        }
        ArgumentBindingType::Constant => {
            if !is_supported_constant_data_type(descriptor.data_type) {
                return Err(ArgumentEncoderError::UnsupportedDataType {
                    data_type: descriptor.data_type,
                });
            }
            if descriptor.access != binding_access::READ_ONLY {
                return Err(ArgumentEncoderError::InvalidAccess {
                    access: descriptor.access,
                });
            }
            let alignment = descriptor.constant_block_alignment;
            if alignment != 0
                && (!alignment.is_power_of_two() || isize::try_from(alignment).is_err())
            {
                return Err(ArgumentEncoderError::InvalidConstantBlockAlignment { alignment });
            }
        }
    }
    Ok(())
}

fn is_supported_constant_data_type(data_type: usize) -> bool {
    (FIRST_CONSTANT_DATA_TYPE..=LAST_CONSTANT_DATA_TYPE).contains(&data_type)
        || (FIRST_LONG_DATA_TYPE..=LAST_LONG_DATA_TYPE).contains(&data_type)
        || (FIRST_BFLOAT_DATA_TYPE..=LAST_BFLOAT_DATA_TYPE).contains(&data_type)
}

fn ensure_native_int(value: usize, field: &'static str) -> Result<(), ArgumentEncoderError> {
    if isize::try_from(value).is_ok() {
        Ok(())
    } else {
        Err(ArgumentEncoderError::IntegerOutOfRange { field, value })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modern_scalar_vector_types_are_supported() {
        for data_type in [81, 88, 121, 124] {
            assert!(is_supported_constant_data_type(data_type));
        }
    }
}
