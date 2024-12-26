#include "../include/instructions.h"
#include "../include/bus.h"
#include<stdio.h>
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


/* Register transfer operations */

/* Transfer the value in the accumulator (A) to the X register (TAX instruction). 
   Update the Zero (Z) and Sign (S) flags based on the value of X. */
void TAX(){
  X = A;
  state.Z = (X == 0);
  state.S = (X & 0x80) != 0;
}

/* Transfer the value in the accumulator (A) to the Y register (TAY instruction). 
   Update the Zero (Z) and Sign (S) flags based on the value of Y. */
void TAY(){
  Y = A;
  state.Z = (Y == 0);
  state.S = (Y & 0x80) != 0;
}

/* Transfer the value in the X register to the accumulator (A) (TXA instruction). 
   Update the Zero (Z) and Sign (S) flags based on the value of A. */
void TXA(){
  A = X;
  state.Z = (A == 0);
  state.S = (A & 0x80) != 0;
}

/* Transfer the value in the Y register to the accumulator (A) (TYA instruction). 
   Update the Zero (Z) and Sign (S) flags based on the value of A. */
void TYA(){
  A = Y;
  state.Z = (A == 0);
  state.S = (A & 0x80) != 0;
}

/* Transfer the value in the X register to the Stack Pointer (SP) (TXS instruction). 
   No flags are affected. */
void TXS(){
  SP = X;
}

/* Transfer the value in the Stack Pointer (SP) to the X register (TSX instruction). 
   Update the Zero (Z) and Sign (S) flags based on the value of X. */
void TSX(){
  X = SP;
  state.Z = (X == 0);
  state.S = (X & 0x80) != 0;
}

/* Push the value in the accumulator (A) onto the stack (PHA instruction). 
   Decrement the Stack Pointer (SP). */
void PHA(){
  cpu_write(0x0100 + SP, A);
  SP--;
}

/* Push the status flags (state.raw) onto the stack (PHP instruction). 
   Decrement the Stack Pointer (SP). */
void PHP(){
  cpu_write(0x0100 + SP, state.raw);
  SP--;
}

/* Pull the top value from the stack into the accumulator (PLA instruction). 
   Increment the Stack Pointer (SP) and update the Zero (Z) and Sign (S) flags. */
void PLA(){
  SP++;
  cpu_read(0x0100 + SP, &A);
  state.Z = (A == 0);
  state.S = (A & 0x80) != 0;
}

/* Pull the top value from the stack into the status flags (PLP instruction). 
   Increment the Stack Pointer (SP). */
void PLP(){
  cpu_read(0x0100 + ++SP, &state.raw);
}


/* Logical Operations*/

void AND(){
  A = A & immval;
  state.Z = A == 0;
  state.S = (A & 0x80) != 0;
}

void EOR(){
  A = A ^ immval;
  state.Z = A == 0;
  state.S = (A & 0x80) != 0;
}

void ORA(){
  A = A | immval;
  state.Z = A == 0;
  state.S = (A & 0x80) != 0;
}

void BIT(){
  uint8_t val = A & immval;
  state.Z = (A == 0);
  state.S = (A & 0x80) != 0;
  state.V = (A & 0x40) != 0;
}