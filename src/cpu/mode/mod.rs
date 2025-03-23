//! # Addressing Modes
//! This file contains the implementations for all the addressing modes of the 6502
use super::Cpu;

impl Cpu {
    ///# Fetch
    /// This is a helper function for the addressing modes.
    /// ## Unsafe
    /// This function does use unsafe rust, hence why it's covered in a function.
    fn fetch(&mut self) -> u8{
        let result = unsafe{
            (*self.bus.unwrap()).cpu_read(self.pc, false)
        };
        self.pc = self.pc.wrapping_add(1);
        result
    }
    ///# Implicit
    /// For many 6502 instructions the source and destination of the information to be manipulated is implied directly by the function of the instruction itself and no further operand needs to be specified. Operations like 'Clear Carry Flag' (CLC) and 'Return from Subroutine' (RTS) are implicit.
    pub fn implicit(&mut self) {
        /* Nothing to do here */
    }

    ///# Accumulator
    /// Some instructions have an option to operate directly upon the accumulator. The programmer specifies this by using a special operand value, 'A'. For example:
    /// ```
    ///  LSR A           ;Logical shift right one bit
    ///  ROR A           ;Rotate right one bit
    /// ```
    pub fn accumulator(&mut self) {
        /* Nothing to do here */
    }

    ///# Immediate
    /// Immediate addressing allows the programmer to directly specify an 8 bit constant within the instruction. It is indicated by a '#' symbol followed by an numeric expression. For example:
    /// ```
    /// LDA #10         ;Load 10 ($0A) into the accumulator
    /// LDX #LO LABEL   ;Load the LSB of a 16 bit address into X
    /// LDY #HI LABEL   ;Load the MSB of a 16 bit address into Y
    /// ```

    pub fn immediate(&mut self) {
        self.addrabs = self.pc;
        self.pc = self.pc.wrapping_add(1);
    }

    ///# Zero Page
    /// An instruction using zero page addressing mode has only an 8 bit address operand. This limits it to addressing only the first 256 bytes of memory (e.g. $0000 to $00FF) where the most significant byte of the address is always zero. In zero page mode only the least significant byte of the address is held in the instruction making it shorter by one byte (important for space saving) and one less memory fetch during execution (important for speed).
    pub fn zeropage(&mut self) {
        self.addrabs = self.fetch() as u16;

    }

    ///# Zero Page,X
    /// The address to be accessed by an instruction using indexed zero page addressing is calculated by taking the 8 bit zero page address from the instruction and adding the current value of the X register to it. For example if the X register contains $0F and the instruction LDA $80,X is executed then the accumulator will be loaded from $008F (e.g. $80 + $0F => $8F).
    /// 
    /// NB:
    /// 
    /// The address calculation wraps around if the sum of the base address and the register exceed $FF. If we repeat the last example but with $FF in the X register then the accumulator will be loaded from $007F (e.g. $80 + $FF => $7F) and not $017F.
    pub fn zeropagex(&mut self) {
        let byte = self.fetch() as u16;
        let byte = byte + (self.x as u16);
        self.addrabs = byte & 0xFF;
    }

    ///# Zero Page,Y
    /// The address to be accessed by an instruction using indexed zero page addressing is calculated by taking the 8 bit zero page address from the instruction and adding the current value of the Y register to it. This mode can only be used with the LDX and STX instructions.
    pub fn zeropagey(&mut self) {
        let byte = self.fetch() as u16;
        let byte = byte + (self.y as u16);
        self.addrabs = byte & 0xFF;
    }
    ///# Relative
    /// Relative addressing mode is used by branch instructions (e.g. BEQ, BNE, etc.) which contain a signed 8 bit relative offset (e.g. -128 to +127) which is added to program counter if the condition is true. As the program counter itself is incremented during instruction execution by two the effective address range for the target instruction must be with -126 to +129 bytes of the branch.
    pub fn relative(&mut self) {
        let byte = self.fetch() as u16;
        self.relval = if byte & 0x80 != 0{
            0xFF00 | byte
        }
        else{
            byte
        };

    }
    ///# Absolute
    /// Instructions using absolute addressing contain a full 16 bit address to identify the target location.
    /// ```
    /// JMP $1234       ;Jump to location $1234
    /// JSR WIBBLE      ;Call subroutine WIBBLE
    /// ```
    pub fn absolute(&mut self) {
        let lobyte = self.fetch() as u16;
        let hibyte = self.fetch() as u16;
        self.addrabs = (hibyte << 8) | lobyte;
    }

