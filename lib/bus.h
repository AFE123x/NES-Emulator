#pragma once
#include <cstdint>
#include <iostream>
class CPU;
class BUS {
public:
  BUS();
  ~BUS();
  uint8_t cpuread(uint16_t addr, bool readonly = true);
  void cpuwrite(uint16_t addr, uint8_t byte);

private:
  uint8_t* ram;
};