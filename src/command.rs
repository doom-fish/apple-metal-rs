#![allow(clippy::missing_errors_doc)]

use crate::{
    ffi, render::RenderTargetFormats, storage_mode, util::take_optional_string, CommandBuffer,
    CommandBufferPhase, CommandBufferState, CommandQueue, ComputePipelineState,
    CounterSampleBuffer, DepthStencilState, Event, Fence, MetalBuffer, MetalTexture,
    RenderPipelineState, SamplerState,
};
use core::ffi::{c_char, c_void, CStr};
use core::marker::PhantomData;
use core::mem::ManuallyDrop;
use core::ops::Range;
use doom_fish_utils::callback_context::CallbackContext;
use std::collections::HashSet;
use std::sync::{Mutex, PoisonError};

type CommandBufferHandler = Mutex<Option<Box<dyn FnOnce(Result<(), CommandBufferError>) + Send>>>;

const MAX_BUFFER_BINDINGS: usize = 31;
const MAX_TEXTURE_BINDINGS: usize = 128;
const MAX_SAMPLER_BINDINGS: usize = 16;

/// `MTLCommandBufferStatus` enum values.
pub mod command_buffer_status {
    /// Mirrors the `Metal` framework constant `NOT_ENQUEUED`.
    pub const NOT_ENQUEUED: usize = 0;
    /// Mirrors the `Metal` framework constant `ENQUEUED`.
    pub const ENQUEUED: usize = 1;
    /// Mirrors the `Metal` framework constant `COMMITTED`.
    pub const COMMITTED: usize = 2;
    /// Mirrors the `Metal` framework constant `SCHEDULED`.
    pub const SCHEDULED: usize = 3;
    /// Mirrors the `Metal` framework constant `COMPLETED`.
    pub const COMPLETED: usize = 4;
    /// Mirrors the `Metal` framework constant `ERROR`.
    pub const ERROR: usize = 5;
}

/// Errors returned for invalid command-buffer or encoder operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandBufferError {
    /// The shared lifecycle lock was poisoned.
    StateLockPoisoned,
    /// The operation is not valid in the command buffer's current state.
    InvalidState {
        operation: &'static str,
        state: &'static str,
    },
    /// A command encoder is still active on this command buffer.
    ActiveEncoder,
    /// The encoder has already ended.
    EncoderEnded,
    /// The native command encoder could not be created.
    EncoderCreationFailed {
        encoder: &'static str,
    },
    /// A resource byte range is invalid.
    RangeOutOfBounds {
        resource: &'static str,
        offset: usize,
        length: usize,
        resource_length: usize,
    },
    /// A range end precedes its start.
    InvalidRange,
    /// A binding index exceeds the supported table.
    InvalidBindingIndex {
        binding: &'static str,
        index: usize,
        limit: usize,
    },
    /// A dimension or offset cannot be represented by the native API.
    IntegerOutOfRange {
        field: &'static str,
        value: usize,
    },
    /// A dispatch dimension must be non-zero.
    EmptyDispatch {
        field: &'static str,
    },
    /// A synchronization operation requires managed storage.
    ManagedStorageRequired {
        storage_mode: usize,
    },
    /// Waiting after updating the same fence in one encoder is illegal.
    FenceWaitAfterUpdate,
    /// The native bridge rejected a validated operation.
    NativeRejected {
        operation: &'static str,
    },
    /// GPU execution completed with an error.
    ExecutionFailed(String),
    NotExecuted {
        status: usize,
    },
    MissingPipelineState,
    IncompatiblePipelineState,
    MissingAttachment {
        attachment: &'static str,
    },
    InvalidAttachment {
        attachment: &'static str,
    },
    InvalidPrimitiveType {
        primitive_type: usize,
    },
    ThreadgroupTooLarge {
        threads: usize,
        maximum: usize,
    },
    ThreadgroupNotMultipleOfExecutionWidth {
        threads: usize,
        execution_width: usize,
    },
}

impl core::fmt::Display for CommandBufferError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::StateLockPoisoned => formatter.write_str("command-buffer state lock is poisoned"),
            Self::InvalidState { operation, state } => {
                write!(
                    formatter,
                    "{operation} is invalid while command buffer is {state}"
                )
            }
            Self::ActiveEncoder => formatter.write_str("a command encoder is still active"),
            Self::EncoderEnded => formatter.write_str("the command encoder has already ended"),
            Self::EncoderCreationFailed { encoder } => {
                write!(
                    formatter,
                    "Metal could not create a {encoder} command encoder"
                )
            }
            Self::RangeOutOfBounds {
                resource,
                offset,
                length,
                resource_length,
            } => write!(
                formatter,
                "{resource} range {offset}..{} exceeds length {resource_length}",
                offset.saturating_add(*length)
            ),
            Self::InvalidRange => formatter.write_str("range end precedes range start"),
            Self::InvalidBindingIndex {
                binding,
                index,
                limit,
            } => write!(
                formatter,
                "{binding} binding index {index} is outside 0..{limit}"
            ),
            Self::IntegerOutOfRange { field, value } => {
                write!(formatter, "{field} value {value} exceeds native Int")
            }
            Self::EmptyDispatch { field } => {
                write!(formatter, "dispatch dimension {field} must be non-zero")
            }
            Self::ManagedStorageRequired { storage_mode } => {
                write!(
                    formatter,
                    "managed storage required, got mode {storage_mode}"
                )
            }
            Self::FenceWaitAfterUpdate => {
                formatter.write_str("cannot wait for a fence after updating it in the same encoder")
            }
            Self::NativeRejected { operation } => {
                write!(formatter, "Metal rejected {operation}")
            }
            Self::ExecutionFailed(message) => write!(formatter, "GPU execution failed: {message}"),
            Self::NotExecuted { status } => write!(
                formatter,
                "the command buffer was released without running (status {status})"
            ),
            Self::MissingPipelineState => {
                formatter.write_str("no pipeline state is bound to the encoder")
            }
            Self::IncompatiblePipelineState => formatter.write_str(
                "the pipeline's attachment formats or sample count differ from the render pass",
            ),
            Self::MissingAttachment { attachment } => write!(
                formatter,
                "the state uses the {attachment} attachment, which the render pass lacks"
            ),
            Self::InvalidAttachment { attachment } => {
                write!(formatter, "the {attachment} attachment cannot be rendered to")
            }
            Self::InvalidPrimitiveType { primitive_type } => {
                write!(formatter, "primitive type {primitive_type} is unknown")
            }
            Self::ThreadgroupTooLarge { threads, maximum } => write!(
                formatter,
                "{threads} threads per threadgroup exceed the pipeline maximum {maximum}"
            ),
            Self::ThreadgroupNotMultipleOfExecutionWidth {
                threads,
                execution_width,
            } => write!(
                formatter,
                "{threads} threads per threadgroup is not a multiple of the execution width {execution_width}"
            ),
        }
    }
}

