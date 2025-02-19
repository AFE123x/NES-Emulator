mod address_modes;
use crate::bus::cpubus::{Cpubus};
use std::{cell::RefCell, rc::Rc};
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
    cpubus: Option<*mut Cpubus>,
}

pub struct StatusFlag{
    c: bool, //carry flag
    z: bool, // zero flag
    i: bool, //interrupt disable
    d: bool, //decimal flag
    b: bool, //break flag
    v: bool, //overflow flag
    n: bool, //sign flag
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
                c: false,
                z: false,
                i: false,
                d: false,
                b: false,
                v: false,
                n: false,
            },
            cycles_left: 0,
            total_cycles: 0,
            abs_addr: 0,
            relval: 0,
            immval: 0,
            cpubus: None,
        }
    }

    pub fn clock(&mut self){
        if self.total_cycles == 0{
            // unsafe{
            //     (*self.cpubus.unwrap()).cpu_read(0xFF, true);
            // }
            let mut x;
            unsafe {
                x = (*self.cpubus.unwrap()).cpu_read(0xFF,true);
            }
            self.pc = self.pc.wrapping_add(1);
            self.decode(0xA9);
        }
        self.total_cycles -= 1;
    }

    pub fn linkbus(&mut self, bus: &mut Cpubus){
        self.cpubus = Some(bus);
    }

    fn decode(&mut self, opcode: u8){
        match opcode{
            0xA9 => self.execute_instruction(AddressModes::Immediate,Instruction::LoadStoreInstructions(LSOperations::LDA), "LDA".to_string(),2),
            _ => todo!(),
        };
    }
    fn execute_instruction(
        &mut self, 
        addrmode: AddressModes, 
        instruction: Instruction, 
        nmeumonic: String, 
        cycles: u8){
            self.cycles_left = cycles;
            self.handle_addr_mode(addrmode);
            
    }
    fn handle_addr_mode(&mut self, addrmode: AddressModes){
        match addrmode{
            AddressModes::Implicit => self.immediate_addressing(),
            AddressModes::Accumulator => todo!(),
            AddressModes::Immediate => todo!(),
            AddressModes::Zeropage => todo!(),
            AddressModes::ZeropageX => todo!(),
            AddressModes::ZeropageY => todo!(),
            AddressModes::Relative => todo!(),
            AddressModes::Absolute => todo!(),
            AddressModes::AbsoluteX => todo!(),
            AddressModes::AbsoluteY => todo!(),
            AddressModes::Indirect => todo!(),
            AddressModes::IndexedIndirect => todo!(),
            AddressModes::IndirectIndexed => todo!(),
        }
    }

    }

