#include "../lib/BUS.h"
#include "../lib/CPU.h"
#include "../lib/PPU.h"
#include <iostream>

BUS::BUS() {
  cpuram = new uint8_t[0x10000]; //0x0800
  this->cpu = new CPU(this);
  this->ppu = new PPU();
  std::cout << "BUS initialized" << std::endl;
}
BUS::~BUS() {
  delete[] cpuram;
  std::cout << "BUS deleted" << std::endl;
}

void BUS::execute(){
  while(1){
    cpu->tick();
  }
}
/*
2000 - 0010 0000 0000 0000
2001 - 0010 0000 0000 0001
2002 - 0010 0000 0000 0010
2003 - 0010 0000 0000 0011
2004 - 0010 0000 0000 0100
2005 - 0010 0000 0000 0101
2006 - 0010 0000 0000 0110
2007 - 0010 0000 0000 0111
2008 - 0010 0000 0000 1000
2009 - 0010 0000 0000 1001
2010 - 0010 0000 0000 1010
*/
uint8_t BUS::cpuread(uint16_t address, bool readonly) {
  if (address <= 0x1FFF) {
    return cpuram[address & 0xFFF] & 0xFF;
  } else if (address >= 0x2000 && address <= 0x3FFF) {
    uint16_t temp = 0x2000 + (address & 0x7);
    return ppu->PPURead(temp);
  } else if (address >= 0x4000 && address <= 4017) {
    // figure out
  }
  return 0;
}

void BUS::cpuwrite(uint16_t address, uint8_t byte) {
  if (address >= 0x000 && address <= 0x1FFF) {
    cpuram[address & 0xFFF] = byte & 0xFF;
  } else if (address >= 0x2000 && address <= 0x3FFF) {
    uint16_t temp = 0x2000 + (address & 0x7);
    return ppu->PPUWrite(temp, byte);
  } else if (address >= 0x4000 && address <= 4017) {
    // figure out
  }
}