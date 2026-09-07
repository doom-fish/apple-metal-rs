use apple_metal::{resource_options, MetalDevice};

fn main() {
    let dev = MetalDevice::system_default().expect("no Metal");
    let queue = dev.new_command_queue().expect("queue");
    let src = dev
        .new_buffer(64, resource_options::STORAGE_MODE_SHARED)
        .expect("src");
    let dst = dev
        .new_buffer(64, resource_options::STORAGE_MODE_SHARED)
        .expect("dst");
    unsafe {
        src.write_bytes(0, b"hello GPU blit from apple-metal-rs!!!!!")
            .expect("write source buffer");
    }

    let cb = queue.new_command_buffer().expect("cb");
    cb.blit_copy_buffer(&src, 0, &dst, 0, 64)
        .expect("encode blit copy");
    cb.commit().expect("commit blit");
    cb.wait_until_completed().expect("complete blit");

    let bytes = {
        let mapping = unsafe { dst.map_read().expect("map destination") };
        let bytes = mapping[..40].to_vec();
        drop(mapping);
        bytes
    };
    let s = String::from_utf8_lossy(&bytes);
    println!("GPU blit result: {s:?}");
    assert!(s.starts_with("hello GPU blit"));
}
