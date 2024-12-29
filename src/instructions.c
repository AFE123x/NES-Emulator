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

/**
 * Perform a bitwise and with the Accumulator register and a byte from memory.
 * Zero flag set if A is 0.
 * Signed flag set if A is negative.
 */
void AND(){
  A = A & immval;
  state.Z = A == 0;
  state.S = (A & 0x80) != 0;
}


/**
 * @brief Emulates the EOR (Exclusive OR) instruction.
 * 
 * @desc The EOR instruction performs a bitwise XOR operation between the accumulator (A)
 *       and an immediate value (immval). The result is stored in the accumulator, and the
 *       processor's zero (Z) and sign (S) flags are updated.
 * 
 * @steps
 * 1. Perform a bitwise XOR operation: A = A ^ immval.
 * 2. Update the zero (Z) flag if the result is zero.
 * 3. Update the sign (S) flag based on the MSB of the result.
 */
void EOR() {
    A = A ^ immval; // @action Perform XOR operation.
    state.Z = A == 0; // @flag Set zero flag if result is zero.
    state.S = (A & 0x80) != 0; // @flag Set sign flag based on MSB of the result.
}

/**
 * @brief Emulates the ORA (Logical Inclusive OR) instruction.
 * 
 * @desc The ORA instruction performs a bitwise OR operation between the accumulator (A)
 *       and an immediate value (immval). The result is stored in the accumulator, and the
 *       processor's zero (Z) and sign (S) flags are updated.
 * 
 * @steps
 * 1. Perform a bitwise OR operation: A = A | immval.
 * 2. Update the zero (Z) flag if the result is zero.
 * 3. Update the sign (S) flag based on the MSB of the result.
 */
void ORA() {
    A = A | immval; // @action Perform OR operation.
    state.Z = A == 0; // @flag Set zero flag if result is zero.
    state.S = (A & 0x80) != 0; // @flag Set sign flag based on MSB of the result.
}

/**
 * @brief Emulates the BIT (Bit Test) instruction.
 * 
 * @desc The BIT instruction performs a bitwise AND operation between the accumulator (A)
 *       and an immediate value (immval). It updates the zero (Z), sign (S), and overflow (V)
 *       flags without storing the result in any register.
 * 
 * @steps
 * 1. Perform a bitwise AND operation: val = A & immval.
 * 2. Update the zero (Z) flag if the result is zero.
 * 3. Update the sign (S) flag based on the MSB of immval.
 * 4. Update the overflow (V) flag based on the sixth bit of immval.
 */
void BIT() {
    uint8_t val = A & immval; // @temp Perform AND operation.
    state.Z = (val == 0); // @flag Set zero flag if result is zero.
    state.S = (val & 0x80) != 0; // @flag Set sign flag based on MSB of immval.
    state.V = (val & 0x40) != 0; // @flag Set overflow flag based on the sixth bit of immval.
}



/**
 * @brief Emulates the ADC (Add with Carry) instruction.
 * 
 * @desc The ADC instruction adds the accumulator (A), an immediate value (immval),
 *       and the carry flag (C) together. It updates the processor's carry (C),
 *       zero (Z), overflow (V), and sign (S) flags.
 * 
 * @steps
 * 1. Add the accumulator, immediate value, and carry flag.
 * 2. Calculate carry flag (C) based on overflow beyond 8 bits.
 * 3. Update the zero (Z) flag if the result is zero.
 * 4. Update the overflow (V) flag based on signed overflow.
 * 5. Update the sign (S) flag based on the most significant bit.
 * 6. Store the lower 8 bits of the result back in the accumulator.
 */
void ADC() {
    uint8_t a = A; // @desc The accumulator value.
    uint8_t b = immval; // @desc The immediate value.
    uint8_t c = (state.C) ? 1 : 0; // @desc The carry flag as a binary value.
    uint16_t result = a + b + c; // @action Perform the addition.
    
    char a_prop = a & 0x80; // @temp Sign bit of A.
    char b_prop = b & 0x80; // @temp Sign bit of B.
    char c_prop = (result & 0x80); // @temp Sign bit of the result.

    state.C = (result > 255); // @flag Set carry flag if result exceeds 8 bits.
    state.Z = (result == 0); // @flag Set zero flag if result is zero.
    state.V = ((c_prop ^ a_prop) & (c_prop ^ b_prop)) != 0; // @flag Detect signed overflow.
    state.S = (result & 0x80) != 0; // @flag Set sign flag based on MSB.
    A = (uint8_t)(result & 0xFF); // @action Store lower 8 bits in A.
}

