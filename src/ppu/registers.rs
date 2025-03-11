use bitflags::{bitflags, Flags};

bitflags! {
    /// # PPU Control Register
    /// This register defines the behavior of the PPU, determined by the CPU
    /// ## Bits
    /// - bit 0 defines which name table to use in the x-axis.
    /// - bit 1 defines which name table to use in the y-axis.
    /// - bit 2 tells us by what factor to increment the CPU read/write of PPU data
    ///     - If the bit is 1, we increment the address by 32
    ///     - If the bit is 0, we increment the address by 1.
    /// - bit 3 defines which pattern table to use for sprites
    ///     - If the bit is 0, we use the table at ppu address $0000
    ///     - If the bit is 1, we use the tabel at ppu address $1000
    /// - bit 4 defines which pattern table to use for the background
    /// - bit 5 decides the sprite size, (lowkey don't know what that means)
    /// - bit 6 doesn't really matter, but it's the master slave select
    /// - bit 7 will enable or disable the VBlank NMI Interrupt.
    pub struct PPUCTRL: u8{
        const name_table_x = 0b0000_0001;
        const name_table_y = 0b0000_0010;
        const vram_increment = 0b0000_0100;
        const sprite_pattern_table_address = 0b0000_1000;
        const background_pattern_table_address = 0b0001_0000;
        const sprite_size = 0b0010_0000;
        const master_slave_select = 0b0100_0000;
        const vblank_enable = 0b1000_0000;
    }
}

bitflags! {
    ///# PPU Mask Register
    /// - This register controls the rendering of sprites and backround, plus some color effects
    /// ## Bits
    /// - bit 0 will enable grepscale, so the graphics will render as a shade of gray.
    /// - bit 1 will determine whether to render the leftmost 8 pixels in the background (column 0 of nametable).
    /// - bit 2 will determe whether to render the leftmost 8 pixels in the sprite level.
    /// - bit 3 will toggle whether to render the background.
    /// - bit 4 will toggle whether to render the sprites.
    /// - bit 5 will emphasize the color red.
    /// - bit 6 will emphasize the color green.
    /// - bit 7 will emphasize the color blue.
    pub struct PPUMASK: u8{
        const greyscale = 0b0000_0001;
        const show_background = 0b0000_0010;
        const sprites_leftmost = 0b0000_0100;
        const enable_background_rendering = 0b0000_1000;
        const enable_sprite_rendering = 0b0001_0000;
        const emphasize_red = 0b0010_0000;
        const emphasize_green = 0b0100_0000;
        const emphasize_blue = 0b1000_0000;
    }
}

bitflags! {
    ///# PPU Status Register
    /// This register reflects the state of rendering events. on cycle one of the scanline, this register is cleared. When the CPU reads from this,
    /// ## Bits
    /// - bit 0-4 plays no role, typically contains the PPU Version
    /// - bit 5 is only used if thre are more than 8 sprites being rendered in a scanline. Realistically only used as a second timing source.
    /// - bit 6 is set when a non-background color of sprite 0 overlaps with a non-background color of the background (mario collides with the castle would trigger this).
    /// - bit 7 will get enabled when at the start of the vblank period (scanline 241, cycle 1) and disabled at bit one of 261 (the pre render scanline)
    pub struct PPUSTATUS: u8{
        const vblank_flag = 0b1000_0000;
        const sprite_0_hit_flag = 0b0100_0000;
        const sprite_overflow_flag = 0b0010_0000;
        const misc = 0b0001_1111;
    }
}

/// # vt_reg
/// This emulates the 15 bit internal registers on the picture processing unit. Each set of bits have different functions.
/// ## bits
/// - bits 0-4: this represents the coarse x scroll
/// - bits 5-9: this represents the coarse y scroll
/// - bits 10-11: this represents which nametable to use
/// - bits 12-14: This represents the fine y scroll
pub struct vt_reg {
    data: u16,
}

