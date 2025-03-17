use registers::{PPUCTRL, PPUMASK, PPUSTATUS};

use crate::cartridge::{Cartridge, Nametable};

pub mod Frame;
mod registers;
use crate::ppu::Frame::Frame as Fr;
pub struct Ppu {
    ppuctrl: PPUCTRL,     //ppu control register (mapped at address $2000)
    ppumask: PPUMASK,     //ppu mask register (mapped at address $2001)
    ppustatus: PPUSTATUS, //ppu status register (mapped at address $2002)
    oamaddr: u8,          //oamaddr register (mapped at address $2003)
    oamdata: u8,          //oamdata register (mapped at address $2004)
    ppuscroll: u8,        //ppuscroll register (mapped at address $2005)
    ppuaddr: u8,          //ppuaddr register (mapped at address $2006)
    ppudata: u8,          //ppudata register (mapped at address $2007)
    oamdma: u8,           //oamdma register (mapped at address $4014)
    // v: vt_reg,            //holds Current VRAM address
    // t: vt_reg,            //holds temporary VRAM address
    v: u16,
    t: u16,
    w: u8, //toggle between first and second write
    x: u8, //fine x scroll
    vram: Vec<u8>,
    internal_buffer: u8,
    nmi: bool,
    cart: *mut Cartridge,
    palette_memory: Vec<u8>,
    system_palette: Vec<(u8, u8, u8)>,
    palette_num: u8,
    cycle_counter: u16,
    scanline_counter: u16,
    frame_updated: bool,
    xscroll: u8,
    yscroll: u8,
    palette_boi: u8,
    total_cycles: usize,
    pattern_table: Vec<Vec<Vec<u8>>>,
    pattern_cached: bool,
    nametable_buffer: Option<*mut Fr>,
}

impl Ppu {
    fn initialize_system_palette() -> Vec<(u8, u8, u8)> {
        let mut toreturn: Vec<(u8, u8, u8)> = vec![(0, 0, 0); 0x40];
        toreturn[10] = (0, 81, 0);
        toreturn[11] = (0, 63, 23);
        toreturn[12] = (27, 63, 95);
        toreturn[13] = (0, 0, 0);
        toreturn[14] = (0, 0, 0);
        toreturn[16] = (188, 188, 188);
        toreturn[17] = (0, 115, 239);
        toreturn[18] = (35, 59, 239);
        toreturn[19] = (131, 0, 243);
        toreturn[1] = (39, 27, 143);
        toreturn[20] = (191, 0, 191);
        toreturn[21] = (231, 0, 91);
        toreturn[22] = (219, 43, 0);
        toreturn[23] = (203, 79, 15);
        toreturn[24] = (139, 115, 0);
        toreturn[25] = (0, 151, 0);
        toreturn[26] = (0, 171, 0);
        toreturn[27] = (0, 147, 59);
        toreturn[28] = (0, 131, 139);
        toreturn[29] = (0, 0, 0);
        toreturn[2] = (0, 0, 171);
        toreturn[30] = (0, 0, 0);
        toreturn[31] = (0, 0, 0);
        toreturn[3] = (71, 0, 159);
        toreturn[4] = (143, 0, 119);
        toreturn[5] = (171, 0, 19);
        toreturn[6] = (167, 0, 0);
        toreturn[7] = (127, 11, 0);
        toreturn[8] = (67, 47, 0);
        toreturn[9] = (0, 71, 0);
        toreturn[0] = (117, 117, 117);
        toreturn[32] = (255, 255, 255);
        toreturn[33] = (63, 191, 255);
        toreturn[34] = (95, 151, 255);
        toreturn[35] = (167, 139, 253);
        toreturn[36] = (247, 123, 255);
        toreturn[37] = (255, 119, 183);
        toreturn[38] = (255, 119, 99);
        toreturn[39] = (255, 155, 59);
        toreturn[40] = (243, 191, 63);
        toreturn[41] = (131, 211, 19);
        toreturn[42] = (79, 223, 75);
        toreturn[43] = (88, 248, 152);
        toreturn[44] = (0, 235, 219);
        toreturn[45] = (0, 0, 0);
        toreturn[46] = (0, 0, 0);
        toreturn[47] = (0, 0, 0);
        toreturn[48] = (255, 255, 255);
        toreturn[49] = (171, 231, 255);
        toreturn[50] = (199, 215, 255);
        toreturn[51] = (215, 203, 255);
        toreturn[52] = (255, 199, 255);
        toreturn[53] = (255, 199, 219);
        toreturn[54] = (255, 191, 179);
        toreturn[55] = (255, 219, 171);
        toreturn[56] = (255, 231, 163);
        toreturn[57] = (227, 255, 163);
        toreturn[58] = (171, 243, 191);
        toreturn[59] = (179, 255, 207);
        toreturn[60] = (159, 255, 243);
        toreturn[61] = (0, 0, 0);
        toreturn[62] = (0, 0, 0);
        toreturn[63] = (0, 0, 0);
        toreturn
    }

