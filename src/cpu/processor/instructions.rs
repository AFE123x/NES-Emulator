use crate::cpu::processor::Cpu;

use super::Flags;

impl Cpu {
    /// Sets or clears a specific condition flag.
    ///
    /// # Arguments
    /// * `flag` - The flag to be modified.
    /// * `state` - `true` to set the flag, `false` to clear it.
    fn setflag(&mut self, flag: Flags, state: bool) {
        match flag {
            Flags::Negative => {
                self.flags = if state {
                    self.flags | 0x80
                } else {
                    self.flags & !0x80
                };
            }
            Flags::Overflow => {
                self.flags = if state {
                    self.flags | 0x40
                } else {
                    self.flags & !0x40
                };
            }
            Flags::Break => {
                self.flags = if state {
                    self.flags | 0x10
                } else {
                    self.flags & !0x10
                };
            }
            Flags::Decimal => {
                self.flags = if state {
                    self.flags | 0x08
                } else {
                    self.flags & !0x08
                };
            }
            Flags::Interrupt => {
                self.flags = if state {
                    self.flags | 0x04
                } else {
                    self.flags & !0x04
                };
            }
            Flags::Zeroflag => {
                self.flags = if state {
                    self.flags | 0x02
                } else {
                    self.flags & !0x02
                };
            }
            Flags::Carry => {
                self.flags = if state {
                    self.flags | 0x01
                } else {
                    self.flags & !0x01
                };
            }
        };
    }

    /// Loads an immediate value into the accumulator register (`A`).
    ///
    /// Updates the Zero and Negative flags based on the value loaded.
    pub fn LDA(&mut self) {
        self.a = self.immval;
        self.setflag(Flags::Zeroflag, self.a == 0);
        self.setflag(Flags::Negative, (self.a & 0x80) != 0);
    }

    pub fn LDX(&mut self) {
        self.x = self.immval;
        self.setflag(Flags::Zeroflag, self.x == 0);
        self.setflag(Flags::Negative, (self.x & 0x80) != 0);
    }

    pub fn LDY(&mut self) {
        self.y = self.immval;
        self.setflag(Flags::Zeroflag, self.y == 0);
        self.setflag(Flags::Negative, (self.y & 0x80) != 0);
    }

    pub fn STA(&mut self) {
        self.write(self.abs_addr, self.a);
    }

    pub fn STX(&mut self) {
        self.write(self.abs_addr, self.x);
    }
    pub fn STY(&mut self) {
        self.write(self.abs_addr, self.y);
    }

    pub fn TAX(&mut self){
        self.x = self.a;
        self.setflag(Flags::Zeroflag, self.x == 0);
        self.setflag(Flags::Negative, (self.x & 0x80) != 0);
    }

    pub fn TAY(&mut self){
        self.y = self.a;
        self.setflag(Flags::Zeroflag, self.y == 0);
        self.setflag(Flags::Negative, (self.y & 0x80) != 0);
    }
    pub fn TXA(&mut self){
        self.a = self.x;
        self.setflag(Flags::Zeroflag, self.a == 0);
        self.setflag(Flags::Negative, (self.a & 0x80) != 0);
    }
    pub fn TYA(&mut self){
        self.a = self.y;
        self.setflag(Flags::Zeroflag, self.a == 0);
        self.setflag(Flags::Negative, (self.a & 0x80) != 0);
    }

    pub fn TSX(&mut self){
        self.x = self.sp;
        self.setflag(Flags::Zeroflag, self.x == 0);
        self.setflag(Flags::Negative, (self.x & 0x80) != 0);
    }

    pub fn TXS(&mut self){
        self.sp = self.x;
    }

    pub fn PHA(&mut self){
        self.write((0x0100 as u16).wrapping_add(self.sp as u16), self.a);
        self.sp = self.sp.wrapping_sub(1);
    }

    pub fn PHP(&mut self){
        self.write((0x0100 as u16).wrapping_add(self.sp as u16), self.flags);
        self.sp = self.sp.wrapping_sub(1);
    }

