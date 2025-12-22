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

    pub fn game_loop(&mut self) -> Result<(), String> {
        let game_loop = true;
        while game_loop {
            // cpu clocking logic
            self.cpu.clock();
        }
        Ok(())
    }
}
