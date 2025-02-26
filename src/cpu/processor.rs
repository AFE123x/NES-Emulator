mod address_modes;
use crate::{bus::cpubus::Cpubus, cartridge::cartridge::Cartridge};
mod instructions;

/// Represents the CPU with its registers, flags, and connection to the bus.
pub struct Cpu {
    a: u8,                       // Accumulator register
    x: u8,                       // X index register
    y: u8,                       // Y index register
    sp: u8,                      // Stack pointer
    pc: u16,                     // Program counter
    flags: u8,                   // Processor status flags
    cycles_left: u8,             // Remaining cycles for the current instruction
    extra_cycles: u8,            // Additional cycles due to page crossing, etc.
    total_cycles: usize,         // Total cycles executed
    immval: u8,                  // Immediate value for addressing modes
    abs_addr: u16,               // Absolute address
    relval: u16,                 // Relative address offset
    cpubus: Option<*mut Cpubus>, // Reference to the system bus
    current_opcode: u8,
}

/// Enum representing Load/Store operations.
enum LSOperations {
    LDA,
    LDX,
    LDY,
    STA,
    STX,
    STY,
}
enum StackOP {
    TSX,
    TXS,
    PHA,
    PHP,
    PLA,
    PLP,
}

enum RegTranOp {
    TAX,
    TAY,
    TXA,
    TYA,
}

enum LogicalOp {
    AND,
    EOR,
    ORA,
    BIT,
}
enum ArithOp {
    ADC,
    SBC,
    CMP,
    CPX,
    CPY,
}

enum IncDecOps {
    INC,
    INX,
    INY,
    DEC,
    DEX,
    DEY,
}

enum ShiftOps {
    ASL,
    LSR,
    ROL,
    ROR,
}

enum JMPCOps {
    JMP,
    JSR,
    RTS,
}

enum BranchOp {
    BCC,
    BCS,
    BEQ,
    BMI,
    BNE,
    BPL,
    BVC,
    BVS,
}

enum StatChangOp {
    CLC,
    CLD,
    CLI,
    CLV,
    SEC,
    SED,
    SEI,
}

enum SystemOp {
    BRK,
    NOP,
    RTI,
}

enum IllegalOp {
    SRE,
}
/// Enum representing all possible instructions.
enum Instruction {
    LoadStoreInstructions(LSOperations),
    RegisterTransferInstructions(RegTranOp),
    StackOperations(StackOP),
    LogicalOperations(LogicalOp),
    ArithmeticOperation(ArithOp),
    IncrementDecrementOperations(IncDecOps),
    ShiftOperations(ShiftOps),
    JumpCallsOperations(JMPCOps),
    BranchOperations(BranchOp),
    StatusChangeOperations(StatChangOp),
    SystemOperations(SystemOp),
    IllegalOperations(IllegalOp),
}
/// Enum representing the different addressing modes.
pub enum AddressModes {
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
pub enum Flags {
    Negative,
    Overflow,
    Break,
    Decimal,
    Interrupt,
    Zeroflag,
    Carry,
}

impl Cpu {
    /// Creates a new CPU instance with default values.
    pub fn new() -> Self {
        println!("CPU - INITIALIZED!");
        Self {
            a: 0,
            x: 0,
            y: 0,
            sp: 0xFD,
            pc: 0xc000,
            flags: 0,
            cycles_left: 0,
            total_cycles: 0,
            abs_addr: 0,
            relval: 0,
            immval: 0,
            cpubus: None,
            extra_cycles: 0,
            current_opcode: 0,
        }
    }

    /// Simulates a single clock cycle.
    pub fn clock(&mut self) {
        if self.cycles_left == 0 {
            let x: u8 = unsafe { (*self.cpubus.unwrap()).cpu_read(self.pc, true) };
            self.pc = self.pc.wrapping_add(1);
            self.current_opcode = x;
            self.decode(x);
            self.cycles_left = self.cycles_left.wrapping_add(self.extra_cycles);
            println!("A:{:#x} X:{:#x} Y:{:#x} SP:{:#x} PC: {:#x} CYC:{}",self.a,self.x,self.y,self.sp,self.pc,self.cycles_left);
        }
        self.total_cycles = self.total_cycles.wrapping_add(1);
        self.cycles_left = self.cycles_left.wrapping_sub(1);
    }

    /// linkbus will link the bus to the CPU
    pub fn linkbus(&mut self, bus: &mut Cpubus) {
        self.cpubus = Some(bus);
    }