    ///# Absolute,X
    /// The address to be accessed by an instruction using X register indexed absolute addressing is computed by taking the 16 bit address from the instruction and added the contents of the X register. For example if X contains $92 then an STA $2000,X instruction will store the accumulator at $2092 (e.g. $2000 + $92).
    /// ```
    /// STA $3000,X     ;Store accumulator between $3000 and $30FF
    /// ROR CRC,X       ;Rotate right one bit
    /// ```
    pub fn absolutex(&mut self) {
        let lobyte = self.fetch() as u16;
        let hibyte = self.fetch() as u16;
        self.addrabs = (hibyte << 8) | lobyte;
        self.addrabs = self.addrabs.wrapping_add(self.x as u16);
    }
    ///# Absolute,Y
    /// The Y register indexed absolute addressing mode is the same as the previous mode only with the contents of the Y register added to the 16 bit address from the instruction.
    /// ```
    /// AND $4000,Y     ;Perform a logical AND with a byte of memory
    /// STA MEM,Y       ;Store accumulator in memory
    /// ```
    pub fn absolutey(&mut self) {
        let lobyte = self.fetch() as u16;
        let hibyte = self.fetch() as u16;
        self.addrabs = (hibyte << 8) | lobyte;
        self.addrabs = self.addrabs.wrapping_add(self.y as u16);
    }

    ///Indirect
    /// 
    /// JMP is the only 6502 instruction to support indirection. The instruction contains a 16 bit address which identifies the location of the least significant byte of another 16 bit memory address which is the real target of the instruction.
    /// 
    /// For example if location $0120 contains $FC and location $0121 contains $BA then the instruction JMP ($0120) will cause the next instruction execution to occur at $BAFC (e.g. the contents of $0120 and $0121).
    /// ```
    /// JMP ($FFFC)     ;Force a power on reset
    /// JMP (TARGET)    ;Jump via a labelled memory area
    /// ```
    pub fn indirect(&mut self) {
        let lobyte = self.fetch() as u16;
        let hibyte = self.fetch() as u16;
        let address = (hibyte << 8) | lobyte;
    
        if lobyte == 0xFF {
            // Simulate the 6502 bug by wrapping around the page
            let lo_byte = self.cpu_read(address, false) as u16;
            let hi_byte = self.cpu_read(address & 0xFF00, false) as u16; // Wraparound
    
            self.addrabs = (hi_byte << 8) | lo_byte;
        } else {
            let lo_byte = self.cpu_read(address, false) as u16;
            let hi_byte = self.cpu_read(address + 1, false) as u16;
    
            self.addrabs = (hi_byte << 8) | lo_byte;
        }
    }
    
    /// # Indexed Indirect
    /// Indexed indirect addressing is normally used in conjunction with a table of address held on zero page. The address of the table is taken from the instruction and the X register added to it (with zero page wrap around) to give the location of the least significant byte of the target address.
    pub fn idx(&mut self) {
        let address = self.fetch() as u16;
        let address = (address + (self.x as u16)) & 0xFF;
        let lobyte = self.cpu_read(address, false) as u16;
        let address = address + 1;
        let address = address & 0xFF;
        let hibyte = self.cpu_read(address, false) as u16;
        self.addrabs = (hibyte << 8) | lobyte;
    }

    ///# Indirect Indexed
    /// Indirect indexed (also known as post-indexed) addressing takes a single operand which gives the zero page address of the least significant byte of a 16-bit address which is then added to the Y register to give the target address. For example, if the operand is bb, 00bb is xx and 00bb + 1 is yy, then the data can be found at yyxx. An example of this addressing mode is AND ($12),Y.
    pub fn idy(&mut self) {
        let address = self.fetch() as u16;
        let lo_byte = self.cpu_read(address & 0xFF, false) as u16;
        let hi_byte = self.cpu_read((address + 1) & 0xFF, false) as u16;
        let address = (hi_byte << 8) | lo_byte;
        let address = address.wrapping_add(self.y as u16);
        if address & 0xFF00 != hi_byte << 8{
            self.cycles_left = self.cycles_left.wrapping_add(1);
        }
        self.addrabs = address;
    }
}
