mod oam;
use std::thread;

use crate::ppu::oam::oam as Oam;
use registers::{vt_reg, PPUCTRL, PPUMASK, PPUSTATUS};
use sdl2::controller;
use sdl2::libc::FSOPT_PACK_INVAL_ATTRS;

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
    oamdma: u8,           //oamdma register (mapped at address $4014)
    v: vt_reg,
    x_scroll: u16,
    y_scroll: u16,
    t: vt_reg,
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
    scanline_counter: i16,
    palette_boi: u8,
    total_cycles: usize,
    pattern_table: Vec<Vec<Vec<u8>>>,
    pattern_cached: bool,
    nametable_buffer: Option<*mut Fr>,
    oam_table: Vec<Oam>,
    whole_frame: Vec<Vec<((u8, u8, u8), u8)>>,
    nametable_changed: bool,
    nametable_index: u8,
    pattern_lo_shift_register: u16,
    pattern_hi_shift_register: u16,
    attribute_lo_shift_register: u16,
    attribute_hi_shift_register: u16,
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
            oamdma: 0,
            v: vt_reg::new(),
            t: vt_reg::new(),
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
            palette_boi: 0,
            total_cycles: 0,
            pattern_table: vec![vec![vec![0; 128]; 128]; 2],
            pattern_cached: false,
            nametable_buffer: None,
            oam_table: vec![Oam::new(); 64],
            whole_frame: vec![vec![((0, 0, 0), 0); 480]; 512],
            x_scroll: 0,
            y_scroll: 0,
            nametable_changed: true,
            nametable_index: 0,
            pattern_lo_shift_register: 0,
            pattern_hi_shift_register: 0,
            attribute_lo_shift_register: 0,
            attribute_hi_shift_register: 0,
        }
    }
    pub fn set_bg_palette_num(&mut self) {
        self.palette_num = (self.palette_num + 1) & 0xF;
    }
    pub fn oam_dma_write(&mut self, address: u8, data: u8) {
        let index = address / 4;
        self.oam_table[index as usize].set_byte(address, data);
        self.nametable_changed = true;
    }
    pub fn get_bgpalette(&mut self, palettenum: u8, paletteindex: u8) -> (u8, u8, u8) {
        let palettenum = palettenum & 0x1F;
        let final_index = (palettenum << 2) | paletteindex;
        let paletteinde = self.ppu_read(0x3F00 | final_index as u16);
        self.system_palette[paletteinde as usize]
    }

    pub fn get_fgpalette(&mut self, palettenum: u8, paletteindex: u8) -> (u8, u8, u8) {
        let palettenum = palettenum & 0x1F;
        let final_index = (palettenum << 2) | paletteindex;
        let paletteinde = self.ppu_read(0x3F10 | final_index as u16);
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
                    let color = self.get_bgpalette(self.palette_num, index);
                    frame.drawpixel(x as u16, y as u16, color);
                }
            }
        }
    }
    pub fn ppu_read(&self, address: u16) -> u8 {
        let mut byte = 0;

        if address <= 0x1FFF {
            unsafe { (*self.cart).ppu_read(address, &mut byte) }; // Reads from cartridge space
        } else if address >= 0x2000 && address <= 0x2FFF {
            let nametable: Nametable = unsafe { (*self.cart).get_nametable() };
            byte = match nametable {
                Nametable::Vertical => {
                    let index = match address {
                        0x2000..=0x23FF => {
                            address & 0x3FF
                        },
                        0x2800..=0x2BFF => {
                            address & 0x3FF
                        },
                        0x2400..=0x27FF => {
                            0x400 + (address & 0x3FF)
                        },
                        0x2C00..=0x2FFF => {
                            0x400 + (address & 0x3FF)
                        },
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
            // unsafe { (*self.cart).ppu_write(address, data) }; // writes to cartridge space
            self.pattern_cached = false;
        } else if address >= 0x2000 && address <= 0x2FFF {
            /* nametable writes */
            let nametable: Nametable = unsafe { (*self.cart).get_nametable() };

            match nametable {
                Nametable::Vertical => {
                    match address {
                        0x2000..=0x23FF => {
                            /* nametable 0 */
                            let addr = address & 0x3FF;
                            self.vram[addr as usize] = data;
                        },
                        0x2800..=0x2BFF => {
                            /* nametable 1 */
                            // address & 0x3FF;
                            let addr = address & 0x3FF;
                            self.vram[addr as usize] = data;
                        },
                        0x2400..=0x27FF  => {
                            /* nametable 2 */
                            let addr = 0x400 + (address & 0x3FF);
                            self.vram[addr as usize] = data;
                        },
                        0x2C00..=0x2FFF => {
                            /* nametable 3 */
                            let addr = 0x400 + (address & 0x3FF);
                            self.vram[addr as usize] = data;
                        },
                        _ => panic!("Address out of range!"),
                    };
                    
                    self.nametable_changed = true;
                }
                Nametable::Horizontal => {
                    let index = match address {
                        0x2000..=0x23FF | 0x2400..=0x27FF => address & 0x3FF,
                        0x2800..=0x2BFF | 0x2C00..=0x2FFF => 0x400 + (address & 0x3FF),
                        _ => panic!("Address out of range!"),
                    };
                    self.vram[index as usize] = data;
                    self.nametable_changed = true;
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
                data = self.oam_table[(self.oamaddr >> 2) as usize].get_byte(self.oamaddr);
            }
            7 => {
                data = self.internal_buffer;
                self.internal_buffer = self.ppu_read(self.v.get_data());
                if address >= 0x3F00 && address <= 0x3FFF {
                    data = self.internal_buffer;
                }
                /* We increment the v register by 32 or 1 depending on the PPUCTRL increment flag */
                if !rdonly {
                    let inc_addr = self.v.get_data();
                    let inc_factor = if self.ppuctrl.contains(PPUCTRL::vram_increment) {
                        32
                    } else {
                        1
                    };
                    let inc_addr = inc_addr.wrapping_add(inc_factor) & 0x3FFF;
                    self.v.set_data(inc_addr);
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
                self.t.set_nametable(data & 0b11);
            }
            1 => {
                self.ppumask = PPUMASK::from_bits_truncate(data);
            }
            3 => {
                self.oamaddr = data;
            }
            4 => {
                self.oam_table[(self.oamaddr >> 2) as usize].set_byte(self.oamaddr, data);
            }
            5 => {
                if self.w == 0 {
                    let temp_val = data >> 3;
                    self.t.set_coarse_xscroll(temp_val);
                    self.x = data & 3;
                    self.w = 1;
                } else if self.w == 1 {
                    self.t.set_fine_y(data & 3);
                    self.t.set_coarse_yscroll(data >> 3);
                    self.w = 0;
                }
            }
            6 => {
                if self.w == 0 {
                    let temp_dat = data as u16;
                    let temp_val = self.t.get_data();
                    let temp_val = temp_val & 0b0000000_11111111;
                    let temp_val = temp_val | (temp_dat << 8);
                    self.t.set_data(temp_val);
                    self.w = 1;
                } else if self.w == 1 {
                    let temp_data = self.t.get_data();
                    let temp_data = temp_data & 0b11111111_00000000;
                    let data = data as u16;
                    let temp_data = temp_data | data;
                    self.t.set_data(temp_data);
                    self.v.set_data(self.t.get_data());
                    self.w = 0;
                }
            }
            7 => {
                let increment_factor = if self.ppuctrl.contains(PPUCTRL::vram_increment) {
                    32
                } else {
                    1
                };
                let address = self.v.get_data();
                self.ppu_write(address, data);
                let address = address.wrapping_add(increment_factor) & 0x3FFF;
                self.v.set_data(address);
            }
            _ => {
                panic!("cpu_write: Cannot write address");
            }
        }
    }


    fn increment_x(&mut self){
        if self.v.get_coarse_xscroll() == 31{
            self.v.set_coarse_xscroll(0);
            let nametable = self.v.get_nametablex() ^ 1;
            self.v.set_nametablex(nametable);
        }
        else{
            let coarsex = self.v.get_coarse_xscroll();
            self.v.set_coarse_xscroll(coarsex + 1);
        }
    }
pub fn increment_y(&mut self) {
    // Extract fine Y from v
    let fine_y = self.v.get_fine_y();

    if fine_y < 7 {
        // Increment fine Y
        self.v.set_fine_y(fine_y + 1);
    } else {
        // Reset fine Y
        self.v.set_fine_y(0);

        // Handle coarse Y increment
        let coarse_y = self.v.get_coarse_yscroll();

        if coarse_y == 29 {
            // Reset coarse Y and toggle vertical nametable
            self.v.set_coarse_yscroll(0);
            let namy = self.v.get_nametabley() ^ 1;
            self.v.set_nametabley(namy);
        } else if coarse_y == 31 {
            // Reset coarse Y without toggling nametable
            self.v.set_coarse_yscroll(0);
        } else {
            // Simply increment coarse Y
            self.v.set_coarse_yscroll(coarse_y + 1);
        }
    }
}
    fn print_address(&self) -> String{
        let v = self.v.get_data();
        let tile = 0x2000 | (v & 0xFFF);
        let attrib = 0x23C0 | (v & 0x0C00) | ((v >> 4) & 0x38) | ((v >> 2) & 0x7);
        format!("tile: {:4x}, attrib: {:4x}\t",tile,attrib)
    }
    fn get_tiles(&mut self) -> (u16, u16){
        let v = self.v.get_data();
        let tile = 0x2000 | (v & 0xFFF);
        let attrib = 0x23C0 | (v & 0x0C00) | ((v >> 4) & 0x38) | ((v >> 2) & 0x7);
        let nametable_tile = self.ppu_read(tile) as u16;
        let attribute_tile = self.ppu_read(attrib) as u16;
        (nametable_tile, attribute_tile)
    }
    fn get_second_tile(&mut self){
        println!("x: {}, {} ,{}\t",self.x,self.print_address(),self.v.print_register());
        /* Retrieve nametable and attribute table */
        let (nametable_tile,attribute_tile) = self.get_tiles();
        
        /* Calculate address for pallette  */
        let address = if self.ppuctrl.contains(PPUCTRL::background_pattern_table_address) {0x1000} else {0} as u16;
        let address = address | (nametable_tile << 4) | (self.v.get_fine_y() as u16);

        /* Retrieve palette bytes */

        /* Hi byte */
        let pattern_table_hibyte = self.ppu_read(address + 8) as u16;
        self.pattern_hi_shift_register = (self.pattern_hi_shift_register & 0xFF00) |  pattern_table_hibyte;

        /* Lo byte */
        let pattern_table_lobyte = self.ppu_read(address) as u16;
        self.pattern_lo_shift_register = (self.pattern_lo_shift_register & 0xFF00) |  pattern_table_lobyte;

        /* Retrieve attribute table bytes */

        let mut attribute_byte = self.ppu_read(attribute_tile);

        /*
            Retriving correct tile

            given the attribute byte 0x76543210, it's represented in the following way
              0    1
            +----+----+
            | 10 | 32 | 0
            +----+----+
            | 54 | 76 | 1
            +----+----+
            
            To extract the correct 2 bits, we can use bit manipulation. Since we know coarse x and y, we can figure out the 2 bits.
        */

        if self.v.get_coarse_yscroll() % 2 != 0{
            attribute_byte >>= 4; //This tells us our tile is on the bottom, so we get rid of the bottom 4 bits
        }
        if self.v.get_coarse_xscroll() % 2 != 0{
            attribute_byte >>= 2; //This tells us that our tile is on the right, so we shift by two.
        }
        attribute_byte = attribute_byte & 3; //we mask our result in case none of the following are true.

        /* Retrieve the low and high byte, then load it on the attribute shift register. */
        let attribute_byte_lo = if attribute_byte & 1 != 0 {0xFF} else {0}; //the lsb is 1, so all the values in the shift register is 1.
        let attribute_byte_hi = if attribute_byte & 2 != 0 {0xFF} else {0};
        self.attribute_hi_shift_register = (self.attribute_hi_shift_register & 0xFF00) |  attribute_byte_hi;
        self.attribute_lo_shift_register = (self.attribute_lo_shift_register & 0xFF00) | attribute_byte_lo;
        self.increment_x();
        
    }

    fn get_first_tile(&mut self){
        /* Retrieve nametable and attribute table */
        let (nametable_tile,attribute_tile) = self.get_tiles();
        
        /* Calculate address for pallette  */
        let address = if self.ppuctrl.contains(PPUCTRL::background_pattern_table_address) {0x1000} else {0} as u16;
        let address = address | (nametable_tile << 4) | (self.v.get_fine_y() as u16);

        /* Retrieve palette bytes */

        /* Hi byte */
        let pattern_table_hibyte = self.ppu_read(address + 8) as u16;
        self.pattern_hi_shift_register = pattern_table_hibyte << 8;

        /* Lo byte */
        let pattern_table_lobyte = self.ppu_read(address) as u16;
        self.pattern_lo_shift_register = pattern_table_lobyte << 8;


        /* Retrieve attribute table bytes */

        let mut attribute_byte = self.ppu_read(attribute_tile);

        /*
            Retriving correct tile

            given the attribute byte 0x76543210, it's represented in the following way
              0    1
            +----+----+
            | 10 | 32 | 0
            +----+----+
            | 54 | 76 | 1
            +----+----+
            
            To extract the correct 2 bits, we can use bit manipulation. Since we know coarse x and y, we can figure out the 2 bits.
        */

        if self.v.get_coarse_yscroll() & 2 != 0{
            attribute_byte >>= 4; //This tells us our tile is on the bottom, so we get rid of the bottom 4 bits
        }
        if self.v.get_coarse_xscroll() & 2 != 0{
            attribute_byte >>= 2; //This tells us that our tile is on the right, so we shift by two.
        }
        attribute_byte = attribute_byte & 3; //we mask our result in case none of the following are true.

        /* Retrieve the low and high byte, then load it on the attribute shift register. */
        let attribute_byte_lo = if attribute_byte & 1 != 0 {0xFF} else {0}; //the lsb is 1, so all the values in the shift register is 1.
        let attribute_byte_hi = if attribute_byte & 2 != 0 {0xFF} else {0};
        self.attribute_hi_shift_register = attribute_byte_hi << 8;
        self.attribute_lo_shift_register = attribute_byte_lo << 8;
        self.increment_x(); //we increment the x register to finish off.
    }
    fn prime_shift_registers(&mut self){
        self.get_first_tile();
        self.get_second_tile();
    }
    pub fn render_scanline(&mut self, scanline: u16){
        self.prime_shift_registers(); //primes our shift register to do stuff
        for x in 0..256{ //iterates through all horizontal pixels

            /*
                every 8th tick, we want to reload the next tile onto the shift register.
                We would load the new tile, attribute table, and load the pattern stuff on
                the shift register.
            */
            if x > 0 && x % 8 == 0{
                self.get_second_tile(); //basically does the job for us.
            }

            /* Read the values from our shift registers for rendering */
            let finex = (0x8000 >> self.x) as u16;
            let pattern_lobit = if self.pattern_lo_shift_register & finex > 0 {1} else {0};
            let pattern_hibit = if self.pattern_hi_shift_register & finex > 0 {1} else {0};
            let attribute_lobit = if self.attribute_lo_shift_register & finex > 0 {1} else {0};
            let attribute_hibit = if self.attribute_hi_shift_register > 0 {1} else {0};

            /* Calculate the attribute bit */
            let attribute_index = (attribute_hibit << 1) | attribute_lobit;
            let pattern_index = (pattern_hibit << 1) | pattern_lobit;
            unsafe{
                (*self.nametable_buffer.unwrap()).drawpixel(x, scanline, self.get_fgpalette(attribute_index, pattern_index));
            }
            /* Update shift registers */
            self.attribute_hi_shift_register <<= 1;
            self.attribute_lo_shift_register <<= 1;
            self.pattern_hi_shift_register <<= 1;
            self.pattern_lo_shift_register <<= 1;
        }
        self.increment_y();
    }

    pub fn update_frame(&mut self){
        self.v.set_data(self.t.get_data());
        for i in 0..240{
            self.render_scanline(i);
            self.v.set_coarse_xscroll(self.t.get_coarse_xscroll());
            self.v.set_nametablex(self.t.get_nametablex());
        }

    }

    pub fn render_88_sprite(&mut self, index: usize) {
        let oam_sprite = self.oam_table[index].clone();
        let x = oam_sprite.get_x_position() as usize;
        let mut y = oam_sprite.get_y_position() as usize;
        if y != 0 {
            y = y - 1;
        }
        let index = oam_sprite.get_index_number() as u16;
        let attribute = oam_sprite.get_attribute();
        let horizontal_factor = attribute & 0x40 > 0;
        let vertical_factor = attribute & 0x80 > 0;
        let attrib_table = attribute & 0x3;
        let x_pat = index & 0xF;
        let y_pat = index >> 4;
        let y_pat = y_pat * 8;
        let x_pat = x_pat * 8;
        let table_index = if self.ppuctrl.contains(PPUCTRL::sprite_pattern_table_address) {1} else {0};
        for i in 0..8{
            for j in 0..8{
                let pixel_num = self.pattern_table[table_index][x_pat as usize + i][y_pat as usize + j];
                let color = self.get_fgpalette(attrib_table & 3, pixel_num);
                if x < 256 && y < 240 {
                    let j_factor = if vertical_factor {7 - j} else {j as usize};
                    let i_factor = if horizontal_factor {7 - i} else {i as usize}; /* x axis render */
                    let y = y + j_factor + 2;
                    let x = x + i_factor;
                    if pixel_num != 0{
                        if self.whole_frame[x as usize][y as usize].1 != 0 && index == 0{
                            self.ppustatus.set(PPUSTATUS::sprite_0_hit_flag, true);
                        }
                        unsafe {(*self.nametable_buffer.unwrap()).drawpixel(x as u16, y as u16, color)};
                    }
                }
            }
        }

    }
    pub fn set_oam_table(&mut self) {
        if self.ppuctrl.contains(PPUCTRL::sprite_size){

        }
        else{
            for i in 0..64 {
                self.render_88_sprite(i);
            }
        }
    }
    pub fn set_name_table(&mut self) {
        self.update_frame();
        self.set_oam_table();
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
            if self.ppuctrl.contains(PPUCTRL::vblank_enable) {
                self.nmi = true;
            }
        } else if self.scanline_counter == 261 && self.cycle_counter == 1 {
            self.ppustatus.set(PPUSTATUS::vblank_flag, false);
            self.ppustatus.set(PPUSTATUS::sprite_0_hit_flag,false);
            self.ppustatus.set(PPUSTATUS::sprite_overflow_flag,false);
            self.scanline_counter = 0;

        }
        self.total_cycles = self.total_cycles.wrapping_add(1);
    }
}
