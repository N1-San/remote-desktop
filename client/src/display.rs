use minifb::{Window, WindowOptions};

pub fn show_frame(width: u32, height: u32, jpeg_data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let img = image::load_from_memory(jpeg_data)?.to_rgb8();

    let width = width as usize;
    let height = height as usize;

    // minifb expects one u32 per pixel, packed as 0x00RRGGBB
    let mut buffer: Vec<u32> = Vec::with_capacity(width * height);
    for pixel in img.pixels() {
        let [r, g, b] = pixel.0;
        buffer.push(((r as u32) << 16) | ((g as u32) << 8) | (b as u32));
    }

    let mut window = Window::new(
        "Remote Desktop - Received Frame",
        width,
        height,
        WindowOptions::default(),
    )?;

    // Static single-frame viewer for now — keep window open until closed
    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        window.update_with_buffer(&buffer, width, height)?;
    }

    Ok(())
}