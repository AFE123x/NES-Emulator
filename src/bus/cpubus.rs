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
        println!("clock");
        unsafe{
            (*self.cpu).clock();
        }
    }

    pub fn cpu_read(&self, address: u16, readonly: bool) -> u8{
        self.memory[address as usize]
    }

    pub fn cpu_write(&mut self, address: u16, byte: u8){
        self.memory[address as usize] = byte;
    }
    
}
