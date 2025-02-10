use std::rc::{Rc, Weak};
pub struct Cpubus{
    memory: Vec<u8>,
    cpu: Option<Weak<crate::cpu::processor::Cpu>>,
}

impl Cpubus{
    pub fn new() -> Self{
        println!("CPU BUS - INITIALIZED");
        Self {
            memory: vec![0; 0x10000],
            cpu: None,
        }
    }
    pub fn link_cpu(&mut self, cpu: Weak<crate::cpu::processor::Cpu>){
        self.cpu = Some(cpu);
    }
    pub fn clock(&self){
       println!("clock"); 
    }
}
