#ifndef CARTRIDGE_H
#define CARTRIDGE_H
#include<stdint.h>
#include<stdio.h>
typedef struct{
    char Constants[4]; //contains "NES" with a MS-DOS EOF
    uint8_t PRG_ROM; //size of PRG-ROM in 16 KB units
    uint8_t CHR_ROM; //size of CHR_ROM in 8 KB units
    uint8_t flag6; //Mapper, mirroring, battery, trainer 
    uint8_t flag7; //Mapper, VS/Playchoice, NES 2.0
    uint8_t flag8; //PRG-RAM size (rarely used extension) 
    uint8_t flag9; //TV system (rarely used extension) 
    uint8_t flag10; //TV system, PRG-RAM presence (unofficial, rarely used extension) 
    char padding[5]; //5 bytes of padding
} NES_Header;



void loadrom(char* romfile);
void freerom();
uint8_t mapper_num;
uint8_t NPRG_ROM;
uint8_t NCHR_ROM;
void rom_cpu_read(uint16_t address, uint8_t* byte);
void rom_ppu_read(uint16_t address, uint8_t* byte);

void rom_cpu_write(uint16_t address, uint8_t byte);
void rom_ppu_write(uint16_t address, uint8_t byte);
#endif