impl std::error::Error for CommandBufferError {}

struct EncoderCore {
    ptr: *mut c_void,
    command_buffer: CommandBuffer,
    ended: bool,
    updated_fences: HashSet<usize>,
}

impl EncoderCore {
    fn new(ptr: *mut c_void, command_buffer: CommandBuffer) -> Self {
        Self {
            ptr,
            command_buffer,
            ended: false,
            updated_fences: HashSet::new(),
        }
    }

    fn with_active<T>(
        &self,
        operation: &'static str,
        encode: impl FnOnce(*mut c_void) -> T,
    ) -> Result<T, CommandBufferError> {
        if self.ended {
            return Err(CommandBufferError::EncoderEnded);
        }
        let state = self
            .command_buffer
            .inner
            .state
            .lock()
            .map_err(|_| CommandBufferError::StateLockPoisoned)?;
        ensure_recording(state.phase, operation)?;
        if !state.active_encoder {
            return Err(CommandBufferError::EncoderEnded);
        }
        drop(state);
        Ok(encode(self.ptr))
    }

    fn finish(&mut self) -> Result<(), CommandBufferError> {
        if self.ended {
            return Err(CommandBufferError::EncoderEnded);
        }
        let mut state = self
            .command_buffer
            .inner
            .state
            .lock()
            .map_err(|_| CommandBufferError::StateLockPoisoned)?;
        ensure_recording(state.phase, "end_encoding")?;
        if !state.active_encoder {
            return Err(CommandBufferError::EncoderEnded);
        }
        unsafe { ffi::ametal_command_encoder_end_encoding(self.ptr) };
        state.active_encoder = false;
        drop(state);
        self.ended = true;
        Ok(())
    }

    fn finish_on_drop(&mut self) {
        if self.ended {
            return;
        }
        let mut state = self
            .command_buffer
            .inner
            .state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if matches!(
            state.phase,
            CommandBufferPhase::Recording | CommandBufferPhase::Enqueued
        ) && state.active_encoder
        {
            unsafe { ffi::ametal_command_encoder_end_encoding(self.ptr) };
            state.active_encoder = false;
        }
        drop(state);
        self.ended = true;
    }

    fn record_fence_update(&mut self, fence: &Fence) {
        self.updated_fences.insert(fence.as_ptr() as usize);
    }

    fn ensure_fence_wait_allowed(&self, fence: &Fence) -> Result<(), CommandBufferError> {
        if self.updated_fences.contains(&(fence.as_ptr() as usize)) {
            Err(CommandBufferError::FenceWaitAfterUpdate)
        } else {
            Ok(())
        }
    }
}

impl Drop for EncoderCore {
    fn drop(&mut self) {
        self.finish_on_drop();
        if !self.ptr.is_null() {
            unsafe { ffi::ametal_object_release(self.ptr) };
            self.ptr = core::ptr::null_mut();
        }
    }
}

macro_rules! command_encoder {
    ($(#[$meta:meta])* pub struct $name:ident { $($field:ident: $ty:ty = $init:expr),* $(,)? }) => {
        $(#[$meta])*
        pub struct $name {
            core: EncoderCore,
            $($field: $ty,)*
        }

        impl $name {
            fn new(ptr: *mut c_void, command_buffer: CommandBuffer) -> Self {
                Self {
                    core: EncoderCore::new(ptr, command_buffer),
                    $($field: $init,)*
                }
            }

            /// Borrowed raw native command-encoder pointer.
            ///
            /// The pointer is valid only until this wrapper is dropped. Calling
            /// `endEncoding` through it bypasses lifecycle tracking.
            #[must_use]
            pub fn as_ptr(&self) -> *mut c_void {
                self.core.ptr
            }

            /// Finish encoding. Dropping an active encoder performs this once
            /// automatically.
            pub fn end_encoding(mut self) -> Result<(), CommandBufferError> {
                self.core.finish()
            }
        }
    };
}

command_encoder!(
    /// Apple's `id<MTLBlitCommandEncoder>` — encodes buffer and texture copy work.
    pub struct BlitCommandEncoder {}
);
command_encoder!(
    /// Apple's `id<MTLComputeCommandEncoder>` — encodes compute dispatches.
    pub struct ComputeCommandEncoder {
        pipeline: Option<ComputeLimits> = None,
    }
);
command_encoder!(
    /// Apple's `id<MTLRenderCommandEncoder>` — encodes render passes.
    pub struct RenderCommandEncoder {
        targets: RenderTargetFormats = RenderTargetFormats::EMPTY,
        pipeline_bound: bool = false,
    }
);

pub struct ForeignEncoding<'a> {
    command_buffer: *mut c_void,
    _command_buffer: PhantomData<&'a CommandBuffer>,
}

impl ForeignEncoding<'_> {
    #[must_use]
    pub const fn command_buffer(&self) -> *mut c_void {
        self.command_buffer
    }
}

#[derive(Clone, Copy)]
struct ComputeLimits {
    execution_width: usize,
    max_threads: usize,
    multiple_of_execution_width: bool,
}

#[derive(Clone, Copy)]
pub struct RenderPassDepthAttachment<'a> {
    pub texture: &'a MetalTexture,
    pub load_action: usize,
    pub store_action: usize,
    pub clear_depth: f64,
}

#[derive(Clone, Copy)]
pub struct RenderPassStencilAttachment<'a> {
    pub texture: &'a MetalTexture,
    pub load_action: usize,
    pub store_action: usize,
    pub clear_stencil: u32,
}

