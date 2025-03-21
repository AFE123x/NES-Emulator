use super::{Cpu, Flags};

pub mod cpu_handlers;
pub mod inst_enum;

impl Cpu {
    ///# LDA
    /// The LDA instruction stores data from a source to the A register
    /// ## Flags Affected
    /// - Zero Flag: If the A register contains zero.
    /// - Negative Flag: If the most significant bit of A is set.
    pub fn lda(&mut self) {
        self.a = self.immval;
        self.flags.set(Flags::Zero, self.a == 0);
        self.flags.set(Flags::Negative, self.a & 0x80 != 0);
    }
    ///# LDX
    /// The LDX instruction stores data from a source to the X register
    /// ## Flags Affected
    /// - Zero Flag: If the X register contains zero.
    /// - Negative Flag: If the most significant bit of X is set.
    pub fn ldx(&mut self) {
        self.x = self.immval;
        self.flags.set(Flags::Zero, self.x == 0);
        self.flags.set(Flags::Negative, self.x & 0x80 != 0);
    }

    ///# LDY
    /// The LDA instruction stores data from a source to the Y register
    /// ## Flags Affected
    /// - Zero Flag: If the Y register contains zero.
    /// - Negative Flag: If the most significant bit of Y is set.
    pub fn ldy(&mut self) {
        self.y = self.immval;
        self.flags.set(Flags::Zero, self.y == 0);
        self.flags.set(Flags::Negative, self.y & 0x80 != 0);
    }

    /// # STA
    /// This will store the value of the Accumulator Register in memory.
    pub fn sta(&mut self) {
        self.cpu_write(self.addrabs, self.a);
    }

    /// # STX
    /// This will store the value of the X Register in memory.
    pub fn stx(&mut self) {
        self.cpu_write(self.addrabs, self.x);
    }

    /// # STY
    /// This will store the value of the Y Register in memory.
    pub fn sty(&mut self) {
        self.cpu_write(self.addrabs, self.y);
    }

    /// `tax` - Transfer Accumulator to X
    ///
    /// Copies the value in the Accumulator (A) to the X register. Sets the following flags:
    /// - `Zero`: Set if the result is zero.
    /// - `Negative`: Set if the result has bit 7 set (negative in two's complement).
    pub fn tax(&mut self) {
        // Transfer value from A to X.
        self.x = self.a;

        // Set the Zero flag if the result is zero.
        self.flags.set(Flags::Zero, self.a == 0);

        // Set the Negative flag if the most significant bit (bit 7) is set.
        self.flags.set(Flags::Negative, self.a & 0x80 != 0);
    }

    /// `tay` - Transfer Accumulator to Y
    ///
    /// Copies the value in the Accumulator (A) to the Y register. Sets the following flags:
    /// - `Zero`: Set if the result is zero.
    /// - `Negative`: Set if the result has bit 7 set (negative in two's complement).
    pub fn tay(&mut self) {
        // Transfer value from A to Y.
        self.y = self.a;

        // Set the Zero flag if the result is zero.
        self.flags.set(Flags::Zero, self.y == 0);

        // Set the Negative flag if the most significant bit (bit 7) is set.
        self.flags.set(Flags::Negative, self.y & 0x80 != 0);
    }

    /// `txa` - Transfer X to Accumulator
    ///
    /// Copies the value in the X register to the Accumulator (A). Sets the following flags:
    /// - `Zero`: Set if the result is zero.
    /// - `Negative`: Set if the result has bit 7 set (negative in two's complement).
    pub fn txa(&mut self) {
        // Transfer value from X to A.
        self.a = self.x;

        // Set the Zero flag if the result is zero.
        self.flags.set(Flags::Zero, self.a == 0);

        // Set the Negative flag if the most significant bit (bit 7) is set.
        self.flags.set(Flags::Negative, self.a & 0x80 != 0);
    }

