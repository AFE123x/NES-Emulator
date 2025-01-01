#ifndef MAPPER0_H
#define MAPPER0_H
#include<stdint.h>

void mapper_0_cpu_read(uint16_t address, uint32_t* mapped_address);
void mapper_0_cpu_write(uint16_t address, uint32_t* mapped_address);
void mapper_0_ppu_write(uint16_t address,uint32_t* mapped_address);
void mapper_0_ppu_read(uint16_t address,uint32_t* mapped_address);
extern uint8_t mapper_num;
extern uint8_t NPRG_ROM;
extern uint8_t NCHR_ROM;
#endif