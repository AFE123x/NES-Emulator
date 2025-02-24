use std::{error::Error, fs};

use crate::cpu::processor::Cpu;
pub struct Cpubus{
    memory: Vec<u8>,
    cpu: *mut Cpu,
}

impl Cpubus{
    pub fn new(cpu: &mut Cpu) -> Self{
        println!("CPU BUS - INITIALIZED");
        Self{
            memory: vec![0;0xFFFF + 1],
            cpu: cpu,
        }
    }
    pub fn clock(&self) {
        unsafe{
            // println!("{:#x} {:#x} {:#x} {:#x}",&self.memory[0],&self.memory[1],&self.memory[2],&self.memory[3]);
            (*self.cpu).clock();
        }
    }
    
    pub fn load_rom(&mut self,s: &String) -> Result<(),Box<dyn Error>>{
        let rom_bytes = fs::read(s)?;
        let rom_data  = &rom_bytes[0x0010..0x4010];
        self.memory[0x8000..0x8000 + rom_data.len()].copy_from_slice(rom_data);
        self.memory[0xC000..0xC000 + rom_data.len()].copy_from_slice(rom_data);
        Ok(())
    }

    pub fn cpu_read(&self, address: u16, readonly: bool) -> u8{
        // println!("{}",readonly);
        self.memory[address as usize]
    }

    pub fn cpu_write(&mut self, address: u16, byte: u8){
        self.memory[address as usize] = byte;
    }
    
}
