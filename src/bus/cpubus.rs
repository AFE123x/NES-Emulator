


use crate::{cartridge::cartridge::Cartridge, cpu::processor::Cpu};
use crate::ppu::ppu::Ppu;
pub struct Cpubus{
    memory: Vec<u8>,
    cpu: *mut Cpu,
    cartridge: *mut Cartridge,
    ppu: *mut Ppu,
}

impl Cpubus{
    pub fn new(cpu: &mut Cpu, cartridge: &mut Cartridge, ppu: &mut Ppu) -> Self{
        println!("CPU BUS - INITIALIZED");
        Self{
            memory: vec![0;2048],
            cpu: cpu,
            cartridge: cartridge,
            ppu: ppu,
        }
    }
    pub fn clock(&self) {
        unsafe{
            (*self.cpu).clock();
        }
    }
    

    pub fn cpu_read(&self, address: u16, readonly: bool) -> u8{

        let (mut data,sucess) = unsafe {
            (*self.cartridge).cpuread(address,false)
        };
        if sucess{

        }
        else if address <= 0x1FFF{
            data = self.memory[(address & 0x7FF) as usize];
        }
        else if address >= 0x2000 && address <= 0x3FFF{
            let address = address & 0x7;
            unsafe{
                data = (*self.ppu).cpu_read(address, readonly);
            }
        }
        else if address >= 0x4016 && address <= 0x4017{
            // todo!();
            data = 0;
        }
        data
    }
    pub fn reset(&mut self){
        unsafe{
            (*self.cpu).reset();
        }
    }
    pub fn cpu_write(&mut self, address: u16, byte: u8){
        let success = unsafe{
            (*self.cartridge).cpu_write(address, byte)
        };
        if success{

        }
        else if address <= 0x1FFF{
             self.memory[(address & 0x7FF) as usize] = byte;
        }
        else if address <= 0x2000 && address <= 0x3FFF{
            unsafe {
                (*self.ppu).cpu_write(address & 0x7, byte);
            }
        }
        else if address >= 0x4016 && address <= 0x4017{
            // todo!();
            
        }
    }
    
}