/**
 * @brief Emulates the SBC (Subtract with Borrow) instruction.
 * 
 * @desc The SBC instruction subtracts the immediate value (immval) and borrow
 *       (inverted carry flag) from the accumulator (A). It updates the carry (C),
 *       zero (Z), overflow (V), and sign (S) flags.
 * 
 * @steps
 * 1. Perform subtraction using two's complement arithmetic.
 * 2. Calculate carry flag (C) based on underflow.
 * 3. Update the zero (Z) flag if the result is zero.
 * 4. Update the overflow (V) flag based on signed overflow.
 * 5. Update the sign (S) flag based on the MSB of the result.
 * 6. Store the lower 8 bits of the result back in the accumulator.
 */
void SBC() {
    uint8_t a = A; // @desc The accumulator value.
    uint8_t b = immval; // @desc The immediate value.
    uint8_t c = (state.C) ? 1 : 0; // @desc The carry flag as a binary value.
    uint16_t result = a + (~b + 1) + 0xFF + c; // @action Perform subtraction using two's complement.

    char a_prop = a & 0x80; // @temp Sign bit of A.
    char b_prop = (~b + 1) & 0x80; // @temp Sign bit of B (inverted).
    char c_prop = (result & 0x80); // @temp Sign bit of the result.

    state.C = (result > 255); // @flag Set carry flag if no underflow occurred.
    state.Z = (result == 0); // @flag Set zero flag if result is zero.
    state.V = ((c_prop ^ a_prop) & (c_prop ^ b_prop)) ? 1 : 0; // @flag Detect signed overflow.
    state.S = (result & 0x80) != 0; // @flag Set sign flag based on MSB.
    A = (uint8_t)(result & 0xFF); // @action Store lower 8 bits in A.
}

/**
 * @brief Emulates the CMP (Compare Accumulator) instruction.
 * 
 * @desc The CMP instruction compares the accumulator (A) with an immediate value (immval)
 *       by performing a subtraction and updating the processor's flags without changing A.
 * 
 * @steps
 * 1. Subtract the immediate value from the accumulator.
 * 2. Update the carry (C) flag if A is greater than or equal to immval.
 * 3. Update the zero (Z) flag if A equals immval.
 * 4. Update the sign (S) flag based on the MSB of the result.
 */
void CMP() {
    state.C = (A >= immval); // @flag Set carry flag if A >= immval.
    state.Z = (A == immval); // @flag Set zero flag if A equals immval.
    uint8_t calculation = (A - immval) & 0xFF; // @temp Perform subtraction.
    state.S = (calculation & 0x80) != 0; // @flag Set sign flag based on MSB of the result.
}

/**
 * @brief Emulates the CPX (Compare X Register) instruction.
 * 
 * @desc The CPX instruction compares the X register with an immediate value (immval)
 *       by performing a subtraction and updating the processor's flags without changing X.
 * 
 * @steps
 * 1. Subtract the immediate value from the X register.
 * 2. Update the carry (C) flag if X is greater than or equal to immval.
 * 3. Update the zero (Z) flag if X equals immval.
 * 4. Update the sign (S) flag based on the MSB of the result.
 */
void CPX() {
    state.C = (X >= immval); // @flag Set carry flag if X >= immval.
    state.Z = (X == immval); // @flag Set zero flag if X equals immval.
    uint8_t calculation = (X - immval) & 0xFF; // @temp Perform subtraction.
    state.S = (calculation & 0x80) != 0; // @flag Set sign flag based on MSB of the result.
}

/**
 * @brief Emulates the CPY (Compare Y Register) instruction.
 * 
 * @desc The CPY instruction compares the Y register with an immediate value (immval)
 *       by performing a subtraction and updating the processor's flags without changing Y.
 * 
 * @steps
 * 1. Subtract the immediate value from the Y register.
 * 2. Update the carry (C) flag if Y is greater than or equal to immval.
 * 3. Update the zero (Z) flag if Y equals immval.
 * 4. Update the sign (S) flag based on the MSB of the result.
 */
