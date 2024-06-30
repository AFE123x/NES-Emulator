#include "../lib/ppu.h"
#include "../lib/cartridge.h"
#include "../lib/telivision.h"
#include <bitset>
#include <iostream>
PPU::PPU() {}
PPU::~PPU() {}
uint8_t PPU::cpuread(uint16_t address, bool readonly) { return 0; }
void PPU::cpuwrite(uint16_t address, uint8_t byte) {}
uint8_t PPU::ppuread(uint16_t address, bool readonly) { return 0; }
void PPU::ppuwrite(uint16_t address, uint8_t byte) {}
void PPU::printBinary(uint8_t value) {
  std::bitset<8> bits(value);
  std::cout << bits;
}
// void PPU::patterntable1() {
//   this->teli = new television();
//   uint8_t height = 128;
//   uint8_t width = 128;
//   teli->initialize(width, height);
//   // 128 x 128
//   int pixel = 0;
//   // y axis, i / 128.
//   // x axis, i % 128.
//   uint16_t address = 0;
//   uint8_t msb = 0;
//   uint8_t lsb = 0;
//   for (int i = 0; i < 0x1000; i += 2) {

//     game->ppuread(i, &lsb);

//     game->ppuread(i + 1, &msb);
//     // read most significant byte
//     std::cout << "bit 1: ";
//     printBinary(lsb);
//     std::cout << "\tbit 2:";
//     printBinary(msb);
//     std::cout << " = ";
//     for (int i = 0; i < 8; i++) {
//       uint8_t bit1 = lsb & (0x80 >> i);
//       uint8_t bit2 = msb & (0x80 >> i);
//       // std::cout << static_cast<int>(bit1) << "\t" <<
//       static_cast<int>(bit2)
//       //  << std::endl;

//       if (bit1 != 0 && bit2 != 0) {
//         std::cout << "3";
//         teli->drawpoint(255, 0, 0, address % width, address / width);
//       } else if (bit1 != 0 && bit2 == 0) {
//         std::cout << "2";
//         teli->drawpoint(0, 255, 0, address % width, address / width);
//       } else if (bit1 == 0 && bit2 != 0) {
//         std::cout << "1";
//         teli->drawpoint(0, 0, 255, address % width, address / width);
//       } else {
//         std::cout << "0";
//         teli->drawpoint(0, 0, 0, address % width, address / width);
//       }
//       address++;
//       //   if (address % 8 == 0) {
//       //     std::cout<<std::endl;
//       //   }
//     }
//     std::cout << std::endl;
//   }
//   teli->refresh();
//   std::cout << static_cast<int>(address) << std::endl;

//   bool running = true;
//   SDL_Event event;
//   while (running) {

//     while (SDL_PollEvent(&event)) {
//       if (event.type == SDL_QUIT) {
//         running = false;
//       }
//     }
//     // Draw static

//     SDL_Delay(50);
//   }
//   teli->tvclose();
//   delete teli;
// }
void PPU::patterntable1() {
  this->teli = new television();
  uint8_t height = 128;
  uint8_t width =  128;
  teli->initialize(width, height);

  uint16_t address = 0;
  uint8_t msb = 0;
  uint8_t lsb = 0;

  for (int r = 0; r < 256;
       r++) { // Iterate over 256 rows (NES pattern table rows)
    for (int col = 0; col < 128;
         col++) { // Iterate over 128 columns (NES pattern table columns)
      // Calculate VRAM address for the current pixel
      uint16_t adr = (r / 8 * 0x100) + (r % 8) + (col / 8) * 0x10;

      // Read pattern table data
      game->ppuread(adr, &lsb);
      game->ppuread(adr + 8, &msb);

      // Determine pixel color index based on bit planes
      uint8_t pixel =
          ((lsb >> (7 - (col % 8))) & 1) + ((msb >> (7 - (col % 8))) & 1) * 2;

      // Draw pixel on SDL surface
      /*
      
        framebuffer_chr[(r * 128 * 3) + (col * 3)] = COLORS[pixel];
        framebuffer_chr[(r * 128 * 3) + (col * 3) + 1] = COLORS[pixel];
        framebuffer_chr[(r * 128 * 3) + (col * 3) + 2] = COLORS[pixel];
*/
      if (pixel == 0) {
        teli->drawpoint(0, 0, 0, col, r); // Transparent or background
      } else if (pixel == 1) {
        teli->drawpoint(0, 0, 255, col, r); // Color index 1
      } else if (pixel == 2) {
        teli->drawpoint(0, 255, 0, col, r); // Color index 2
      } else if (pixel == 3) {
        teli->drawpoint(215, 203, 255, col, r); // Color index 3
      } //rgb(215,203,255)

      address++;
    }
  }

  // Event handling loop to keep the SDL window open
  bool running = true;
  SDL_Event event;
  while (running) {
    while (SDL_PollEvent(&event)) {
      if (event.type == SDL_QUIT) {
        running = false;
      }
    }
    teli->refresh(); // Refresh the SDL surface
    SDL_Delay(50);   // Delay to control frame rate
  }

  teli->tvclose(); // Close the SDL window
  delete teli;     // Clean up allocated resources
}

void PPU::connectcart(cartridge *cart) { this->game = cart; }
void PPU::clock() {}