    /// # TYA
    /// Transfers the content of the Y register into the A register
    /// ## Flags
    /// - Zero flag is set if the accumulator is 0
    /// - Negative flag is set if the msb of the accumulator is 1
    pub fn tya(&mut self) {
        self.a = self.y;
        self.flags.set(Flags::Zero, self.a == 0);
        self.flags.set(Flags::Negative, self.a & 0x80 != 0);
    }

    pub fn tsx(&mut self) {
        self.x = self.sp;
        self.flags.set(Flags::Zero, self.x == 0);
        self.flags.set(Flags::Negative, self.x & 0x80 != 0);
    }

    pub fn txs(&mut self) {
        self.sp = self.x;
    }
    ///# PHA
    /// This instruction pushed the accumulator on the stack
    pub fn pha(&mut self) {
        let write_addr = self.sp as u16;
        self.cpu_write(0x100 + write_addr, self.a);
        self.sp = self.sp.wrapping_sub(1);
    }

    pub fn php(&mut self) {
        let write_addr = self.sp as u16;
        self.cpu_write(0x100 + write_addr, self.flags.bits());
        self.sp = self.sp.wrapping_sub(1);
    }
    /// # PHA
    /// - this instruction will pop a value from the stack, and store it in the Accumulator
    /// ## Flags
    /// - Zero flag is set if the result value is 0.
    /// - Negative flag is set if the MSB of the result is 1.
    pub fn pla(&mut self) {
        self.sp = self.sp.wrapping_add(1);
        let write_address = self.sp as u16;
        self.a = self.cpu_read(0x100 + write_address, false);
        self.flags.set(Flags::Zero, self.a == 0);
        self.flags.set(Flags::Negative, self.a & 0x80 != 0);
    }

    pub fn plp(&mut self) {
        self.sp = self.sp.wrapping_add(1);
        let write_address = self.sp as u16;
        let byte = self.cpu_read(0x100 + write_address, false);
        self.flags = Flags::from_bits_truncate(byte);

    }
    /// # AND
    /// This instruction will perform a bitwise AND instruction between the Accumulator and Value in memory. A = A & M
    /// ## Flags
    /// - Zero flag is set if the result is 0
    /// - Negative flag set if the msb of the result is 1.
    pub fn and(&mut self) {
        self.a = self.a & self.immval;
        self.flags.set(Flags::Zero, self.a == 0);
        self.flags.set(Flags::Negative, self.a & 0x80 != 0);
    }
    /// # EOR
    /// This instructions will perform a bitwise XOR between the Accumulator and Value in memory. A = A ^ M;
    pub fn eor(&mut self) {
        self.a = self.a ^ self.immval;
        self.flags.set(Flags::Zero, self.a == 0);
        self.flags.set(Flags::Negative, self.a & 0x80 != 0);
    }
    /// # ORA
    /// This instruction will perform a bitwise OR between the accumulator and value in memory. A = A | M
    /// ## Flags
    /// - Zero flag is set if the result is 0
    /// - Negative flag is set if the msb of the result is 1
    pub fn ora(&mut self) {
        self.a = self.a | self.immval;
        self.flags.set(Flags::Zero, self.a == 0);
        self.flags.set(Flags::Negative, self.a & 0x80 != 0);
    }

    pub fn bit(&mut self) {
        let temp = self.a & self.immval;
        self.flags.set(Flags::Zero, temp == 0);
        self.flags.set(Flags::Overflow,self.immval & 0x40 != 0);
        self.flags.set(Flags::Negative,self.immval & 0x80 != 0);
    }

    /// # ADC
    /// The Add with Carry instruction will add the accumulator, memory, and carry bit, and store the result into the accumulator.
    /// ## Flags
    /// - The Zero flag is set if the sum is 0
    /// - The Carry flag is set if the result > 255
    /// - The Overflow flag is set if the result sign is incorrect
    /// - The Negative flag is set if the msb of the result is 1
    pub fn adc(&mut self) {
        let a = self.a as u16;
        let m = self.immval as u16;
        let c = if self.flags.contains(Flags::Carry) {
            1
        } else {
            0
        };
        let result = a + m + c;
        self.flags.set(Flags::Carry, result > 255);
        let result = result & 0xFF;
        self.flags.set(Flags::Zero, result == 0);
        self.flags.set(Flags::Negative, result & 0x80 != 0);
        let prop_a = ((a & 0x80) ^ (result & 0x80)) != 0;
        let prop_b = ((m & 0x80) ^ (result & 0x80)) != 0;
        self.flags.set(Flags::Overflow, prop_a && prop_b);
        self.a = result as u8;
    }

