use crate::cpu::Cpu;
use crate::cpu::Flags;
impl Cpu{
    fn asl_accumulator(&mut self){
        self.flags.set(Flags::Carry,self.a & 0x80 != 0);
        self.a <<= 1;
        self.flags.set(Flags::Zero,self.a == 0);
        self.flags.set(Flags::Negative,self.a & 0x80 != 0);
    }

    fn asl_memory(&mut self){
        let mut immval = self.cpu_read(self.addrabs, false);
        self.flags.set(Flags::Carry,immval & 0x80 != 0);
        immval <<= 1;
        self.flags.set(Flags::Zero,immval == 0);
        self.flags.set(Flags::Negative,immval & 0x80 != 0);
        self.cpu_write(self.addrabs, immval);
    }
    ///# `ASL` - Arithmetic Shift Left
    /// - A,Z,C,N = M*2 or M,Z,C,N = M*2
    /// - This operation shifts all the bits of the accumulator or memory contents one bit left. Bit 0 is set to 0 and bit 7 is placed in the carry flag. The effect of this operation is to multiply the memory contents by 2 (ignoring 2's complement considerations), setting the carry if the result will not fit in 8 bits.
    pub fn asl(&mut self) {
        if self.opcode == 0xA{
            self.asl_accumulator();
        }
        else{
            self.asl_memory();
        }
    }

    fn lsr_accumulator_helper(&mut self){
        self.flags.set(Flags::Carry,self.a & 1 != 0); /* Set the carry flag to bit 0 of the accumulator */
        self.a >>= 1;
        self.flags.set(Flags::Zero,self.a == 0); /* Set zero flag */
        self.flags.set(Flags::Negative,self.a & 0x80 != 0); /* Set negative flag */
    }

    fn lsr_memory_helper(&mut self){
        let mut immval = self.cpu_read(self.addrabs, false);
        self.flags.set(Flags::Carry,immval & 1 != 0); /* Set the carry flag to bit 0 of the accumulator */
        immval >>= 1;
        self.flags.set(Flags::Zero,immval == 0); /* Set zero flag */
        self.flags.set(Flags::Negative,immval & 0x80 != 0); /* Set negative flag */
        self.cpu_write(self.addrabs, immval);
    }

    ///# `LSR` - Logical Shift Right
    /// A,C,Z,N = A/2 or M,C,Z,N = M/2
    /// Each of the bits in A or M is shift one place to the right. The bit that was in bit 0 is shifted into the carry flag. Bit 7 is set to zero.
    /// 
    /// C	Carry Flag	Set to contents of old bit 0
    /// 
    /// Z	Zero Flag	Set if result = 0
    /// 
    /// N	Negative Flag	Set if bit 7 of the result is set
    pub fn lsr(&mut self) {
        if self.opcode == 0x4A{
            self.lsr_accumulator_helper();
        }
        else{
            self.lsr_memory_helper();
        }
    }

    fn rol_accumulator(&mut self){
        let bit = if self.flags.contains(Flags::Carry) {1} else {0};
        self.flags.set(Flags::Carry, self.a & 0x80 != 0);
        self.a <<= 1;
        self.a |= bit;
        self.flags.set(Flags::Zero,self.a == 0);
        self.flags.set(Flags::Negative,self.a & 0x80 != 0);
    }

    fn rol_memory(&mut self){
        let bit = if self.flags.contains(Flags::Carry) {1} else {0};
        let mut immval = self.cpu_read(self.addrabs, false);
        self.flags.set(Flags::Carry, immval & 0x80 != 0);
        immval <<= 1;
        immval |= bit;
        self.flags.set(Flags::Zero,immval == 0);
        self.flags.set(Flags::Negative,immval & 0x80 != 0);
        self.cpu_write(self.addrabs, immval);
    }
    ///# ROL - Rotate Left
    ///- Move each of the bits in either A or M one place to the left. Bit 0 is filled with the current value of the carry flag whilst the old bit 7 becomes the new carry flag value.
    pub fn rol(&mut self) {
        if self.opcode == 0x2A{
            self.rol_accumulator();
        }
        else{
            self.rol_memory();
        }
    }


    fn ror_accumulator(&mut self){
        let mask = if self.flags.contains(Flags::Carry) {0x80} else {0};
        self.flags.set(Flags::Carry,self.a & 0x1 != 0);
        self.a >>= 1;
        self.a |= mask;
        self.flags.set(Flags::Negative,self.a & 0x80 != 0);
        self.flags.set(Flags::Zero,self.a == 0);
    }

    fn ror_memory(&mut self){
        let mask = if self.flags.contains(Flags::Carry) {0x80} else {0};
        let mut immval = self.cpu_read(self.addrabs, false);
        self.flags.set(Flags::Carry,immval & 0x1 != 0);
        immval >>= 1;
        immval |= mask;
        self.flags.set(Flags::Negative,immval & 0x80 != 0);
        self.flags.set(Flags::Zero,immval == 0);
        self.cpu_write(self.addrabs, immval);
    }
    
    ///# `ROR` - Rotate Right
    /// - Move each of the bits in either A or M one place to the right. Bit 7 is filled with the current value of the carry flag whilst the old bit 0 becomes the new carry flag value.
    /// 
    /// C	Carry Flag	Set to contents of old bit 0
    /// 
    /// Z	Zero Flag	Set if A = 0
    /// 
    /// N	Negative Flag	Set if bit 7 of the result is set
    pub fn ror(&mut self) {
        if self.opcode == 0x6A{
            self.ror_accumulator();
        }
        else{
            self.ror_memory();
        }
    }
}