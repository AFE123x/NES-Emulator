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
}

impl Controller {
    // Creates a new controller instance with no buttons pressed
    pub fn new() -> Self {
        Self {
            button: Buttons::empty(),
            index: 0,
            strobe: false,
        }
    }

    // Handles writes from the CPU, controlling the strobe signal
    pub fn cpu_write(&mut self, data: u8) {
        self.strobe = data & 1 == 1;
        if self.strobe {
            self.index = 0; // Reset index when strobe is set
        }
    }

    // Reads button state in a serial fashion, one bit at a time
    pub fn cpu_read(&mut self) -> u8 {
        if self.index > 7 {
            return 1; // Return 1 if all buttons have been read
        }
        let response = (self.button.bits() & (1 << self.index)) >> self.index;
        if !self.strobe && self.index <= 7 {
            self.index += 1; // Increment index only if strobe is low
        }
        response
    }
    
    // Sets or clears a button state based on input
    pub fn set_button(&mut self, button: Buttons, pressed: bool) {
        // Update the button state regardless of strobe state
        self.button.set(button, pressed);
    }
}
