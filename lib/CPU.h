#ifndef CPU_H
#define CPU_H
#include "./mainbus.h"
// defining some condition flag macros;
/*
+---------- N: NEGATIVE
|+--------- V: Overflow
||+-------- B: Unused
|||+------- B: The B flag
||||
00000000
    ||||
    |||+--- C: Carry flag
    ||+---- Z: Zero flag
    |+----- I: Interrupt Disable
    +------ D: Decimal (Not used in NES)

*/
#define NEGATIVE 0b10000000
#define OVERFLOW 0b01000000
#define BFLAG 0b00010000
#define DECIMAL 0b00001000
#define INTERRUPT_DISABLE 0b00000100
#define ZERO 0b00000010
#define CARRY 0b00000001

class CPU
{
public:
    CPU(mainbus bus);
    ~CPU();
private:
    uint8_t fetch();
    uint8_t get_flag(char flag);
    void set_flag(char flag, bool enable);
    mainbus* bus;
    // registers
    uint16_t PC;            // program counter
    uint8_t SP;             // stack pointer
    uint8_t A;              // accumulator register
    uint8_t X;              // X register
    uint8_t Y;              // Y register
    uint8_t Register_flags; // register flags

    // our instruction
    typedef struct
    {
        uint8_t (*addressing_mode)();
        void (*instruction)(uint8_t data);
        uint8_t bytes;
        uint8_t cycles;

    } instruction_t;
};
#endif