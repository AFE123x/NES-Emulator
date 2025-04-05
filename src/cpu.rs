mod mode;


use std::fmt::format;

use crate::bus::Bus;
mod instructions;
bitflags! {
    // Define CPU status flags as a bitfield structure
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
use minifb_fonts::font5x8;

/// Representation of the CPU state
pub struct Cpu {
    flags: Flags,    // Processor status flags
    a: u8,          // Accumulator register
    x: u8,          // X register
    y: u8,          // Y register
    pc: u16,        // Program counter
    sp: u8,         // Stack pointer
    addrabs: u16,   // Absolute memory address
    relval: u16,    // Relative value for branch instructions
    cycles_left: u16, // Remaining cycles for the current instruction
    total_cycles: usize, // Total executed cycles
    bus: Option<*mut Bus>, // Pointer to the system bus
    opcode: u8,      // Current opcode being executed
    oldpc: u16,      // Previous program counter value
    updated_state: bool,
}

impl Cpu {
    /// Constructor to initialize CPU state
    pub fn new() -> Self {
        let mut flags = Flags::empty();
        flags.set(Flags::Unused, true);
        Self {
            flags: flags,
            a: 0,
            x: 0,
            y: 0,
            pc: 0x8000, // Typically the reset vector address
            sp: 0xFD, // Stack starts near the top of memory
            bus: None,
            addrabs: 0,
            relval: 0,
            cycles_left: 0,
            total_cycles: 0,
            opcode: 0,
            oldpc: 0,
            updated_state: true,
        }
    }
    
    /// Returns a string representation of the CPU status flags
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
    
    /// Prints the current CPU state and instruction being executed
    fn print_state(&self) -> String{
            format!("PC: {:#x}\nA: {:#x}, X: {:#x}, Y: {:#x}\nSP: {:#x}, flags: {}",self.pc,self.a,self.x,self.y,self.sp,self.print_status_reg())

    }
    
    /// Links the CPU to a system bus
    pub fn linkbus(&mut self, bus: &mut Bus) {
        self.bus = Some(bus);
    }
    
    /// Reads a byte from memory via the system bus
    fn cpu_read(&self, address: u16, rdonly: bool) -> u8 {
        unsafe { (*self.bus.unwrap()).cpu_read(address, rdonly) }
    }
    pub fn update_cpuwindow(&mut self, buf: &mut Vec<u32>){
        let text = font5x8::new_renderer(128,64,0xFFFFFF);
        text.draw_text(buf, 0, 0, self.print_state().as_str());
    }
    pub fn isUpdated(&mut self) -> bool{
        if self.updated_state{
            self.updated_state = false;
            return true;
        }
        false
    }

    /// Writes a byte to memory via the system bus
    fn cpu_write(&self, address: u16, byte: u8) {
        unsafe {
            (*self.bus.unwrap()).cpu_write(address, byte);
        };
    }
    
    /// Advances the CPU clock cycle, fetching and executing an instruction if needed
    pub fn clock(&mut self) {
        // Fetch the next instruction if there are no remaining cycles
        if self.cycles_left == 0 {
            if self.total_cycles & 0x03FFFF == 0{
                self.updated_state = true;
            }
            self.flags.set(Flags::Unused,true);
            let opcode = self.cpu_read(self.pc,false);
            self.oldpc = self.pc;
            self.opcode = opcode;
            self.pc = self.pc.wrapping_add(1);
            self.handle_opcode(opcode); // Execute instruction
        }
        
        // Decrement cycle count and increment total executed cycles
        self.cycles_left = self.cycles_left.wrapping_sub(1);
        self.total_cycles = self.total_cycles.wrapping_add(1);
    }
}
