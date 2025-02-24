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

    fn getflag(&mut self, flag: Flags) -> bool {
        let toreturn = match flag {
            Flags::Negative => self.flags & 0x80,
            Flags::Overflow => self.flags & 0x40,
            Flags::Break => self.flags & 0x10,
            Flags::Decimal => self.flags & 0x8,
            Flags::Interrupt => self.flags & 0x4,
            Flags::Zeroflag => self.flags & 0x2,
            Flags::Carry => self.flags & 0x1,
        };
        toreturn != 0
    }

    /// Loads an immediate value into the accumulator register (`A`).
    ///
    /// Updates the Zero and Negative flags based on the value loaded.
    pub fn lda(&mut self) {
        self.a = self.immval;
        self.setflag(Flags::Zeroflag, self.a == 0);
        self.setflag(Flags::Negative, (self.a & 0x80) != 0);
    }

    pub fn ldx(&mut self) {
        self.x = self.immval;
        self.setflag(Flags::Zeroflag, self.x == 0);
        self.setflag(Flags::Negative, (self.x & 0x80) != 0);
    }

    pub fn ldy(&mut self) {
        self.y = self.immval;
        self.setflag(Flags::Zeroflag, self.y == 0);
        self.setflag(Flags::Negative, (self.y & 0x80) != 0);
    }

    pub fn sta(&mut self) {
        self.write(self.abs_addr, self.a);
    }

    pub fn stx(&mut self) {
        self.write(self.abs_addr, self.x);
    }
    pub fn sty(&mut self) {
        self.write(self.abs_addr, self.y);
    }

    pub fn tax(&mut self) {
        self.x = self.a;
        self.setflag(Flags::Zeroflag, self.x == 0);
        self.setflag(Flags::Negative, (self.x & 0x80) != 0);
    }

    pub fn tay(&mut self) {
        self.y = self.a;
        self.setflag(Flags::Zeroflag, self.y == 0);
        self.setflag(Flags::Negative, (self.y & 0x80) != 0);
    }
    pub fn txa(&mut self) {
        self.a = self.x;
        self.setflag(Flags::Zeroflag, self.a == 0);
        self.setflag(Flags::Negative, (self.a & 0x80) != 0);
    }
    pub fn tya(&mut self) {
        self.a = self.y;
        self.setflag(Flags::Zeroflag, self.a == 0);
        self.setflag(Flags::Negative, (self.a & 0x80) != 0);
    }

    pub fn tsx(&mut self) {
        self.x = self.sp;
        self.setflag(Flags::Zeroflag, self.x == 0);
        self.setflag(Flags::Negative, (self.x & 0x80) != 0);
    }

    pub fn txs(&mut self) {
        self.sp = self.x;
    }

    pub fn pha(&mut self) {
        self.write((0x0100 as u16).wrapping_add(self.sp as u16), self.a);
        self.sp = self.sp.wrapping_sub(1);
    }

    pub fn php(&mut self) {
        self.write((0x0100 as u16).wrapping_add(self.sp as u16), self.flags);
        self.sp = self.sp.wrapping_sub(1);
    }

    pub fn pla(&mut self) {
        self.sp = self.sp.wrapping_add(1);
        self.a = self.read((0x0100 as u16).wrapping_add(self.sp as u16));
        self.setflag(Flags::Zeroflag, self.a == 0);
        self.setflag(Flags::Negative, (self.a & 0x80) != 0);
    }

    pub fn plp(&mut self) {
        self.sp = self.sp.wrapping_add(1);
        self.flags = self.read((0x0100 as u16).wrapping_add(self.sp as u16));
    }

    pub fn and(&mut self) {
        self.a = self.a & self.immval;
        self.setflag(Flags::Zeroflag, self.a == 0);
        self.setflag(Flags::Negative, (self.a & 0x80) != 0);
    }

    pub fn eor(&mut self) {
        self.a = self.a ^ self.immval;
        self.setflag(Flags::Zeroflag, self.a == 0);
        self.setflag(Flags::Negative, (self.a & 0x80) != 0);
    }

    pub fn ora(&mut self) {
        self.a = self.a | self.immval;
        self.setflag(Flags::Zeroflag, self.a == 0);
        self.setflag(Flags::Negative, (self.a & 0x80) != 0);
    }

    pub fn bit(&mut self) {
        let temp = self.a & self.immval;
        self.setflag(Flags::Negative, (temp & 0x80) != 0);
        self.setflag(Flags::Overflow, (temp & 0x40) != 0);
        self.setflag(Flags::Zeroflag, temp == 0);
    }

    pub fn adc(&mut self) {
        let temp: u16 = (self.a as u16)
            .wrapping_add(self.immval as u16)
            .wrapping_add(if self.flags & 0x1 != 0 { 1 } else { 0 });

        self.setflag(Flags::Carry, temp > 255);
        self.setflag(Flags::Zeroflag, temp == 0);
        self.setflag(
            Flags::Overflow,
            !((self.a as u16) ^ (self.immval as u16)) & ((self.a as u16) ^ (temp as u16)) & 0x0080
                != 0,
        );
        self.setflag(Flags::Negative, temp & 0x80 != 0);
        self.a = (temp & 0xFF) as u8;
    }

    pub fn sbc(&mut self) {
        let value: u16 = !self.immval as u16;
        let carry: u16 = if self.flags & 0x1 != 0 { 1 } else { 0 };
        let temp = (self.a as u16)
            .wrapping_sub(self.immval as u16)
            .wrapping_sub(1 - carry);
        self.setflag(Flags::Carry, temp > 255);
        self.setflag(Flags::Zeroflag, temp == 0);
        self.setflag(
            Flags::Overflow,
            ((temp ^ (self.a as u16)) & (temp ^ value) & 0x0080) != 0,
        );
        self.setflag(Flags::Negative, (temp & 0x0080) != 0);
        self.a = (temp & 0x00FF) as u8;
    }

    pub fn cmp(&mut self) {
        let temp = self.a.wrapping_sub(self.immval);
        self.setflag(Flags::Zeroflag, temp == 0);
        self.setflag(Flags::Carry, self.a >= self.immval);
        self.setflag(Flags::Negative, (temp & 0x80) != 0);
    }

    pub fn cpx(&mut self) {
        let temp = self.x.wrapping_sub(self.immval);
        self.setflag(Flags::Zeroflag, temp == 0);
        self.setflag(Flags::Carry, self.x >= self.immval);
        self.setflag(Flags::Negative, (temp & 0x80) != 0);
    }

    pub fn cpy(&mut self) {
        let temp = self.y.wrapping_sub(self.immval);
        self.setflag(Flags::Zeroflag, temp == 0);
        self.setflag(Flags::Carry, self.y >= self.immval);
        self.setflag(Flags::Negative, (temp & 0x80) != 0);
    }

    pub fn inc(&mut self) {
        let mut byte = self.immval;
        byte = byte.wrapping_add(1);
        self.write(self.abs_addr, byte);
        self.setflag(Flags::Zeroflag, byte == 0);
        self.setflag(Flags::Negative, (byte & 0x80) != 0);
    }

    pub fn inx(&mut self) {
        self.x = self.x.wrapping_add(1);
        self.setflag(Flags::Zeroflag, self.x == 0);
        self.setflag(Flags::Negative, (self.x & 0x80) != 0);
    }

    pub fn iny(&mut self) {
        self.y = self.x.wrapping_add(1);
        self.setflag(Flags::Zeroflag, self.y == 0);
        self.setflag(Flags::Negative, (self.y & 0x80) != 0);
    }

    pub fn dec(&mut self) {
        let mut byte = self.immval;
        byte = byte.wrapping_sub(1);
        self.write(self.abs_addr, byte);
        self.setflag(Flags::Zeroflag, byte == 0);
        self.setflag(Flags::Negative, (byte & 0x80) != 0);
    }

    pub fn dex(&mut self) {
        self.x = self.x.wrapping_sub(1);
        self.setflag(Flags::Zeroflag, self.x == 0);
        self.setflag(Flags::Negative, (self.x & 0x80) != 0);
    }

    pub fn dey(&mut self) {
        self.y = self.y.wrapping_sub(1);
        self.setflag(Flags::Zeroflag, self.y == 0);
        self.setflag(Flags::Negative, (self.y & 0x80) != 0);
    }

    pub fn asl(&mut self) {
        if self.current_opcode == 0x0A {
            self.setflag(Flags::Carry, (self.a & 0x80) != 0);
            self.a = self.a << 1;
            self.setflag(Flags::Zeroflag, self.a == 0);
            self.setflag(Flags::Negative, (self.a & 0x80) != 0);
        } else {
            self.setflag(Flags::Carry, (self.immval & 0x80) != 0);
            self.immval = self.immval << 1;
            self.setflag(Flags::Zeroflag, self.immval == 0);
            self.setflag(Flags::Negative, (self.immval & 0x80) != 0);
            self.write(self.abs_addr, self.immval);
        }
    }

    pub fn lsr(&mut self) {
        if self.current_opcode == 0x4A {
            self.setflag(Flags::Carry, (self.a & 0x01) != 0);
            self.a = self.a >> 1;
            self.setflag(Flags::Zeroflag, self.a == 0);
            self.setflag(Flags::Negative, (self.a & 0x80) != 0);
        } else {
            self.setflag(Flags::Carry, (self.immval & 0x01) != 0);
            self.immval = self.immval >> 1;
            self.setflag(Flags::Zeroflag, self.immval == 0);
            self.setflag(Flags::Negative, (self.immval & 0x80) != 0);
            self.write(self.abs_addr, self.immval);
        }
    }

    pub fn rol(&mut self) {
        if self.current_opcode == 0x2A {
            let carry = if (self.flags & 0x1) != 0 { 1 } else { 0 };
            self.setflag(Flags::Carry, (self.a & 0x80) != 0);
            self.a = self.a << 1;
            self.a = self.a | carry;
            self.setflag(Flags::Zeroflag, self.a == 0);
            self.setflag(Flags::Negative, (self.a & 0x80) != 0);
        } else {
            let carry = if (self.flags & 0x1) != 0 { 1 } else { 0 };
            self.setflag(Flags::Carry, (self.immval & 0x80) != 0);
            self.immval = self.immval << 1;
            self.immval = self.immval | carry;
            self.write(self.abs_addr, self.immval);
            self.setflag(Flags::Zeroflag, self.immval == 0);
            self.setflag(Flags::Negative, (self.immval & 0x80) != 0);
        }
    }

    pub fn ror(&mut self) {
        if self.current_opcode == 0x6A {
            let carry = if (self.flags & 0x1) != 0 { 0x80 } else { 0x00 };
            self.setflag(Flags::Carry, (self.a & 0x01) != 0);
            self.a = self.a >> 1;
            self.a = self.a | carry;
            self.setflag(Flags::Zeroflag, self.a == 0);
            self.setflag(Flags::Negative, (self.a & 0x80) != 0);
        } else {
            let carry = if (self.flags & 0x1) != 0 { 0x80 } else { 0x00 };
            self.setflag(Flags::Carry, (self.immval & 0x80) != 0);
            self.immval = self.immval >> 1;
            self.immval = self.immval | carry;
            self.write(self.abs_addr, self.immval);
            self.setflag(Flags::Zeroflag, self.immval == 0);
            self.setflag(Flags::Negative, (self.immval & 0x80) != 0);
        }
    }

    pub fn jmp(&mut self) {
        self.pc = self.abs_addr;
        println!("JMP {}",self.pc);
    }

    pub fn jsr(&mut self) {
        self.pc = self.pc.wrapping_sub(1);
        self.write(0x0100 + (self.sp as u16), ((self.pc >> 8) & 0xFF) as u8);
        self.sp = self.sp.wrapping_sub(1);
        self.write(0x0100 + (self.sp as u16), (self.pc & 0xFF) as u8);
        self.pc = self.abs_addr;
    }

    pub fn rts(&mut self) {
        self.sp = self.sp.wrapping_add(1);
        self.pc = self.read(0x0100 + (self.sp as u16)) as u16;
        self.sp = self.sp.wrapping_add(1);
        self.pc = self.pc | self.read(0x0100 + (self.sp as u16)) as u16;
        self.pc = self.pc.wrapping_add(1);
    }

    pub fn bcc(&mut self) {
        if !self.getflag(Flags::Carry) {
            self.extra_cycles = self.extra_cycles.wrapping_add(1);
            self.abs_addr = self.pc.wrapping_add(self.relval);
            if (self.abs_addr & 0xFF00) != (self.pc & 0xFF00) {
                self.extra_cycles += self.extra_cycles.wrapping_add(2);
            }
        }
    }

    pub fn bcs(&mut self) {
        if self.getflag(Flags::Carry) {
            self.extra_cycles = self.extra_cycles.wrapping_add(1);
            self.abs_addr = self.pc.wrapping_add(self.relval);
            if (self.abs_addr & 0xFF00) != (self.pc & 0xFF00) {
                self.extra_cycles += self.extra_cycles.wrapping_add(2);
            }
        }
    }

    pub fn beq(&mut self) {
        if self.getflag(Flags::Zeroflag) {
            self.extra_cycles = self.extra_cycles.wrapping_add(1);
            self.abs_addr = self.pc.wrapping_add(self.relval);
            if (self.abs_addr & 0xFF00) != (self.pc & 0xFF00) {
                self.extra_cycles += self.extra_cycles.wrapping_add(2);
            }
        }
    }

    pub fn bmi(&mut self) {
        if self.getflag(Flags::Negative) {
            self.extra_cycles = self.extra_cycles.wrapping_add(1);
            self.abs_addr = self.pc.wrapping_add(self.relval);
            if (self.abs_addr & 0xFF00) != (self.pc & 0xFF00) {
                self.extra_cycles += self.extra_cycles.wrapping_add(2);
            }
        }
    }
    pub fn bne(&mut self) {
        if !self.getflag(Flags::Zeroflag) {
            self.extra_cycles = self.extra_cycles.wrapping_add(1);
            self.abs_addr = self.pc.wrapping_add(self.relval);
            if (self.abs_addr & 0xFF00) != (self.pc & 0xFF00) {
                self.extra_cycles += self.extra_cycles.wrapping_add(2);
            }
        }
    }

    pub fn bpl(&mut self) {
        if !self.getflag(Flags::Negative) {
            self.extra_cycles = self.extra_cycles.wrapping_add(1);
            self.abs_addr = self.pc.wrapping_add(self.relval);
            if (self.abs_addr & 0xFF00) != (self.pc & 0xFF00) {
                self.extra_cycles += self.extra_cycles.wrapping_add(2);
            }
        }
    }

    pub fn bvc(&mut self) {
        if !self.getflag(Flags::Overflow) {
            self.extra_cycles = self.extra_cycles.wrapping_add(1);
            self.abs_addr = self.pc.wrapping_add(self.relval);
            if (self.abs_addr & 0xFF00) != (self.pc & 0xFF00) {
                self.extra_cycles += self.extra_cycles.wrapping_add(2);
            }
        }
    }

    pub fn bvs(&mut self) {
        if self.getflag(Flags::Overflow) {
            self.extra_cycles = self.extra_cycles.wrapping_add(1);
            self.abs_addr = self.pc.wrapping_add(self.relval);
            if (self.abs_addr & 0xFF00) != (self.pc & 0xFF00) {
                self.extra_cycles += self.extra_cycles.wrapping_add(2);
            }
        }
    }

    pub fn clc(&mut self) {
        self.setflag(Flags::Carry, false);
    }

    pub fn cld(&mut self) {
        self.setflag(Flags::Decimal, false);
    }

    pub fn cli(&mut self) {
        self.setflag(Flags::Interrupt, false);
    }

    pub fn clv(&mut self) {
        self.setflag(Flags::Overflow, false);
    }

    pub fn sec(&mut self) {
        self.setflag(Flags::Carry, true);
    }

    pub fn sed(&mut self) {
        self.setflag(Flags::Decimal, true);
    }

    pub fn sei(&mut self) {
        self.setflag(Flags::Interrupt, true);
    }

    pub fn brk(&mut self){
        self.pc = self.pc.wrapping_add(1);
        self.setflag(Flags::Interrupt, true);
        self.write(0x0100 + self.sp as u16, ((self.pc >> 8) & 0xff) as u8);
        self.sp = self.sp.wrapping_sub(1);
        self.write(0x0100 + self.sp as u16, (self.pc & 0xff) as u8);
        self.sp = self.sp.wrapping_sub(1);
        self.setflag(Flags::Break, true);
        self.write(0x0100 + self.sp as u16, self.flags);
        self.sp = self.sp.wrapping_sub(1);
        self.setflag(Flags::Break,false);
        self.pc = (self.read(0xFFFE) as u16).wrapping_mul(256) | (self.read(0xFFFF) as u16);
    }

    pub fn nop(&mut self){
        let _x = 1;
    }

    pub fn rti(&mut self){
        self.sp = self.sp.wrapping_add(1);
        self.flags = self.read(0x100 + self.sp as u16);
        self.sp = self.sp.wrapping_add(1);
        self.pc = self.read(0x100 + self.sp as u16) as u16;
        self.sp = self.sp.wrapping_add(1);
        self.pc = self.pc | (self.read(0x100 + self.sp as u16) as u16).wrapping_mul(256) as u16;
        self.setflag(Flags::Break, false);

    }

    pub fn sre(&mut self){
        self.setflag(Flags::Carry,(self.immval & 1) != 0);
        let temp = self.immval >> 1;
        self.a = temp ^ self.a;
        self.setflag(Flags::Negative,(self.a & 0x80) != 0);
        self.setflag(Flags::Zeroflag,self.a == 0);
    }

}

