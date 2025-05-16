use super::{mapper::Mapper, MirrorMode};

pub struct Mapper003{
    _n_prgbanks: u8,
    n_chrbanks: u8,
    n_chrbank_select: u8,
    mirrormode: MirrorMode,
}

impl Mapper003{
    pub fn new(_n_prgbanks: u8, n_chrbanks: u8, mirrormode: MirrorMode) -> Self{
        Self { _n_prgbanks, n_chrbanks, n_chrbank_select: 0, mirrormode }
    }
}
impl Mapper for Mapper003{
    fn cpu_read(&self, address: u16,mapped_addr: &mut u32, _data: &mut u8) -> bool {
        *mapped_addr = address as u32;
        true
    }

    fn cpu_write(&mut self, address: u16,_mapped_addr: &mut u32, data: u8) -> bool {
        if address >= 0x8000{
            self.n_chrbank_select = data & 0x3;
        }
        false
    }

    fn ppu_read(&mut self, address: u16,mapped_addr: &mut u32, _data: u8) -> bool {
        *mapped_addr = 0;
        if address <= 0x1FFF{
            *mapped_addr = ((self.n_chrbank_select as u32) * 0x2000) + (address as u32);
            return true;
        }
        false
    }

    fn ppu_write(&mut self, address: u16,mapped_addr: &mut u32, _data: u8) -> bool {
        if self.n_chrbanks == 0{
            *mapped_addr = address as u32;
            return true;
        }
        false
    }

    fn get_mirror_mode(&self) -> MirrorMode {
        self.mirrormode.clone()
    }

    fn savestate(&self) {
        panic!("save not available on mapper 3");
    }

    fn loadstate(&mut self) {
        panic!("load not available for mapper 3")
    }
    
    fn hasirq(&mut self) -> bool {
        return false;
    }
    
    fn scanline(&mut self) {
    }
    
    fn reset(&mut self) {
    }
    
    fn irq_clear(&mut self) {
    }
}