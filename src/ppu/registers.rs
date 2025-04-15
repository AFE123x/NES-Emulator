use bitflags::bitflags;

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
pub struct VtReg {
    data: u16,
}

impl VtReg {
    // pub fn print_register(&mut self) -> String{
    // format!("fine y: {}\tnametable: {}\ty_scroll: {}\tx_scroll: {}\t",self.get_fine_y(),self.get_nametable(),self.get_coarse_yscroll(),self.get_coarse_xscroll())
    // }
    /// This initializes the vt_reg structure
    pub fn new() -> Self {
        Self { data: 0 }
    }

    /// This sets the fine_y bits to whatever specified
    pub fn set_fine_y(&mut self, input: u8) {
        let mask = 7 << 12;
        self.data &= !mask;
        let input = input & 7;
        let input = input as u16;
        self.data |= input << 12;
    }
    ///This retrieves the bits stored in the fine_y section
    pub fn get_fine_y(&self) -> u8 {
        let temp = self.data >> 12;
        (temp & 7) as u8
    }
    ///this will set the bits in the nametable section
    pub fn set_nametable(&mut self, input: u8) {
        let input = (input & 3) as u16;
        /* ...GH.. ........ */
        let input = input << 10;
        self.data &= 0b111_00_11_1111_1111;
        self.data |= input;
    }
    ///this will retrieve the bits in the nametable
    pub fn _get_nametable(&self) -> u8 {
        let nametable = (self.data >> 10) & 3;
        nametable as u8
    }
    ///This will set the bits for the coarse y scroll.
    pub fn set_coarse_yscroll(&mut self, input: u8){
        let input = input & 0x1F;
        let mask = 0x1F << 5;
        self.data &= !mask;
        let input = input as u16;
        let input = input << 5;
        self.data |= input;
    }
    ///this will set the bits in the coarse y scroll
    pub fn get_coarse_yscroll(&self) -> u8{
        ((self.data >> 5) & 0x1F) as u8
    }

    ///This will set the bits in the x scroll
    pub fn set_coarse_xscroll(&mut self, input: u8){
        let input = input & 0x1F;
        let input = input as u16;
        self.data &= !(0x1F);
        self.data |= input;
    }
    ///This will retrieve the bits in the x scroll
    pub fn get_coarse_xscroll(&self) -> u8{
        let toreturn = self.data & 0x1F;
        toreturn as u8
    }
    ///this will set the raw bits of the vt_register (we ignore the msb of the u16)
    pub fn set_data(&mut self, val: u16){
        self.data = val & 0x7FFF;
    }
    ///this will set the raw bits of the vt_register
    pub fn get_data(&self) -> u16{
        self.data
    }

    pub fn set_nametablex(&mut self, table: u8){
        let table = table & 1;
        let table = table as u16;
        let temp = 1 << 10;
        self.data &= !temp;
        self.data |= table << 10;
    }

    pub fn set_nametabley(&mut self, table: u8){
        let table = table & 1;
        let table = table as u16;
        let temp = 1 << 11;
        self.data &= !temp;
        self.data |= table << 11;  
    }

    pub fn get_nametablex(&self) -> u8{
        (((self.data)>> 10) & 1) as u8
    }

    pub fn get_nametabley(&self) -> u8{
        (((self.data)>> 11) & 1) as u8
    }

    pub fn increment_x(&mut self){
        let v = self.get_coarse_xscroll();
        if v == 31{
            self.set_coarse_xscroll(0);
            let nametablex = self.get_nametablex() ^ 1;
            self.set_nametablex(nametablex);
        }
        else{
            self.set_coarse_xscroll(v + 1);
        }
    }
    pub fn increment_y(&mut self){
        let finey = self.get_fine_y();
        if finey < 7{
            self.set_fine_y(finey + 1);
        }
        else{
            self.set_fine_y(0);
            let y = self.get_coarse_yscroll();
            if y == 29{
                self.set_coarse_yscroll(0);
                let nametabley = !self.get_nametabley();
                self.set_nametabley(nametabley);
            }
            else if y == 31{
                self.set_coarse_yscroll(0);
            }
            else{
                self.set_coarse_yscroll(y + 1);
            }
        }
    }

