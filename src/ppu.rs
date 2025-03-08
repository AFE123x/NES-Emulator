pub mod Frame;
mod registers;
use registers::{PPUCTRL, PPUMASK, PPUSTATUS};
use Frame::Frame as Fram;

use crate::cartridge::Cartridge;

pub struct Ppu {
    pattern_table: Vec<Vec<u8>>,
    ppuctrl: PPUCTRL,
    ppumask: PPUMASK,
    ppustatus: PPUSTATUS,
    oamaddr: u8,
    oamdata: u8,
    ppuscroll: u8,
    ppuaddr: u16,
    ppudata: u8,
    oamdma: u8,
    vlatch: u16,
    tlatch: u16,
    xreg: u16,
    wlatch: u16,
    cart: *mut Cartridge,
    vram: Vec<u8>,
    ppu_scroll_index: u8,
    scroll_x: u8,
    scroll_y: u8,
    ppu_addr_index: u8,
    ppu_data_buffer: u8,
    cached_table: bool,
    cycle: u16,
}

impl Ppu {
    pub fn new(cart: &mut Cartridge) -> Self {
        Self {
            pattern_table: vec![vec![0; 128 as usize * 128 as usize]; 2],
            ppuctrl: PPUCTRL::empty(),
            ppumask: PPUMASK::empty(),
            ppustatus: PPUSTATUS::empty(),
            oamaddr: 0,
            oamdata: 0,
            ppuscroll: 0,
            ppuaddr: 0,
            ppudata: 0,
            oamdma: 0,
            vlatch: 0,
            tlatch: 0,
            xreg: 0,
            wlatch: 0,
            cart: cart,
            vram: vec![0; 2048],
            ppu_scroll_index: 0,
            scroll_x: 0,
            scroll_y: 0,
            ppu_addr_index: 0,
            ppu_data_buffer: 0,
            cached_table: false,
            cycle: 0,
        }
    }
    pub fn ppu_read(&self, address: u16) -> u8 {
        unsafe { (*self.cart).ppu_read(address) }
    }
    pub fn create_palette_table(&mut self) -> Fram {
        let mut fr = Fram::new(128, 256); // 128x256 NES pattern table

        for r in 0..256 {
            for col in 0..128 {
                let addr = (r / 8 * 0x100) + (r % 8) + (col / 8) * 0x10;
                let lo = self.ppu_read(addr);
                let hi = self.ppu_read(addr + 8);
                let pixel_idx = ((hi >> (7 - (col % 8))) & 1) * 2 + ((lo >> (7 - (col % 8))) & 1);

                if r < 128 {
                    self.pattern_table[0][((r as usize) << 7) + col as usize] = pixel_idx;
                } else {
                    self.pattern_table[1][(((r as usize) - 128) << 7) + col as usize] = pixel_idx;
                }
            }
        }

        for r in 0..256 {
            for col in 0..128 {
                let table_idx = if r < 128 { 0 } else { 1 };
                let color_idx = self.pattern_table[table_idx][((r % 128) << 7) + col];

                let color = match color_idx {
                    0 => (0, 0, 0),
                    1 => (255, 0, 0),
                    2 => (0, 255, 0),
                    3 => (0, 0, 255),
                    _ => (0, 0, 0),
                };

                fr.drawpixel(col as u16, r as u16, color);
            }
        }

        fr
    }

    pub fn cpu_read(&mut self, address: u16) -> u8 {
        let temp_addr = address & 0x7;
        match temp_addr {
            0x00 => 0,
            0x01 => self.ppumask.bits(),
            0x02 => {
                self.wlatch = 0;
                self.ppustatus.set(PPUSTATUS::vblank_flag, true);
                self.ppustatus.bits()
            }
            0x03 => 0,
            0x04 => {
                let tdata = self.vram[self.oamaddr as usize];
                self.oamaddr = self.oamaddr.wrapping_add(1);
                tdata
            }
            0x05 => 0,
            0x06 => 0,
            0x07 => {
                let mut data = self.ppu_data_buffer;
                self.ppu_data_buffer = self.vram[self.ppuaddr as usize];
                self.ppuaddr =
                    self.ppuaddr
                        .wrapping_add(if self.ppuctrl.contains(PPUCTRL::vram_increment) {
                            32
                        } else {
                            1
                        });
                if address >= 0x3F00 {
                    data = self.ppu_data_buffer;
                }
                data
            }
            _ => {
                todo!()
            }
        }
    }

    pub fn cpu_write(&mut self, address: u16, byte: u8) {
        let temp_addr = address & 0x7;
        match temp_addr {
            0x00 => {
                self.ppuctrl = PPUCTRL::from_bits_truncate(byte);
            }
            0x01 => {
                self.ppumask = PPUMASK::from_bits_truncate(byte);
            }
            0x02 => { /* nothing happens */ }
            0x03 => {
                self.oamaddr = byte;
            }
            0x04 => {
                self.vram[self.oamaddr as usize] = byte;
            }
            0x05 => {
                /* Scroll register */
                if self.ppu_scroll_index == 0 {
                    self.scroll_x = byte;
                    self.scroll_x = if self.ppuctrl.bits() & 0x1 != 0 {
                        self.scroll_x | 0x80
                    } else {
                        self.scroll_x
                    };
                    self.ppu_scroll_index = 1;
                } else {
                    self.scroll_y = byte;
                    self.scroll_y = if self.ppuctrl.bits() & 0x2 != 0 {
                        self.scroll_x | 0x80
                    } else {
                        self.scroll_x
                    };
                    self.ppu_scroll_index = 0;
                }
            }
            0x06 => {
                if self.ppu_addr_index == 0 {
                    self.ppuaddr = (byte as u16) << 8;
                    self.ppu_addr_index = 1;
                } else {
                    self.ppuaddr = self.ppuaddr | byte as u16;
                    self.ppu_addr_index = 0;
                }
            }
            0x07 => {
                self.vram[self.ppuaddr as usize] = byte;
            }
            _ => todo!(),
        }
    }

    pub fn clock() {

    }
}
