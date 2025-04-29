use crate::cpu::Cpu;
use crate::cpu::Flags;
impl Cpu{
    // #`BCC` - Branch if Carry Clear
    /// - If the carry flag is clear then add the relative displacement to the program counter to cause a branch to a new location.
    pub fn bcc(&mut self) {
        if !self.flags.contains(Flags::Carry){
            self.cycles_left = self.cycles_left.wrapping_add(1);
            let jump_addr = self.pc.wrapping_add(self.relval);
            if jump_addr & 0xFF00 != self.pc & 0xFF00{
                self.cycles_left = self.cycles_left.wrapping_add(2);
            }
            self.pc = jump_addr;
        }
    }
    ///# `BCS` - Branch if Carry Set
    /// - If the carry flag is set then add the relative displacement to the program counter to cause a branch to a new location.
    pub fn bcs(&mut self) {
        if self.flags.contains(Flags::Carry){
            self.cycles_left = self.cycles_left.wrapping_add(1);
            let jump_addr = self.pc.wrapping_add(self.relval);
            if jump_addr & 0xFF00 != self.pc & 0xFF00{
                self.cycles_left = self.cycles_left.wrapping_add(2);
            }
            self.pc = jump_addr;
        }
    }
    ///# `BEQ` - Branch if Equal
    ///- If the zero flag is set then add the relative displacement to the program counter to cause a branch to a new location.
    pub fn beq(&mut self) {
        if self.flags.contains(Flags::Zero){
            self.cycles_left = self.cycles_left.wrapping_add(1);
            let jump_addr = self.pc.wrapping_add(self.relval);
            if jump_addr & 0xFF00 != self.pc & 0xFF00{
                self.cycles_left = self.cycles_left.wrapping_add(2);
            }
            self.pc = jump_addr;
        }
    }
    ///# `BMI` - Branch if Minus
    /// - If the negative flag is set then add the relative displacement to the program counter to cause a branch to a new location.
    pub fn bmi(&mut self) {
        if self.flags.contains(Flags::Negative){
            self.cycles_left = self.cycles_left.wrapping_add(1);
            let jump_addr = self.pc.wrapping_add(self.relval);
            if jump_addr & 0xFF00 != self.pc & 0xFF00{
                self.cycles_left = self.cycles_left.wrapping_add(2);
            }
            self.pc = jump_addr;
        }
    }
    ///# `BNE` - Branch if Not Equal
    /// If the zero flag is clear then add the relative displacement to the program counter to cause a branch to a new location.
    pub fn bne(&mut self) {
        if !self.flags.contains(Flags::Zero){
            self.cycles_left = self.cycles_left.wrapping_add(1);
            let jump_addr = self.pc.wrapping_add(self.relval);
            if jump_addr & 0xFF00 != self.pc & 0xFF00{
                self.cycles_left = self.cycles_left.wrapping_add(2);
            }
            self.pc = jump_addr;
        }
    }
    ///# `BPL` - Branch if Positive
    /// - If the negative flag is clear then add the relative displacement to the program counter to cause a branch to a new location.
    pub fn bpl(&mut self) {
        if !self.flags.contains(Flags::Negative){
            self.cycles_left = self.cycles_left.wrapping_add(1);
            let jump_addr = self.pc.wrapping_add(self.relval);
            if jump_addr & 0xFF00 != self.pc & 0xFF00{
                self.cycles_left = self.cycles_left.wrapping_add(2);
            }
            self.pc = jump_addr;
        }
    }
    ///# `BVC` - Branch if Overflow Clear
    /// - If the overflow flag is clear then add the relative displacement to the program counter to cause a branch to a new location.


    pub fn bvc(&mut self) {
        if !self.flags.contains(Flags::Overflow){
            self.cycles_left = self.cycles_left.wrapping_add(1);
            let jump_addr = self.pc.wrapping_add(self.relval);
            if jump_addr & 0xFF00 != self.pc & 0xFF00{
                self.cycles_left = self.cycles_left.wrapping_add(2);
            }
            self.pc = jump_addr;
        }
    }
    ///# `BVS` - Branch if Overflow Set
    /// - If the overflow flag is set then add the relative displacement to the program counter to cause a branch to a new location.
    pub fn bvs(&mut self) {
        if self.flags.contains(Flags::Overflow){
            self.cycles_left = self.cycles_left.wrapping_add(1);
            let jump_addr = self.pc.wrapping_add(self.relval);
            if jump_addr & 0xFF00 != self.pc & 0xFF00{
                self.cycles_left = self.cycles_left.wrapping_add(2);
            }
            self.pc = jump_addr;
        }
    }
}