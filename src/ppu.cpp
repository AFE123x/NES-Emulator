#include "../lib/ppu.h"

PPU::PPU(){

}
PPU::~PPU(){

}
uint8_t PPU::cpuread(uint16_t address, bool readonly) {
    return 0;
}
void PPU::cpuwrite(uint16_t address, uint8_t byte) {
    return 0;
}
uint8_t PPU::ppuread(uint16_t address, bool readonly) {
    return 0;
}
void PPU::ppuwrite(uint16_t address, uint8_t byte) {}
void PPU::connectcart(cartridge* cart){
    this->game = cart;
}
void PPU::clock(){
    
}