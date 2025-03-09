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
    cycle: i16,
    scanline: i16,
    main: *mut Fram,
    frame_completed: bool,
}

impl Ppu {
    pub fn new(cart: &mut Cartridge, main_frame: &mut Fram) -> Self {
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
            main: main_frame,
            scanline: 0,
            frame_completed: false,
        }
    }
    pub fn get_frame_com(&self) -> bool {
        self.frame_completed
    }

    pub fn reset_frame_com(&mut self) {
        self.frame_completed = false;
    }
    pub fn ppu_read(&self, address: u16) -> u8 {
        if address <= 0x1FFF {
            unsafe { (*self.cart).ppu_read(address) }
        }
        else if address <= 0x27FF{
            println!("VRAM");
            for i in &self.vram{
                println!("{}",i);
            }
            println!("END");
            self.vram[(address & 0x7FF) as usize]
        }
        else{
            todo!()
        }
    }

    pub fn ppu_write(&self, address: u16, byte: u8) {
        unsafe { (*self.cart).ppu_write(address, byte) }
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

                fr.drawpixel(col as u16, r as u16, color); //self.getpalettetable(4, color_idx)
            }
        }

        fr
    }
    fn getpalettetable(&self, palette: u8, pixel: u8) -> (u8, u8, u8) {
        const PAL_SCREEN: [(u8, u8, u8); 64] = [
            (84, 84, 84),
            (0, 30, 116),
            (8, 16, 144),
            (48, 0, 136),
            (68, 0, 100),
            (92, 0, 48),
            (84, 4, 0),
            (60, 24, 0),
            (32, 42, 0),
            (8, 58, 0),
            (0, 64, 0),
            (0, 60, 0),
            (0, 50, 60),
            (0, 0, 0),
            (0, 0, 0),
            (0, 0, 0),
            (152, 150, 152),
            (8, 76, 196),
            (48, 50, 236),
            (92, 30, 228),
            (136, 20, 176),
            (160, 20, 100),
            (152, 34, 32),
            (120, 60, 0),
            (84, 90, 0),
            (40, 114, 0),
            (8, 124, 0),
            (0, 118, 40),
            (0, 102, 120),
            (0, 0, 0),
            (0, 0, 0),
            (0, 0, 0),
            (236, 238, 236),
            (76, 154, 236),
            (120, 124, 236),
            (176, 98, 236),
            (228, 84, 236),
            (236, 88, 180),
            (236, 106, 100),
            (212, 136, 32),
            (160, 170, 0),
            (116, 196, 0),
            (76, 208, 32),
            (56, 204, 108),
            (56, 180, 204),
            (60, 60, 60),
            (0, 0, 0),
            (0, 0, 0),
            (236, 238, 236),
            (168, 204, 236),
            (188, 188, 236),
            (212, 178, 236),
            (236, 174, 236),
            (236, 174, 212),
            (236, 180, 176),
            (228, 196, 144),
            (204, 210, 120),
            (180, 222, 120),
            (168, 226, 144),
            (152, 226, 180),
            (160, 214, 228),
            (160, 162, 160),
            (0, 0, 0),
            (0, 0, 0),
        ];

        PAL_SCREEN[pixel as usize + palette as usize * 4 as usize]
    }
    pub fn cpu_read(&mut self, address: u16) -> u8 {
        let temp_addr = address & 0x7;
        match temp_addr {
            0x00 => 0,
            0x01 => self.ppumask.bits(),
            0x02 => {
                self.wlatch = 0;
                // self.ppustatus.set(PPUSTATUS::vblank_flag, true);
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
                //println!("{:#x}", self.ppuaddr);
                self.ppu_data_buffer = self.ppu_read(self.ppuaddr);
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
                    let temp = (byte as u16) << 8;

                    self.ppuaddr = temp;
                    self.ppu_addr_index = 1;
                } else {
                    self.ppuaddr = self.ppuaddr | byte as u16;
                    //println!("{:#x}", self.ppuaddr);
                    self.ppu_addr_index = 0;
                }
            }
            0x07 => {
                self.ppu_write(self.ppuaddr, byte);
            }
            _ => todo!(),
        }
    }

    pub fn clock(&mut self) {
        if self.cycle < 255 && self.scanline < 240 {
            unsafe {
                (*self.main).drawpixel(
                    self.cycle as u16,
                    self.scanline as u16,
                    if rand::random_bool(0.5) {
                        (0, 0, 0)
                    } else {
                        (255, 255, 255)
                    },
                );
            }
        }

        self.cycle = self.cycle.wrapping_add(1);
        if self.cycle == 341 {
            self.cycle = 0;
            self.scanline = self.scanline.wrapping_add(1);
            if self.scanline > 240 {
                self.frame_completed = true;
                self.ppuctrl.insert(PPUCTRL::vblank_enable);
                self.ppustatus.set(PPUSTATUS::vblank_flag, true);
            }
            if self.scanline == 261 {
                self.ppuctrl.set(PPUCTRL::vblank_enable, false);
                self.ppustatus.set(PPUSTATUS::vblank_flag, false);
                self.scanline = 0;
            }
        }
    }
}
