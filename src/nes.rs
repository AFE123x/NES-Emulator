use std::{cell::RefCell, rc::Rc};

mod bus;
mod cpu;

pub struct Nes {
    pub cpu: cpu::Nes6502,
    pub bus: Rc<RefCell<bus::Bus>>,
}

impl Nes {
    pub fn new() -> Self {
        let bus = Rc::new(RefCell::new(bus::Bus::new()));
        let cpu = cpu::Nes6502::new(Rc::clone(&bus));
        Nes { cpu, bus }
    }
}
