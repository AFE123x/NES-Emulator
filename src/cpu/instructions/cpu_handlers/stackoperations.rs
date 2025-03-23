use crate::cpu::Cpu;
use crate::cpu::Flags;
impl Cpu{
    ///# `TSX` - Transfer Stack Pointer to X
    /// - X = S
    /// - Copies the current contents of the stack register into the X register and sets the zero and negative flags as appropriate.
    pub fn tsx(&mut self) {
        self.x = self.sp;
        self.flags.set(Flags::Zero,self.x == 0);
        self.flags.set(Flags::Negative,self.x & 0x80 != 0);
    }
    ///# `TXS` - Transfer X to Stack Pointer
    /// - S = X
    /// - Copies the current contents of the X register into the stack register.
    pub fn txs(&mut self) {
        self.sp = self.x;
    }
    ///# `PHA` - Push Accumulator
    /// - Pushes a copy of the accumulator on to the stack.
    pub fn pha(&mut self) {
        self.push(self.a);
    }
    ///# `PHP` - Push Processor Status
    /// - Pushes a copy of the status flags on to the stack.
    pub fn php(&mut self) {
        self.push(self.flags.bits());
    }
    ///# `PLA` - Pull Accumulator
    /// - Pulls an 8 bit value from the stack and into the accumulator. The zero and negative flags are set as appropriate.
    pub fn pla(&mut self) {
        self.a = self.pop();
        self.flags.set(Flags::Negative,self.a & 0x80 != 0);
        self.flags.set(Flags::Zero,self.a == 0);
    }
    ///# `PLP` - Pull Processor Status
    /// - Pulls an 8 bit value from the stack and into the processor flags. The flags will take on new states as determined by the value pulled.
    pub fn plp(&mut self) {
        self.flags = Flags::from_bits_truncate(self.pop());
        self.flags.set(Flags::Unused,true);    
    }
}