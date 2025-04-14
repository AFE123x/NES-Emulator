
use super::{registers::PPUMASK, Ppu};

impl Ppu {
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