    pub fn get_nametable_address(&self) -> u16{
        let v = self.get_data();
        0x2000 | (v & 0xFFF)
    }
    // attribute address = 0x23C0 | (v & 0x0C00) | ((v >> 4) & 0x38) | ((v >> 2) & 0x07)
    pub fn get_attribute_address(&self) -> u16{
        let v = self.get_data() & 0x3FFF;
        let address = 0x23C0 | (v & 0x0C00) | ((v >> 4) & 0x38) | ((v >> 2) & 0x07);
        address
    }

}

#[cfg(test)]
///# Unit tests module
/// - These are comprehensive tests to illustrate and prove it's functionatily and correctness
mod vt_tests {
    use super::VtReg;
    #[test]
    pub fn nametable_test_1(){
        let mut vt_reg = VtReg::new();
        vt_reg.set_coarse_xscroll(1);
        vt_reg.set_coarse_yscroll(1);
        assert_eq!(vt_reg.get_nametable_address(),0x2021,"nametable test, FAILED!");
    }
    #[test]
    pub fn increment_nametable(){
        let mut vt_reg = VtReg::new();
        vt_reg.increment_x();
        vt_reg.set_nametable(2);
        assert_eq!(vt_reg.get_nametable_address(),0x2801,"nametable test - FAILED!");
    }
    #[test]
    pub fn nametable_test_2(){
        let vt_reg = VtReg::new();
        assert_eq!(vt_reg.get_attribute_address(),0x23C0,"nametable test, FAILED!");
    }
    #[test]
    pub fn increment_x_1(){
        let mut vt_reg = VtReg::new();
        vt_reg.increment_x();
        assert_eq!(vt_reg.get_coarse_xscroll(),1,"Coarse x value, failed!");
    }
    #[test]
    pub fn increment_x_2(){
        let mut vt_reg = VtReg::new();
        vt_reg.set_coarse_xscroll(31);
        vt_reg.increment_x();
        assert_eq!(vt_reg.get_coarse_xscroll(),0,"Coarse x value, failed!");
        assert_eq!(vt_reg.get_nametablex(),1,"nametable value, failed!")
    }

    #[test]
    pub fn increment_x_3(){
        let mut vt_reg = VtReg::new();
        vt_reg.set_coarse_xscroll(31);
        vt_reg.set_nametablex(1);
        vt_reg.increment_x();
        assert_eq!(vt_reg.get_coarse_xscroll(),0,"Coarse x value, failed!");
        assert_eq!(vt_reg.get_nametablex(),0,"nametable value, failed!")
    }
    #[test]
    pub fn increment_y_1(){
        let mut vt_reg = VtReg::new();
        vt_reg.increment_y();
        assert_eq!(vt_reg.get_fine_y(),1,"fine y value, FAILED!");
    }

    #[test]
    pub fn increment_y_2(){
        let mut vt_reg = VtReg::new();
        vt_reg.set_fine_y(7);
        vt_reg.increment_y();

        assert_eq!(vt_reg.get_fine_y(),0,"fine y value, FAILED!");
        assert_eq!(vt_reg.get_coarse_yscroll(),1,"coarse y value, FAILED!");
    }

    #[test]
    pub fn increment_y_3(){
        let mut vt_reg = VtReg::new();
        vt_reg.set_fine_y(7);
        vt_reg.set_coarse_yscroll(29);
        vt_reg.increment_y();
        assert_eq!(vt_reg.get_fine_y(),0,"fine y value, FAILED!");
        assert_eq!(vt_reg.get_coarse_yscroll(),0,"coarse y value, FAILED!");
        assert_eq!(vt_reg.get_nametabley(),1,"nametable y value - FAILED!")
    }

    #[test]
    pub fn increment_y_4(){
        let mut vt_reg = VtReg::new();
        vt_reg.set_fine_y(7);
        vt_reg.set_coarse_yscroll(29);
        vt_reg.set_nametabley(1);
        vt_reg.increment_y();
        assert_eq!(vt_reg.get_fine_y(),0,"fine y value, FAILED!");
        assert_eq!(vt_reg.get_coarse_yscroll(),0,"coarse y value, FAILED!");
        assert_eq!(vt_reg.get_nametabley(),0,"nametable y value - FAILED!")
    }

