use crate::cpu::Cpu;
use crate::cpu::Flags;
impl Cpu{
    ///# `INC` - Increment Memory
    /// - M,Z,N = M+1
    /// - Adds one to the value held at a specified memory location setting the zero and negative flags as appropriate.
    pub fn inc(&mut self) {
        let immval = self.cpu_read(self.addrabs, false);
        let immval = immval.wrapping_add(1);
        self.flags.set(Flags::Zero,immval == 0);
        self.flags.set(Flags::Negative,immval & 0x80 != 0);
        self.cpu_write(self.addrabs, immval);
    }
    ///# `INX` - Increment X Register
    /// - X,Z,N = X+1
    /// - Adds one to the X register setting the zero and negative flags as appropriate.
    pub fn inx(&mut self) {
        self.x = self.x.wrapping_add(1);
        self.flags.set(Flags::Zero,self.x == 0);
        self.flags.set(Flags::Negative,self.x & 0x80 != 0);
    }
    ///# `INY` - Increment Y Register
    /// Y,Z,N = Y+1
    /// Adds one to the Y register setting the zero and negative flags as appropriate.
    pub fn iny(&mut self) {
        self.y = self.y.wrapping_add(1);
        self.flags.set(Flags::Zero,self.y == 0);
        self.flags.set(Flags::Negative,self.y & 0x80 != 0);
    }
    ///DEC - Decrement Memory
    /// - M,Z,N = M-1
    /// - Subtracts one from the value held at a specified memory location setting the zero and negative flags as appropriate.
    pub fn dec(&mut self) {
        let immval = self.cpu_read(self.addrabs, false);
        let immval = immval.wrapping_sub(1);
        self.flags.set(Flags::Zero,immval == 0);
        self.flags.set(Flags::Negative,immval & 0x80 != 0);
        self.cpu_write(self.addrabs, immval);
    }
    ///# `DEX` - Decrement X Register
    /// - X,Z,N = X-1
    /// - Subtracts one from the X register setting the zero and negative flags as appropriate.
    pub fn dex(&mut self) {
        self.x = self.x.wrapping_sub(1);
        self.flags.set(Flags::Zero,self.x == 0);
        self.flags.set(Flags::Negative,self.x & 0x80 != 0);
    }
    ///`DEY` - Decrement Y Register
    /// - Y,Z,N = Y-1
    /// - Subtracts one from the Y register setting the zero and negative flags as appropriate.
    pub fn dey(&mut self) {
        self.y = self.y.wrapping_sub(1);
        self.flags.set(Flags::Zero,self.y == 0);
        self.flags.set(Flags::Negative,self.y & 0x80 != 0);
    }
}