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

        /* Update shift registers */
        self.pattern_lo_shift_register |= pattern_lo;
        self.pattern_hi_shift_register |= pattern_hi;

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
        self.attribute_hi_shift_register |= attribute_hi;
        self.attribute_lo_shift_register |= attribute_lo;

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
        for x in 0..32 {
            if x > 0 {
                self.prime_steptile();
            }
            let finex = self.x as u16;
            let mux = 0x8000 >> finex;

            /* get the attribute bytes */
            for i in 0..8 {
                let attribute_lobit = if self.attribute_lo_shift_register & mux > 0 {
                    1
                } else {
                    0
                } as u16;
                let attribute_hibit = if self.attribute_hi_shift_register & mux > 0 {
                    1
                } else {
                    0
                };

                let attribute_index = (attribute_hibit << 1) | attribute_lobit;
                let attribute_index = attribute_index & 3;

                /* Get our pixel number */
                let pattern_lobit = if self.pattern_lo_shift_register & mux > 0 {
                    1
                } else {
                    0
                } as u16;
                let pattern_hibit = if self.pattern_hi_shift_register & mux > 0 {
                    1
                } else {
                    0
                };
                let pattern_index = (pattern_hibit << 1) | pattern_lobit;
                self.shift_registers();
                unsafe {
                    (*self.nametable_buffer.unwrap()).drawpixel(
                        (x * 8) + i,
                        scanline as u16,
                        self.get_fgpalette(attribute_index as u8, pattern_index as u8),
                    );
                }
            }
        }
    }
}
