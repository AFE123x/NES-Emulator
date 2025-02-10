mod cpu;
mod bus;
use std::rc::{Rc, Weak};

fn main() {
    // Create CPU and Bus separately
    let mut cpu = Rc::new(crate::cpu::processor::Cpu::new());
    let mut bus = Rc::new(crate::bus::cpubus::Cpubus::new());

    // Convert Rc<Cpubus> to Weak<Cpubus> and link to CPU
    let weakbus = Rc::downgrade(&bus);
    cpu.linkbus(weakbus);

    // Convert Rc<Cpu> to Weak<Cpu> and link to Bus
    let weakcpu = Rc::downgrade(&cpu);
    bus.link_cpu(weakcpu);

    // Start the clock loop
    loop {
        bus.clock();
    }
}

