#pragma once
#include <cstdint>
#include <iostream>
class cartridge;
class CPU;
class PPU;
class BUS {
public:
  BUS();
  ~BUS();
  uint8_t cpuread(uint16_t addr, bool readonly = true);
  void cpuwrite(uint16_t addr, uint8_t byte);
  void clock();
private:
  uint8_t* ram;
  cartridge* game;
  PPU* ppu;
  CPU* cpu;
};