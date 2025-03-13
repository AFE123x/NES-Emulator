mod mode;
use std::{io::{self, stdin}, thread, time};

use crate::bus::Bus;
mod instructions;
bitflags! {
    pub struct Flags: u8 {
        const Negative = 0b10000000;
        const Overflow = 0b01000000;
        const Break = 0b00010000;
        const Decimal = 0b0000_1000;
        const IDisable = 0b00000100;
        const Zero = 0b00000010;
        const Carry = 0b00000001;
    }
}
use bitflags::bitflags;
use instructions::inst_enum::{AddressMode, Instruction};

pub struct Cpu {
    flags: Flags,
    a: u8,
    x: u8,
    y: u8,
    pc: u16,
    sp: u8,
    immval: u8,
    addrabs: u16,
    relval: u16,
    cycles_left: u8,
    total_cycles: usize,
    bus: Option<*mut Bus>,
    opcode: u8,
    oldpc: u16,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            flags: Flags::empty(),
            a: 0,
            x: 0,
            y: 0,
            pc: 0x8000,
            sp: 0xFD,
            immval: 0,
            bus: None,
            addrabs: 0,
            relval: 0,
            cycles_left: 0,
            total_cycles: 0,
            opcode: 0,
            oldpc: 0,
        }
    }

    fn print_state(&self, instruction: &Instruction, addr_mode: &AddressMode){
        println!("PC: {:#x}\tA: {:#x}\tX {:#x}\tY {:#x}\tSP {:#x}\tFLAGS {:08b}\t{:?}({:?})\tADDR: {:4x}\tIMMVAL: {:2x}\tOPCODE: {:2x}\tRELVAL: {:4x}",self.oldpc,self.a,self.x,self.y,self.sp,self.flags.bits(),instruction,addr_mode,self.addrabs,self.immval,self.opcode,self.relval);
    }
    pub fn linkbus(&mut self, bus: &mut Bus) {
        self.bus = Some(bus);
    }
    fn cpu_read(&self, address: u16) -> u8 {
        unsafe { (*self.bus.unwrap()).cpu_read(address,false) }
    }
    fn cpu_write(&self, address: u16, byte: u8) {
        // println!("writing to address {:#x}: {:#x} ",address,byte);
        unsafe {
            (*self.bus.unwrap()).cpu_write(address, byte);
        };
    }
    pub fn clock(&mut self) {
        /* fetch our instruction */
        if self.cycles_left == 0 {
            let opcode = self.cpu_read(self.pc);
            self.oldpc = self.pc;
            self.opcode = opcode;
            self.pc = self.pc.wrapping_add(1);
            self.handle_opcode(opcode);
            let mut str = String::new();
            // io::stdin().read_line(&mut str);
        }
        self.cycles_left = self.cycles_left.wrapping_sub(1);
        self.total_cycles = self.total_cycles.wrapping_add(1);
    }
}

#[cfg(test)]
impl Cpu {
    pub fn get_a(&self) -> u8 {
        self.a
    }

    pub fn get_x(&self) -> u8 {
        self.x
    }

    pub fn get_y(&self) -> u8 {
        self.y
    }

    pub fn get_flag(&self) -> u8 {
        self.flags.bits()
    }

    pub fn set_a(&mut self, val: u8) {
        self.a = val;
    }

    pub fn set_x(&mut self, val: u8) {
        self.x = val;
    }

    pub fn set_y(&mut self, val: u8) {
        self.y = val;
    }
    pub fn set_immval(&mut self, val: u8) {
        self.immval = val;
    }

    pub fn set_flags(&mut self, val: u8) {
        self.flags = Flags::from_bits_truncate(val);
    }
}
