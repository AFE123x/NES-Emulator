use crate::cpu::Cpu;
use crate::cpu::Flags;
impl Cpu{
    ///# `TAX` - Transfer Accumulator to X
    /// - X = A
    /// - Copies the current contents of the accumulator into the X register and sets the zero and negative flags as appropriate.
    pub fn tax(&mut self) {
        self.x = self.a;
        self.flags.set(Flags::Zero,self.x == 0);
        self.flags.set(Flags::Negative,self.x & 0x80 != 0);
    }
    ///# `TAY` - Transfer Accumulator to Y
    /// - Y = A
    /// - Copies the current contents of the accumulator into the Y register and sets the zero and negative flags as appropriate.
    pub fn tay(&mut self) {
        self.y = self.a;
        self.flags.set(Flags::Negative, self.y & 0x80 != 0);
        self.flags.set(Flags::Zero,self.y == 0);
    }
    ///# `TXA` - Transfer X to Accumulator
    /// - A = X
    /// - Copies the current contents of the X register into the accumulator and sets the zero and negative flags as appropriate.
    pub fn txa(&mut self) {
        self.a = self.x;
        self.flags.set(Flags::Zero,self.a == 0);
        self.flags.set(Flags::Negative,self.a & 0x80 != 0);
    }
    ///# `TYA` - Transfer Y to Accumulator
    /// - A = Y
    /// - Copies the current contents of the Y register into the accumulator and sets the zero and negative flags as appropriate.
    pub fn tya(&mut self) {
        self.a = self.y;
        self.flags.set(Flags::Zero,self.a == 0);
        self.flags.set(Flags::Negative,self.a & 0x80 != 0);
    }
}