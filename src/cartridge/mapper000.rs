

use crate::cartridge::cartridge::mapper::Map;


pub struct Mapper000{
    nPRG: u8,
    nCHR: u8,
}

impl Mapper000{
    pub fn new(nPRG: u8, nCHR: u8) -> Self{
        Self { nPRG: nPRG, nCHR: nCHR }
    }
}
impl Map for Mapper000{
    fn cpuread(&self, address: u16) -> (u16,bool) {
        if address >= 0x8000 && address <= 0xFFFF
        {
            let mapped_addr: u16 = (address & (if self.nPRG > 1 {0x7FFF} else {0x3FFF}));
            return (mapped_addr,true);
        }
        (0,false)
    }
    
    fn cpuwrite(&mut self, address: u16, byte: u8) -> (u16, bool) {
        if address >= 0x8000 && address <= 0xFFFF
        {
            let mapped_address: u16  = address & (if self.nPRG > 1 {0x7FFF} else {0x3FFF});
            return (mapped_address,true);
        }
        return (0,false);
    }
    
    fn ppu_map_read(&self, address: u16, mapped_addr: &mut u16) -> bool {
        if address <= 0x1FFF{
            *mapped_addr = address;
            return true;
        }
        return false;
    }
    
    fn ppu_map_write(&self, address: u16, mapped_addr: &mut u16) -> bool {
        if address <= 0x1FFF{
            if self.nCHR == 0{
                *mapped_addr = address;
                return true;
            }
        }
        return false;
    }

}