void CPY() {
    state.C = (Y >= immval); // @flag Set carry flag if Y >= immval.
    state.Z = (Y == immval); // @flag Set zero flag if Y equals immval.
    uint8_t calculation = (Y - immval) & 0xFF; // @temp Perform subtraction.
    state.S = (calculation & 0x80) != 0; // @flag Set sign flag based on MSB of the result.
}


/**
 * @brief Emulates the INC (Increment Memory) instruction.
 * 
 * @desc The INC instruction increments the value at a memory location by 1.
 *       The zero (Z) and sign (S) flags are updated based on the result.
 * 
 * @steps
 * 1. Increment the value in the target memory location.
 * 2. Update the zero flag if the result is zero.
 * 3. Update the sign flag based on the most significant bit (MSB).
 * 4. Write the incremented value back to memory.
 */
void INC() {
    immval++; // @action Increment the value in memory.
    state.Z = (immval == 0); // @flag Update the zero flag.
    state.S = (immval & 0x80) != 0; // @flag Update the sign flag based on MSB.
    cpu_write(abs_addr, immval); // @action Write the result back to memory.
}

/**
 * @brief Emulates the INX (Increment X Register) instruction.
 * 
 * @desc The INX instruction increments the X register by 1. 
 *       The zero (Z) and sign (S) flags are updated based on the result.
 * 
 * @steps
 * 1. Increment the X register.
 * 2. Update the zero flag if the result is zero.
 * 3. Update the sign flag based on the MSB.
 */
void INX() {
    X++; // @action Increment the X register.
    state.Z = (X == 0); // @flag Update the zero flag.
    state.S = (X & 0x80) != 0; // @flag Update the sign flag based on MSB.
}

/**
 * @brief Emulates the INY (Increment Y Register) instruction.
 * 
 * @desc The INY instruction increments the Y register by 1. 
 *       The zero (Z) and sign (S) flags are updated based on the result.
 * 
 * @steps
 * 1. Increment the Y register.
 * 2. Update the zero flag if the result is zero.
 * 3. Update the sign flag based on the MSB.
 */
void INY() {
    Y++; // @action Increment the Y register.
    state.Z = (Y == 0); // @flag Update the zero flag.
    state.S = (Y & 0x80) != 0; // @flag Update the sign flag based on MSB.
}

/**
 * @brief Emulates the DEC (Decrement Memory) instruction.
 * 
 * @desc The DEC instruction decrements the value at a memory location by 1.
 *       The zero (Z) and sign (S) flags are updated based on the result.
 * 
 * @steps
 * 1. Decrement the value in the target memory location.
 * 2. Update the zero flag if the result is zero.
 * 3. Update the sign flag based on the MSB.
 * 4. Write the decremented value back to memory.
 */
void DEC() {
    immval--; // @action Decrement the value in memory.
    state.Z = (immval == 0); // @flag Update the zero flag.
    state.S = (immval & 0x80) != 0; // @flag Update the sign flag based on MSB.
    cpu_write(abs_addr, immval); // @action Write the result back to memory.
}

/**
 * @brief Emulates the DEX (Decrement X Register) instruction.
 * 
 * @desc The DEX instruction decrements the X register by 1. 
 *       The zero (Z) and sign (S) flags are updated based on the result.
 * 
 * @steps
 * 1. Decrement the X register.
 * 2. Update the zero flag if the result is zero.
 * 3. Update the sign flag based on the MSB.
 */
void DEX() {
    X--; // @action Decrement the X register.
    state.Z = (X == 0); // @flag Update the zero flag.
    state.S = (X & 0x80) != 0; // @flag Update the sign flag based on MSB.
}

/**
 * @brief Emulates the DEY (Decrement Y Register) instruction.
 * 
 * @desc The DEY instruction decrements the Y register by 1. 
 *       The zero (Z) and sign (S) flags are updated based on the result.
 * 
 * @steps
 * 1. Decrement the Y register.
 * 2. Update the zero flag if the result is zero.
 * 3. Update the sign flag based on the MSB.
 */
