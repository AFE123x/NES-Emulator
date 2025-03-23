use crate::cpu::Cpu;
use crate::cpu::Flags;
impl Cpu{
    ///# `AND` - Logical AND
    /// A,Z,N = A&M
    /// A logical AND is performed, bit by bit, on the accumulator contents using the contents of a byte of memory.
    pub fn and(&mut self) {
        let immvar = self.cpu_read(self.addrabs, false);
        self.a = self.a & immvar;
        self.flags.set(Flags::Zero,self.a == 0);
        self.flags.set(Flags::Negative, self.a & 0x80 != 0);
    }
    ///# `EOR` - Exclusive OR
    /// - A,Z,N = A^M
    /// - An exclusive OR is performed, bit by bit, on the accumulator contents using the contents of a byte of memory.
    pub fn eor(&mut self) {
        let immvar = self.cpu_read(self.addrabs, false);
        self.a = self.a ^ immvar;
        self.flags.set(Flags::Zero,self.a == 0);
        self.flags.set(Flags::Negative, self.a & 0x80 != 0);
    }
    ///# `ORA` - Logical Inclusive OR
    /// - A,Z,N = A|M
    /// - An inclusive OR is performed, bit by bit, on the accumulator contents using the contents of a byte of memory.
    pub fn ora(&mut self) {
        let immvar = self.cpu_read(self.addrabs, false);
        self.a = self.a | immvar;
        self.flags.set(Flags::Zero,self.a == 0);
        self.flags.set(Flags::Negative, self.a & 0x80 != 0);
    }
    ///# `BIT` - Bit Test
    /// - A & M, N = M7, V = M6
    /// - This instructions is used to test if one or more bits are set in a target memory location. The mask pattern in A is ANDed with the value in memory to set or clear the zero flag, but the result is not kept. Bits 7 and 6 of the value from memory are copied into the N and V flags.
    pub fn bit(&mut self) {
        let immval = self.cpu_read(self.addrabs, false);
        let temp = self.a & immval;
        self.flags.set(Flags::Zero,temp == 0);
        self.flags.set(Flags::Negative, immval & 0x80 != 0);
        self.flags.set(Flags::Overflow,immval & 0x40 != 0);
    }
}