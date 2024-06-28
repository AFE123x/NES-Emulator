#pragma once
#include "./mapper.h"
class mapper000 : public Mapper {
public:
  mapper000(uint8_t prg_size, uint8_t chr_size);
  bool cpuread(uint16_t address, uint16_t* map_address) override;
  bool cpuwrite(uint16_t address, uint16_t* map_address) override;
  bool ppuread(uint16_t address, uint16_t* map_address) override;
  bool ppuwrite(uint16_t address, uint16_t* map_address) override;
};