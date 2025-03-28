mod mode;

use std::{thread, time::Duration};

use crate::bus::Bus;
mod instructions;
bitflags! {
    pub struct Flags: u8 {
        const Negative = 0b10000000;
        const Overflow = 0b01000000;
        const Break = 0b00010000;
        const Decimal = 0b0000_1000;
        const Unused = 0b0010_0000;
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
    addrabs: u16,
    relval: u16,
    cycles_left: u16,
    total_cycles: usize,
    bus: Option<*mut Bus>,
    opcode: u8,
    oldpc: u16,
}

impl Cpu {
    

    pub fn new() -> Self {
        let mut flags = Flags::empty();
        flags.set(Flags::Unused, true);
        Self {
            flags: flags,
            a: 0,
            x: 0,
            y: 0,
            pc: 0x8000,
            sp: 0xFD,
            bus: None,
            addrabs: 0,
            relval: 0,
            cycles_left: 0,
            total_cycles: 0,
            opcode: 0,
            oldpc: 0,
        }
    }
    fn print_status_reg(&self) -> String{
        let a = if self.flags.contains(Flags::Negative) {'N'} else {'-'};
        let b = if self.flags.contains(Flags::Overflow) {'V'} else {'-'};
        let c = if self.flags.contains(Flags::Unused) {'U'} else {'-'};
        let d = if self.flags.contains(Flags::Break) {'B'} else {'-'};
        let e = if self.flags.contains(Flags::Decimal) {'D'} else {'-'};
        let f = if self.flags.contains(Flags::IDisable) {'I'} else {'-'};
        let g = if self.flags.contains(Flags::Zero) {'Z'} else {'-'};
        let h = if self.flags.contains(Flags::Carry) {'C'} else {'-'};
        let string = format!("{}{}{}{}{}{}{}{}",a,b,c,d,e,f,g,h);
        string
    }
    fn print_state(&self, instruction: &Instruction, addr_mode: &AddressMode){
        println!("PC: {:#x}\tA: {:#x}\tX {:#x}\tY {:#x}\tSP {:#x}\tFLAGS {}\t{:?}({:?}), cycles: {}\t",self.oldpc,self.a,self.x,self.y,self.sp,self.print_status_reg(),instruction,addr_mode,self.total_cycles);
    }
    pub fn linkbus(&mut self, bus: &mut Bus) {
        self.bus = Some(bus);
    }
    fn cpu_read(&self, address: u16, rdonly: bool) -> u8 {
        unsafe { (*self.bus.unwrap()).cpu_read(address,rdonly) }
    }
    fn cpu_write(&self, address: u16, byte: u8) {
        unsafe {
            (*self.bus.unwrap()).cpu_write(address, byte);
        };
    }
    pub fn clock(&mut self) {
        /* fetch our instruction */
        if self.cycles_left == 0 {
            self.flags.set(Flags::Unused,true);
            let opcode = self.cpu_read(self.pc,false);
            self.oldpc = self.pc;
            self.opcode = opcode;
            self.pc = self.pc.wrapping_add(1);
            self.handle_opcode(opcode);
        }
        self.cycles_left = self.cycles_left.wrapping_sub(1);
        self.total_cycles = self.total_cycles.wrapping_add(1);
    }
}
