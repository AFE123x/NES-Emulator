use super::{mapper::Mapper, MirrorMode};

pub struct Mapper004 {
    n_prgbanks: u8,
    n_chrbanks: u8, // Added to match Go implementation

    n_target_register: u8,
    b_prgbank_mode: bool,
    b_chrinversion: bool,

    mirrormode: MirrorMode,
    p_register: [i32; 8],
    p_chrbank: [u32; 8],
    p_prgbank: [u32; 4],

    n_irqreload: u16, // Changed to u16 to match Go implementation
    n_irqcounter: u16,
    b_irqenable: bool,
    b_irqactive: bool,
    b_irqupdate: bool,

    ram: Vec<u8>,
    
    // For A12 detection
    last_a12_state: bool,
}

impl Mapper004 {
    pub fn new(prg_rom_size: u8, chr_rom_size: u8) -> Self {
        let mut mapper = Self {
            n_prgbanks: prg_rom_size,
            n_chrbanks: chr_rom_size, // Store CHR ROM size

            n_target_register: 0,
            b_prgbank_mode: false,
            b_chrinversion: false,
            mirrormode: MirrorMode::Horizontal,
            p_register: [0; 8],
            p_chrbank: [0; 8],
            p_prgbank: [0; 4],
            ram: vec![0; 32 * 1024],
            n_irqreload: 0,
            n_irqcounter: 0,
            b_irqenable: false,
            b_irqactive: false,
            b_irqupdate: false,
            last_a12_state: false,
        };
        mapper.reset();
        mapper
    }

    // Helper methods for bank offsets, matching Go implementation
    fn prg_bank_offset(&self, index: i32) -> u32 {
        let mut adjusted_index = index;
        if adjusted_index >= 0x80 {
            adjusted_index -= 0x100;
        }
        adjusted_index %= (self.n_prgbanks as i32) * 2;
        
        let mut offset = adjusted_index * 0x2000;
        if offset < 0 {
            offset += (self.n_prgbanks as i32) * 16 * 1024;
        }
        offset as u32
    }

    fn chr_bank_offset(&self, index: i32) -> u32 {
        let mut adjusted_index = index;
        if adjusted_index >= 0x80 {
            adjusted_index -= 0x100;
        }
        
        // Use CHR banks or fall back to 1 if none
        let actual_chr_banks = if self.n_chrbanks == 0 { 1 } else { self.n_chrbanks };
        adjusted_index %= (actual_chr_banks as i32) * 8;
        
        let mut offset = adjusted_index * 0x0400;
        if offset < 0 {
            offset += (actual_chr_banks as i32) * 8 * 1024;
        }
        offset as u32
    }

    fn update_bank_offset(&mut self) {
        // Update CHR banks based on CHR inversion
        if self.b_chrinversion {
            self.p_chrbank[0] = self.chr_bank_offset(self.p_register[2]);
            self.p_chrbank[1] = self.chr_bank_offset(self.p_register[3]);
            self.p_chrbank[2] = self.chr_bank_offset(self.p_register[4]);
            self.p_chrbank[3] = self.chr_bank_offset(self.p_register[5]);
            self.p_chrbank[4] = self.chr_bank_offset(self.p_register[0] & 0xFE);
            self.p_chrbank[5] = self.chr_bank_offset(self.p_register[0] | 0x01);
            self.p_chrbank[6] = self.chr_bank_offset(self.p_register[1] & 0xFE);
            self.p_chrbank[7] = self.chr_bank_offset(self.p_register[1] | 0x01);
        } else {
            self.p_chrbank[0] = self.chr_bank_offset(self.p_register[0] & 0xFE);
            self.p_chrbank[1] = self.chr_bank_offset(self.p_register[0] | 0x01);
            self.p_chrbank[2] = self.chr_bank_offset(self.p_register[1] & 0xFE);
            self.p_chrbank[3] = self.chr_bank_offset(self.p_register[1] | 0x01);
            self.p_chrbank[4] = self.chr_bank_offset(self.p_register[2]);
            self.p_chrbank[5] = self.chr_bank_offset(self.p_register[3]);
            self.p_chrbank[6] = self.chr_bank_offset(self.p_register[4]);
            self.p_chrbank[7] = self.chr_bank_offset(self.p_register[5]);
        }

        // Update PRG banks based on PRG bank mode
        if self.b_prgbank_mode {
            self.p_prgbank[0] = self.prg_bank_offset(-2);
            self.p_prgbank[2] = self.prg_bank_offset(self.p_register[6]);
        } else {
            self.p_prgbank[0] = self.prg_bank_offset(self.p_register[6]);
            self.p_prgbank[2] = self.prg_bank_offset(-2);
        }

        self.p_prgbank[1] = self.prg_bank_offset(self.p_register[7]);
        self.p_prgbank[3] = self.prg_bank_offset(-1); // Last 8K bank
    }
}

