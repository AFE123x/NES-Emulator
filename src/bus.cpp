#include "./../lib/bus.h"
#include "./../lib/cartridge.h"
#include "./../lib/cpu.h"
#include "./../lib/ppu.h"
#include <cstring>
BUS::BUS() {
  this->ram = new uint8_t[0x0800];
  memset(ram, 0, 0x0800);
  game = new cartridge();
  bool success = !game->insert("/home/afe123x/Documents/projects/NES-Emulator/tests/nestest.nes");
  if (success) {
    game->clean();
    return;
  }
  ppu = new PPU();
  ppu->connectcart(game);
  cpu = new CPU(this);
  cpu->debug_enable = true;
}
BUS::~BUS() {
  delete[] ram;
  game->clean();
  delete game;
  delete ppu;
  // game->clean();
  delete cpu;
}
uint8_t BUS::cpuread(uint16_t addr, bool readonly) {
  uint8_t data = 0x00;
  if (addr <= 0x1FFF) {
    data = ram[addr & 0x07FF] & 0xFF;
  } else if (addr >= 0x2000 && addr <= 0x3FFF) {
    data = ppu->cpuread(addr, readonly);
  } else if (addr >= 0x4000 && addr <= 0x4017) {
    // NES APU and I/O registers
  } else {
    game->cpuread(addr, &data);
  }

  return data;
}
void BUS::cpuwrite(uint16_t addr, uint8_t byte) {
  if (addr <= 0x1FFF) {
    ram[addr & 0x07FF] = byte & 0xFF;
  } else if (addr >= 0x2000 && addr <= 0x3FFF) {
    ppu->cpuwrite(addr, byte);
  } else if (addr >= 0x4000 && addr <= 0x4017) {
    // NES APU and I/O registers
  } else {
    game->cpuwrite(addr, byte);
  }
}

void BUS::clock() {
  ppu->patterntable1();
  ppu->patterntable2();
  std::string astring = cpu->disassemble(0x8000, 0x8000);
  ppu->drawdisassembly(astring);
}