impl CommandQueue {
    /// Create a command buffer whose native object does not retain references.
    ///
    /// # Safety
    ///
    /// Every object referenced directly or indirectly by encoded commands must
    /// remain alive until the command buffer reaches `COMPLETED` or `ERROR`.
    /// Prefer [`Self::new_command_buffer`] unless the caller owns that lifetime
    /// protocol.
    #[must_use]
    pub unsafe fn new_command_buffer_with_unretained_references(&self) -> Option<CommandBuffer> {
        let ptr =
            ffi::ametal_command_queue_new_command_buffer_with_unretained_references(self.as_ptr());
        if ptr.is_null() {
            None
        } else {
            Some(CommandBuffer::from_retained_ptr(ptr))
        }
    }
}

impl CommandBuffer {
    /// Enqueue the command buffer on its queue without committing it.
    pub fn enqueue(&self) -> Result<(), CommandBufferError> {
        let mut state = self
            .inner
            .state
            .lock()
            .map_err(|_| CommandBufferError::StateLockPoisoned)?;
        if state.phase != CommandBufferPhase::Recording {
            return Err(invalid_state("enqueue", state.phase));
        }
        self.ensure_natively_recording(&mut state, "enqueue")?;
        if state.active_encoder {
            return Err(CommandBufferError::ActiveEncoder);
        }
        unsafe { ffi::ametal_command_buffer_enqueue(self.as_ptr()) };
        state.phase = CommandBufferPhase::Enqueued;
        drop(state);
        Ok(())
    }

    /// Submit the recorded commands for execution.
    pub fn commit(&self) -> Result<(), CommandBufferError> {
        let mut state = self
            .inner
            .state
            .lock()
            .map_err(|_| CommandBufferError::StateLockPoisoned)?;
        ensure_recording(state.phase, "commit")?;
        self.ensure_natively_recording(&mut state, "commit")?;
        if state.active_encoder {
            return Err(CommandBufferError::ActiveEncoder);
        }
        unsafe { ffi::ametal_command_buffer_commit(self.as_ptr()) };
        state.phase = CommandBufferPhase::Committed;
        drop(state);
        Ok(())
    }

    /// Block until Metal schedules this committed command buffer.
    pub fn wait_until_scheduled(&self) -> Result<(), CommandBufferError> {
        let state = self
            .inner
            .state
            .lock()
            .map_err(|_| CommandBufferError::StateLockPoisoned)?;
        match state.phase {
            CommandBufferPhase::Completed => return Ok(()),
            CommandBufferPhase::Error => return Err(self.execution_error()),
            CommandBufferPhase::Committed => {}
            phase => return Err(invalid_state("wait_until_scheduled", phase)),
        }
        drop(state);
        unsafe { ffi::ametal_command_buffer_wait_until_scheduled(self.as_ptr()) };
        Ok(())
    }

    /// Block until all submitted commands finish.
    pub fn wait_until_completed(&self) -> Result<(), CommandBufferError> {
        {
            let state = self
                .inner
                .state
                .lock()
                .map_err(|_| CommandBufferError::StateLockPoisoned)?;
            match state.phase {
                CommandBufferPhase::Completed => return Ok(()),
                CommandBufferPhase::Error => return Err(self.execution_error()),
                CommandBufferPhase::Committed => {}
                phase => return Err(invalid_state("wait_until_completed", phase)),
            }
        }
        unsafe { ffi::ametal_command_buffer_wait_until_completed(self.as_ptr()) };
        let status = unsafe { ffi::ametal_command_buffer_status(self.as_ptr()) };
        let mut state = self
            .inner
            .state
            .lock()
            .map_err(|_| CommandBufferError::StateLockPoisoned)?;
        if status == command_buffer_status::ERROR {
            state.phase = CommandBufferPhase::Error;
            drop(state);
            Err(self.execution_error())
        } else {
            state.phase = CommandBufferPhase::Completed;
            drop(state);
            Ok(())
        }
    }

    /// Current `MTLCommandBufferStatus` value.
    #[must_use]
    pub fn status(&self) -> usize {
        let status = unsafe { ffi::ametal_command_buffer_status(self.as_ptr()) };
        let mut state = self
            .inner
            .state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        match status {
            command_buffer_status::COMPLETED => state.phase = CommandBufferPhase::Completed,
            command_buffer_status::ERROR => state.phase = CommandBufferPhase::Error,
            _ => {}
        }
        status
    }

    pub fn add_scheduled_handler<F>(&self, handler: F) -> Result<(), CommandBufferError>
    where
        F: FnOnce(Result<(), CommandBufferError>) + Send + 'static,
    {
        self.add_handler("add_scheduled_handler", false, Box::new(handler))
    }

    pub fn add_completed_handler<F>(&self, handler: F) -> Result<(), CommandBufferError>
    where
        F: FnOnce(Result<(), CommandBufferError>) + Send + 'static,
    {
        self.add_handler("add_completed_handler", true, Box::new(handler))
    }

    fn add_handler(
        &self,
        operation: &'static str,
        completed: bool,
        handler: Box<dyn FnOnce(Result<(), CommandBufferError>) + Send>,
    ) -> Result<(), CommandBufferError> {
        let mut state = self
            .inner
            .state
            .lock()
            .map_err(|_| CommandBufferError::StateLockPoisoned)?;
        ensure_recording(state.phase, operation)?;
        self.ensure_natively_recording(&mut state, operation)?;
        let context = ManuallyDrop::new(CallbackContext::<CommandBufferHandler>::new(Mutex::new(
            Some(handler),
        )));
        let accepted = unsafe {
            ffi::ametal_command_buffer_add_handler(
                self.as_ptr(),
                completed,
                context.as_ptr(),
                Some(command_buffer_handler_trampoline),
                Some(CallbackContext::<CommandBufferHandler>::RELEASE),
            )
        };
        drop(state);
        if accepted {
            Ok(())
        } else {
            Err(CommandBufferError::NativeRejected { operation })
        }
    }

    /// Localized Metal error string for a failed command buffer.
    #[must_use]
    pub fn error(&self) -> Option<String> {
        unsafe { take_optional_string(ffi::ametal_command_buffer_error_message(self.as_ptr())) }
    }

