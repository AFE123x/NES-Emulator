use bitflags::bitflags;

pub struct controller {
    controller: u8,
    controller_state: u8,
}

pub enum Button {
    A,
    B,
    UP,
    DOWN,
    LEFT,
    RIGHT,
    START,
    SELECT,
}

impl controller {
    pub fn new() -> Self {
        Self {
            controller: 0,
            controller_state: 0,
        }
    }

    pub fn set_button(&mut self,button: Button){
        match button{ //A, B, Select, Start, Up, Down, Left, Right. 
            Button::A => self.controller |= 0x80,
            Button::B => self.controller |= 0x40,
            Button::UP => self.controller |= 0x8,
            Button::DOWN => self.controller |= 0x4,
            Button::LEFT => self.controller |= 0x2,
            Button::RIGHT => self.controller |= 0x4,
            Button::START => self.controller |= 0x10,
            Button::SELECT => self.controller |= 0x20,
        }
    }
    pub fn unset_button(&mut self, button: Button){
        match button{ //A, B, Select, Start, Up, Down, Left, Right. 
            Button::A => self.controller &= !0x80,
            Button::B => self.controller &= !0x40,
            Button::UP => self.controller &= !0x8,
            Button::DOWN => self.controller &= !0x4,
            Button::LEFT => self.controller &= !0x2,
            Button::RIGHT => self.controller &= !0x4,
            Button::START => self.controller &= !0x10,
            Button::SELECT => self.controller &= !0x20,
        }
    }
}
