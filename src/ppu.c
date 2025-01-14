#include "../include/ppu.h"
#include "../include/cartridge.h"
#include "../include/sdl_interface.h"
#include<string.h>
static PPU_CTRL ppu_control;
static PPU_MASK ppu_mask;
static PPU_STATUS ppu_status;
static void init_color(uint16_t code,uint32_t red, uint32_t green, uint32_t blue){
    colors[code] = (red << 24) | (red << 16) | (blue << 8) | 0xFF;
}
void ppu_init(){
    memset(&ppu_control,0,sizeof(ppu_control));
    init_color(0,84, 84, 84);
    init_color(1,0, 30, 116);
    init_color(2,8, 16, 144);
    init_color(3,48, 0, 136);
    init_color(4,68, 0, 100);
    init_color(5,92, 0, 48);
    init_color(6,84, 4, 0);
    init_color(7,60, 24, 0);
    init_color(8,32, 42, 0);
    init_color(9,8, 58, 0);
    init_color(10,0, 64, 0);
    init_color(11,0, 60, 0);
    init_color(12,0, 50, 60);
    init_color(13,0, 0, 0);
    init_color(14,0, 0, 0);
    init_color(15,0, 0, 0);
    init_color(16,152, 150, 152);
    init_color(17,8, 76, 196);
    init_color(18,48, 50, 236);
    init_color(19,92, 30, 228);
    init_color(20,136, 20, 176);
    init_color(21,160, 20, 100);
    init_color(22,152, 34, 32);
    nit_color(23,120, 60, 0);
    init_color(24,84, 90, 0);
    init_color(25,40, 114, 0);
    init_color(26,8, 124, 0);
    init_color(27,0, 118, 40);
    init_color(28,0, 102, 120);
    init_color(29,0, 0, 0);
    init_color(30,0, 0, 0);
    init_color(31,0, 0, 0);
    init_color(32,236, 238, 236);
    init_color(33,76, 154, 236);
    init_color(34,120, 124, 236);
    init_color(35,176, 98, 236);
    init_color(36,228, 84, 236);
    init_color(37,236, 88, 180);
    init_color(38,236, 106, 100);
    init_color(39,212, 136, 32);
    init_color(40,160, 170, 0);
    init_color(41,116, 196, 0);
    init_color(42,76, 208, 32);
    init_color(43,56, 204, 108);
    init_color(44,56, 180, 204);
    init_color(45,60, 60, 60);
    init_color(46,0, 0, 0);
    init_color(47,0, 0, 0);
    init_color(48,236, 238, 236);
    init_color(49,168, 204, 236);
    init_color(50,188, 188, 236);
    init_color(51,212, 178, 236);
    init_color(52,236, 174, 236);
    init_color(53,236, 174, 212);
    init_color(54,236, 180, 176);
    init_color(55,228, 196, 144);
    init_color(56,204, 210, 120);
    init_color(57,180, 222, 120);
    init_color(58,168, 226, 144);
    init_color(59,152, 226, 180);
    init_color(60,160, 214, 228);
    init_color(61,160, 162, 160);
    init_color(62,0, 0, 0);
    init_color(63,0, 0, 0);
}
void cpu_ppu_Write(uint16_t addr, uint8_t byte){

}

void cpu_ppu_read(uint16_t addr, uint8_t* byte){
    switch(addr){
        case 0x0000:
            *byte = ppu_control.raw;
            break;
        case 0x0001:
            *byte = ppu_mask.raw;
            break;
        case 0x0002:
            /*
            data = (status.reg & 0xE0) | (ppu_data_buffer & 0x1F);

			// Clear the vertical blanking flag
			status.vertical_blank = 0;

			// Reset Loopy's Address latch flag
			address_latch = 0;
			break;
            */
           *byte = (status.raw & 0xE0) | ppu
    }
}

void ppu_ppu_Write(uint16_t addr, uint8_t byte){

}

void ppu_ppu_read(uint16_t addr, uint8_t* byte){
    rom_ppu_read(addr,byte);
}

// static void make_table(){
//     for (int r = 0; r < 256; r++) {
// 	for (int col = 0; col < 128; col++) {
// 		uint16_t adr = (r / 8 * 0x100) + (r % 8) + (col / 8) * 0x10;
// 		// uint8_t pixel = ((VRAM[adr] >> (7-(col % 8))) & 1) + ((VRAM[adr + 8] >> (7-(col % 8))) & 1) * 2;
// 		// framebuffer_chr[(r * 128 * 3) + (col * 3)] = COLORS[pixel];
// 		framebuffer_chr[(r * 128 * 3) + (col * 3) + 1] = COLORS[pixel];
// 		framebuffer_chr[(r * 128 * 3) + (col * 3) + 2] = COLORS[pixel];
// 	}
// }
void get_pattern_table(uint8_t index){
    for(int row = 0; row < 256; row++){
        for(int col = 0; col < 128; col++){
            uint16_t address = (row / 8 * 0x100) + (row % 8) + (col / 8) * 0x10;

        }
    }
}

/*
olc::Sprite& olc2C02::GetPatternTable(uint8_t i, uint8_t palette)
{

	for (uint16_t nTileY = 0; nTileY < 16; nTileY++)
	{
		for (uint16_t nTileX = 0; nTileX < 16; nTileX++)
		{
			uint16_t nOffset = nTileY * 256 + nTileX * 16;
			for (uint16_t row = 0; row < 8; row++)
			{
				uint8_t tile_lsb = ppuRead(i * 0x1000 + nOffset + row + 0x0000);
				uint8_t tile_msb = ppuRead(i * 0x1000 + nOffset + row + 0x0008);
				for (uint16_t col = 0; col < 8; col++)
				{
					uint8_t pixel = (tile_lsb & 0x01) + (tile_msb & 0x01);
					tile_lsb >>= 1; tile_msb >>= 1;

					sprPatternTable[i]->SetPixel
					(
						nTileX * 8 + (7 - col), 

						nTileY * 8 + row, 
						GetColourFromPaletteRam(palette, pixel)
					);
				}
			}
		}
	}

	return *sprPatternTable[i];
}
*/

