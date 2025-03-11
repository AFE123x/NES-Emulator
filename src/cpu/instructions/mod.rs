use super::{Cpu, Flags};

pub mod cpu_handlers;
pub mod inst_enum;

impl Cpu {
    pub fn lda(&mut self) {
        self.a = self.immval;
        self.flags.set(Flags::Zero, self.a == 0);
        self.flags.set(Flags::Negative, self.a & 0x80 != 0);
    }

    pub fn ldx(&mut self) {
        self.x = self.immval;
        self.flags.set(Flags::Zero, self.x == 0);
        self.flags.set(Flags::Negative, self.x & 0x80 != 0);
    }

    pub fn ldy(&mut self) {
        self.y = self.immval;
        self.flags.set(Flags::Zero, self.y == 0);
        self.flags.set(Flags::Negative, self.y & 0x80 != 0);
    }

    pub fn sta(&mut self) {
        self.cpu_write(self.addrabs, self.a);
    }

    pub fn stx(&mut self) {
        self.cpu_write(self.addrabs, self.x);
    }

    pub fn sty(&mut self) {
        self.cpu_write(self.addrabs, self.y);
    }
    pub fn tax(&mut self) {
        self.x = self.a;
        self.flags.set(Flags::Zero, self.x == 0);
        self.flags.set(Flags::Negative, self.x & 0x80 != 0);
    }

    pub fn tay(&mut self) {
        self.y = self.a;
        self.flags.set(Flags::Zero, self.y == 0);
        self.flags.set(Flags::Negative, self.y & 0x80 != 0);
    }

    pub fn txa(&mut self) {
        self.a = self.x;
        self.flags.set(Flags::Zero, self.a == 0);
        self.flags.set(Flags::Negative, self.a & 0x80 != 0);
    }
    pub fn tya(&mut self) {
        self.a = self.y;
        self.flags.set(Flags::Zero, self.a == 0);
        self.flags.set(Flags::Negative, self.a & 0x80 != 0);
    }

    pub fn tsx(&mut self) {
        self.x = self.sp;
        self.flags.set(Flags::Zero, self.x == 0);
        self.flags.set(Flags::Negative, self.x & 0x80 != 0);
    }

    pub fn txs(&mut self) {
        self.sp = self.x;
    }

    pub fn pha(&mut self) {
        self.cpu_write((0x100 as u16).wrapping_add(self.sp as u16), self.a);
        self.sp = self.sp.wrapping_sub(1);
    }

    pub fn php(&mut self) {
        self.cpu_write(
            (0x100 as u16).wrapping_add(self.sp as u16),
            self.flags.bits(),
        );
        self.sp = self.sp.wrapping_sub(1);
    }

    pub fn pla(&mut self) {
        self.sp = self.sp.wrapping_add(1);
        self.a = self.cpu_read((0x100 as u16).wrapping_add(self.sp as u16));
        self.flags.set(Flags::Zero, self.a == 0);
        self.flags.set(Flags::Negative, self.a & 0x80 != 0);
    }

    pub fn plp(&mut self) {
        self.sp = self.sp.wrapping_add(1);
        let temp = self.cpu_read((0x100 as u16).wrapping_add(self.sp as u16));
        self.flags = Flags::from_bits_truncate(temp);
    }

    pub fn and(&mut self) {
        self.a = self.a & self.immval;
        self.flags.set(Flags::Zero, self.a == 0);
        self.flags.set(Flags::Negative, self.a & 0x80 != 0);
    }

    pub fn eor(&mut self) {
        self.a = self.a ^ self.immval;
        self.flags.set(Flags::Zero, self.a == 0);
        self.flags.set(Flags::Negative, self.a & 0x80 != 0);
    }

    pub fn ora(&mut self) {
        self.a = self.a | self.immval;
        self.flags.set(Flags::Zero, self.a == 0);
        self.flags.set(Flags::Negative, self.a & 0x80 != 0);
    }

    pub fn bit(&mut self) {
        let temp = self.a & self.immval;
        self.flags.set(Flags::Negative, self.immval & 0x80 != 0);
        self.flags.set(Flags::Overflow, self.immval & 0x40 != 0);
        self.flags.set(Flags::Zero, temp == 0);
    }