    /// Decodes an opcode and executes the corresponding instruction.
    fn decode(&mut self, opcode: u8) {
        match opcode {
            0xA9 => self.execute_instruction(
                AddressModes::Immediate,
                Instruction::LoadStoreInstructions(LSOperations::LDA),
                "LDA {IMM}".to_string(),
                2,
            ),
            0xA5 => self.execute_instruction(
                AddressModes::Zeropage,
                Instruction::LoadStoreInstructions(LSOperations::LDA),
                "LDA {ZP0}".to_string(),
                3,
            ),
            0xB5 => self.execute_instruction(
                AddressModes::ZeropageX,
                Instruction::LoadStoreInstructions(LSOperations::LDA),
                "LDA {ZPX}".to_string(),
                4,
            ),
            0xAD => self.execute_instruction(
                AddressModes::Absolute,
                Instruction::LoadStoreInstructions(LSOperations::LDA),
                "LDA {ABS}".to_string(),
                4,
            ),
            0xBD => {
                self.execute_instruction(
                    AddressModes::AbsoluteX,
                    Instruction::LoadStoreInstructions(LSOperations::LDA),
                    "LDA {ABX}".to_string(),
                    4,
                );
            }
            0xB9 => {
                self.execute_instruction(
                    AddressModes::AbsoluteY,
                    Instruction::LoadStoreInstructions(LSOperations::LDA),
                    "LDA {ABY}".to_string(),
                    4,
                );
            }
            0xA1 => {
                self.execute_instruction(
                    AddressModes::IndexedIndirect,
                    Instruction::LoadStoreInstructions(LSOperations::LDA),
                    "LDA {IDX}".to_string(),
                    6,
                );
            }
            0xB1 => {
                self.execute_instruction(
                    AddressModes::IndirectIndexed,
                    Instruction::LoadStoreInstructions(LSOperations::LDA),
                    "LDA {IDY}".to_string(),
                    5,
                );
            }
            0xA2 => {
                self.execute_instruction(
                    AddressModes::Immediate,
                    Instruction::LoadStoreInstructions(LSOperations::LDX),
                    "LDX {IMM}".to_string(),
                    2,
                );
            }
            0xA6 => {
                self.execute_instruction(
                    AddressModes::Zeropage,
                    Instruction::LoadStoreInstructions(LSOperations::LDX),
                    "LDX {ZP0}".to_string(),
                    3,
                );
            }
            0xB6 => {
                self.execute_instruction(
                    AddressModes::ZeropageY,
                    Instruction::LoadStoreInstructions(LSOperations::LDX),
                    "LDX {ZPY}".to_string(),
                    4,
                );
            }
            0xAE => {
                self.execute_instruction(
                    AddressModes::Absolute,
                    Instruction::LoadStoreInstructions(LSOperations::LDX),
                    "LDX {ABS}".to_string(),
                    4,
                );
            }
            0xBE => {
                self.execute_instruction(
                    AddressModes::AbsoluteY,
                    Instruction::LoadStoreInstructions(LSOperations::LDX),
                    "LDX {ABY}".to_string(),
                    4,
                );
            }
            0xA0 => {
                self.execute_instruction(
                    AddressModes::Immediate,
                    Instruction::LoadStoreInstructions(LSOperations::LDY),
                    "LDY {IMM}".to_string(),
                    2,
                );
            }
            0xA4 => {
                self.execute_instruction(
                    AddressModes::Zeropage,
                    Instruction::LoadStoreInstructions(LSOperations::LDY),
                    "LDY {ZP0}".to_string(),
                    3,
                );
            }
            0xB4 => {
                self.execute_instruction(
                    AddressModes::ZeropageX,
                    Instruction::LoadStoreInstructions(LSOperations::LDY),
                    "LDY {ZPX}".to_string(),
                    4,
                );
            }
            0xAC => {
                self.execute_instruction(
                    AddressModes::Absolute,
                    Instruction::LoadStoreInstructions(LSOperations::LDY),
                    "LDY {ABS}".to_string(),
                    4,
                );
            }
            0xBC => {
                self.execute_instruction(
                    AddressModes::AbsoluteX,
                    Instruction::LoadStoreInstructions(LSOperations::LDY),
                    "LDY {ABX}".to_string(),
                    4,
                );
            }
            0x85 => {
                self.execute_instruction(
                    AddressModes::Zeropage,
                    Instruction::LoadStoreInstructions(LSOperations::STA),
                    "STA {ZP0}".to_string(),
                    3,
                );
            }
            0x95 => {
                self.execute_instruction(
                    AddressModes::ZeropageX,
                    Instruction::LoadStoreInstructions(LSOperations::STA),
                    "STA {ZPX}".to_string(),
                    4,
                );
            }
            0x8d => {
                self.execute_instruction(
                    AddressModes::Absolute,
                    Instruction::LoadStoreInstructions(LSOperations::STA),
                    "STA {ABS}".to_string(),
                    4,
                );
            }
            0x9d => {
                self.execute_instruction(
                    AddressModes::AbsoluteX,
                    Instruction::LoadStoreInstructions(LSOperations::STA),
                    "STA {ABX}".to_string(),
                    5,
                );
            }
            0x99 => {
                self.execute_instruction(
                    AddressModes::AbsoluteY,
                    Instruction::LoadStoreInstructions(LSOperations::STA),
                    "STA {ABY}".to_string(),
                    5,
                );
            }
            0x81 => {
                self.execute_instruction(
                    AddressModes::IndexedIndirect,
                    Instruction::LoadStoreInstructions(LSOperations::STA),
                    "STA {IDX}".to_string(),
                    6,
                );
            }
            0x91 => {
                self.execute_instruction(
                    AddressModes::IndirectIndexed,
                    Instruction::LoadStoreInstructions(LSOperations::STA),
                    "STA {IDY}".to_string(),
                    6,
                );
            }
            0x86 => {
                self.execute_instruction(
                    AddressModes::Zeropage,
                    Instruction::LoadStoreInstructions(LSOperations::STX),
                    "STX {ZP0}".to_string(),
                    3,
                );
            }
            0x96 => {
                self.execute_instruction(
                    AddressModes::ZeropageY,
                    Instruction::LoadStoreInstructions(LSOperations::STX),
                    "STX {ZPY}".to_string(),
                    4,
                );
            }
            0x8e => {
                self.execute_instruction(
                    AddressModes::Absolute,
                    Instruction::LoadStoreInstructions(LSOperations::STX),
                    "STX {ABS}".to_string(),
                    4,
                );
            }
            0x84 => {
                self.execute_instruction(
                    AddressModes::Zeropage,
                    Instruction::LoadStoreInstructions(LSOperations::STY),
                    "STY {ZP0}".to_string(),
                    3,
                );
            }
            0x94 => {
                self.execute_instruction(
                    AddressModes::ZeropageX,
                    Instruction::LoadStoreInstructions(LSOperations::STY),
                    "STY {ZPX}".to_string(),
                    4,
                );
            }
            0x8c => {
                self.execute_instruction(
                    AddressModes::Absolute,
                    Instruction::LoadStoreInstructions(LSOperations::STY),
                    "STY {ABS}".to_string(),
                    4,
                );
            }
            0xaa => {
                self.execute_instruction(
                    AddressModes::Implicit,
                    Instruction::RegisterTransferInstructions(RegTranOp::TAX),
                    "TAX {IMP}".to_string(),
                    2,
                );
            }
            0xa8 => {
                self.execute_instruction(
                    AddressModes::Implicit,
                    Instruction::RegisterTransferInstructions(RegTranOp::TAY),
                    "TAY {IMP}".to_string(),
                    2,
                );
            }
            0x8a => {
                self.execute_instruction(
                    AddressModes::Implicit,
                    Instruction::RegisterTransferInstructions(RegTranOp::TXA),
                    "TXA {IMP}".to_string(),
                    2,
                );
            }
            0x98 => {
                self.execute_instruction(
                    AddressModes::Implicit,
                    Instruction::RegisterTransferInstructions(RegTranOp::TYA),
                    "TYA {IMP}".to_string(),
                    2,
                );
            }
            0xba => {
                self.execute_instruction(
                    AddressModes::Implicit,
                    Instruction::StackOperations(StackOP::TSX),
                    "TSX {IMP}".to_string(),
                    2,
                );
            }
            0x9a => {
                self.execute_instruction(
                    AddressModes::Implicit,
                    Instruction::StackOperations(StackOP::TXS),
                    "TXS {IMP}".to_string(),
                    2,
                );
            }
            0x48 => {
                self.execute_instruction(
                    AddressModes::Implicit,
                    Instruction::StackOperations(StackOP::PHA),
                    "PHA {IMP}".to_string(),
                    3,
                );
            }
            0x8 => {
                self.execute_instruction(
                    AddressModes::Implicit,
                    Instruction::StackOperations(StackOP::PHP),
                    "PHP {IMP}".to_string(),
                    3,
                );
            }
            0x68 => {
                self.execute_instruction(
                    AddressModes::Implicit,
                    Instruction::StackOperations(StackOP::PLA),
                    "PLA {IMP}".to_string(),
                    4,
                );
            }
            0x28 => {
                self.execute_instruction(
                    AddressModes::Implicit,
                    Instruction::StackOperations(StackOP::PLP),
                    "PLP {IMP}".to_string(),
                    4,
                );
            }
            0x29 => {
                self.execute_instruction(
                    AddressModes::Immediate,
                    Instruction::LogicalOperations(LogicalOp::AND),
                    "AND {IMM}".to_string(),
                    2,
                );
            }
            0x25 => {
                self.execute_instruction(
                    AddressModes::Zeropage,
                    Instruction::LogicalOperations(LogicalOp::AND),
                    "AND {ZP0}".to_string(),
                    3,
                );
            }
            0x35 => {
                self.execute_instruction(
                    AddressModes::ZeropageX,
                    Instruction::LogicalOperations(LogicalOp::AND),
                    "AND {ZPX}".to_string(),
                    4,
                );
            }
            0x2d => {
                self.execute_instruction(
                    AddressModes::Absolute,
                    Instruction::LogicalOperations(LogicalOp::AND),
                    "AND {ABS}".to_string(),
                    4,
                );
            }
            0x3d => {
                self.execute_instruction(
                    AddressModes::AbsoluteX,
                    Instruction::LogicalOperations(LogicalOp::AND),
                    "AND {ABX}".to_string(),
                    4,
                );
            }
            0x39 => {
                self.execute_instruction(
                    AddressModes::AbsoluteY,
                    Instruction::LogicalOperations(LogicalOp::AND),
                    "AND {ABY}".to_string(),
                    4,
                );
            }
            0x21 => {
                self.execute_instruction(
                    AddressModes::IndexedIndirect,
                    Instruction::LogicalOperations(LogicalOp::AND),
                    "AND {IDX}".to_string(),
                    6,
                );
            }
            0x31 => {
                self.execute_instruction(
                    AddressModes::IndirectIndexed,
                    Instruction::LogicalOperations(LogicalOp::AND),
                    "AND {IDY}".to_string(),
                    5,
                );
            }
            0x49 => {
                self.execute_instruction(
                    AddressModes::Immediate,
                    Instruction::LogicalOperations(LogicalOp::EOR),
                    "EOR {IMM}".to_string(),
                    2,
                );
            }
            0x45 => {
                self.execute_instruction(
                    AddressModes::Zeropage,
                    Instruction::LogicalOperations(LogicalOp::EOR),
                    "EOR {ZP0}".to_string(),
                    3,
                );
            }
            0x55 => {
                self.execute_instruction(
                    AddressModes::ZeropageX,
                    Instruction::LogicalOperations(LogicalOp::EOR),
                    "EOR {ZPX}".to_string(),
                    4,
                );
            }
            0x4d => {
                self.execute_instruction(
                    AddressModes::Absolute,
                    Instruction::LogicalOperations(LogicalOp::EOR),
                    "EOR {ABS}".to_string(),
                    4,
                );
            }
            0x5d => {
                self.execute_instruction(
                    AddressModes::AbsoluteX,
                    Instruction::LogicalOperations(LogicalOp::EOR),
                    "EOR {ABX}".to_string(),
                    4,
                );
            }
            0x59 => {
                self.execute_instruction(
                    AddressModes::AbsoluteY,
                    Instruction::LogicalOperations(LogicalOp::EOR),
                    "EOR {ABY}".to_string(),
                    4,
                );
            }
            0x41 => {
                self.execute_instruction(
                    AddressModes::IndexedIndirect,
                    Instruction::LogicalOperations(LogicalOp::EOR),
                    "EOR {IDX}".to_string(),
                    6,
                );
            }
            0x51 => {
                self.execute_instruction(
                    AddressModes::IndirectIndexed,
                    Instruction::LogicalOperations(LogicalOp::EOR),
                    "EOR {IDY}".to_string(),
                    5,
                );
            }
            0x9 => {
                self.execute_instruction(
                    AddressModes::Immediate,
                    Instruction::LogicalOperations(LogicalOp::ORA),
                    "ORA {IMM}".to_string(),
                    2,
                );
            }
            0x5 => {
                self.execute_instruction(
                    AddressModes::Zeropage,
                    Instruction::LogicalOperations(LogicalOp::ORA),
                    "ORA {ZP0}".to_string(),
                    3,
                );
            }
            0x15 => {
                self.execute_instruction(
                    AddressModes::ZeropageX,
                    Instruction::LogicalOperations(LogicalOp::ORA),
                    "ORA {ZPX}".to_string(),
                    4,
                );
            }
            0xd => {
                self.execute_instruction(
                    AddressModes::Absolute,
                    Instruction::LogicalOperations(LogicalOp::ORA),
                    "ORA {ABS}".to_string(),
                    4,
                );
            }
            0x1d => {
                self.execute_instruction(
                    AddressModes::AbsoluteX,
                    Instruction::LogicalOperations(LogicalOp::ORA),
                    "ORA {ABX}".to_string(),
                    4,
                );
            }
            0x19 => {
                self.execute_instruction(
                    AddressModes::AbsoluteY,
                    Instruction::LogicalOperations(LogicalOp::ORA),
                    "ORA {ABY}".to_string(),
                    4,
                );
            }
            0x1 => {
                self.execute_instruction(
                    AddressModes::IndexedIndirect,
                    Instruction::LogicalOperations(LogicalOp::ORA),
                    "ORA {IDX}".to_string(),
                    6,
                );
            }
            0x11 => {
                self.execute_instruction(
                    AddressModes::IndirectIndexed,
                    Instruction::LogicalOperations(LogicalOp::ORA),
                    "ORA {IDY}".to_string(),
                    5,
                );
            }
            0x24 => {
                self.execute_instruction(
                    AddressModes::Zeropage,
                    Instruction::LogicalOperations(LogicalOp::BIT),
                    "BIT {ZP0}".to_string(),
                    3,
                );
            }
            0x2c => {
                self.execute_instruction(
                    AddressModes::Absolute,
                    Instruction::LogicalOperations(LogicalOp::BIT),
                    "BIT {ABS}".to_string(),
                    4,
                );
            }
            0x69 => {
                self.execute_instruction(
                    AddressModes::Immediate,
                    Instruction::ArithmeticOperation(ArithOp::ADC),
                    "ADC {IMM}".to_string(),
                    2,
                );
            }
            0x65 => {
                self.execute_instruction(
                    AddressModes::Zeropage,
                    Instruction::ArithmeticOperation(ArithOp::ADC),
                    "ADC {ZP0}".to_string(),
                    3,
                );
            }
            0x75 => {
                self.execute_instruction(
                    AddressModes::ZeropageX,
                    Instruction::ArithmeticOperation(ArithOp::ADC),
                    "ADC {ZPX}".to_string(),
                    4,
                );
            }
            0x6d => {
                self.execute_instruction(
                    AddressModes::Absolute,
                    Instruction::ArithmeticOperation(ArithOp::ADC),
                    "ADC {ABS}".to_string(),
                    4,
                );
            }
            0x7d => {
                self.execute_instruction(
                    AddressModes::AbsoluteX,
                    Instruction::ArithmeticOperation(ArithOp::ADC),
                    "ADC {ABX}".to_string(),
                    4,
                );
            }
            0x79 => {
                self.execute_instruction(
                    AddressModes::AbsoluteY,
                    Instruction::ArithmeticOperation(ArithOp::ADC),
                    "ADC {ABY}".to_string(),
                    4,
                );
            }
            0x61 => {
                self.execute_instruction(
                    AddressModes::IndexedIndirect,
                    Instruction::ArithmeticOperation(ArithOp::ADC),
                    "ADC {IDX}".to_string(),
                    6,
                );
            }
            0x71 => {
                self.execute_instruction(
                    AddressModes::IndirectIndexed,
                    Instruction::ArithmeticOperation(ArithOp::ADC),
                    "ADC {IDY}".to_string(),
                    5,
                );
            }
            0xe9 => {
                self.execute_instruction(
                    AddressModes::Immediate,
                    Instruction::ArithmeticOperation(ArithOp::SBC),
                    "SBC {IMM}".to_string(),
                    2,
                );
            }
            0xe5 => {
                self.execute_instruction(
                    AddressModes::Zeropage,
                    Instruction::ArithmeticOperation(ArithOp::SBC),
                    "SBC {ZP0}".to_string(),
                    3,
                );
            }
            0xf5 => {
                self.execute_instruction(
                    AddressModes::ZeropageX,
                    Instruction::ArithmeticOperation(ArithOp::SBC),
                    "SBC {ZPX}".to_string(),
                    4,
                );
            }
            0xed => {
                self.execute_instruction(
                    AddressModes::Absolute,
                    Instruction::ArithmeticOperation(ArithOp::SBC),
                    "SBC {ABS}".to_string(),
                    4,
                );
            }
            0xfd => {
                self.execute_instruction(
                    AddressModes::AbsoluteX,
                    Instruction::ArithmeticOperation(ArithOp::SBC),
                    "SBC {ABX}".to_string(),
                    4,
                );
            }
            0xf9 => {
                self.execute_instruction(
                    AddressModes::AbsoluteY,
                    Instruction::ArithmeticOperation(ArithOp::SBC),
                    "SBC {ABY}".to_string(),
                    4,
                );
            }
            0xe1 => {
                self.execute_instruction(
                    AddressModes::IndexedIndirect,
                    Instruction::ArithmeticOperation(ArithOp::SBC),
                    "SBC {IDX}".to_string(),
                    6,
                );
            }
            0xf1 => {
                self.execute_instruction(
                    AddressModes::IndirectIndexed,
                    Instruction::ArithmeticOperation(ArithOp::SBC),
                    "SBC {IDY}".to_string(),
                    5,
                );
            }
            0xc9 => {
                self.execute_instruction(
                    AddressModes::Immediate,
                    Instruction::ArithmeticOperation(ArithOp::CMP),
                    "CMP {IMM}".to_string(),
                    2,
                );
            }
            0xc5 => {
                self.execute_instruction(
                    AddressModes::Zeropage,
                    Instruction::ArithmeticOperation(ArithOp::CMP),
                    "CMP {ZP0}".to_string(),
                    3,
                );
            }
            0xd5 => {
                self.execute_instruction(
                    AddressModes::ZeropageX,
                    Instruction::ArithmeticOperation(ArithOp::CMP),
                    "CMP {ZPX}".to_string(),
                    4,
                );
            }
            0xcd => {
                self.execute_instruction(
                    AddressModes::Absolute,
                    Instruction::ArithmeticOperation(ArithOp::CMP),
                    "CMP {ABS}".to_string(),
                    4,
                );
            }
            0xdd => {
                self.execute_instruction(
                    AddressModes::AbsoluteX,
                    Instruction::ArithmeticOperation(ArithOp::CMP),
                    "CMP {ABX}".to_string(),
                    4,
                );
            }
            0xd9 => {
                self.execute_instruction(
                    AddressModes::AbsoluteY,
                    Instruction::ArithmeticOperation(ArithOp::CMP),
                    "CMP {ABY}".to_string(),
                    4,
                );
            }
            0xc1 => {
                self.execute_instruction(
                    AddressModes::IndexedIndirect,
                    Instruction::ArithmeticOperation(ArithOp::CMP),
                    "CMP {IDX}".to_string(),
                    6,
                );
            }
            0xd1 => {
                self.execute_instruction(
                    AddressModes::IndirectIndexed,
                    Instruction::ArithmeticOperation(ArithOp::CMP),
                    "CMP {IDY}".to_string(),
                    5,
                );
            }
            0xe0 => {
                self.execute_instruction(
                    AddressModes::Immediate,
                    Instruction::ArithmeticOperation(ArithOp::CPX),
                    "CPX {IMM}".to_string(),
                    2,
                );
            }
            0xe4 => {
                self.execute_instruction(
                    AddressModes::Zeropage,
                    Instruction::ArithmeticOperation(ArithOp::CPX),
                    "CPX {ZP0}".to_string(),
                    3,
                );
            }
            0xec => {
                self.execute_instruction(
                    AddressModes::Absolute,
                    Instruction::ArithmeticOperation(ArithOp::CPX),
                    "CPX {ABS}".to_string(),
                    4,
                );
            }
            0xc0 => {
                self.execute_instruction(
                    AddressModes::Immediate,
                    Instruction::ArithmeticOperation(ArithOp::CPY),
                    "CPY {IMM}".to_string(),
                    2,
                );
            }
            0xc4 => {
                self.execute_instruction(
                    AddressModes::Zeropage,
                    Instruction::ArithmeticOperation(ArithOp::CPY),
                    "CPY {ZP0}".to_string(),
                    3,
                );
            }
            0xcc => {
                self.execute_instruction(
                    AddressModes::Absolute,
                    Instruction::ArithmeticOperation(ArithOp::CPY),
                    "CPY {ABS}".to_string(),
                    4,
                );
            }
            0xe6 => {
                self.execute_instruction(
                    AddressModes::Zeropage,
                    Instruction::IncrementDecrementOperations(IncDecOps::INC),
                    "INC {ZP0}".to_string(),
                    5,
                );
            }
            0xf6 => {
                self.execute_instruction(
                    AddressModes::ZeropageX,
                    Instruction::IncrementDecrementOperations(IncDecOps::INC),
                    "INC {ZPX}".to_string(),
                    6,
                );
            }
            0xee => {
                self.execute_instruction(
                    AddressModes::Absolute,
                    Instruction::IncrementDecrementOperations(IncDecOps::INC),
                    "INC {ABS}".to_string(),
                    6,
                );
            }
            0xfe => {
                self.execute_instruction(
                    AddressModes::AbsoluteX,
                    Instruction::IncrementDecrementOperations(IncDecOps::INC),
                    "INC {ABX}".to_string(),
                    7,
                );
            }
            0xe8 => {
                self.execute_instruction(
                    AddressModes::Implicit,
                    Instruction::IncrementDecrementOperations(IncDecOps::INX),
                    "INX {IMP}".to_string(),
                    2,
                );
            }
            0xc8 => {
                self.execute_instruction(
                    AddressModes::Implicit,
                    Instruction::IncrementDecrementOperations(IncDecOps::INY),
                    "INY {IMP}".to_string(),
                    2,
                );
            }
            0xc6 => {
                self.execute_instruction(
                    AddressModes::Zeropage,
                    Instruction::IncrementDecrementOperations(IncDecOps::DEC),
                    "DEC {ZP0}".to_string(),
                    5,
                );
            }
            0xd6 => {
                self.execute_instruction(
                    AddressModes::ZeropageX,
                    Instruction::IncrementDecrementOperations(IncDecOps::DEC),
                    "DEC {ZPX}".to_string(),
                    6,
                );
            }
            0xce => {
                self.execute_instruction(
                    AddressModes::Absolute,
                    Instruction::IncrementDecrementOperations(IncDecOps::DEC),
                    "DEC {ABS}".to_string(),
                    6,
                );
            }
            0xde => {
                self.execute_instruction(
                    AddressModes::AbsoluteX,
                    Instruction::IncrementDecrementOperations(IncDecOps::DEC),
                    "DEC {ABX}".to_string(),
                    7,
                );
            }
            0xca => {
                self.execute_instruction(
                    AddressModes::Implicit,
                    Instruction::IncrementDecrementOperations(IncDecOps::DEX),
                    "DEX {IMP}".to_string(),
                    2,
                );
            }
            0x88 => {
                self.execute_instruction(
                    AddressModes::Implicit,
                    Instruction::IncrementDecrementOperations(IncDecOps::DEY),
                    "DEY {IMP}".to_string(),
                    2,
                );
            }
            0xa => {
                self.execute_instruction(
                    AddressModes::Accumulator,
                    Instruction::ShiftOperations(ShiftOps::ASL),
                    "ASL {ACC}".to_string(),
                    2,
                );
            }
            0x6 => {
                self.execute_instruction(
                    AddressModes::Zeropage,
                    Instruction::ShiftOperations(ShiftOps::ASL),
                    "ASL {ZP0}".to_string(),
                    5,
                );
            }
            0x16 => {
                self.execute_instruction(
                    AddressModes::ZeropageX,
                    Instruction::ShiftOperations(ShiftOps::ASL),
                    "ASL {ZPX}".to_string(),
                    6,
                );
            }
            0xe => {
                self.execute_instruction(
                    AddressModes::Absolute,
                    Instruction::ShiftOperations(ShiftOps::ASL),
                    "ASL {ABS}".to_string(),
                    6,
                );
            }
            0x1e => {
                self.execute_instruction(
                    AddressModes::AbsoluteX,
                    Instruction::ShiftOperations(ShiftOps::ASL),
                    "ASL {ABX}".to_string(),
                    7,
                );
            }
            0x4a => {
                self.execute_instruction(
                    AddressModes::Accumulator,
                    Instruction::ShiftOperations(ShiftOps::LSR),
                    "LSR {ACC}".to_string(),
                    2,
                );
            }
            0x46 => {
                self.execute_instruction(
                    AddressModes::Zeropage,
                    Instruction::ShiftOperations(ShiftOps::LSR),
                    "LSR {ZP0}".to_string(),
                    5,
                );
            }
            0x56 => {
                self.execute_instruction(
                    AddressModes::ZeropageX,
                    Instruction::ShiftOperations(ShiftOps::LSR),
                    "LSR {ZPX}".to_string(),
                    6,
                );
            }
            0x4e => {
                self.execute_instruction(
                    AddressModes::Absolute,
                    Instruction::ShiftOperations(ShiftOps::LSR),
                    "LSR {ABS}".to_string(),
                    6,
                );
            }
            0x5e => {
                self.execute_instruction(
                    AddressModes::AbsoluteX,
                    Instruction::ShiftOperations(ShiftOps::LSR),
                    "LSR {ABX}".to_string(),
                    7,
                );
            }
            0x2a => {
                self.execute_instruction(
                    AddressModes::Accumulator,
                    Instruction::ShiftOperations(ShiftOps::ROL),
                    "ROL {ACC}".to_string(),
                    2,
                );
            }
            0x26 => {
                self.execute_instruction(
                    AddressModes::Zeropage,
                    Instruction::ShiftOperations(ShiftOps::ROL),
                    "ROL {ZP0}".to_string(),
                    5,
                );
            }
            0x36 => {
                self.execute_instruction(
                    AddressModes::ZeropageX,
                    Instruction::ShiftOperations(ShiftOps::ROL),
                    "ROL {ZPX}".to_string(),
                    6,
                );
            }
            0x2e => {
                self.execute_instruction(
                    AddressModes::Absolute,
                    Instruction::ShiftOperations(ShiftOps::ROL),
                    "ROL {ABS}".to_string(),
                    6,
                );
            }
            0x3e => {
                self.execute_instruction(
                    AddressModes::AbsoluteX,
                    Instruction::ShiftOperations(ShiftOps::ROL),
                    "ROL {ABX}".to_string(),
                    7,
                );
            }
            0x6a => {
                self.execute_instruction(
                    AddressModes::Accumulator,
                    Instruction::ShiftOperations(ShiftOps::ROR),
                    "ROR {ACC}".to_string(),
                    2,
                );
            }
            0x66 => {
                self.execute_instruction(
                    AddressModes::Zeropage,
                    Instruction::ShiftOperations(ShiftOps::ROR),
                    "ROR {ZP0}".to_string(),
                    5,
                );
            }
            0x76 => {
                self.execute_instruction(
                    AddressModes::ZeropageX,
                    Instruction::ShiftOperations(ShiftOps::ROR),
                    "ROR {ZPX}".to_string(),
                    6,
                );
            }
            0x6e => {
                self.execute_instruction(
                    AddressModes::Absolute,
                    Instruction::ShiftOperations(ShiftOps::ROR),
                    "ROR {ABS}".to_string(),
                    6,
                );
            }
            0x7e => {
                self.execute_instruction(
                    AddressModes::AbsoluteX,
                    Instruction::ShiftOperations(ShiftOps::ROR),
                    "ROR {ABX}".to_string(),
                    7,
                );
            }
            0x4c => {
                self.execute_instruction(
                    AddressModes::Absolute,
                    Instruction::JumpCallsOperations(JMPCOps::JMP),
                    "JMP {ABS}".to_string(),
                    3,
                );
            }
            0x6c => {
                self.execute_instruction(
                    AddressModes::Indirect,
                    Instruction::JumpCallsOperations(JMPCOps::JMP),
                    "JMP {IND}".to_string(),
                    5,
                );
            }
            0x20 => {
                self.execute_instruction(
                    AddressModes::Absolute,
                    Instruction::JumpCallsOperations(JMPCOps::JSR),
                    "JSR {ABS}".to_string(),
                    6,
                );
            }
            0x60 => {
                self.execute_instruction(
                    AddressModes::Implicit,
                    Instruction::JumpCallsOperations(JMPCOps::RTS),
                    "RTS {IMP}".to_string(),
                    6,
                );
            }
            0x90 => {
                self.execute_instruction(
                    AddressModes::Relative,
                    Instruction::BranchOperations(BranchOp::BCC),
                    "BCC {REL}".to_string(),
                    2,
                );
            }
            0xb0 => {
                self.execute_instruction(
                    AddressModes::Relative,
                    Instruction::BranchOperations(BranchOp::BCS),
                    "BCS {REL}".to_string(),
                    2,
                );
            }
            0xf0 => {
                self.execute_instruction(
                    AddressModes::Relative,
                    Instruction::BranchOperations(BranchOp::BEQ),
                    "BEQ {REL}".to_string(),
                    2,
                );
            }
            0x30 => {
                self.execute_instruction(
                    AddressModes::Relative,
                    Instruction::BranchOperations(BranchOp::BMI),
                    "BMI {REL}".to_string(),
                    2,
                );
            }
            0xd0 => {
                self.execute_instruction(
                    AddressModes::Relative,
                    Instruction::BranchOperations(BranchOp::BNE),
                    "BNE {REL}".to_string(),
                    2,
                );
            }
            0x10 => {
                self.execute_instruction(
                    AddressModes::Relative,
                    Instruction::BranchOperations(BranchOp::BPL),
                    "BPL {REL}".to_string(),
                    2,
                );
            }
            0x50 => {
                self.execute_instruction(
                    AddressModes::Relative,
                    Instruction::BranchOperations(BranchOp::BVC),
                    "BVC {REL}".to_string(),
                    2,
                );
            }
            0x70 => {
                self.execute_instruction(
                    AddressModes::Relative,
                    Instruction::BranchOperations(BranchOp::BVS),
                    "BVS {REL}".to_string(),
                    2,
                );
            }
            0x18 => {
                self.execute_instruction(
                    AddressModes::Implicit,
                    Instruction::StatusChangeOperations(StatChangOp::CLC),
                    "CLC {IMP}".to_string(),
                    2,
                );
            }
            0xd8 => {
                self.execute_instruction(
                    AddressModes::Implicit,
                    Instruction::StatusChangeOperations(StatChangOp::CLD),
                    "CLD {IMP}".to_string(),
                    2,
                );
            }
            0x58 => {
                self.execute_instruction(
                    AddressModes::Implicit,
                    Instruction::StatusChangeOperations(StatChangOp::CLI),
                    "CLI {IMP}".to_string(),
                    2,
                );
            }
            0xb8 => {
                self.execute_instruction(
                    AddressModes::Implicit,
                    Instruction::StatusChangeOperations(StatChangOp::CLV),
                    "CLV {IMP}".to_string(),
                    2,
                );
            }
            0x38 => {
                self.execute_instruction(
                    AddressModes::Implicit,
                    Instruction::StatusChangeOperations(StatChangOp::SEC),
                    "SEC {IMP}".to_string(),
                    2,
                );
            }
            0xf8 => {
                self.execute_instruction(
                    AddressModes::Implicit,
                    Instruction::StatusChangeOperations(StatChangOp::SED),
                    "SED {IMP}".to_string(),
                    2,
                );
            }
            0x78 => {
                self.execute_instruction(
                    AddressModes::Implicit,
                    Instruction::StatusChangeOperations(StatChangOp::SEI),
                    "SEI {IMP}".to_string(),
                    2,
                );
            }
            0x0 => {
                self.execute_instruction(
                    AddressModes::Implicit,
                    Instruction::SystemOperations(SystemOp::BRK),
                    "BRK {IMP}".to_string(),
                    7,
                );
            }
            0xea => {
                self.execute_instruction(
                    AddressModes::Implicit,
                    Instruction::SystemOperations(SystemOp::NOP),
                    "NOP {IMP}".to_string(),
                    2,
                );
            }
            0x40 => {
                self.execute_instruction(
                    AddressModes::Implicit,
                    Instruction::SystemOperations(SystemOp::RTI),
                    "RTI {IMP}".to_string(),
                    6,
                );
            },
            0x47 => {
                self.execute_instruction(
                    AddressModes::Zeropage, 
                    Instruction::IllegalOperations(IllegalOp::SRE),
                    "SRE {ZP0}".to_string(),
                    5
                );
            }
/*
zeropage	SRE oper	47	2	5  	
zeropage,X	SRE oper,X	57	2	6  	
absolute	SRE oper	4F	3	6  	
absolut,X	SRE oper,X	5F	3	7  	
absolut,Y	SRE oper,Y	5B	3	7  	
(indirect,X)	SRE (oper,X)	43	2	8  	
(indirect),Y	SRE (oper),Y	53	2	8  
*/
            _ => {
                panic!("{:#x} not implemented yet", opcode);
            }
        };
    }

