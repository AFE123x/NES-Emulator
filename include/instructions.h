#ifndef INSTRUCTIONS_H
#define INSTRUCTIONS_H
#include<stdint.h>
/* load / store operations */
/**
 * @brief Processor state representation.
 * 
 * The 6502 processor's status flags are represented as individual bits
 * within a union. The flags include:
 * - C: Carry Flag
 * - Z: Zero Flag
 * - I: Interrupt Disable
 * - D: Decimal Mode (unused in some systems)
 * - B: Break Command
 * - V: Overflow Flag
 * - S: Negative Flag (sign bit)
 * 
 * The `raw` field allows direct access to the entire status byte.
 */
typedef union {
    struct {
        uint8_t C : 1;
        uint8_t Z : 1;
        uint8_t I : 1;
        uint8_t D : 1;
        uint8_t B : 1;
        uint8_t V : 1;
        uint8_t S : 1;
    };
    uint8_t raw; // Direct access to all flags as a single byte.
} processor_state;


extern uint8_t immval;   // Immediate value fetched from memory.
extern uint16_t abs_addr; // Absolute address calculated during decoding.
extern int8_t rel_addr;   // Relative address used for branching.
extern uint16_t PC;        // Program Counter.
extern uint8_t X;          // X Register.
extern uint8_t Y;          // Y Register.
extern processor_state state; // CPU status flags instance.
extern uint8_t A; // Accumulator: Used for arithmetic and logic operations.


/* LOAD/STORE OPERATIONS */

void LDA();
void LDX();
void LDY();
void STA();
void STX();
void STY();

/* REGISTER TRANSFERS OPERATIONS */

void TAX();
void TAY();
void TXA();
void TYA();


#endif