    pub fn PLA(&mut self){
        self.sp = self.sp.wrapping_add(1);
        self.a = self.read((0x0100 as u16).wrapping_add(self.sp as u16));
        self.setflag(Flags::Zeroflag, self.a == 0);
        self.setflag(Flags::Negative, (self.a & 0x80) != 0);
    }

    pub fn PLP(&mut self){
        self.sp = self.sp.wrapping_add(1);
        self.flags = self.read((0x0100 as u16).wrapping_add(self.sp as u16));
    }

    pub fn AND(&mut self){
        self.a = self.a & self.immval;
        self.setflag(Flags::Zeroflag, self.a == 0);
        self.setflag(Flags::Negative, (self.a & 0x80) != 0);
    }

    pub fn EOR(&mut self){
        self.a = self.a ^ self.immval;
        self.setflag(Flags::Zeroflag, self.a == 0);
        self.setflag(Flags::Negative, (self.a & 0x80) != 0);
    }

    pub fn ORA(&mut self){
        self.a = self.a | self.immval;
        self.setflag(Flags::Zeroflag, self.a == 0);
        self.setflag(Flags::Negative, (self.a & 0x80) != 0);
    }

    pub fn BIT(&mut self){
        let temp = self.a & self.immval;
        self.setflag(Flags::Negative,(temp & 0x80) != 0);
        self.setflag(Flags::Overflow,(temp & 0x40) != 0);
        self.setflag(Flags::Zeroflag,temp == 0);
    }
}

#[cfg(test)]
mod tests {
    use crate::bus::cpubus::Cpubus;
    use crate::cpu::processor::Flags;
    use crate::Cpu;

    #[test]
    /// Tests loading an immediate value (0x45) into the accumulator.
    fn cpu_test1() {
        let mut cpu = Cpu::new();
        let mut bus = Cpubus::new(&mut cpu);
        cpu.linkbus(&mut bus);
        bus.cpu_write(0x8000, 0xA9); // LDA immediate instruction
        bus.cpu_write(0x8001, 0x45); // Value to load
        bus.clock();

        assert_eq!(cpu.get_accumulator(), 69, "{} != 69", cpu.get_accumulator());
    }

    #[test]
    /// Tests loading a value (0xFF) from memory into the accumulator using LDA (zero page addressing).
    fn cpu_test2() {
        let mut cpu = Cpu::new();
        let mut bus = Cpubus::new(&mut cpu);
        cpu.linkbus(&mut bus);
        bus.cpu_write(0x8000, 0xA5); // LDA zero page
        bus.cpu_write(0x8001, 0x45); // Address 0x45
        bus.cpu_write(0x0045, 0xFF); // Store 0xFF at 0x45
        bus.clock();

        assert_eq!(
            cpu.get_accumulator(),
            255,
            "{} != 255",
            cpu.get_accumulator()
        );
    }

    #[test]
    /// Tests indexed zero page addressing mode with the X register.
    fn cpu_test3() {
        let mut cpu = Cpu::new();
        let mut bus = Cpubus::new(&mut cpu);
        cpu.linkbus(&mut bus);
        bus.cpu_write(0x8000, 0xB5); // LDA zero page, X indexed
        bus.cpu_write(0x8001, 0x44); // Base address 0x44
        cpu.set_x(1); // Set X to 1
        bus.cpu_write(0x0045, 0xFF); // Store 0xFF at 0x45 (0x44 + X)
        bus.clock();

        assert_eq!(
            cpu.get_accumulator(),
            255,
            "{} != 255",
            cpu.get_accumulator()
        );
    }

    #[test]
    /// Tests whether the zero flag is correctly set when loading a zero value into the accumulator.
    fn cpu_zero_flag() {
        let mut cpu = Cpu::new();
        let mut bus = Cpubus::new(&mut cpu);
        cpu.linkbus(&mut bus);
        bus.cpu_write(0x8000, 0xB5); // LDA zero page, X indexed
        bus.cpu_write(0x8001, 0x44); // Base address 0x44
        cpu.set_x(1);
        bus.cpu_write(0x0045, 0x00); // Store 0 at 0x45
        bus.clock();

        assert_eq!(cpu.get_flag() & 0x02, 0x02, "Zero flag not enabled!");
    }

