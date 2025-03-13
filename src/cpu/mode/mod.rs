use super::Cpu;

impl Cpu {
    pub fn implicit(&mut self) {
    }

    pub fn accumulator(&mut self) {
    }

    pub fn immediate(&mut self) {
        self.addrabs = self.pc;
        self.immval = self.cpu_read(self.pc,true);
        self.pc = self.pc.wrapping_add(1);
    }
    pub fn zeropage(&mut self) {
        self.addrabs = self.cpu_read(self.pc,false) as u16;
        self.addrabs = self.addrabs & 0xFF;
        self.pc = self.pc.wrapping_add(1);
        self.immval = self.cpu_read(self.addrabs,true);

    }

    pub fn zeropagex(&mut self) {
        self.addrabs = self.cpu_read(self.pc,false) as u16;
        self.pc = self.pc.wrapping_add(1);
        self.addrabs = self.addrabs.wrapping_add(self.x as u16);
        self.addrabs = self.addrabs & 0xFF;
        self.immval = self.cpu_read(self.addrabs,true);
    }

    pub fn zeropagey(&mut self) {
        self.addrabs = self.cpu_read(self.pc,false) as u16;
        self.pc = self.pc.wrapping_add(1);
        self.addrabs = self.addrabs.wrapping_add(self.y as u16);
        self.addrabs = self.addrabs & 0xFF;
        self.immval = self.cpu_read(self.addrabs,true);
    }

    pub fn relative(&mut self) {
        let temp = self.cpu_read(self.pc,false) as i8;
        self.pc = self.pc.wrapping_add(1);
        self.relval = temp as u16;
        if self.relval & 0x80 != 0 {
            self.relval |= 0xFF00;
        }
    }

    pub fn absolute(&mut self) {
        let lo = self.cpu_read(self.pc,false) as u16;
        self.pc = self.pc.wrapping_add(1);
        let hi = self.cpu_read(self.pc,false) as u16;
        self.pc = self.pc.wrapping_add(1);
        self.addrabs = (hi << 8) | lo;
        self.immval = self.cpu_read(self.addrabs,true);
    }
    pub fn absolutex(&mut self) {
        let lo = self.cpu_read(self.pc,false) as u16;
        self.pc = self.pc.wrapping_add(1);
        let hi = self.cpu_read(self.pc,false) as u16;
        self.pc = self.pc.wrapping_add(1);
        self.addrabs = (hi << 8) | lo;
        let temp = self.addrabs;
        self.addrabs = self.addrabs.wrapping_add(self.x as u16);
        self.immval = self.cpu_read(self.addrabs,true);
        if self.addrabs & 0xFF00 != temp & 0xFF00 {
            self.cycles_left = self.cycles_left.wrapping_add(1);
        }
    }

    pub fn absolutey(&mut self) {
        let lo = self.cpu_read(self.pc,false) as u16;
        self.pc = self.pc.wrapping_add(1);
        let hi = self.cpu_read(self.pc,false) as u16;
        self.pc = self.pc.wrapping_add(1);
        self.addrabs = (hi << 8) | lo;
        let temp = self.addrabs;
        self.addrabs = self.addrabs.wrapping_add(self.y as u16);
        self.immval = self.cpu_read(self.addrabs,true);
        if self.addrabs & 0xFF00 != temp & 0xFF00 {
            self.cycles_left = self.cycles_left.wrapping_add(1);
        }
    }

    pub fn indirect(&mut self) {
        let lo = self.cpu_read(self.pc,false) as u16;
        self.pc = self.pc.wrapping_add(1);
        let hi = self.cpu_read(self.pc,false) as u16;
        self.pc = self.pc.wrapping_add(1);
        self.addrabs = (hi << 8) | lo;
    
        // Read the low byte at the target address
        let lo = self.cpu_read(self.addrabs,false) as u16;
    
        // Simulate the page boundary wraparound bug of the 6502
        let hi_location = (self.addrabs & 0xFF00) | ((self.addrabs + 1) & 0x00FF);
        let hi = self.cpu_read(hi_location,false) as u16;
    
        self.addrabs = (hi << 8) | lo;
    
        // Fetch the actual value from the calculated address
        self.immval = self.cpu_read(self.addrabs,true);
    }
    

    pub fn idx(&mut self) {
        let temp = self.cpu_read(self.pc,false) as u16;
        self.pc = self.pc.wrapping_add(1);
        let lo = self.cpu_read((temp + (self.x as u16)) & 0xFF,false) as u16;
        let hi = self.cpu_read((temp + (self.x as u16) + 1) & 0xFF,false) as u16;
        self.addrabs = (hi << 8) | lo;
        self.immval = self.cpu_read(self.addrabs,true);
    }
    pub fn idy(&mut self) {
        let temp = self.cpu_read(self.pc,false) as u16; //the byte from the zero page
        self.pc = self.pc.wrapping_add(1);
        let lo = self.cpu_read(temp,false) as u16;
        let hi = self.cpu_read(temp + 1,false) as u16;
        self.addrabs = (hi << 8) | lo;
        self.addrabs = self.addrabs.wrapping_add(self.y as u16);
        self.immval = self.cpu_read(self.addrabs,true);
        if self.addrabs & 0xFF00 != temp & 0xFF00 {
            self.cycles_left = self.cycles_left.wrapping_add(1);
        }
    }
}
