use std::{cell::RefCell, rc::Rc};

use crate::{cartridge::Cartridge, controller::Controller, cpu::Cpu, ppu::Ppu};

pub struct Bus {
    memory: Vec<u8>,
    cartridge: Option<Rc<RefCell<Cartridge>>>,
    ppu: Option<*mut Ppu>,
    controller: Option<Rc<RefCell<Controller>>>,
    total_cycles: usize,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            memory: vec![0; 2048],
            cartridge: None,
            controller: None,
            ppu: None,
            total_cycles: 0,
        }
    }
    pub fn link_cartridge(&mut self, cart: Rc<RefCell<Cartridge>>){
        self.cartridge = Some(cart);
    }
    pub fn link_ppu(&mut self, ppu: &mut Ppu){
        self.ppu = Some(ppu);
    }

    pub fn link_controller(&mut self, controller: Rc<RefCell<Controller>>) {
        self.controller = Some(controller);
    }
    pub fn cpu_read(&self, address: u16, rdonly: bool) -> u8 {
        let mut data = 0;
        if address <= 0x1FFF {
            data = self.memory[(address & 0x7FF) as usize];
        } else if address <= 0x3FFF {
            data = unsafe { (*self.ppu.unwrap()).cpu_read(address, rdonly) };
        } else if address <= 0x4017 {
            if address == 0x4016 {
                data = if let Some(controller) = &self.controller{
                    controller.borrow_mut().cpu_read()
                }
                else{
                    panic!("ERROR - Controller");
                };
            }
        } else if address <= 0x401F {
            // todo!();
            data = 0;
        } else {
            if let Some(cart) = &self.cartridge{
                cart.borrow_mut().cpu_read(address, &mut data);
            }
            else{
                panic!("Cartridge Error");
            }
        }

        data
    }

    pub fn cpu_write(&mut self, address: u16, byte: u8) {
        if address <= 0x1FFF {
            self.memory[(address & 0x7FF) as usize] = byte;
        } else if address <= 0x3FFF {
            unsafe {
                (*self.ppu.unwrap()).cpu_write(address, byte);
            };
        }
        else if address == 0x4014{
            let byte = byte as usize;
            let byte = byte << 8;
            for i in 0..=0xFF{
                let data = self.memory[byte | i];
                unsafe{
                        (*self.ppu.unwrap()).oam_dma_write(i as u8, data);
                    
                }
            }
        }
        else if address <= 0x4017 {
            if address == 0x4016 {
                if let Some(controller) = &self.controller{
                    controller.borrow_mut().cpu_write(byte);
                }
            }
        } 
        else if address <= 0x401F {
            // todo!()
        } else {

                if let Some(cartridge) = &self.cartridge{
                    cartridge.borrow_mut().cpu_write(address, byte);
                }
        }
    }

}