    #[test]
    pub fn increment_y_5(){
        let mut vt_reg = VtReg::new();
        vt_reg.set_fine_y(7);
        vt_reg.set_coarse_yscroll(31);
        vt_reg.set_nametabley(0);
        vt_reg.increment_y();
        assert_eq!(vt_reg.get_fine_y(),0,"fine y value, FAILED!");
        assert_eq!(vt_reg.get_coarse_yscroll(),0,"coarse y value, FAILED!");
        assert_eq!(vt_reg.get_nametabley(),0,"nametable y value - FAILED!")
    }
    #[test]
    pub fn test11(){
        let mut vt_reg = VtReg::new();
        vt_reg.set_nametablex(1);
        assert_eq!(vt_reg.get_data(),0b0_000_01_00000_00000);
    }

    #[test]
    pub fn test12(){
        let mut vt_reg = VtReg::new();
        vt_reg.set_nametabley(1);
        assert_eq!(vt_reg.get_data(),0b0_000_10_00000_00000);
    }

    pub fn test13(){
        let mut vt_reg = VtReg::new();
        vt_reg.set_nametablex(1);
        let x = vt_reg.get_nametablex();
        assert_eq!(x,1);
    }

    #[test]
    pub fn test14(){
        let mut vt_reg = VtReg::new();
        vt_reg.set_nametabley(1);
        let y = vt_reg.get_nametabley();
        assert_eq!(y,1);
    }

    #[test]
    ///
    pub fn test1() {
        let mut vt_reg = VtReg::new();
        vt_reg.set_fine_y(4);
        assert_eq!(
            vt_reg.data, 0b0100000000000000,
            "set_fine_y test 1 - FAILED!"
        );
    }

    #[test]
    pub fn test2() {
        let mut vt_reg = VtReg::new();
        vt_reg.set_fine_y(7);
        assert_eq!(
            vt_reg.data, 0b0111000000000000,
            "set_fine_y test 2 - FAILED!"
        );
    }
    #[test]
    pub fn test3() {
        let mut vt_reg = VtReg::new();
        vt_reg.data = 0b0101000000000000;
        assert_eq!(vt_reg.get_fine_y(), 5, "get_fine_y test 3 - FAILED!")
    }

    #[test]
    pub fn test4() {
        let mut vt_reg = VtReg::new();
        vt_reg.data = 0b0110000000000000;
        assert_eq!(vt_reg.get_fine_y(), 6, "get_fine_y test 4 - FAILED!")
    }

    #[test]
    pub fn test5() {
        let mut vt_reg = VtReg::new();
        vt_reg.set_nametable(3);
        assert_eq!(vt_reg.data,0b0000110000000000, "set_nametable test 5 - FAILED!")
    }

    #[test]
    pub fn test6() {
        let mut vt_reg = VtReg::new();
        vt_reg.data = 0b0000110000000000;
        
        assert_eq!(vt_reg.get_nametable(),3, "get_nametable test 6 - FAILED!")
    }

    #[test]
    pub fn test7(){
        let mut vt_reg = VtReg::new();
        vt_reg.set_coarse_yscroll(0b11111);
        assert_eq!(vt_reg.data,0b0_000_00_11111_00000,"set_coarse_yscroll test 7 - FAILED!")
    }
    #[test]
    pub fn test8(){
        let mut vt_reg = VtReg::new();
        vt_reg.data = 0b0_000_00_11111_00000;
        assert_eq!(vt_reg.get_coarse_yscroll(),0x1F,"get_coarse_yscroll test 8 - FAILED!")
    }

    #[test]
    pub fn test9(){
        let mut vt_reg = VtReg::new();
        vt_reg.set_coarse_xscroll(0b11111);
        assert_eq!(vt_reg.data,0b0_000_00_00000_11111,"set_coarse_xscroll test 9 - FAILED!")
    }
    #[test]
    pub fn test10(){
        let mut vt_reg = VtReg::new();
        vt_reg.data = 0b0_000_00_00000_11111;
        assert_eq!(vt_reg.get_coarse_xscroll(),0x1F,"get_coarse_kscroll test 10 - FAILED!")
    }
}
