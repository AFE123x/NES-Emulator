use std::{fs::File, io::{self, Read, Write}};

use super::{mapper::Mapper, Nametable};

pub struct Mapper001 {
    // MMC1 registers
    shift_register: u8,
    shift_count: u8,
    control: u8,
    chr_bank_0: u8,
    chr_bank_1: u8,
    prg_bank: u8,
    
    // ROM configuration
    prg_banks: u8,
    chr_banks: u8,
    
    // RAM
    prg_ram: Vec<u8>,
    nametable: Nametable,

    // Last write cycle tracking (for MMC1 write throttling)
    last_write_cycle: u64
}

impl Mapper001 {
    pub fn new(prg_banks: u8, chr_banks: u8, nametable: Nametable, save: Option<String>) -> Self {
        let mut toreturn = Mapper001 {
            shift_register: 0x10, // Reset state (bit 4 set)
            shift_count: 0,
            control: 0x0C,       // Default control (bits 2 and 3 set)
            chr_bank_0: 0,
            chr_bank_1: 0,
            prg_bank: 0,
            prg_banks,
            chr_banks,
            prg_ram: vec![0; 0x2000], // 8KB PRG RAM
            nametable,
            last_write_cycle: 0
        };
        if let Some(string) = save{
            let mut file = File::open(string).unwrap();
            file.read(&mut toreturn.prg_ram).unwrap();
        }
        toreturn
    }
    
    // Helper to reset the shift register state
    fn reset_shift_register(&mut self) {
        self.shift_register = 0x10; // Bit 4 set
        self.shift_count = 0;
    }
    
    // PRG bank offset calculations based on control register bits 2-3
    fn get_prg_bank_offset(&self, addr: u16) -> u32 {
        let bank = ((addr & 0x4000) >> 14) as u8; // 0 for 0x8000, 1 for 0xC000
        let prg_mode = (self.control >> 2) & 0x03;
        
        let prg_bank_number = match prg_mode {
            0 | 1 => {
                // 32KB mode: Switch 32KB at $8000, ignore bit 0 of bank number
                ((self.prg_bank & 0x0E) >> 1) as u16
            },
            2 => {
                // Fix first bank at $8000, switch 16KB bank at $C000
                if bank == 0 {
                    0 // First bank is fixed to the first bank
                } else {
                    self.prg_bank as u16 // Second bank comes from register
                }
            },
            3 => {
                // Fix last bank at $C000, switch 16KB bank at $8000
                if bank == 0 {
                    self.prg_bank as u16 // First bank comes from register
                } else {
                    (self.prg_banks - 1) as u16 // Last bank is fixed to the last bank
                }
            },
            _ => unreachable!()
        };

        // Calculate the actual address
        // For 32KB modes, we still use 16KB banks internally
        if prg_mode == 0 || prg_mode == 1 {
            // 32KB mode: Each bank is 32KB, so we multiply by 2 for 16KB offsets
            (prg_bank_number * 2 + bank as u16) as u32 * 0x4000 + (addr & 0x3FFF) as u32
        } else {
            // 16KB mode: Direct mapping
            prg_bank_number as u32 * 0x4000 + (addr & 0x3FFF) as u32
        }
    }
    