    /// Create a standalone blit command encoder.
    pub fn new_blit_command_encoder(&self) -> Result<BlitCommandEncoder, CommandBufferError> {
        let core = self.begin_encoder("blit", || unsafe {
            ffi::ametal_command_buffer_new_blit_command_encoder(self.as_ptr())
        })?;
        Ok(BlitCommandEncoder::new(core, self.clone()))
    }

    /// Create a standalone compute command encoder.
    pub fn new_compute_command_encoder(&self) -> Result<ComputeCommandEncoder, CommandBufferError> {
        let core = self.begin_encoder("compute", || unsafe {
            ffi::ametal_command_buffer_new_compute_command_encoder(self.as_ptr())
        })?;
        Ok(ComputeCommandEncoder::new(core, self.clone()))
    }

    /// Create a render command encoder that renders into `texture`.
    pub fn new_render_command_encoder(
        &self,
        texture: &MetalTexture,
        load_action: usize,
        store_action: usize,
        clear_color: [f64; 4],
        depth: Option<RenderPassDepthAttachment<'_>>,
        stencil: Option<RenderPassStencilAttachment<'_>>,
    ) -> Result<RenderCommandEncoder, CommandBufferError> {
        let targets = render_targets(
            texture,
            load_action,
            store_action,
            depth.as_ref(),
            stencil.as_ref(),
        )?;
        let core = self.begin_encoder("render", || unsafe {
            ffi::ametal_command_buffer_new_render_command_encoder(
                self.as_ptr(),
                texture.as_ptr(),
                load_action,
                store_action,
                clear_color[0],
                clear_color[1],
                clear_color[2],
                clear_color[3],
                depth.map_or(core::ptr::null_mut(), |depth| depth.texture.as_ptr()),
                depth.map_or(0, |depth| depth.load_action),
                depth.map_or(0, |depth| depth.store_action),
                depth.map_or(1.0, |depth| depth.clear_depth),
                stencil.map_or(core::ptr::null_mut(), |stencil| stencil.texture.as_ptr()),
                stencil.map_or(0, |stencil| stencil.load_action),
                stencil.map_or(0, |stencil| stencil.store_action),
                stencil.map_or(0, |stencil| stencil.clear_stencil),
            )
        })?;
        let mut encoder = RenderCommandEncoder::new(core, self.clone());
        encoder.targets = targets;
        Ok(encoder)
    }

    /// Encode a wait until `event` reaches at least `value`.
    pub fn encode_wait_for_event(
        &self,
        event: &Event,
        value: u64,
    ) -> Result<(), CommandBufferError> {
        self.encode_without_encoder("encode_wait_for_event", || unsafe {
            ffi::ametal_command_buffer_encode_wait_for_event(self.as_ptr(), event.as_ptr(), value);
        })
    }

    /// Encode a signal that updates `event` to `value`.
    pub fn encode_signal_event(&self, event: &Event, value: u64) -> Result<(), CommandBufferError> {
        self.encode_without_encoder("encode_signal_event", || unsafe {
            ffi::ametal_command_buffer_encode_signal_event(self.as_ptr(), event.as_ptr(), value);
        })
    }

    /// Record a blit copy from `src` into `dst`.
    pub fn blit_copy_buffer(
        &self,
        src: &MetalBuffer,
        src_offset: usize,
        dst: &MetalBuffer,
        dst_offset: usize,
        size: usize,
    ) -> Result<(), CommandBufferError> {
        let mut encoder = self.new_blit_command_encoder()?;
        encoder.copy_buffer(src, src_offset, dst, dst_offset, size)?;
        encoder.end_encoding()
    }

    /// Record a one-dimensional compute dispatch.
    pub fn dispatch_compute_1d(
        &self,
        pipeline: &ComputePipelineState,
        buffers: &[&MetalBuffer],
        threadgroups: usize,
        threads_per_group: usize,
    ) -> Result<(), CommandBufferError> {
        let mut encoder = self.new_compute_command_encoder()?;
        encoder.set_compute_pipeline_state(pipeline)?;
        for (index, buffer) in buffers.iter().enumerate() {
            encoder.set_buffer(buffer, 0, index)?;
        }
        encoder.dispatch_threadgroups((threadgroups, 1, 1), (threads_per_group, 1, 1))?;
        encoder.end_encoding()
    }

    pub(crate) fn encode_without_encoder(
        &self,
        operation: &'static str,
        encode: impl FnOnce(),
    ) -> Result<(), CommandBufferError> {
        let mut state = self
            .inner
            .state
            .lock()
            .map_err(|_| CommandBufferError::StateLockPoisoned)?;
        ensure_recording(state.phase, operation)?;
        if state.active_encoder {
            return Err(CommandBufferError::ActiveEncoder);
        }
        self.ensure_natively_recording(&mut state, operation)?;
        encode();
        drop(state);
        Ok(())
    }