#[cfg(test)]
mod tests {
    use crate::bus::cpubus::Cpubus;
    use crate::Cpu;

    #[test]
    /// Tests loading an immediate value (0x45) into the accumulator.
    fn cpu_test1() {
        let mut cpu = Cpu::new();
        let mut bus = Cpubus::new(&mut cpu);
        cpu.linkbus(&mut bus);
        cpu.set_pc(0x8000);
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
        cpu.set_pc(0x8000);
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
        cpu.set_pc(0x8000);
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
        cpu.set_pc(0x8000);
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
        cpu.set_pc(0x8000);
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
        cpu.set_pc(0x8000);
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
        cpu.set_pc(0x8000);
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
        cpu.set_pc(0x8000);
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
        cpu.set_pc(0x8000);
        bus.cpu_write(0x8000, 0x8E);
        bus.cpu_write(0x8001, 0xAD);
        bus.cpu_write(0x8002, 0xDE);
        cpu.set_x(0x45);
        bus.clock();
        assert_eq!(bus.cpu_read(0xDEAD, false), 69, "STX test - FAILED!")
    }
    #[test]
    pub fn cpu_store_y() {
        let mut cpu = Cpu::new();
        let mut bus = Cpubus::new(&mut cpu);
        cpu.linkbus(&mut bus);
        cpu.set_pc(0x8000);
        bus.cpu_write(0x8000, 0x8C);
        bus.cpu_write(0x8001, 0xAD);
        bus.cpu_write(0x8002, 0xDE);
        cpu.set_y(0x45);
        bus.clock();
        assert_eq!(bus.cpu_read(0xDEAD, false), 69, "STX test - FAILED!")
    }

