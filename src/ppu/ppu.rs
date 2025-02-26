use sdl2::pixels::Palette;

use crate::{
    cartridge::cartridge::{Cartridge, Mirror},
    frame::frame::Frame,
};

pub struct Ppu {
    ppuctrl: u8,
    ppumask: u8,
    ppustatus: u8,
    oamaddr: u8,
    oamdata: u8,
    ppuscroll: u8,
    ppuaddr: u8,
    ppudata: u8,
    oamdma: u8,
    ppu_buffer: u8,
    address_latch: u8,
    vram_addr: u16,
    tram_addr: u16,
    fine_x: u8,
    cartridge: *mut Cartridge,
    tbl_pattern: Vec<Vec<u8>>,
    tbl_name: Vec<Vec<u8>>,
    tbl_palette: Vec<u8>,
    system_palette: Vec<(u8, u8, u8)>,
}

impl Ppu {
    fn get_sys_palette() -> Vec<(u8, u8, u8)> {
        let toreturn: Vec<(u8, u8, u8)> = vec![
            (0x80, 0x80, 0x80),
            (0x00, 0x3D, 0xA6),
            (0x00, 0x12, 0xB0),
            (0x44, 0x00, 0x96),
            (0xA1, 0x00, 0x5E),
            (0xC7, 0x00, 0x28),
            (0xBA, 0x06, 0x00),
            (0x8C, 0x17, 0x00),
            (0x5C, 0x2F, 0x00),
            (0x10, 0x45, 0x00),
            (0x05, 0x4A, 0x00),
            (0x00, 0x47, 0x2E),
            (0x00, 0x41, 0x66),
            (0x00, 0x00, 0x00),
            (0x05, 0x05, 0x05),
            (0x05, 0x05, 0x05),
            (0xC7, 0xC7, 0xC7),
            (0x00, 0x77, 0xFF),
            (0x21, 0x55, 0xFF),
            (0x82, 0x37, 0xFA),
            (0xEB, 0x2F, 0xB5),
            (0xFF, 0x29, 0x50),
            (0xFF, 0x22, 0x00),
            (0xD6, 0x32, 0x00),
            (0xC4, 0x62, 0x00),
            (0x35, 0x80, 0x00),
            (0x05, 0x8F, 0x00),
            (0x00, 0x8A, 0x55),
            (0x00, 0x99, 0xCC),
            (0x21, 0x21, 0x21),
            (0x09, 0x09, 0x09),
            (0x09, 0x09, 0x09),
            (0xFF, 0xFF, 0xFF),
            (0x0F, 0xD7, 0xFF),
            (0x69, 0xA2, 0xFF),
            (0xD4, 0x80, 0xFF),
            (0xFF, 0x45, 0xF3),
            (0xFF, 0x61, 0x8B),
            (0xFF, 0x88, 0x33),
            (0xFF, 0x9C, 0x12),
            (0xFA, 0xBC, 0x20),
            (0x9F, 0xE3, 0x0E),
            (0x2B, 0xF0, 0x35),
            (0x0C, 0xF0, 0xA4),
            (0x05, 0xFB, 0xFF),
            (0x5E, 0x5E, 0x5E),
            (0x0D, 0x0D, 0x0D),
            (0x0D, 0x0D, 0x0D),
            (0xFF, 0xFF, 0xFF),
            (0xA6, 0xFC, 0xFF),
            (0xB3, 0xEC, 0xFF),
            (0xDA, 0xAB, 0xEB),
            (0xFF, 0xA8, 0xF9),
            (0xFF, 0xAB, 0xB3),
            (0xFF, 0xD2, 0xB0),
            (0xFF, 0xEF, 0xA6),
            (0xFF, 0xF7, 0x9C),
            (0xD7, 0xE8, 0x95),
            (0xA6, 0xED, 0xAF),
            (0xA2, 0xF2, 0xDA),
            (0x99, 0xFF, 0xFC),
            (0xDD, 0xDD, 0xDD),
            (0x11, 0x11, 0x11),
            (0x11, 0x11, 0x11),
        ];
        toreturn
    }
    pub fn new(cartridge: &mut Cartridge) -> Self {
        Self {
            ppuctrl: 0,
            ppumask: 0,
            ppustatus: 0,
            oamaddr: 0,
            oamdata: 0,
            ppuscroll: 0,
            ppuaddr: 0,
            ppudata: 0,
            oamdma: 0,
            ppu_buffer: 0,
            address_latch: 0,
            vram_addr: 0,
            tram_addr: 0,
            fine_x: 0,
            cartridge: cartridge,
            tbl_pattern: vec![vec![0; 4096]; 2],
            tbl_name: vec![vec![0; 1024]; 2],
            tbl_palette: vec![0; 32],
            system_palette: Ppu::get_sys_palette(),
            
        }
    }
    pub fn make_pallet_table(&mut self, index: u8, palette: u8) -> Frame {
        let mut buffer = Frame::new(128, 128);
        for y in 0..16 {
            for x in 0..16 {
                let offset: u16 = x * 16 + y * 256;
                for row in 0..8 {
                    let mut tile_lsb =
                        self.ppu_read(index as u16 * 0x1000 + offset + row + 0x0000, true);
                    let mut tile_msb =
                        self.ppu_read(index as u16 * 0x1000 + offset + row + 0x0008, true);
                    for col in 0..8 {
                        let pixel = (tile_lsb & 0x1) | ((tile_msb & 0x01) << 1);
                        tile_lsb >>= 1;
                        tile_msb >>= 1;
                        let color_index = (*self).ppu_read(0x3F00 + ((palette as u16) << 2) + pixel as u16, true);
                        let color = self.system_palette[color_index as usize];
                        buffer.draw_pixel(
                            x as usize & 8 + (7 - col),
                            y as usize * 8 + row as usize,
                            color
                        );
                    }
                }
            }
        }
        buffer
    }
    pub fn ppu_read(&mut self, address: u16, readonly: bool) -> u8 {
        let mut data = 0;
        let success = unsafe { (*self.cartridge).ppu_read(address, &mut data) };
        if success {
        } else if address <= 0x1FFF {
            data = self.tbl_pattern[(address as usize & 0x1000) >> 12][address as usize & 0xFFF];
        } else if address >= 0x2000 && address <= 0x3EFF {
            let mirror = unsafe { (*self.cartridge).get_mirror() };
            match mirror {
                Mirror::VERTICAL => {
                    if address <= 0x3FF {
                        data = self.tbl_name[0][address as usize & 0x3FF];
                    } else if address >= 0x400 && address <= 0x7FF {
                        data = self.tbl_name[1][address as usize & 0x3FF];
                    } else if address >= 0x800 && address <= 0xBFF {
                        data = self.tbl_name[0][address as usize & 0x3FF];
                    } else if address >= 0xC00 && address <= 0xFFF {
                        data = self.tbl_name[1][address as usize & 0x3FF];
                    }
                }
                Mirror::HORIZONTAL => {
                    if address <= 0x3FF {
                        data = self.tbl_name[0][address as usize & 0x3FF];
                    } else if address >= 0x400 && address <= 0x7FF {
                        data = self.tbl_name[0][address as usize & 0x3FF];
                    } else if address >= 0x800 && address <= 0xBFF {
                        data = self.tbl_name[1][address as usize & 0x3FF];
                    } else if address >= 0xC00 && address <= 0xFFF {
                        data = self.tbl_name[1][address as usize & 0x3FF];
                    }
                }
            };
        } else if address >= 0x3F00 && address <= 0x3FFF {
            let address = address & 0x1F;
            let address = match address {
                0x10 => 0x0,
                0x14 => 0x4,
                0x18 => 0x8,
                0x1C => 0xC,
                _ => address,
            };
            let temp = self.tbl_palette[address as usize];
            let and_mask = if self.ppumask & 1 != 0 { 0x30 } else { 0x3F };
            data = temp & and_mask;
        }
        data
    }