    pub fn encode_foreign<R>(
        &self,
        encode: impl FnOnce(&ForeignEncoding<'_>) -> R,
    ) -> Result<R, CommandBufferError> {
        {
            let mut state = self
                .inner
                .state
                .lock()
                .map_err(|_| CommandBufferError::StateLockPoisoned)?;
            ensure_recording(state.phase, "encode_foreign")?;
            if state.active_encoder {
                return Err(CommandBufferError::ActiveEncoder);
            }
            self.ensure_natively_recording(&mut state, "encode_foreign")?;
            state.active_encoder = true;
        }
        let result = encode(&ForeignEncoding {
            command_buffer: self.as_ptr(),
            _command_buffer: PhantomData,
        });
        let mut state = self
            .inner
            .state
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        state.active_encoder = false;
        let _ = self.ensure_natively_recording(&mut state, "encode_foreign");
        drop(state);
        Ok(result)
    }

    fn ensure_natively_recording(
        &self,
        state: &mut CommandBufferState,
        operation: &'static str,
    ) -> Result<(), CommandBufferError> {
        match unsafe { ffi::ametal_command_buffer_status(self.as_ptr()) } {
            command_buffer_status::NOT_ENQUEUED | command_buffer_status::ENQUEUED => Ok(()),
            status => {
                state.phase = match status {
                    command_buffer_status::COMPLETED => CommandBufferPhase::Completed,
                    command_buffer_status::ERROR => CommandBufferPhase::Error,
                    _ => CommandBufferPhase::Committed,
                };
                Err(invalid_state(operation, state.phase))
            }
        }
    }

    fn begin_encoder(
        &self,
        encoder: &'static str,
        create: impl FnOnce() -> *mut c_void,
    ) -> Result<*mut c_void, CommandBufferError> {
        let mut state = self
            .inner
            .state
            .lock()
            .map_err(|_| CommandBufferError::StateLockPoisoned)?;
        ensure_recording(state.phase, "create command encoder")?;
        if state.active_encoder {
            return Err(CommandBufferError::ActiveEncoder);
        }
        self.ensure_natively_recording(&mut state, "create command encoder")?;
        let pointer = create();
        if pointer.is_null() {
            return Err(CommandBufferError::EncoderCreationFailed { encoder });
        }
        state.active_encoder = true;
        drop(state);
        Ok(pointer)
    }

    fn execution_error(&self) -> CommandBufferError {
        let message = unsafe {
            take_optional_string(ffi::ametal_command_buffer_error_message(self.as_ptr()))
        }
        .unwrap_or_else(|| "Metal reported an unspecified command-buffer error".to_string());
        CommandBufferError::ExecutionFailed(message)
    }
}

impl BlitCommandEncoder {
    /// Copy `size` bytes from `src` into `dst`.
    pub fn copy_buffer(
        &mut self,
        src: &MetalBuffer,
        src_offset: usize,
        dst: &MetalBuffer,
        dst_offset: usize,
        size: usize,
    ) -> Result<(), CommandBufferError> {
        checked_resource_range("source buffer", src_offset, size, src.length())?;
        checked_resource_range("destination buffer", dst_offset, size, dst.length())?;
        ensure_native_int(src_offset, "source offset")?;
        ensure_native_int(dst_offset, "destination offset")?;
        ensure_native_int(size, "copy size")?;
        let accepted = self.core.with_active("copy_buffer", |encoder| unsafe {
            ffi::ametal_blit_command_encoder_copy_buffer(
                encoder,
                src.as_ptr(),
                src_offset,
                dst.as_ptr(),
                dst_offset,
                size,
            )
        })?;
        if accepted {
            Ok(())
        } else {
            Err(CommandBufferError::NativeRejected {
                operation: "buffer copy",
            })
        }
    }

    /// Fill a byte range of `buffer` with `value`.
    pub fn fill_buffer(
        &mut self,
        buffer: &MetalBuffer,
        range: Range<usize>,
        value: u8,
    ) -> Result<(), CommandBufferError> {
        if range.start > range.end {
            return Err(CommandBufferError::InvalidRange);
        }
        let length = range.end - range.start;
        checked_resource_range("buffer", range.start, length, buffer.length())?;
        ensure_native_int(range.start, "fill offset")?;
        ensure_native_int(length, "fill length")?;
        let accepted = self.core.with_active("fill_buffer", |encoder| unsafe {
            ffi::ametal_blit_command_encoder_fill_buffer(
                encoder,
                buffer.as_ptr(),
                range.start,
                length,
                value,
            )
        })?;
        if accepted {
            Ok(())
        } else {
            Err(CommandBufferError::NativeRejected {
                operation: "buffer fill",
            })
        }
    }

    /// Sample hardware counters into `sample_buffer`.
    pub fn sample_counters(
        &mut self,
        sample_buffer: &CounterSampleBuffer,
        sample_index: usize,
        barrier: bool,
    ) -> Result<(), CommandBufferError> {
        if sample_index >= sample_buffer.sample_count() {
            return Err(CommandBufferError::InvalidBindingIndex {
                binding: "counter sample",
                index: sample_index,
                limit: sample_buffer.sample_count(),
            });
        }
        ensure_native_int(sample_index, "sample index")?;
        let accepted = self.core.with_active("sample_counters", |encoder| unsafe {
            ffi::ametal_blit_command_encoder_sample_counters(
                encoder,
                sample_buffer.as_ptr(),
                sample_index,
                barrier,
            )
        })?;
        if accepted {
            Ok(())
        } else {
            Err(CommandBufferError::NativeRejected {
                operation: "counter sampling",
            })
        }
    }

    /// Make managed resource writes visible to the CPU after completion.
    pub fn synchronize_resource(&mut self, buffer: &MetalBuffer) -> Result<(), CommandBufferError> {
        synchronize_resource(&self.core, buffer.as_ptr(), buffer.storage_mode())
    }

    /// Make managed texture writes visible to the CPU after completion.
    pub fn synchronize_texture(
        &mut self,
        texture: &MetalTexture,
    ) -> Result<(), CommandBufferError> {
        synchronize_resource(&self.core, texture.as_ptr(), texture.storage_mode())
    }

    /// Update `fence` with work encoded so far.
    pub fn update_fence(&mut self, fence: &Fence) -> Result<(), CommandBufferError> {
        self.core.with_active("update_fence", |encoder| unsafe {
            ffi::ametal_blit_command_encoder_update_fence(encoder, fence.as_ptr());
        })?;
        self.core.record_fence_update(fence);
        Ok(())
    }

    /// Wait for `fence` before executing subsequent work.
    pub fn wait_for_fence(&mut self, fence: &Fence) -> Result<(), CommandBufferError> {
        self.core.ensure_fence_wait_allowed(fence)?;
        self.core.with_active("wait_for_fence", |encoder| unsafe {
            ffi::ametal_blit_command_encoder_wait_for_fence(encoder, fence.as_ptr());
        })
    }
}

impl ComputeCommandEncoder {
    /// Bind a compute pipeline state.
    pub fn set_compute_pipeline_state(
        &mut self,
        pipeline: &ComputePipelineState,
    ) -> Result<(), CommandBufferError> {
        self.core
            .with_active("set_compute_pipeline_state", |encoder| unsafe {
                ffi::ametal_compute_command_encoder_set_pipeline_state(encoder, pipeline.as_ptr());
            })?;
        self.pipeline = Some(ComputeLimits {
            execution_width: pipeline.thread_execution_width(),
            max_threads: pipeline.max_total_threads_per_threadgroup(),
            multiple_of_execution_width: pipeline.threadgroup_multiple_of_execution_width(),
        });
        Ok(())
    }

