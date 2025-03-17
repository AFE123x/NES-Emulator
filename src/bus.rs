use crate::{cartridge::Cartridge, controller::Controller, cpu::Cpu, ppu::Ppu};

pub struct Bus {
    memory: Vec<u8>,
    cpu: *mut Cpu,
    cartridge: *mut Cartridge,
    ppu: *mut Ppu,
    controller: Option<*mut Controller>,
    total_cycles: usize,
}

impl Bus {
    pub fn new(cpu: &mut Cpu, cart: &mut Cartridge, ppu: &mut Ppu) -> Self {
        Self {
            memory: vec![0; 2048],
            cpu,
            cartridge: cart,
            ppu: ppu,
            controller: None,
            total_cycles: 0,
        }
    }

    pub fn link_controller(&mut self, controller: &mut Controller) {
        self.controller = Some(controller);
    }
    pub fn cpu_read(&self, address: u16, rdonly: bool) -> u8 {
        let mut data = 0;
        if address <= 0x1FFF {
            data = self.memory[(address & 0x7FF) as usize];
        } else if address <= 0x3FFF {
            data = unsafe { (*self.ppu).cpu_read(address, rdonly) };
        } else if address <= 0x4017 {
            if address == 0x4016 {
                data = unsafe { (*self.controller.unwrap()).cpu_read() };
            }
        } else if address <= 0x401F {
            // todo!();
            data = 0;
        } else {
            unsafe { (*self.cartridge).cpu_read(address, &mut data) };
        }

        data
    }

    pub fn cpu_write(&mut self, address: u16, byte: u8) {
        if address <= 0x1FFF {
            self.memory[(address & 0x7FF) as usize] = byte;
        } else if address <= 0x3FFF {
            unsafe {
                (*self.ppu).cpu_write(address, byte);
            };
        } else if address <= 0x4017 {
            if address == 0x4016 {
                unsafe { (*self.controller.unwrap()).cpu_write(byte) };
            }
        } else if address <= 0x401F {
            // todo!()
        } else {
            unsafe {
                (*self.cartridge).cpu_write(address, byte);
            }
        }
    }

    pub fn clock(&mut self) {
        unsafe {
            (*self.ppu).clock();
            (*self.ppu).clock();
            (*self.ppu).clock();
            (*self.cpu).clock();
        }
        self.total_cycles += 1;
    }
}
