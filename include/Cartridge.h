#pragma once
#include<memory>
#include<cstdint>
#include <string>
class Cartridge{
public:
    Cartridge(std::string rom);
    ~Cartridge();
    bool cpuread(uint16_t address, uint8_t& byte);
    bool cpuwrite(uint16_t address, uint8_t byte);
    bool ppuread(uint16_t address, uint8_t& byte);
    bool ppuwrite(uint16_t address, uint8_t byte);  
private:
    std::unique_ptr<uint8_t[]> CHR_ROM; //character rom
    std::unique_ptr<uint8_t[]> PRG_ROM; //program rom
    int fd;
    struct NESHeader{
        char header[4];
        uint8_t prg_rom_size;
        uint8_t chr_rom_size;
        uint8_t flag6;
        uint8_t flag7;
        uint8_t flag8;
        uint8_t flag9;
        uint8_t flag10;
        char padding[5];
    };
    NESHeader metadata;
};