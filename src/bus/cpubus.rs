use std::rc::Rc;
pub struct Cpubus{
    memory: Vec<u8>,
    cpu: Rc<crate::cpu::processor::Cpu>
}

impl Cpubus{
    pub fn new() -> Self{
        println!("CPU BUS - INITIALIZED");
        Self {
            memory: vec![0; 0x10000],
            cpu: Rc::new(crate::cpu::processor::Cpu::new()),
        }
    }
    pub fn clock(&self){
       println!("clock"); 
    }
}
