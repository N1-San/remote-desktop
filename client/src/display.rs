use minifb::{Window, WindowOptions};

pub struct FrameDisplay {
    window: Window,
}

impl FrameDisplay {
    pub fn new(width: u32, height: u32) -> Result<Self, Box<dyn std::error::Error>> {
        let window = Window::new(
            "Remote Desktop - Live Stream",
            width as usize,
            height as usize,
            WindowOptions::default(),
        )?;
        Ok(Self { window })
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open() && !self.window.is_key_down(minifb::Key::Escape)
    }

    pub fn render_frame(&mut self, width: u32, height: u32, jpeg_data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let img = image::load_from_memory(jpeg_data)?.to_rgb8();
        let width = width as usize;
        let height = height as usize;

        let mut buffer: Vec<u32> = Vec::with_capacity(width * height);
        for pixel in img.pixels() {
            let [r, g, b] = pixel.0;
            buffer.push(((r as u32) << 16) | ((g as u32) << 8) | (b as u32));
        }

        self.window.update_with_buffer(&buffer, width, height)?;
        Ok(())
    }
}