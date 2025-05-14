mod mode;



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
    irqset: bool,
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
            irqset: false,
        }
    }
    
    
    /// Links the CPU to a system bus
    pub fn linkbus(&mut self, bus: &mut Bus) {
        self.bus = Some(bus);
    }
    
    /// Reads a byte from memory via the system bus
    fn cpu_read(&self, address: u16, rdonly: bool) -> u8 {
        unsafe { (*self.bus.unwrap()).cpu_read(address, rdonly) }
    }
    fn cpu_write(&self, address: u16, byte: u8) {
        unsafe {
            (*self.bus.unwrap()).cpu_write(address, byte);
        };
    }
    
    /// Advances the CPU clock cycle, fetching and executing an instruction if needed
    pub fn clock(&mut self) -> u64 {
        // Fetch the next instruction if there are no remaining cycles
        if self.cycles_left == 0 {
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
        (self.cycles_left as u64).wrapping_add(1)
    }
}