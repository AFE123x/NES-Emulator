
use super::{mapper::Mapper, MirrorMode};

pub struct Mapper066 {
    _n_chrbanks: u8,
    _n_prgbanks: u8,
    _n_prgbank_select: u8,
    _n_chrbank_select: u8,
    mirrormode: MirrorMode
}

impl Mapper066 {
    pub fn new(prg_rom_chunks: u8, chr_rom_chunks: u8, mirrormode: MirrorMode) -> Self {
        println!("{:?}", mirrormode);
        let mut toreturn = Self {
            _n_chrbanks: chr_rom_chunks,
            _n_prgbanks: prg_rom_chunks,
            _n_prgbank_select: 0,
            _n_chrbank_select: 0,
            mirrormode,
        };
        toreturn.reset();
        toreturn
    }

    pub fn reset(&mut self) {
        self._n_chrbank_select = 0;
        self._n_prgbank_select = 0;
    }
}

impl Mapper for Mapper066 {
    fn cpu_read(&self, address: u16, mapped_addr: &mut u32, _data: &mut u8) -> bool {
        if address >= 0x8000 {
            // Fixed: Use 32KB banks (0x8000) instead of just the offset calculation
            *mapped_addr = ((self._n_prgbank_select as u32) * 0x8000) + (address as u32 - 0x8000);
            return true;
        }
        false
    }

    fn cpu_write(&mut self, address: u16, _mapped_addr: &mut u32, data: u8) -> bool {
        if address >= 0x8000 {
            self._n_chrbank_select = data & 0x3;
            self._n_prgbank_select = (data & 0x30) >> 4;
            // Return true to indicate the cartridge handled the write
            return true;
        }
        false
    }

    fn ppu_read(&mut self, address: u16, mapped_addr: &mut u32, _data: u8) -> bool {
        if address <= 0x1FFF {
            *mapped_addr = (self._n_chrbank_select as u32 * 0x2000) + (address as u32);
            return true;
        }
        false
    }

    fn ppu_write(&mut self, address: u16, mapped_addr: &mut u32, _data: u8) -> bool {
        // Mapper 066 typically supports CHR-RAM in some configurations
        // If CHR banks is 0, allow writing to CHR-RAM
        if address <= 0x1FFF && self._n_chrbanks == 0 {
            *mapped_addr = (self._n_chrbank_select as u32 * 0x2000) + (address as u32);
            return true;
        }
        false
    }

    fn get_mirror_mode(&self) -> super::MirrorMode {
        self.mirrormode.clone()
    }

    fn irq_clear(&mut self) {
        // Mapper 066 doesn't support IRQs
    }

    fn savestate(&self) {
        // Implementation depends on your savestate system
    }

    fn loadstate(&mut self) {
        // Implementation depends on your savestate system  
    }

    fn hasirq(&mut self) -> bool {
        false
    }

    fn scanline(&mut self) {
        // Mapper 066 doesn't have scanline-based logic
    }

    fn reset(&mut self) {
        self._n_chrbank_select = 0;
        self._n_prgbank_select = 0;
    }

    
}