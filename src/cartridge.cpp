#include "../lib/cartridge.h"

#include <fcntl.h>
#include <iostream>
#include <unistd.h>
cartridge::cartridge() {}
cartridge::~cartridge() {}
bool cartridge::cpuread(uint16_t address, uint8_t *byte) {
  uint16_t mapped_address;
  if (mapper->cpuread(address, &mapped_address)) {
    *byte = PRG_ROM[mapped_address];
    return true;
  }
  return false;
}
bool cartridge::cpuwrite(uint16_t address, uint8_t byte) {
  uint16_t mapped_address;
  if (mapper->cpuwrite(address, &mapped_address)) {
    PRG_ROM[mapped_address] = byte;
    return true;
  }
  return false;
}
bool cartridge::ppuread(uint16_t address, uint8_t *byte) {
  uint16_t mapped_address;
  if (mapper->ppuread(address, &mapped_address)) {
    *byte = CHR_ROM[mapped_address];
    return true;
  }
  return false;
}
bool cartridge::ppuwrite(uint16_t address, uint8_t byte) {
  uint16_t mapped_address;
  if (mapper->ppuwrite(address, &mapped_address)) {
    CHR_ROM[mapped_address] = byte;
    return true;
  }
  return false;
}
bool cartridge::insert(std::string buf) {
  int fd = open(buf.c_str(), O_RDONLY);
  if (fd == -1) {
    std::cerr << "unable to open file" << std::endl;
    return false;
  }
  ssize_t bytes_read = read(fd, &data, sizeof(ines_t));
  if (bytes_read != sizeof(ines_t)) {
    std::cerr << "invalid nes file" << std::endl;
    return false;
  }
  PRG_SIZE = data.PRG_ROM_size * 16384;
  CHR_SIZE = data.CHR_ROM_size * 8192;
  PRG_ROM = new uint8_t[PRG_SIZE];
  CHR_ROM = new uint8_t[CHR_SIZE];
  if (data.flags_6 & 0x4) {
    char BS[512];
    read(fd, BS, 512);
  }
  read(fd, PRG_ROM, PRG_SIZE); // read program
  read(fd, CHR_ROM, CHR_SIZE); // read character stuff
  uint8_t mappa = ((data.flags_7 & 0xF0) | (data.flags_6 >> 4));
  close(fd);
  switch (mappa) {
  case 00:
    mapper = new mapper000(data.PRG_ROM_size, data.CHR_ROM_size);
    return true;
  default:
    std::cerr << "invalid mapper" << std::endl;
    return false;
  }

  return true;
}

void cartridge::clean() {
    delete[] PRG_ROM;
    delete[] CHR_ROM;
  if (mapper != nullptr) {
    delete mapper;
  }
  mapper = nullptr;
  PRG_ROM = nullptr;
  CHR_ROM = nullptr;
}