#pragma once
#include<cstdint>
class Mapper{
    public:
    Mapper();
    // virtual ~mapper() = default;
    virtual bool cpuread(uint16_t address, uint16_t* map_address);
    virtual bool cpuwrite(uint16_t address, uint16_t* map_address);
    virtual bool ppuread(uint16_t address, uint16_t* map_address);
    virtual bool ppuwrite(uint16_t address, uint16_t* map_address);
    protected:
    uint8_t prg_size;
    uint8_t chr_size;
};