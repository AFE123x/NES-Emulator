pub mod frame;
mod registers;
use frame::Frame;
use registers::{PPUCTRL, PPUMASK, PPUSTATUS};

use crate::cartridge::Cartridge;

pub struct Ppu {
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
    cycle: i16,
    scanline: i16,
    main_frame: *mut Frame,
    frame_complete: bool,
    palette: Vec<u8>,
    indexboi: u16,
    enable_interrupt: bool,
}

impl Ppu {
    pub fn new(cart: &mut Cartridge, frame: &mut Frame) -> Self {
        Self {
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
            cycle: 0,
            main_frame: frame,
            scanline: 0,
            frame_complete: false,
            palette: vec![0; 0x20],
            indexboi: 0,
            enable_interrupt: false,
        }
    }
    pub fn get_frame_comp(&mut self) -> bool {
        let toreturn = self.frame_complete;
        if toreturn {
            self.frame_complete = false;
        }
        toreturn
    }

    pub fn get_enable_interrupt(&mut self) -> bool {
        let toreturn = self.enable_interrupt;
        if toreturn {
            self.enable_interrupt = false;
        }
        toreturn
    }

    pub fn ppu_read(&self, address: u16) -> u8 {
        let mut data = 0;
        if address <= 0x1FFF {
            unsafe { (*self.cart).ppu_read(address, &mut data) };
        } else if address <= 0x3EFF {
            data = self.vram[address as usize & 0x7FE];
        } else if address <= 0x3FFF {
            let temp_address = address & 0x1F;
            data = self.palette[temp_address as usize];
        } else {
            todo!()
        }
        data
    }
    pub fn ppu_write(&mut self, address: u16, byte: u8) {
        if address <= 0x1FFF {
            unsafe { (*self.cart).ppu_write(address, byte) };
        } else if address <= 0x3EFF {
            self.vram[address as usize & 0x7FE] = byte;
        } else if address <= 0x3FFF {
            //println!("Trying to write to address {:#x}", address);
            let temp_address = address & 0x1F;
            self.palette[temp_address as usize] = byte;
        } else {
            todo!()
        }
    }
    pub fn create_palette_table(&mut self) -> Frame {
        let mut palette_frame = Frame::new(128, 256); // 128x256 NES pattern table

        for r in 0..256 {
            for col in 0..128 {
                let addr = (r / 8 * 0x100) + (r % 8) + (col / 8) * 0x10;
                let lo = self.ppu_read(addr);
                let hi = self.ppu_read(addr + 8);
                let pixel_idx = ((hi >> (7 - (col % 8))) & 1) * 2 + ((lo >> (7 - (col % 8))) & 1);
                let color = match pixel_idx {
                    0 => (0, 0, 0),
                    1 => (255, 0, 0),
                    2 => (0, 255, 0),
                    3 => (0, 0, 255),
                    _ => (0, 0, 0),
                };
                palette_frame.drawpixel(col as u16, r as u16, color);
            }
        }

        palette_frame
    }

    pub fn cpu_read(&mut self, address: u16) -> u8 {
        let temp_addr = address & 0x7;
        match temp_addr {
            0x00 => 0,
            0x01 => self.ppumask.bits(),
            0x02 => {
                self.ppu_addr_index = 0;
                let data = self.ppustatus.bits();
                self.ppustatus.set(PPUSTATUS::vblank_flag, false);
                data
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
                self.ppu_data_buffer = self.ppu_read(self.ppuaddr);
                if address >= 0x3F00 {
                    data = self.ppu_data_buffer;
                }
                self.ppuaddr =
                    self.ppuaddr
                        .wrapping_add(if self.ppuctrl.contains(PPUCTRL::vram_increment) {
                            32
                        } else {
                            1
                        });

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
                self.ppu_write(self.ppuaddr, byte);
                self.ppuaddr = self.ppuaddr.wrapping_add(if self.ppuctrl.contains(PPUCTRL::vram_increment) {32} else {1});
            }
            _ => todo!(),
        }
    }
    pub fn get_lines(&self) -> (i16, i16) {
        (self.scanline, self.cycle)
    }
    pub fn create_static(&mut self) {
        for i in 0..256 {
            for j in 0..240 {
                unsafe {
                    (*self.main_frame).drawpixel(
                        i,
                        j,
                        if rand::random_bool(0.5) {
                            (0, 0, 0)
                        } else {
                            (255, 255, 255)
                        },
                    );
                }
            }
        }
    }

    pub fn create_name_table(&mut self) {
        println!("NAMETABLE START");
        for row in 0..30 {
            for col in 0..32 {
                let index: u16 = col + (row * 32);
                let index = 0x2400 + index;
                let paletteindex = self.ppu_read(index as u16) as u16;
                print!("{:2x}",paletteindex);
                let paletteaddr_lo = paletteindex << 4;
                let paletteaddr_hi = paletteaddr_lo | 0x8;
                for i in 0..7 {
                    let mut hi_value = self.ppu_read(paletteaddr_hi + i);
                    let mut lo_value = self.ppu_read(paletteaddr_lo + i);
                    for j in 0..7 {
                        let hi = hi_value & 1;
                        let lo = lo_value & 1;
                        let pixel = (hi << 1) | lo;
                        hi_value >>= 1;
                        lo_value >>= 1;
                        let color = match pixel {
                            0 => (0, 0, 0),
                            1 => (255, 0, 0),
                            2 => (0, 255, 0),
                            3 => (0, 0, 255),
                            _ => panic!("invalid color"),
                        };
                        unsafe {
                            (*self.main_frame).drawpixel(
                                (col * 8) as u16 + (8 - j),
                                (row * 8) + i,
                                color,
                            );
                        }
                    }
                }
            }
            println!("");
        }
        println!("NAMETABLE END");
    }
    pub fn clock(&mut self) {
        self.cycle = self.cycle.wrapping_add(1);
        if self.cycle > 340 {
            self.cycle = self.cycle.wrapping_sub(341);
            self.scanline = self.scanline.wrapping_add(1);
        }
        if 0 <= self.scanline && self.scanline <= 239 {
        } else if self.scanline == 241 && self.cycle == 1 {
            self.ppustatus.set(PPUSTATUS::vblank_flag, true);
            if self.ppuctrl.contains(PPUCTRL::vblank_enable) {
                self.enable_interrupt = true;
            }
            self.create_name_table();
            self.frame_complete = true;
        } else if self.scanline == 261 && self.cycle == 1 {
            self.ppustatus.set(PPUSTATUS::vblank_flag, false);
            self.frame_complete = false;
            self.scanline = 0;
        }
    }
}
