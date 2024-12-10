#include "../include/instructions.h"
#include "../include/bus.h"

/**
 * @brief Load Accumulator with Memory (LDA)
 * 
 * Loads a value from memory (or an immediate value) into the accumulator (A).
 * Updates the processor's Zero (Z) and Sign (S) flags based on the value.
 */
void LDA()
{
    A = immval;            // Load the immediate value into the accumulator.
    state.Z = A == 0;      // Set Zero flag if A is 0.
    state.S = ((A & 0x80) != 0); // Set Sign flag if the most significant bit of A is 1 (negative).
}

/**
 * @brief Load X Register with Memory (LDX)
 * 
 * Loads a value from memory (or an immediate value) into the X register.
 * Updates the processor's Zero (Z) and Sign (S) flags based on the value.
 */
void LDX()
{
    X = immval;            // Load the immediate value into the X register.
    state.Z = X == 0;      // Set Zero flag if X is 0.
    state.S = ((X & 0x80) != 0); // Set Sign flag if the most significant bit of X is 1 (negative).
}

/**
 * @brief Load Y Register with Memory (LDY)
 * 
 * Loads a value from memory (or an immediate value) into the Y register.
 * Updates the processor's Zero (Z) and Sign (S) flags based on the value.
 */
void LDY()
{
    Y = immval;            // Load the immediate value into the Y register.
    state.Z = Y == 0;      // Set Zero flag if Y is 0.
    state.S = ((Y & 0x80) != 0); // Set Sign flag if the most significant bit of Y is 1 (negative).
}

/**
 * @brief Store Accumulator in Memory (STA)
 * 
 * Stores the value of the accumulator (A) into a memory location specified by abs_addr.
 */
void STA()
{
    cpu_write(abs_addr, A); // Write the value of A to the absolute memory address.
}

/**
 * @brief Store X Register in Memory (STX)
 * 
 * Stores the value of the X register into a memory location specified by abs_addr.
 */
void STX()
{
    cpu_write(abs_addr, X); // Write the value of X to the absolute memory address.
}

/**
 * @brief Store Y Register in Memory (STY)
 * 
 * Stores the value of the Y register into a memory location specified by abs_addr.
 */
void STY()
{
    cpu_write(abs_addr, Y); // Write the value of Y to the absolute memory address.
}
