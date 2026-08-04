use xcap::Monitor;
use image::{ImageBuffer, Rgba, Rgb, DynamicImage};
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

    // JPEG has no alpha channel support — convert RGBA to RGB, dropping transparency
    let rgb_image: ImageBuffer<Rgb<u8>, Vec<u8>> =
        DynamicImage::ImageRgba8(rgba_buffer).to_rgb8();

    let mut jpeg_bytes: Vec<u8> = Vec::new();
    let mut cursor = Cursor::new(&mut jpeg_bytes);
    rgb_image.write_to(&mut cursor, image::ImageFormat::Jpeg)?;

    Ok(CapturedFrame { width, height, jpeg_data: jpeg_bytes })
}