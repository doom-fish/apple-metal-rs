use std::sync::Arc;
use std::thread::sleep;
use std::time::{Duration, Instant};

use apple_metal::{
    copy_all_devices, copy_all_devices_with_observer, remove_device_observer, MetalDevice,
};

fn settled_strong_count(token: &Arc<()>, expected: usize) -> usize {
    let deadline = Instant::now() + Duration::from_secs(5);
    while Arc::strong_count(token) != expected && Instant::now() < deadline {
        sleep(Duration::from_millis(10));
    }
    Arc::strong_count(token)
}

#[test]
fn dropping_the_observer_unregisters_and_releases_its_handler() {
    if MetalDevice::system_default().is_none() {
        eprintln!("skipping: no Metal device");
        return;
    }
    let token = Arc::new(());
    let captured = Arc::clone(&token);
    let (devices, observer) = copy_all_devices_with_observer(move |device, name| {
        assert!(!name.is_empty());
        assert!(!device.as_ptr().is_null());
        assert!(Arc::strong_count(&captured) >= 2);
    });
    assert!(!devices.is_empty());
    assert_eq!(devices.len(), copy_all_devices().len());
    let observer = observer.expect("device observer");
    assert!(!observer.as_ptr().is_null());
    assert_eq!(Arc::strong_count(&token), 2);

    drop(observer);
    assert_eq!(settled_strong_count(&token, 1), 1);
}

#[test]
fn explicit_removal_is_idempotent_and_drop_still_releases() {
    if MetalDevice::system_default().is_none() {
        eprintln!("skipping: no Metal device");
        return;
    }
    let token = Arc::new(());
    let captured = Arc::clone(&token);
    let (_devices, observer) = copy_all_devices_with_observer(move |_device, _name| {
        assert!(Arc::strong_count(&captured) >= 2);
    });
    let observer = observer.expect("device observer");

    observer.remove();
    remove_device_observer(&observer);
    observer.remove();
    assert_eq!(Arc::strong_count(&token), 2);

    drop(observer);
    assert_eq!(settled_strong_count(&token, 1), 1);
}

#[test]
fn observers_can_move_between_threads() {
    if MetalDevice::system_default().is_none() {
        eprintln!("skipping: no Metal device");
        return;
    }
    let (_devices, observer) = copy_all_devices_with_observer(|_device, _name| {});
    let observer = observer.expect("device observer");
    std::thread::spawn(move || drop(observer))
        .join()
        .expect("drop on another thread");
}
