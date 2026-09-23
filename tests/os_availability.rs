use std::process::Command;

use apple_metal::{log_level, resource_options, MetalDevice};

const LOG_STATE_UNAVAILABLE: &str = "MTLLogState requires macOS 15.0 or later";
const RESIDENCY_SET_UNAVAILABLE: &str = "MTLResidencySet requires macOS 15.0 or later";

fn macos_major_version() -> Option<u32> {
    let output = Command::new("sw_vers")
        .arg("-productVersion")
        .output()
        .ok()?;
    let version = String::from_utf8(output.stdout).ok()?;
    version.trim().split('.').next()?.parse().ok()
}

#[test]
fn macos_15_objects_follow_the_running_os() {
    let Some(device) = MetalDevice::system_default() else {
        eprintln!("skipping: no Metal device");
        return;
    };
    let Some(major) = macos_major_version() else {
        eprintln!("skipping: macOS version unknown");
        return;
    };
    let log_state = device.new_log_state(log_level::INFO, 1_024);
    let residency_set = device.new_residency_set(Some("availability"), 4);

    if major < 15 {
        assert_eq!(log_state.err().as_deref(), Some(LOG_STATE_UNAVAILABLE));
        assert_eq!(
            residency_set.err().as_deref(),
            Some(RESIDENCY_SET_UNAVAILABLE)
        );
        return;
    }

    match log_state {
        Ok(log_state) => assert!(device
            .new_command_queue_with_log_state(4, &log_state)
            .is_some()),
        Err(message) => assert_ne!(message, LOG_STATE_UNAVAILABLE),
    }

    let residency_set = residency_set.expect("residency set on macOS 15 or later");
    let buffer = device
        .new_buffer(256, resource_options::STORAGE_MODE_SHARED)
        .expect("shared buffer");
    residency_set.add_buffer(&buffer);
    residency_set.commit();
    assert!(residency_set.contains_buffer(&buffer));
    assert_eq!(residency_set.allocation_count(), 1);
    residency_set.remove_buffer(&buffer);
    residency_set.commit();
    assert!(!residency_set.contains_buffer(&buffer));
    assert_eq!(residency_set.allocation_count(), 0);

    let queue = device.new_command_queue().expect("command queue");
    queue.add_residency_set(&residency_set);
    queue.remove_residency_set(&residency_set);
}
