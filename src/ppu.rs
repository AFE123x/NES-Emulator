mod loopy;
mod oam;

use core::panic;
use std::cell::RefCell;
use std::rc::Rc;

use crate::ppu::oam::oam as Oam;
use registers::{vt_reg, PPUCTRL, PPUMASK, PPUSTATUS};

use crate::cartridge::{Cartridge, Nametable};

pub mod Frame;
mod registers;

use crate::ppu::Frame::Frame as Fr;
pub struct Ppu {
    ppuctrl: PPUCTRL,     //ppu control register (mapped at address $2000)
    ppumask: PPUMASK,     //ppu mask register (mapped at address $2001)
    ppustatus: PPUSTATUS, //ppu status register (mapped at address $2002)
    oamaddr: u8,          //oamaddr register (mapped at address $2003)
    v: vt_reg,
    t: vt_reg,
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
    nametable_frame: Option<*mut Fr>,
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
    pub fn new(cartridge: Rc<RefCell<Cartridge>>) -> Self {
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
            total_cycles: 0,
            nametable_frame: None,
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
        }
    }
    pub fn set_bg_palette_num(&mut self) {
        
        self.palette_num = (self.palette_num + 1) & 0xF;
    }
    pub fn oam_dma_write(&mut self, address: u8, data: u8) {
        let index = address / 4;
        self.oam_table[index as usize].set_byte(address, data);
    }
    pub fn get_bgpalette(&mut self, palettenum: u8, paletteindex: u8) -> (u8, u8, u8) {
        let palettenum = palettenum & 0x1F;
        let final_index = (palettenum << 2) | paletteindex;
        let palleteindex = if paletteindex == 0{
            0x3F00
        }
        else{
            0x3F00 | (final_index as u16)
        };
        let paletteinde = self.ppu_read(palleteindex as u16);
        self.system_palette[paletteinde as usize]
    }

    pub fn get_fgpalette(&mut self, palettenum: u8, paletteindex: u8) -> (u8, u8, u8) {
        let palettenum = palettenum & 0x1F;
        let final_index = (palettenum << 2) | paletteindex;
        let paletteinde = self.ppu_read(0x3F10 | final_index as u16);
        self.system_palette[paletteinde as usize]
    }

    pub fn linkpattern(&mut self, frame: &mut Fr) {
        self.nametable_frame = Some(frame);
    }

    pub fn get_pattern_table(&mut self, frame: &mut Fr) {
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
                        let color = self.get_bgpalette(3, pattern_number);
                        frame.drawpixel(x, y, color);
                        pattern_lo <<= 1;
                        pattern_hi <<= 1;
                    }
                }
            }
        }
    }

    fn ppu_read(&self, address: u16) -> u8 {
        let mut byte = 0;

        if address <= 0x1FFF {
            self.cart.borrow_mut().ppu_read(address, &mut byte);
            // unsafe { (*self.cart).ppu_read(address, &mut byte) }; // Reads from cartridge space
        } else if address >= 0x2000 && address <= 0x2FFF {
            // let nametable: Nametable = unsafe { (*self.cart).get_nametable() };
            let nametable = self.cart.borrow_mut().get_nametable();
            byte = match nametable {
                Nametable::Vertical => {
                    let index = match address {
                        0x2000..=0x23FF => address & 0x3FF,
                        0x2800..=0x2BFF => address & 0x3FF,
                        0x2400..=0x27FF => 0x400 + (address & 0x3FF),
                        0x2C00..=0x2FFF => 0x400 + (address & 0x3FF),
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
            let mut addr = address & 0x001F;
            if addr == 0x0010 {addr = 0x0000;}
            if addr == 0x0014 {addr = 0x0004;}
            if addr == 0x0018 {addr = 0x0008;}
            if addr == 0x001C {addr = 0x000C;}
        
            byte = self.palette_memory[addr as usize];
        } else {
            // Open bus or undefined memory area
            byte = 0;
        }

        byte
    }

    fn ppu_write(&mut self, address: u16, data: u8) {
        if address <= 0x1FFF {
            // unsafe { (*self.cart).ppu_write(address, data) }; // writes to cartridge space
            self.cart.borrow_mut().ppu_write(address, data);
        } else if address >= 0x2000 && address <= 0x2FFF {
            /* nametable writes */
            // let nametable: Nametable = unsafe { (*self.cart).get_nametable() };
            let nametable = self.cart.borrow_mut().get_nametable();

            match nametable {
                Nametable::Vertical => {
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
            let mut addr = address & 0x001F;
            if addr == 0x0010 {addr = 0x0000;}
            if addr == 0x0014 {addr = 0x0004;}
            if addr == 0x0018 {addr = 0x0008;}
            if addr == 0x001C {addr = 0x000C;}
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
                self.t.set_nametable(data);
            }
            1 => {
                self.ppumask = PPUMASK::from_bits_truncate(data);
            },
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
            },
            _ => {
                panic!("cpu_write: Cannot write address");
            }
        }
    }

    pub fn render_816_sprite(&mut self, index: usize) {
        let oam_sprite = self.oam_table[index].clone();
        let x = oam_sprite.get_x_position();
        let y = oam_sprite.get_y_position() + 1;
        let index = oam_sprite.get_index_number() as u16;
        let attribute = oam_sprite.get_attribute();
        let horizontal_factor = attribute & 0x40 > 0;
        let vertical_factor = attribute & 0x80 > 0;
        let attrib_table = attribute & 0x3;
        let table_index = if self.ppuctrl.contains(PPUCTRL::sprite_pattern_table_address) {
            0x1000
        } else {
            0
        };
        for i in 0..16 {
            let pattern_address = (table_index | (index << 4)) + i;
            let mut pattern_lo = self.ppu_read(pattern_address);
            let mut pattern_hi = self.ppu_read(pattern_address + 8);
            for j in 0..8 {
                let pattern_bit_lo = if pattern_lo & 0x80 != 0 { 1 } else { 0 };
                let pattern_bit_hi = if pattern_hi & 0x80 != 0 { 1 } else { 0 };
                let pixel_num = (pattern_bit_hi << 1) | pattern_bit_lo;
                pattern_lo <<= 1;
                pattern_hi <<= 1;
                let color = self.get_fgpalette(attrib_table & 3, pixel_num);
                if x < 241 && y < 231 {
                    let j = if horizontal_factor { 7 - j } else { j };
                    let i = if vertical_factor { 15 - i } else { i };
                    let x = x + j;
                    let y = y + i;
                    if index == 0 && self.frame_array[x as usize][y as usize] != 0 {
                        self.ppustatus.set(PPUSTATUS::sprite_0_hit_flag, true);
                        
                    }
                    let priority = attribute & 0x20 > 0; //if it's one, we render the sprite behind the background
                    if priority {
                        if self.frame_array[x as usize][y as usize] == 0 {
                            if pixel_num != 0 {
                                unsafe {
                                    (*self.nametable_frame.unwrap())
                                        .drawpixel(x as u16, y as u16, color)
                                };
                            }
                        }
                    } else {
                        if pixel_num != 0 {
                            unsafe {
                                (*self.nametable_frame.unwrap())
                                    .drawpixel(x as u16, y as u16, color)
                            };
                        }
                    }
                }
            }
        }
    }
    pub fn render_88_sprite(&mut self, index: usize) {
        let oam_sprite = self.oam_table[index].clone();
        let x = oam_sprite.get_x_position();
        let y = oam_sprite.get_y_position() + 1;
        let index = oam_sprite.get_index_number() as u16;
        let attribute = oam_sprite.get_attribute();
        let horizontal_factor = attribute & 0x40 > 0;
        let vertical_factor = attribute & 0x80 > 0;
        let attrib_table = attribute & 0x3;
        let table_index = if self.ppuctrl.contains(PPUCTRL::sprite_pattern_table_address) {
            0x1000
        } else {
            0
        };
        for i in 0..8 {
            let pattern_address = (table_index | (index << 4)) + i;
            let mut pattern_lo = self.ppu_read(pattern_address);
            let mut pattern_hi = self.ppu_read(pattern_address + 8);
            for j in 0..8 {
                let pattern_bit_lo = if pattern_lo & 0x80 != 0 { 1 } else { 0 };
                let pattern_bit_hi = if pattern_hi & 0x80 != 0 { 1 } else { 0 };
                let pixel_num = (pattern_bit_hi << 1) | pattern_bit_lo;
                pattern_lo <<= 1;
                pattern_hi <<= 1;
                let color = self.get_fgpalette(attrib_table & 3, pixel_num);
                if x < 241 && y < 231 {
                    let j = if horizontal_factor { 7 - j } else { j };
                    let i = if vertical_factor { 7 - i } else { i };
                    let x = x + j;
                    let y = y + i;
                    if index == 0 && self.frame_array[x as usize][y as usize] != 0 {
                        self.ppustatus.set(PPUSTATUS::sprite_0_hit_flag, true);
                        
                    }
                    let priority = attribute & 0x20 > 0; //if it's one, we render the sprite behind the background
                    if priority {
                        if self.frame_array[x as usize][y as usize] == 0 {
                            if pixel_num != 0 {
                                unsafe {
                                    (*self.nametable_frame.unwrap())
                                        .drawpixel(x as u16, y as u16, color)
                                };
                            }
                        }
                    } else {
                        if pixel_num != 0 {
                            unsafe {
                                (*self.nametable_frame.unwrap())
                                    .drawpixel(x as u16, y as u16, color)
                            };
                        }
                    }
                }
            }
        }
    }
    pub fn set_oam_table(&mut self) {
        if self.ppuctrl.contains(PPUCTRL::sprite_size) {
            for i in 0..64 {
                self.render_816_sprite(i);
            }
        } else {
            for i in 0..64 {
                self.render_88_sprite(i);
            }
        }
    }
    pub fn set_name_table(&mut self) {
        self.set_oam_table();
        for i in 0..255{
            self.oam_table[i / 4].set_byte(i as u8, 0xFF);
        }
    }
    fn get_pattern_address(&self) -> u16{
        let toreturn = if self.ppuctrl.contains(PPUCTRL::background_pattern_table_address){0x1000} else {0x0000};
        let finey = self.v.get_fine_y() as u16;
        let toreturn = toreturn | (self.next_nametable_tile << 4) | finey;
        toreturn
    }
    pub fn clock(&mut self) {

        /* Background Rendering */
        if self.scanline_counter >= -1 && self.scanline_counter < 240{ /* Pre Render and Visible Scanlines */
            
            if self.cycle_counter == 0{
                self.cycle_counter = 1; /* We skip this cycle */
            }
            if (self.cycle_counter >= 1 && self.cycle_counter < 256) || (self.cycle_counter >= 321 && self.cycle_counter <= 338) {
                match (self.cycle_counter - 1) % 8 {
                    0 => {
                        self.loadregisters();
                        self.next_nametable_tile = self.ppu_read(self.v.get_nametable_address()) as u16;
                    },
                    1 => {},
                    2 => {
                        // Fetch attribute byte
                        let attr_byte = self.ppu_read(self.v.get_attribute_address());
                        self.next_attribute_tile = attr_byte as u16;
                        
                        // Select correct 2-bit palette group
    
                        // Calculate quadrant selection (0-3)
                        if self.v.get_coarse_yscroll() & 2 != 0{
                            self.next_attribute_tile >>= 4;
                        }
                        if self.v.get_coarse_xscroll() & 2 != 0{
                            self.next_attribute_tile >>= 2;
                        }
                        self.next_attribute_tile &= 3;
                    },
                    3 => {},
                    4 => {
                        // Fetch pattern LSB
                        let lo_address = self.get_pattern_address();
                        self.next_pattern_lo = self.ppu_read(lo_address) as u16;
                        self.next_attribute_lo = if self.next_attribute_tile & 1 > 0 {0xFF} else {0x00};
                    },
                    5 => {},
                    6 => {
                        // Fetch pattern MSB
                        let hi_address = self.get_pattern_address() + 8;
                        self.next_pattern_hi = self.ppu_read(hi_address) as u16;
                        self.next_attribute_hi = if self.next_attribute_tile & 2 > 0 {0xFF} else {0x00};
                        // println!("{:4x} hi: {:08b} lo: {:08b}",self.v.get_nametable_address(),self.next_pattern_hi,self.next_attribute_lo);
                    },
                    7 => {
                        self.increment_x();
                    },
                    _ => unreachable!(),
                }
            }
            if self.cycle_counter == 256{
                self.increment_y();
                self.transfer_x();
            }
        }
        if self.scanline_counter == -1 && self.cycle_counter >= 280 && self.cycle_counter <= 304{
            self.transfer_y(); /* We transfer the vertical factor from the T register here. */
        }

        
        if self.ppumask.contains(PPUMASK::enable_background_rendering) || self.ppumask.contains(PPUMASK::enable_sprite_rendering){
            let mux = 0x8000 >> (self.x as u16); /* Our multiplexer to select the bit */
            let pixel_lo = if self.pattern_lo_shift_register & mux != 0 {1} else {0};
            let pixel_hi = if self.pattern_hi_shift_register & mux != 0 {1} else {0};
            let attrib_lo = if self.attribute_lo_shift_register & mux != 0 {1} else {0};
            let attrib_hi = if self.attribute_hi_shift_register & mux != 0 {1} else {0};
            let pixel = (pixel_hi << 1) | (pixel_lo);
            let pattern = (attrib_hi << 1) | (attrib_lo);
            self.shift();
            if self.cycle_counter < 256 && self.scanline_counter > 0 && self.scanline_counter < 240{
                let color = self.get_bgpalette(pattern & 3, pixel);
                self.frame_array[self.cycle_counter as usize][self.scanline_counter as usize] = pixel;
                unsafe{
                    (*self.nametable_frame.unwrap()).drawpixel(self.cycle_counter, self.scanline_counter as u16, color);
                }
            }
        }

        /* Incrementing Logic */
        self.cycle_counter += 1;
        if self.cycle_counter > 340 {

            self.cycle_counter = 0;
            self.scanline_counter += 1;
        }
        if self.scanline_counter == self.oam_table[0].get_y_position().wrapping_add(8) as i16 && self.cycle_counter == self.oam_table[0].get_x_position() as u16{
            self.ppustatus.set(PPUSTATUS::sprite_0_hit_flag,true);
        }
        if self.scanline_counter <= 239 {
        } else if self.scanline_counter == 241 && self.cycle_counter == 1 {
            self.ppustatus.set(PPUSTATUS::vblank_flag, true);
            if self.ppuctrl.contains(PPUCTRL::vblank_enable) {
                self.nmi = true;
            }
        }
        if self.scanline_counter == -1 && self.cycle_counter == 1 {
            self.scanline_counter = 0;
        }
        if self.scanline_counter > 260{
            self.scanline_counter = -1;
            self.ppustatus.set(PPUSTATUS::vblank_flag, false);
            self.ppustatus.set(PPUSTATUS::sprite_0_hit_flag, false);
            self.ppustatus.set(PPUSTATUS::sprite_overflow_flag, false);
        }
        self.total_cycles = self.total_cycles.wrapping_add(1);
    }
}
