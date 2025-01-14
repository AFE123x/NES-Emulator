#ifndef PPU_H
#define PPU_H
#include<stdint.h>
/*
7  bit  0
---- ----
VPHB SINN
|||| ||||
|||| ||++- Base nametable address (0 = $2000; 1 = $2400; 2 = $2800; 3 = $2C00)
|||| ||    (name_table_select)
|||| |+--- VRAM address increment per CPU read/write of PPUDATA
|||| |     (0: add 1, horizontal; 1: add 32, vertical)
|||| +---- Sprite pattern table base address for 8x8 sprites
||||       (0: $0000; 1: $1000; ignored in 8x16 mode)
|||+------ Background pattern table base address (0: $0000; 1: $1000)
||+------- Sprite size (0: 8x8 pixels; 1: 8x16 pixels – see PPU OAM#Byte 1)
|+-------- PPU master/slave select
|          (0: read backdrop from EXT pins; 1: output color on EXT pins)
+--------- Vblank NMI enable (0: off, 1: on)
*/

typedef union {
    struct {
        uint8_t name_table_select: 2;
        uint8_t vram_address_increment: 1;
        uint8_t sprite_pattern_table_base: 1;
        uint8_t background_pattern_table_base: 1;
        uint8_t sprite_size: 1;
        uint8_t ppu_master_slave_select: 1;
        uint8_t vblank_nmi_enable: 1;
    };
    uint8_t raw; // Raw 8-bit value for direct access
} PPU_CTRL;



/*
7  bit  0
---- ----
BGRs bMmG
|||| ||||
|||| |||+- Greyscale mode (0: normal color, 1: greyscale)
|||| ||+-- Show background in leftmost 8 pixels of screen (0: Hide, 1: Show)
|||| |+--- Show sprites in leftmost 8 pixels of screen (0: Hide, 1: Show)
|||| +---- Enable background rendering
|||+------ Enable sprite rendering
||+------- Emphasize red (green on PAL/Dendy)
|+-------- Emphasize green (red on PAL/Dendy)
+--------- Emphasize blue
*/

typedef union {
    struct {
        uint8_t greyscale: 1;
        uint8_t show_background_left: 1;
        uint8_t show_sprites_left: 1;
        uint8_t enable_background: 1;
        uint8_t enable_sprites: 1;
        uint8_t emphasize_red: 1;
        uint8_t emphasize_green: 1;
        uint8_t emphasize_blue: 1;
    };
    uint8_t raw;
} PPU_MASK;

/*
7  bit  0
---- ----
VSOx xxxx
|||| ||||
|||+-++++- PPU open bus or 2C05 PPU identifier
||+------- Sprite overflow flag
|+-------- Sprite 0 hit flag
+--------- Vblank flag, cleared on read. Unreliable; see below.
*/

typedef union {
    struct {
        uint8_t open_bus: 5;          // Bits 0-4: PPU open bus or 2C05 PPU identifier
        uint8_t sprite_overflow: 1;  // Bit 5: Sprite overflow flag
        uint8_t sprite_0_hit: 1;     // Bit 6: Sprite 0 hit flag
        uint8_t vblank_flag: 1;      // Bit 7: Vblank flag, cleared on read
    };
    uint8_t raw; // Raw 8-bit value for direct access
} PPU_STATUS;

/*
7  bit  0
---- ----
AAAA AAAA
|||| ||||
++++-++++- OAM address
*/

uint8_t OAMADDR;

/*
7  bit  0
---- ----
DDDD DDDD
|||| ||||
++++-++++- OAM data
*/

uint8_t OAMDATA;

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

uint8_t PPU_SCROLL;

/*
1st write  2nd write
15 bit  8  7  bit  0
---- ----  ---- ----
..AA AAAA  AAAA AAAA
  || ||||  |||| ||||
  ++-++++--++++-++++- VRAM address
*/

uint8_t PPU_ADDR;

/*
7  bit  0
---- ----
DDDD DDDD
|||| ||||
++++-++++- VRAM data
*/

uint8_t PPU_DATA;

/*
7  bit  0
---- ----
AAAA AAAA
|||| ||||
++++-++++- Source page (high byte of source address)
*/

uint8_t OAMDMA;

/*
The PPU also has 4 internal registers, described in detail on PPU scrolling:

    v: During rendering, used for the scroll position. Outside of rendering, used as the current VRAM address.
    t: During rendering, specifies the starting coarse-x scroll for the next scanline and the starting y scroll for the screen. Outside of rendering, holds the scroll or VRAM address before transferring it to v.
    x: The fine-x position of the current scroll, used during rendering alongside v.
    w: Toggles on each write to either PPUSCROLL or PPUADDR, indicating whether this is the first or second write. Clears on reads of PPUSTATUS. Sometimes called the 'write latch' or 'write toggle'.
*/

uint8_t v, t, x, w;

uint8_t patterntable[16384 * 2];
uint32_t colors[64];

void cpu_ppu_Write(uint16_t addr, uint8_t byte);
void cpu_ppu_read(uint16_t addr, uint8_t* byte);

void ppu_ppu_Write(uint16_t addr, uint8_t byte);
void ppu_ppu_read(uint16_t addr, uint8_t* byte);

void ppu_init();

void get_pattern_table(uint8_t index);

#endif