void DEY() {
    Y--; // @action Decrement the Y register.
    state.Z = (Y == 0); // @flag Update the zero flag.
    state.S = (Y & 0x80) != 0; // @flag Update the sign flag based on MSB.
}


/**
 * @brief Emulates the ASL (Arithmetic Shift Left) instruction.
 * 
 * @desc The ASL instruction shifts the bits of a memory location or accumulator 
 *       one bit to the left. The carry flag (C) is set to the value of the 
 *       shifted-out bit. The zero (Z) and sign (S) flags are updated accordingly.
 * 
 * @steps
 * 1. Determine the target (accumulator or memory).
 * 2. Set the carry flag to the high bit of the target.
 * 3. Perform the left shift operation.
 * 4. Update the zero (Z) and sign (S) flags.
 * 5. Write the result back to memory if applicable.
 */
void ASL() {
    uint8_t* ptr = (opcode == 0x0A) ? &A : &immval; // @desc Choose the target (accumulator or memory).
    state.C = (*ptr & 0x80) != 0; // @action Set the carry flag to the value of the high bit.
    *ptr = *ptr << 1; // @action Perform the left shift.
    state.S = (*ptr & 0x80) != 0; // @flag Update the sign flag based on the high bit.
    state.Z = (*ptr == 0); // @flag Update the zero flag if the result is zero.
    if (opcode != 0x0A) cpu_write(abs_addr, immval); // @action Write the result to memory if applicable.
}

/**
 * @brief Emulates the LSR (Logical Shift Right) instruction.
 * 
 * @desc The LSR instruction shifts the bits of a memory location or accumulator 
 *       one bit to the right. The carry flag (C) is set to the value of the 
 *       shifted-out bit. The zero (Z) and sign (S) flags are updated accordingly.
 * 
 * @steps
 * 1. Determine the target (accumulator or memory).
 * 2. Set the carry flag to the low bit of the target.
 * 3. Perform the right shift operation.
 * 4. Update the zero (Z) and sign (S) flags.
 * 5. Write the result back to memory if applicable.
 */
void LSR() {
    uint8_t* ptr = (opcode == 0x4A) ? &A : &immval; // @desc Choose the target (accumulator or memory).
    state.C = (*ptr & 0x1) != 0; // @action Set the carry flag to the value of the low bit.
    *ptr = *ptr >> 1; // @action Perform the right shift.
    state.Z = (*ptr == 0); // @flag Update the zero flag if the result is zero.
    state.S = (*ptr & 0x80) != 0; // @flag Update the sign flag based on the high bit.
    if (opcode != 0x4A) cpu_write(abs_addr, immval); // @action Write the result to memory if applicable.
}

/**
 * @brief Emulates the ROL (Rotate Left) instruction.
 * 
 * @desc The ROL instruction rotates the bits of a memory location or accumulator 
 *       one bit to the left, including the carry flag as the new least significant bit (LSB).
 * 
 * @steps
 * 1. Determine the target (accumulator or memory).
 * 2. Save the carry flag as a temporary bit.
 * 3. Set the carry flag to the high bit of the target.
 * 4. Perform the left shift and include the temporary bit as the LSB.
 * 5. Update the zero (Z) and sign (S) flags.
 * 6. Write the result back to memory if applicable.
 */
void ROL() {
    uint8_t* ptr = (opcode == 0x2A) ? &A : &immval; // @desc Choose the target (accumulator or memory).
    uint8_t temp = (state.C != 0); // @temp Save the current carry flag.
    state.C = (*ptr & 0x80) ? 1 : 0; // @action Set the carry flag to the value of the high bit.
    *ptr = (*ptr << 1) & 0xFE; // @action Perform the left shift and clear the LSB.
    *ptr = *ptr | temp; // @action Insert the temporary carry flag as the new LSB.
    state.Z = (*ptr == 0); // @flag Update the zero flag if the result is zero.
    state.S = (*ptr & 0x80) != 0; // @flag Update the sign flag based on the high bit.
    if (opcode != 0x2A) cpu_write(abs_addr, immval); // @action Write the result to memory if applicable.
}

