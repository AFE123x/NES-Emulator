#include "../include/cpu.h"
#include "../include/bus.h"
#include <stdint.h>
#include <string.h>
#include <stdio.h>
#include <assert.h>
#ifdef UNIT_TESTS
#include<criterion/criterion.h>
#endif
typedef struct
{
    uint8_t A;
    uint8_t B;
    uint8_t C;
    uint8_t D;
    uint8_t E;
    uint8_t F;
    uint8_t H;
    uint8_t L;
} registers_t;

static registers_t registers;

struct flags
{
    uint8_t Z : 1;
    uint8_t N : 1;
    uint8_t H : 1;
    uint8_t C : 1;
} flag;

static uint64_t total_clock_cycles;
static uint8_t cycles_left;
static uint16_t PC; // program counter
static uint16_t SP; // stack pointer

// memory R/W
static void read(uint16_t address, uint8_t *byte)
{
    cpuread(address, byte);
}

static void write(uint16_t address, uint8_t byte)
{
    cpuwrite(address, byte);
}

void initializecpu()
{
    memset(&registers, 0, sizeof(registers_t));
    memset(&flag, 0, sizeof(struct flags));
    total_clock_cycles = 0;
    cycles_left = 0;
    SP = 0;
    PC = 0x0;
}
void printcpustate(){
    printf("A: %x, B: %x, C: %x, D: %x, E: %x, F: %x, H: %x, L: %x\n",registers.A,registers.B,registers.C,registers.D,registers.E,registers.F,registers.H,registers.L);
}
static void combine(uint16_t* output,uint8_t hi, uint8_t lo){
    uint16_t hiw = (uint16_t)hi;
    uint16_t low = (uint16_t)lo;
    *output = (hiw << 8) | low;
}
static void apart(uint16_t input, uint8_t* hi, uint8_t* lo){
    *hi = (uint8_t)(input >> 8);
    *lo = (uint8_t)(input & 0xff);
}


static void ld_r16_imm16(uint8_t registernum){
    uint8_t* mhi = NULL;
    uint8_t* mlo = NULL;
    switch(registernum){
        case 0:
            mhi = &registers.B;
            mlo = &registers.C;
            break;
        case 1:
            mhi = &registers.D;
            mlo = &registers.E;
            break;
        case 2:
            mhi = &registers.H;
            mlo = &registers.L;
            break;
        case 3:
            uint8_t* ptr = (uint8_t*)&SP;
            mhi = &ptr[1];
            mlo = &ptr[0];
            break;
    }
    assert(mhi != NULL);
    assert(mlo != NULL);

    uint8_t lob, hib;
    read(PC++,&lob);
    read(PC++,&hib);
    uint16_t word;
    combine(&word,hib,lob);
    apart(word,mhi,mlo);
}

static void ld_r16mem_a(uint8_t registernum){
    uint8_t* mhi = NULL;
    uint8_t* mlo = NULL;
    switch(registernum){
        case 0:
            mhi = &registers.B;
            mlo = &registers.C;
            break;
        case 1:
            mhi = &registers.D;
            mlo = &registers.E;
            break;
        case 2:
            mhi = &registers.H;
            mlo = &registers.L;
            break;
        case 3:
            uint8_t* ptr = (uint8_t*)&SP;
            mhi = &ptr[1];
            mlo = &ptr[0];
            break;
    }
    assert(mhi != NULL);
    assert(mlo != NULL);
    uint16_t word;
    combine(&word,*mhi,*mlo);
    write(word,registers.A);
}
static void ld_a_r16mem(uint8_t registernum){
    uint8_t* mhi = NULL;
    uint8_t* mlo = NULL;
    switch(registernum){
        case 0:
            mhi = &registers.B;
            mlo = &registers.C;
            break;
        case 1:
            mhi = &registers.D;
            mlo = &registers.E;
            break;
        case 2:
            mhi = &registers.H;
            mlo = &registers.L;
            break;
        case 3:
            uint8_t* ptr = (uint8_t*)&SP;
            mhi = &ptr[1];
            mlo = &ptr[0];
            break;
    }
    assert(mhi != NULL);
    assert(mlo != NULL);
    uint16_t word;
    combine(&word,*mhi,*mlo);
    read(word,&registers.A);
}
void clock()
{
    if (cycles_left == 0)
    {
        uint8_t opcode;
        read(PC++, &opcode);
        if((opcode & 0b11001111) == 00000001){
            ld_r16_imm16((opcode >> 4) & 3);
            cycles_left = 12;
        }
        else if((opcode & 0b11001111) == 0b00000010){
            ld_r16mem_a((opcode >> 4) & 3);
            cycles_left = 8;
        }
        else if((opcode & 0b11001111) == 0b00001010){
            ld_a_r16mem((opcode >> 4) & 3);
            cycles_left = 8;
        }
        else if(opcode == 0b00001000){
            uint8_t byte;
            read(SP,&byte);
            uint8_t lo, hi;
            read(PC++,&lo);
            read(PC++,&hi);
            uint16_t address;
            combine(&address,hi,lo);
            write(address,byte);
            cycles_left = 20;
        }
        write(0,1);
    }
    cycles_left--;
    total_clock_cycles++;
}

#ifdef UNIT_TESTS
Test(cputests,ldr16immtest){
    initializecpu();
    initializebus();
    //moving DEAD to BC
    write(0x0000,0xAD);
    write(0x0001,0xDE);
    ld_r16_imm16(3);
    cr_expect(SP == 0xDEAD,"ldr16immtest instruction - FAILED!\n");
    freebus();
}
Test(cputests,ldr16memtest){
    initializebus();
    initializecpu();
    write(0x0000,0xBE);
    write(0x0001,0xEF);
    registers.A = 69;
    registers.B = 0x7F;
    registers.C = 0xFF;
    ld_r16mem_a(0);
    uint8_t result;
    read(0x7FFF,&result);
    // printf("%d\n",result);
    cr_expect(result == 69,"ldr16memtest - FAILED\n");
    freebus();
}
Test(cputests,ldra16memtest){
    initializebus();
    initializecpu();
    registers.H = 0x7F;
    registers.L = 0xFF;
    write(0x7FFF,95);
    ld_a_r16mem(2);
    cr_expect(registers.A == 95,"ldra16memtest - FAILED\n");
    freebus();
}

Test(cputests, sptor16memtest){
    initializebus();
    initializecpu();
    SP = 0x7FFF;
    write(0x7FFF,69);
    write(0x0000,0x08);
    write(0x0001,0x00);
    write(0x0002,0x10);
    clock();
    uint8_t result;
    read(0x1000,&result);
    cr_expect(result == 69, "sptor16memtest - FAILED!\n");
    freebus();
}
#endif