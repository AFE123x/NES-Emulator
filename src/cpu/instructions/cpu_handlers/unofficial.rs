// SRE (LSE)
// LSR oper + EOR oper

// M = 0 -> [76543210] -> C, A EOR M -> A
// N	Z	C	I	D	V
// +	+	+	-	-	-
// addressing	assembler	opc	bytes	cycles
// zeropage	SRE oper	47	2	5
// zeropage,X	SRE oper,X	57	2	6
// absolute	SRE oper	4F	3	6
// absolute,X	SRE oper,X	5F	3	7
// absolute,Y	SRE oper,Y	5B	3	7
// (indirect,X)	SRE (oper,X)	43	2	8
// (indirect),Y	SRE (oper),Y	53	2	8

use crate::cpu::{Cpu, Flags};

impl Cpu {
    pub fn sre(&mut self) {
        let mut immval = self.cpu_read(self.addrabs, false);
        self.flags.set(Flags::Carry, immval & 0x01 != 0); // Set carry flag to bit 0
        immval >>= 1;

        self.cpu_write(self.addrabs, immval);
        self.a = immval ^ self.a;
        self.flags.set(Flags::Zero, self.a == 0);
        self.flags.set(Flags::Negative, self.a & 0x80 != 0);
    }

    pub fn lax(&mut self) {
        let immval = self.cpu_read(self.addrabs, false);
        self.a = immval;
        self.x = immval;
        self.flags.set(Flags::Zero, self.a == 0);
        self.flags.set(Flags::Negative, self.a & 0x80 != 0);
    }

    pub fn las(&mut self) {
        let immval = self.cpu_read(self.addrabs, false);
        let temp = self.sp & immval;
        self.a = temp;
        self.x = temp;
        self.sp = temp;
        self.flags.set(Flags::Zero, self.a == 0);
        self.flags.set(Flags::Negative, self.a & 0x80 != 0);
    }

    pub fn shy(&mut self) {
        // let hibyte = (self.addrabs.wrapping_add(1) >> 8) as u8;
        // let val = self.y & hibyte;
        // self.cpu_write(self.addrabs, val);
        todo!()
    }

    pub fn rra(&mut self) {
        let mask = if self.flags.contains(Flags::Carry) {
            0x80
        } else {
            0
        };
        let mut immval = self.cpu_read(self.addrabs, false);
        self.flags.set(Flags::Carry, immval & 0x01 != 0);
        immval >>= 1;
        immval |= mask;
        self.flags.set(Flags::Zero, immval == 0);
        self.flags.set(Flags::Negative, immval & 0x80 != 0);
        self.cpu_write(self.addrabs, immval);

        /* ADC */
        let m = immval as u16;
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

    pub fn dcp(&mut self) {
        let immval = self.cpu_read(self.addrabs, false);
        let immval = immval.wrapping_sub(1);
        self.flags.set(Flags::Zero, immval == 0);
        self.flags.set(Flags::Negative, immval & 0x80 != 0);
        self.cpu_write(self.addrabs, immval);

        let immval = self.cpu_read(self.addrabs, true);
        let temp = self.a.wrapping_sub(immval);
        self.flags.set(Flags::Carry, self.a >= immval);
        self.flags.set(Flags::Zero, temp == 0);
        self.flags.set(Flags::Negative, temp & 0x80 != 0);
    }

    pub fn sax(&mut self) {
        let m = self.a & self.x;
        self.cpu_write(self.addrabs, m);
    }

    pub fn rla(&mut self) {
        let bit = if self.flags.contains(Flags::Carry) {
            1
        } else {
            0
        };
        let mut immval = self.cpu_read(self.addrabs, false);
        self.flags.set(Flags::Carry, immval & 0x80 != 0);
        immval <<= 1;
        immval |= bit;
        self.flags.set(Flags::Zero, immval == 0);
        self.flags.set(Flags::Negative, immval & 0x80 != 0);
        self.cpu_write(self.addrabs, immval);
        self.a = self.a & immval;
        self.flags.set(Flags::Zero, self.a == 0);
        self.flags.set(Flags::Negative, self.a & 0x80 != 0);
    }

    pub fn slo(&mut self) {
        let mut immval = self.cpu_read(self.addrabs, false);
        self.flags.set(Flags::Carry, immval & 0x80 != 0);
        immval <<= 1;
        self.cpu_write(self.addrabs, immval);
        self.a = self.a | immval;
        self.flags.set(Flags::Zero, self.a == 0);
        self.flags.set(Flags::Negative, self.a & 0x80 != 0);
    }

    pub fn isc(&mut self) {
        let immval = self.cpu_read(self.addrabs, false);
        let immval = immval.wrapping_add(1);
        self.flags.set(Flags::Zero, immval == 0);
        self.flags.set(Flags::Negative, immval & 0x80 != 0);
        self.cpu_write(self.addrabs, immval);

        // Read the value from memory
        let m = immval;

        // In the 6502, SBC actually performs A - M - (1-C)
        // This is equivalent to A + (~M) + C
        let m_complement = !m;

        // Get the current value of carry flag (0 or 1)
        let carry_bit = if self.flags.contains(Flags::Carry) {
            1
        } else {
            0
        };

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
            ((a ^ m_comp) & 0x80 == 0) && ((a ^ result) & 0x80 != 0),
        );

        // Update the accumulator with the result
        self.a = result_byte;
    }