    pub fn sbc(&mut self) {
        let a = self.a as u16;
        let m = !self.immval;
        let m = m.wrapping_add(1);
        let m = m as u16;
        let c = if self.flags.contains(Flags::Carry) {
            0
        } else {
            1
        } as u8;
        let c = !c;
        let c = c.wrapping_add(1);
        let c = c as u16;
        let result = a + m + c;
        self.flags.set(Flags::Carry, result > 255);
        let result = result & 0xFF;
        self.flags.set(Flags::Zero, result == 0);
        self.flags.set(Flags::Negative, result & 0x80 != 0);
        let prop_a = ((a & 0x80) ^ (result & 0x80)) != 0;
        let prop_b = ((m & 0x80) ^ (result & 0x80)) != 0;
        self.flags.set(Flags::Overflow, prop_a && prop_b);
        self.a = result as u8;
    }

    pub fn cmp(&mut self) {
        let temp = self.a.wrapping_sub(self.immval);
        self.flags.set(Flags::Carry, self.a >= self.immval);
        self.flags.set(Flags::Zero, self.a == self.immval);
        self.flags.set(Flags::Negative, temp & 0x80 != 0);
    }

    pub fn cpx(&mut self) {
        let temp = self.x.wrapping_sub(self.immval);
        self.flags.set(Flags::Carry, self.x >= self.immval);
        self.flags.set(Flags::Zero, self.x == self.immval);
        self.flags.set(Flags::Negative, temp & 0x80 != 0);
    }

    pub fn cpy(&mut self) {
        let temp = self.y.wrapping_sub(self.immval);
        self.flags.set(Flags::Carry, self.y >= self.immval);
        self.flags.set(Flags::Zero, self.y == self.immval);
        self.flags.set(Flags::Negative, temp & 0x80 != 0);
    }

    /// `inc` - Increment Memory
    ///
    /// Increments the value at a memory location by 1. Sets the following flags:
    /// - `Zero`: Set if the result is zero.
    /// - `Negative`: Set if the result has bit 7 set (negative in two's complement).
    pub fn inc(&mut self) {
        // Increment the immediate value by 1 with wrapping addition (handles overflow).
        self.immval = self.immval.wrapping_add(1);

        // Set the Zero flag if the result is zero.
        self.flags.set(Flags::Zero, self.immval == 0);

        // Set the Negative flag if the most significant bit (bit 7) is set.
        self.flags.set(Flags::Negative, self.immval & 0x80 != 0);

        // Write the modified value back to memory at the absolute address.
        self.cpu_write(self.addrabs, self.immval);
    }

    /// `inx` - Increment X Register
    ///
    /// Increments the value in the X register by 1. Sets the following flags:
    /// - `Zero`: Set if the result is zero.
    /// - `Negative`: Set if the result has bit 7 set (negative in two's complement).
    pub fn inx(&mut self) {
        // Increment the X register by 1 with wrapping addition (handles overflow).
        self.x = self.x.wrapping_add(1);

        // Set the Zero flag if the result is zero.
        self.flags.set(Flags::Zero, self.x == 0);

        // Set the Negative flag if the most significant bit (bit 7) is set.
        self.flags.set(Flags::Negative, self.x & 0x80 != 0);
    }

    /// `iny` - Increment Y Register
    ///
    /// Increments the value in the Y register by 1. Sets the following flags:
    /// - `Zero`: Set if the result is zero.
    /// - `Negative`: Set if the result has bit 7 set (negative in two's complement).
    pub fn iny(&mut self) {
        // Increment the Y register by 1 with wrapping addition (handles overflow).
        self.y = self.y.wrapping_add(1);

        // Set the Zero flag if the result is zero.
        self.flags.set(Flags::Zero, self.y == 0);

        // Set the Negative flag if the most significant bit (bit 7) is set.
        self.flags.set(Flags::Negative, self.y & 0x80 != 0);
    }

