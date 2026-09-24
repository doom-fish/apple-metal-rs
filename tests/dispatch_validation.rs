mod common;

use apple_metal::{
    resource_options, CommandBufferError, ComputePipelineDescriptor, MetalBuffer, MetalDevice,
};

fn shared_buffer(device: &MetalDevice, length: usize) -> MetalBuffer {
    device
        .new_buffer(length, resource_options::STORAGE_MODE_SHARED)
        .expect("shared buffer")
}

#[test]
fn dispatches_need_a_pipeline_and_a_legal_threadgroup() {
    let device = common::device();
    let (library, increment, _args, pipeline) = common::compile_compute(&device);
    let mut multiple_descriptor = ComputePipelineDescriptor::new(&increment);
    multiple_descriptor.thread_group_size_is_multiple_of_thread_execution_width = true;
    let multiple = device
        .new_compute_pipeline_state_with_descriptor(&multiple_descriptor)
        .expect("thread-group-multiple pipeline");
    drop(library);
    let width = multiple.thread_execution_width();
    assert!(width > 1);
    let buffer = shared_buffer(&device, 64);
    common::write_u32_words(&buffer, &[0, 1, 2, 3]);

    let queue = device.new_command_queue().expect("command queue");
    let command_buffer = queue.new_command_buffer().expect("command buffer");
    let mut encoder = command_buffer
        .new_compute_command_encoder()
        .expect("compute encoder");
    assert_eq!(
        encoder.dispatch_threadgroups((1, 1, 1), (1, 1, 1)),
        Err(CommandBufferError::MissingPipelineState)
    );
    assert_eq!(
        encoder.dispatch_threads((4, 1, 1), (1, 1, 1)),
        Err(CommandBufferError::MissingPipelineState)
    );

    encoder
        .set_compute_pipeline_state(&pipeline)
        .expect("bind pipeline");
    let maximum = pipeline.max_total_threads_per_threadgroup();
    assert_eq!(
        encoder.dispatch_threadgroups((1, 1, 1), (maximum + 1, 1, 1)),
        Err(CommandBufferError::ThreadgroupTooLarge {
            threads: maximum + 1,
            maximum,
        })
    );
    assert_eq!(
        encoder.dispatch_threads((4, 1, 1), (maximum, 2, 1)),
        Err(CommandBufferError::ThreadgroupTooLarge {
            threads: maximum * 2,
            maximum,
        })
    );
    assert!(matches!(
        encoder.dispatch_threads((4, 1, 1), (usize::MAX / 2, 4, 1)),
        Err(CommandBufferError::ThreadgroupTooLarge { .. }
            | CommandBufferError::IntegerOutOfRange { .. })
    ));

    encoder
        .set_compute_pipeline_state(&multiple)
        .expect("bind thread-group-multiple pipeline");
    encoder.set_buffer(&buffer, 0, 0).expect("bind buffer");
    assert_eq!(
        encoder.dispatch_threads((4, 1, 1), (1, 1, 1)),
        Err(CommandBufferError::ThreadgroupNotMultipleOfExecutionWidth {
            threads: 1,
            execution_width: width,
        })
    );
    assert_eq!(
        encoder.dispatch_threadgroups((1, 1, 1), (width + 1, 1, 1)),
        Err(CommandBufferError::ThreadgroupNotMultipleOfExecutionWidth {
            threads: width + 1,
            execution_width: width,
        })
    );
    encoder
        .dispatch_threads((4, 1, 1), (width, 1, 1))
        .expect("dispatch a whole SIMD group");
    encoder.end_encoding().expect("end compute encoder");
    command_buffer.commit().expect("commit");
    command_buffer.wait_until_completed().expect("complete");
    assert_eq!(common::read_u32_words(&buffer, 4), vec![1, 2, 3, 4]);
}

#[test]
fn buffer_resource_options_must_name_real_modes() {
    let device = common::device();
    for options in [
        2,
        3,
        3 << 4,
        4 << 4,
        0xF << 4,
        3 << 8,
        1 << 10,
        1 << 16,
        usize::MAX,
    ] {
        assert!(device.new_buffer(64, options).is_none(), "{options:#x}");
        assert!(
            device.new_buffer_with_bytes(&[1, 2, 3], options).is_none(),
            "{options:#x}"
        );
    }
    for options in [
        resource_options::STORAGE_MODE_SHARED,
        resource_options::STORAGE_MODE_PRIVATE,
        resource_options::CPU_CACHE_MODE_WRITE_COMBINED,
        resource_options::HAZARD_TRACKING_MODE_UNTRACKED,
        resource_options::HAZARD_TRACKING_MODE_TRACKED,
    ] {
        assert!(device.new_buffer(64, options).is_some(), "{options:#x}");
    }
    let Some(heap) = device.new_heap(1 << 20, apple_metal::storage_mode::SHARED) else {
        return;
    };
    assert!(heap.new_buffer(64, 2).is_none());
    assert!(heap.new_buffer(64, 1 << 10).is_none());
}