/**
 * @brief Emulates the ROR (Rotate Right) instruction.
 * 
 * @desc The ROR instruction rotates the bits of a memory location or accumulator 
 *       one bit to the right, including the carry flag as the new most significant bit (MSB).
 * 
 * @steps
 * 1. Determine the target (accumulator or memory).
 * 2. Save the carry flag as a temporary bit.
 * 3. Set the carry flag to the low bit of the target.
 * 4. Perform the right shift and include the temporary bit as the MSB.
 * 5. Update the zero (Z) and sign (S) flags.
 * 6. Write the result back to memory if applicable.
 */
void ROR() {
    uint8_t* ptr = (opcode == 0x6A) ? &A : &immval; // @desc Choose the target (accumulator or memory).
    uint8_t temp = (state.C != 0) ? 0x80 : 0; // @temp Save the carry flag as the new MSB.
    state.C = (*ptr & 0x1) ? 1 : 0; // @action Set the carry flag to the value of the low bit.
    *ptr = (*ptr >> 1) & 0x7F; // @action Perform the right shift and clear the MSB.
    *ptr = *ptr | temp; // @action Insert the temporary carry flag as the new MSB.
    state.Z = (*ptr == 0); // @flag Update the zero flag if the result is zero.
    state.S = (*ptr & 0x80) != 0; // @flag Update the sign flag based on the high bit.
    if (opcode != 0x6A) cpu_write(abs_addr, immval); // @action Write the result to memory if applicable.
}


/**
 * @brief Emulates the JMP (Jump) instruction.
 * 
 * @desc The JMP instruction sets the program counter (PC) to a specified 
 *       absolute address (abs_addr). This is used for unconditional branching.
 */
void JMP() {
    PC = abs_addr; // @action Set PC to the absolute address
}

/**
 * @brief Emulates the JSR (Jump to Subroutine) instruction.
 * 
 * @desc The JSR instruction saves the address of the next instruction 
 *       (PC - 1) on the stack and then sets the PC to an absolute address.
 *       This is used for subroutine calls, allowing the program to return later.
 * 
 * @steps
 * 1. Calculate the return address (PC - 1).
 * 2. Push the high byte of the return address onto the stack.
 * 3. Push the low byte of the return address onto the stack.
 * 4. Set PC to the absolute address (abs_addr).
 */
void JSR() {
    uint16_t temp = PC - 1; // @desc Store the return address
    uint8_t lo_byte = temp & 0xFF; // @desc Extract the low byte of the return address
    uint8_t hi_byte = (temp >> 8) & 0xFF; // @desc Extract the high byte of the return address

    cpu_write(SP, hi_byte); // @action Push the high byte onto the stack
    SP--; // @action Decrement the stack pointer
    cpu_write(SP, lo_byte); // @action Push the low byte onto the stack
    SP--; // @action Decrement the stack pointer

    PC = abs_addr; // @action Set PC to the absolute address
}

/**
 * @brief Emulates the RTS (Return from Subroutine) instruction.
 * 
 * @desc The RTS instruction pops the return address from the stack and 
 *       increments it by 1, then sets the PC to this incremented address.
 *       This is used to return from subroutine calls.
 * 
 * @steps
 * 1. Increment the stack pointer.
 * 2. Read the low byte of the return address from the stack.
 * 3. Increment the stack pointer.
 * 4. Read the high byte of the return address from the stack.
 * 5. Combine the high and low bytes into the full address.
 * 6. Increment the address by 1 and set it as the new PC.
 */
void RTS() {
    SP++; // @action Increment the stack pointer
    uint8_t lo; // @temp Placeholder for the low byte of the return address
    uint16_t hi; // @temp Placeholder for the high byte of the return address

    cpu_read(SP, &lo); // @action Read the low byte from the stack
    SP++; // @action Increment the stack pointer
    cpu_read(SP, (uint8_t*)&hi); // @action Read the high byte from the stack

    uint16_t temp = (hi << 8) | lo; // @desc Combine high and low bytes into a full address
    PC = temp + 1; // @action Set PC to the incremented return address
}


/**
 * @brief branch if the carry flag is clear.
 */