    /// `dec` - Decrement Memory
    ///
    /// Decrements the value at a memory location by 1. Sets the following flags:
    /// - `Zero`: Set if the result is zero.
    /// - `Negative`: Set if the result has bit 7 set (negative in two's complement).
    pub fn dec(&mut self) {
        // Decrement the immediate value by 1 with wrapping subtraction (handles underflow).
        self.immval = self.immval.wrapping_sub(1);

        // Set the Zero flag if the result is zero.
        self.flags.set(Flags::Zero, self.immval == 0);

        // Set the Negative flag if the most significant bit (bit 7) is set.
        self.flags.set(Flags::Negative, self.immval & 0x80 != 0);

        // Write the modified value back to memory at the absolute address.
        self.cpu_write(self.addrabs, self.immval);
    }

    /// `dex` - Decrement X Register
    ///
    /// Decrements the value in the X register by 1. Sets the following flags:
    /// - `Zero`: Set if the result is zero.
    /// - `Negative`: Set if the result has bit 7 set (negative in two's complement).
    pub fn dex(&mut self) {
        // Decrement the X register by 1 with wrapping subtraction (handles underflow).
        self.x = self.x.wrapping_sub(1);

        // Set the Zero flag if the result is zero.
        self.flags.set(Flags::Zero, self.x == 0);

        // Set the Negative flag if the most significant bit (bit 7) is set.
        self.flags.set(Flags::Negative, self.x & 0x80 != 0);
    }

    /// # DEY
    /// This instruction will Decrement the Y register by 1.
    /// ## Flags
    /// - Zero Flag is Y is 0
    /// - Negative Flag if the msb of Y is 1
    pub fn dey(&mut self) {
        self.y = self.y.wrapping_sub(1);
        self.flags.set(Flags::Zero, self.y == 0);
        self.flags.set(Flags::Negative, self.y & 0x80 != 0);
    }
    /// # ASL
    /// This instruction performs a arithmetic left shift on either the accumulator or value in memory.
    /// ## Flags
    /// - Carry flag is if the msb of the pre-calculated value is 1.
    /// - Zero flag is set if the calculated value is 0
    /// - Negative flag is the msb of the calculated value is 1.
    pub fn asl(&mut self) {
        let mut temp = if self.opcode == 0x0a {
            self.a
        } else {
            self.immval
        };
        self.flags.set(Flags::Carry, temp & 0x80 != 0);
        temp = temp << 1;
        self.flags.set(Flags::Zero, temp == 0);
        self.flags.set(Flags::Negative, temp & 0x80 != 0);

        if self.opcode == 0x0a {
            self.a = temp;
        } else {
            self.cpu_write(self.addrabs, temp);
        }
    }

    /// # LSR
    /// This will perform a logical right shift on either the Accumulator, or some value in memory by 1.
    /// ## Flags
    /// - Zero Flag is enabled if the result is Zero
    /// - Negative flag is enabled if the msb of the result is 1.
    pub fn lsr(&mut self) {
        if self.opcode == 0x4a {
            /* Accumulator addressing mode */
            self.flags.set(Flags::Carry, self.a & 0x1 != 0);
            self.a = self.a >> 1;
            self.flags.set(Flags::Zero, self.a == 0);
            self.flags.set(Flags::Negative, self.a & 0x80 != 0);
        } else {
            self.flags.set(Flags::Carry, self.immval & 0x1 != 0);
            self.immval = self.immval >> 1;
            self.flags.set(Flags::Zero, self.immval == 0);
            self.flags.set(Flags::Negative, self.immval & 0x80 != 0);
            self.cpu_write(self.addrabs, self.immval);
        }
    }

