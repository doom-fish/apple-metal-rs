use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc;
use std::sync::Arc;
use std::thread::sleep;
use std::time::{Duration, Instant};

use apple_metal::{
    resource_options, CommandBufferError, CommandQueue, MetalDevice, MetalSharedEventListener,
};

const TIMEOUT: Duration = Duration::from_secs(10);

fn queue() -> Option<(MetalDevice, CommandQueue)> {
    let device = MetalDevice::system_default()?;
    let queue = device.new_command_queue()?;
    Some((device, queue))
}

fn settled_strong_count(token: &Arc<()>, expected: usize) -> usize {
    let deadline = Instant::now() + Duration::from_secs(5);
    while Arc::strong_count(token) != expected && Instant::now() < deadline {
        sleep(Duration::from_millis(10));
    }
    Arc::strong_count(token)
}

#[test]
fn scheduled_and_completed_handlers_run_once_in_order() {
    let Some((device, queue)) = queue() else {
        eprintln!("skipping: no Metal device");
        return;
    };
    let source = device
        .new_buffer(64, resource_options::STORAGE_MODE_SHARED)
        .expect("source");
    let destination = device
        .new_buffer(64, resource_options::STORAGE_MODE_SHARED)
        .expect("destination");
    let command_buffer = queue.new_command_buffer().expect("command buffer");
    command_buffer
        .blit_copy_buffer(&source, 0, &destination, 0, 64)
        .expect("encode copy");

    let order = Arc::new(AtomicUsize::new(0));
    let (sender, receiver) = mpsc::channel();
    let scheduled_order = Arc::clone(&order);
    let scheduled_sender = sender.clone();
    command_buffer
        .add_scheduled_handler(move |result| {
            let position = scheduled_order.fetch_add(1, Ordering::SeqCst);
            scheduled_sender
                .send(("scheduled", position, result))
                .expect("send scheduled");
        })
        .expect("add scheduled handler");
    let completed_order = Arc::clone(&order);
    command_buffer
        .add_completed_handler(move |result| {
            let position = completed_order.fetch_add(1, Ordering::SeqCst);
            sender
                .send(("completed", position, result))
                .expect("send completed");
        })
        .expect("add completed handler");

    command_buffer.commit().expect("commit");
    let first = receiver.recv_timeout(TIMEOUT).expect("first handler");
    let second = receiver.recv_timeout(TIMEOUT).expect("second handler");
    assert_eq!(first, ("scheduled", 0, Ok(())));
    assert_eq!(second, ("completed", 1, Ok(())));
    command_buffer.wait_until_completed().expect("completion");
    assert!(receiver.recv_timeout(Duration::from_millis(200)).is_err());
}

#[test]
fn handlers_are_rejected_once_the_buffer_is_committed() {
    let Some((_device, queue)) = queue() else {
        eprintln!("skipping: no Metal device");
        return;
    };
    let command_buffer = queue.new_command_buffer().expect("command buffer");
    command_buffer.commit().expect("commit");
    assert_eq!(
        command_buffer.add_completed_handler(|_| {}),
        Err(CommandBufferError::InvalidState {
            operation: "add_completed_handler",
            state: "committed",
        })
    );
    assert_eq!(
        command_buffer.add_scheduled_handler(|_| {}),
        Err(CommandBufferError::InvalidState {
            operation: "add_scheduled_handler",
            state: "committed",
        })
    );
    command_buffer.wait_until_completed().expect("completion");
}

#[test]
fn enqueued_buffers_still_accept_handlers() {
    let Some((_device, queue)) = queue() else {
        eprintln!("skipping: no Metal device");
        return;
    };
    let command_buffer = queue.new_command_buffer().expect("command buffer");
    command_buffer.enqueue().expect("enqueue");
    let (sender, receiver) = mpsc::channel();
    command_buffer
        .add_completed_handler(move |result| sender.send(result).expect("send"))
        .expect("add handler after enqueue");
    command_buffer.commit().expect("commit");
    assert_eq!(receiver.recv_timeout(TIMEOUT).expect("handler"), Ok(()));
}

