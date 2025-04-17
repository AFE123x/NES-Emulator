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
    pub fn clock(&mut self) {
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
    }
    pub fn setcpu(&mut self, pc: u16, a: u8, x: u8, y: u8, p: u8, sp: u8){
        self.pc = pc;
        self.a = a;
        self.x = x;
        self.y = y;
        self.flags = Flags::from_bits_truncate(p);
        self.sp = sp;
    }

    pub fn getpc(&self) -> u16{
        self.pc
    }

    pub fn geta(&self) -> u8{
        self.a
    }

 
    pub fn getx(&self) -> u8{
        self.x
    }

    pub fn gety(&self) -> u8{
        self.y
    }

    pub fn getp(&self) -> u8{
        self.flags.bits()
    }
    pub fn getsp(&self) -> u8{
        self.sp
    }
}


#[cfg(test)]
mod tests{
    use serde::Deserialize;
    use std::fs;

    use crate::bus::{self, Bus};

    use super::Cpu;
    
    #[derive(Debug, Deserialize)]
    struct CpuState {
        name: String,
        initial: CpuSnapshot,
        final_: CpuSnapshot,
        cycles: Vec<Cycle>,
    }
    
    #[derive(Debug, Deserialize)]
    struct CpuSnapshot {
        pc: u16,
        s: u8,
        a: u8,
        x: u8,
        y: u8,
        p: u8,
        ram: Vec<(u16, u8)>,
    }
    
    #[derive(Debug, Deserialize)]
    struct Cycle(u16, u8, String); // (address, value, access type)
    
    #[test]
pub fn test00() {
    use std::fs;
    use std::path::Path;
    
    // Get all JSON files in the v1 directory
    let dir_path = "./nes6502/v1/";
    let paths = fs::read_dir(dir_path).expect("Unable to read directory");
    
    for path in paths {
        let path = path.expect("Invalid path").path();
        
        // Skip non-JSON files
        if !path.is_file() || path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        
        println!("Testing file: {:?}", path);
        
        // Read and parse the file
        let file = fs::read_to_string(&path).unwrap_or_else(|_| panic!("Could not read file {:?}", path));
        let parsed: Vec<CpuState> = serde_json::from_str(&file).unwrap_or_else(|_| panic!("Could not parse JSON in file {:?}", path));
        
        let mut Cpu = Cpu::new();
        let mut Bus = Bus::new();
        Cpu.linkbus(&mut Bus);
        
        /* iterate through all states */
        for i in 0..parsed.len() {
            Cpu.reset();
            let cpustate = &parsed[i];
            Cpu.setcpu(
                cpustate.initial.pc,
                cpustate.initial.a,
                cpustate.initial.x,
                cpustate.initial.y,
                cpustate.initial.p,
                cpustate.initial.s
            );
            
            for j in 0..parsed[i].initial.ram.len() {
                Bus.cpu_write(parsed[i].initial.ram[j].0, parsed[i].initial.ram[j].1);
            }
            
            Cpu.cycles_left = 0;
            Cpu.clock();
            
            assert_eq!(
                Cpu.getpc(), parsed[i].final_.pc,
                "test {} in file {:?} failed - PC doesn't match",
                parsed[i].name, path
            );
            assert_eq!(
                Cpu.geta(), parsed[i].final_.a,
                "test {} in file {:?} failed - A doesn't match",
                parsed[i].name, path
            );
            assert_eq!(
                Cpu.getx(), parsed[i].final_.x,
                "test {} in file {:?} failed - X doesn't match",
                parsed[i].name, path
            );
            assert_eq!(
                Cpu.gety(), parsed[i].final_.y,
                "test {} in file {:?} failed - Y doesn't match",
                parsed[i].name, path
            );
            assert_eq!(
                Cpu.getsp(), parsed[i].final_.s,
                "test {} in file {:?} failed - SP doesn't match",
                parsed[i].name, path
            );
            assert_eq!(
                Cpu.getp(), parsed[i].final_.p,
                "test {} in file {:?} flag failed - expected: {:08b}, actual: {:08b}",
                parsed[i].name, path, parsed[i].final_.p, Cpu.getp()
            );
            
            for x in 0..parsed[i].final_.ram.len() {
                let val = Bus.cpu_read(parsed[i].final_.ram[x].0, false);
                assert_eq!(
                    val, parsed[i].final_.ram[x].1,
                    "test {} in file {:?} failed, value at address {:4x} doesn't match",
                    parsed[i].name, path, parsed[i].final_.ram[x].0
                );
            }
        }
    }
}
    
}