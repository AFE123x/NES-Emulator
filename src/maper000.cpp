
#include "../lib/mapper000.h"
#include <iostream>

mapper000::mapper000(uint8_t prg_size, uint8_t chr_size) {
  this->prg_size = prg_size;
  this->chr_size = chr_size;
}

bool mapper000::cpuread(uint16_t address, uint16_t *map_address) {
  if (address >= 0x8000 && address <= 0xFFFF) {
    uint16_t mapped = prg_size > 1 ? 0x7FFF : 0x3FFF;
    *map_address = address & mapped;
    return true;
  }
  return false;
}
bool mapper000::cpuwrite(uint16_t address, uint16_t *map_address) {
  if (address >= 0x8000 && address <= 0xFFFF) {
    uint16_t mapped = prg_size > 1 ? 0x7FFF : 0x3FFF;
    *map_address = address & mapped;
    return true;
  }
  return false;
}
bool mapper000::ppuread(uint16_t address, uint16_t *map_address) {
  if (address <= 0x1FFF) {
    *map_address = address;
    return true;
  }
  return false;
}
bool mapper000::ppuwrite(uint16_t address, uint16_t *map_address) {

  return false;
}