    pub fn get_nmi(&mut self) -> bool {
        let data = self.nmi;
        if data {
            self.nmi = false;
        }
        data
    }
    pub fn new(cartridge: &mut Cartridge) -> Self {
        let pal: Vec<u8> = vec![0; 0x20];
        let mut vram: Vec<u8> = vec![0; 2048];
        for i in &mut vram {
            *i = rand::random_range(0..=0xFF);
        }
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
            v: 0,
            t: 0,
            w: 0,
            x: 0,
            vram: vram,
            internal_buffer: 0,
            nmi: false,
            cart: cartridge,
            palette_memory: pal,
            system_palette: Ppu::initialize_system_palette(),
            palette_num: 0,
            cycle_counter: 0,
            scanline_counter: 0,
            xscroll: 0,
            yscroll: 0,
            palette_boi: 0,
            total_cycles: 0,
            frame_updated: false,
            pattern_table: vec![vec![vec![0; 128]; 128]; 2],
            pattern_cached: false,
            nametable_buffer: None,
        }
    }
    pub fn set_bg_palette_num(&mut self){
        self.palette_num = (self.palette_num + 1) & 0xF;
    }
    pub fn get_palette(&mut self, palettenum: u8, paletteindex: u8) -> (u8, u8, u8) {
        self.palette_boi = paletteindex;
        let palettenum = palettenum & 0x7;
        let final_index = (palettenum << 2) | paletteindex;
        let paletteinde = self.ppu_read(0x3F00 | final_index as u16);
        self.system_palette[paletteinde as usize]
    }

    pub fn linkpattern(&mut self, frame: &mut Fr) {
        self.nametable_buffer = Some(frame);
    }

    pub fn get_pattern_table(&mut self, frame: &mut Fr, palette: u8) {
        self.palette_boi = palette;
        if !self.pattern_cached {
            /* There's nothing in the palette_table memory */
            /* filling first (and populating frame as well) */
            for row in 0..0x20 {
                for col in 0..0x10 {
                    let table = if row < 16 { 0 } else { 0b0_1_00000000_0_000 }; //tells us which table to fill
                    let tile_num = ((row & 0xF) * 16) + (col) as u16;
                    let palette_address = table | (tile_num << 4);
                    for i in 0..8 {
                        let mut lo_byte = self.ppu_read(palette_address + i); /* get low byte of palette */
                        let mut hi_byte = self.ppu_read(palette_address + 8 + i); /* get high byte of palette */
                        for j in 0..8 {
                            let lo_bit = lo_byte & 0x1;
                            let hi_bit = hi_byte & 0x1;

                            let pixel_num = (hi_bit << 1) | lo_bit;
                            /* y = row * 8 + i, x = col * 8 */
                            let x = (col * 8) + (7 - j);
                            let y = ((row & 0xF) * 8) + i;
                            let y = y as usize;

                            let table_num = if row < 16 { 0 } else { 1 };
                            self.pattern_table[table_num][x][y] = pixel_num;
                            hi_byte = hi_byte >> 1;
                            lo_byte = lo_byte >> 1;
                        }
                    }
                }
            }
            self.pattern_cached = true
        } else {
            for x in 0..256 {
                for y in 0..128 {
                    let table_index = if x > 127 { 1 } else { 0 };
                    let index = self.pattern_table[table_index][x & 0x7F][y];
                    let color = self.get_palette(self.palette_num, index);
                    frame.drawpixel(x as u16, y as u16, color);
                }
            }
        }
    }
    pub fn ppu_read(&mut self, address: u16) -> u8 {
        let mut byte = 0;

        if address <= 0x1FFF {
            unsafe { (*self.cart).ppu_read(address, &mut byte) }; // Reads from cartridge space
        } else if address >= 0x2000 && address <= 0x2FFF {
            let nametable: Nametable = unsafe { (*self.cart).get_nametable() };
            byte = match nametable {
                Nametable::Vertical => {
                    let index = match address {
                        0x2000..=0x23FF | 0x2800..=0x2BFF => address & 0x3FF,
                        0x2400..=0x27FF | 0x2C00..=0x2FFF => 0x400 + (address & 0x3FF),
                        _ => panic!("Address out of range!"),
                    };
                    self.vram[index as usize]
                }
                Nametable::Horizontal => {
                    let index = match address {
                        0x2000..=0x23FF | 0x2400..=0x27FF => address & 0x3FF,
                        0x2800..=0x2BFF | 0x2C00..=0x2FFF => 0x400 + (address & 0x3FF),
                        _ => panic!("Address out of range!"),
                    };
                    self.vram[index as usize]
                }
            };
        } else if address >= 0x3000 && address <= 0x3EFF {
            // Mirror of 0x2000 - 0x2EFF
            byte = self.ppu_read(address - 0x1000);
        } else if address >= 0x3F00 && address <= 0x3FFF {
            // Palette memory handling with mirroring
            byte = self.palette_memory[(address & 0x1F) as usize];
        } else {
            // Open bus or undefined memory area
            byte = 0;
        }

        byte
    }

    pub fn ppu_write(&mut self, address: u16, data: u8) {
        if address <= 0x1FFF {
            unsafe { (*self.cart).ppu_write(address, data) }; // writes to cartridge space
            // self.pattern_cached = false;
        } else if address >= 0x2000 && address <= 0x2FFF {
            let nametable: Nametable = unsafe { (*self.cart).get_nametable() };
            match nametable {
                Nametable::Vertical => {
                    let index = match address {
                        0x2000..=0x23FF | 0x2800..=0x2BFF => address & 0x3FF,
                        0x2400..=0x27FF | 0x2C00..=0x2FFF => 0x400 + (address & 0x3FF),
                        _ => panic!("Address out of range!"),
                    };
                    self.vram[index as usize] = data;
                }
                Nametable::Horizontal => {
                    let index = match address {
                        0x2000..=0x23FF | 0x2400..=0x27FF => address & 0x3FF,
                        0x2800..=0x2BFF | 0x2C00..=0x2FFF => 0x400 + (address & 0x3FF),
                        _ => panic!("Address out of range!"),
                    };
                    self.vram[index as usize] = data;
                }
            };
        } else if address >= 0x3000 && address <= 0x3EFF {
        } else if address >= 0x3F00 && address <= 0x3FFF {
            self.palette_memory[(address & 0x1F) as usize] = data;
        } else {
            // todo!()
        }
    }

    ///# cpu_read
    /// This function lets the cpu read from the PPU Address space.
    /// ## Addresses
    pub fn cpu_read(&mut self, address: u16, rdonly: bool) -> u8 {
        let masked_address = address & 0x7;
        let mut data = 0;
        match masked_address {
            0 | 1 | 3 | 5 | 6 => {
                data = 0;
            }
            2 => {
                data = self.ppustatus.bits();
                if !rdonly {
                    self.ppustatus.set(PPUSTATUS::vblank_flag, false);
                    self.w = 0;
                }
            }
            4 => {
                todo!() //handle OAM reads
            }
            7 => {
                data = self.internal_buffer;
                self.internal_buffer = self.ppu_read(self.v);
                if address >= 0x3F00 && address <= 0x3FFF {
                    data = self.internal_buffer;
                }
                /* We increment the v register by 32 or 1 depending on the PPUCTRL increment flag */
                if !rdonly {
                    let inc_addr = self.v;
                    let inc_factor = if self.ppuctrl.contains(PPUCTRL::vram_increment) {
                        32
                    } else {
                        1
                    };
                    let inc_addr = inc_addr.wrapping_add(inc_factor);
                    self.v = inc_addr;
                }
            }
            _ => {
                panic!("cpu_read: Cannot read address");
            }
        }
        data
    }

    /// # cpu_write
    /// This function provides CPU access to the PPU, letting it define certain parameters and behaviors of the PPU.
    /// ## Address map
    /// - address $2000: Set the nametable on the t register and update the ppu control register.
    /// - address $2001: Set the PPU Mask register to data.
    /// - address $2003 and $2004 cannot be written to.
    /// - address $2005 will define the scroll information on the PPUScroll X register
    /// - address $2006 initializes the PPU Addresses on writes
    /// - address $2007 will write data to the address in the PPU address space.
    pub fn cpu_write(&mut self, address: u16, data: u8) {
        let masked_address = address & 0x7;
        match masked_address {
            0 => {
                self.ppuctrl = PPUCTRL::from_bits_truncate(data);
            }
            1 => {
                self.ppumask = PPUMASK::from_bits_truncate(data);
            }
            3 => {
                // todo!()
            }
            4 => {
                // todo!()
            }
            5 => {
                if self.w == 0 {
                    self.xscroll = data;
                    self.w = 1;
                } else if self.w == 1 {
                    self.yscroll = data;
                    self.w = 0;
                }
            }
            6 => {
                if self.w == 0 {
                    let data = data as u16;
                    self.v = data << 8;
                    self.w = 1;
                } else if self.w == 1 {
                    self.v |= data as u16;
                    self.w = 0;
                }
                return;
            }
            7 => {
                self.ppu_write(self.v, data);
                let v_addr = self.v;
                let add_factor = if self.ppuctrl.contains(PPUCTRL::vram_increment) {
                    32
                } else {
                    1
                };
                let v_addr = v_addr.wrapping_add(add_factor);
                self.v = v_addr;
            }
            _ => {
                panic!("cpu_write: Cannot write address");
            }
        }
    }

    pub fn update_block(&mut self, row: u16, col: u16) {
        let background_index = if self
            .ppuctrl
            .contains(PPUCTRL::background_pattern_table_address)
        {
            1
        } else {
            0
        };
        let name_index = (row * 32) + col;
        let name_table_index = self.ppu_read(0x2000 + name_index as u16);
        let x_index = name_table_index & 0xF;
        let y_index = name_table_index >> 4;
        let x_index = x_index * 8;
        let y_index = y_index * 8;

        for i in 0..8 {
            for j in 0..8 {
                let palette_index = self.pattern_table[background_index][(x_index + i) as usize]
                    [(y_index + j) as usize];
                let color = self.get_palette(self.palette_num, palette_index);
                let x = (col * 8) + i as u16 as u16;
                let y = ((row * 8) + j as u16) as u16;
                unsafe {
                    (*self.nametable_buffer.unwrap()).drawpixel(x, y, color);
                };
            }
        }
    }

    pub fn set_name_table(&mut self){
        for col in 0..32{
            for row in 0..30{
                self.update_block(row, col);
            }
        }
    }

    pub fn clock(&mut self) {
        self.cycle_counter += 1;
        if self.cycle_counter > 340 {
            self.cycle_counter = 0;
            self.scanline_counter += 1;
        }
        if self.scanline_counter <= 239 {
        } else if self.scanline_counter == 241 && self.cycle_counter == 1 {
            self.ppustatus.set(PPUSTATUS::vblank_flag, true);
            self.nmi = true;
        } else if self.scanline_counter == 261 && self.cycle_counter == 1 {
            self.ppustatus.set(PPUSTATUS::vblank_flag, false);
            self.scanline_counter = 0;
        }
        self.total_cycles = self.total_cycles.wrapping_add(1);
    }
}