#[test]
fn a_panicking_handler_is_contained() {
    let Some((_device, queue)) = queue() else {
        eprintln!("skipping: no Metal device");
        return;
    };
    let command_buffer = queue.new_command_buffer().expect("command buffer");
    let (sender, receiver) = mpsc::channel();
    command_buffer
        .add_completed_handler(|_| panic!("handler panic"))
        .expect("add panicking handler");
    command_buffer
        .add_completed_handler(move |result| sender.send(result).expect("send"))
        .expect("add second handler");
    command_buffer.commit().expect("commit");
    assert_eq!(
        receiver.recv_timeout(TIMEOUT).expect("second handler"),
        Ok(())
    );
}

#[test]
fn handlers_outlive_the_rust_command_buffer() {
    let Some((_device, queue)) = queue() else {
        eprintln!("skipping: no Metal device");
        return;
    };
    let token = Arc::new(());
    let captured = Arc::clone(&token);
    let (sender, receiver) = mpsc::channel();
    {
        let command_buffer = queue.new_command_buffer().expect("command buffer");
        command_buffer
            .add_completed_handler(move |result| {
                sender
                    .send((result, Arc::strong_count(&captured)))
                    .expect("send");
            })
            .expect("add handler");
        command_buffer.commit().expect("commit");
    }
    let (result, count) = receiver.recv_timeout(TIMEOUT).expect("handler");
    assert_eq!(result, Ok(()));
    assert!(count >= 2);
    assert_eq!(settled_strong_count(&token, 1), 1);
}

#[test]
fn handlers_of_an_uncommitted_buffer_report_that_it_never_ran() {
    let Some((_device, queue)) = queue() else {
        eprintln!("skipping: no Metal device");
        return;
    };
    let token = Arc::new(());
    let captured = Arc::clone(&token);
    let (sender, receiver) = mpsc::channel();
    let scheduled_sender = sender.clone();
    let command_buffer = queue.new_command_buffer().expect("command buffer");
    command_buffer
        .add_scheduled_handler(move |result| {
            scheduled_sender.send(result).expect("send scheduled");
        })
        .expect("add scheduled handler");
    command_buffer
        .add_completed_handler(move |result| {
            let _ = &captured;
            sender.send(result).expect("send completed");
        })
        .expect("add completed handler");
    assert_eq!(Arc::strong_count(&token), 2);
    drop(command_buffer);
    for _ in 0..2 {
        assert_eq!(
            receiver.recv_timeout(TIMEOUT).expect("handler on release"),
            Err(CommandBufferError::NotExecuted { status: 0 })
        );
    }
    assert_eq!(settled_strong_count(&token, 1), 1);
}

#[test]
fn shared_event_listeners_fire_once_the_value_is_reached() {
    let Some(device) = MetalDevice::system_default() else {
        eprintln!("skipping: no Metal device");
        return;
    };
    let event = device.new_shared_event().expect("shared event");
    let listener = MetalSharedEventListener::new().expect("listener");
    let (sender, receiver) = mpsc::channel();
    let pending = sender.clone();
    assert!(event.notify_listener(&listener, 5, move |value| {
        pending.send(("pending", value)).expect("send");
    }));
    assert!(receiver.recv_timeout(Duration::from_millis(200)).is_err());
    event.set_signaled_value(7);
    assert_eq!(
        receiver.recv_timeout(TIMEOUT).expect("notification"),
        ("pending", 5)
    );
    assert_eq!(event.signaled_value(), 7);

    assert!(event.notify_listener(&listener, 3, move |value| {
        sender.send(("reached", value)).expect("send");
    }));
    assert_eq!(
        receiver
            .recv_timeout(TIMEOUT)
            .expect("immediate notification"),
        ("reached", 3)
    );
}

#[test]
fn a_panicking_shared_event_handler_is_contained() {
    let Some(device) = MetalDevice::system_default() else {
        eprintln!("skipping: no Metal device");
        return;
    };
    let event = device.new_shared_event().expect("shared event");
    let listener = MetalSharedEventListener::new().expect("listener");
    let (sender, receiver) = mpsc::channel();
    assert!(event.notify_listener(&listener, 1, |_| panic!("listener panic")));
    assert!(event.notify_listener(&listener, 1, move |value| {
        sender.send(value).expect("send");
    }));
    event.set_signaled_value(1);
    assert_eq!(receiver.recv_timeout(TIMEOUT).expect("second listener"), 1);
}
