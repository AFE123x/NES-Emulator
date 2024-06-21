#include <cstdint>
#pragma once
class CPU;
class PPU;
class BUS {
public:
  BUS();
  ~BUS();
  uint8_t cpuread(uint16_t address, bool readonly = true);
  void cpuwrite(uint16_t address, uint8_t byte);
  void execute();

private:
  uint8_t *cpuram;
  CPU *cpu;
  PPU *ppu;
};