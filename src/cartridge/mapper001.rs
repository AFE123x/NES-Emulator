use super::mapper::Mapper;

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
}

impl Mapper001 {
    pub fn new(prg_banks: u8, chr_banks: u8) -> Self {
        Mapper001 {
            shift_register: 0x10, // Reset state (bit 4 set)
            shift_count: 0,
            control: 0x0C,       // Default control (bits 2 and 3 set)
            chr_bank_0: 0,
            chr_bank_1: 0,
            prg_bank: 0,
            prg_banks,
            chr_banks,
            prg_ram: vec![0; 0x2000], // 8KB PRG RAM
        }
    }
    
    // Helper to reset the shift register state
    fn reset_shift_register(&mut self) {
        self.shift_register = 0x10; // Bit 4 set
        self.shift_count = 0;
    }
    
    // PRG bank offset calculations based on control register bits 2-3
    fn get_prg_bank_offset(&self, bank: u8) -> u32 {
        let prg_mode = (self.control >> 2) & 0x03;
        match prg_mode {
            0 | 1 => {
                // 32KB mode: ignore lowest bit of bank number
                ((self.prg_bank & 0x0E) as u32) * 0x4000
            },
            2 => {
                // Fix first bank at 0x8000, switch second bank
                if bank == 0 {
                    0
                } else {
                    ((self.prg_bank & 0x0F) as u32) * 0x4000
                }
            },
            3 => {
                // Fix last bank at 0xC000, switch first bank
                if bank == 0 {
                    ((self.prg_bank & 0x0F) as u32) * 0x4000
                } else {
                    ((self.prg_banks as u32) - 1) * 0x4000
                }
            },
            _ => unreachable!()
        }
    }
    
    // CHR bank offset calculations based on control register bit 4
    fn get_chr_bank_offset(&self, bank: u8) -> u32 {
        let chr_mode = (self.control >> 4) & 0x01;
        match chr_mode {
            0 => {
                // 8KB mode: ignore lowest bit of bank number
                ((self.chr_bank_0 & 0x1E) as u32) * 0x1000
            },
            1 => {
                // 4KB mode: use chr_bank_0 for lower bank and chr_bank_1 for upper bank
                if bank == 0 {
                    ((self.chr_bank_0 & 0x1F) as u32) * 0x1000
                } else {
                    ((self.chr_bank_1 & 0x1F) as u32) * 0x1000
                }
            },
            _ => unreachable!()
        }
    }
    
    // Handle register writes (with serial protocol)
    fn write_register(&mut self, address: u16, data: u8) {
        // If bit 7 is set, reset the shift register
        if (data & 0x80) != 0 {
            self.reset_shift_register();
            self.control |= 0x0C; // Set bits 2-3 in control (prg rom banking mode)
            return;
        }
        
        // Serial loading of the shift register
        self.shift_register = ((self.shift_register >> 1) | ((data & 0x01) << 4)) & 0x1F;
        self.shift_count += 1;
        
        // After 5 bits are loaded, update the target register
        if self.shift_count == 5 {
            let register = (address >> 13) & 0x03;
            match register {
                0 => {
                    // Control register
                    self.control = self.shift_register;
                },
                1 => {
                    // CHR bank 0 register
                    self.chr_bank_0 = self.shift_register;
                },
                2 => {
                    // CHR bank 1 register
                    self.chr_bank_1 = self.shift_register;
                },
                3 => {
                    // PRG bank register
                    self.prg_bank = self.shift_register & 0x0F;
                },
                _ => unreachable!()
            }
            self.reset_shift_register();
        }
    }
}

impl Mapper for Mapper001 {
    fn cpu_read(&self, address: u16, mapped_addr: &mut u32, _data: u8) -> bool {
        if address >= 0x6000 && address <= 0x7FFF {
            // PRG RAM area
            *mapped_addr = (address - 0x6000) as u32;
            return true;
        } else if address >= 0x8000 && address <= 0xFFFF {
            // PRG ROM area
            let bank = (address >= 0xC000) as u8;
            let offset = self.get_prg_bank_offset(bank);
            *mapped_addr = offset + ((address & 0x3FFF) as u32);
            return true;
        }
        false
    }
    
    fn cpu_write(&mut self, address: u16, _mapped_addr: &mut u32, data: u8) -> bool {
        if address >= 0x6000 && address <= 0x7FFF {
            // PRG RAM area
            let ram_addr = (address - 0x6000) as usize;
            self.prg_ram[ram_addr] = data;
            return true;
        } else if address >= 0x8000 && address <= 0xFFFF {
            // Register write
            self.write_register(address, data);
            return true;
        }
        false
    }
    
    fn ppu_read(&self, address: u16, mapped_addr: &mut u32, _data: u8) -> bool {
        if address < 0x2000 {
            // CHR ROM/RAM area
            let bank = (address >= 0x1000) as u8;
            let offset = self.get_chr_bank_offset(bank);
            *mapped_addr = offset + ((address & 0x0FFF) as u32);
            return true;
        }
        
        // Handle nametable mirroring based on control register bits 0-1
        if address >= 0x2000 && address <= 0x3EFF {
            let mirroring = self.control & 0x03;
            let mirror_addr = address & 0x0FFF;
            
            *mapped_addr = match mirroring {
                0 => { // Single screen lower bank
                    mirror_addr & 0x03FF
                },
                1 => { // Single screen upper bank
                    0x0400 + (mirror_addr & 0x03FF)
                },
                2 => { // Vertical mirroring
                    mirror_addr & 0x07FF
                },
                3 => { // Horizontal mirroring
                    ((mirror_addr / 0x0800) * 0x0400) + (mirror_addr & 0x03FF)
                },
                _ => unreachable!()
            } as u32;
            
            return true;
        }
        
        false
    }
    
    fn ppu_write(&self, address: u16, mapped_addr: &mut u32, _data: u8) -> bool {
        if address >= 0x2000 && address <= 0x3EFF {
            // Handle nametable mirroring for writes
            let mirroring = self.control & 0x03;
            let mirror_addr = address & 0x0FFF;
            
            *mapped_addr = match mirroring {
                0 => { // Single screen lower bank
                    mirror_addr & 0x03FF
                },
                1 => { // Single screen upper bank
                    0x0400 + (mirror_addr & 0x03FF)
                },
                2 => { // Vertical mirroring
                    mirror_addr & 0x07FF
                },
                3 => { // Horizontal mirroring
                    ((mirror_addr / 0x0800) * 0x0400) + (mirror_addr & 0x03FF)
                },
                _ => unreachable!()
            } as u32;
            
            return true;
        }
        
        false
    }
}