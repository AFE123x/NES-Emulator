#pragma once
class cartridge;
#include<cstdint>
class television;
class PPU {
public:
  PPU();
  ~PPU();
  uint8_t cpuread(uint16_t address, bool readonly = true);
  void cpuwrite(uint16_t address, uint8_t byte);
  uint8_t ppuread(uint16_t address, bool readonly = true);
  void ppuwrite(uint16_t address, uint8_t byte);
  void connectcart(cartridge* cartridge);
  void patterntable1();
  void clock();
  void printBinary(uint8_t value);
private:
cartridge* game;
television* teli;
};