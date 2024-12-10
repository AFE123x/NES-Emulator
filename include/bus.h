#ifndef BUS_H
#define BUS_H
#include<stdint.h>
void run_system(char* rom);
void cpu_read(uint16_t address, uint8_t* byte);
void cpu_write(uint16_t address,uint8_t byte);
#endif