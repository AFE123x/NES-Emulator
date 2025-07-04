mod loopy;
mod oam;

use core::panic;
use std::cell::RefCell;
use std::rc::Rc;

use crate::ppu::oam::Oam;
use frame::Frame;
use registers::{VtReg, PPUCTRL, PPUMASK, PPUSTATUS};

use crate::cartridge::{Cartridge, MirrorMode};

pub mod frame;
mod registers;
///# PPU 
/// ## Picture Processing Unit
/// Handles rendering the 256 x 240 video on the NES
pub struct Ppu {
    ppuctrl: PPUCTRL,     //ppu control register (mapped at address $2000)
    ppumask: PPUMASK,     //ppu mask register (mapped at address $2001)
    ppustatus: PPUSTATUS, //ppu status register (mapped at address $2002)
    oamaddr: u8,          //oamaddr register (mapped at address $2003)
    v: VtReg,
    t: VtReg,
    w: u8, //toggle between first and second write
    x: u8, //fine x scroll
    vram: Vec<u8>,
    internal_buffer: u8,
    nmi: bool,
    cart: Rc<RefCell<Cartridge>>,
    palette_memory: Vec<u8>,
    system_palette: Vec<(u8, u8, u8)>,
    palette_num: u8,
    cycle_counter: u16,
    scanline_counter: i16,

    total_cycles: usize,
    frame_array: Vec<Vec<u8>>,
    oam_table: Vec<Oam>,
    /* Shift registers */
    pattern_lo_shift_register: u16,
    pattern_hi_shift_register: u16,
    attribute_lo_shift_register: u16,
    attribute_hi_shift_register: u16,

    /* Holding attribute and pattern bytes */
    next_pattern_lo: u16,
    next_pattern_hi: u16,
    next_attribute_lo: u16,
    next_attribute_hi: u16,
    next_nametable_tile: u16,
    next_attribute_tile: u16,
    sprite0xcoord: u16,
    sprite0ycoord: u16,
    sprite0poss: bool,
}

impl Ppu {
    ///# `initialize_system_palette()`
    /// - initializes the system palette
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

    /// # `get_nmi`
    /// - checks if the picture processing unit invokes a non-maskable interrupt.
    /// - utilizes test and set method
    pub fn get_nmi(&mut self) -> bool {
        let data = self.nmi;
        if data {
            self.nmi = false;
        }
        data
    }
    ///# `new(cartridge)`
    /// Constructor creating new PPU instance
    pub fn new(cartridge: Rc<RefCell<Cartridge>>) -> Self {
        let pal: Vec<u8> = vec![0; 0x20];
        let vram: Vec<u8> = vec![0; 2048];
        Self {
            ppuctrl: PPUCTRL::empty(),
            ppumask: PPUMASK::empty(),
            ppustatus: PPUSTATUS::empty(),
            oamaddr: 0,
            v: VtReg::new(),
            t: VtReg::new(),
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
            total_cycles: 0,
            oam_table: vec![Oam::new(); 64],
            pattern_lo_shift_register: 0,
            pattern_hi_shift_register: 0,
            attribute_lo_shift_register: 0,
            attribute_hi_shift_register: 0,
            frame_array: vec![vec![0; 240]; 256],
            next_pattern_lo: 0,
            next_pattern_hi: 0,
            next_attribute_lo: 0,
            next_attribute_hi: 0,
            next_nametable_tile: 0,
            next_attribute_tile: 0,
            sprite0xcoord: 0,
            sprite0ycoord: 0,
            sprite0poss: false,
        }
    }

    ///# `set_bg_palette_num()`
    /// Toggles the pallette table for debugging pattern table
    pub fn set_bg_palette_num(&mut self) {
        self.palette_num = (self.palette_num + 1) & 0xF;
    }

