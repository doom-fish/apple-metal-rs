#![allow(dead_code)]

use apple_metal::{pixel_format, storage_mode, texture_usage, MetalBuffer, TextureDescriptor};
use std::fs;
use std::path::PathBuf;

pub const COMPUTE_SRC: &str = r"
#include <metal_stdlib>
using namespace metal;

struct Args {
    device uint *buffer;
    texture2d<float> texture;
};

kernel void increment(device uint *data [[buffer(0)]],
                      uint gid [[thread_position_in_grid]]) {
    if (gid < 4) {
        data[gid] += 1;
    }
}

kernel void use_args(constant Args &args [[buffer(0)]],
                     uint gid [[thread_position_in_grid]]) {
    if (gid == 0) {
        args.buffer[0] = 7;
    }
}
";

pub const RENDER_SRC: &str = r"
#include <metal_stdlib>
using namespace metal;

struct VertexOut {
    float4 position [[position]];
};

vertex VertexOut fullscreen_vertex(uint vertex_id [[vertex_id]]) {
    float2 positions[3] = {
        float2(-1.0, -1.0),
        float2( 3.0, -1.0),
        float2(-1.0,  3.0)
    };

    VertexOut out;
    out.position = float4(positions[vertex_id], 0.0, 1.0);
    return out;
}

fragment float4 solid_fragment() {
    return float4(0.2, 0.4, 0.8, 1.0);
}
";

pub const DYNAMIC_LIB_SRC: &str = r"
#include <metal_stdlib>
using namespace metal;

kernel void dynamic_noop(device uint *data [[buffer(0)]],
                         uint gid [[thread_position_in_grid]]) {
    if (gid == 0) {
        data[gid] += 1;
    }
}
";

pub fn artifact_path(name: &str) -> PathBuf {
    let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    dir.push("target");
    dir.push("apple-metal-example-artifacts");
    fs::create_dir_all(&dir).expect("create example artifact dir");
    let mut path = dir;
    path.push(name);
    let _ = fs::remove_file(&path);
    path
}

pub fn write_u32_words(buffer: &MetalBuffer, data: &[u32]) {
    let mut mapping = unsafe { buffer.map_write().expect("shared buffer mapping") };
    assert!(mapping.len() >= core::mem::size_of_val(data));
    for (bytes, value) in mapping.chunks_exact_mut(4).zip(data) {
        bytes.copy_from_slice(&value.to_ne_bytes());
    }
    drop(mapping);
}

pub fn read_u32_words(buffer: &MetalBuffer, len: usize) -> Vec<u32> {
    let mapping = unsafe { buffer.map_read().expect("shared buffer mapping") };
    mapping
        .chunks_exact(4)
        .take(len)
        .map(|bytes| u32::from_ne_bytes(bytes.try_into().expect("four-byte word")))
        .collect()
}

pub const fn shared_render_target(width: usize, height: usize) -> TextureDescriptor {
    let mut descriptor =
        TextureDescriptor::render_target_2d(width, height, pixel_format::BGRA8UNORM);
    descriptor.storage_mode = storage_mode::SHARED;
    descriptor.usage = texture_usage::RENDER_TARGET | texture_usage::SHADER_READ;
    descriptor
}
