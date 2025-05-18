use super::{mapper::Mapper, MirrorMode};

pub struct Mapper066{
    nCHRBanks: u8,
    nPRGBanks: u8,
    nPRGBankSelect: u8,
    nCHRBankSelect: u8,
    mirrormode: MirrorMode
}

impl Mapper066{
    pub fn new(prg_rom_chunks: u8, chr_rom_chunks: u8, mirrormode: MirrorMode) -> Self{
        let mut toreturn = Self{
            nCHRBanks: chr_rom_chunks,
            nPRGBanks: prg_rom_chunks,
            nPRGBankSelect: 0,
            nCHRBankSelect: 0,
            mirrormode,
        };
        toreturn.reset();
        toreturn

    }

    pub fn reset(&mut self){
        self.nCHRBankSelect = 0;
        self.nPRGBankSelect = 0;
    }
}

impl Mapper for Mapper066{
    fn cpu_read(&self, address: u16,mapped_addr: &mut u32, data: &mut u8) -> bool {
        if address >= 0x8000{
            *mapped_addr = ((self.nPRGBankSelect as u32) * 0x8000) + (address as u32 & 0x7FFF);
            return true;
        }
        return false;
    }

    fn cpu_write(&mut self, address: u16,mapped_addr: &mut u32, data: u8) -> bool {
        if address >= 0x8000{
            self.nCHRBankSelect = data & 0x3;
            self.nPRGBankSelect = (data & 0x30) >> 4;
        }
        return false;
    }

    fn ppu_read(&mut self, address: u16,mapped_addr: &mut u32, data: u8) -> bool {
        if address <= 0x1FFF{
            *mapped_addr = (self.nCHRBankSelect as u32 * 0x2000) + (address as u32);
            return true;
        }
        false
    }

    fn ppu_write(&mut self, _address: u16,_mapped_addr: &mut u32, _data: u8) -> bool {
        return false;
    }

    fn get_mirror_mode(&self) -> super::MirrorMode {
        self.mirrormode.clone()
    }

    fn irq_clear(&mut self) {
        
    }

    fn savestate(&self) {

    }

    fn loadstate(&mut self) {

    }

    fn hasirq(&mut self) -> bool {
        false
    }

    fn scanline(&mut self) {

    }

    fn reset(&mut self) {

    }
}