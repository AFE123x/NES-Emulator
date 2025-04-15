use std::{fs::File, io::Read};

use super::{mapper::Mapper, Nametable};

pub struct Mapper001 {
    n_load_register: u8,
    n_load_register_count: u8,
    n_control_register: u8,
    n_chrbank_select4_lo: u8,
    n_chrbank_select4_hi: u8,
    n_chrbank_select8: u8,
    n_prgbank_select16_lo: u8,
    n_prgbank_select16_hi: u8,
    n_prgbank_select32: u8,
    mirrormode: Nametable,
    n_prgbanks: u8,
    n_chrbanks: u8,
    ram: Vec<u8>,
}

impl Mapper001 {
    pub fn new(
        prg_rom_size: u8,
        chr_rom_size: u8,
        _nametable_arrangement: Nametable,
        save: Option<String>,
    ) -> Self {
        let mut toreturn = Self {
            n_load_register: 0,
            n_load_register_count: 0,
            n_control_register: 0,
            n_chrbank_select4_lo: 0,
            n_chrbank_select4_hi: 0,
            n_prgbank_select16_lo: 0,
            n_prgbank_select16_hi: 0,
            n_prgbank_select32: 0,
            mirrormode: Nametable::Horizontal,
            ram: vec![0; 8192],
            n_prgbanks: prg_rom_size,
            n_chrbanks: chr_rom_size,
            n_chrbank_select8: 0,
        };
        toreturn.reset();
        if let Some(path) = save {
            if let Ok(mut file) = File::open(path) {
                let _ = file.read(&mut toreturn.ram);
            }
        }
        toreturn
    }
    pub fn reset(&mut self) {
        self.n_control_register = 0x1C;
        self.n_load_register = 0x00;
        self.n_load_register_count = 0;
        self.n_chrbank_select4_lo = 0;
        self.n_chrbank_select4_hi = 0;
        self.n_chrbank_select8 = 0;

        self.n_prgbank_select32 = 0;
        self.n_prgbank_select16_lo = 0;
        self.n_prgbank_select16_hi = self.n_prgbanks - 1;
    }

}

impl Mapper for Mapper001 {
    fn get_nametable(&self) -> Nametable {
        self.mirrormode.clone()
    }

    fn ppu_read(&self, address: u16, mapped_addr: &mut u32, _data: u8) -> bool {
        *mapped_addr = 0;
        if address < 0x2000 {
            if self.n_chrbanks == 0 {
                // CHR-RAM mode
                *mapped_addr = address as u32;
                return true;
            } else {
                // CHR-ROM mode
                if self.n_control_register & 0b10000 != 0 {
                    // 4K CHR Bank Mode
                    if address <= 0x0FFF {
                        *mapped_addr = (self.n_chrbank_select4_lo as u32 * 0x1000) + (address as u32 & 0x0FFF);
                        return true;
                    }
    
                    if address >= 0x1000 && address <= 0x1FFF {
                        *mapped_addr = (self.n_chrbank_select4_hi as u32 * 0x1000) + (address as u32 & 0x0FFF);
                        return true;
                    }
                } else {
                    // 8K CHR Bank Mode - IMPORTANT: Using the Go implementation's approach
                    *mapped_addr = (self.n_chrbank_select8 as u32 * 0x1000) + (address as u32 & 0x1FFF);
                    return true;
                }
            }
        }
        return false;
    }

    fn ppu_write(&mut self, address: u16, mapped_addr: &mut u32, _data: u8) -> bool {
        *mapped_addr = 0;
        // Only allow writes to CHR-RAM
        if address < 0x2000 && self.n_chrbanks == 0 {
            *mapped_addr = address as u32;
            return true;
        }
        false
    }

    fn cpu_read(&self, address: u16, mapped_addr: &mut u32, data: &mut u8) -> bool {
        *mapped_addr = 0;
        // Handle reading from SRAM (0x6000-0x7FFF)
        if address >= 0x6000 && address <= 0x7FFF {
            *mapped_addr = 0xFFFFFFFF; // Signal to use internal RAM
            *data = self.ram[(address & 0x1FFF) as usize];
            return true;
        }
        // Handle PRG ROM banking (0x8000-0xFFFF)
        if address >= 0x8000 {
            if self.n_control_register & 0b01000 != 0 {
                // 16KB mode
                if address < 0xC000 {
                    *mapped_addr =
                        (self.n_prgbank_select16_lo as u32 * 0x4000) + (address & 0x3FFF) as u32;
                    return true;
                } else {
                    // address >= 0xC000
                    *mapped_addr =
                        (self.n_prgbank_select16_hi as u32 * 0x4000) + (address & 0x3FFF) as u32;
                    return true;
                }
            } else {
                // 32KB mode
                *mapped_addr =
                    (self.n_prgbank_select32 as u32 * 0x8000) + (address as u32 & 0x7FFF);
                return true;
            }
        }
        false
    }

