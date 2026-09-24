use apple_metal::{
    storage_mode, MetalDevice, MetalTensorDataType, MetalTensorUsage, TensorDescriptor, TensorError,
};

fn device() -> Option<MetalDevice> {
    let device = MetalDevice::system_default();
    if device.is_none() {
        eprintln!("skipping: no Metal device");
    }
    device
}

#[test]
fn tensors_are_created_from_checked_descriptors() {
    let Some(device) = device() else { return };
    let descriptor = TensorDescriptor::new(&[4, 2], MetalTensorDataType::FLOAT32);
    let tensor = match device.new_tensor(&descriptor) {
        Err(TensorError::Unsupported) => {
            eprintln!("skipping: MTLTensor needs macOS 26");
            return;
        }
        other => other.expect("tensor"),
    };
    assert!(!tensor.as_ptr().is_null());

    let mut machine_learning = TensorDescriptor::new(&[8], MetalTensorDataType::FLOAT16);
    machine_learning.usage = MetalTensorUsage::MACHINE_LEARNING;
    machine_learning.storage_mode = storage_mode::PRIVATE;
    device
        .new_tensor(&machine_learning)
        .expect("machine-learning tensor");
    device
        .new_tensor(&TensorDescriptor::new(&[], MetalTensorDataType::INT32))
        .expect("scalar tensor");

    assert_eq!(
        device
            .new_tensor(&TensorDescriptor::new(
                &[1; 17],
                MetalTensorDataType::FLOAT32
            ))
            .err(),
        Some(TensorError::InvalidRank { rank: 17 })
    );
    assert_eq!(
        device
            .new_tensor(&TensorDescriptor::new(&[4], MetalTensorDataType(9_999)))
            .err(),
        Some(TensorError::UnsupportedDataType { data_type: 9_999 })
    );
    let mut bad_usage = descriptor.clone();
    bad_usage.usage = MetalTensorUsage(0x80);
    assert_eq!(
        device.new_tensor(&bad_usage).err(),
        Some(TensorError::InvalidUsage { usage: 0x80 })
    );
    let mut memoryless = descriptor;
    memoryless.storage_mode = storage_mode::MEMORYLESS;
    assert_eq!(
        device.new_tensor(&memoryless).err(),
        Some(TensorError::UnsupportedStorageMode {
            storage_mode: storage_mode::MEMORYLESS
        })
    );
    let maximum_bytes = device.max_buffer_length();
    assert!(maximum_bytes > 0);
    assert_eq!(
        device
            .new_tensor(&TensorDescriptor::new(
                &[maximum_bytes / 4 + 1],
                MetalTensorDataType::FLOAT32
            ))
            .err(),
        Some(TensorError::TooLarge { maximum_bytes })
    );
    assert_eq!(
        device
            .new_tensor(&TensorDescriptor::new(
                &[1 << 40, 1 << 30],
                MetalTensorDataType::FLOAT32
            ))
            .err(),
        Some(TensorError::TooLarge { maximum_bytes })
    );
    assert!(matches!(
        device.new_tensor(&TensorDescriptor::new(
            &[0, 2],
            MetalTensorDataType::FLOAT32
        )),
        Err(TensorError::Native(_))
    ));
}