    /// # `oam_dma_write(address, data)`
    /// Copies byte from cpu memory to object-attribute memory
    pub fn oam_dma_write(&mut self, address: u8, data: u8) {
        let sprite_index = (address / 4) as usize;
        let sprite_offset = address % 4;
        self.oam_table[sprite_index].set_byte(sprite_offset, data);
    }
    /// # `find_sprite0_coord()`
    /// - Finds the location of sprite 0 hit flag.
    /// - Determines which point to enable the sprite_0_hit_flag register.
    pub fn find_sprite0_coord(&mut self) {
        let mut sprite = self.oam_table[0].clone();
        let y_pos = sprite.get_byte(0) as u16;
        let tile_index = sprite.get_byte(1) as u16;
        let attributes = sprite.get_byte(2);
        let x_pos = sprite.get_byte(3) as u16;
        let flip_horizontal = (attributes & 0x40) != 0;
        let flip_vertical = (attributes & 0x80) != 0;

        if self.ppuctrl.contains(PPUCTRL::sprite_size) { //8 x 16
            //println!("8 x 16 rendering");
            let pattern_table_base: u16 = if tile_index & 0x01 != 0 {
                0x1000
            } else {
                0x0000
            };
            let tile_number = tile_index & 0xFE;
            for tile_half in 0..2 {
                let current_tile = if flip_vertical {
                    tile_number + (1 - tile_half)
                } else {
                    tile_number + tile_half
                };
                let tile_address = pattern_table_base + (current_tile * 16);
                for row in 0..8 {
                    let y_in_tile = if flip_vertical { 7 - row } else { row };
                    let low_byte = self.ppu_read(tile_address + y_in_tile);
                    let high_byte = self.ppu_read(tile_address + y_in_tile + 8);
                    for col in 0..8 {
                        let x_in_tile = if flip_horizontal { 7 - col } else { col };
                        let low_bit = (low_byte >> (7 - x_in_tile)) & 0x01;
                        let high_bit = (high_byte >> (7 - x_in_tile)) & 0x01;
                        let pixel_value = (high_bit << 1) | low_bit;
                        if pixel_value != 0 {
                            let y_offset = (tile_half * 8) as u16 + row as u16;

                            self.sprite0xcoord = x_pos + col as u16;
                            self.sprite0ycoord = y_pos + y_offset;
                            self.sprite0poss = true;
                            return;
                        }
                    }
                }
            }
        } else {
            let pattern_table_base: u16 =
                if self.ppuctrl.contains(PPUCTRL::sprite_pattern_table_address) {
                    0x1000
                } else {
                    0x0000
                };
            let tile_address = pattern_table_base + (tile_index * 16);
            for row in 0..8 {
                let y_in_tile = if flip_vertical { 7 - row } else { row };
                let low_byte = self.ppu_read(tile_address + y_in_tile);
                let high_byte = self.ppu_read(tile_address + y_in_tile + 8);
                for col in 0..8 {
                    let x_in_tile = if flip_horizontal { 7 - col } else { col };
                    let low_bit = (low_byte >> (7 - x_in_tile)) & 0x01;
                    let high_bit = (high_byte >> (7 - x_in_tile)) & 0x01;
                    let pixel_value = (high_bit << 1) | low_bit;
                    if pixel_value != 0 {
                        self.sprite0xcoord = x_pos + col as u16;
                        self.sprite0ycoord = y_pos + row as u16;
                        self.sprite0poss = true;
                        return;
                    }
                }
            }
        }
        self.sprite0xcoord = x_pos;
        self.sprite0ycoord = y_pos;
        self.sprite0poss = false;
    }
    ///# `get_bgpalette(palettenum, paletteindex`
    /// Retrieves the correct bg color to draw in the clock() function
    pub fn get_bgpalette(&mut self, palettenum: u8, paletteindex: u8) -> (u8, u8, u8) {
        let palettenum = palettenum & 0x07;
        let palette_addr = if paletteindex == 0 {
            0x3F00
        } else {
            0x3F00 + ((palettenum as u16) * 4) + (paletteindex as u16)
        };
        let palette_addr = if self.ppumask.contains(PPUMASK::greyscale) {
            0x3F00 + (palette_addr & 0xF)
        } else {
            palette_addr
        };
        let color_index = self.ppu_read(palette_addr);
        let safe_index = (color_index & 0x3F) as usize;
        self.system_palette[safe_index]
    }
    ///# `get_fgpalette(palettenum, paletteindex`
    /// Retrieves the correct fg color to draw in the draw_scanline function
    pub fn get_fgpalette(&mut self, palettenum: u8, paletteindex: u8) -> (u8, u8, u8) {
        let palettenum = palettenum & 0x07;
        let palette_addr = if paletteindex == 0 {
            0x3F00
        } else {
            0x3F10 + ((palettenum as u16) * 4) + (paletteindex as u16)
        };
        let color_index = self.ppu_read(palette_addr);
        let safe_index = (color_index & 0x3F) as usize;
        self.system_palette[safe_index]
    }
    ///# `get_pattern_table(frame)`
    /// - draws the pattern table to frame
    pub fn get_pattern_table(&mut self, frame: &mut Frame) {
        for coarse_y in 0..0x10 {
            for coarse_x in 0..0x20 {
                let pattern_address = if coarse_x >= 0x10 { 0x1000 } else { 0 };
                let nametable_location = coarse_y << 4 | (coarse_x & 0xF);
                for fine_y in 0..8 {
                    let address = pattern_address | (nametable_location << 4) | fine_y;
                    let mut pattern_lo = self.ppu_read(address);
                    let mut pattern_hi = self.ppu_read(address + 8);
                    for fine_x in 0..8 {
                        let bitlo = if pattern_lo & 0x80 > 0 { 1 } else { 0 };
                        let bithi = if pattern_hi & 0x80 > 0 { 1 } else { 0 };
                        let pattern_number = (bitlo << 1) | bithi;
                        let x = (coarse_x << 3) + fine_x;
                        let y = (coarse_y << 3) + fine_y;
                        let color = self.get_bgpalette(self.palette_num, pattern_number);
                        frame.drawpixel(x + 256, y, color);
                        pattern_lo <<= 1;
                        pattern_hi <<= 1;
                    }
                }
            }
        }
    }
    ///# `ppu_read(address)`
    /// - handle ppu reads.
    fn ppu_read(&self, address: u16) -> u8 {
        let mut byte = 0;

        if address <= 0x1FFF {
            self.cart.borrow_mut().ppu_read(address, &mut byte);
        } else if address >= 0x2000 && address <= 0x2FFF {

            let nametable = self.cart.borrow_mut().get_nametable();
            byte = match nametable {
                MirrorMode::Vertical => {
                    let index = match address {
                        0x2000..=0x23FF => address & 0x3FF,
                        0x2800..=0x2BFF => address & 0x3FF,
                        0x2400..=0x27FF => 0x400 + (address & 0x3FF),
                        0x2C00..=0x2FFF => 0x400 + (address & 0x3FF),
                        _ => panic!("Address out of range!"),
                    };
                    self.vram[index as usize]
                }
                MirrorMode::Horizontal => {
                    let index = match address {
                        0x2000..=0x23FF | 0x2400..=0x27FF => address & 0x3FF,
                        0x2800..=0x2BFF | 0x2C00..=0x2FFF => 0x400 + (address & 0x3FF),
                        _ => panic!("Address out of range!"),
                    };
                    self.vram[index as usize]
                }
                MirrorMode::OneScreenLo => {
                    let index = address & 0x3FF;
                    self.vram[index as usize]
                }
                MirrorMode::OneScreenHi => {
                    let index = address & 0x3FF;
                    let index = index.wrapping_add(0x400);
                    self.vram[index as usize]
                }
            };
        } else if address >= 0x3000 && address <= 0x3EFF {
            // Mirror of 0x2000 - 0x2EFF
            byte = self.ppu_read(address - 0x1000);
        } else if address >= 0x3F00 && address <= 0x3FFF {
            let mut addr = address & 0x001F;
            if addr == 0x0010 {
                addr = 0x0000;
            }
            if addr == 0x0014 {
                addr = 0x0004;
            }
            if addr == 0x0018 {
                addr = 0x0008;
            }
            if addr == 0x001C {
                addr = 0x000C;
            }

            byte = self.palette_memory[addr as usize];
        } else {
            // Open bus or undefined memory area
            byte = 0;
        }

        byte
    }
    ///# `ppu_write()`
    /// - Handle PPU Writes
    fn ppu_write(&mut self, address: u16, data: u8) {
        if address <= 0x1FFF {
            self.cart.borrow_mut().ppu_write(address, data);
        } else if address >= 0x2000 && address <= 0x2FFF {
            /* nametable writes */
            let nametable = self.cart.borrow_mut().get_nametable();
            match nametable {
                MirrorMode::Vertical => {
                    match address {
                        0x2000..=0x23FF => {
                            /* nametable 0 */
                            let addr = address & 0x3FF;
                            self.vram[addr as usize] = data;
                        }
                        0x2800..=0x2BFF => {
                            /* nametable 1 */
                            // address & 0x3FF;
                            let addr = address & 0x3FF;
                            self.vram[addr as usize] = data;
                        }
                        0x2400..=0x27FF => {
                            /* nametable 2 */
                            let addr = 0x400 + (address & 0x3FF);
                            self.vram[addr as usize] = data;
                        }
                        0x2C00..=0x2FFF => {
                            /* nametable 3 */
                            let addr = 0x400 + (address & 0x3FF);
                            self.vram[addr as usize] = data;
                        }
                        _ => panic!("Address out of range!"),
                    };
                }
                MirrorMode::Horizontal => {
                    let index = match address {
                        0x2000..=0x23FF | 0x2400..=0x27FF => address & 0x3FF,
                        0x2800..=0x2BFF | 0x2C00..=0x2FFF => 0x400 + (address & 0x3FF),
                        _ => panic!("Address out of range!"),
                    };
                    self.vram[index as usize] = data;
                }
                MirrorMode::OneScreenLo => {
                    let index = address & 0x3FF;
                    self.vram[index as usize] = data;
                }
                MirrorMode::OneScreenHi => {
                    let index = address & 0x3FF;
                    let index = index.wrapping_add(0x400);
                    self.vram[index as usize] = data;
                }
            };
        } else if address >= 0x3F00 && address <= 0x3FFF {
            let mut addr = address & 0x001F;
            if addr == 0x0010 {
                addr = 0x0000;
            }
            if addr == 0x0014 {
                addr = 0x0004;
            }
            if addr == 0x0018 {
                addr = 0x0008;
            }
            if addr == 0x001C {
                addr = 0x000C;
            }
            self.palette_memory[addr as usize] = data;
        } else {
            // todo!()
        }
    }

