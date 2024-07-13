#include "../include/Cartridge.h"
#include <cstdio>
#include <cstring>
#include <fcntl.h>
#include <iostream>
#include <memory>
#include <sys/types.h>
#include <unistd.h>
Cartridge::Cartridge(std::string rom) {
  fd = open(rom.c_str(), O_RDONLY);
  memset(&metadata, 0, sizeof(metadata));
  ssize_t bytesread = read(fd, &metadata, sizeof(struct NESHeader));
  if (bytesread != sizeof(struct NESHeader)) {
    std::cerr << "error reading file" << std::endl;
    exit(0);
  }
  ssize_t chr_size = metadata.chr_rom_size * 8192;
  ssize_t prg_size = metadata.prg_rom_size * 16384;
  CHR_ROM = std::make_unique<uint8_t[]>(chr_size);
  PRG_ROM = std::make_unique<uint8_t[]>(prg_size);
  // check if there's  the trainer
  if (metadata.flag6 & 0x4) {
    lseek(fd, 512, SEEK_CUR);
  }
  bytesread = read(fd, PRG_ROM.get(), prg_size);
  bytesread = read(fd, CHR_ROM.get(), chr_size);
  close(fd);
}

Cartridge::~Cartridge() {}

bool Cartridge::cpuread(uint16_t address, uint8_t &byte) {
  if (address >= 0x8000) {
    byte = PRG_ROM[address & 0x3FFF];
    return true;
  }
  return false;
}
bool cpuwrite(uint16_t address, uint8_t byte);