    #[test]
    pub fn cpu_tax() {
        let mut cpu = Cpu::new();
        let mut bus = Cpubus::new(&mut cpu);
        cpu.linkbus(&mut bus);
        cpu.set_pc(0x8000);
        bus.cpu_write(0x8000, 0xAA);
        cpu.set_a(0x45);
        bus.clock();
        assert_eq!(cpu.get_x(), 0x45, "TAX INSTRUCTION - FAILED!");
    }

    #[test]
    pub fn cpu_tay() {
        let mut cpu = Cpu::new();
        let mut bus = Cpubus::new(&mut cpu);
        cpu.linkbus(&mut bus);
        cpu.set_pc(0x8000);
        bus.cpu_write(0x8000, 0xA8);
        cpu.set_a(0x45);
        bus.clock();
        assert_eq!(cpu.get_y(), 0x45, "TAY INSTRUCTION - FAILED!");
    }

    #[test]
    pub fn cpu_txa() {
        let mut cpu = Cpu::new();
        let mut bus = Cpubus::new(&mut cpu);
        cpu.linkbus(&mut bus);
        cpu.set_pc(0x8000);
        bus.cpu_write(0x8000, 0x8A);
        cpu.set_x(0x45);
        bus.clock();
        assert_eq!(cpu.get_a(), 0x45, "TAY INSTRUCTION - FAILED!");
    }
    #[test]
    pub fn cpu_tya() {
        let mut cpu = Cpu::new();
        let mut bus = Cpubus::new(&mut cpu);
        cpu.linkbus(&mut bus);
        cpu.set_pc(0x8000);
        bus.cpu_write(0x8000, 0x98);
        cpu.set_y(0x45);
        bus.clock();
        assert_eq!(cpu.get_a(), 0x45, "TAY INSTRUCTION - FAILED!");
    }

