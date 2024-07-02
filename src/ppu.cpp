#include "../lib/ppu.h"
#include "../lib/cartridge.h"
#include "../lib/telivision.h"
#include <bitset>
#include <iostream>
PPU::PPU() {}
PPU::~PPU() {}

/**
 * @brief PPU read function for cpu
 *
 * @param address the address we want to read from
 * @param readonly toggle between read only and write only
 * @return uint8_t The byte to read from.
 */
uint8_t PPU::cpuread(uint16_t address, bool readonly) { return 0; }

/**
 * @brief PPU write function for cpu bus.
 *
 * @param address we want to write to
 * @param byte the byte we want to write.
 */
void PPU::cpuwrite(uint16_t address, uint8_t byte) {}

/**
 * @brief ppu read function for ppu bus.
 *
 * @param address
 * @param readonly
 * @return uint8_t
 */
uint8_t PPU::ppuread(uint16_t address, bool readonly) { return 0; }
void PPU::ppuwrite(uint16_t address, uint8_t byte) {}
void PPU::printBinary(uint8_t value) {
  std::bitset<8> bits(value);
  std::cout << bits;
}

void PPU::patterntable1() {
  this->teli = new television();
  uint16_t height = 128;
  uint16_t width = 128;
  teli->initialize(1280/2, 720/2);
  uint8_t table1[width][height];
  uint16_t address = 0;
  uint8_t msb = 0;
  uint8_t lsb = 0;

  for (int col = 0; col < height;
       col++) { // Iterate over 256 rows (NES pattern table rows)
    for (int row = 0; row < width; row++) {
      address = ((row >> 3) << 4) + ((col >> 3) << 8);

      // Read pattern table data
      game->ppuread(address + (col & 0x7), &lsb);
      game->ppuread(address + ((col & 0x7) + 8), &msb);
      lsb = (lsb & (0x80 >> (row % 8))) ? 1 : 0;
      msb = (msb & (0x80 >> (row % 8))) ? 1 : 0;
      uint8_t pixel = (msb << 1) | lsb;

      if (pixel == 0) {
        table1[col][row] = 0;
        teli->drawpoint(0, 0, 0, 1004 + row,
                        582 + col); // Transparent or background

      } else if (pixel == 1) {
        table1[col][row] = 1;
        teli->drawpoint(0, 0, 255, 364 + row, 222 + col); // Color index 1
      } else if (pixel == 2) {
        table1[col][row] = 2;
        teli->drawpoint(0, 255, 0, 364 + row, 222 + col); // Color index 2
      } else if (pixel == 3) {
        table1[col][row] = 3;
        teli->drawpoint(215, 203, 255, 364 + row, 222 + col); // Color index 3
      } // rgb(215,203,255)

      address++;
    }
  }
  // teli->refresh(); // Refresh the SDL surface
  // // Event handling loop to keep the SDL window open
  // bool running = true;
  // SDL_Event event;
  // while (running) {
  //   while (SDL_PollEvent(&event)) {
  //     if (event.type == SDL_QUIT) {
  //       running = false;
  //     }
  //   }

  //   SDL_Delay(50); // Delay to control frame rate
  // }

  // teli->tvclose(); // Close the SDL window
  // delete teli;     // Clean up allocated resources
}
void PPU::patterntable2() {
  // this->teli = new television();
  uint16_t height = 128;
  uint16_t width = 128;
  // teli->initialize(width, height);
  uint8_t table1[width][height];
  uint16_t address = 0;
  uint8_t msb = 0;
  uint8_t lsb = 0;

  for (int col = 0; col < height;
       col++) { // Iterate over 256 rows (NES pattern table rows)
    for (int row = 0; row < width; row++) {
      address = ((row >> 3) << 4) + ((col >> 3) << 8);

      // Read pattern table data
      game->ppuread(0x1000 + address + (col & 0x7), &lsb);
      game->ppuread(0x1000 + address + ((col & 0x7) + 8), &msb);
      lsb = (lsb & (0x80 >> (row % 8))) ? 1 : 0;
      msb = (msb & (0x80 >> (row % 8))) ? 1 : 0;
      uint8_t pixel = (msb << 1) | lsb;
      if (pixel == 0) {
        table1[col][row] = 0;
        teli->drawpoint(0, 0, 0, 502 + row, 222 + col); // Transparent or background
      } else if (pixel == 1) {
        table1[col][row] = 1;
        teli->drawpoint(0, 0, 255, 502 + row, 222 + col); // Color index 1
      } else if (pixel == 2) {
        table1[col][row] = 2;
        teli->drawpoint(0, 255, 0, 502 + row, 222 + col); // Color index 2
      } else if (pixel == 3) {
        table1[col][row] = 3;
        teli->drawpoint(215, 203, 255, 502 + row, 222 + col); // Color index 3
      } // rgb(215,203,255)

      address++;
    }
  }
}

void PPU::connectcart(cartridge *cart) { this->game = cart; }
void PPU::clock() {}

bool PPU::drawdisassembly(std::string &dissasembly) {
  bool thing =  teli->drawdisassembly(dissasembly);
  teli->refresh(); // Refresh the SDL surface
  // Event handling loop to keep the SDL window open
  bool running = true;
  SDL_Event event;
  while (running) {
    while (SDL_PollEvent(&event)) {
      if (event.type == SDL_QUIT) {
        running = false;
      }
    }

    SDL_Delay(50); // Delay to control frame rate
  }

  teli->tvclose(); // Close the SDL window
  delete teli;     // Clean up allocated resources
  return true;
}