pub struct Frame {
    buffer: Vec<u32>,
    width: u16,
    height: u16,
}

impl Frame {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            buffer: vec![0;(width as usize * height as usize) as usize],
            width,
            height,
        }
    }

    pub fn drawpixel(&mut self, x: u16, y: u16, color: (u8, u8, u8)){
        let index = (y as usize * self.width as usize) + (x as usize);
        self.buffer[index] = ((color.0 as u32) << 16) | ((color.1 as u32) << 8) | ((color.2 as u32));
    }

    pub fn get_buf(&self) -> &Vec<u32>{
        &self.buffer
    }
}