    ///# cpu_read
    /// This function lets the cpu read from the PPU Address space.
    /// ## Addresses
    /// 2: Read from the PPU Status Register
    /// 4: Read oam data
    /// 7: read value from ppu from ppu address set in the PPUADDR Register
    pub fn cpu_read(&mut self, address: u16, rdonly: bool) -> u8 {
        let mut _data = 0;
        let masked_address = address & 0x7;
        match masked_address {
            0 | 1 | 3 | 5 | 6 => {
                _data = 0;
            }
            2 => {
                _data = self.ppustatus.bits();
                self.ppustatus.set(PPUSTATUS::vblank_flag, false);
                self.w = 0;
            }
            4 => {
                _data = self.oam_table[(self.oamaddr >> 2) as usize].get_byte(self.oamaddr);
            }
            7 => {
                _data = self.internal_buffer;
                self.internal_buffer = self.ppu_read(self.v.get_data());
                if address >= 0x3F00 && address <= 0x3FFF {
                    _data = self.internal_buffer;
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
        _data
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
                self.t.set_nametable(data & 3);
            }
            1 => {
                self.ppumask = PPUMASK::from_bits_truncate(data);
            }
            2 => {}
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
                    self.x = data & 7;
                    self.w = 1;
                } else if self.w == 1 {
                    self.t.set_fine_y(data & 7);
                    self.t.set_coarse_yscroll(data >> 3);
                    self.w = 0;
                }
            }
            6 => {
                if self.w == 0 {
                    let temp_dat = data as u16;
                    let temp_val = self.t.get_data();
                    let temp_val = temp_val & 0x00FF;
                    let temp_val = temp_val | (temp_dat << 8);
                    self.t.set_data(temp_val);
                    self.w = 1;
                } else if self.w == 1 {
                    let temp_data = self.t.get_data();
                    let temp_data = temp_data & 0xFF00;
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

    fn get_pattern_address(&self) -> u16 {
        let toreturn = if self
            .ppuctrl
            .contains(PPUCTRL::background_pattern_table_address)
        {
            0x1000
        } else {
            0x0000
        };
        let finey = self.v.get_fine_y() as u16;
        let toreturn = toreturn | (self.next_nametable_tile << 4) | finey;
        toreturn
    }

/// # `render_88_sprite(&mut self, index: usize, scanline: u16)`
/// Renders an 8x8 sprite on the given scanline.
/// 
/// This function processes the sprite from OAM at the given index, 
/// and draws it on the `scanline` if it intersects with it.
/// It considers sprite flipping, priority, transparency, and pattern table selection.
/// 
/// # Arguments
/// * `index` - Index of the sprite in the OAM table (0–63)
/// * `scanline` - The current scanline being rendered
pub fn render_88_sprite(&mut self, index: usize, scanline: u16, nametable_frame: &mut Frame) {
    let oam_sprite = self.oam_table[index].clone();
    let sprite_x = oam_sprite.get_x_position() as u16;
    let sprite_y = oam_sprite.get_y_position() + 1; // Sprites are offset by one scanline
    let tile_index = oam_sprite.get_index_number() as u16;
    let attribute = oam_sprite.get_attribute();
    let flip_horizontal = attribute & 0x40 > 0;
    let flip_vertical = attribute & 0x80 > 0;
    let palette = attribute & 0x3;
    let behind_background = attribute & 0x20 > 0;
    if scanline < sprite_y || scanline >= sprite_y + 8 || sprite_y >= 238 {
        return;
    }
    let pattern_table = if self.ppuctrl.contains(PPUCTRL::sprite_pattern_table_address) {
        0x1000
    } else {
        0
    };
    let tile_base = pattern_table | (tile_index << 4);
    let sprite_y_offset = scanline - sprite_y;
    let row = sprite_y_offset;
    let effective_row = if flip_vertical { 7 - row } else { row };
    let pattern_lo = self.ppu_read(tile_base + effective_row);
    let pattern_hi = self.ppu_read(tile_base + effective_row + 8);
    for col in 0..8 {
        let effective_col = if flip_horizontal { 7 - col } else { col };
        let pixel_bit_lo = (pattern_lo >> (7 - effective_col)) & 1;
        let pixel_bit_hi = (pattern_hi >> (7 - effective_col)) & 1;
        let pixel_value = (pixel_bit_hi << 1) | pixel_bit_lo;
        if pixel_value == 0 {
            continue;
        }
        let screen_x = (sprite_x + col) as u16;
        let screen_y = scanline;
        if screen_x >= 256 {
            continue;
        }
        let color =
            if !self.ppumask.contains(PPUMASK::sprites_leftmost) && self.cycle_counter < 8 {
                self.get_fgpalette(palette, 0)
            } else {
                self.get_fgpalette(palette, pixel_value)
            };
        if screen_x < 256 && screen_y < 240 {
            let bg_pixel = self.frame_array[screen_x as usize][screen_y as usize];
            let should_draw = if behind_background {
                bg_pixel == 0
            } else {
                true
            };

            if should_draw {
                if self.ppumask.contains(PPUMASK::enable_sprite_rendering) {
                    nametable_frame.drawpixel(
                            screen_x as u16,
                            screen_y as u16,
                            color,
                        );
                    self.frame_array[screen_x as usize][screen_y as usize] = pixel_value;
                }
            }
        }
    }
}

    /// # `render_816_sprite(&mut self, index: usize, scanline: u16)`
    /// Renders a single 8x16 sprite onto the current scanline.
    /// 
    /// This function checks if the given sprite index should appear on the current scanline.
    /// It extracts pattern data based on sprite attributes (including flipping and priority),
    /// calculates the correct pattern address for the tile row, and renders non-transparent pixels
    /// to the frame buffer with proper background priority handling.
    ///
    /// - `index`: Index into the OAM table (0–63)
    /// - `scanline`: Current scanline being rendered (0–239)   
    pub fn render_816_sprite(&mut self, index: usize, scanline: u16, nametable_frame: &mut Frame) {
        let oam_sprite = self.oam_table[index].clone();
        let x = oam_sprite.get_x_position();
        let y = oam_sprite.get_y_position() + 1;
        let tile_index = oam_sprite.get_index_number() as u16;
        let attribute = oam_sprite.get_attribute();
        let flip_horizontal = attribute & 0x40 > 0;
        let flip_vertical = attribute & 0x80 > 0;
        let palette_idx = attribute & 0x3;
        let behind_background = attribute & 0x20 > 0;

        // Skip if sprite is not on this scanline
        if scanline < y || scanline >= y + 16 {
            return;
        }

        // For 8x16 sprites, bit 0 of the tile index selects the pattern table
        let pattern_table = if tile_index & 1 == 0 { 0 } else { 0x1000 };
        let tile_number = tile_index & 0xFE; // Remove bit 0 as it's used for pattern table

        // Determine which of the two tiles contains the scanline
        let sprite_y_offset = scanline - y;
        let tile = (sprite_y_offset / 8) as usize;
        let row_in_tile = sprite_y_offset % 8;

        // Process the specific tile that contains this scanline
        let effective_tile = if flip_vertical { 1 - tile } else { tile };

        // Calculate base address for this tile
        let tile_base = pattern_table | ((tile_number + effective_tile as u16) << 4);

        // Process the specific row in this tile
        let effective_row = if flip_vertical {
            7 - row_in_tile
        } else {
            row_in_tile
        };
        let pattern_address = tile_base + effective_row;

        let pattern_lo = self.ppu_read(pattern_address);
        let pattern_hi = self.ppu_read(pattern_address + 8);

        // Process the 8 pixels in this row
        for col in 0..8 {
            let effective_col = if flip_horizontal { 7 - col } else { col };

            // Extract pixel data
            let bit_lo = (pattern_lo >> (7 - effective_col)) & 1;
            let bit_hi = (pattern_hi >> (7 - effective_col)) & 1;
            let pixel_value = (bit_hi << 1) | bit_lo;

            // Skip transparent pixels
            if pixel_value == 0 {
                continue;
            }

            // Calculate screen coordinates
            let screen_x = ((x as u16) + col) as u16;
            // We already know the screen_y is the scanline
            let screen_y = scanline;

            // Skip if off-screen
            if screen_x >= 256 || screen_y >= 240 {
                continue;
            }

            // Get color and render
            let color =
                if !self.ppumask.contains(PPUMASK::sprites_leftmost) && self.cycle_counter < 8 {
                    self.get_fgpalette(palette_idx, 0)
                } else {
                    self.get_fgpalette(palette_idx, pixel_value)
                };
            // Apply priority rules correctly
            let bg_pixel = self.frame_array[screen_x as usize][screen_y as usize];

            // Proper NES priority logic:
            // - If behind_background flag is set and bg_pixel is not transparent (0),
            //   then background has priority
            // - Otherwise, sprite has priority
            let should_draw = if behind_background {
                bg_pixel == 0 // Only draw if background is transparent
            } else {
                true // Always draw if not behind background
            };

            if should_draw {
                    if self.ppumask.contains(PPUMASK::enable_sprite_rendering) {
                        nametable_frame.drawpixel(
                            screen_x as u16,
                            screen_y as u16,
                            color,
                        );
                        self.frame_array[screen_x as usize][screen_y as usize] = pixel_value;
                    }
            }
        }
    }
    ///# `clock(&mut self)`
    /// Executes one PPU cycle, handling background and sprite rendering, scroll updates, and VBlank events.
    ///
    /// This function simulates a single clock cycle of the NES PPU. It performs:
    /// - Background tile fetching and shift register updates (visible and pre-render scanlines).
    /// - Scanline-based rendering of background pixels to the frame buffer.
    /// - Sprite evaluation at the end of each visible scanline (cycle 257), determining up to 8 sprites.
    /// - 8x8 or 8x16 sprite rendering based on `PPUCTRL` sprite size flag.
    /// - Sprite 0 hit detection logic (for gameplay effects like "start flashing").
    /// - VBlank handling and NMI signaling at the start of scanline 241.
    /// - Internal VRAM address transfers and vertical/horizontal scrolling logic.
    ///
    /// The function should be called once per PPU clock cycle (~89342 times per frame at NTSC).
    ///
    /// It also tracks `total_cycles`, `scanline_counter`, and `cycle_counter` internally.
    pub fn clock(&mut self, frame: &mut Frame) {
        /* Background Rendering */
        if self.scanline_counter >= -1 && self.scanline_counter < 240 {
            /* Pre Render and Visible Scanlines */

            if self.cycle_counter == 0 {
                self.cycle_counter = 1; /* We skip this cycle */
            }
            if (self.cycle_counter >= 1 && self.cycle_counter < 256)
                || (self.cycle_counter >= 321 && self.cycle_counter <= 338)
            {
                match (self.cycle_counter - 1) % 8 {
                    0 => {
                        self.loadregisters();
                        self.next_nametable_tile =
                            self.ppu_read(self.v.get_nametable_address()) as u16;
                    }
                    1 => {}
                    2 => {
                        // Fetch attribute byte
                        let attr_byte = self.ppu_read(self.v.get_attribute_address());
                        self.next_attribute_tile = attr_byte as u16;

                        // Select correct 2-bit palette group

                        // Calculate quadrant selection (0-3)
                        if self.v.get_coarse_yscroll() & 2 != 0 {
                            self.next_attribute_tile >>= 4;
                        }
                        if self.v.get_coarse_xscroll() & 2 != 0 {
                            self.next_attribute_tile >>= 2;
                        }
                        self.next_attribute_tile &= 3;
                    }
                    3 => {}
                    4 => {
                        // Fetch pattern LSB
                        let lo_address = self.get_pattern_address();
                        self.next_pattern_lo = self.ppu_read(lo_address) as u16;
                        self.next_attribute_lo = if self.next_attribute_tile & 1 > 0 {
                            0xFF
                        } else {
                            0x00
                        };
                    }
                    5 => {}
                    6 => {
                        // Fetch pattern MSB
                        let hi_address = self.get_pattern_address() + 8;
                        self.next_pattern_hi = self.ppu_read(hi_address) as u16;
                        self.next_attribute_hi = if self.next_attribute_tile & 2 > 0 {
                            0xFF
                        } else {
                            0x00
                        };
                    }
                    7 => {
                        self.increment_x();
                    }
                    _ => unreachable!(),
                }
            }
            if self.cycle_counter == 256 {
                self.increment_y();
                self.transfer_x();
            }
        }
        if self.scanline_counter == -1 && self.cycle_counter >= 280 && self.cycle_counter <= 304 {
            self.transfer_y(); /* We transfer the vertical factor from the T register here. */
        }

        if self.ppumask.contains(PPUMASK::enable_background_rendering)
            || self.ppumask.contains(PPUMASK::enable_sprite_rendering)
        {
            let mux = 0x8000 >> (self.x as u16); /* Our multiplexer to select the bit */
            let pixel_lo = if self.pattern_lo_shift_register & mux != 0 {
                1
            } else {
                0
            };
            let pixel_hi = if self.pattern_hi_shift_register & mux != 0 {
                1
            } else {
                0
            };
            let attrib_lo = if self.attribute_lo_shift_register & mux != 0 {
                1
            } else {
                0
            };
            let attrib_hi = if self.attribute_hi_shift_register & mux != 0 {
                1
            } else {
                0
            };
            let bgpixel = (pixel_hi << 1) | (pixel_lo);
            let bgpattern = (attrib_hi << 1) | (attrib_lo);
            self.shift();
            if self.cycle_counter < 256 && self.scanline_counter >= 0 && self.scanline_counter < 240
            {
                let color = if !self.ppumask.contains(PPUMASK::sprites_leftmost)
                    && self.cycle_counter < 8
                {
                    self.get_bgpalette(0, 0)
                } else {
                    self.get_bgpalette(bgpattern & 3, bgpixel)
                };

                //TODO: implement PPUMASK color emphasis
                // Store the background pixel value for sprite priority comparisons
                if self.cycle_counter > 0 {
                    let x = self.cycle_counter - 1;
                    let y = self.scanline_counter;
                    if x < 256 && y < 240 {
                        self.frame_array[x as usize][y as usize] = bgpixel;
                        if self.ppumask.contains(PPUMASK::enable_background_rendering) {
                            frame.drawpixel(x, y as u16, color);
                        }
                    }
                }
            }
        }

        // Modified sprite rendering section - moved to the end of scanline
        if self.cycle_counter == 257 && self.scanline_counter >= 0 && self.scanline_counter < 240 {
            let current_scanline = self.scanline_counter as u16;
            let mut sprite_count = 0;
            let mut visible_sprites = Vec::new();

            // First pass: collect all sprites that are on this scanline
            for i in 0..64 {
                let sprite_y = self.oam_table[i].get_y_position() + 1;
                let sprite_height = if self.ppuctrl.contains(PPUCTRL::sprite_size) {
                    16
                } else {
                    8
                };

                // Check if this sprite is on the current scanline
                if current_scanline >= sprite_y as u16
                    && current_scanline < (sprite_y + sprite_height) as u16
                {
                    visible_sprites.push(i);
                    sprite_count += 1;

                    if sprite_count > 8 {
                        // Set overflow flag but keep checking for the rest of the sprites
                        self.ppustatus.set(PPUSTATUS::sprite_overflow_flag, true);
                        break;
                    }
                }
            }

            // Second pass: render sprites in reverse order (so sprite 0 has highest priority)
            visible_sprites.reverse();
            for &sprite_index in &visible_sprites {
                if self.ppuctrl.contains(PPUCTRL::sprite_size) {
                    self.render_816_sprite(sprite_index, current_scanline,frame);
                } else {
                    self.render_88_sprite(sprite_index, current_scanline, frame);
                }
            }
        }
        // Sprite 0 hit detection
        if self.scanline_counter.wrapping_sub(1) == self.sprite0ycoord as i16
            && self.cycle_counter == self.sprite0xcoord as u16 && self.sprite0poss
        {
            self.ppustatus.set(PPUSTATUS::sprite_0_hit_flag, true);
        }

        if self.ppumask.contains(PPUMASK::enable_background_rendering)
            || self.ppumask.contains(PPUMASK::enable_sprite_rendering)
        {
            if self.cycle_counter == 260 && self.scanline_counter < 240 {
                self.cart.borrow_mut().scanline();
            }
        }

        /* Incrementing Logic */
        self.cycle_counter += 1;
        if self.cycle_counter > 340 {
            self.cycle_counter = 0;
            self.scanline_counter += 1;
        }

        if self.scanline_counter <= 239 {
            // Visible scanlines
        } else if self.scanline_counter == 241 && self.cycle_counter == 1 {
            // for i in 0..8{
            //     for j in 0..8{
            //         frame.drawpixel(self.sprite0xcoord.wrapping_add(i), self.sprite0ycoord.wrapping_add(j), (255,0,0));
            //     }
            // }
            self.ppustatus.set(PPUSTATUS::vblank_flag, true);
            if self.ppuctrl.contains(PPUCTRL::vblank_enable) {
                self.nmi = true;
            }
        }
        if self.scanline_counter == -1 && self.cycle_counter == 1 {
            self.scanline_counter = 0;
        }
        if self.scanline_counter > 260 {
            self.scanline_counter = -1;
            self.find_sprite0_coord();
            self.ppustatus.set(PPUSTATUS::vblank_flag, false);
            self.ppustatus.set(PPUSTATUS::sprite_0_hit_flag, false);
            self.ppustatus.set(PPUSTATUS::sprite_overflow_flag, false);
        }
        self.total_cycles = self.total_cycles.wrapping_add(1);
    }
}