    /// # ROL
    /// This will perform a bitwise left shift, and set the lsb to the carry bit.
    /// ## Flags
    /// - Carry flag if the msb of the old value is 1.
    /// - Zero flag set if the final value is 0.
    /// - Negative flag set if the msb of the final value is 1.
    pub fn rol(&mut self) {
        /* Get the source (it depends on the addressing mode) */
        let mut temp = if self.opcode == 0x2a {
            self.a
        } else {
            self.immval
        };

        /* Read the Carry bit for wrapping the bit around. */
        let set_bit = if self.flags.contains(Flags::Carry) {
            1
        } else {
            0
        };

        self.flags.set(Flags::Carry, temp & 0x80 != 0); //set the new carry flag (old msb)

        /* wraps bits based on old carry bit */
        temp = temp << 1;
        temp = temp | set_bit;

        self.flags.set(Flags::Zero, temp == 0); //set if the result is 0
        self.flags.set(Flags::Negative, temp & 0x80 != 0); //set if the msb of the result is 1.

        /* Write the value to either the accumulator or the address where the value was read from. */
        if self.opcode == 0x2a {
            self.a = temp;
        } else {
            self.cpu_write(self.addrabs, temp);
        }
    }

    pub fn ror(&mut self) {
        let mask = if self.flags.contains(Flags::Carry) {
            0x80
        } else {
            0
        };
        let mut temp = if self.opcode == 0x6a {
            self.a
        } else {
            self.immval
        };
        self.flags.set(Flags::Carry, temp & 0x1 != 0);
        temp = temp >> 1;
        temp = temp | mask;
        self.flags.set(Flags::Negative, temp & 0x80 != 0);
        self.flags.set(Flags::Zero, temp == 0);
        /* write result */
        if self.opcode == 0x6a {
            self.a = temp;
        } else {
            self.cpu_write(self.addrabs, temp);
        }
    }

    pub fn jmp(&mut self) {
        self.pc = self.addrabs
    }
    ///# JSR
    /// The JSR instruction pushes the address (minus one) of the return point on to the stack and then sets the program counter to the target memory address.
    /// ## Steps
    /// - Calculate PC - 1
    /// - Store PC - 1 on the stack
    /// - set PC to the target address
    pub fn jsr(&mut self) {
        let temp_address = self.pc - 1;
        let lo = temp_address & 0xFF;
        let hi = temp_address >> 8;
        /* write hi byte first */
        let write_address = self.sp as u16;
        self.cpu_write(0x100 + write_address, hi as u8);
        self.sp = self.sp.wrapping_sub(1);
        let write_address = self.sp as u16;
        self.cpu_write(0x100 + write_address, lo as u8);
        self.sp = self.sp.wrapping_sub(1);

        self.pc = self.addrabs;
    }

    ///# RTS
    /// This instruction is called at the end of a subroutine, to return to the calling routine.
    /// ## Steps
    /// We get (PC - 1) from the stack
    /// Add one to the result.
    /// assign the PC to the result
    pub fn rts(&mut self) {
        self.sp = self.sp.wrapping_add(1);
        let write_address = self.sp as u16;
        let lo = self.cpu_read(0x100 + write_address, false) as u16;
        self.sp = self.sp.wrapping_add(1);
        let write_address = self.sp as u16;
        let hi = self.cpu_read(0x100 + write_address, false) as u16;
        self.pc = (hi << 8) | lo;
        self.pc = self.pc + 1;
    }
    /// # BCC
    /// The BCC Instruction will branch off to a different part of the program if the Carry Flag is clear
    /// ## Cycles
    /// - If the Carry flag is clear, it adds one clock cycle
    /// - If the new PC address is on a different page then the program counter, we add two clock cycles.
    pub fn bcc(&mut self) {
        if !self.flags.contains(Flags::Carry) {
            /* branch succeeds! */
            self.cycles_left += 1; // one clock cycle if it succeeds.
            let temp: u16 = self.pc.wrapping_add(self.relval);
            if temp & 0xFF00 != self.pc & 0xFF00 {
                /* If the result is on a new page, we add two clock cycles. */
                self.total_cycles += 2;
            }
            self.pc = temp;
        }
    }

