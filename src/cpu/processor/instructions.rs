use crate::cpu::processor::Cpu;

use super::Flags;

impl Cpu {
    /// Sets or clears a specific condition flag.
    ///
    /// # Arguments
    /// * `flag` - The flag to be modified.
    /// * `state` - `true` to set the flag, `false` to clear it.
    pub fn setflag(&mut self, flag: Flags, state: bool) {
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
        println!("JMP {}", self.pc);
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

    pub fn brk(&mut self) {
        self.pc = self.pc.wrapping_add(1);
        self.setflag(Flags::Interrupt, true);
        self.write(0x0100 + self.sp as u16, ((self.pc >> 8) & 0xff) as u8);
        self.sp = self.sp.wrapping_sub(1);
        self.write(0x0100 + self.sp as u16, (self.pc & 0xff) as u8);
        self.sp = self.sp.wrapping_sub(1);
        self.setflag(Flags::Break, true);
        self.write(0x0100 + self.sp as u16, self.flags);
        self.sp = self.sp.wrapping_sub(1);
        self.setflag(Flags::Break, false);
        self.pc = (self.read(0xFFFE) as u16).wrapping_mul(256) | (self.read(0xFFFF) as u16);
    }

    pub fn nop(&mut self) {
        let _x = 1;
    }

    pub fn rti(&mut self) {
        self.sp = self.sp.wrapping_add(1);
        self.flags = self.read(0x100 + self.sp as u16);
        self.sp = self.sp.wrapping_add(1);
        self.pc = self.read(0x100 + self.sp as u16) as u16;
        self.sp = self.sp.wrapping_add(1);
        self.pc = self.pc | (self.read(0x100 + self.sp as u16) as u16).wrapping_mul(256) as u16;
        self.setflag(Flags::Break, false);
    }

    pub fn sre(&mut self) {
        self.setflag(Flags::Carry, (self.immval & 1) != 0);
        let temp = self.immval >> 1;
        self.a = temp ^ self.a;
        self.setflag(Flags::Negative, (self.a & 0x80) != 0);
        self.setflag(Flags::Zeroflag, self.a == 0);
    }
}
