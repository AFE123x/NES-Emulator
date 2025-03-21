use super::{registers::PPUCTRL, Ppu};

impl Ppu {
    fn prime_stepone(&mut self) {
        /* fetch the nametable and attribute addresses */
        let nametable_address: u16 = self.v.get_nametable_address();
        let attribute_address = self.v.get_attribute_address();

        /* calculate palette addresses */
        let pattern_table_address = if self
            .ppuctrl
            .contains(PPUCTRL::background_pattern_table_address)
        {
            0x1000
        } else {
            0x0
        };
        let fine_y = self.v.get_fine_y() as u16;
        let pattern_table_address = pattern_table_address | (nametable_address << 4) | fine_y;

        /* Read the pattern bytes */
        let pattern_lo = self.ppu_read(pattern_table_address) as u16;
        let pattern_hi = self.ppu_read(pattern_table_address + 8) as u16;

        /* Update shift registers */
        self.pattern_lo_shift_register = pattern_lo << 8;
        self.pattern_hi_shift_register = pattern_hi << 8;

        /* Read the attribute byte */
        let mut attribute_tile = self.ppu_read(attribute_address) as u16;

        if self.v.get_coarse_yscroll() & 0x02 != 0 {
            attribute_tile >>= 4;
        }
        if self.v.get_coarse_xscroll() & 0x02 != 0 {
            attribute_tile >>= 2;
        }
        attribute_tile &= 0x03;

        /* Update shift registers */
        let attribute_lo = if attribute_tile & 1 != 0 { 0xFF } else { 0x00 };
        let attribute_hi = if attribute_tile & 2 != 0 { 0xFF } else { 0x00 };
        self.attribute_hi_shift_register = attribute_hi << 8;
        self.attribute_lo_shift_register = attribute_lo << 8;

        self.v.increment_x();
    }

    fn prime_steptile(&mut self) {
        /* fetch the nametable and attribute addresses */
        let nametable_address: u16 = self.v.get_nametable_address();
        let attribute_address = self.v.get_attribute_address();

        /* calculate palette addresses */
        let pattern_table_address = if self
            .ppuctrl
            .contains(PPUCTRL::background_pattern_table_address)
        {
            0x1000
        } else {
            0x0
        };
        let fine_y = self.v.get_fine_y() as u16;
        let pattern_table_address = pattern_table_address | (nametable_address << 4) | fine_y;

        /* Read the pattern bytes */
        let pattern_lo = self.ppu_read(pattern_table_address) as u16;
        let pattern_hi = self.ppu_read(pattern_table_address + 8) as u16;
        println!("pattern_lo: {:4x}, pattern_hi: {:4x}",pattern_lo,pattern_hi);
        /* Updateshift registers */
        self.pattern_lo_shift_register =  (self.pattern_lo_shift_register & 0xFF00) | pattern_lo;
        self.pattern_hi_shift_register = (self.pattern_hi_shift_register & 0xFF00) | pattern_hi;

        /* Read the attribute byte */
        let mut attribute_tile = self.ppu_read(attribute_address) as u16;

        if self.v.get_coarse_yscroll() & 0x02 != 0 {
            attribute_tile >>= 4;
        }
        if self.v.get_coarse_xscroll() & 0x02 != 0 {
            attribute_tile >>= 2;
        }
        attribute_tile &= 0x03;

        /* Update shift registers */
        let attribute_lo = if attribute_tile & 1 != 0 { 0xFF } else { 0x00 };
        let attribute_hi = if attribute_tile & 2 != 0 { 0xFF } else { 0x00 };
        self.attribute_hi_shift_register = (self.attribute_hi_shift_register & 0xFF00) | attribute_hi & 0xFF;
        self.attribute_lo_shift_register = (self.attribute_lo_shift_register & 0xFF00) | attribute_lo & 0xFF;

        self.v.increment_x();
    }

    fn prime_shiftregister(&mut self) {
        /* reset the registers */
        self.attribute_hi_shift_register = 0;
        self.attribute_lo_shift_register = 0;
        self.pattern_hi_shift_register = 0;
        self.pattern_lo_shift_register = 0;
        /* Perform the steps */
        self.prime_stepone();
        self.prime_steptile();
    }

    fn shift_registers(&mut self) {
        self.attribute_hi_shift_register <<= 1;
        self.attribute_lo_shift_register <<= 1;
        self.pattern_hi_shift_register <<= 1;
        self.pattern_lo_shift_register <<= 1;
    }
    pub fn render_scanline(&mut self, scanline: u16) {
        self.prime_shiftregister();
        for x in 0..256{
            if x > 0 && x % 8 == 0{
                self.prime_steptile();
            }
            let finex = 0x8000 >> (self.x as u16);
            let pixel_lo = if self.pattern_lo_shift_register & finex != 0 {1} else {0};
            let pixel_hi = if self.pattern_hi_shift_register & finex != 0 {1} else {0};
            let attrib_lo = if self.attribute_lo_shift_register & finex != 0 {1} else {0};
            let attrib_hi = if self.attribute_hi_shift_register & finex != 0 {1} else {0};
            self.shift_registers();
            let pixel_num = (pixel_hi << 1) | pixel_lo;
            let attrib_num = (attrib_hi << 1) | attrib_lo;
            unsafe{
                (*self.nametable_buffer.unwrap()).drawpixel(x, scanline, self.get_fgpalette(attrib_num, pixel_num));
            }
        }
        self.v.set_coarse_xscroll(self.t.get_coarse_xscroll());
        self.v.set_nametablex(self.t.get_nametablex());
    }
}
