use super::{mapper::Mapper, Nametable};

pub struct Mapper002 {
    n_prgbank_select_lo: u8,
    n_prgbank_select_hi: u8,
    n_prgbanks: u8,
    n_chrbanks: u8,
    nametable: Nametable,
}

impl Mapper002 {
    pub fn new(prg_rom: u8, chr_rom: u8, nametable: Nametable) -> Self {
        let mut toreturn = Mapper002{
            n_prgbanks: prg_rom,
            n_chrbanks: chr_rom,
            nametable,
            n_prgbank_select_hi: 0,
            n_prgbank_select_lo: 0,
        };
        toreturn.reset();
        toreturn
    }
}

impl Mapper002{
    fn reset(&mut self){
        self.n_prgbank_select_hi = self.n_prgbanks - 1;
        self.n_prgbank_select_lo = 0;
    }
}
impl Mapper for Mapper002 {
    
    fn cpu_read(&self, address: u16, mapped_addr: &mut u32, _data: &mut u8) -> bool {
        *mapped_addr = 0;
        if address >= 0x8000 && address <= 0xBFFF{
            *mapped_addr = ((self.n_prgbank_select_lo as u32) * 0x4000) + ((address as u32) & 0x3FFF);
            return true;
        }
        if address >= 0xC000{
            *mapped_addr = ((self.n_prgbank_select_hi as u32) * 0x4000) + ((address as u32) & 0x3FFF);
            return true;
        }
        false
    }

    fn cpu_write(&mut self, address: u16, _mapped_addr: &mut u32, data: u8) -> bool {
        if address >= 0x8000{
            self.n_prgbank_select_lo = data & 0xF;
        }
        false
    }

    fn ppu_read(&self, address: u16, mapped_addr: &mut u32, _data:  u8) -> bool {
        if address <= 0x1FFF{
            *mapped_addr = address as u32;
            return true;
        }
        false
    }

    fn ppu_write(&mut self, address: u16, mapped_addr: &mut u32, _data: u8) -> bool {
        if self.n_chrbanks == 0{
            *mapped_addr = address as u32;
            return true;
        }
        false
    }

    fn get_nametable(&self) -> super::Nametable {
        self.nametable.clone()
    }

    fn savestate(&self) {
        panic!("mapper 002 doesn't have prg-ram");
    }
    
    fn loadstate(&mut self) {
        panic!("mapper 001 doesn't have prg-ram");
    }
}