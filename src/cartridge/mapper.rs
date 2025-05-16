use super::MirrorMode;

pub trait Mapper{
    fn cpu_read(&self, address: u16,mapped_addr: &mut u32, data: &mut u8) -> bool;
    fn cpu_write(&mut self, address: u16,mapped_addr: &mut u32, data: u8) -> bool;
    fn ppu_read(&mut self, address: u16,mapped_addr: &mut u32, data: u8) -> bool;
    fn ppu_write(&mut self, address: u16,mapped_addr: &mut u32, data: u8) -> bool;
    fn get_mirror_mode(&self) -> MirrorMode;
    fn irq_clear(&mut self);
    fn savestate(&self);
    fn loadstate(&mut self);
    fn hasirq(&mut self) -> bool;
    fn scanline(&mut self);
    fn reset(&mut self);
}