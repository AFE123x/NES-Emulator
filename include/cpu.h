#ifndef CPU_H
#define CPU_H
void cpu_init();
void clock_cpu();
#include<stdint.h>
struct cpu_test{
    uint16_t PC;
    uint8_t two_byte;
    uint8_t three_byte;
    uint8_t A;
    uint8_t X;
    uint8_t Y;
    uint8_t SP;
};
struct cpu_test get_status();

#endif