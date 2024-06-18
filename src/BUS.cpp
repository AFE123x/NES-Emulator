#include "../lib/BUS.h"
#include "../lib/CPU.h"
#include <iostream>

BUS::BUS() {
  ram = new uint8_t[65535];
  std::cout << "BUS initialized" << std::endl;
}
BUS::~BUS() {
  delete[] ram;
  std::cout << "BUS deleted" << std::endl;
}
uint8_t BUS::read(uint16_t address, bool readonly) {
 return ram[address] &0xFF;
}
void BUS::write(uint16_t address, uint8_t byte) {
  ram[address] = byte & 0xFF;
}