impl Mapper for Mapper004 {
    fn cpu_read(&self, address: u16, mapped_addr: &mut u32, data: &mut u8) -> bool {
        if address >= 0x6000 && address <= 0x7FFF {
            *mapped_addr = 0xFFFFFFFF;
            *data = self.ram[(address as usize) & 0x1FFF];
            return true;
        }

        if address >= 0x8000  {
            let idx_chunk = (address - 0x8000) / 0x2000;
            *mapped_addr = self.p_prgbank[idx_chunk as usize] + ((address as u32) & 0x1FFF);
            return true;
        }

        false
    }

    fn cpu_write(&mut self, address: u16, mapped_addr: &mut u32, data: u8) -> bool {
        if address >= 0x6000 && address <= 0x7FFF {
            *mapped_addr = 0xFFFFFFFF;
            self.ram[(address as usize) & 0x1FFF] = data;
            return true;
        }

        if address >= 0x8000 && address <= 0x9FFF {
            if address & 0x1 == 0 {
                self.n_target_register = data & 0x7;
                self.b_prgbank_mode = data & 0x40 != 0;
                self.b_chrinversion = data & 0x80 != 0;
            } else {
                self.p_register[self.n_target_register as usize] = data as i32;
                self.update_bank_offset();
            }
            return false;
        }

        if address >= 0xA000 && address <= 0xBFFF {
            if address & 1 == 0 {
                if data & 1 != 0 {
                    self.mirrormode = MirrorMode::Horizontal;
                } else {
                    self.mirrormode = MirrorMode::Vertical;
                }
            } else {
                // PRG Ram Protect - Marked as TODO in both implementations
            }
            return false;
        }

        if address >= 0xC000 && address <= 0xDFFF {
            if address & 0x1 == 0 {
                self.n_irqreload = data as u16;
            } else {
                self.n_irqcounter = 0;
            }
            return false;
        }

        if address >= 0xE000 {
            if address & 1 == 0 {
                self.b_irqenable = false;
                self.b_irqactive = false;
            } else {
                self.b_irqenable = true;
            }
            return false;
        }

        false
    }

    fn ppu_read(&mut self, address: u16, mapped_addr: &mut u32, _data: u8) -> bool {
        if address <= 0x1FFF {
            let idx_chunk = address / 0x400;
            *mapped_addr = self.p_chrbank[idx_chunk as usize] + ((address as u32) & 0x03FF);
            return true;
        }
        false
    }

    fn ppu_write(&mut self, address: u16, mapped_addr: &mut u32, _data: u8) -> bool {
        if address <= 0x1FFF {
            if self.n_chrbanks == 0 {
                let idx_chunk = address / 0x400;
                *mapped_addr = self.p_chrbank[idx_chunk as usize] + ((address as u32) & 0x03FF);
                return true;
            }
        }
        false
    }

    fn get_mirror_mode(&self) -> MirrorMode {
        self.mirrormode.clone()
    }

    fn irq_clear(&mut self) {
        self.b_irqactive = false;
    }

    fn hasirq(&mut self) -> bool {
        self.b_irqactive
    }

    // Updated scanline method to match Go implementation
    fn scanline(&mut self) {
        if self.n_irqcounter == 0 {
            self.n_irqcounter = self.n_irqreload;
        } else {
            self.n_irqcounter -= 1;
        }

        if self.n_irqcounter == 0 && self.b_irqenable {
            self.b_irqactive = true;
        }
    }

    // Implement A12 detection for IRQ counting
    fn ppu_access(&mut self, address: u16) {
        // MMC3 IRQ is triggered by specific pattern on A12 address line
        if address <= 0x1FFF {
            let current_a12_state = (address & 0x1000) != 0;
            
            // Detect rising edge on A12 (transition from 0 to 1)
            if !self.last_a12_state && current_a12_state {
                if self.n_irqcounter == 0 || self.b_irqupdate {
                    self.n_irqcounter = self.n_irqreload;
                    self.b_irqupdate = false;
                } else {
                    self.n_irqcounter -= 1;
                }

                if self.n_irqcounter == 0 && self.b_irqenable {
                    self.b_irqactive = true;
                }
            }
            
            self.last_a12_state = current_a12_state;
        }
    }

    fn reset(&mut self) {
        self.n_target_register = 0;
        self.b_prgbank_mode = false;
        self.b_chrinversion = false;
        self.mirrormode = MirrorMode::Horizontal;

        self.b_irqactive = false;
        self.b_irqenable = false;
        self.b_irqupdate = false;
        self.n_irqcounter = 0;
        self.n_irqreload = 0;
        self.last_a12_state = false;

        // Clear registers
        for reg in self.p_register.iter_mut() {
            *reg = 0;
        }

        // Setup default banking - match the Go implementation
        self.p_prgbank[0] = 0 * 0x2000;
        self.p_prgbank[1] = 1 * 0x2000;
        self.p_prgbank[2] = ((self.n_prgbanks as u32) * 2 - 2) * 0x2000;
        self.p_prgbank[3] = ((self.n_prgbanks as u32) * 2 - 1) * 0x2000;
    }

    fn savestate(&self) {}
    fn loadstate(&mut self) {}
    fn step_m2(&mut self, _cpu_clock: u64) {}
}