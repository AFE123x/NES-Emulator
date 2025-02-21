mod address_modes;
use crate::bus::cpubus::{Cpubus};
use std::{cell::RefCell, rc::Rc};
mod instructions;

/// Represents the CPU with its registers, flags, and connection to the bus.
pub struct Cpu{
    a: u8,  // Accumulator register
    x: u8,  // X index register
    y: u8,  // Y index register
    sp: u8, // Stack pointer
    pc: u16, // Program counter
    flags: u8, // Processor status flags
    cycles_left: u8, // Remaining cycles for the current instruction
    extra_cycles: u8, // Additional cycles due to page crossing, etc.
    total_cycles: usize, // Total cycles executed
    immval: u8, // Immediate value for addressing modes
    abs_addr: u16, // Absolute address
    relval: u16, // Relative address offset
    cpubus: Option<*mut Cpubus>, // Reference to the system bus
}

/// Enum representing Load/Store operations.
pub enum LSOperations{
    LDA,
    LDX,
    LDY,
    STA,
    STX,
    STY,
}

/// Enum representing all possible instructions.
pub enum Instruction{
    LoadStoreInstructions(LSOperations),
}

/// Enum representing the different addressing modes.
pub enum AddressModes{
    Implicit,
    Accumulator,
    Immediate,
    Zeropage,
    ZeropageX,
    ZeropageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndexedIndirect,
    IndirectIndexed,
}

/// Enum representing the processor status flags.
pub enum Flags{
    Negative,
    Overflow,
    Break,
    Decimal,
    Interrupt,
    Zeroflag,
    Carry,
}

impl Cpu{
    /// Creates a new CPU instance with default values.
    pub fn new() -> Self{
        println!("CPU - INITIALIZED!");
        Self {
            a: 0,
            x: 0,
            y: 0,
            sp: 0xFD,
            pc: 0x8000,
            flags: 0,
            cycles_left: 0,
            total_cycles: 0,
            abs_addr: 0,
            relval: 0,
            immval: 0,
            cpubus: None,
            extra_cycles: 0,
        }
    }

    /// Simulates a single clock cycle.
    pub fn clock(&mut self){
        if self.cycles_left == 0{
            let x: u8 = unsafe {
                (*self.cpubus.unwrap()).cpu_read(self.pc,true)
            };
            self.pc = self.pc.wrapping_add(1);
            self.decode(x);
            self.cycles_left = self.cycles_left.wrapping_add(self.extra_cycles);
        }
        self.total_cycles = self.total_cycles.wrapping_sub(1);
    }

    /// linkbus will link the bus to the CPU
    pub fn linkbus(&mut self, bus: &mut Cpubus){
        self.cpubus = Some(bus);
    }

    /// Decodes an opcode and executes the corresponding instruction.
    fn decode(&mut self, opcode: u8){
        match opcode{
            0xA9 => {
                self.execute_instruction(AddressModes::Immediate,Instruction::LoadStoreInstructions(LSOperations::LDA), "LDA {IMM}".to_string(),2)
            },
            0xA5 => {
                self.execute_instruction(AddressModes::Zeropage,Instruction::LoadStoreInstructions(LSOperations::LDA), "LDA {ZP0}".to_string(),3)
            },
            0xB5 => {
                self.execute_instruction(AddressModes::ZeropageX,Instruction::LoadStoreInstructions(LSOperations::LDA), "LDA {ZPX}".to_string(),4)
            }
            0xAD => {
                self.execute_instruction(AddressModes::Absolute,Instruction::LoadStoreInstructions(LSOperations::LDA), "LDA {ABS}".to_string(),4)
            },
            0xBD => {
                self.execute_instruction(AddressModes::AbsoluteX, Instruction::LoadStoreInstructions(LSOperations::LDA), "LDA {ABX}".to_string(), 4);
            },
            0xB9 => {
                self.execute_instruction(AddressModes::AbsoluteY, Instruction::LoadStoreInstructions(LSOperations::LDA), "LDA {ABY}".to_string(),4);
            },
            0xA1 => {
                self.execute_instruction(AddressModes::IndexedIndirect, Instruction::LoadStoreInstructions(LSOperations::LDA), "LDA {IDX}".to_string(),6);
            },
            0xB1 => {
                self.execute_instruction(AddressModes::IndirectIndexed, Instruction::LoadStoreInstructions(LSOperations::LDA), "LDA {IDY}".to_string(),5);
            },
            _ => todo!(),
        };
    }
    
    /// Executes an instruction based on its addressing mode and cycles required.
    fn execute_instruction(
        &mut self, 
        addrmode: AddressModes, 
        instruction: Instruction, 
        nmeumonic: String, 
        cycles: u8){
            self.cycles_left = cycles;
            self.handle_addr_mode(addrmode);
            self.handle_instruction(instruction);
            println!("instruction: {}",nmeumonic);
    }
    
    /// Handles different addressing modes.
    fn handle_addr_mode(&mut self, addrmode: AddressModes){
        match addrmode{
            AddressModes::Implicit => self.implied_addressing(),
            AddressModes::Accumulator => self.accumulator_addressing(),
            AddressModes::Immediate => self.immediate_addressing(),
            AddressModes::Zeropage => self.zeropage_addressing(),
            AddressModes::ZeropageX => self.zeropagex_addressing(),
            AddressModes::ZeropageY => self.zeropagey_addressing(),
            AddressModes::Relative => self.relative_addressing(),
            AddressModes::Absolute => self.absolute_addressing(),
            AddressModes::AbsoluteX => self.absolutex_addressing(),
            AddressModes::AbsoluteY => self.absolutey_addressing(),
            AddressModes::Indirect => self.indirect_addressing(),
            AddressModes::IndexedIndirect => self.indexedindirect_addressing(),
            AddressModes::IndirectIndexed => self.indirect_indexed(),
        }
    }
    fn handle_instruction(&mut self, instruction: Instruction){
        match instruction{
            Instruction::LoadStoreInstructions(lsoperations) => {
                match lsoperations{
                    LSOperations::LDA => {
                        self.LDA();
                    }
                    LSOperations::LDX => todo!(),
                    LSOperations::LDY => todo!(),
                    LSOperations::STA => todo!(),
                    LSOperations::STX => todo!(),
                    LSOperations::STY => todo!(),
                }
            },
            _ => todo!(),
        };
    }

    //helper functions solely for testing
    #[cfg(test)]
    pub fn get_accumulator(&self) -> u8{
        self.a
    }

    #[cfg(test)]
    pub fn set_x(&mut self,byte: u8){
        self.x = byte;
    }
    #[cfg(test)]
    pub fn get_flag(&self) -> u8{
        self.flags
    }
}