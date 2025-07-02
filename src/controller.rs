use bitflags::bitflags;

// Define button mappings using bitflags
bitflags! {
    pub struct Buttons: u8 {
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

// NES Controller struct to handle button states and communication
pub struct Controller {
    button: Buttons, // Holds the current button press states
    strobe: bool,    // Strobe signal for reading input
    index: u8,       // Current bit index for reading button states
    dataread: bool,
}

impl Controller {
    // Creates a new controller instance with no buttons pressed
    pub fn new() -> Self {
        Self {
            button: Buttons::empty(),
            index: 0,
            strobe: false,
            dataread: false,
        }
    }
    // Handles writes from the CPU, controlling the strobe signal
    pub fn cpu_write(&mut self, data: u8) {
        self.strobe = data & 1 == 1;
        if self.strobe {
            self.index = 0; // Reset index when strobe is set
        }
    }

    pub fn readfullregister(&mut self) -> bool{
        let result = self.dataread;
        self.dataread = false;
        result
    }
    // Reads button state in a serial fashion, one bit at a time
    pub fn cpu_read(&mut self) -> u8 {
        if self.index > 7 {
            self.dataread = true;
            return 1;
        } 
        let response = (self.button.bits() & (1 << self.index)) >> self.index;
        
        if !self.strobe && self.index <= 7 {
            self.index += 1;
        }
        
        response
    }
    pub fn _set_reg_value(&mut self, byte: u8){
        self.button = Buttons::from_bits_truncate(byte);
    } 

    pub fn _get_reg_value(&self) -> u8{
        self.button.bits()
    }
    // Sets or clears a button state based on input
    pub fn set_button(&mut self, button: Buttons, pressed: bool) {
        self.button.set(button, pressed);
    }
}