use super::{mapper::Mapper, Nametable};

const READFROMSRAM: u32 = 0xFFFFFFFF; // Special value to indicate reading from SRAM

pub struct Mapper004 {
    // ROM bank information
    prg_banks: u8,
    chr_banks: u8,

    // Mapper registers
    target_register: u8,
    prg_bank_mode: bool,
    chr_inversion: bool,

    mirror_mode: Nametable,

    registers: [i32; 8],
    // Bank offsets for CHR and PRG ROM
    chr_bank: [u32; 8],
    prg_bank: [u32; 4],

    // IRQ status flags
    irq_active: bool,
    irq_enable: bool,
    irq_update: bool,

    // IRQ counter
    irq_counter: u16,
    irq_reload: u16,

    // Static RAM
    ram_static: Vec<u8>,
}

impl Mapper004 {
    pub fn new(prg_rom_chunks: u8, chr_rom_chunks: u8) -> Self {
        let mut mapper = Mapper004 {
            prg_banks: prg_rom_chunks,
            chr_banks: chr_rom_chunks,
            mirror_mode: Nametable::Horizontal,
            ram_static: vec![0; 8192],

            target_register: 0,
            prg_bank_mode: false,
            chr_inversion: false,

            registers: [0; 8],
            chr_bank: [0; 8],
            prg_bank: [0; 4],

            irq_active: false,
            irq_enable: false,
            irq_update: false,
            irq_counter: 0,
            irq_reload: 0,
        };

        mapper.reset();
        mapper
    }

    fn reset(&mut self) {
        self.target_register = 0;
        self.prg_bank_mode = false;
        self.chr_inversion = false;
        self.mirror_mode = Nametable::Horizontal;

        self.irq_active = false;
        self.irq_enable = false;
        self.irq_update = false;
        self.irq_counter = 0;
        self.irq_reload = 0;

        for i in 0..self.prg_bank.len() {
            self.prg_bank[i] = 0;
        }
        for i in 0..self.chr_bank.len() {
            self.chr_bank[i] = 0;
            if i < self.registers.len() {
                self.registers[i] = 0;
            }
        }

        // Initial bank setup
        self.prg_bank[0] = 0 * 0x2000;
        self.prg_bank[1] = 1 * 0x2000;

        // Fixed upper 16K mode by default
        self.prg_bank[2] = (self.prg_banks as u32 * 2 - 2) * 0x2000;
        self.prg_bank[3] = (self.prg_banks as u32 * 2 - 1) * 0x2000;
        
        // Set up initial CHR banking as well
        self.update_bank_offset();
    }

    fn update_bank_offset(&mut self) {
        // Update Pointer Table
        if self.chr_inversion {
            self.chr_bank[0] = self.chr_bank_offset(self.registers[2]);
            self.chr_bank[1] = self.chr_bank_offset(self.registers[3]);
            self.chr_bank[2] = self.chr_bank_offset(self.registers[4]);
            self.chr_bank[3] = self.chr_bank_offset(self.registers[5]);
            self.chr_bank[4] = self.chr_bank_offset(self.registers[0] & 0xFE);
            self.chr_bank[5] = self.chr_bank_offset(self.registers[0] | 0x01);
            self.chr_bank[6] = self.chr_bank_offset(self.registers[1] & 0xFE);
            self.chr_bank[7] = self.chr_bank_offset(self.registers[1] | 0x01);
        } else {
            self.chr_bank[0] = self.chr_bank_offset(self.registers[0] & 0xFE);
            self.chr_bank[1] = self.chr_bank_offset(self.registers[0] | 0x01);
            self.chr_bank[2] = self.chr_bank_offset(self.registers[1] & 0xFE);
            self.chr_bank[3] = self.chr_bank_offset(self.registers[1] | 0x01);
            self.chr_bank[4] = self.chr_bank_offset(self.registers[2]);
            self.chr_bank[5] = self.chr_bank_offset(self.registers[3]);
            self.chr_bank[6] = self.chr_bank_offset(self.registers[4]);
            self.chr_bank[7] = self.chr_bank_offset(self.registers[5]);
        }

        if self.prg_bank_mode {
            self.prg_bank[0] = self.prg_bank_offset(-2);
            self.prg_bank[2] = self.prg_bank_offset(self.registers[6]);
        } else {
            self.prg_bank[0] = self.prg_bank_offset(self.registers[6]);
            self.prg_bank[2] = self.prg_bank_offset(-2);
        }

        self.prg_bank[1] = self.prg_bank_offset(self.registers[7]);
        self.prg_bank[3] = self.prg_bank_offset(-1); // last 8k bank
    }

    fn prg_bank_offset(&self, index: i32) -> u32 {
        let mut idx = index;
        if idx >= 0x80 {
            idx -= 0x100;
        }
        
        // Ensure we have a valid index
        let total_banks = self.prg_banks as i32 * 2;
        
        // Handle negative indices correctly
        if idx < 0 {
            idx = total_banks + idx;
        } else {
            idx %= total_banks;
        }
        
        (idx as u32) * 0x2000
    }

