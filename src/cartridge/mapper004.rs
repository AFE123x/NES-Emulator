use super::{mapper::Mapper, MirrorMode};

pub struct Mapper004 {
    n_prgbanks: u8,
    n_chrbanks: u8,

    n_target_register: u8,
    b_prgbank_mode: bool,
    b_chrinversion: bool,

    mirrormode: MirrorMode,
    p_register: [i32; 8],
    p_chrbank: [u32; 8],
    p_prgbank: [u32; 4],

    b_irqactive: bool,
    b_irqenable: bool,
    b_irqupdate: bool,

    n_irqcounter: u16,
    n_irqreload: u16,

    ram: Vec<u8>,
}

impl Mapper004 {
    pub fn new(prg_rom_size: u8, chr_rom_size: u8) -> Self {
        let mut mapper = Self {
            n_prgbanks: prg_rom_size,
            n_chrbanks: chr_rom_size,
            n_target_register: 0,
            b_prgbank_mode: false,
            b_chrinversion: false,
            mirrormode: MirrorMode::Horizontal,
            p_register: [0; 8],
            p_chrbank: [0; 8],
            p_prgbank: [0; 4],
            b_irqactive: false,
            b_irqenable: false,
            b_irqupdate: false,
            n_irqcounter: 0,
            n_irqreload: 0,
            ram: vec![0; 8192],
        };
        mapper.reset();
        mapper
    }
    
    fn prg_bank_offset(&self, index: i32) -> u32 {
        let mut index = index;
        // Handle negative indices
        if index >= 0x80 {
            index -= 0x100;
        }
        
        // Ensure index is within valid range
        index %= (self.n_prgbanks as i32) * 2;
        
        let mut offset = index * 0x2000;
        
        // Handle negative offset
        if offset < 0 {
            offset += (self.n_prgbanks as i32) * 16 * 1024;
        }
        
        offset as u32
    }

    fn chr_bank_offset(&self, index: i32) -> u32 {
        let mut index = index;
        // Handle negative indices
        if index >= 0x80 {
            index -= 0x100;
        }
        
        // Handle case where n_chrbanks is 0 (CHR RAM)
        let n_actual_chrbank = if self.n_chrbanks == 0 { 1 } else { self.n_chrbanks };
        
        // Ensure index is within valid range
        index %= (n_actual_chrbank as i32) * 8;
        
        let mut offset = index * 0x400; // 1KB banks
        
        // Handle negative offset
        if offset < 0 {
            offset += (n_actual_chrbank as i32) * 8 * 1024;
        }
        
        offset as u32
    }
    
    fn update_bank_offset(&mut self) {
        // Update CHR banks based on inversion flag
        if self.b_chrinversion {
            // CHR Inversion Mode 1 (PPU $0000-$0FFF 4KB switchable, $1000-$1FFF 4KB switchable)
            self.p_chrbank[0] = self.chr_bank_offset(self.p_register[2]);
            self.p_chrbank[1] = self.chr_bank_offset(self.p_register[3]);
            self.p_chrbank[2] = self.chr_bank_offset(self.p_register[4]);
            self.p_chrbank[3] = self.chr_bank_offset(self.p_register[5]);
            self.p_chrbank[4] = self.chr_bank_offset(self.p_register[0] & 0xFE);
            self.p_chrbank[5] = self.chr_bank_offset(self.p_register[0] | 0x01);
            self.p_chrbank[6] = self.chr_bank_offset(self.p_register[1] & 0xFE);
            self.p_chrbank[7] = self.chr_bank_offset(self.p_register[1] | 0x01);
        } else {
            // CHR Inversion Mode 0 (PPU $0000-$0FFF 2x2KB, $1000-$1FFF 4x1KB)
            self.p_chrbank[0] = self.chr_bank_offset(self.p_register[0] & 0xFE);
            self.p_chrbank[1] = self.chr_bank_offset(self.p_register[0] | 0x01);
            self.p_chrbank[2] = self.chr_bank_offset(self.p_register[1] & 0xFE);
            self.p_chrbank[3] = self.chr_bank_offset(self.p_register[1] | 0x01);
            self.p_chrbank[4] = self.chr_bank_offset(self.p_register[2]);
            self.p_chrbank[5] = self.chr_bank_offset(self.p_register[3]);
            self.p_chrbank[6] = self.chr_bank_offset(self.p_register[4]);
            self.p_chrbank[7] = self.chr_bank_offset(self.p_register[5]);
        }

        // Update PRG banks based on mode
        if self.b_prgbank_mode {
            // PRG Mode 1: Swap $8000 with second-last bank
            self.p_prgbank[0] = self.prg_bank_offset(-2);
            self.p_prgbank[2] = self.prg_bank_offset(self.p_register[6]);
        } else {
            // PRG Mode 0: $8000 switchable, $C000 fixed to second-last bank
            self.p_prgbank[0] = self.prg_bank_offset(self.p_register[6]);
            self.p_prgbank[2] = self.prg_bank_offset(-2);
        }

        // These banks are the same regardless of mode
        self.p_prgbank[1] = self.prg_bank_offset(self.p_register[7]);
        self.p_prgbank[3] = self.prg_bank_offset(-1); // Last bank is always fixed
    }
    
    
}

impl Mapper for Mapper004 {
    fn irq_clear(&mut self) {
        self.b_irqactive = false;
    }
    fn cpu_read(&self, address: u16, mapped_addr: &mut u32, data: &mut u8) -> bool {
        if address >= 0x6000 && address <= 0x7FFF {
            // Reading from static RAM
            *mapped_addr = 0xFFFFFFFF; // Special flag for SRAM
            *data = self.ram[(address & 0x1FFF) as usize];
            return true;
        }
        
        if address >= 0x8000 {
            // Reading from PRG ROM
            let idx_chunk = ((address - 0x8000) / 0x2000) as usize;
            *mapped_addr = self.p_prgbank[idx_chunk] + ((address & 0x1FFF) as u32);
            return true;
        }
        
        false
    }
    
