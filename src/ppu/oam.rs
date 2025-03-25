use crate::ppu::Ppu;

pub struct oam {
    y_position: u16,
    index_number: u8,
    attribute: u8,
    x_position: u8,
}

impl Clone for oam {
    fn clone(&self) -> Self {
        Self {
            y_position: self.y_position,
            index_number: self.index_number,
            attribute: self.attribute,
            x_position: self.x_position,
        }
    }
}
impl oam {
    pub fn new() -> Self {
        Self {
            y_position: 0,
            index_number: 0,
            attribute: 0,
            x_position: 0,
        }
    }
    pub fn print_oam(&mut self) -> String {
        format!(
            "y_pos: {}, index: {:2x}, attribute: {:2x}, x_pos: {}",
            self.y_position, self.index_number, self.attribute, self.x_position
        )
    }

    pub fn get_y_position(&self) -> u16 {
        self.y_position
    }
    pub fn get_x_position(&self) -> u8 {
        self.x_position
    }
    pub fn get_attribute(&self) -> u8 {
        self.attribute
    }
    pub fn get_index_number(&self) -> u8 {
        self.index_number
    }

    pub fn set_byte(&mut self, address: u8, data: u8) {
        match address % 4 {
            0 => {
                self.y_position = data as u16;
            }
            1 => {
                self.index_number = data;
            }
            2 => {
                self.attribute = data;
            }
            3 => {
                self.x_position = data;
            }
            _ => {
                panic!("erm, what the sigma");
            }
        };
    }

    pub fn get_byte(&mut self, address: u8) -> u8 {
        match address % 4 {
            0 => self.y_position as u8,
            1 => self.index_number,
            2 => self.attribute,
            3 => self.x_position,
            _ => {
                panic!("erm, what the sigma");
            }
        }
    }
}
impl Ppu {}
