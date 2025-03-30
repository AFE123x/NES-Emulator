use crate::cartridge::Nametable;

use super::{registers::{PPUCTRL, PPUMASK}, Ppu};

impl Ppu {
    // fn prime_shift_register(&mut self) {
    //     /* first tile */
    //     let nametable_address = self.v.get_nametable_address();
    //     let attribute_address = self.v.get_attribute_address();
    //     /* palette bytes */
    //     let pattern_address = if self
    //         .ppuctrl
    //         .contains(PPUCTRL::background_pattern_table_address)
    //     {
    //         0x1000
    //     } else {
    //         0
    //     };
    //     let finey = self.v.get_fine_y() as u16;
    //     let nametable_address = self.ppu_read(nametable_address) as u16;
    //     let pattern_address = pattern_address | (nametable_address << 4) | finey;
    //     let pattern_lo = self.ppu_read(pattern_address) as u16;
    //     let pattern_hi = self.ppu_read(pattern_address + 8) as u16;
    //     self.pattern_hi_shift_register = pattern_hi << 8;
    //     self.pattern_lo_shift_register = pattern_lo << 8;

    //     /* Get attribute table */
    //     let mut attribute_tile = self.ppu_read(attribute_address);
    //     if self.v.get_coarse_yscroll() & 2 != 0 {
    //         //we're on bottom row
    //         attribute_tile >>= 4;
    //     }
    //     if self.v.get_coarse_xscroll() & 2 != 0 {
    //         //we're on the right column
    //         attribute_tile >>= 2;
    //     }
    //     attribute_tile &= 0x3;

    //     let attribute_lobyte = if attribute_tile & 1 != 0 { 0xFF } else { 0 };
    //     let attribute_hibyte = if attribute_tile & 2 != 0 { 0xFF } else { 0 };
    //     self.attribute_hi_shift_register = attribute_hibyte << 8;
    //     self.attribute_lo_shift_register = attribute_lobyte << 8;

    //     self.v.increment_x();

    //     /* second tile */
    //     let nametable_address = self.v.get_nametable_address();
    //     let attribute_address = self.v.get_attribute_address();
    //     /* palette bytes */
    //     let pattern_address = if self
    //         .ppuctrl
    //         .contains(PPUCTRL::background_pattern_table_address)
    //     {
    //         0x1000
    //     } else {
    //         0
    //     };
    //     let finey = self.v.get_fine_y() as u16;
    //     let nametable_address = self.ppu_read(nametable_address) as u16;
    //     let pattern_address = pattern_address | (nametable_address << 4) | finey;
    //     let pattern_lo = self.ppu_read(pattern_address) as u16;
    //     let pattern_hi = self.ppu_read(pattern_address + 8) as u16;
    //     self.pattern_hi_shift_register |= pattern_hi;
    //     self.pattern_lo_shift_register |= pattern_lo;

    //     /* Get attribute table */
    //     let mut attribute_tile = self.ppu_read(attribute_address);
    //     if self.v.get_coarse_yscroll() & 2 != 0 {
    //         //we're on bottom row
    //         attribute_tile >>= 4;
    //     }
    //     if self.v.get_coarse_xscroll() & 2 != 0 {
    //         //we're on the right column
    //         attribute_tile >>= 2;
    //     }
    //     attribute_tile &= 0x3;

    //     let attribute_lobyte = if attribute_tile & 1 != 0 { 0xFF } else { 0 };
    //     let attribute_hibyte = if attribute_tile & 2 != 0 { 0xFF } else { 0 };
    //     self.attribute_hi_shift_register |= attribute_hibyte;
    //     self.attribute_lo_shift_register |= attribute_lobyte;

    //     self.v.increment_x();
    // }

    // fn render_scanline(&mut self, scanline: u16) {
    //     /* prime shift registers */
    //     self.prime_shift_register();
    //     for x in 0..256 {
    //         if x > 0 && x % 8 == 0 {
    //             /* Fetch the nametable and attribute tiles */
    //             let nametable_address = self.v.get_nametable_address();
    //             let attribute_address = self.v.get_attribute_address();
    //             /* Fetch the nametable */
    //             let nametable_tile = self.ppu_read(nametable_address) as u16;

    //             /* Fetch the pattern hi and lo tiles */
    //             let pattern_address = if self
    //                 .ppuctrl
    //                 .contains(PPUCTRL::background_pattern_table_address)
    //             {
    //                 0x1000
    //             } else {
    //                 0
    //             };
    //             let finey = self.v.get_fine_y() as u16;
    //             let pattern_address = pattern_address | (nametable_tile << 4) | finey;
    //             let patternlo = self.ppu_read(pattern_address) as u16;
    //             let patternhi = self.ppu_read(pattern_address + 8) as u16;