    pub fn ppu_write(&mut self, address: u16, data: u8) {
        let address = address & 0x3FFF;

        let success = unsafe { (*self.cartridge).ppu_write(address, data) };
        if success {
        } else if address <= 0x1FFF {
            self.tbl_pattern[(address as usize & 0x1000) >> 12][address as usize & 0xFFF] = data;
        } else if address >= 0x2000 && address <= 0x3EFF {
            let address = address & 0xFFF;
            let mirror = unsafe { (*self.cartridge).get_mirror() };
            match &mirror {
                Mirror::VERTICAL => {
                    if address <= 0x3FF {
                        self.tbl_name[0][address as usize & 0x3FF] = data;
                    } else if address >= 0x400 && address <= 0x7FF {
                        self.tbl_name[1][address as usize & 0x3FF] = data;
                    } else if address >= 0x800 && address <= 0xBFF {
                        self.tbl_name[0][address as usize & 0x3FF] = data;
                    } else if address >= 0xC00 && address <= 0xFFF {
                        self.tbl_name[1][address as usize & 0x3FF] = data;
                    }
                }
                Mirror::HORIZONTAL => {
                    if address <= 0x3FF {
                        self.tbl_name[0][address as usize & 0x3FF] = data;
                    } else if address >= 0x400 && address <= 0x7FF {
                        self.tbl_name[0][address as usize & 0x3FF] = data;
                    } else if address >= 0x800 && address <= 0xBFF {
                        self.tbl_name[1][address as usize & 0x3FF] = data;
                    } else if address >= 0xC00 && address <= 0xFFF {
                        self.tbl_name[1][address as usize & 0x3FF] = data;
                    }
                }
            }
        } else if address >= 0x3F00 && address <= 0x3FFF {
            let address = address & 0x1F;
            let address = match address {
                0x10 => 0x0,
                0x14 => 0x4,
                0x18 => 0x8,
                0x1C => 0xC,
                _ => address,
            };
            self.tbl_palette[address as usize] = data;
        }
    }

