use bitflags::bitflags;

bitflags! {
    pub struct Buttons: u8{
        const A = 0b0000_0001;
        const B = 0b0000_0010;
        const Select = 0b0000_0100;
        const Start = 0b0000_1000;
        const Up = 0b0001_0000;
        const Down = 0b0010_0000;
        const Left = 0b0100_0000;
        const Right = 0b1000_0000;
    }
}

pub struct Controller {
    button: Buttons,
    strobe: bool,
    index: u8,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            button: Buttons::empty(),
            index: 0,
            strobe: false,
        }
    }

    pub fn cpu_write(&mut self, data: u8){
        self.strobe = data & 1 == 1;
        if self.strobe{
            self.index = 0;
        }
    }

    pub fn cpu_read(&mut self) -> u8{
        if self.index > 7{
            return 1;
        }
        let response = (self.button.bits() & (1 << self.index)) >> self.index;
        if !self.strobe && self.index <= 7{
            self.index += 1;
        }
        response
    }
    
    pub fn set_button(&mut self, button: Buttons, pressed: bool){
        // Update the button state regardless of strobe state
        self.button.set(button, pressed);
    }
}