impl vt_reg {
    /// This initializes the vt_reg structure
    pub fn new() -> Self {
        Self { data: 0 }
    }
    /// This sets the fine_y bits to whatever specified
    pub fn set_fine_y(&mut self, input: u8) {
        let input = input & 0x7;
        self.data &= !0b1111000000000000;
        let mut temp = input as u16;
        temp <<= 12;
        self.data |= temp;
    }
    ///This retrieves the bits stored in the fine_y section
    pub fn get_fine_y(&mut self) -> u8 {
        let temp = (self.data >> 12) & 0x7;
        let temp = temp as u8;
        temp
    }
    ///this will set the bits in the nametable section
    pub fn set_nametable(&mut self, input: u8) {
        let input = input & 0x3;
        self.data &= !0b1000110000000000;
        let mut temp = input as u16;
        temp <<= 10;
        self.data |= temp;
    }
    ///this will retrieve the bits in the nametable
    pub fn get_nametable(&mut self) -> u8 {
        let temp = (self.data >> 10) & 0x3;
        let temp = temp as u8;
        temp
    }
    ///This will set the bits for the coarse y scroll.
    pub fn set_coarse_yscroll(&mut self, input: u8){
        let input = input & 0x1F;
        self.data &= !(0b1_000_00_11111_00000);
        let input = input as u16;
        let input = input << 5;
        self.data |= input;
    }
    ///this will set the bits in the coarse y scroll
    pub fn get_coarse_yscroll(&mut self) -> u8{
        let input = self.data >> 5;
        let input = input & 0x1F;
        let input = input as u8;
        input
    }
    ///This will set the bits in the x scroll
    pub fn set_coarse_xscroll(&mut self, input: u8){
        let input = input & 0x1F;
        self.data &= !(0b1_000_00_00000_11111);
        let input = input as u16;
        self.data |= input;
    }
    ///This will retrieve the bits in the x scroll
    pub fn get_coarse_xscroll(&mut self) -> u8{
        let input = self.data & 0x1F;
        let input = input as u8;
        input
    }
    ///this will set the raw bits of the vt_register (we ignore the msb of the u16)
    pub fn set_data(&mut self, val: u16){
        let val = val & 0b0_111_11_11111_11111;
        self.data = val;
    }
    ///this will set the raw bits of the vt_register
    pub fn get_data(&self) -> u16{
        self.data
    }

}

#[cfg(test)]
///# Unit tests module
/// - These are comprehensive tests to illustrate and prove it's functionatily and correctness
mod vt_tests {
    use super::vt_reg;

    #[test]
    ///
    pub fn test1() {
        let mut vt_reg = vt_reg::new();
        vt_reg.set_fine_y(4);
        assert_eq!(
            vt_reg.data, 0b0100000000000000,
            "set_fine_y test 1 - FAILED!"
        );
    }

    #[test]
    pub fn test2() {
        let mut vt_reg = vt_reg::new();
        vt_reg.set_fine_y(7);
        assert_eq!(
            vt_reg.data, 0b0111000000000000,
            "set_fine_y test 2 - FAILED!"
        );
    }
    #[test]
    pub fn test3() {
        let mut vt_reg = vt_reg::new();
        vt_reg.data = 0b0101000000000000;
        assert_eq!(vt_reg.get_fine_y(), 5, "get_fine_y test 3 - FAILED!")
    }

    #[test]
    pub fn test4() {
        let mut vt_reg = vt_reg::new();
        vt_reg.data = 0b0110000000000000;
        assert_eq!(vt_reg.get_fine_y(), 6, "get_fine_y test 4 - FAILED!")
    }

    #[test]
    pub fn test5() {
        let mut vt_reg = vt_reg::new();
        vt_reg.set_nametable(3);
        assert_eq!(vt_reg.data,0b0000110000000000, "set_nametable test 5 - FAILED!")
    }

    #[test]
    pub fn test6() {
        let mut vt_reg = vt_reg::new();
        vt_reg.data = 0b0000110000000000;
        
        assert_eq!(vt_reg.get_nametable(),3, "get_nametable test 6 - FAILED!")
    }

    #[test]
    pub fn test7(){
        let mut vt_reg = vt_reg::new();
        vt_reg.set_coarse_yscroll(0b11111);
        assert_eq!(vt_reg.data,0b0_000_00_11111_00000,"set_coarse_yscroll test 7 - FAILED!")
    }
    #[test]
    pub fn test8(){
        let mut vt_reg = vt_reg::new();
        vt_reg.data = 0b0_000_00_11111_00000;
        assert_eq!(vt_reg.get_coarse_yscroll(),0x1F,"get_coarse_yscroll test 8 - FAILED!")
    }

    #[test]
    pub fn test9(){
        let mut vt_reg = vt_reg::new();
        vt_reg.set_coarse_xscroll(0b11111);
        assert_eq!(vt_reg.data,0b0_000_00_00000_11111,"set_coarse_xscroll test 9 - FAILED!")
    }
    #[test]
    pub fn test10(){
        let mut vt_reg = vt_reg::new();
        vt_reg.data = 0b0_000_00_00000_11111;
        assert_eq!(vt_reg.get_coarse_xscroll(),0x1F,"get_coarse_kscroll test 10 - FAILED!")
    }
}
