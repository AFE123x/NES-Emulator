use super::Nametable;

pub trait Mapper{
    fn cpu_read(&self, address: u16,mapped_addr: &mut u32, data: &mut u8) -> bool;
    fn cpu_write(&mut self, address: u16,mapped_addr: &mut u32, data: u8) -> bool;
    fn ppu_read(&self, address: u16,mapped_addr: &mut u32, data: u8) -> bool;
    fn ppu_write(&mut self, address: u16,mapped_addr: &mut u32, data: u8) -> bool;
    fn get_nametable(&self) -> Nametable;
    fn savestate(&self);
    fn loadstate(&mut self);
}