    /// # BCS
    /// The BCS Instruction will branch off to a different part of the program if the Carry Flag is enabled
    /// ## Cycles
    /// - If the Carry flag is set, it adds one clock cycle
    /// - If the new PC address is on a different page then the program counter, we add two clock cycles.
    pub fn bcs(&mut self) {
        if self.flags.contains(Flags::Carry) {
            /* branch succeeds! */
            self.cycles_left += 1; // one clock cycle if it succeeds.
            let temp: u16 = self.pc.wrapping_add(self.relval);
            if temp & 0xFF00 != self.pc & 0xFF00 {
                /* If the result is on a new page, we add two clock cycles. */
                self.total_cycles += 2;
            }
            self.pc = temp;
        }
    }

    /// # BEQ
    /// The BEQ Instruction will branch off to a different part of the program if the Zero Flag is enabled
    /// ## Cycles
    /// - If the Zero flag is set, it adds one clock cycle
    /// - If the new PC address is on a different page then the program counter, we add two clock cycles.
    pub fn beq(&mut self) {
        if self.flags.contains(Flags::Zero) {
            /* branch succeeds! */
            self.cycles_left += 1; // one clock cycle if it succeeds.
            let temp: u16 = self.pc.wrapping_add(self.relval);
            if temp & 0xFF00 != self.pc & 0xFF00 {
                /* If the result is on a new page, we add two clock cycles. */
                self.total_cycles += 2;
            }
            self.pc = temp;
        }
    }

    /// # BMI
    /// The BMI Instruction branches if the Negative Flag is set
    /// ## Cycles
    /// - If the Negative flag is set, we add one clock cycle.
    /// - If the new PC address is on a different page then the program counter, we add two clock cycles.
    pub fn bmi(&mut self) {
        if self.flags.contains(Flags::Negative) {
            /* branch succeeds! */
            self.cycles_left += 1; // one clock cycle if it succeeds.
            let temp: u16 = self.pc.wrapping_add(self.relval);
            if temp & 0xFF00 != self.pc & 0xFF00 {
                /* If the result is on a new page, we add two clock cycles. */
                self.total_cycles += 2;
            }
            self.pc = temp;
        }
    }
    ///# BNE
    /// This instruction will jump to a different part of the program if the Zero Flag is not enabled.
    /// ## Cycles
    /// - If the Zero Flag is not set, we add one clock cycle
    /// - If the new PC address is on a different page, then we add two clock cycles.
    pub fn bne(&mut self) {
        if !self.flags.contains(Flags::Zero) {
            /* branch succeeds! */
            self.cycles_left += 1; // one clock cycle if it succeeds.
            let temp: u16 = self.pc.wrapping_add(self.relval);
            if temp & 0xFF00 != self.pc & 0xFF00 {
                /* If the result is on a new page, we add two clock cycles. */
                self.total_cycles += 2;
            }
            self.pc = temp;
        }
    }

    /// # BPL
    /// This instruction will branch if the negative flag is not enabled
    /// ## Cycles
    /// - If the Zero Flag is not set, we add one clock cycle
    /// - If the new PC address is on a different page, then we add two clock cycles.
    pub fn bpl(&mut self) {
        if !self.flags.contains(Flags::Negative) {
            /* branch succeeds! */
            self.cycles_left += 1; // one clock cycle if it succeeds.
            let temp: u16 = self.pc.wrapping_add(self.relval);
            if temp & 0xFF00 != self.pc & 0xFF00 {
                /* If the result is on a new page, we add two clock cycles. */
                self.total_cycles += 2;
            }
            self.pc = temp;
        }
    }

    pub fn bvc(&mut self) {
        if !self.flags.contains(Flags::Overflow) {
            /* branch succeeds! */
            self.cycles_left += 1; // one clock cycle if it succeeds.
            let temp: u16 = self.pc.wrapping_add(self.relval);
            if temp & 0xFF00 != self.pc & 0xFF00 {
                /* If the result is on a new page, we add two clock cycles. */
                self.total_cycles += 2;
            }
            self.pc = temp;
        }
    }