    fn validate_threadgroup(
        &self,
        threads_per_threadgroup: (usize, usize, usize),
    ) -> Result<(), CommandBufferError> {
        let limits = self
            .pipeline
            .ok_or(CommandBufferError::MissingPipelineState)?;
        let threads = threads_per_threadgroup
            .0
            .checked_mul(threads_per_threadgroup.1)
            .and_then(|threads| threads.checked_mul(threads_per_threadgroup.2))
            .ok_or(CommandBufferError::ThreadgroupTooLarge {
                threads: usize::MAX,
                maximum: limits.max_threads,
            })?;
        if threads > limits.max_threads {
            return Err(CommandBufferError::ThreadgroupTooLarge {
                threads,
                maximum: limits.max_threads,
            });
        }
        if limits.multiple_of_execution_width
            && (limits.execution_width == 0 || threads % limits.execution_width != 0)
        {
            return Err(CommandBufferError::ThreadgroupNotMultipleOfExecutionWidth {
                threads,
                execution_width: limits.execution_width,
            });
        }
        Ok(())
    }

    /// Bind a buffer at `index`.
    pub fn set_buffer(
        &mut self,
        buffer: &MetalBuffer,
        offset: usize,
        index: usize,
    ) -> Result<(), CommandBufferError> {
        validate_binding_index("buffer", index, MAX_BUFFER_BINDINGS)?;
        checked_resource_range("buffer", offset, 0, buffer.length())?;
        ensure_native_int(offset, "buffer offset")?;
        self.core.with_active("set_buffer", |encoder| unsafe {
            ffi::ametal_compute_command_encoder_set_buffer(encoder, buffer.as_ptr(), offset, index);
        })
    }

    /// Bind a texture at `index`.
    pub fn set_texture(
        &mut self,
        texture: &MetalTexture,
        index: usize,
    ) -> Result<(), CommandBufferError> {
        validate_binding_index("texture", index, MAX_TEXTURE_BINDINGS)?;
        self.core.with_active("set_texture", |encoder| unsafe {
            ffi::ametal_compute_command_encoder_set_texture(encoder, texture.as_ptr(), index);
        })
    }

    /// Bind a sampler state at `index`.
    pub fn set_sampler_state(
        &mut self,
        sampler: &SamplerState,
        index: usize,
    ) -> Result<(), CommandBufferError> {
        validate_binding_index("sampler", index, MAX_SAMPLER_BINDINGS)?;
        self.core
            .with_active("set_sampler_state", |encoder| unsafe {
                ffi::ametal_compute_command_encoder_set_sampler_state(
                    encoder,
                    sampler.as_ptr(),
                    index,
                );
            })
    }

    /// Bind a visible function table at `index`.
    pub fn set_visible_function_table(
        &mut self,
        table: &crate::VisibleFunctionTable,
        index: usize,
    ) -> Result<(), CommandBufferError> {
        validate_binding_index("visible function table", index, MAX_BUFFER_BINDINGS)?;
        self.core
            .with_active("set_visible_function_table", |encoder| unsafe {
                ffi::ametal_compute_command_encoder_set_visible_function_table(
                    encoder,
                    table.as_ptr(),
                    index,
                );
            })
    }

    /// Bind an intersection function table at `index`.
    pub fn set_intersection_function_table(
        &mut self,
        table: &crate::IntersectionFunctionTable,
        index: usize,
    ) -> Result<(), CommandBufferError> {
        validate_binding_index("intersection function table", index, MAX_BUFFER_BINDINGS)?;
        self.core
            .with_active("set_intersection_function_table", |encoder| unsafe {
                ffi::ametal_compute_command_encoder_set_intersection_function_table(
                    encoder,
                    table.as_ptr(),
                    index,
                );
            })
    }

    /// Bind an acceleration structure at `index`.
    pub fn set_acceleration_structure(
        &mut self,
        acceleration_structure: &crate::AccelerationStructure,
        index: usize,
    ) -> Result<(), CommandBufferError> {
        validate_binding_index("acceleration structure", index, MAX_BUFFER_BINDINGS)?;
        self.core
            .with_active("set_acceleration_structure", |encoder| unsafe {
                ffi::ametal_compute_command_encoder_set_acceleration_structure(
                    encoder,
                    acceleration_structure.as_ptr(),
                    index,
                );
            })
    }

    /// Dispatch threadgroups of fixed size.
    pub fn dispatch_threadgroups(
        &mut self,
        threadgroups: (usize, usize, usize),
        threads_per_threadgroup: (usize, usize, usize),
    ) -> Result<(), CommandBufferError> {
        validate_size(threadgroups, "threadgroup")?;
        validate_size(threads_per_threadgroup, "threads-per-threadgroup")?;
        self.validate_threadgroup(threads_per_threadgroup)?;
        self.core
            .with_active("dispatch_threadgroups", |encoder| unsafe {
                ffi::ametal_compute_command_encoder_dispatch_threadgroups(
                    encoder,
                    threadgroups.0,
                    threadgroups.1,
                    threadgroups.2,
                    threads_per_threadgroup.0,
                    threads_per_threadgroup.1,
                    threads_per_threadgroup.2,
                );
            })
    }

    /// Dispatch an arbitrary thread grid.
    pub fn dispatch_threads(
        &mut self,
        threads: (usize, usize, usize),
        threads_per_threadgroup: (usize, usize, usize),
    ) -> Result<(), CommandBufferError> {
        validate_size(threads, "thread grid")?;
        validate_size(threads_per_threadgroup, "threads-per-threadgroup")?;
        self.validate_threadgroup(threads_per_threadgroup)?;
        self.core.with_active("dispatch_threads", |encoder| unsafe {
            ffi::ametal_compute_command_encoder_dispatch_threads(
                encoder,
                threads.0,
                threads.1,
                threads.2,
                threads_per_threadgroup.0,
                threads_per_threadgroup.1,
                threads_per_threadgroup.2,
            );
        })
    }

    /// Update `fence` with work encoded so far.
    pub fn update_fence(&mut self, fence: &Fence) -> Result<(), CommandBufferError> {
        self.core.with_active("update_fence", |encoder| unsafe {
            ffi::ametal_compute_command_encoder_update_fence(encoder, fence.as_ptr());
        })?;
        self.core.record_fence_update(fence);
        Ok(())
    }

