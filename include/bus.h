#ifndef BUS_H
#define BUS_H
#include<stdint.h>
//to initialize the system
void initializebus();
void freebus();
void clockbus();

//load rom into memory
void loadrom(const char* path);

//for the CPU
void cpuread(uint16_t address, uint8_t* byte);
void cpuwrite(uint16_t address, uint8_t byte);
#endif