    #[test]
    /// Tests whether the negative flag is correctly set when loading a negative value (0xFF) into the accumulator.
    fn cpu_negative_flag() {
        let mut cpu = Cpu::new();
        let mut bus = Cpubus::new(&mut cpu);
        cpu.linkbus(&mut bus);
        bus.cpu_write(0x8000, 0xB5); // LDA zero page, X indexed
        bus.cpu_write(0x8001, 0x44); // Base address 0x44
        cpu.set_x(1);
        bus.cpu_write(0x0045, 0xFF); // Store 0xFF at 0x45
        bus.clock();

        assert_eq!(cpu.get_flag() & 0x80, 0x80, "Negative flag not enabled!");
    }

    #[test]
    fn cpu_ldx_test1() {
        //using zero page y
        let mut cpu = Cpu::new();
        let mut bus = Cpubus::new(&mut cpu);
        cpu.linkbus(&mut bus);
        bus.cpu_write(0x8000, 0xB6); // LDA zero page, X indexed
        bus.cpu_write(0x8001, 0x44); // Base address 0x44
        cpu.set_y(1);
        bus.cpu_write(0x0045, 0xFF); // Store 0xFF at 0x45
        bus.clock();

        assert_eq!(cpu.get_x(), 0xFF, "LDX (ZPY) - FAILED!");
    }
    #[test]
    pub fn cpu_ldy_test1() {
        let mut cpu = Cpu::new();
        let mut bus = Cpubus::new(&mut cpu);
        cpu.linkbus(&mut bus);
        bus.cpu_write(0x8000, 0xAC); // LDA zero page, X indexed
        bus.cpu_write(0x8001, 0xAD); // Base address 0x44
        bus.cpu_write(0x8002, 0xDE); // Store 0xFF at 0x45
        bus.cpu_write(0xDEAD, 0x45);
        bus.clock();

        assert_eq!(cpu.get_y(), 69, "LDY (ABS) - FAILED!");
    }
    #[test]
    pub fn cpu_store_accumulator() {
        let mut cpu = Cpu::new();
        let mut bus = Cpubus::new(&mut cpu);
        cpu.linkbus(&mut bus);
        bus.cpu_write(0x8000, 0x81);
        bus.cpu_write(0x8001, 0x44);
        cpu.set_x(1);
        bus.cpu_write(0x0045, 0xAD);
        bus.cpu_write(0x0046, 0xDE);
        cpu.set_a(0xFF);
        bus.cpu_write(0xDEAD, 0x0);
        bus.clock();
        assert_eq!(bus.cpu_read(0xDEAD, false), 0xFF, "STA test - FAILED!");
    }
    #[test]
    pub fn cpu_store_x() {
        let mut cpu = Cpu::new();
        let mut bus = Cpubus::new(&mut cpu);
        cpu.linkbus(&mut bus);
        bus.cpu_write(0x8000, 0x8E);
        bus.cpu_write(0x8001, 0xAD);
        bus.cpu_write(0x8002, 0xDE);
        cpu.set_x(0x45);
        bus.clock();
        assert_eq!(bus.cpu_read(0xDEAD, false),69,"STX test - FAILED!")
    }
    #[test]
    pub fn cpu_store_y() {
        let mut cpu = Cpu::new();
        let mut bus = Cpubus::new(&mut cpu);
        cpu.linkbus(&mut bus);
        bus.cpu_write(0x8000, 0x8C);
        bus.cpu_write(0x8001, 0xAD);
        bus.cpu_write(0x8002, 0xDE);
        cpu.set_y(0x45);
        bus.clock();
        assert_eq!(bus.cpu_read(0xDEAD, false),69,"STX test - FAILED!")
    }

    #[test]
    pub fn cpu_tax(){
        let mut cpu = Cpu::new();
        let mut bus = Cpubus::new(&mut cpu);
        cpu.linkbus(&mut bus);
        bus.cpu_write(0x8000, 0xAA);
        cpu.set_a(0x45);
        bus.clock();
        assert_eq!(cpu.get_x(),0x45,"TAX INSTRUCTION - FAILED!");
    }

