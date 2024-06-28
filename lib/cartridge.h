#pragma once
#include <cstdint>
#include "../lib/mapper000.h"
#include <string>
class cartridge {
public:
  cartridge();
  ~cartridge();
  bool cpuread(uint16_t address, uint8_t* byte);
  bool cpuwrite(uint16_t address, uint8_t byte);
  bool ppuread(uint16_t address, uint8_t* byte);
  bool ppuwrite(uint16_t address, uint8_t byte);
  bool insert(std::string buf);
  void clean();

private:
Mapper* mapper = nullptr;
  uint8_t *PRG_ROM = nullptr;
  uint8_t *CHR_ROM = nullptr;
  uint32_t PRG_SIZE;
  uint32_t CHR_SIZE;
  struct ines_t {
    char magic[4]; // Should be "NES\x1A"
    uint8_t PRG_ROM_size;
    uint8_t CHR_ROM_size;
    uint8_t flags_6;
    uint8_t flags_7;
    uint8_t flags_8;
    uint8_t flags_9;
    uint8_t flags_10;
    char unused[5]; // Unused bytes (should be zero-filled)
  };
  ines_t data;
};