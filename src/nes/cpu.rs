use std::{cell::RefCell, rc::Rc};
use crate::nes::bus::Bus;

pub struct Nes6502{
    a: u8,
    x: u8,
    y: u8,
    sp: u8,
    pc: u16,
    status: u8,
    bus: Rc<RefCell<Bus>>,
    cycles_left: u8,
}

impl Nes6502 {
    pub fn new(bus: Rc<RefCell<Bus>>) -> Self {
        Nes6502 {
            a: 0,
            x: 0,
            y: 0,
            sp: 0xFD,
            pc: 0,
            status: 0x24,
            bus,
            cycles_left: 0,
        }
    }

    pub fn cpu_read(&self, addr: u16) -> Option<u8>{
        self.bus.borrow().cpu_read(addr)
    }

    pub fn cpu_write(&self, addr: u16, data: u8) {
        self.bus.borrow_mut().cpu_write(addr, data);
    }

    pub fn clock(&mut self) {
        if self.cycles_left == 0 {
            // Fetch, Decode, Execute instructions here
            // For now, we will just set cycles_left to a dummy value
            self.cycles_left = 2; // Dummy cycle count for an instruction
        }
        self.cycles_left -= 1;
    }
}