    #[test]
    pub fn cpu_stackops() {
        let mut cpu = Cpu::new();
        let mut bus = Cpubus::new(&mut cpu);
        cpu.linkbus(&mut bus);
        cpu.set_pc(0x8000);
        cpu.set_a(0x45);
        bus.cpu_write(0x8000, 0x48); //PHA - 3
        cpu.set_sflag(0x80);
        bus.cpu_write(0x8001, 0x8); //PHP - 3
        bus.cpu_write(0x8002, 0xA9); //LDA - 2
        bus.cpu_write(0x8003, 0);
        bus.cpu_write(0x8002, 0x28); //PLP - 4
        bus.cpu_write(0x8002, 0x68); //PLA - 4
        cpu.set_y(0x45);
        for _ in 0..16 {
            bus.clock();
        }
        assert_eq!(cpu.get_a(), 0x45, "PLA instruction - FAILED!");
        assert_eq!(cpu.get_sflag(), 0x80, "PLP instruction - FAILED!");
    }

    #[test]
    pub fn cpu_andtest() {
        let mut cpu = Cpu::new();
        let mut bus = Cpubus::new(&mut cpu);
        cpu.linkbus(&mut bus);
        cpu.set_pc(0x8000);
        bus.cpu_write(0x8000, 0x29);
        bus.cpu_write(0x8001, 0x03);
        cpu.set_a(75);
        bus.clock();
        assert_eq!(cpu.get_a(), 75 & 3, "AND test - FAILED!");
    }

