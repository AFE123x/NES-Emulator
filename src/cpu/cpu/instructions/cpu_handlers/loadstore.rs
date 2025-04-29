use crate::cpu::Cpu;
use crate::cpu::Flags;
impl Cpu{
    ///# `LDA` - Load Accumulator
    /// - A,Z,N = M
    /// - Loads a byte of memory into the accumulator setting the zero and negative flags as appropriate.
    /// 
    pub fn lda(&mut self) {
        let immval = self.cpu_read(self.addrabs, false);
        self.a = immval;
        self.flags.set(Flags::Zero,self.a == 0);
        self.flags.set(Flags::Negative,self.a & 0x80 != 0);
    }
    ///`LDX` - Load X Register
    /// - X,Z,N = M
    /// - Loads a byte of memory into the X register setting the zero and negative flags as appropriate.
    pub fn ldx(&mut self) {
        let immval = self.cpu_read(self.addrabs, false);
        self.x = immval;
        self.flags.set(Flags::Zero,self.x == 0);
        self.flags.set(Flags::Negative,self.x & 0x80 != 0);
    }
    ///# `LDY` - Load Y Register
    /// - Y,Z,N = M
    /// - Loads a byte of memory into the Y register setting the zero and negative flags as appropriate.
    pub fn ldy(&mut self) {
        let immval = self.cpu_read(self.addrabs, false);
        self.y = immval;
        self.flags.set(Flags::Zero, self.y == 0);
        self.flags.set(Flags::Negative,self.y & 0x80 != 0);
    }
    ///# `STA` - Store Accumulator
    /// - M = A
    /// - Stores the contents of the accumulator into memory.
    pub fn sta(&mut self) {
        self.cpu_write(self.addrabs, self.a);
    }
    ///# `STX` - Store X Register
    /// - M = X
    /// - Stores the contents of the X register into memory.
    pub fn stx(&mut self) {
        self.cpu_write(self.addrabs, self.x);
    }
    ///# `STY` - Store Y Register
    /// - M = Y
    /// - Stores the contents of the Y register into memory.
    pub fn sty(&mut self) {
        self.cpu_write(self.addrabs, self.y);
    }
}