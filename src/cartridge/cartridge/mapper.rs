pub trait Map{
    fn cpuread(&self, address: u16) -> (u16,bool);
    fn cpuwrite(&mut self, address: u16, byte: u8) -> (u16, bool);
    fn ppu_map_read(&self, address: u16, mapped_addr: &mut u16) -> bool;
    fn ppu_map_write(&self, address: u16, mapped_addr: &mut u16) -> bool;
}