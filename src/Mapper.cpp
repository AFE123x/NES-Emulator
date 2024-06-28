// Mapper.cpp
#include "../lib/mapper.h"

Mapper::Mapper() : prg_size(0), chr_size(0) {}

bool Mapper::cpuread(uint16_t address, uint16_t* map_address) {
    return false;
}

bool Mapper::cpuwrite(uint16_t address, uint16_t* map_address) {
    return false;
}

bool Mapper::ppuread(uint16_t address, uint16_t* map_address) {
    return false;
}

bool Mapper::ppuwrite(uint16_t address, uint16_t* map_address) {
    return false;
}
