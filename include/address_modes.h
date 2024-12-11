#ifndef ADDRESSING_MODES_H
#define ADDRESSING_MODES_H

#include <stdint.h>

/* Declare external variables that will be used in addressing modes. */
extern uint8_t immval;   // Immediate value fetched from memory.
extern uint16_t abs_addr; // Absolute address calculated during decoding.
extern int8_t rel_addr;   // Relative address used for branching.
extern uint16_t PC;        // Program Counter.
extern uint8_t X;          // X Register.
extern uint8_t Y;          // Y Register.
extern uint64_t total_cycles; //total clock cycles
extern uint8_t cycles; //cycles left
/* Function prototypes for addressing modes. */
void addr_immediate(void);
void addr_zero_page(void);
void addr_zero_page_x(void);
void addr_zero_page_y(void);
void addr_relative(void);
void addr_absolute(void);
void addr_absolute_x(void);
void addr_absolute_y(void);
void addr_indirect(void);
void addr_indexed_indirect(void);
void addr_indirect_indexed(void);
void addr_implied(void);
#endif // ADDRESSING_MODES_H