    /// Wait for `fence` before executing subsequent work.
    pub fn wait_for_fence(&mut self, fence: &Fence) -> Result<(), CommandBufferError> {
        self.core.ensure_fence_wait_allowed(fence)?;
        self.core.with_active("wait_for_fence", |encoder| unsafe {
            ffi::ametal_compute_command_encoder_wait_for_fence(encoder, fence.as_ptr());
        })
    }
}

impl RenderCommandEncoder {
    /// Bind a render pipeline state.
    pub fn set_render_pipeline_state(
        &mut self,
        pipeline: &RenderPipelineState,
    ) -> Result<(), CommandBufferError> {
        if !pipeline.is_drawable() || pipeline.targets() != self.targets {
            return Err(CommandBufferError::IncompatiblePipelineState);
        }
        self.core
            .with_active("set_render_pipeline_state", |encoder| unsafe {
                ffi::ametal_render_command_encoder_set_render_pipeline_state(
                    encoder,
                    pipeline.as_ptr(),
                );
            })?;
        self.pipeline_bound = true;
        Ok(())
    }

    /// Bind a vertex buffer at `index`.
    pub fn set_vertex_buffer(
        &mut self,
        buffer: &MetalBuffer,
        offset: usize,
        index: usize,
    ) -> Result<(), CommandBufferError> {
        validate_binding_index("vertex buffer", index, MAX_BUFFER_BINDINGS)?;
        checked_resource_range("vertex buffer", offset, 0, buffer.length())?;
        ensure_native_int(offset, "vertex buffer offset")?;
        self.core
            .with_active("set_vertex_buffer", |encoder| unsafe {
                ffi::ametal_render_command_encoder_set_vertex_buffer(
                    encoder,
                    buffer.as_ptr(),
                    offset,
                    index,
                );
            })
    }

    /// Bind a fragment sampler state at `index`.
    pub fn set_fragment_sampler_state(
        &mut self,
        sampler: &SamplerState,
        index: usize,
    ) -> Result<(), CommandBufferError> {
        validate_binding_index("fragment sampler", index, MAX_SAMPLER_BINDINGS)?;
        self.core
            .with_active("set_fragment_sampler_state", |encoder| unsafe {
                ffi::ametal_render_command_encoder_set_fragment_sampler_state(
                    encoder,
                    sampler.as_ptr(),
                    index,
                );
            })
    }

    /// Bind a depth/stencil state object.
    pub fn set_depth_stencil_state(
        &mut self,
        state: &DepthStencilState,
    ) -> Result<(), CommandBufferError> {
        if state.tests_depth() && self.targets.depth == crate::pixel_format::INVALID {
            return Err(CommandBufferError::MissingAttachment {
                attachment: "depth",
            });
        }
        if state.tests_stencil() && self.targets.stencil == crate::pixel_format::INVALID {
            return Err(CommandBufferError::MissingAttachment {
                attachment: "stencil",
            });
        }
        self.core
            .with_active("set_depth_stencil_state", |encoder| unsafe {
                ffi::ametal_render_command_encoder_set_depth_stencil_state(encoder, state.as_ptr());
            })
    }

    /// Draw a non-indexed primitive range.
    pub fn draw_primitives(
        &mut self,
        primitive_type: usize,
        vertex_start: usize,
        vertex_count: usize,
    ) -> Result<(), CommandBufferError> {
        if !self.pipeline_bound {
            return Err(CommandBufferError::MissingPipelineState);
        }
        if primitive_type > crate::primitive_type::TRIANGLE_STRIP {
            return Err(CommandBufferError::InvalidPrimitiveType { primitive_type });
        }
        ensure_native_int(vertex_start, "vertex start")?;
        ensure_native_int(vertex_count, "vertex count")?;
        vertex_start
            .checked_add(vertex_count)
            .filter(|end| isize::try_from(*end).is_ok())
            .ok_or_else(|| CommandBufferError::IntegerOutOfRange {
                field: "vertex range end",
                value: vertex_start.saturating_add(vertex_count),
            })?;
        self.core.with_active("draw_primitives", |encoder| unsafe {
            ffi::ametal_render_command_encoder_draw_primitives(
                encoder,
                primitive_type,
                vertex_start,
                vertex_count,
            );
        })
    }

    /// Update `fence` with work encoded so far.
    pub fn update_fence(&mut self, fence: &Fence) -> Result<(), CommandBufferError> {
        self.core.with_active("update_fence", |encoder| unsafe {
            ffi::ametal_render_command_encoder_update_fence(encoder, fence.as_ptr());
        })?;
        self.core.record_fence_update(fence);
        Ok(())
    }

