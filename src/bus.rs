use std::{cell::RefCell, rc::Rc};

use crate::{apu::Apu, cartridge::Cartridge, controller::Controller, ppu::Ppu};

pub struct Bus {
    memory: Vec<u8>,
    cartridge: Option<Rc<RefCell<Cartridge>>>,
    ppu: Option<*mut Ppu>,
    controller1: Option<Rc<RefCell<Controller>>>,
    controller2: Option<Rc<RefCell<Controller>>>,
    apu: Option<Rc<RefCell<Apu>>>,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            memory: vec![0; 2048],
            cartridge: None,
            controller1: None,
            controller2: None,
            ppu: None,
            apu: None,
        }
    }

    pub fn link_cartridge(&mut self, cart: Rc<RefCell<Cartridge>>){
        self.cartridge = Some(cart);
    }
    pub fn link_ppu(&mut self, ppu: &mut Ppu){
        self.ppu = Some(ppu);
    }
    pub fn link_apu(&mut self, apu: Rc<RefCell<Apu>>){
        self.apu = Some(apu);
    }
    pub fn link_controller1(&mut self, controller: Rc<RefCell<Controller>>) {
        self.controller1 = Some(controller);
    }
    pub fn link_controller2(&mut self, controller: Rc<RefCell<Controller>>) {
        self.controller2 = Some(controller);
    }
    pub fn cpu_read(&self, address: u16, rdonly: bool) -> u8 {
        let mut data = 0;
        
        if address <= 0x1FFF {
            // CPU RAM (mirrored every 0x800 bytes)
            data = self.memory[(address & 0x7FF) as usize];
        } else if address <= 0x3FFF {
            // PPU registers ($2000-$3FFF)
            data = unsafe { (*self.ppu.unwrap()).cpu_read(address, rdonly) };
        } else if address <= 0x4017 {
            // APU and I/O registers
            match address {
                0x4000..=0x4013 | 0x4015 => {
                    if let Some(apu) = &self.apu{
                        data = apu.borrow_mut().cpu_read(address);
                    }
                    else{
                        panic!("APU Error");
                    }
                    
                },
                0x4016 => {
                    // Controller 1 data
                    if let Some(controller) = &self.controller1 {
                        data = controller.borrow_mut().cpu_read();
                    } else {
                        panic!("ERROR - Controller 1 not initialized");
                    }
                },
                0x4017 => {
                    // Controller 2 data
                    if let Some(controller) = &self.controller2 {
                        data = controller.borrow_mut().cpu_read();
                    } else {
                        // Some games don't use controller 2, so return 0 instead of panicking
                        data = 0;
                    }
                },
                _ => {
                    // Other APU/IO registers (not implemented)
                }
            }
        } else if address <= 0x401F {
            // APU and I/O functionality that is normally disabled
            data = 0;
        } else {
            // Cartridge space ($4020-$FFFF)
            if let Some(cart) = &self.cartridge {
                cart.borrow_mut().cpu_read(address, &mut data);
            } else {
                panic!("Cartridge Error - Attempted read with no cartridge loaded");
            }
        }
        
        data
    }
    
    pub fn cpu_write(&mut self, address: u16, byte: u8) {
        if address <= 0x1FFF {
            // CPU RAM (mirrored every 0x800 bytes)
            self.memory[(address & 0x7FF) as usize] = byte;
        } else if address <= 0x3FFF {
            // PPU registers ($2000-$3FFF)
            unsafe {
                (*self.ppu.unwrap()).cpu_write(address, byte);
            };
        } else if address <= 0x4017 {
            // APU and I/O registers
            match address {
                0x4000..=0x4013 | 0x4015 | 0x4017 => {
                    if let Some(apu) = &self.apu{
                        apu.borrow_mut().cpu_write(address,byte);
                    }
                    else{
                        panic!("APU Error");
                    }
                    
                },
                0x4014 => {
                    // OAM DMA transfer
                    // This writes 256 bytes from CPU memory at byte*0x100 to the PPU's OAM memory
                    let base = (byte as usize) << 8;
                    for i in 0..=0xFF {
                        let data = self.memory[(base + i) % 2048];
                        unsafe {
                            if let Some(ppu_ptr) = self.ppu {
                                (*ppu_ptr).oam_dma_write(i as u8, data);
                            } else {
                                panic!("PPU pointer is null during DMA transfer");
                            }
                        }
                    }
                    // Note: Real NES CPU is suspended during DMA for ~513-514 cycles
                    // Consider adding CPU cycle stall logic here
                },
                0x4016 => {
                    // Controller strobe
                    // On the NES, writing to $4016 strobes both controllers
                    if let Some(controller) = &self.controller1 {
                        controller.borrow_mut().cpu_write(byte);
                    }
                    if let Some(controller) = &self.controller2 {
                        controller.borrow_mut().cpu_write(byte);
                    }
                },
                _ => {
                    // Other APU registers (not implemented in this code)
                    // Implement APU register writes here if needed
                }
            }
        } else if address <= 0x401F {
            // APU and I/O functionality that is normally disabled
            // Usually ignored
        } else {
            // Cartridge space ($4020-$FFFF)
            if let Some(cartridge) = &self.cartridge {
                cartridge.borrow_mut().cpu_write(address, byte);
            } else {
                panic!("Cartridge Error - Attempted write with no cartridge loaded");
            }
        }
    }
}