    pub fn adc(&mut self) {
        let temp: u16 = (self.a as u16)
            + (self.immval as u16)
            + (if self.flags.contains(Flags::Carry) {
                1
            } else {
                0
            });
        self.flags.set(Flags::Carry, temp > 255);
        let a = self.a;
        self.a = (temp & 0xFF) as u8;
        let l_test = self.a & 0x80;
        let r_test = self.immval & 0x80;
        let prop_one = l_test ^ r_test;
        let prop_one = prop_one != 0;

        let l_test = self.a & 0x80;
        let r_test = a & 0x80;
        let prop_two = l_test ^ r_test;
        let prop_two = prop_two != 0;

        self.flags.set(Flags::Overflow, prop_one && prop_two);
        self.flags.set(Flags::Zero, self.a == 0);
        self.flags.set(Flags::Negative, self.a & 0x80 != 0);
    }

    pub fn sbc(&mut self) {
        //retrieve values.
        let m: u8 = self.immval;
        let c: u8 = if self.flags.contains(Flags::Carry) {
            0
        } else {
            1
        };
        //change values to twos complement
        let c = (!c).wrapping_add(1);
        let m = (!m).wrapping_add(1);
        let a = self.a;
        //perform addition
        let temp_sum: u16 = m as u16 + c as u16 + self.a as u16;
        self.a = (temp_sum & 0xFF) as u8;
        //set zero flag
        self.flags.set(Flags::Zero, self.a == 0);
        self.flags.set(Flags::Carry, temp_sum > 255);
        self.flags.set(Flags::Zero, self.a & 0x80 != 0);

        //overflow logic
        let prop_one = (a & 0x80) ^ (self.a & 0x80);
        let prop_one = prop_one != 0;
        let prop_two = (m & 0x80) ^ (self.a & 0x80);
        let prop_two = prop_two != 0;
        self.flags.set(Flags::Overflow, prop_one && prop_two);
    }

    pub fn cmp(&mut self) {
        let temp = (self.a as u16).wrapping_sub(self.immval as u16);
        self.flags.set(Flags::Carry, temp > 255);
        let temp = (temp & 0xff) as u8;
        self.flags.set(Flags::Zero, temp == 0);
        self.flags.set(Flags::Negative, temp & 0x80 != 0);
    }

    pub fn cpx(&mut self) {
        let temp = (self.x as u16).wrapping_sub(self.immval as u16);
        self.flags.set(Flags::Carry, temp > 255);
        let temp = (temp & 0xff) as u8;
        self.flags.set(Flags::Zero, temp == 0);
        self.flags.set(Flags::Negative, temp & 0x80 != 0);
    }

    pub fn cpy(&mut self) {
        let temp = (self.y as u16).wrapping_sub(self.immval as u16);
        self.flags.set(Flags::Carry, temp > 255);
        let temp = (temp & 0xff) as u8;
        self.flags.set(Flags::Zero, temp == 0);
        self.flags.set(Flags::Negative, temp & 0x80 != 0);
    }

    pub fn inc(&mut self) {
        let temp = self.immval.wrapping_add(1);
        self.flags.set(Flags::Zero, temp == 0);
        self.flags.set(Flags::Negative, temp & 0x80 != 0);
        self.cpu_write(self.addrabs, temp);
    }

    pub fn inx(&mut self) {
        self.x = self.x.wrapping_add(1);
        self.flags.set(Flags::Zero, self.x == 0);
        self.flags.set(Flags::Negative, self.x & 0x80 != 0);
    }

    pub fn iny(&mut self) {
        self.y = self.y.wrapping_add(1);
        self.flags.set(Flags::Zero, self.y == 0);
        self.flags.set(Flags::Negative, self.y & 0x80 != 0);
    }

    pub fn dec(&mut self) {
        let temp = self.immval.wrapping_sub(1);
        self.flags.set(Flags::Zero, temp == 0);
        self.flags.set(Flags::Negative, temp & 0x80 != 0);
        self.cpu_write(self.addrabs, temp);
    }

    pub fn dex(&mut self) {
        self.x = self.x.wrapping_sub(1);
        self.flags.set(Flags::Zero, self.x == 0);
        self.flags.set(Flags::Negative, self.x & 0x80 != 0);
    }

    pub fn dey(&mut self) {
        self.y = self.y.wrapping_sub(1);
        self.flags.set(Flags::Zero, self.y == 0);
        self.flags.set(Flags::Negative, self.y & 0x80 != 0);
    }

