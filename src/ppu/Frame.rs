pub struct Frame {
    buffer: Vec<u8>,
    width: u16,
    height: u16,
}

impl Frame {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            buffer: vec![0;(width as usize * 3 * height as usize) as usize],
            width,
            height,
        }
    }

    pub fn drawpixel(&mut self, x: u16, y: u16, color: (u8, u8, u8)){
        if x >= self.width || y >= self.height{
            return;
        }
        let index = (y as usize * self.width as usize * 3) + (x as usize * 3);
        self.buffer[index] = color.0;
        self.buffer[index + 1] = color.1;
        self.buffer[index + 2] = color.2;
    }

    pub fn get_buf(&self) -> &Vec<u8>{
        &self.buffer
    }
}
