use super::Cpu;

impl Cpu {
    pub fn implicit(&mut self) {
    }

    pub fn accumulator(&mut self) {
    }

    pub fn immediate(&mut self) {
        self.addrabs = self.pc;
        self.immval = self.cpu_read(self.pc);
        self.pc = self.pc.wrapping_add(1);
    }
    pub fn zeropage(&mut self) {
        self.addrabs = self.cpu_read(self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);
        self.immval = self.cpu_read(self.addrabs);

    }

    pub fn zeropagex(&mut self) {
        self.addrabs = self.cpu_read(self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);
        self.addrabs = self.addrabs.wrapping_add(self.x as u16);
        self.immval = self.cpu_read(self.addrabs);
    }

    pub fn zeropagey(&mut self) {
        self.addrabs = self.cpu_read(self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);
        self.addrabs = self.addrabs.wrapping_add(self.y as u16);
        self.immval = self.cpu_read(self.addrabs);
    }

    pub fn relative(&mut self) {
        let temp = self.cpu_read(self.pc) as i8;
        self.pc = self.pc.wrapping_add(1);
        self.relval = temp as u16;
        if self.relval & 0x80 != 0 {
            self.relval |= 0xFF00;
        }
    }

    pub fn absolute(&mut self) {
        let lo = self.cpu_read(self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);
        let hi = self.cpu_read(self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);
        self.addrabs = (hi << 8) | lo;
        self.immval = self.cpu_read(self.addrabs);
    }
    pub fn absolutex(&mut self) {
        let lo = self.cpu_read(self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);
        let hi = self.cpu_read(self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);
        self.addrabs = (hi << 8) | lo;
        let temp = self.addrabs;
        self.addrabs = self.addrabs.wrapping_add(self.x as u16);
        self.immval = self.cpu_read(self.addrabs);
        if self.addrabs & 0xFF00 != temp & 0xFF00 {
            self.cycles_left = self.cycles_left.wrapping_add(1);
        }
    }

    pub fn absolutey(&mut self) {
        let lo = self.cpu_read(self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);
        let hi = self.cpu_read(self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);
        self.addrabs = (hi << 8) | lo;
        let temp = self.addrabs;
        self.addrabs = self.addrabs.wrapping_add(self.y as u16);
        self.immval = self.cpu_read(self.addrabs);
        if self.addrabs & 0xFF00 != temp & 0xFF00 {
            self.cycles_left = self.cycles_left.wrapping_add(1);
        }
    }

    pub fn indirect(&mut self) {
        let lo = self.cpu_read(self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);
        let hi = self.cpu_read(self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);
        self.addrabs = (hi << 8) | lo;
        let lo = self.cpu_read(self.addrabs) as u16;
        let hi = self.cpu_read(self.addrabs.wrapping_add(1)) as u16;
        self.addrabs = (hi << 8) | lo;
        self.immval = self.cpu_read(self.addrabs);
    }

    pub fn indexedindirect(&mut self) {
        self.addrabs = self.cpu_read(self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);
        self.addrabs = self.addrabs.wrapping_add(self.x as u16);
        self.addrabs = self.addrabs & 0xFF;
        let lo = self.cpu_read(self.addrabs) as u16;
        let hi = self.cpu_read(self.addrabs.wrapping_add(1)) as u16;
        self.addrabs = (hi << 8) | lo;
        self.immval = self.cpu_read(self.addrabs);
    }

    pub fn indirectindexed(&mut self) {
        self.addrabs = self.cpu_read(self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);
        let lo = self.cpu_read(self.addrabs) as u16;
        let hi = self.cpu_read(self.addrabs.wrapping_add(1)) as u16;
        self.addrabs = (hi << 8) | lo;
        let temp = self.addrabs;
        self.addrabs = self.addrabs.wrapping_add(self.y as u16);
        self.addrabs = self.addrabs & 0xFF;
        self.immval = self.cpu_read(self.addrabs);
        if self.addrabs & 0xFF00 != temp & 0xFF00 {
            self.cycles_left = self.cycles_left.wrapping_add(1);
        }
    }
}