    pub fn cpu_write(&mut self, address: u16, data: u8) {
        match address {
            0 => {
                self.ppuctrl = data;
                if self.ppuctrl & 0x1 != 0 {
                    //hanldes nametable x
                    self.tram_addr |= 0x0400;
                } else {
                    self.tram_addr &= !0x0400;
                }

                if self.ppuctrl & 0x2 != 0 {
                    //hanldes nametable y
                    self.tram_addr |= 0x0800;
                } else {
                    self.tram_addr &= !0x0800;
                }
            }
            1 => {
                self.ppumask = data;
            }
            5 => {
                if self.address_latch == 0 {
                    self.fine_x = data & 0x07;
                    let temp = data >> 3;
                    let temp = temp as u16 & 0x1F;
                    self.tram_addr = self.tram_addr & !0x1F;
                    self.tram_addr |= temp;
                    self.address_latch = 1;
                } else {
                    let temp = data as u16 & 0x07;
                    let temp = temp << 12;
                    self.tram_addr &= !0x7000;
                    self.tram_addr |= temp;
                    let temp = data as u16 >> 3;
                    let temp = temp & 0x1F;
                    let temp = temp << 5;
                    self.tram_addr &= !0x03E0;
                    self.tram_addr |= temp;
                    self.address_latch = 0;
                }
            }
            6 => {
                if self.address_latch == 0 {
                    self.tram_addr = ((data as u16 & 0x3F) << 8) | (self.tram_addr & 0xFF);
                    self.address_latch = 1;
                } else {
                    self.tram_addr = (self.tram_addr & 0xFF) | data as u16;
                    self.vram_addr = self.tram_addr;
                    self.address_latch = 0;
                }
            }
            7 => {
                self.ppu_write(self.vram_addr, data);
                self.vram_addr += if self.ppuctrl & 0x4 != 0 { 32 } else { 1 };
            }
            _ => {
                println!("ppu tryna write to {}", address);
                todo!();
            }
        }
    }
    pub fn cpu_read(&mut self, address: u16, readonly: bool) -> u8 {
        if readonly {
            let data: u8 = match address {
                0 => self.ppuctrl,
                1 => self.ppumask,
                2 => self.ppustatus,
                _ => {
                    println!("ppu tryna read {}", address);
                    0
                }
            };
            data
        } else {
            let data: u8 = match address {
                2 => {
                    let temp = (self.ppustatus & 0xE0) | (self.ppu_buffer & 0x1F);
                    self.ppustatus &= 0x7F;
                    self.address_latch = 0;
                    temp
                }
                7 => {
                    let temp = self.ppu_buffer;
                    self.ppu_buffer = self.ppu_read(self.vram_addr, true);
                    let temp = if self.vram_addr >= 0x3FF0 {
                        self.ppu_buffer
                    } else {
                        temp
                    };
                    self.vram_addr += if self.ppuctrl & 0x04 != 0 { 32 } else { 1 };
                    temp
                }
                _ => 0,
            };
            data
        }
    }
}
