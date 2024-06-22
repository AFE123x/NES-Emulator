#include "./../lib/bus.h"
#include "./../lib/cpu.h"

  BUS::BUS(){
    this.ram = new uint8_t[0x10000];
  }
  BUS::~BUS(){
    delete[] ram;
  }
  uint8_t BUS::cpuread(uint16_t addr, bool readonly){
    return ram[addr & 0xFFFF] & 0xFF;
  }
  void BUS::cpuwrite(uint16_t addr, uint8_t byte){
    ram[addr & 0xFFFF] = byte & 0xFF;
  }