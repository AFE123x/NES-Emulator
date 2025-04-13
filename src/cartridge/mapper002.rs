use std::io;
use super::{mapper::Mapper, Nametable};

pub struct Mapper002 {
    bank_select: u8,
    prg_rom: u8,
    chr_rom: u8,
    last_bank_address: u32,
    nametable: Nametable,
}

impl Mapper002 {
    pub fn new(prg_rom: u8, chr_rom: u8, nametable: Nametable) -> Self {
        println!("prg_rom {}", prg_rom);
        let last_bank_address: u32 = ((prg_rom - 1) as u32) * 0x4000;
        Self {
            bank_select: 0,
            prg_rom,
            chr_rom,
            last_bank_address,
            nametable,
        }
    }
}

impl Mapper for Mapper002 {
    /*
     CPU $8000-$BFFF: 16 KB switchable PRG ROM bank
     CPU $C000-$FFFF: 16 KB PRG ROM bank, fixed to the last bank
     */
    fn cpu_read(&self, address: u16, mapped_addr: &mut u32, data: &mut u8) -> bool {
        if address >= 0x8000 && address <= 0xBFFF {
            // Map the switchable bank (0x8000-0xBFFF)
            *mapped_addr = ((self.bank_select as u32) * 0x4000) + (address as u32 & 0x3FFF);
            return true;
        } else if address >= 0xC000 {
            // Map to the fixed last bank (0xC000-0xFFFF)
            *mapped_addr = self.last_bank_address + (address as u32 & 0x3FFF);
            return true;
        }
        false
    }

    fn cpu_write(&mut self, address: u16, mapped_addr: &mut u32, data: u8) -> bool {
        if address >= 0x8000 {
            // Any write to 0x8000-0xFFFF changes the bank select register
            self.bank_select = data & 0x0F; // Limit to 16 banks (4 bits)
            return true;
        }
        false
    }

    fn ppu_read(&self, address: u16, mapped_addr: &mut u32, data:  u8) -> bool {
        if address <= 0x1FFF {
            // CHR ROM/RAM access (0x0000-0x1FFF)
            // UNROM mapper doesn't have CHR banking, straight mapping
            *mapped_addr = address as u32;
            return true;
        }
        false
    }

    fn ppu_write(&mut self, address: u16, mapped_addr: &mut u32, data: u8) -> bool {
        if address <= 0x1FFF {
            // CHR RAM is only writable if chr_rom == 0 (meaning it's RAM, not ROM)
            if self.chr_rom == 0 {
                *mapped_addr = address as u32;
                return true;
            }
        }
        false
    }

    fn get_nametable(&self) -> super::Nametable {
        self.nametable.clone()
    }

    fn savestate(&self) {
        println!("mapper 002 doesn't have prg-ram");
    }
}