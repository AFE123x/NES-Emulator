#include "../include/cpu.h"
#include "../include/bus.h"
#include "../include/address_modes.h"
#include "../include/instructions.h"
#include <stdio.h>
#include<stdlib.h>
#include<assert.h>

/* Helper global variables */
// Temporary variables used during instruction decoding and execution.
uint8_t immval;   // Immediate value fetched from memory.
uint16_t abs_addr; // Absolute address calculated during decoding.
int8_t rel_addr;   // Relative address used for branching.

/* Special registers */
// Registers specific to the 6502 CPU architecture.
uint16_t PC; // Program Counter: Points to the next instruction to execute.
uint8_t SP;  // Stack Pointer: Points to the top of the stack in memory.


processor_state state; // CPU status flags instance.

/* General-purpose registers */
// Registers used during computation and addressing.
uint8_t A; // Accumulator: Used for arithmetic and logic operations.
uint8_t X; // X Register: Often used for indexing.
uint8_t Y; // Y Register: Often used for indexing.

/**
 * @brief Structure for representing a CPU instruction.
 * 
 * - `address_mode`: Function pointer to the addressing mode handler.
 * - `instruction`: Function pointer to the actual instruction handler.
 * - `cycles`: Number of clock cycles the instruction takes.
 * - `name`: Human-readable name of the instruction (for debugging/logging).
 */
typedef struct {
    void (*address_mode)(void); // Addressing mode handler.
    void (*instruction)(void); // Instruction handler.
    uint8_t cycles;            // Cycle count.
    char name[10];             // Instruction name.
} instructions_t;





/* CPU Initialization */
/**
 * @brief Initializes the CPU.
 */
void cpu_init() {
    printf("Initialized\n");
}

/* CPU Clock Cycle */
/**
 * @brief Executes a single CPU clock cycle.
 */
void clock() {
    printf("clock");
}
