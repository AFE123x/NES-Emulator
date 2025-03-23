use crate::cpu::Cpu;
use crate::cpu::Flags;
impl Cpu {
    ///# `ADC` - Add with Carry
    /// - A,Z,C,N = A+M+C
    /// - This instruction adds the contents of a memory location to the accumulator together with the carry bit. If overflow occurs the carry bit is set, this enables multiple byte addition to be performed.
    pub fn adc(&mut self) {
        let immval = self.cpu_read(self.addrabs, false) as u16;
        let carry_bit = if self.flags.contains(Flags::Carry) {
            1
        } else {
            0
        };
        let a = self.a as u16;
        let result = a + immval + carry_bit;
        self.flags.set(Flags::Carry, result > 255);
        self.flags.set(Flags::Negative, result & 0x80 != 0);
        let propa = (result & 0x80) ^ (immval & 0x80);
        let propa = propa != 0;
        let propb = (a & 0x80) ^ (result & 0x80);
        let propb = propb != 0;
        self.flags.set(Flags::Overflow, propa && propb);
        self.a = (result as u8) & 0xFF;
        self.flags.set(Flags::Zero, self.a == 0);
    }

    ///# `SBC` - Subtract with Carry
    /// - A,Z,C,N = A-M-(1-C)
    /// - This instruction subtracts the contents of a memory location to the accumulator together with the not of the carry bit. If overflow occurs the carry bit is clear, this enables multiple byte subtraction to be performed.
    pub fn sbc(&mut self) {
        let a = self.a;
        let m = self.cpu_read(self.addrabs, false);
        let m = (!m).wrapping_add(1);
        let c = if self.flags.contains(Flags::Carry) {0} else {0xFF};
        let m = m as u16;
        let a = a as u16;
        let result = a + m + c;
        self.flags.set(Flags::Carry,result > 255);
        self.flags.set(Flags::Negative,result & 0x80 != 0);
        let propa = (result & 0x80) ^ (m & 0x80);
        let propa = propa != 0;
        let propb = (a & 0x80) ^ (result & 0x80);
        let propb = propb != 0;
        self.flags.set(Flags::Overflow, propa && propb);
        self.a = (result as u8) & 0xFF;
        self.flags.set(Flags::Zero, self.a == 0);
        


    }
    
    ///# `CMP` - Compare
    /// - Z,C,N = A-M
    /// - This instruction compares the contents of the accumulator with another memory held value and sets the zero and carry flags as appropriate.
    /// C	Carry Flag	Set if A >= M
    ///
    /// Z	Zero Flag	Set if A = M
    ///
    /// N	Negative Flag	Set if bit 7 of the result is set

    pub fn cmp(&mut self) {
        let immval = self.cpu_read(self.addrabs, false);
        let temp = self.a.wrapping_sub(immval);
        self.flags.set(Flags::Carry, self.a >= immval);
        self.flags.set(Flags::Zero, temp == 0);
        self.flags.set(Flags::Negative, temp & 0x80 != 0);
    }
    ///# `CPX` - Compare X Register
    /// - Z,C,N = X-M
    /// - This instruction compares the contents of the X register with another memory held value and sets the zero and carry flags as appropriate.
    ///
    /// - C	Carry Flag	Set if X >= M
    ///
    /// - Z	Zero Flag	Set if X = M
    /// - N	Negative Flag	Set if bit 7 of the result is set
    pub fn cpx(&mut self) {
        let immval = self.cpu_read(self.addrabs, false);
        let result = self.x.wrapping_sub(immval);
        self.flags.set(Flags::Carry, self.x >= immval);
        self.flags.set(Flags::Zero, result == 0);
        self.flags.set(Flags::Negative, result & 0x80 != 0);
    }
    ///# CPY - Compare Y Register
    /// - Z,C,N = Y-M
    /// - This instruction compares the contents of the Y register with another memory held value and sets the zero and carry flags as appropriate.
    /// C	Carry Flag	Set if Y >= M
    /// Z	Zero Flag	Set if Y = M
    /// N	Negative Flag	Set if bit 7 of the result is set
    pub fn cpy(&mut self) {
        let immval = self.cpu_read(self.addrabs, false);
        let temp = self.y.wrapping_sub(immval);
        self.flags.set(Flags::Carry,self.y >= immval);
        self.flags.set(Flags::Zero,temp == 0);
        self.flags.set(Flags::Negative,temp & 0x80 != 0);
    }
}