    // CHR ROM is divided into 1k banks
    fn chr_bank_offset(&self, index: i32) -> u32 {
        let mut idx = index;
        if idx >= 0x80 {
            idx -= 0x100;
        }

        // If chr_banks is 0, it indicates CHR RAM which is treated as one 8KB bank
        if self.chr_banks == 0 {
            return (idx as u32 & 0x07) * 0x0400;
        }

        // Calculate the total number of 1KB CHR banks
        let total_banks = self.chr_banks as i32 * 8;
        
        // Handle negative indices correctly
        if idx < 0 {
            idx = total_banks + idx;
        } else {
            idx %= total_banks;
        }
        
        (idx as u32) * 0x0400
    }
}

impl Mapper for Mapper004 {
    fn scanline(&mut self) {
        // First check if we need to reload
        if self.irq_counter == 0 || self.irq_update {
            self.irq_counter = self.irq_reload;
            self.irq_update = false;
        } else {
            self.irq_counter -= 1;
        }

        // Then check if we need to trigger an IRQ
        if self.irq_counter == 0 && self.irq_enable {
            self.irq_active = true;
        }
    }

    fn cpu_read(&self, address: u16, mapped_addr: &mut u32, data: &mut u8) -> bool {
        if address >= 0x6000 && address <= 0x7FFF {
            // Read from static RAM on cartridge
            *mapped_addr = READFROMSRAM;
            // Read data from RAM
            *data = self.ram_static[(address & 0x1FFF) as usize];
            // Signal mapper has handled request
            return true;
        }

        if address >= 0x8000 {
            let idx_chunk = ((address - 0x8000) / 0x2000) as usize;
            *mapped_addr = self.prg_bank[idx_chunk] + (address & 0x1FFF) as u32;
            return true;
        }
        
        false
    }

    fn cpu_write(&mut self, address: u16, mapped_addr: &mut u32, data: u8) -> bool {
        if address >= 0x6000 && address <= 0x7FFF {
            // Write to static RAM on cartridge
            *mapped_addr = READFROMSRAM;
            // Write data to RAM
            self.ram_static[(address & 0x1FFF) as usize] = data;
            // Signal mapper has handled request
            return true;
        }

        if address >= 0x8000 && address <= 0x9FFF {
            // Bank Select
            if address & 0x0001 == 0 {  // even
                self.target_register = data & 0x07;
                self.prg_bank_mode = (data & 0x40) != 0;
                self.chr_inversion = (data & 0x80) != 0;
            } else {  // odd
                // Update target register
                self.registers[self.target_register as usize] = data as i32;
                self.update_bank_offset();
            }
            return true; // Fixed: Return true to indicate handled write
        }

        // Handle mirroring
        if address >= 0xA000 && address <= 0xBFFF {
            if address & 0x0001 == 0 {  // even
                // Mirroring
                if (data & 0x01) != 0 {
                    self.mirror_mode = Nametable::Horizontal;
                } else {
                    self.mirror_mode = Nametable::Vertical;
                }
            } else {  // odd
                // PRG RAM Protect - not implemented but handled
            }
            return true; // Fixed: Return true to indicate handled write
        }

        // Handle IRQ
        if address >= 0xC000 && address <= 0xDFFF {
            if address & 0x0001 == 0 {  // even
                self.irq_reload = data as u16;
            } else {  // odd
                self.irq_counter = 0;
                self.irq_update = true;
            }
            return true; // Fixed: Return true to indicate handled write
        }

        // Enable/Disable IRQ
        if address >= 0xE000 {
            if address & 0x0001 == 0 {  // even
                self.irq_enable = false;
                self.irq_active = false;
            } else {  // odd
                self.irq_enable = true;
            }
            return true; // Fixed: Return true to indicate handled write
        }

        false
    }

    fn ppu_read(&self, address: u16, mapped_addr: &mut u32, _data: u8) -> bool {
        if address <= 0x1FFF {
            let idx_chunk = (address / 0x0400) as usize;
            *mapped_addr = self.chr_bank[idx_chunk] + (address & 0x03FF) as u32;
            return true;
        }
        
        false
    }

    fn ppu_write(&mut self, address: u16, mapped_addr: &mut u32, _data: u8) -> bool {
        if address <= 0x1FFF {
            if self.chr_banks == 0 {
                // Only allow writes if using CHR RAM
                let idx_chunk = (address / 0x0400) as usize;
                *mapped_addr = self.chr_bank[idx_chunk] + (address & 0x03FF) as u32;
                return true;
            }
        }
        
        false
    }

    fn get_nametable(&self) -> Nametable {
        self.mirror_mode.clone()
    }

    fn savestate(&self) {
        // TODO: Implement savestate functionality
    }

    fn loadstate(&mut self) {
        // TODO: Implement loadstate functionality
    }

    fn hasirq(&mut self) -> bool {
        let test = self.irq_active;
        self.irq_active = false;
        return test;
    }
}