    #[test]
    pub fn cpu_eortest() {
        let mut cpu = Cpu::new();
        let mut bus = Cpubus::new(&mut cpu);
        cpu.linkbus(&mut bus);
        cpu.set_pc(0x8000);
        bus.cpu_write(0x8000, 0x49);
        bus.cpu_write(0x8001, 0x46);
        cpu.set_a(0x46);
        bus.clock();
        assert_eq!(cpu.get_a(), 0x46 ^ 0x46, "EOR test - FAILED!");
    }

    #[test]
    pub fn cpu_oratest() {
        let mut cpu = Cpu::new();
        let mut bus = Cpubus::new(&mut cpu);
        cpu.linkbus(&mut bus);
        cpu.set_pc(0x8000);
        bus.cpu_write(0x8000, 0x9);
        bus.cpu_write(0x8001, 0x7F);
        cpu.set_a(0x80);
        bus.clock();
        assert_eq!(cpu.get_a(), 0xFF, "ORA test - FAILED!");
    }

    #[test]
    pub fn cpu_bittest() {
        let mut cpu = Cpu::new();
        let mut bus = Cpubus::new(&mut cpu);
        cpu.linkbus(&mut bus);
        cpu.set_pc(0x8000);
        bus.cpu_write(0x8000, 0x24);
        bus.cpu_write(0x8001, 0x7F);
        bus.cpu_write(0x7F as u16, 0);
        cpu.set_a(0x80);
        bus.clock();
        assert_eq!(cpu.get_sflag(), 0x02, "ORA test - FAILED!");
    }

