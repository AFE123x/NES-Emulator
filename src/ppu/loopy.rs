use super::{registers::PPUMASK, Ppu};

impl Ppu {
    /// Increments the horizontal scroll position (`v`) by one tile.
    ///
    /// This method only performs the increment if background or sprite rendering
    /// is enabled in the `PPUMASK` register. It corresponds to horizontal scrolling logic
    /// during background rendering in the NES PPU pipeline.
    pub fn increment_x(&mut self) {
        if self.ppumask.contains(PPUMASK::enable_background_rendering)
            | self.ppumask.contains(PPUMASK::enable_sprite_rendering)
        {
            self.v.increment_x();
        }
    }

    /// Increments the vertical scroll position (`v`) by one scanline.
    ///
    /// Like `increment_x`, this method checks if rendering is enabled. It's typically called
    /// at the end of a scanline during the rendering phase to advance the vertical scroll.
    pub fn increment_y(&mut self) {
        if self.ppumask.contains(PPUMASK::enable_background_rendering)
            | self.ppumask.contains(PPUMASK::enable_sprite_rendering)
        {
            self.v.increment_y();
        }
    }

    /// Transfers horizontal scrolling information from temporary (`t`) to current (`v`) VRAM address.
    ///
    /// This is used during the horizontal scroll reload phase in rendering, typically at cycle 257
    /// of each visible scanline. It sets the coarse X scroll and horizontal nametable selection.
    pub fn transfer_x(&mut self) {
        if self.ppumask.contains(PPUMASK::enable_background_rendering)
            | self.ppumask.contains(PPUMASK::enable_sprite_rendering)
        {
            self.v.set_coarse_xscroll(self.t.get_coarse_xscroll());
            self.v.set_nametablex(self.t.get_nametablex());
        }
    }

    /// Transfers vertical scrolling information from temporary (`t`) to current (`v`) VRAM address.
    ///
    /// This is typically done during the pre-render scanline (scanline -1) at cycle 304,
    /// and copies the coarse Y, fine Y, and vertical nametable bits from the temporary address.
    pub fn transfer_y(&mut self) {
        if self.ppumask.contains(PPUMASK::enable_background_rendering)
            | self.ppumask.contains(PPUMASK::enable_sprite_rendering)
        {
            self.v.set_coarse_yscroll(self.t.get_coarse_yscroll());
            self.v.set_nametabley(self.t.get_nametabley());
            self.v.set_fine_y(self.t.get_fine_y());
        }
    }

    /// Shifts all background-related shift registers left by one bit.
    ///
    /// This is done every cycle during rendering to prepare the next bit of pattern/attribute data
    /// for pixel output. All four shift registers (2 pattern, 2 attribute) participate in this.
    pub fn shift(&mut self) {
        self.attribute_hi_shift_register <<= 1;
        self.attribute_lo_shift_register <<= 1;
        self.pattern_hi_shift_register <<= 1;
        self.pattern_lo_shift_register <<= 1;
    }

    /// Loads fetched tile and attribute data into the corresponding shift registers.
    ///
    /// This is done every 8 cycles during rendering to reload the low byte of each
    /// shift register with new data fetched from memory.
    pub fn loadregisters(&mut self) {
        self.attribute_hi_shift_register =
            (self.attribute_hi_shift_register & 0xFF00) | self.next_attribute_hi;
        self.attribute_lo_shift_register =
            (self.attribute_lo_shift_register & 0xFF00) | self.next_attribute_lo;
        self.pattern_hi_shift_register =
            (self.pattern_hi_shift_register & 0xFF00) | self.next_pattern_hi;
        self.pattern_lo_shift_register =
            (self.pattern_lo_shift_register & 0xFF00) | self.next_pattern_lo;
    }
}
