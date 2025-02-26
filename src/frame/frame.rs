pub struct Frame{
    buf: Vec<u8>,
    xsize: usize,
    ysize: usize,
}

impl Frame{
    pub fn new(x: usize, y: usize) -> Self{
        Self{
            buf: vec![0;x * y * 3],
            xsize: x,
            ysize: y,
        }
    }

    pub fn draw_pixel(&mut self, x: usize, y: usize, color:(u8,u8,u8)){
        let base = x * 3 * self.xsize + x * 3;
        if base + 2 < self.buf.len(){
            self.buf[base] = color.0;
            self.buf[base + 1] = color.1;
            self.buf[base + 2] = color.2;
        }
    }

    pub fn get_buf(&self) -> &Vec<u8>{
        &self.buf
    }
}