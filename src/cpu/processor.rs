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
pub enum StackOP{
    TSX,
    TXS,
    PHA,
    PHP,
    PLA,
    PLP,
}

pub enum RegTranOp{
    TAX,
    TAY,
    TXA,
    TYA,
}

pub enum LogicalOp{
    AND,
    EOR,
    ORA,
    BIT,
}
/// Enum representing all possible instructions.
pub enum Instruction{
    LoadStoreInstructions(LSOperations),
    RegisterTransferInstructions(RegTranOp),
    StackOperations(StackOP),
    LogicalOperations(LogicalOp),
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
            0xA2 => {
                self.execute_instruction(AddressModes::Immediate, Instruction::LoadStoreInstructions(LSOperations::LDX), "LDX {IMM}".to_string(), 2);
            },
            0xA6 => {
                self.execute_instruction(AddressModes::Zeropage, Instruction::LoadStoreInstructions(LSOperations::LDX), "LDX {ZP0}".to_string(), 3);
            },
            0xB6 => {
                self.execute_instruction(AddressModes::ZeropageY, Instruction::LoadStoreInstructions(LSOperations::LDX), "LDX {ZPY}".to_string(), 4);
            },
            0xAE => {
                self.execute_instruction(AddressModes::Absolute, Instruction::LoadStoreInstructions(LSOperations::LDX), "LDX {ABS}".to_string(), 4);
            },
            0xBE => {
                self.execute_instruction(AddressModes::AbsoluteY, Instruction::LoadStoreInstructions(LSOperations::LDX), "LDX {ABY}".to_string(), 4);  
            },
            0xA0 => {
                self.execute_instruction(AddressModes::Immediate, Instruction::LoadStoreInstructions(LSOperations::LDY), "LDY {IMM}".to_string(), 2);
            },
            0xA4 => {
                self.execute_instruction(AddressModes::Zeropage, Instruction::LoadStoreInstructions(LSOperations::LDY), "LDY {ZP0}".to_string(), 3);
            },
            0xB4 => {
                self.execute_instruction(AddressModes::ZeropageX, Instruction::LoadStoreInstructions(LSOperations::LDY), "LDY {ZPX}".to_string(), 4);
            },
            0xAC => {
                self.execute_instruction(AddressModes::Absolute, Instruction::LoadStoreInstructions(LSOperations::LDY), "LDY {ABS}".to_string(), 4);
            },
            0xBC => {
                self.execute_instruction(AddressModes::AbsoluteX, Instruction::LoadStoreInstructions(LSOperations::LDY), "LDY {ABX}".to_string(), 4);
            },
            0x85 => {
                self.execute_instruction(AddressModes::Zeropage, Instruction::LoadStoreInstructions(LSOperations::STA), "STA {ZP0}".to_string(), 3);
            },
            0x95 => {
                self.execute_instruction(AddressModes::ZeropageX, Instruction::LoadStoreInstructions(LSOperations::STA), "STA {ZPX}".to_string(), 4);
            },
            0x8d => {
                self.execute_instruction(AddressModes::Absolute, Instruction::LoadStoreInstructions(LSOperations::STA), "STA {ABS}".to_string(), 4);
            },
            0x9d => {
                self.execute_instruction(AddressModes::AbsoluteX, Instruction::LoadStoreInstructions(LSOperations::STA), "STA {ABX}".to_string(), 5);
            },
            0x99 => {
                self.execute_instruction(AddressModes::AbsoluteY, Instruction::LoadStoreInstructions(LSOperations::STA), "STA {ABY}".to_string(), 5);
            },
            0x81 => {
                self.execute_instruction(AddressModes::IndexedIndirect, Instruction::LoadStoreInstructions(LSOperations::STA), "STA {IDX}".to_string(), 6);
            },
            0x91 => {
                self.execute_instruction(AddressModes::IndirectIndexed, Instruction::LoadStoreInstructions(LSOperations::STA), "STA {IDY}".to_string(), 6);
            },
            0x86 => {
                self.execute_instruction(AddressModes::Zeropage, Instruction::LoadStoreInstructions(LSOperations::STX), "STX {ZP0}".to_string(), 3);
            },
            0x96 => {
                self.execute_instruction(AddressModes::ZeropageY, Instruction::LoadStoreInstructions(LSOperations::STX), "STX {ZPY}".to_string(), 4);
            },
            0x8e => {
                self.execute_instruction(AddressModes::Absolute, Instruction::LoadStoreInstructions(LSOperations::STX), "STX {ABS}".to_string(), 4);
            },
            0x84 => {
                self.execute_instruction(AddressModes::Zeropage, Instruction::LoadStoreInstructions(LSOperations::STY), "STY {ZP0}".to_string(), 3);
            },
            0x94 => {
                self.execute_instruction(AddressModes::ZeropageX, Instruction::LoadStoreInstructions(LSOperations::STY), "STY {ZPX}".to_string(), 4);
            },
            0x8c => {
                self.execute_instruction(AddressModes::Absolute, Instruction::LoadStoreInstructions(LSOperations::STY), "STY {ABS}".to_string(), 4);
            },
            0xaa => {
                self.execute_instruction(AddressModes::Implicit, Instruction::RegisterTransferInstructions(RegTranOp::TAX), "TAX {IMP}".to_string(), 2);
            },
            0xa8 => {
                self.execute_instruction(AddressModes::Implicit, Instruction::RegisterTransferInstructions(RegTranOp::TAY), "TAY {IMP}".to_string(), 2);
            },
            0x8a => {
                self.execute_instruction(AddressModes::Implicit, Instruction::RegisterTransferInstructions(RegTranOp::TXA), "TXA {IMP}".to_string(), 2);
            },
            0x98 => {
                self.execute_instruction(AddressModes::Implicit, Instruction::RegisterTransferInstructions(RegTranOp::TYA), "TYA {IMP}".to_string(), 2);
            },
            0xba => {
                self.execute_instruction(AddressModes::Implicit, Instruction::StackOperations(StackOP::TSX), "TSX {IMP}".to_string(), 2);
            },
            0x9a => {
                self.execute_instruction(AddressModes::Implicit, Instruction::StackOperations(StackOP::TXS), "TXS {IMP}".to_string(), 2);
            },
            0x48 => {
                self.execute_instruction(AddressModes::Implicit, Instruction::StackOperations(StackOP::PHA), "PHA {IMP}".to_string(), 3);
            },
            0x8 => {
                self.execute_instruction(AddressModes::Implicit, Instruction::StackOperations(StackOP::PHP), "PHP {IMP}".to_string(), 3);
            },
            0x68 => {
                self.execute_instruction(AddressModes::Implicit, Instruction::StackOperations(StackOP::PLA), "PLA {IMP}".to_string(), 4);
            },
            0x28 => {
                self.execute_instruction(AddressModes::Implicit, Instruction::StackOperations(StackOP::PLP), "PLP {IMP}".to_string(), 4);
            },
            0x29 => {
                self.execute_instruction(AddressModes::Immediate, Instruction::LogicalOperations(LogicalOp::AND), "AND {IMM}".to_string(), 2);
            },
            0x25 => {
                self.execute_instruction(AddressModes::Zeropage, Instruction::LogicalOperations(LogicalOp::AND), "AND {ZP0}".to_string(), 3);
            },
            0x35 => {
                self.execute_instruction(AddressModes::ZeropageX, Instruction::LogicalOperations(LogicalOp::AND), "AND {ZPX}".to_string(), 4);
            },
            0x2d => {
                self.execute_instruction(AddressModes::Absolute, Instruction::LogicalOperations(LogicalOp::AND), "AND {ABS}".to_string(), 4);
            },
            0x3d => {
                self.execute_instruction(AddressModes::AbsoluteX, Instruction::LogicalOperations(LogicalOp::AND), "AND {ABX}".to_string(), 4);
            },
            0x39 => {
                self.execute_instruction(AddressModes::AbsoluteY, Instruction::LogicalOperations(LogicalOp::AND), "AND {ABY}".to_string(), 4);
            },
            0x21 => {
                self.execute_instruction(AddressModes::IndexedIndirect, Instruction::LogicalOperations(LogicalOp::AND), "AND {IDX}".to_string(), 6);
            },
            0x31 => {
                self.execute_instruction(AddressModes::IndirectIndexed, Instruction::LogicalOperations(LogicalOp::AND), "AND {IDY}".to_string(), 5);
            },
            0x49 => {
                self.execute_instruction(AddressModes::Immediate, Instruction::LogicalOperations(LogicalOp::EOR), "EOR {IMM}".to_string(), 2);
            },
            0x45 => {
                self.execute_instruction(AddressModes::Zeropage, Instruction::LogicalOperations(LogicalOp::EOR), "EOR {ZP0}".to_string(), 3);
            },
            0x55 => {
                self.execute_instruction(AddressModes::ZeropageX, Instruction::LogicalOperations(LogicalOp::EOR), "EOR {ZPX}".to_string(), 4);
            },
            0x4d => {
                self.execute_instruction(AddressModes::Absolute, Instruction::LogicalOperations(LogicalOp::EOR), "EOR {ABS}".to_string(), 4);
            },
            0x5d => {
                self.execute_instruction(AddressModes::AbsoluteX, Instruction::LogicalOperations(LogicalOp::EOR), "EOR {ABX}".to_string(), 4);
            },
            0x59 => {
                self.execute_instruction(AddressModes::AbsoluteY, Instruction::LogicalOperations(LogicalOp::EOR), "EOR {ABY}".to_string(), 4);
            },
            0x41 => {
                self.execute_instruction(AddressModes::IndexedIndirect, Instruction::LogicalOperations(LogicalOp::EOR), "EOR {IDX}".to_string(), 6);
            },
            0x51 => {
                self.execute_instruction(AddressModes::IndirectIndexed, Instruction::LogicalOperations(LogicalOp::EOR), "EOR {IDY}".to_string(), 5);
            },
            0x9 => {
                self.execute_instruction(AddressModes::Immediate, Instruction::LogicalOperations(LogicalOp::ORA), "ORA {IMM}".to_string(), 2);
            },
            0x5 => {
                self.execute_instruction(AddressModes::Zeropage, Instruction::LogicalOperations(LogicalOp::ORA), "ORA {ZP0}".to_string(), 3);
            },
            0x15 => {
                self.execute_instruction(AddressModes::ZeropageX, Instruction::LogicalOperations(LogicalOp::ORA), "ORA {ZPX}".to_string(), 4);
            },
            0xd => {
                self.execute_instruction(AddressModes::Absolute, Instruction::LogicalOperations(LogicalOp::ORA), "ORA {ABS}".to_string(), 4);
            },
            0x1d => {
                self.execute_instruction(AddressModes::AbsoluteX, Instruction::LogicalOperations(LogicalOp::ORA), "ORA {ABX}".to_string(), 4);
            },
            0x19 => {
                self.execute_instruction(AddressModes::AbsoluteY, Instruction::LogicalOperations(LogicalOp::ORA), "ORA {ABY}".to_string(), 4);
            },
            0x1 => {
                self.execute_instruction(AddressModes::IndexedIndirect, Instruction::LogicalOperations(LogicalOp::ORA), "ORA {IDX}".to_string(), 6);
            },
            0x11 => {
                self.execute_instruction(AddressModes::IndirectIndexed, Instruction::LogicalOperations(LogicalOp::ORA), "ORA {IDY}".to_string(), 5);
            },
            0x24 => {
                self.execute_instruction(AddressModes::Zeropage, Instruction::LogicalOperations(LogicalOp::BIT), "BIT {ZP0}".to_string(), 3);
            },
            0x2c => {
                self.execute_instruction(AddressModes::Absolute, Instruction::LogicalOperations(LogicalOp::BIT), "BIT {ABS}".to_string(), 4);
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
                                            LSOperations::LDX => {
                                                self.LDX();
                                            },
                                            LSOperations::LDY => {
                                                self.LDY();
                                            },
                                            LSOperations::STA => self.STA(),
                                            LSOperations::STX => self.STX(),
                                            LSOperations::STY => self.STY(),
                                        }
                                    },
            Instruction::RegisterTransferInstructions(reg_tran_op) => {
                                match reg_tran_op{
                                    RegTranOp::TAX => self.TAX(),
                                    RegTranOp::TAY => self.TAY(),
                                    RegTranOp::TYA => self.TYA(),
                                    RegTranOp::TXA => self.TXA(),
                                }
                            },
            Instruction::StackOperations(stack_op) => {
                        match stack_op{
                            StackOP::TSX => self.TSX(),
                            StackOP::TXS => self.TXS(),
                            StackOP::PHA => self.PHA(),
                            StackOP::PHP => self.PHP(),
                            StackOP::PLA => self.PLA(),
                            StackOP::PLP => self.PLP(),
                        }
                    },
            Instruction::LogicalOperations(logical_op) => {
                match logical_op{
                    LogicalOp::AND => self.AND(),
                    LogicalOp::EOR => self.EOR(),
                    LogicalOp::ORA => self.ORA(),
                    LogicalOp::BIT => self.BIT(),
                }
            },
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
    pub fn get_x(&mut self) -> u8{
        self.x
    }
    #[cfg(test)]
    pub fn set_y(&mut self,byte: u8){
        self.y = byte;
    }
    #[cfg(test)]
    pub fn get_y(&mut self) -> u8{
        self.y
    }
    #[cfg(test)]
    pub fn get_flag(&self) -> u8{
        self.flags
    }
    #[cfg(test)]
    pub fn get_a(&self) -> u8{
        self.a
    }
    #[cfg(test)]
    pub fn set_a(&mut self,byte: u8){
        self.a = byte;
    }
    #[cfg(test)]
    pub fn get_sflag(&self) -> u8{
        self.flags
    }
    #[cfg(test)]
    pub fn set_sflag(&mut self,byte: u8){
        self.flags = byte;
    }
}