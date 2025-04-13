use crate::cpu::Cpu;
use crate::cpu::Flags;
impl Cpu {
    ///# `ADC` - Add with Carry
    /// - A,Z,C,N = A+M+C
    /// - This instruction adds the contents of a memory location to the accumulator together with the carry bit. If overflow occurs the carry bit is set, this enables multiple byte addition to be performed.
    pub fn adc(&mut self) {
        let m = self.cpu_read(self.addrabs, false) as u16;
        let carry_bit = if self.flags.contains(Flags::Carry) {
            1
        } else {
            0
        };
        let a = self.a as u16;
        let result = a + m + carry_bit;

        // Set carry flag if result overflows 8 bits
        self.flags.set(Flags::Carry, result > 0xFF);

        // Get the 8-bit result by masking
        let result_byte = (result & 0xFF) as u8;

        // Set zero flag if result is zero
        self.flags.set(Flags::Zero, result_byte == 0);

        // Set negative flag if bit 7 is set
        self.flags.set(Flags::Negative, result_byte & 0x80 != 0);

        // Set overflow flag when both inputs have the same sign, but the result has a different sign
        self.flags.set(
            Flags::Overflow,
            ((a ^ m) & 0x80 == 0) && ((a ^ result) & 0x80 != 0),
        );

        self.a = result_byte;
    }

    /// # `SBC` - Subtract with Carry
    /// - A,Z,C,N = A-M-(1-C)
    /// - This instruction subtracts the contents of a memory location from the
    ///   accumulator together with the NOT of the carry bit. If overflow occurs the
    ///   carry bit is cleared, enabling multiple byte subtraction to be performed.
    pub fn sbc(&mut self) {
        // Read the value from memory
        let m = self.cpu_read(self.addrabs, false);
        
        // In the 6502, SBC actually performs A - M - (1-C)
        // This is equivalent to A + (~M) + C
        let m_complement = !m;
        
        // Get the current value of carry flag (0 or 1)
        let carry_bit = if self.flags.contains(Flags::Carry) { 1 } else { 0 };
        
        // Convert values to u16 for calculations
        let a = self.a as u16;
        let m_comp = m_complement as u16;
        
        // Calculate result: A + ~M + C
        let result = a + m_comp + carry_bit;
        
        // Set carry flag if result overflows 8 bits
        self.flags.set(Flags::Carry, result > 0xFF);
        
        // Get the 8-bit result by masking
        let result_byte = (result & 0xFF) as u8;
        
        // Set zero flag if result is zero
        self.flags.set(Flags::Zero, result_byte == 0);
        
        // Set negative flag if bit 7 is set
        self.flags.set(Flags::Negative, result_byte & 0x80 != 0);
        
        // Set overflow flag
        // Overflow occurs when both inputs have the same sign but the result has a different sign
        self.flags.set(
            Flags::Overflow, 
            ((a ^ m_comp) & 0x80 == 0) && ((a ^ result) & 0x80 != 0)
        );
        
        // Update the accumulator with the result
        self.a = result_byte;
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
        self.flags.set(Flags::Carry, self.y >= immval);
        self.flags.set(Flags::Zero, temp == 0);
        self.flags.set(Flags::Negative, temp & 0x80 != 0);
    }
}
