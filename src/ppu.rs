use frame::Frame;
use registers::{vt_reg, PPUCTRL, PPUMASK, PPUSTATUS};

use crate::cartridge::{Cartridge, Nametable};

pub mod frame;
mod registers;
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
    v: vt_reg,            //holds Current VRAM address
    t: vt_reg,            //holds temporary VRAM address
    w: u8,                //toggle between first and second write
    x: u8,                //fine x scroll
    vram: Vec<u8>,
    internal_buffer: u8,
    nmi: bool,
    cart: *mut Cartridge,
    palette_memory: Vec<u8>,
    system_palette: Vec<(u8, u8, u8)>,
    palette_num: u8,
    cycle_counter: u16,
    scanline_counter: u16,
    nmi_count: u8,
    /* for rendering */
    bg_next_tile_id: u8,
    bg_next_tile_attrib: u8,
    bg_next_tile_lsb: u8,
    bg_next_tile_msb: u8,
    bg_shifter_pattern_lo: u16,
    bg_shifter_pattern_hi: u16,
    bg_shifter_attrib_lo: u16,
    bg_shifter_attrib_hi: u16,
    fine_x: u8,
    game_frame: *mut Frame,
}

impl Ppu {
    fn initialize_system_palette() -> Vec<(u8, u8, u8)> {
        let mut toreturn: Vec<(u8, u8, u8)> = vec![(0, 0, 0); 0x40];
        toreturn[10] = (0, 81, 0);
        toreturn[11] = (0, 63, 23);
        toreturn[12] = (27, 63, 95);
        toreturn[13] = (0, 0, 0);
        toreturn[14] = (0, 0, 0);
        toreturn[15] = (0, 0, 0);
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
    pub fn new(cartridge: &mut Cartridge, frame:&mut Frame) -> Self {
        let mut pal: Vec<u8> = vec![0; 0x20];
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
            v: vt_reg::new(),
            t: vt_reg::new(),
            w: 0,
            x: 0,
            vram: vec![0; 2048],
            internal_buffer: 0,
            nmi: false,
            cart: cartridge,
            palette_memory: pal,
            system_palette: Ppu::initialize_system_palette(),
            palette_num: 0,
            cycle_counter: 0,
            scanline_counter: 0,
            nmi_count: 0,
            bg_next_tile_id: 0,
            bg_next_tile_attrib: 0,
            bg_next_tile_lsb: 0,
            bg_next_tile_msb: 0,
            bg_shifter_pattern_lo: 0,
            bg_shifter_pattern_hi: 0,
            bg_shifter_attrib_lo: 0,
            bg_shifter_attrib_hi: 0,
            fine_x: 0,
            game_frame: frame,
        }
    }
    pub fn get_palette(&mut self, palettenum: u8, paletteindex: u8) -> (u8, u8, u8) {
        let palettenum = palettenum & 0x7;
        let final_index = (palettenum << 2) | paletteindex;
        let paletteinde = self.ppu_read(0x3F00 | final_index as u16);
        self.system_palette[paletteinde as usize]
    }
    pub fn get_palette_table(&mut self, frame: &mut Frame, palette: u8) {
        for row in 0..16 {
            for col in 0..32 {
                let mut address: u16 = 0;
                let tile_index: u16 = row * 16 + (col % 16);
                let table: u16 = if col > 15 { 0b01000000000000 } else { 0 };
                address += table;
                let tile_index_addr = tile_index << 4;
                address |= tile_index_addr;
                for i in 0..8 {
                    let mut lobyte = self.ppu_read(address + i);
                    let mut hibyte = self.ppu_read(address + 8 + i);
                    for x in 0..8 {
                        let lo = lobyte & 1;
                        let hi = hibyte & 1;
                        let color = (hi << 1) | (lo);
                        let index_num = color;
                        let color = match color {
                            0 => self.system_palette[0x0a],
                            1 => self.system_palette[0x1a],
                            2 => self.system_palette[0x2a],
                            3 => self.system_palette[0x3a],
                            _ => self.system_palette[0x34],
                        };
                        lobyte = lobyte >> 1;
                        hibyte = hibyte >> 1;
                        let xx = (col * 8) + (8 - x);
                        let y = (row * 8) + i;
                        let color = self.get_palette(palette, index_num);
                        frame.drawpixel(xx - 1, y, color);
                    }
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
            byte = self.vram[(address & 0x7FF) as usize];
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
            self.vram[(address & 0x7FF) as usize] = data;
        } else if address >= 0x3000 && address <= 0x3EFF {
            // Mirror of 0x2000 - 0x2EFF
            // self.ppu_write(address - 0x1000, data);
        } else if address >= 0x3F00 && address <= 0x3FFF {
            // Palette memory handling
            self.palette_memory[(address & 0x1F) as usize] = data;
        } else {
            todo!()
        }
    }

    ///# cpu_read
    /// This function lets the cpu read from the PPU Address space.
    /// ## Addresses
    pub fn cpu_read(&mut self, address: u16) -> u8 {
        let masked_address = address & 0x7;
        let mut data = 0;
        match masked_address {
            0 | 1 | 3 | 5 | 6 => {
                data = 0;
            }
            2 => {
                data = self.ppustatus.bits();
                self.ppustatus.set(PPUSTATUS::vblank_flag, false);
                self.w = 0;
            }
            4 => {
                //todo!() //handle OAM reads
            }
            7 => {
                data = self.internal_buffer;
                self.internal_buffer = self.ppu_read(self.v.get_data());
                if address >= 0x3F00 && address <= 0x3FFF {
                    data = self.internal_buffer;
                }
                /* We increment the v register by 32 or 1 depending on the PPUCTRL increment flag */
                let inc_addr = self.v.get_data();
                let inc_factor = if self.ppuctrl.contains(PPUCTRL::vram_increment) {
                    32
                } else {
                    1
                };
                let inc_addr = inc_addr.wrapping_add(inc_factor);
                self.v.set_data(inc_addr);
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
                self.t.set_nametable(data);
                self.ppuctrl = PPUCTRL::from_bits_truncate(data);
            }
            1 => {
                self.ppumask = PPUMASK::from_bits_truncate(data);
            }
            3 => {
                //todo!()
            }
            4 => {
                //todo!()
            }
            5 => {
                if self.w == 0 {
                    let x_dest = data & 0x7;
                    self.x = x_dest;
                    let t_dest = data >> 3;
                    self.t.set_coarse_xscroll(t_dest);
                    self.w = 1;
                } else if self.w == 1 {
                    let fine_y_dest = data & 0x7;
                    self.t.set_fine_y(fine_y_dest);
                    let course_y_dest = data >> 3;
                    self.t.set_coarse_yscroll(course_y_dest);
                    self.w = 0;
                }
            }
            6 => {
                if self.w == 0 {
                    /*
                    yyy NN YYYYY XXXXX
                    ||| || ||||| +++++-- coarse X scroll
                    ||| || +++++-------- coarse Y scroll
                    ||| ++-------------- nametable select
                    +++----------------- fine Y scroll
                    */
                    // println!("");
                    let fine_y_nametable = data & 0x3F;
                    let fine_y_nametable = fine_y_nametable as u16;
                    let temp_data = self.t.get_data();
                    let temp_data_mask = !0b0_111111_00000000;
                    let temp_data = temp_data & temp_data_mask;
                    let fine_y_nametable = fine_y_nametable << 8;
                    let temp_data = temp_data | fine_y_nametable;
                    let temp_data = temp_data & 0b0_0111111_11111111;
                    self.t.set_data(temp_data);
                    self.w = 1;
                } else if self.w == 1 {
                    let modify_t_reg = self.t.get_data();
                    let t_reg_mask = !0b0000000_11111111;
                    let modify_t_reg = modify_t_reg & t_reg_mask;
                    let data = data as u16;
                    let modify_t_reg = modify_t_reg | data;
                    self.t.set_data(modify_t_reg);
                    self.v.set_data(modify_t_reg);
                    self.w = 0;
                }
            }
            7 => {
                self.ppu_write(self.v.get_data(), data);
                let v_addr = self.v.get_data();
                let add_factor = if self.ppuctrl.contains(PPUCTRL::vram_increment) {
                    32
                } else {
                    1
                };
                let v_addr = v_addr.wrapping_add(add_factor);
                self.v.set_data(v_addr);
            }
            _ => {
                panic!("cpu_write: Cannot write address");
            }
        }
    }
    /*
    yyy NN YYYYY XXXXX
    ||| || ||||| +++++-- coarse X scroll
    ||| || +++++-------- coarse Y scroll
    ||| ++-------------- nametable select
    +++----------------- fine Y scroll
    */

    fn update_shift_register(&mut self) {
        if self.ppumask.contains(PPUMASK::enable_background_rendering) {
            self.bg_shifter_pattern_lo <<= 1;
            self.bg_shifter_pattern_hi <<= 1;
            self.bg_shifter_attrib_lo <<= 1;
            self.bg_shifter_attrib_hi <<= 1;
        }
    }

    fn y_increment(&mut self) {
        if self.ppumask.contains(PPUMASK::enable_background_rendering)
            || self.ppumask.contains(PPUMASK::enable_sprite_rendering)
        {
            let fine_y_value = self.v.get_fine_y();
            if fine_y_value != 7 {
                self.v.set_fine_y(fine_y_value + 1);
            } else {
                self.v.set_fine_y(0);
                let coarse_y = self.v.get_coarse_yscroll();
                if coarse_y == 29 {
                    self.v.set_coarse_yscroll(0);
                    let vert_nametable = self.v.get_nametable();
                    let vert_nametable = vert_nametable ^ 0x2;
                    self.v.set_nametable(vert_nametable);
                } else if coarse_y == 31 {
                    self.v.set_coarse_yscroll(0);
                } else {
                    self.v.set_coarse_yscroll(coarse_y + 1);
                }
            }
        }
    }

    fn x_increment(&mut self) {
        if self.ppumask.contains(PPUMASK::enable_background_rendering)
            || self.ppumask.contains(PPUMASK::enable_sprite_rendering)
        {
            let coarse_x = self.v.get_coarse_xscroll();
            if coarse_x == 31 {
                self.v.set_coarse_xscroll(0);
                let nametable = self.v.get_nametable();
                self.v.set_nametable(nametable ^ 1);
            } else {
                self.v.set_coarse_xscroll(coarse_x + 1);
            }
        }
    }

    fn transfervtx(&mut self) {
        if self.ppumask.contains(PPUMASK::enable_background_rendering)
            || self.ppumask.contains(PPUMASK::enable_sprite_rendering)
        {
            self.v.set_nametablex(self.t.get_nametablex());
            self.v.set_coarse_xscroll(self.t.get_coarse_xscroll());
        }
    }
    fn transfervty(&mut self) {
        if self.ppumask.contains(PPUMASK::enable_background_rendering)
            || self.ppumask.contains(PPUMASK::enable_sprite_rendering)
        {
            self.v.set_fine_y(self.t.get_fine_y());
            self.v.set_nametabley(self.t.get_fine_y());
            self.v.set_coarse_yscroll(self.t.get_coarse_yscroll());
        }
    }

    fn load_shift_register(&mut self) {
        self.bg_shifter_pattern_lo =
            (self.bg_shifter_pattern_lo & 0xFF00) | self.bg_next_tile_lsb as u16;
        self.bg_shifter_pattern_hi =
            (self.bg_shifter_pattern_hi & 0xFF00) | self.bg_next_tile_msb as u16;
        self.bg_shifter_attrib_lo = (self.bg_shifter_attrib_lo & 0xFF00)
            | if self.bg_next_tile_attrib & 1 != 0 {
                0xFF
            } else {
                0x00
            };
        self.bg_shifter_attrib_hi = (self.bg_shifter_attrib_hi & 0xFF00)
            | if self.bg_next_tile_attrib & 2 != 0 {
                0xFF
            } else {
                0x00
            }
    }
    fn set_pixel(&mut self,x: u16,y: u16, palettenum: u8,paletteindex: u8){
        println!("x: {}, y: {}",x,y);
        let color = self.get_palette(palettenum, paletteindex);
        unsafe {(*self.game_frame).drawpixel(x, y, color);}
    }
    
    pub fn clock(&mut self) {
        if self.scanline_counter < 240 {
            if self.scanline_counter == 0 && self.cycle_counter == 0 {
                self.cycle_counter = 1;
            }
            if self.scanline_counter == 261 && self.cycle_counter == 1 {
                self.ppustatus.set(PPUSTATUS::vblank_flag, false);
            }
            if (self.cycle_counter >= 2 && self.cycle_counter < 258)
                || (self.cycle_counter >= 321 && self.cycle_counter < 338)
            {
                self.update_shift_register();
                match (self.cycle_counter - 1) % 8 {
                    0 => {
                        self.load_shift_register();
                        self.bg_next_tile_id = self.ppu_read(0x2000 | (self.v.get_data() & 0xFFF));
                    },
                    2 => {
                        let addr = 0x23C0 | ((self.v.get_data() & 0x0C00)) | (((self.v.get_coarse_yscroll() as u16) >> 2) << 3) | ((self.v.get_coarse_xscroll() as u16) >> 2);
                        self.bg_next_tile_attrib = self.ppu_read(addr);
                        if (self.v.get_coarse_yscroll() & 0x2) != 0 { self.bg_next_tile_attrib >>= 4; }
                        if (self.v.get_coarse_xscroll() & 0x2) != 0 { self.bg_next_tile_attrib >>= 2; }
                        self.bg_next_tile_attrib &= 0x3;
                    },
                    4 => {
                        let base = if self.ppuctrl.contains(PPUCTRL::background_pattern_table_address) { 0x1000 } else { 0x0000 };
                        let addr = base + ((self.bg_next_tile_id as u16) << 4) + self.v.get_fine_y() as u16 + if (self.cycle_counter - 1) % 8 == 4 { 0 } else { 8 };
                        if (self.cycle_counter - 1) % 8 == 4 {
                            self.bg_next_tile_lsb = self.ppu_read(addr);
                        } else {
                            self.bg_next_tile_msb = self.ppu_read(addr);
                        }
                    },
                    7 => {
                        self.x_increment();
                    },
                    _ => {},
                }
            }
            if self.cycle_counter == 256 {
                self.y_increment();
            }
            if self.cycle_counter == 257 {
                self.load_shift_register();
                self.transfervtx();
            }
            if self.cycle_counter == 338 || self.cycle_counter == 340 {
                self.bg_next_tile_id = self.ppu_read(0x2000 | (self.v.get_data() & 0xFFF));
            }
            if self.scanline_counter == 261 && self.cycle_counter >= 280 && self.cycle_counter < 305 {
                self.transfervty();
            }
        }
    
        if self.scanline_counter == 240 {}
    
        if self.scanline_counter >= 241 && self.scanline_counter < 261 {
            if self.scanline_counter == 241 && self.cycle_counter == 1 {
                self.ppustatus.set(PPUSTATUS::vblank_flag, true);
                if self.ppuctrl.contains(PPUCTRL::vblank_enable) {
                    self.nmi = true;
                }
            }
        }
    
        if self.ppumask.contains(PPUMASK::enable_background_rendering) {
            let bit_mux = 0x8000 >> self.fine_x;
            let p0_pixel = ((self.bg_shifter_pattern_lo & bit_mux) > 0) as u8;
            let p1_pixel = ((self.bg_shifter_pattern_hi & bit_mux) > 0) as u8;
            let bg_pixel = (p1_pixel << 1) | p0_pixel;
    
            let bg_pal0 = ((self.bg_shifter_attrib_lo & bit_mux) > 0) as u8;
            let bg_pal1 = ((self.bg_shifter_attrib_hi & bit_mux) > 0) as u8;
            let bg_palette = (bg_pal1 << 1) | bg_pal0;
    
            self.set_pixel(self.cycle_counter, self.scanline_counter, bg_palette, bg_pixel);
        }
    
        self.cycle_counter += 1;
        if self.cycle_counter >= 341 {
            self.cycle_counter = 0;
            self.scanline_counter += 1;
            if self.scanline_counter > 261 {
                self.scanline_counter = 0;
            }
        }
    }
}