    pub fn asl(&mut self) {
        if self.opcode == 0x0A {
            self.flags.set(Flags::Carry, self.a & 0x80 != 0);
            self.a = self.a << 1;
            self.flags.set(Flags::Zero, self.a == 0);
            self.flags.set(Flags::Zero, self.a & 0x80 != 0);
        } else {
            self.flags.set(Flags::Carry, self.immval & 0x80 != 0);
            self.immval = self.immval << 1;
            self.flags.set(Flags::Zero, self.immval == 0);
            self.flags.set(Flags::Zero, self.immval & 0x80 != 0);
            self.cpu_write(self.addrabs, self.immval);
        }
    }

    pub fn lsr(&mut self) {
        if self.opcode == 0x4A {
            self.flags.set(Flags::Carry, self.a & 0x1 != 0);
            self.a = self.a >> 1;
            self.flags.set(Flags::Zero, self.a == 0);
            self.flags.set(Flags::Negative, self.a & 0x80 != 0);
        } else {
            self.flags.set(Flags::Carry, self.immval & 0x1 != 0);
            self.immval = self.immval >> 1;
            self.flags.set(Flags::Zero, self.immval == 0);
            self.flags.set(Flags::Negative, self.immval & 0x80 != 0);
            self.cpu_write(self.addrabs, self.immval);
        }
    }

    pub fn rol(&mut self) {
        if self.opcode == 0x2A {
            let wrap_fac = if self.a & 0x80 != 0 { 1 } else { 0 };
            self.flags.set(Flags::Carry, wrap_fac != 0);
            self.a = self.a << 1;
            self.a = self.a | wrap_fac;
            self.flags.set(Flags::Zero, self.a == 0);
            self.flags.set(Flags::Negative, self.a & 0x80 != 0);
        } else {
            let wrap_fac = if self.immval & 0x80 != 0 { 1 } else { 0 };
            self.flags.set(Flags::Carry, wrap_fac != 0);
            self.immval = self.immval << 1;
            self.immval = self.immval | wrap_fac;
            self.flags.set(Flags::Zero, self.immval == 0);
            self.flags.set(Flags::Negative, self.immval & 0x80 != 0);
            self.cpu_write(self.addrabs, self.immval);
        }
    }

    pub fn ror(&mut self) {
        if self.opcode == 0x6A {
            let wrap_fac = if self.a & 0x01 != 0 { 0x80 } else { 0 };
            self.flags.set(Flags::Carry, wrap_fac != 0);
            self.a = self.a >> 1;
            self.a = self.a | wrap_fac;
            self.flags.set(Flags::Zero, self.a == 0);
            self.flags.set(Flags::Negative, self.a & 0x80 != 0);
        } else {
            let wrap_fac = if self.immval & 0x80 != 0 { 0x80 } else { 0 };
            self.flags.set(Flags::Carry, wrap_fac != 0);
            self.immval = self.immval >> 1;
            self.immval = self.immval | wrap_fac;
            self.flags.set(Flags::Zero, self.immval == 0);
            self.flags.set(Flags::Negative, self.immval & 0x80 != 0);
            self.cpu_write(self.addrabs, self.immval);
        }
    }

    pub fn jmp(&mut self) {
        self.pc = self.addrabs;
    }

    pub fn jsr(&mut self) {
        let temp = self.pc - 1;
        let lo = (temp & 0xFF) as u8;
        let hi = (temp >> 8) as u8;
        self.cpu_write(0x100 as u16 + self.sp as u16, hi);
        self.sp = self.sp.wrapping_sub(1);
        self.cpu_write(0x100 as u16 + self.sp as u16, lo);
        self.sp = self.sp.wrapping_sub(1);
        self.pc = self.addrabs;
    }

    pub fn rts(&mut self) {
        self.sp = self.sp.wrapping_add(1);
        let lo = self.cpu_read(0x100 as u16 + self.sp as u16);
        self.sp = self.sp.wrapping_add(1);
        let hi = self.cpu_read(0x100 as u16 + self.sp as u16);
        let temp = ((hi as u16) << 8) | (lo as u16);
        let temp = temp + 1;
        self.pc = temp;
    }