    #[test]

    pub fn cpu_adctest() {
        let mut cpu = Cpu::new();
        let mut bus = Cpubus::new(&mut cpu);
        cpu.linkbus(&mut bus);
        cpu.set_pc(0x8000);
        bus.cpu_write(0x8000, 0x69);
        bus.cpu_write(0x8001, 1);
        cpu.set_a(126);
        bus.clock();
        assert_eq!(cpu.get_a(), 127, "ADC - addition FAILED");
    }

    #[test]

    pub fn cpu_adctest2() {
        let mut cpu = Cpu::new();
        let mut bus = Cpubus::new(&mut cpu);
        cpu.linkbus(&mut bus);
        cpu.set_pc(0x8000);
        bus.cpu_write(0x8000, 0x69);
        bus.cpu_write(0x8001, 1);
        cpu.set_a(127);
        bus.clock();
        assert!(cpu.get_sflag() & 0x40 != 0, "ADC - overflow FAILED");
    }

    #[test]

    pub fn cpu_sbctest() {
        let mut cpu = Cpu::new();
        let mut bus = Cpubus::new(&mut cpu);
        cpu.linkbus(&mut bus);
        cpu.set_pc(0x8000);
        bus.cpu_write(0x8000, 0xE9);
        bus.cpu_write(0x8001, 1);
        cpu.setflag(crate::cpu::processor::Flags::Carry, true);
        cpu.set_a(126);
        bus.clock();
        assert_eq!(cpu.get_a(), 125, "SBC - Subtraction FAILED");
    }

