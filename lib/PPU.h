#pragma once
#include<cstdint>
class PPU{
    public:
        PPU();
        ~PPU();
        void PPUWrite(uint16_t address, uint8_t data);
        uint8_t PPURead(uint16_t address, bool readonly = true);
    private:

};