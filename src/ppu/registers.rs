use bitflags::{bitflags, Flags};
/*
7  bit  0
---- ----
VPHB SINN
|||| ||||
|||| ||++- Base nametable address
|||| ||    (0 = $2000; 1 = $2400; 2 = $2800; 3 = $2C00)
|||| |+--- VRAM address increment per CPU read/write of PPUDATA
|||| |     (0: add 1, going across; 1: add 32, going down)
|||| +---- Sprite pattern table address for 8x8 sprites
||||       (0: $0000; 1: $1000; ignored in 8x16 mode)
|||+------ Background pattern table address (0: $0000; 1: $1000)
||+------- Sprite size (0: 8x8 pixels; 1: 8x16 pixels – see PPU OAM#Byte 1)
|+-------- PPU master/slave select
|          (0: read backdrop from EXT pins; 1: output color on EXT pins)
+--------- Vblank NMI enable (0: off, 1: on)
*/


bitflags! {
    pub struct PPUCTRL: u8{
        const name_table_x = 0b0000_0001;
        const name_table_y = 0b0000_0010;
        const vram_increment = 0b0000_0100;
        const sprite_pattern_table_address = 0b0000_1000;
        const background_pattern_table_address = 0b0001_0000;
        const sprite_size = 0b0010_0000;
        const vblank_enable = 0b1000_0000;
        const master_slave_select = 0b0100_0000;
    }
}




/*
7  bit  0
---- ----
BGRs bMmG
|||| ||||
|||| |||+- Greyscale (0: normal color, 1: greyscale)
|||| ||+-- 1: Show background in leftmost 8 pixels of screen, 0: Hide
|||| |+--- 1: Show sprites in leftmost 8 pixels of screen, 0: Hide
|||| +---- 1: Enable background rendering
|||+------ 1: Enable sprite rendering
||+------- Emphasize red (green on PAL/Dendy)
|+-------- Emphasize green (red on PAL/Dendy)
+--------- Emphasize blue

*/

bitflags! {
    pub struct PPUMASK: u8{
        const greyscale = 0b0000_0001;
        const background_leftmost = 0b0000_0010;
        const sprites_leftmost = 0b0000_0100;
        const enable_background_rendering = 0b0000_1000;
        const enable_sprite_rendering = 0b0001_0000;
        const emphasize_red = 0b0010_0000;
        const emphasize_green = 0b0100_0000;
        const emphasize_blue = 0b1000_0000;
    }
}

/*
7  bit  0
---- ----
VSOx xxxx
|||| ||||
|||+-++++- (PPU open bus or 2C05 PPU identifier)
||+------- Sprite overflow flag
|+-------- Sprite 0 hit flag
+--------- Vblank flag, cleared on read. Unreliable; see below.
*/

bitflags! {
    pub struct PPUSTATUS: u8{
        const vblank_flag = 0b1000_0000;
        const sprite_0_hit_flag = 0b0100_0000;
        const sprite_overflow_flag = 0b0010_0000;
        const misc = 0b0001_1111;
    }
}

/*
1st write
7  bit  0
---- ----
XXXX XXXX
|||| ||||
++++-++++- X scroll bits 7-0 (bit 8 in PPUCTRL bit 0)

2nd write
7  bit  0
---- ----
YYYY YYYY
|||| ||||
++++-++++- Y scroll bits 7-0 (bit 8 in PPUCTRL bit 1)
*/