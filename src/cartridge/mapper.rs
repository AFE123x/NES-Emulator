pub trait Mapper{
    fn cpu_read(&self, address: u16,mapped_addr: &mut u32, data: u8) -> bool;
    fn cpu_write(&mut self, address: u16,mapped_addr: &mut u32, data: u8) -> bool;
    fn ppu_read(&self, address: u16,mapped_addr: &mut u32, data: u8) -> bool;
    fn ppu_write(&self, address: u16,mapped_addr: &mut u32, data: u8) -> bool;
}