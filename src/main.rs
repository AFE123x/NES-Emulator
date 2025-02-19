mod cpu;
mod bus;
use cpu::processor::Cpu;
use bus::cpubus::Cpubus;

fn main() {
    // Create CPU and Bus separately
    let mut cpu = Cpu::new();
    let mut bus = Cpubus::new(&mut cpu);
    cpu.linkbus(&mut bus);

    // Start the clock loop
    loop {
        bus.clock();
    }
}

