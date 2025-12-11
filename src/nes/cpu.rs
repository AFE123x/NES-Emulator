use std::{cell::RefCell, rc::Rc};
use crate::nes::bus::Bus;

pub struct Nes6502{
    a: u8,
    x: u8,
    y: u8,
    sp: u8,
    pc: u16,
    status: u8,
    bus: Rc<RefCell<Bus>>
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
        }
    }
}