void BCC(){
  if(!state.C){
    cycles += 1;
    uint16_t temp = PC + (int8_t)rel_addr;
    if((temp & 0xFF00) != (PC & 0xFF00)){
      cycles += 2;
    }
    PC = temp;
  }
}

/**
 * @brief branch if the carry flag is set. 
 */
void BCS(){
  if(state.C){
    cycles += 1;
    uint16_t temp = PC + (int8_t)rel_addr;
    if((temp & 0xFF00) != (PC & 0xFF00)){
      cycles += 2;
    }
    PC = temp;
  }
}

/**
 *  @brief branch if the zero flag is set.
 */
void BEQ(){
  if(state.Z){
    cycles += 1;
    uint16_t temp = PC + (int8_t)rel_addr;
    if((temp & 0xFF00) != (PC & 0xFF00)){
      cycles += 2;
    }
    PC = temp;
  }
}

/**
 * @brief branches if the signed flag is set. 
 */
void BMI(){
  if(state.S){
    cycles += 1;
    uint16_t temp = PC + (int8_t)rel_addr;
    if((temp & 0xFF00) != (PC & 0xFF00)){
      cycles += 2;
    }
    PC = temp;
  }
}

/**
 * @brief branches if the Zero flag is clear
 */
void BNE(){
  if(!state.Z){
    cycles += 1;
    uint16_t temp = PC + (int8_t)rel_addr;
    if((temp & 0xFF00) != (PC & 0xFF00)){
      cycles += 2;
    }
    PC = temp;
  }
}

/**
 * @brief branches if the Signed flag is clear
 */
void BPL(){
  if(!state.S){
    cycles += 1;
    uint16_t temp = PC + (int8_t)rel_addr;
    if((temp & 0xFF00) != (PC & 0xFF00)){
      cycles += 2;
    }
    PC = temp;
  }
}

/**
 * @brief branches if the Overflag is clear
 */
void BVC(){
  if(!state.V){
    cycles += 1;
    uint16_t temp = PC + (int8_t)rel_addr;
    if((temp & 0xFF00) != (PC & 0xFF00)){
      cycles += 2;
    }
    PC = temp;
  }
}

/**
 * @brief branches if the Overflow Flag is set
 */
void BVS(){
  if(state.V){
    cycles += 1;
    uint16_t temp = PC + (int8_t)rel_addr;
    if((temp & 0xFF00) != (PC & 0xFF00)){
      cycles += 2;
    }
    PC = temp;
  }
}

/**
 * @brief clears the carry flag
 */
void CLC(){
  state.C = 0;
}

/**
 * @brief Clears the Decimal flag
 */
void CLD(){
  state.D = 0;
}

/**
 * @brief clears the Interrupt flag
 */
void CLI(){
  state.I = 0;
}

/**
 * @brief clears the overflow flag
 */
void CLV(){
  state.V = 0;
}

/**
 * @brief sets the carry flag
 */
void SEC(){
  state.C = 1;
}

/**
 * @brief Sets the decimal flag
 */
void SED(){
  state.D = 1;
}

/**
 * @brief Sets the Interrupt flag
 */
void SEI(){
  state.I = 1;
}

/**
 * @brief Break Interrupt Instruction
 * @details Manually causes an interrupt.
 */
void BRK(){
  uint16_t temp = PC + 1;
  uint8_t hi = temp >> 8;
  uint8_t lo = temp & 0xFF;
  cpu_write(SP,hi);
  SP--;
  cpu_write(SP,lo);
  SP--;
  cpu_write(SP,state.raw);
  SP--;
  cpu_read(0xFFFF,&hi);
  cpu_read(0xFFFE,&lo);
  PC = ((uint16_t)hi << 8) | lo;
  state.B = 1;
}

/**
 * Performs nothing
 */
void NOP(){
  return;
}

/**
 * @brief Return from Interrupt
 * @details The RTI instruction is used at the end of an interrupt processing 
 * routine. It pulls the processor flags from the stack followed by the
 * program counter.
 */
void RTI(){
  uint8_t lo;
  uint16_t hi;
  SP++;
  cpu_read(SP,&state.raw);
  SP++;
  cpu_read(SP,&lo);
  SP++;
  cpu_read(SP,(uint8_t*)&hi);
  PC = ((uint16_t)hi << 8) | lo;
}