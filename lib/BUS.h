#include <cstdint>
#pragma once

class BUS {
public:
  BUS();
  ~BUS();
  uint8_t read(uint16_t address, bool readonly = true);
  void write(uint16_t address, uint8_t byte);

private:
  uint8_t *ram;
};