    /// Wait for `fence` before executing subsequent work.
    pub fn wait_for_fence(&mut self, fence: &Fence) -> Result<(), CommandBufferError> {
        self.core.ensure_fence_wait_allowed(fence)?;
        self.core.with_active("wait_for_fence", |encoder| unsafe {
            ffi::ametal_render_command_encoder_wait_for_fence(encoder, fence.as_ptr());
        })
    }
}

unsafe extern "C" fn command_buffer_handler_trampoline(
    context: *mut c_void,
    status: usize,
    error_message: *const c_char,
) {
    let result = match status {
        command_buffer_status::SCHEDULED | command_buffer_status::COMPLETED => Ok(()),
        command_buffer_status::ERROR => {
            let message = if error_message.is_null() {
                "Metal reported an unspecified command-buffer error".to_string()
            } else {
                unsafe { CStr::from_ptr(error_message) }
                    .to_string_lossy()
                    .into_owned()
            };
            Err(CommandBufferError::ExecutionFailed(message))
        }
        status => Err(CommandBufferError::NotExecuted { status }),
    };
    let _ = unsafe {
        CallbackContext::<CommandBufferHandler>::with(
            context,
            "CommandBuffer handler",
            move |slot| {
                let handler = slot.lock().unwrap_or_else(PoisonError::into_inner).take();
                if let Some(handler) = handler {
                    handler(result);
                }
            },
        )
    };
}

fn render_targets(
    color: &MetalTexture,
    load_action: usize,
    store_action: usize,
    depth: Option<&RenderPassDepthAttachment<'_>>,
    stencil: Option<&RenderPassStencilAttachment<'_>>,
) -> Result<RenderTargetFormats, CommandBufferError> {
    use crate::pixel_format::{
        color_bytes_per_pixel, is_depth_attachment_format, is_stencil_attachment_format,
        DEPTH24UNORM_STENCIL8, DEPTH32FLOAT_STENCIL8, INVALID,
    };
    use crate::texture_type::{
        CUBE, CUBE_ARRAY, TYPE_2D, TYPE_2D_ARRAY, TYPE_2D_MULTISAMPLE, TYPE_2D_MULTISAMPLE_ARRAY,
        TYPE_3D,
    };

    let sample_count = color.sample_count();
    let attachment_format = |attachment: &'static str,
                             texture: &MetalTexture,
                             load_action: usize,
                             store_action: usize,
                             format_ok: fn(usize) -> bool|
     -> Result<usize, CommandBufferError> {
        let format = texture.pixel_format();
        let renderable_type = matches!(
            texture.texture_type(),
            TYPE_2D
                | TYPE_2D_ARRAY
                | TYPE_2D_MULTISAMPLE
                | TYPE_2D_MULTISAMPLE_ARRAY
                | CUBE
                | CUBE_ARRAY
                | TYPE_3D
        );
        if format_ok(format)
            && texture.usage() & crate::texture_usage::RENDER_TARGET != 0
            && renderable_type
            && texture.sample_count() == sample_count
            && load_action <= crate::load_action::CLEAR
            && store_action <= crate::store_action::STORE
        {
            Ok(format)
        } else {
            Err(CommandBufferError::InvalidAttachment { attachment })
        }
    };
    let color_format = attachment_format("color", color, load_action, store_action, |format| {
        color_bytes_per_pixel(format).is_some()
    })?;
    let depth_format = depth
        .map(|depth| {
            attachment_format(
                "depth",
                depth.texture,
                depth.load_action,
                depth.store_action,
                is_depth_attachment_format,
            )
        })
        .transpose()?
        .unwrap_or(INVALID);
    let stencil_format = stencil
        .map(|stencil| {
            attachment_format(
                "stencil",
                stencil.texture,
                stencil.load_action,
                stencil.store_action,
                is_stencil_attachment_format,
            )
        })
        .transpose()?
        .unwrap_or(INVALID);
    if let (Some(depth), Some(stencil)) = (depth, stencil) {
        let combined = |format| matches!(format, DEPTH24UNORM_STENCIL8 | DEPTH32FLOAT_STENCIL8);
        if (combined(depth_format) || combined(stencil_format))
            && depth.texture.as_ptr() != stencil.texture.as_ptr()
        {
            return Err(CommandBufferError::InvalidAttachment {
                attachment: "stencil",
            });
        }
    }
    if depth.is_some_and(|depth| {
        !depth.clear_depth.is_finite() || !(0.0..=1.0).contains(&depth.clear_depth)
    }) {
        return Err(CommandBufferError::InvalidAttachment {
            attachment: "depth",
        });
    }
    let mut colors = [INVALID; 8];
    colors[0] = color_format;
    Ok(RenderTargetFormats {
        colors,
        depth: depth_format,
        stencil: stencil_format,
        sample_count,
    })
}

fn ensure_recording(
    phase: CommandBufferPhase,
    operation: &'static str,
) -> Result<(), CommandBufferError> {
    if matches!(
        phase,
        CommandBufferPhase::Recording | CommandBufferPhase::Enqueued
    ) {
        Ok(())
    } else {
        Err(invalid_state(operation, phase))
    }
}

fn invalid_state(operation: &'static str, phase: CommandBufferPhase) -> CommandBufferError {
    CommandBufferError::InvalidState {
        operation,
        state: match phase {
            CommandBufferPhase::Recording => "recording",
            CommandBufferPhase::Enqueued => "enqueued",
            CommandBufferPhase::Committed => "committed",
            CommandBufferPhase::Completed => "completed",
            CommandBufferPhase::Error => "failed",
        },
    }
}

fn checked_resource_range(
    resource: &'static str,
    offset: usize,
    length: usize,
    resource_length: usize,
) -> Result<(), CommandBufferError> {
    let end = offset
        .checked_add(length)
        .ok_or(CommandBufferError::RangeOutOfBounds {
            resource,
            offset,
            length,
            resource_length,
        })?;
    if end > resource_length {
        Err(CommandBufferError::RangeOutOfBounds {
            resource,
            offset,
            length,
            resource_length,
        })
    } else {
        Ok(())
    }
}

fn validate_binding_index(
    binding: &'static str,
    index: usize,
    limit: usize,
) -> Result<(), CommandBufferError> {
    if index < limit {
        Ok(())
    } else {
        Err(CommandBufferError::InvalidBindingIndex {
            binding,
            index,
            limit,
        })
    }
}

fn ensure_native_int(value: usize, field: &'static str) -> Result<(), CommandBufferError> {
    if isize::try_from(value).is_ok() {
        Ok(())
    } else {
        Err(CommandBufferError::IntegerOutOfRange { field, value })
    }
}

fn validate_size(
    size: (usize, usize, usize),
    field: &'static str,
) -> Result<(), CommandBufferError> {
    for (axis, value) in [("width", size.0), ("height", size.1), ("depth", size.2)] {
        if value == 0 {
            return Err(CommandBufferError::EmptyDispatch { field: axis });
        }
        ensure_native_int(value, field)?;
    }
    Ok(())
}

fn synchronize_resource(
    core: &EncoderCore,
    resource: *mut c_void,
    resource_storage_mode: usize,
) -> Result<(), CommandBufferError> {
    if resource_storage_mode != storage_mode::MANAGED {
        return Err(CommandBufferError::ManagedStorageRequired {
            storage_mode: resource_storage_mode,
        });
    }
    let accepted = core.with_active("synchronize_resource", |encoder| unsafe {
        ffi::ametal_blit_command_encoder_synchronize_resource(encoder, resource)
    })?;
    if accepted {
        Ok(())
    } else {
        Err(CommandBufferError::NativeRejected {
            operation: "managed resource synchronization",
        })
    }
}
