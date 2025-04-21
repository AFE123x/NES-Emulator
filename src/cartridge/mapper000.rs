use super::{mapper::Mapper, Nametable};

pub struct Mapper000{
    pub(crate) n_chr: u8,
    pub(crate) n_prg: u8,
    pub(crate) nametable: Nametable,
}

impl Mapper for Mapper000{
    fn cpu_read(&self, _address: u16,mapped_addr: &mut u32, _data: &mut u8) -> bool {
        if *mapped_addr >= 0x8000{
            *mapped_addr &= if self.n_prg > 1 {0x7FFF} else {0x3FFF};
            return true;
        }
        false
    }

    fn cpu_write(&mut self, _address: u16,mapped_addr: &mut u32, _data: u8) -> bool {
        if *mapped_addr >= 0x8000{
            *mapped_addr &= if self.n_prg > 1 {0x7FFF} else {0x3FFF};
            return true;
        }
        false
    }

    fn ppu_read(&self, _address: u16,mapped_addr: &mut u32, _data: u8) -> bool {
        if *mapped_addr <= 0x1FFF {
            if self.n_prg > 0{
                *mapped_addr = *mapped_addr;
                return true;
            }
        }
        false
    }

    fn ppu_write(&mut self, _address: u16,mapped_addr: &mut u32, _data: u8) -> bool {
        if *mapped_addr <= 0x1FFF {
            if self.n_chr == 0{
                *mapped_addr = *mapped_addr;
                return true;
            }
        }
        false
    }
    
    fn get_nametable(&self) -> super::Nametable {
        self.nametable.clone()
    }
    
    fn savestate(&self) {
        panic!("mapper 000 doesn't have prg-ram");
    }
    
    fn loadstate(&mut self) {
        panic!("mapper 001 doesn't have prg-ram");
    }
    
    fn hasirq(&mut self) -> bool {
        return false;
    }
    
    fn scanline(&mut self) {
        
    }
}