    /// Executes an instruction based on its addressing mode and cycles required.
    fn execute_instruction(
        &mut self,
        addrmode: AddressModes,
        instruction: Instruction,
        nmeumonic: String,
        cycles: u8,
    ) {
        self.cycles_left = cycles;
        self.handle_addr_mode(addrmode);
        self.handle_instruction(instruction);
        print!("instruction: {} ", nmeumonic);
    }

    /// Handles different addressing modes.
    fn handle_addr_mode(&mut self, addrmode: AddressModes) {
        match addrmode {
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
    fn handle_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::LoadStoreInstructions(lsoperations) => match lsoperations {
                LSOperations::LDA => self.lda(),
                LSOperations::LDX => self.ldx(),
                LSOperations::LDY => self.ldy(),
                LSOperations::STA => self.sta(),
                LSOperations::STX => self.stx(),
                LSOperations::STY => self.sty(),
            },
            Instruction::RegisterTransferInstructions(reg_tran_op) => match reg_tran_op {
                RegTranOp::TAX => self.tax(),
                RegTranOp::TAY => self.tay(),
                RegTranOp::TYA => self.tya(),
                RegTranOp::TXA => self.txa(),
            },
            Instruction::StackOperations(stack_op) => match stack_op {
                StackOP::TSX => self.tsx(),
                StackOP::TXS => self.txs(),
                StackOP::PHA => self.pha(),
                StackOP::PHP => self.php(),
                StackOP::PLA => self.pla(),
                StackOP::PLP => self.plp(),
            },
            Instruction::LogicalOperations(logical_op) => match logical_op {
                LogicalOp::AND => self.and(),
                LogicalOp::EOR => self.eor(),
                LogicalOp::ORA => self.ora(),
                LogicalOp::BIT => self.bit(),
            },
            Instruction::ArithmeticOperation(arith_op) => match arith_op {
                ArithOp::ADC => self.adc(),
                ArithOp::SBC => self.sbc(),
                ArithOp::CMP => self.cmp(),
                ArithOp::CPX => self.cpx(),
                ArithOp::CPY => self.cpy(),
            },
            Instruction::IncrementDecrementOperations(inc_dec_ops) => match inc_dec_ops {
                IncDecOps::INC => self.inc(),
                IncDecOps::INX => self.inx(),
                IncDecOps::INY => self.iny(),
                IncDecOps::DEC => self.dec(),
                IncDecOps::DEX => self.dex(),
                IncDecOps::DEY => self.dey(),
            },
            Instruction::ShiftOperations(shift_ops) => match shift_ops {
                ShiftOps::ASL => self.asl(),
                ShiftOps::LSR => self.lsr(),
                ShiftOps::ROL => self.rol(),
                ShiftOps::ROR => self.ror(),
            },
            Instruction::JumpCallsOperations(jmpcops) => match jmpcops {
                JMPCOps::JMP => self.jmp(),
                JMPCOps::JSR => self.jsr(),
                JMPCOps::RTS => self.rts(),
            },
            Instruction::BranchOperations(branch_op) => match branch_op {
                BranchOp::BCC => self.bcc(),
                BranchOp::BCS => self.bcs(),
                BranchOp::BEQ => self.beq(),
                BranchOp::BMI => self.bmi(),
                BranchOp::BNE => self.bne(),
                BranchOp::BPL => self.bpl(),
                BranchOp::BVC => self.bvc(),
                BranchOp::BVS => self.bvs(),
            },
            Instruction::StatusChangeOperations(stat_chang_op) => match stat_chang_op {
                StatChangOp::CLC => self.clc(),
                StatChangOp::CLD => self.cld(),
                StatChangOp::CLI => self.cli(),
                StatChangOp::CLV => self.clv(),
                StatChangOp::SEC => self.sec(),
                StatChangOp::SED => self.sed(),
                StatChangOp::SEI => self.sei(),
            },
            Instruction::SystemOperations(system_op) => match system_op {
                SystemOp::BRK => self.brk(),
                SystemOp::NOP => self.nop(),
                SystemOp::RTI => self.rti(),
            },
            Instruction::IllegalOperations(illegal_op) => match illegal_op {
                IllegalOp::SRE => self.sre(),
            },
        };
    }