    pub fn bvs(&mut self) {
        if self.flags.contains(Flags::Overflow) {
            /* branch succeeds! */
            self.cycles_left += 1; // one clock cycle if it succeeds.
            let temp: u16 = self.pc.wrapping_add(self.relval);
            if temp & 0xFF00 != self.pc & 0xFF00 {
                /* If the result is on a new page, we add two clock cycles. */
                self.total_cycles += 2;
            }
            self.pc = temp;
        }
    }

    pub fn brk(&mut self) {
        todo!()
    }

    /// `rti` - Return from Interrupt
    ///
    /// Restores the processor state from the stack. This includes:
    /// - Flags: Pulled from the stack and restored.
    /// - Program Counter (PC): Pulled from the stack (2 bytes).
    pub fn rti(&mut self) {
        // Pull flags from the stack and restore them.
        self.sp = self.sp.wrapping_add(1);
        let written_address = self.sp as u16;
        let flag = self.cpu_read(0x100 + written_address, false);
        self.flags = Flags::from_bits_truncate(flag);

        // Pull the Program Counter (PC) from the stack (2 bytes: low byte, then high byte).
        self.sp = self.sp.wrapping_add(1);
        let written_address = self.sp as u16;
        let lo = self.cpu_read(0x100 + written_address, false) as u16;
        self.sp = self.sp.wrapping_add(1);
        let written_address = self.sp as u16;
        let hi = self.cpu_read(0x100 + written_address, false) as u16;

        // Combine high and low bytes to form the complete program counter.
        self.pc = (hi << 8) | lo;
    }

    ///# NMI
    /// This is a **Non Maskable Interrupt**. This means that, regardless of the Interrupt disable flag, the CPU will have to handle the interrupt.
    /// ## Steps
    /// - Push the Program Counter onto the Stack.
    /// - Set the Interrupt Disable flag.
    /// - Push the Status Register on the Stack.
    /// - Consult the vector table, get the address of the handler at address $FFFA and $FFFB.
    /// - Store the address of the handler into the program counter
    pub fn nmi(&mut self) {
        /* Step one: get the lo and hi byte of the PC */
        let lo = self.pc & 0xFF;
        let hi = self.pc >> 8;

        /* The 6502 is Little Endian, so we store the hi byte first, then lo byte */

        let write_address = self.sp as u16;
        self.cpu_write(0x100 + write_address, hi as u8); //writing the high byte
        self.sp = self.sp.wrapping_sub(1); // Subtract the Stack Pointer Register

        let write_address = self.sp as u16;
        self.cpu_write(0x100 + write_address, lo as u8); //writing the low byte.
        self.sp = self.sp.wrapping_sub(1);

        /* Step two: Set the Interrupt disable flag */
        self.flags.set(Flags::IDisable, true);

        /* Step three, push the flag register to the stack */
        let write_address = self.sp as u16;
        self.cpu_write(0x100 + write_address, self.flags.bits()); //writing the status register
        self.sp = self.sp.wrapping_sub(1);

        /* get the new program counter */
        let lo = self.cpu_read(0xFFFA, false) as u16; // get the low byte
        let hi = self.cpu_read(0xFFFB, false) as u16;
        self.pc = (hi << 8) | lo;
    }

    /// # Reset
    /// This function resets the cpu to it's initial state.
    /// - The PC is set to the address located at 0xFFFC (little endian).
    /// - a, x, y and the flag is set to 0.
    /// - Resets take 8 clock cycles to complete
    /// - Total cycles is reset to 0.
    pub fn reset(&mut self) {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        let lo_byte = self.cpu_read(0xFFFC, false) as u16;
        let hi_byte = self.cpu_read(0xFFFD, false) as u16;
        self.pc = (hi_byte << 8) | lo_byte;
        self.flags = Flags::empty();
        self.flags.set(Flags::Unused, true);
        self.cycles_left = 8;
        self.total_cycles = 0;
    }
}