    fn cpu_write(&mut self, address: u16, mapped_addr: &mut u32, data: u8) -> bool {
        if address >= 0x6000 && address <= 0x7FFF {
            // Writing to static RAM
            *mapped_addr = 0xFFFFFFFF; // Special flag for SRAM
            self.ram[(address & 0x1FFF) as usize] = data;
            return true;
        }
        
        if address >= 0x8000 && address <= 0x9FFF {
            // Bank select and bank data
            if address & 0x0001 == 0 {
                // Even: Select register
                self.n_target_register = data & 0x07;
                self.b_prgbank_mode = (data & 0x40) != 0;
                self.b_chrinversion = (data & 0x80) != 0;
            } else {
                // Odd: Set register data
                self.p_register[self.n_target_register as usize] = data as i32;
                self.update_bank_offset();
            }
            return false;
        }

        if address >= 0xA000 && address <= 0xBFFF {
            if address & 0x0001 == 0 {
                // Even: Mirroring
                if data & 0x01 != 0 {
                    self.mirrormode = MirrorMode::Horizontal;
                } else {
                    self.mirrormode = MirrorMode::Vertical;
                }
            } else {
                // Odd: PRG RAM protect (not implemented)
            }
            return false;
        }

        if address >= 0xC000 && address <= 0xDFFF {
            if address & 0x0001 == 0 {
                // Even: IRQ latch
                self.n_irqreload = data as u16;
            } else {
                // Odd: IRQ reload
                self.n_irqcounter = 0; // This will be reloaded on next scanline
            }
            return false;
        }
        
        if address >= 0xE000 {
            if address & 0x0001 == 0 {
                // Even: IRQ disable
                self.b_irqenable = false;
                self.b_irqactive = false;
            } else {
                // Odd: IRQ enable
                self.b_irqenable = true;
            }
            return false;
        }

        false
    }

    fn ppu_read(&mut self, address: u16, mapped_addr: &mut u32, _data: u8) -> bool {
        if address <= 0x1FFF {
            // Calculate which 1KB bank to use
            let idx_chunk = (address / 0x400) as usize;
            // Each chunk is 1KB (0x400 bytes)
            *mapped_addr = self.p_chrbank[idx_chunk] + ((address & 0x03FF) as u32);
            return true;
        }
        false
    }

    fn ppu_write(&mut self, address: u16, mapped_addr: &mut u32, _data: u8) -> bool {
        if address <= 0x1FFF {
            // Only allow writes if using CHR RAM (n_chrbanks == 0)
            if self.n_chrbanks == 0 {
                let idx_chunk = (address / 0x400) as usize;
                *mapped_addr = self.p_chrbank[idx_chunk] + ((address & 0x03FF) as u32);
                return true;
            }
        }
        false
    }

    fn get_mirror_mode(&self) -> MirrorMode {
        self.mirrormode.clone()
    }

    fn savestate(&self) {
        todo!()
    }

    fn loadstate(&mut self) {
        todo!()
    }

    fn hasirq(&mut self) -> bool {
        // Test and clear the IRQ flag
        let irq_status = self.b_irqactive;
        self.b_irqactive = false;
        irq_status
    }

    fn scanline(&mut self) {
        // Handle scanline counter for IRQs
        if self.n_irqcounter == 0 {
            // Counter is 0, reload from latch
            self.n_irqcounter = self.n_irqreload;
        } else {
            // Decrement counter
            self.n_irqcounter = self.n_irqcounter.wrapping_sub(1);
        }
        
        // FIX: Activate IRQ if counter reaches 0 and IRQs are enabled
        if self.n_irqcounter == 0 && self.b_irqenable {
            self.b_irqactive = true;
        }
    }

    fn reset(&mut self) {
        // Reset mapper registers
        self.n_target_register = 0;
        self.b_prgbank_mode = false;
        self.b_chrinversion = false;
        self.mirrormode = MirrorMode::Horizontal;

        // Reset IRQ state
        self.b_irqactive = false;
        self.b_irqenable = false;
        self.b_irqupdate = false;
        self.n_irqcounter = 0;
        self.n_irqreload = 0;
        
        // Reset all registers to 0
        for i in 0..8 {
            self.p_register[i] = 0;
        }
        
        // Clear bank mappings
        for i in 0..4 {
            self.p_prgbank[i] = 0;
        }
        
        for i in 0..8 {
            self.p_chrbank[i] = 0;
        }
        
        // Setup default PRG banking
        // - First bank switchable
        // - Second bank points to third bank
        // - Third bank points to second-last bank
        // - Fourth bank points to last bank
        self.p_prgbank[0] = 0 * 0x2000;                           // First 8KB bank
        self.p_prgbank[1] = 1 * 0x2000;                           // Second 8KB bank
        self.p_prgbank[2] = ((self.n_prgbanks as u32) * 2 - 2) * 0x2000; // Second-last 8KB bank
        self.p_prgbank[3] = ((self.n_prgbanks as u32) * 2 - 1) * 0x2000; // Last 8KB bank
    }
    
    fn step_m2(&mut self, _cpu_clock: u64) {
        // Implementation can be left empty as in the original
        // or could be used for IRQ timing if needed
    }
    
    fn ppu_access(&mut self, _address: u16) {
        // Implementation can be left empty as in the original
        // Could be used for PPU-specific operations if needed
    }
}