pub trait Mapper{
    fn cpu_read(&self, address: &mut u16) -> bool;
    fn cpu_write(&self, address: &mut u16) -> bool;
    fn ppu_read(&self, address: &mut u16) -> bool;
    fn ppu_write(&self, address: &mut u16) -> bool;
}