    pub fn reset(&mut self){
        let lo: u16 = self.read(0xFFFC) as u16;
        let hi: u16 = self.read(0xFFFD) as u16;
        self.pc = (hi << 8) | lo;
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.sp = 0xfd;
        self.flags = 0;
        self.flags = self.flags | 0x20;

        self.abs_addr = 0;
        self.relval = 0;
        self.cycles_left = 0;
        self.total_cycles = 0;
        self.extra_cycles = 0;
    }

    pub fn nmi(&mut self){
        self.write(0x100 + self.sp as u16, (self.pc >> 8) as u8);
        self.sp = self.sp.wrapping_sub(1);
        self.write(0x100 + self.sp as u16,(self.pc & 0xFF) as u8);
        self.sp = self.sp.wrapping_sub(1);

        self.setflag(Flags::Break, false);
        self.setflag(Flags::Interrupt, true);
        let lo = self.read(0xFFFA) as u16;
        let hi = self.read(0xFFFB) as u16;
        self.pc = (hi << 8) | lo;
        self.cycles_left = 8;
    }

    //helper functions solely for testing
    #[cfg(test)]
    pub fn get_accumulator(&self) -> u8 {
        self.a
    }

    #[cfg(test)]
    pub fn set_x(&mut self, byte: u8) {
        self.x = byte;
    }
    #[cfg(test)]
    pub fn get_x(&mut self) -> u8 {
        self.x
    }
    #[cfg(test)]
    pub fn set_y(&mut self, byte: u8) {
        self.y = byte;
    }
    #[cfg(test)]
    pub fn get_y(&mut self) -> u8 {
        self.y
    }
    #[cfg(test)]
    pub fn get_flag(&self) -> u8 {
        self.flags
    }
    #[cfg(test)]
    pub fn get_a(&self) -> u8 {
        self.a
    }
    #[cfg(test)]
    pub fn set_a(&mut self, byte: u8) {
        self.a = byte;
    }
    #[cfg(test)]
    pub fn get_sflag(&self) -> u8 {
        self.flags
    }
    #[cfg(test)]
    pub fn set_sflag(&mut self, byte: u8) {
        self.flags = byte;
    }
    #[cfg(test)]
    pub fn get_pc(&mut self) -> u16 {
        self.pc
    }
    #[cfg(test)]
    pub fn set_pc(&mut self, address: u16) {
        self.pc = address;
    }

    #[cfg(test)]
    pub fn set_sp(&mut self, byte: u8){
        self.sp = byte;
    }
}