    fn cpu_write(&mut self, address: u16, mapped_addr: &mut u32, data: u8) -> bool {
        *mapped_addr = 0;
        // Handle writing to SRAM (0x6000-0x7FFF)
        if address >= 0x6000 && address <= 0x7FFF {
            *mapped_addr = 0xFFFFFFFF; // Signal to use internal RAM
            self.ram[address as usize & 0x1FFF] = data;
            return true;
        }
        // Handle MMC1 register writes (0x8000-0xFFFF)
        if address >= 0x8000 {
            if data & 0x80 != 0 {
                // Reset loading if bit 7 is set
                self.n_load_register = 0;
                self.n_load_register_count = 0;
                self.n_control_register = self.n_control_register | 0x0C;
            } else {
                // Serial loading of register data
                self.n_load_register >>= 1;
                self.n_load_register &= !(1 << 4);
                self.n_load_register |= (data & 0x01) << 4;
                self.n_load_register_count = self.n_load_register_count.wrapping_add(1);

                if self.n_load_register_count == 5 {
                    // Process the loaded register based on the address
                    let ntargetregister = ((address >> 13) & 0x3) as u8;

                    if ntargetregister == 0 {
                        // 0x8000-0x9FFF
                        // Control register
                        self.n_control_register = self.n_load_register & 0x1F;
                        self.mirrormode = match self.n_control_register & 0x03 {
                            0 => Nametable::OneScreenLo,
                            1 => Nametable::OneScreenHi,
                            2 => Nametable::Vertical,
                            3 => Nametable::Horizontal,
                            _ => unreachable!(),
                        };
                    } else if ntargetregister == 1 {
                        // 0xA000-0xBFFF
                        // CHR bank 0
                        if self.n_control_register & 0b10000 != 0 {
                            // 4KB CHR Bank at PPU 0x0000
                            self.n_chrbank_select4_lo = self.n_load_register & 0x1F;
                        } else {
                            // 8KB CHR Bank at PPU 0x0000
                            self.n_chrbank_select8 = self.n_load_register & 0x1E;
                        }
                    } else if ntargetregister == 2 {
                        // 0xC000-0xDFFF
                        // CHR bank 1
                        if self.n_control_register & 0b10000 != 0 {
                            // 4KB CHR Bank at PPU 0x1000
                            self.n_chrbank_select4_hi = self.n_load_register & 0x1F;
                        }
                        // In 8KB mode, this register is ignored
                    } else if ntargetregister == 3 {
                        // 0xE000-0xFFFF
                        // PRG bank
                        let n_prgmode = (self.n_control_register >> 2) & 0x3;

                        if n_prgmode == 0 || n_prgmode == 1 {
                            // 32KB mode
                            self.n_prgbank_select32 = (self.n_load_register & 0x0E) >> 1;
                        } else if n_prgmode == 2 {
                            // 16KB mode with fixed first bank
                            self.n_prgbank_select16_lo = 0;
                            self.n_prgbank_select16_hi = self.n_load_register & 0x0F;
                        } else if n_prgmode == 3 {
                            // 16KB mode with fixed last bank
                            self.n_prgbank_select16_lo = self.n_load_register & 0x0F;
                            self.n_prgbank_select16_hi = self.n_prgbanks - 1;
                        }
                    }

                    // Reset for next 5-bit sequence
                    self.n_load_register = 0;
                    self.n_load_register_count = 0;
                }
            }
        }
        false
    }

    fn savestate(&self) {
        use std::fs::File;
        use std::io::Write;
        let file = rfd::FileDialog::new()
        .set_title("Save")
        .save_file();
        let file = match file{
            Some(file) => file,
            None => 
            {
                return;
            },
        };
        let mut file = File::create(file).unwrap();

        file.write_all(&self.ram).unwrap();
        std::process::exit(0);
    }

    fn loadstate(&mut self) {
        let file = rfd::FileDialog::new().set_title("Open save").pick_file().unwrap();
        if let Ok(mut file) = File::open(file) {
            let _ = file.read(&mut self.ram);
        }
    }
}