    #[test]
    pub fn cpu_tay(){
        let mut cpu = Cpu::new();
        let mut bus = Cpubus::new(&mut cpu);
        cpu.linkbus(&mut bus);
        bus.cpu_write(0x8000, 0xA8);
        cpu.set_a(0x45);
        bus.clock();
        assert_eq!(cpu.get_y(),0x45,"TAY INSTRUCTION - FAILED!");
    }

    #[test]
    pub fn cpu_txa(){
        let mut cpu = Cpu::new();
        let mut bus = Cpubus::new(&mut cpu);
        cpu.linkbus(&mut bus);
        bus.cpu_write(0x8000, 0x8A);
        cpu.set_x(0x45);
        bus.clock();
        assert_eq!(cpu.get_a(),0x45,"TAY INSTRUCTION - FAILED!");
    }
    #[test]
    pub fn cpu_tya(){
        let mut cpu = Cpu::new();
        let mut bus = Cpubus::new(&mut cpu);
        cpu.linkbus(&mut bus);
        bus.cpu_write(0x8000, 0x98);
        cpu.set_y(0x45);
        bus.clock();
        assert_eq!(cpu.get_a(),0x45,"TAY INSTRUCTION - FAILED!");
    }

    #[test]
    pub fn cpu_stackops(){
        let mut cpu = Cpu::new();
        let mut bus = Cpubus::new(&mut cpu);
        cpu.linkbus(&mut bus);
        cpu.set_a(0x45);
        bus.cpu_write(0x8000, 0x48); //PHA - 3
        cpu.set_sflag(0x80);
        bus.cpu_write(0x8001,0x8); //PHP - 3
        bus.cpu_write(0x8002,0xA9); //LDA - 2
        bus.cpu_write(0x8003,0);
        bus.cpu_write(0x8002,0x28); //PLP - 4
        bus.cpu_write(0x8002,0x68); //PLA - 4
        cpu.set_y(0x45);
        for _ in 0..16{
            bus.clock();
        }
        assert_eq!(cpu.get_a(),0x45,"PLA instruction - FAILED!");
        assert_eq!(cpu.get_sflag(),0x80,"PLP instruction - FAILED!");
    }

    #[test]
    pub fn cpu_andtest(){
        let mut cpu = Cpu::new();
        let mut bus = Cpubus::new(&mut cpu);
        cpu.linkbus(&mut bus);
        bus.cpu_write(0x8000,0x29);
        bus.cpu_write(0x8001,0x03);
        cpu.set_a(75);
        bus.clock();
        assert_eq!(cpu.get_a(),75 & 3,"AND test - FAILED!");
    }

    #[test]
    pub fn cpu_eortest(){
        let mut cpu = Cpu::new();
        let mut bus = Cpubus::new(&mut cpu);
        cpu.linkbus(&mut bus);
        bus.cpu_write(0x8000,0x49);
        bus.cpu_write(0x8001,0x46);
        cpu.set_a(0x46);
        bus.clock();
        assert_eq!(cpu.get_a(),0x46 ^ 0x46,"EOR test - FAILED!");
    }

    #[test]
    pub fn cpu_oratest(){
        let mut cpu = Cpu::new();
        let mut bus = Cpubus::new(&mut cpu);
        cpu.linkbus(&mut bus);
        bus.cpu_write(0x8000,0x9);
        bus.cpu_write(0x8001,0x7F);
        cpu.set_a(0x80);
        bus.clock();
        assert_eq!(cpu.get_a(),0xFF,"ORA test - FAILED!");
    }

    #[test]
    pub fn cpu_bittest(){
        let mut cpu = Cpu::new();
        let mut bus = Cpubus::new(&mut cpu);
        cpu.linkbus(&mut bus);
        bus.cpu_write(0x8000,0x24);
        bus.cpu_write(0x8001,0x7F);
        bus.cpu_write(0x7F as u16,0);
        cpu.set_a(0x80);
        bus.clock();
        assert_eq!(cpu.get_sflag(),0x02,"ORA test - FAILED!");
    }
}