    //             self.pattern_hi_shift_register |= patternhi;
    //             self.pattern_lo_shift_register |= patternlo;

    //             /* Load attribute table */
    //             let mut attribute_tile = self.ppu_read(attribute_address) as u16;
    //             if self.v.get_coarse_yscroll() & 2 != 0 {
    //                 //we're on bottom row
    //                 attribute_tile >>= 4;
    //             }
    //             if self.v.get_coarse_xscroll() & 2 != 0 {
    //                 //we're on the right column
    //                 attribute_tile >>= 2;
    //             }
    //             attribute_tile &= 0x3;
    //             let attribute_lo = if attribute_tile & 1 != 0 {0xFF} else {0};
    //             let attribute_hi = if attribute_tile & 2 != 0 {0xFF} else {0x0};
    //             self.attribute_hi_shift_register |= attribute_hi;
    //             self.attribute_lo_shift_register |= attribute_lo;
    //             self.v.increment_x();
    //         }

    //         let finex = self.x as u16;
    //         let mux = 0x8000 >> finex;
    //         /* Retrieve the bits */
    //         let pattern_hibit = if self.pattern_hi_shift_register & mux > 0 {1} else {0};
    //         let pattern_lobit = if self.pattern_lo_shift_register & mux > 0 {1} else {0};

    //         let pattern_index = (pattern_hibit << 1) | pattern_lobit;
    //         let attribute_hibit = if self.attribute_hi_shift_register & mux > 0 {1} else {0};
    //         let attribute_lobit = if self.attribute_lo_shift_register & mux > 0 {1} else {0};
    //         let attribute_index = (attribute_hibit << 1) | attribute_lobit;

    //         /* Shift the registers */
    //         self.pattern_hi_shift_register <<= 1;
    //         self.pattern_lo_shift_register <<= 1;
    //         self.attribute_hi_shift_register <<= 1;
    //         self.attribute_lo_shift_register <<= 1;
    //         /* Draw the graphics to pixel */
    //         if self.ppumask.contains(PPUMASK::enable_background_rendering){
    //             unsafe{
    //                 (*self.nametable_frame.unwrap()).drawpixel(x, scanline, self.get_bgpalette(attribute_index, pattern_index));
    //             }
    //             self.frame_array[x as usize][scanline as usize] = pattern_index;
    //         }

    //     }
    //     self.v.increment_y();
    // }
    // pub fn render_nametable(&mut self) {
    //     self.v.set_data(self.t.get_data());
    //     for i in 0..240 {
    //         self.render_scanline(i);
    //         self.v.set_coarse_xscroll(self.t.get_coarse_xscroll());
    //         self.v.set_nametablex(self.t.get_nametablex());
    //     }
    // }


    pub fn increment_x(&mut self){
        if self.ppumask.contains(PPUMASK::enable_background_rendering) | self.ppumask.contains(PPUMASK::enable_sprite_rendering){
            self.v.increment_x();
        }
    }

    pub fn increment_y(&mut self){
        if self.ppumask.contains(PPUMASK::enable_background_rendering) | self.ppumask.contains(PPUMASK::enable_sprite_rendering){
            self.v.increment_y();
        }
    }
    pub fn transfer_x(&mut self){
        if self.ppumask.contains(PPUMASK::enable_background_rendering) | self.ppumask.contains(PPUMASK::enable_sprite_rendering){
            self.v.set_coarse_xscroll(self.t.get_coarse_xscroll());
            self.v.set_nametablex(self.t.get_nametablex());
        }
    }
    pub fn transfer_y(&mut self){
        if self.ppumask.contains(PPUMASK::enable_background_rendering) | self.ppumask.contains(PPUMASK::enable_sprite_rendering){
            self.v.set_coarse_yscroll(self.t.get_coarse_yscroll());
            self.v.set_nametabley(self.t.get_nametabley());
            self.v.set_fine_y(self.t.get_fine_y());
        }
    }

    pub fn shift(&mut self){
        self.attribute_hi_shift_register <<= 1;
        self.attribute_lo_shift_register <<= 1;
        self.pattern_hi_shift_register <<= 1;
        self.pattern_lo_shift_register <<= 1;
    }

    pub fn loadregisters(&mut self){
        self.attribute_hi_shift_register = (self.attribute_hi_shift_register & 0xFF00) | self.next_attribute_hi;
        self.attribute_lo_shift_register = (self.attribute_lo_shift_register & 0xFF00) | self.next_attribute_lo;
        self.pattern_hi_shift_register = (self.pattern_hi_shift_register & 0xFF00) | self.next_pattern_hi;
        self.pattern_lo_shift_register = (self.pattern_lo_shift_register & 0xFF00) | self.next_pattern_lo;
    }
}