    #[test]

    pub fn cpu_sbctest2() {
        let mut cpu = Cpu::new();
        let mut bus = Cpubus::new(&mut cpu);
        cpu.linkbus(&mut bus);
        cpu.set_pc(0x8000);
        bus.cpu_write(0x8000, 0xE9);
        bus.cpu_write(0x8001, 1);
        cpu.set_a(128);
        bus.clock();
        assert!(cpu.get_sflag() & 0x40 != 0, "SBC - overflow FAILED");
    }

    #[test]

    pub fn cpu_cmp_zero_test() {
        let mut cpu = Cpu::new();
        let mut bus = Cpubus::new(&mut cpu);
        cpu.linkbus(&mut bus);
        cpu.set_pc(0x8000);
        bus.cpu_write(0x8000, 0xC9);
        bus.cpu_write(0x8001, 128);
        cpu.set_a(128);
        bus.clock();
        assert!(cpu.get_sflag() & 0x02 != 0, "CMP - ZP FAILED");
    }

    #[test]

    pub fn cpu_cmp_carry_test() {
        let mut cpu = Cpu::new();
        let mut bus = Cpubus::new(&mut cpu);
        cpu.linkbus(&mut bus);
        cpu.set_pc(0x8000);
        bus.cpu_write(0x8000, 0xC9);
        bus.cpu_write(0x8001, 25);
        cpu.set_a(128);
        bus.clock();
        assert!(cpu.get_sflag() & 0x01 != 0, "CMP - CARRY FAILED");
    }

    #[test]

    pub fn cpu_cmp_negative_test() {
        let mut cpu = Cpu::new();
        let mut bus = Cpubus::new(&mut cpu);
        cpu.linkbus(&mut bus);
        cpu.set_pc(0x8000);
        bus.cpu_write(0x8000, 0xC9);
        bus.cpu_write(0x8001, 200);
        cpu.set_a(128);
        bus.clock();
        assert!(cpu.get_sflag() & 0x80 != 0, "CMP - NEGATIVE FAILED");
    }

