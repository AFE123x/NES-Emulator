use crate::cpu::Cpu;
impl Cpu{
    ///# `JMP` - Jump
    /// - Sets the program counter to the address specified by the operand.
    pub fn jmp(&mut self) {
        self.pc = self.addrabs;

    }
    ///# `JSR` - Jump to Subroutine
    /// - The JSR instruction pushes the address (minus one) of the return point on to the stack and then sets the program counter to the target memory address.
    pub fn jsr(&mut self) {
        let pc = self.pc.wrapping_sub(1);
        let hi_byte = pc >> 8;
        let lo_byte = pc & 0xFF;
        let hi_byte = hi_byte as u8;
        let lo_byte = lo_byte as u8;

        /* Push high byte first (little endian)*/
        self.push(hi_byte);
        /* Push the low byte */
        self.push(lo_byte);
        self.pc = self.addrabs;

    }
    ///RTS - Return from Subroutine
    /// - The RTS instruction is used at the end of a subroutine to return to the calling routine. It pulls the program counter (minus one) from the stack.
    pub fn rts(&mut self) {
        let lo_byte = self.pop() as u16;
        let hi_byte = self.pop() as u16;
        self.pc = (hi_byte << 8) | lo_byte;
        self.pc = self.pc.wrapping_add(1);
    }
}