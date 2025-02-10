pub struct Cpu{
    a: u8,
    x: u8,
    y: u8,
    sp: u8,
    pc: u16,
    flags: StatusFlag,
    cycles_left: u8,
    total_cycles:usize,
    immval: u8,
    abs_addr: u16,
    relval: u8,
    opcode_table: Vec<InstructionsT>,
}

pub struct StatusFlag{
    cf: bool, //carry flag
    zf: bool, // zero flag
    i: bool, //interrupt disable
    b: bool, //break flag
    v: bool, //overflow flag
    s: bool, //sign flag
}

pub enum LSOperations{
    LDA,
    LDX,
    LDY,
    STA,
    STX,
    STY,

}

pub enum Instruction{
    LoadStoreInstructions(LSOperations),
}

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

pub struct InstructionsT{
    opcode: u8,
    addrmode: AddressModes,
    instruction: Instruction,
    nmeumonic: String,
}
/*
pub trait Clone: Sized {
    // Required method
    fn clone(&self) -> Self;

    // Provided method
    fn clone_from(&mut self, source: &Self) { ... }
}
*/
impl Clone for InstructionsT{
    fn clone(&self) -> Self{
       Self{
            opcode: self.opcode,
            addrmode: self.addrmode,
            instruction: self.instruction,
            nmeumonic: self.nmeumonic,
        }
    }
    fn clone_from(&mut self, source: &Self){
    }
}


impl Cpu{
    pub fn new() -> Self{
        println!("CPU - INITIALIZED!");
        Self {
            a: 0,
            x: 0,
            y: 0,
            sp: 0,
            pc: 0x8000,
            flags: StatusFlag{
                cf: false,
                zf: false,
                i: false,
                b: false,
                v: false,
                s: false,
            },
            cycles_left: 0,
            total_cycles: 0,
            abs_addr: 0,
            relval: 0,
            immval: 0,
            opcode_table: vec![InstructionsT; 0x100],
        }
    }
    pub fn clock(&mut self){
        if self.total_cycles == 0{
        
        }
        self.total_cycles -= 1;
    }
}
