/// A simple RGB framebuffer abstraction that stores pixels as 32-bit integers (0xRRGGBB).
/// Designed for use with NES PPU output or similar graphics rendering.
pub struct Frame {
    buffer: Vec<u32>, // Stores pixel data in row-major order (0xRRGGBB format).
    width: u16,       // Width of the frame in pixels.
    // height: u16,    // Height is currently unused, but could be stored if needed.
}

impl Frame {
    /// Creates a new frame buffer with the specified width and height.
    /// Initializes all pixels to black (0x000000).
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            buffer: vec![0; width as usize * height as usize],
            width,
            // height,
        }
    }

    /// Draws a single pixel at coordinates (x, y) with the given RGB color.
    /// The color tuple (r, g, b) is packed into a 32-bit integer as 0xRRGGBB.
    pub fn drawpixel(&mut self, x: u16, y: u16, color: (u8, u8, u8)) {
        let index = (y as usize * self.width as usize) + (x as usize);
        self.buffer[index] = ((color.0 as u32) << 16) | ((color.1 as u32) << 8) | (color.2 as u32);
    }

    /// Returns an immutable reference to the internal buffer.
    /// Useful for passing to a renderer or GUI framework.
    pub fn get_buf(&self) -> &Vec<u32> {
        &self.buffer
    }
}