#include <cstdint>
#pragma once

class BUS {
public:
  BUS();
  ~BUS();
  void read(uint16_t address, bool readonly = 1);
  void write(uint16_t address, uint8_t byte);

private:
  uint8_t *ram;
};