    pub fn bcc(&mut self) {
        if !self.flags.contains(Flags::Carry) {
            self.cycles_left = self.cycles_left.wrapping_add(1);
            let temp = self.pc.wrapping_add(self.relval);
            if self.pc & 0xFF00 != temp & 0xFF {
                self.cycles_left = self.cycles_left.wrapping_add(2);
            }
            self.pc = temp;
        }
    }

    pub fn bcs(&mut self) {
        if self.flags.contains(Flags::Carry) {
            self.cycles_left = self.cycles_left.wrapping_add(1);
            let temp = self.pc.wrapping_add(self.relval);
            if self.pc & 0xFF00 != temp & 0xFF {
                self.cycles_left = self.cycles_left.wrapping_add(2);
            }
            self.pc = temp;
        }
    }

    pub fn beq(&mut self) {
        if self.flags.contains(Flags::Zero) {
            self.cycles_left = self.cycles_left.wrapping_add(1);
            let temp = self.pc.wrapping_add(self.relval);
            if self.pc & 0xFF00 != temp & 0xFF {
                self.cycles_left = self.cycles_left.wrapping_add(2);
            }
            self.pc = temp;
        }
    }

    pub fn bmi(&mut self) {
        if self.flags.contains(Flags::Negative) {
            self.cycles_left = self.cycles_left.wrapping_add(1);
            let temp = self.pc.wrapping_add(self.relval);
            if self.pc & 0xFF00 != temp & 0xFF {
                self.cycles_left = self.cycles_left.wrapping_add(2);
            }
            self.pc = temp;
        }
    }

    pub fn bne(&mut self) {
        if !self.flags.contains(Flags::Zero) {
            self.cycles_left = self.cycles_left.wrapping_add(1);
            let temp = self.pc.wrapping_add(self.relval);
            if self.pc & 0xFF00 != temp & 0xFF {
                self.cycles_left = self.cycles_left.wrapping_add(2);
            }
            self.pc = temp;
        }
    }

    pub fn bpl(&mut self) {
        if !self.flags.contains(Flags::Negative) {
            self.cycles_left = self.cycles_left.wrapping_add(1);
            let temp = self.pc.wrapping_add(self.relval);
            if self.pc & 0xFF00 != temp & 0xFF {
                self.cycles_left = self.cycles_left.wrapping_add(2);
            }
            self.pc = temp;
        }
    }

    pub fn bvc(&mut self) {
        if !self.flags.contains(Flags::Overflow) {
            self.cycles_left = self.cycles_left.wrapping_add(1);
            let temp = self.pc.wrapping_add(self.relval);
            if self.pc & 0xFF00 != temp & 0xFF {
                self.cycles_left = self.cycles_left.wrapping_add(2);
            }
            self.pc = temp;
        }
    }

    pub fn bvs(&mut self) {
        if self.flags.contains(Flags::Overflow) {
            self.cycles_left = self.cycles_left.wrapping_add(1);
            let temp = self.pc.wrapping_add(self.relval);
            if self.pc & 0xFF00 != temp & 0xFF {
                self.cycles_left = self.cycles_left.wrapping_add(2);
            }
            self.pc = temp;
        }
    }

    pub fn brk(&mut self) {
        let temp = self.pc.wrapping_add(1);
        let lo = (temp & 0xFF) as u8;
        let hi = (temp >> 8) as u8;
        self.cpu_write((0x100 as u16).wrapping_add(self.sp as u16), hi);
        self.sp = self.sp.wrapping_sub(1);
        self.cpu_write((0x100 as u16).wrapping_add(self.sp as u16), lo);
        self.sp = self.sp.wrapping_sub(1);
        self.cpu_write(
            (0x100 as u16).wrapping_add(self.sp as u16),
            self.flags.bits(),
        );
        self.sp = self.sp.wrapping_sub(1);
        self.flags.set(Flags::Break, true);
        let lo = self.cpu_read(0xFFFE) as u16;
        let hi = self.cpu_read(0xFFFF) as u16;
        self.pc = (hi << 8) | lo;
    }
    pub fn rti(&mut self) {
        self.sp = self.sp.wrapping_add(1);
        let flag = self.cpu_read((0x100 as u16).wrapping_add(self.sp as u16));
        self.flags = Flags::from_bits_truncate(flag);
        self.sp = self.sp.wrapping_add(1);
        let lo = self.cpu_read((0x100 as u16).wrapping_add(self.sp as u16)) as u16;
        self.sp = self.sp.wrapping_add(1);
        let hi = self.cpu_read((0x100 as u16).wrapping_add(self.sp as u16)) as u16;
        self.pc = (hi << 8) | lo;
        self.flags.set(Flags::Break, false);
        println!("rti executed!");
    }


