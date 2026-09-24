mod common;

use std::sync::mpsc;
use std::sync::Barrier;
use std::thread;

use apple_metal::{ffi, resource_options, CommandBufferError};

#[test]
fn foreign_code_can_encode_through_the_borrowed_handle() {
    let device = common::device();
    let queue = device.new_command_queue().expect("command queue");
    let source = device
        .new_buffer(16, resource_options::STORAGE_MODE_SHARED)
        .expect("source");
    let destination = device
        .new_buffer(16, resource_options::STORAGE_MODE_SHARED)
        .expect("destination");
    common::write_u32_words(&source, &[5, 6, 7, 8]);
    common::write_u32_words(&destination, &[0, 0, 0, 0]);
    let command_buffer = queue.new_command_buffer().expect("command buffer");

    let copied = command_buffer
        .encode_foreign(|foreign| {
            assert_eq!(foreign.command_buffer(), command_buffer.as_ptr());
            unsafe {
                let encoder =
                    ffi::ametal_command_buffer_new_blit_command_encoder(foreign.command_buffer());
                assert!(!encoder.is_null());
                let copied = ffi::ametal_blit_command_encoder_copy_buffer(
                    encoder,
                    source.as_ptr(),
                    0,
                    destination.as_ptr(),
                    0,
                    16,
                );
                ffi::ametal_command_encoder_end_encoding(encoder);
                ffi::ametal_object_release(encoder);
                copied
            }
        })
        .expect("foreign encoding");
    assert!(copied);
    command_buffer.commit().expect("commit");
    command_buffer.wait_until_completed().expect("complete");
    assert_eq!(common::read_u32_words(&destination, 4), vec![5, 6, 7, 8]);
}

#[test]
fn foreign_encoding_needs_a_recording_buffer_without_an_open_encoder() {
    let device = common::device();
    let queue = device.new_command_queue().expect("command queue");

    let command_buffer = queue.new_command_buffer().expect("command buffer");
    let encoder = command_buffer
        .new_blit_command_encoder()
        .expect("blit encoder");
    let mut ran = false;
    assert_eq!(
        command_buffer.encode_foreign(|_| ran = true),
        Err(CommandBufferError::ActiveEncoder)
    );
    assert!(!ran);
    encoder.end_encoding().expect("end encoder");
    command_buffer
        .encode_foreign(|_| ran = true)
        .expect("encoder ended");
    assert!(ran);

    command_buffer.commit().expect("commit");
    assert_eq!(
        command_buffer.encode_foreign(|_| ()),
        Err(CommandBufferError::InvalidState {
            operation: "encode_foreign",
            state: "committed",
        })
    );
    command_buffer.wait_until_completed().expect("complete");
}

#[test]
fn the_buffer_stays_locked_while_foreign_code_runs() {
    let device = common::device();
    let queue = device.new_command_queue().expect("command queue");
    let event = device.new_shared_event().expect("shared event");
    let command_buffer = queue.new_command_buffer().expect("command buffer");
    let barrier = Barrier::new(2);
    let (sender, receiver) = mpsc::channel();

    thread::scope(|scope| {
        let other = command_buffer.clone();
        let barrier = &barrier;
        scope.spawn(move || {
            barrier.wait();
            sender.send(other.commit()).expect("send commit result");
            barrier.wait();
        });
        command_buffer
            .encode_foreign(|_| {
                assert_eq!(
                    command_buffer.commit(),
                    Err(CommandBufferError::ActiveEncoder)
                );
                assert_eq!(
                    command_buffer.enqueue(),
                    Err(CommandBufferError::ActiveEncoder)
                );
                assert!(matches!(
                    command_buffer.new_compute_command_encoder(),
                    Err(CommandBufferError::ActiveEncoder)
                ));
                assert_eq!(
                    command_buffer.encode_signal_event(&event, 1),
                    Err(CommandBufferError::ActiveEncoder)
                );
                assert_eq!(
                    command_buffer.encode_foreign(|_| ()),
                    Err(CommandBufferError::ActiveEncoder)
                );
                barrier.wait();
                assert_eq!(
                    receiver.recv().expect("commit result"),
                    Err(CommandBufferError::ActiveEncoder)
                );
                barrier.wait();
            })
            .expect("foreign encoding");
    });

    command_buffer
        .commit()
        .expect("commit after foreign encoding");
    command_buffer.wait_until_completed().expect("complete");
}

#[test]
fn a_native_commit_inside_the_closure_is_noticed() {
    let device = common::device();
    let queue = device.new_command_queue().expect("command queue");
    let command_buffer = queue.new_command_buffer().expect("command buffer");
    command_buffer
        .encode_foreign(|foreign| unsafe {
            ffi::ametal_command_buffer_commit(foreign.command_buffer());
        })
        .expect("foreign encoding");
    assert!(matches!(
        command_buffer.commit(),
        Err(CommandBufferError::InvalidState {
            operation: "commit",
            ..
        })
    ));
    command_buffer
        .wait_until_completed()
        .expect("completes after the native commit");
}

#[test]
fn a_panicking_closure_leaves_the_buffer_unusable() {
    let device = common::device();
    let queue = device.new_command_queue().expect("command queue");
    let command_buffer = queue.new_command_buffer().expect("command buffer");
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _ = command_buffer.encode_foreign(|_| panic!("foreign encoder panic"));
    }));
    assert!(result.is_err());
    assert_eq!(
        command_buffer.commit(),
        Err(CommandBufferError::ActiveEncoder)
    );
    assert!(matches!(
        command_buffer.new_blit_command_encoder(),
        Err(CommandBufferError::ActiveEncoder)
    ));
}
