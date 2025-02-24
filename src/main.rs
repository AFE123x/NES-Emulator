mod bus;
mod cpu;
use bus::cpubus::Cpubus;
use cpu::processor::Cpu;

fn main() {
    let mut cpu = Cpu::new();
    let mut bus = Cpubus::new(&mut cpu);
    match bus.load_rom(&"roms/nestest.nes".to_string()) {
        Ok(_) => {}
        Err(_) => {
            panic!("INVALID ROM!!!");
        }
    }
    cpu.linkbus(&mut bus);

    // Start the clock loop
    loop {
        bus.clock();
    }
}