    // CHR bank offset calculations based on control register bit 4
    fn get_chr_bank_offset(&self, addr: u16) -> u32 {
        // If there's no CHR ROM banks, we're using CHR RAM (always bank 0)
        if self.chr_banks == 0 {
            return addr as u32;
        }
        
        let chr_mode = (self.control >> 4) & 0x01;
        
        match chr_mode {
            0 => {
                // 8KB mode: use chr_bank_0 for the entire 8KB
                // The lowest bit is ignored in 8KB mode
                let bank_number = (self.chr_bank_0 & 0x1E) >> 1;
                // Ensure we don't exceed available CHR banks (each bank is 8KB)
                let effective_bank = bank_number % (self.chr_banks / 2 + self.chr_banks % 2);
                (effective_bank as u32) * 0x2000 + (addr & 0x1FFF) as u32
            },
            1 => {
                // 4KB mode: use chr_bank_0 for lower bank and chr_bank_1 for upper bank
                let bank = ((addr & 0x1000) >> 12) as u8; // 0 for 0x0000, 1 for 0x1000
                
                if bank == 0 {
                    // Lower 4KB bank from chr_bank_0
                    let effective_bank = self.chr_bank_0 % self.chr_banks;
                    (effective_bank as u32) * 0x1000 + (addr & 0x0FFF) as u32
                } else {
                    // Upper 4KB bank from chr_bank_1
                    let effective_bank = self.chr_bank_1 % self.chr_banks;
                    (effective_bank as u32) * 0x1000 + (addr & 0x0FFF) as u32
                }
            },
            _ => unreachable!()
        }
    }

    
    // Handle register writes (with serial protocol)
    fn write_register(&mut self, address: u16, data: u8, cycle: u64) {
        // MMC1 ignores writes on consecutive cycles (prevents CPU-clock-speed-dependent issues)
        if cycle == self.last_write_cycle + 1 {
            self.last_write_cycle = cycle;
            return;
        }
        self.last_write_cycle = cycle;

        // If bit 7 is set, reset the shift register
        if (data & 0x80) != 0 {
            self.reset_shift_register();
            // Set bits 2-3 in control (prg rom banking mode 3 - fix last bank)
            self.control |= 0x0C;
            self.update_nametable_mirroring();
            return;
        }
        
        // Serial loading of the shift register - only the LSB matters
        self.shift_register = ((self.shift_register >> 1) | ((data & 0x01) << 4));
        self.shift_count += 1;
        
        // After 5 bits are loaded, update the target register
        if self.shift_count == 5 {
            let register = (address >> 13) & 0x03;
            match register {
                0 => {
                    // Control register ($8000-$9FFF)
                    self.control = self.shift_register;
                    // Update nametable mirroring based on control bits
                    self.update_nametable_mirroring();
                },
                1 => {
                    // CHR bank 0 register ($A000-$BFFF)
                    self.chr_bank_0 = self.shift_register;
                },
                2 => {
                    // CHR bank 1 register ($C000-$DFFF)
                    self.chr_bank_1 = self.shift_register;
                },
                3 => {
                    // PRG bank register ($E000-$FFFF)
                    self.prg_bank = self.shift_register & 0x0F;
                },
                _ => unreachable!()
            }
            self.reset_shift_register();
        }
    }
    
    // Update nametable mirroring based on control register bits 0-1
    fn update_nametable_mirroring(&mut self) {
        let mirroring = self.control & 0x03;
        self.nametable = match mirroring {
            0 => Nametable::OneScreenLo,
            1 => Nametable::OneScreenHi,
            2 => Nametable::Vertical,
            3 => Nametable::Horizontal,
            _ => unreachable!()
        };
    }
}

impl Mapper for Mapper001 {
    fn cpu_read(&self, address: u16, mapped_addr: &mut u32, data: &mut u8) -> bool {
        if address >= 0x6000 && address <= 0x7FFF {
            // PRG RAM area (battery-backed save RAM)
            *mapped_addr = (address - 0x6000) as u32;
            *data = self.prg_ram[*mapped_addr as usize];
            *mapped_addr = 0xFFFFFFFF;
            return true;
        } else if address >= 0x8000 && address <= 0xFFFF {
            // PRG ROM area
            *mapped_addr = self.get_prg_bank_offset(address);
            return true;
        }
        false
    }
    
    fn cpu_write(&mut self, address: u16, mapped_addr: &mut u32, data: u8) -> bool {
        // Track CPU cycle if your Mapper trait allows it
        let cycle = 0; // If you can add a cycle parameter to cpu_write in your Mapper trait, use that
        
        if address >= 0x6000 && address <= 0x7FFF {
            // PRG RAM area
            let ram_addr = (address - 0x6000) as usize;
            if ram_addr < self.prg_ram.len() {
                self.prg_ram[ram_addr] = data;
            }
            *mapped_addr = 0xFFFFFFFF;
            return true;
        } else if address >= 0x8000 && address <= 0xFFFF {
            // Register write
            self.write_register(address, data, cycle);
            return true;
        }
        false
    }
    
    fn ppu_read(&self, address: u16, mapped_addr: &mut u32, _data: u8) -> bool {
        if address < 0x2000 {
            // CHR ROM/RAM area
            *mapped_addr = self.get_chr_bank_offset(address);
            return true;
        }
        
        // For nametable addressing, let the PPU handle it
        // using the nametable enum
        false
    }
    
    fn ppu_write(&mut self, address: u16, mapped_addr: &mut u32, data: u8) -> bool {
        if address < 0x2000 {
            // Calculate the mapped address
            *mapped_addr = self.get_chr_bank_offset(address);
            
            // Only writable if using CHR RAM (chr_banks == 0 indicates CHR RAM)
            // Some cartridges have both CHR ROM and CHR RAM, but for simplicity
            // we're assuming chr_banks == 0 means the cart only has CHR RAM
            return self.chr_banks == 0;
        }
        
        // For nametable addressing, let the PPU handle it
        // using the nametable enum
        false
    }
    
    fn get_nametable(&self) -> super::Nametable {
        self.nametable.clone()
    }
    
    fn savestate(&self) {
        let file = rfd::FileDialog::new()
        .set_title("Select a save file")
        .pick_file().unwrap();
        let mut file = File::create(file.to_str().unwrap()).unwrap();
        file.write_all(&self.prg_ram).unwrap();
    }
}