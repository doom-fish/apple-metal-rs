//! Smoke test for the v0.5 compute pipeline: compiles a trivial
//! "multiply by 2" Metal kernel, dispatches it on a shared buffer
//! of 16 floats, and verifies every element doubled.

#![allow(clippy::cast_precision_loss, clippy::float_cmp)]

use apple_metal::{resource_options, MetalDevice};

const KERNEL_SRC: &str = "
#include <metal_stdlib>
using namespace metal;

kernel void mul2(device float *data [[buffer(0)]],
                 uint i [[thread_position_in_grid]]) {
    data[i] = data[i] * 2.0;
}
";

const N: usize = 16;

fn main() {
    let device = MetalDevice::system_default().expect("MTLCreateSystemDefaultDevice");
    println!("Device unified={}", device.has_unified_memory());

    let lib = device
        .new_library_with_source(KERNEL_SRC)
        .expect("compile MSL source");
    println!("✅ Compiled library {:p}", lib.as_ptr());

    let func = lib.new_function("mul2").expect("locate function 'mul2'");
    println!("✅ Found function mul2 {:p}", func.as_ptr());

    let pso = device
        .new_compute_pipeline_state(&func)
        .expect("build compute pipeline state");
    println!("✅ Compute pipeline state {:p}", pso.as_ptr());

    let byte_len = N * core::mem::size_of::<f32>();
    let buffer = device
        .new_buffer(byte_len, resource_options::STORAGE_MODE_SHARED)
        .expect("allocate buffer");

    {
        let mut mapping = unsafe { buffer.map_write().expect("map compute input") };
        for (i, bytes) in mapping.chunks_exact_mut(4).take(N).enumerate() {
            bytes.copy_from_slice(&(i as f32).to_ne_bytes());
        }
        let input: Vec<f32> = mapping
            .chunks_exact(4)
            .take(N)
            .map(|bytes| f32::from_ne_bytes(bytes.try_into().expect("four-byte float")))
            .collect();
        drop(mapping);
        println!("Input : {input:?}");
    }

    let queue = device.new_command_queue().expect("MTLCommandQueue");
    let cb = queue.new_command_buffer().expect("MTLCommandBuffer");
    cb.dispatch_compute_1d(&pso, &[&buffer], N, 1)
        .expect("dispatch compute");
    cb.commit().expect("commit compute");
    cb.wait_until_completed().expect("complete compute");

    let mapping = unsafe { buffer.map_read().expect("map compute output") };
    let output: Vec<f32> = mapping
        .chunks_exact(4)
        .take(N)
        .map(|bytes| f32::from_ne_bytes(bytes.try_into().expect("four-byte float")))
        .collect();
    drop(mapping);
    println!("Output: {output:?}");

    for (i, &v) in output.iter().enumerate() {
        let expected = (i as f32) * 2.0;
        assert_eq!(v, expected, "element {i} expected {expected} got {v}");
    }
    println!("✅ All {N} elements correctly doubled by the GPU kernel");
}