    #[test]
    pub fn cpu_test_inc() {
        let mut cpu = Cpu::new();
        let mut bus = Cpubus::new(&mut cpu);
        cpu.linkbus(&mut bus);
        cpu.set_pc(0x8000);
        bus.cpu_write(0x8000, 0xE6);
        bus.cpu_write(0x8001, 0x45);
        bus.cpu_write(0x45, 100);
        bus.clock();
        assert_eq!(cpu.read(0x45), 101, "INC - TEST FAILED!");
    }

    #[test]
    pub fn cpu_test_dec() {
        let mut cpu = Cpu::new();
        let mut bus = Cpubus::new(&mut cpu);
        cpu.linkbus(&mut bus);
        cpu.set_pc(0x8000);
        bus.cpu_write(0x8000, 0xC6);
        bus.cpu_write(0x8001, 0x45);
        bus.cpu_write(0x45, 100);
        bus.clock();
        assert_eq!(cpu.read(0x45), 99, "INC - TEST FAILED!");
    }

    #[test]
    pub fn cpu_test_asl() {
        let mut cpu = Cpu::new();
        let mut bus = Cpubus::new(&mut cpu);
        cpu.linkbus(&mut bus);
        cpu.set_pc(0x8000);
        bus.cpu_write(0x8000, 0x0A);
        cpu.set_a(1);
        bus.clock();
        assert_eq!(cpu.get_a(), 0x2, "ASL - TEST 1 FAILED!");
    }

    #[test]
    pub fn cpu_test_asl2() {
        let mut cpu = Cpu::new();
        let mut bus = Cpubus::new(&mut cpu);
        cpu.linkbus(&mut bus);
        cpu.set_pc(0x8000);
        bus.cpu_write(0x8000, 0x06);
        bus.cpu_write(0x8001, 0x45);
        bus.cpu_write(0x45, 25);
        bus.clock();
        assert_eq!(cpu.read(0x45), 50, "ASL - TEST 2 FAILED!");
    }

    #[test]
    pub fn cpu_test_lsr() {
        let mut cpu = Cpu::new();
        let mut bus = Cpubus::new(&mut cpu);
        cpu.linkbus(&mut bus);
        cpu.set_pc(0x8000);
        bus.cpu_write(0x8000, 0x46);
        bus.cpu_write(0x8001, 0x45);
        bus.cpu_write(0x45, 50);
        bus.clock();
        assert_eq!(cpu.read(0x45), 25, "LSR - TEST FAILED!");
    }

    #[test]
    pub fn cpu_test_rol() {
        let mut cpu = Cpu::new();
        let mut bus = Cpubus::new(&mut cpu);
        cpu.linkbus(&mut bus);
        cpu.set_pc(0x8000);
        bus.cpu_write(0x8000, 0x26);
        bus.cpu_write(0x8001, 0x45);
        bus.cpu_write(0x45, 0b01111111);
        cpu.setflag(crate::cpu::processor::Flags::Carry, true);
        bus.clock();
        assert_eq!(cpu.read(0x45), 0xFF, "ROL - TEST FAILED!");
    }

    #[test]
    pub fn cpu_test_ror() {
        let mut cpu = Cpu::new();
        let mut bus = Cpubus::new(&mut cpu);
        cpu.linkbus(&mut bus);
        cpu.set_pc(0x8000);
        bus.cpu_write(0x8000, 0x66);
        bus.cpu_write(0x8001, 0x45);
        bus.cpu_write(0x45, 254);
        bus.clock();
        assert_eq!(cpu.read(0x45), 127, "ROR - TEST FAILED!");
    }

    #[test]
    pub fn cpu_test_jmp() {
        let mut cpu = Cpu::new();
        let mut bus = Cpubus::new(&mut cpu);
        cpu.linkbus(&mut bus);
        cpu.set_pc(0x8000);
        bus.cpu_write(0x8000, 0x4c);
        bus.cpu_write(0x8001, 0x45);
        bus.cpu_write(0x8002, 0x01);
        bus.clock();
        assert_eq!(cpu.get_pc(), 0x145, "JMP - TEST FAILED!");
    }
}