    pub fn tas(&mut self) {
        // self.sp = self.a & self.x;
        // let temp = self.a & self.x & ((self.addrabs >> 8) as u8);
        // self.cpu_write(self.addrabs,temp);
        todo!()
    }

    pub fn lxa(&mut self) {
        // Read the immediate operand
        let oper = self.cpu_read(self.addrabs, false);

        // The "magic constant" is typically 0xEE or 0xFF, but can vary
        // between different 6502 chips and even between operations
        let magic_constant = 0xEE; // You might want to make this configurable

        // Perform the operation: (A OR magic_constant) AND oper
        let result = (self.a | magic_constant) & oper;

        // Store the result in both A and X
        self.a = result;
        self.x = result;

        // Update status flags (N and Z)
        self.flags.set(Flags::Zero, result == 0);
        self.flags.set(Flags::Negative, (result & 0x80) != 0);
    }

    pub fn jam(&mut self) {
        self.pc = self.pc.wrapping_sub(1);
    }

    pub fn shx(&mut self) {
        todo!()
    }

    pub fn sbx(&mut self) {
        // Get the immediate operand
        let oper = self.cpu_read(self.addrabs, false);

        // Perform (A AND X) - oper
        let a_and_x = self.a & self.x;
        let result = a_and_x.wrapping_sub(oper);

        // Set the carry flag if (A AND X) >= oper (like CMP)
        self.flags.set(Flags::Carry, a_and_x >= oper);

        // Store result in X register
        self.x = result;

        // Update N and Z flags based on the result
        self.flags.set(Flags::Zero, result == 0);
        self.flags.set(Flags::Negative, (result & 0x80) != 0);
    }

    pub fn ane(&mut self) {
        // Read the immediate operand
        let oper = self.cpu_read(self.addrabs, false);

        // Magic constant - this varies between chip revisions and conditions
        // Common values are 0x00, 0xFF, or 0xEE
        let magic_constant = 0xEE; // Could be configurable

        // Perform the operation: (A OR magic_constant) AND X AND oper
        let result = (self.a | magic_constant) & self.x & oper;

        // Store result in A
        self.a = result;

        // Update N and Z flags
        self.flags.set(Flags::Zero, result == 0);
        self.flags.set(Flags::Negative, (result & 0x80) != 0);
    }

    pub fn sha(&mut self) {
        todo!()
    }
    pub fn arr(&mut self) {
        todo!()
    }
/*
ALR (ASR)
AND oper + LSR

A AND oper, 0 -> [76543210] -> C
N	Z	C	I	D	V
+	+	+	-	-	-
addressing	assembler	opc	bytes	cycles	
immediate	ALR #oper	4B	2	2  	

*/
    pub fn alr(&mut self){
        todo!()
    }

    pub fn anc(&mut self){
        todo!()
    }
}
