use std::{fs::File, io::Read};

use super::{mapper::Mapper, MirrorMode};

/// Mapper001 (MMC1) implementation for NES emulator.
///
/// Supports CHR-ROM/CHR-RAM, SRAM, and PRG bank switching.
/// Includes serial register loading (5-bit shift register),
/// mirroring control, and SRAM save/load functionality.
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
    mirrormode: MirrorMode,
    n_prgbanks: u8,
    n_chrbanks: u8,
    ram: Vec<u8>,
}

impl Mapper001 {
    /// Constructs a new `Mapper001` instance.
    ///
    /// Initializes internal registers and memory based on
    /// PRG and CHR ROM sizes, and loads battery-backed SRAM
    /// from a file if provided.
    ///
    /// # Arguments
    ///
    /// * `prg_rom_size` - Number of 16KB PRG banks.
    /// * `chr_rom_size` - Number of 8KB CHR banks.
    /// * `_nametable_arrangement` - Reserved for mirroring setup (unused).
    /// * `save` - Optional path to a saved SRAM file.
    pub fn new(
        prg_rom_size: u8,
        chr_rom_size: u8,
        _nametable_arrangement: MirrorMode,
        save: Option<String>,
    ) -> Self {
        let mut toreturn = Self {
            n_load_register: 0,
            n_load_register_count: 0,
            n_control_register: 0,
            n_chrbank_select4_lo: 0,
            n_chrbank_select4_hi: 0,
            n_chrbank_select8: 0,
            n_prgbank_select16_lo: 0,
            n_prgbank_select16_hi: 0,
            n_prgbank_select32: 0,
            mirrormode: MirrorMode::Horizontal,
            ram: vec![0; 8192],
            n_prgbanks: prg_rom_size,
            n_chrbanks: chr_rom_size,
        };
        toreturn.reset();
        if let Some(path) = save {
            if let Ok(mut file) = File::open(path) {
                let _ = file.read(&mut toreturn.ram);
            }
        }
        toreturn
    }
}

