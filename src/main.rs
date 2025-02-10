mod cpu;
mod bus;
fn main() {
    let bus = crate::bus::cpubus::Cpubus::new();
    loop {
        bus.clock();
    }
}
