use xcap::Monitor;
use image::{ImageBuffer, Rgba, Rgb, DynamicImage};
use image::codecs::jpeg::JpegEncoder;
use std::io::Cursor;

pub struct CapturedFrame {
    pub width: u32,
    pub height: u32,
    pub jpeg_data: Vec<u8>,
}

pub fn capture_primary_monitor() -> Result<CapturedFrame, Box<dyn std::error::Error>> {
    let monitors = Monitor::all()?;

    if monitors.is_empty() {
        return Err("No monitors detected at all".into());
    }

    let monitor = monitors
        .iter()
        .find(|m| m.is_primary())
        .or_else(|| monitors.first())
        .ok_or("No monitors available")?;

    let image = monitor.capture_image()?;
    let width = image.width();
    let height = image.height();

    let rgba_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_raw(width, height, image.into_raw())
            .ok_or("Failed to build image buffer from captured pixels")?;

    let rgb_image: ImageBuffer<Rgb<u8>, Vec<u8>> =
        DynamicImage::ImageRgba8(rgba_buffer).to_rgb8();

    let mut jpeg_bytes: Vec<u8> = Vec::new();
    let mut cursor = Cursor::new(&mut jpeg_bytes);

    // Quality 0-100; higher = better image, larger file. 85 is a solid quality/size middle ground.
    let mut encoder = JpegEncoder::new_with_quality(&mut cursor, 85);
    encoder.encode(
        rgb_image.as_raw(),
        width,
        height,
        image::ExtendedColorType::Rgb8,
    )?;

    Ok(CapturedFrame { width, height, jpeg_data: jpeg_bytes })
}