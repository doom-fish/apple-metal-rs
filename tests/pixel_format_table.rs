use std::collections::BTreeSet;
use std::path::PathBuf;
use std::process::Command;

use apple_metal::{
    bytes_per_pixel, pixel_format, storage_mode, texture_usage, MetalDevice, TextureDescriptor,
    TextureTransferError,
};

fn sdk_pixel_format_header() -> Option<PathBuf> {
    let output = Command::new("xcrun")
        .args(["--sdk", "macosx", "--show-sdk-path"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let sdk = String::from_utf8(output.stdout).ok()?;
    let header = PathBuf::from(sdk.trim())
        .join("System/Library/Frameworks/Metal.framework/Headers/MTLPixelFormat.h");
    header.exists().then_some(header)
}

fn digits_after(name: &str, prefix: &str) -> usize {
    name.strip_prefix(prefix)
        .map(|rest| {
            rest.chars()
                .take_while(char::is_ascii_digit)
                .collect::<String>()
        })
        .and_then(|digits| digits.parse().ok())
        .unwrap_or_else(|| panic!("{name} has no bit count after {prefix}"))
}

fn expected_bytes(section: &str, name: &str) -> Option<usize> {
    match section {
        "Normal 8 bit formats" => Some(1),
        "Normal 16 bit formats" | "Packed 16 bit formats" => Some(2),
        "Normal 32 bit formats" | "Packed 32 bit formats" => Some(4),
        "Normal 64 bit formats" => Some(8),
        "Normal 128 bit formats" => Some(16),
        "Depth" => Some(digits_after(name, "Depth") / 8),
        "Stencil" => Some(digits_after(name, "Stencil") / 8),
        ""
        | "Compressed formats."
        | "S3TC/DXT"
        | "RGTC"
        | "BPTC"
        | "PVRTC"
        | "ETC2"
        | "ASTC"
        | "ASTC HDR (High Dynamic Range) Formats"
        | "Depth Stencil" => None,
        other => panic!("MTLPixelFormat.h has an unknown section {other:?} (before {name})"),
    }
}

fn parse_entry(line: &str) -> Option<(String, usize)> {
    let rest = line.strip_prefix("MTLPixelFormat")?;
    let (_, value) = rest.rsplit_once('=')?;
    let name: String = rest
        .chars()
        .take_while(|character| character.is_ascii_alphanumeric() || *character == '_')
        .collect();
    let value = value.trim().trim_end_matches(',').trim().parse().ok()?;
    Some((name, value))
}

#[test]
fn sizes_match_the_sdk_header_sections() {
    let Some(header) = sdk_pixel_format_header() else {
        eprintln!("skipping: no macOS SDK MTLPixelFormat.h available");
        return;
    };
    let text = std::fs::read_to_string(&header).expect("read MTLPixelFormat.h");
    let mut section = String::new();
    let mut checked = 0;
    let mut sized = BTreeSet::new();
    for raw in text.lines() {
        let line = raw.trim();
        if line.starts_with("/*") && !line.starts_with("/*!") && line.ends_with("*/") {
            section = line
                .trim_start_matches("/*")
                .trim_end_matches("*/")
                .trim()
                .to_string();
            continue;
        }
        if let Some(comment) = line.strip_prefix("//") {
            section = comment.trim().to_string();
            continue;
        }
        let Some((name, value)) = parse_entry(line) else {
            continue;
        };
        let expected = expected_bytes(&section, &name);
        assert_eq!(
            bytes_per_pixel(value),
            expected,
            "MTLPixelFormat{name} = {value} in section {section:?}"
        );
        if expected.is_some() {
            sized.insert(value);
        }
        checked += 1;
    }
    assert!(checked >= 140, "parsed only {checked} formats");
    let table: BTreeSet<usize> = (0..=4096)
        .filter(|value| bytes_per_pixel(*value).is_some())
        .collect();
    assert_eq!(table, sized);
}

#[test]
fn extended_range_formats_have_their_sdk_sizes() {
    assert_eq!(bytes_per_pixel(pixel_format::BGRA10_XR), Some(8));
    assert_eq!(bytes_per_pixel(pixel_format::BGRA10_XR_SRGB), Some(8));
    assert_eq!(bytes_per_pixel(pixel_format::BGR10_XR), Some(4));
    assert_eq!(bytes_per_pixel(pixel_format::BGR10_XR_SRGB), Some(4));
    assert_eq!(pixel_format::BGRA10_XR, 552);
    assert_eq!(pixel_format::BGRA10_XR_SRGB, 553);
}

#[test]
fn table_spot_checks() {
    for (format, expected) in [
        (pixel_format::A8UNORM, Some(1)),
        (pixel_format::R8UNORM_SRGB, Some(1)),
        (pixel_format::B5G6R5UNORM, Some(2)),
        (pixel_format::RGBA8UNORM, Some(4)),
        (pixel_format::RGB10A2UNORM, Some(4)),
        (pixel_format::RG11B10FLOAT, Some(4)),
        (pixel_format::RG32FLOAT, Some(8)),
        (pixel_format::RGBA16FLOAT, Some(8)),
        (pixel_format::RGBA32FLOAT, Some(16)),
        (pixel_format::DEPTH16UNORM, Some(2)),
        (pixel_format::DEPTH32FLOAT, Some(4)),
        (pixel_format::STENCIL8, Some(1)),
        (pixel_format::DEPTH24UNORM_STENCIL8, None),
        (pixel_format::DEPTH32FLOAT_STENCIL8, None),
        (pixel_format::X32_STENCIL8, None),
        (pixel_format::BC1_RGBA, None),
        (pixel_format::ASTC_4X4_LDR, None),
        (pixel_format::GBGR422, None),
        (pixel_format::INVALID, None),
        (pixel_format::UNSPECIALIZED, None),
        (usize::MAX, None),
    ] {
        assert_eq!(bytes_per_pixel(format), expected, "format {format}");
    }
    assert_eq!(
        pixel_format::bytes_per_pixel(pixel_format::RGBA16FLOAT),
        bytes_per_pixel(pixel_format::RGBA16FLOAT)
    );
}

#[test]
fn bgra10_xr_transfers_use_eight_bytes_per_pixel() {
    let Some(device) = MetalDevice::system_default() else {
        eprintln!("skipping: no Metal device");
        return;
    };
    let mut descriptor = TextureDescriptor::new_2d(4, 2, pixel_format::BGRA10_XR);
    descriptor.usage = texture_usage::SHADER_READ;
    descriptor.storage_mode = storage_mode::SHARED;
    let Some(texture) = device.new_texture(descriptor) else {
        eprintln!("skipping: device cannot create a shared BGRA10_XR texture");
        return;
    };
    assert_eq!(texture.pixel_format(), pixel_format::BGRA10_XR);

    let mut narrow = vec![0_u8; 4 * 4 * 2];
    let narrow_result = unsafe { texture.read_bytes_2d(&mut narrow, 16, (0, 0), (4, 2), 0) };
    assert_eq!(
        narrow_result,
        Err(TextureTransferError::BytesPerRowTooSmall {
            bytes_per_row: 16,
            minimum: 32,
        })
    );

    let pattern: Vec<u8> = (0..32_u16)
        .flat_map(|channel| ((channel * 29 % 1024) << 6).to_le_bytes())
        .collect();
    unsafe { texture.replace_region_2d(&pattern, 32, (0, 0), (4, 2), 0) }.expect("upload");
    let mut read_back = vec![0_u8; 64];
    unsafe { texture.read_bytes_2d(&mut read_back, 32, (0, 0), (4, 2), 0) }.expect("readback");
    assert_eq!(read_back, pattern);

    let short = vec![0_u8; 63];
    assert_eq!(
        unsafe { texture.replace_region_2d(&short, 32, (0, 0), (4, 2), 0) },
        Err(TextureTransferError::BufferTooShort {
            actual: 63,
            required: 64,
        })
    );
}