    pub fn nmi(&mut self){
        println!("nmi initialized");
        let hi = (self.pc >> 8) & 0xFF;
        let lo = self.pc & 0xFF;
        let addr = 0x100 + self.sp as u16;
        self.cpu_write(addr, hi as u8);
        self.sp = self.sp.wrapping_sub(1);
        let addr = 0x100 + self.sp as u16;
        self.cpu_write(addr, lo as u8);
        self.sp = self.sp.wrapping_sub(1);
        self.flags.set(Flags::IDisable, true);
        let addr = 0x100 + self.sp as u16;
        self.cpu_write(addr, self.flags.bits());
        self.sp = self.sp.wrapping_sub(1);
        let lo = self.cpu_read(0xFFFA) as u16;
        let hi = self.cpu_read(0xFFFB) as u16;
        self.pc = (hi << 8) | lo;
        self.cycles_left = 8;
    }
    pub fn reset(&mut self){
        let lo = self.cpu_read(0xFFFC) as u16;
        let hi = self.cpu_read(0xFFFD) as u16;
        self.pc = (hi << 8) | lo;
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.sp = 0xfd;
        self.flags = Flags::empty();
        self.addrabs = 0;
        self.relval = 0;
        self.immval = 0;
        self.cycles_left = 8;
        self.total_cycles = 0;
    }
}

#[cfg(test)]
mod cputest {
    use crate::cpu::Cpu;

    #[test]
    fn lda_test1() {
        let mut cpu = Cpu::new();
        cpu.set_immval(0x80);
        cpu.lda();
        //check if A register is correct
        assert_eq!(
            cpu.get_a(),
            0x80,
            "LDA_Test1 - FAILED! A value not correct!"
        );
        assert_eq!(
            cpu.get_flag(),
            0x80,
            "LDA Test1 - FAILED! Negative flag not enabled!"
        );
    }

    #[test]
    fn lda_test2() {
        let mut cpu = Cpu::new();
        cpu.immval = 0x0;
        cpu.lda();
        //check if A register is correct
        assert_eq!(cpu.get_a(), 0x0, "LDA_Test1 - FAILED! A value not correct!");
        assert_eq!(
            cpu.get_flag(),
            0x02,
            "LDA Test1 - FAILED! Zero flag not enabled!"
        );
    }
    #[test]
    fn ldx_test1() {
        let mut cpu = Cpu::new();
        cpu.immval = 0x80;
        cpu.ldx();
        //check if A register is correct
        assert_eq!(
            cpu.get_x(),
            0x80,
            "LDX_Test1 - FAILED! X value not correct!"
        );
        assert_eq!(
            cpu.get_flag(),
            0x80,
            "LDX Test1 - FAILED! Negative flag not enabled!"
        );
    }

    #[test]
    fn ldx_test2() {
        let mut cpu = Cpu::new();
        cpu.immval = 0x0;
        cpu.ldx();
        //check if A register is correct
        assert_eq!(cpu.get_x(), 0x0, "LDX_Test1 - FAILED! X value not correct!");
        assert_eq!(
            cpu.get_flag(),
            0x02,
            "LDX Test1 - FAILED! Zero flag not enabled!"
        );
    }

    #[test]
    fn ldy_test1() {
        let mut cpu = Cpu::new();
        cpu.immval = 0x80;
        cpu.ldy();
        //check if A register is correct
        assert_eq!(
            cpu.get_y(),
            0x80,
            "LDY_Test1 - FAILED! Y value not correct!"
        );
        assert_eq!(
            cpu.get_flag(),
            0x80,
            "LDY Test1 - FAILED! Negative flag not enabled!"
        );
    }

    #[test]
    fn ldy_test2() {
        let mut cpu = Cpu::new();
        cpu.immval = 0x0;
        cpu.ldy();
        //check if A register is correct
        assert_eq!(cpu.get_x(), 0x0, "LDY_Test1 - FAILED! Y value not correct!");
        assert_eq!(
            cpu.get_flag(),
            0x02,
            "LDY Test1 - FAILED! Zero flag not enabled!"
        );
    }
}