impl Mapper for Mapper001 {
    /// Resets all mapper registers to default power-on state.
    fn reset(&mut self) {
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

    /// Returns the current nametable mirroring mode.
    fn get_mirror_mode(&self) -> MirrorMode {
        self.mirrormode.clone()
    }

    /// Handles PPU reads from CHR-ROM/CHR-RAM.
    ///
    /// Computes the mapped address based on the CHR bank mode and address.
    fn ppu_read(&mut self, address: u16, mapped_addr: &mut u32, _data: u8) -> bool {
        *mapped_addr = 0;
        if address < 0x2000 {
            if self.n_chrbanks == 0 {
                *mapped_addr = address as u32;
                return true;
            } else {
                if self.n_control_register & 0b10000 != 0 {
                    if address <= 0x0FFF {
                        *mapped_addr = (self.n_chrbank_select4_lo as u32 * 0x1000) + (address as u32 & 0x0FFF);
                        return true;
                    }
                    if address >= 0x1000 && address <= 0x1FFF {
                        *mapped_addr = (self.n_chrbank_select4_hi as u32 * 0x1000) + (address as u32 & 0x0FFF);
                        return true;
                    }
                } else {
                    *mapped_addr = (self.n_chrbank_select8 as u32 * 0x1000) + (address as u32 & 0x1FFF);
                    return true;
                }
            }
        }
        false
    }

    /// Handles PPU writes to CHR-RAM.
    ///
    /// Writes are only allowed if CHR is in RAM mode.
    fn ppu_write(&mut self, address: u16, mapped_addr: &mut u32, _data: u8) -> bool {
        *mapped_addr = 0;
        if address < 0x2000 && self.n_chrbanks == 0 {
            *mapped_addr = address as u32;
            return true;
        }
        false
    }

    /// Handles CPU reads from PRG-ROM and SRAM regions.
    ///
    /// Returns mapped address or data from internal RAM for battery saves.
    fn cpu_read(&self, address: u16, mapped_addr: &mut u32, data: &mut u8) -> bool {
        *mapped_addr = 0;
        if address >= 0x6000 && address <= 0x7FFF {
            *mapped_addr = 0xFFFFFFFF;
            *data = self.ram[(address & 0x1FFF) as usize];
            return true;
        }
        if address >= 0x8000 {
            if self.n_control_register & 0b01000 != 0 {
                if address < 0xC000 {
                    *mapped_addr =
                        (self.n_prgbank_select16_lo as u32 * 0x4000) + (address & 0x3FFF) as u32;
                    return true;
                } else {
                    *mapped_addr =
                        (self.n_prgbank_select16_hi as u32 * 0x4000) + (address & 0x3FFF) as u32;
                    return true;
                }
            } else {
                *mapped_addr =
                    (self.n_prgbank_select32 as u32 * 0x8000) + (address as u32 & 0x7FFF);
                return true;
            }
        }
        false
    }

    /// Handles CPU writes to SRAM and control registers (0x8000–0xFFFF).
    ///
    /// Implements the 5-bit serial register logic for MMC1.
    fn cpu_write(&mut self, address: u16, mapped_addr: &mut u32, data: u8) -> bool {
        *mapped_addr = 0;
        if address >= 0x6000 && address <= 0x7FFF {
            *mapped_addr = 0xFFFFFFFF;
            self.ram[address as usize & 0x1FFF] = data;
            return true;
        }

        if address >= 0x8000 {
            if data & 0x80 != 0 {
                self.n_load_register = 0;
                self.n_load_register_count = 0;
                self.n_control_register |= 0x0C;
            } else {
                self.n_load_register >>= 1;
                self.n_load_register &= !(1 << 4);
                self.n_load_register |= (data & 0x01) << 4;
                self.n_load_register_count = self.n_load_register_count.wrapping_add(1);

                if self.n_load_register_count == 5 {
                    let ntargetregister = ((address >> 13) & 0x3) as u8;

                    match ntargetregister {
                        0 => {
                            self.n_control_register = self.n_load_register & 0x1F;
                            self.mirrormode = match self.n_control_register & 0x03 {
                                0 => MirrorMode::OneScreenLo,
                                1 => MirrorMode::OneScreenHi,
                                2 => MirrorMode::Vertical,
                                3 => MirrorMode::Horizontal,
                                _ => unreachable!(),
                            };
                        }
                        1 => {
                            if self.n_control_register & 0b10000 != 0 {
                                self.n_chrbank_select4_lo = self.n_load_register & 0x1F;
                            } else {
                                self.n_chrbank_select8 = self.n_load_register & 0x1E;
                            }
                        }
                        2 => {
                            if self.n_control_register & 0b10000 != 0 {
                                self.n_chrbank_select4_hi = self.n_load_register & 0x1F;
                            }
                        }
                        3 => {
                            let n_prgmode = (self.n_control_register >> 2) & 0x3;
                            match n_prgmode {
                                0 | 1 => {
                                    self.n_prgbank_select32 = (self.n_load_register & 0x0E) >> 1;
                                }
                                2 => {
                                    self.n_prgbank_select16_lo = 0;
                                    self.n_prgbank_select16_hi = self.n_load_register & 0x0F;
                                }
                                3 => {
                                    self.n_prgbank_select16_lo = self.n_load_register & 0x0F;
                                    self.n_prgbank_select16_hi = self.n_prgbanks - 1;
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }

                    self.n_load_register = 0;
                    self.n_load_register_count = 0;
                }
            }
        }
        false
    }

    /// Saves the internal 8KB SRAM to a file chosen by the user.
    ///
    /// Uses a GUI file picker to choose the save file path.
    fn savestate(&self) {
        use std::fs::File;
        use std::io::Write;
        let file = rfd::FileDialog::new()
            .set_title("Save")
            .save_file();
        let file = match file {
            Some(file) => file,
            None => return,
        };
        let mut file = File::create(file).unwrap();
        file.write_all(&self.ram).unwrap();
    }

    /// Returns false; MMC1 does not support IRQs.
    fn hasirq(&mut self) -> bool {
        false
    }

    /// Placeholder for scanline IRQ logic (not used in MMC1).
    fn scanline(&mut self) {}

    /// Placeholder to clear IRQ flags (not used in MMC1).
    fn irq_clear(&mut self) {}
}
