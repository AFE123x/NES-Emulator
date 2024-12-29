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
 * Performs a exclusive or
 */
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
  state.Z = (val == 0);
  state.S = (val & 0x80) != 0;
  state.V = (val & 0x40) != 0;
}


void ADC(){
  uint8_t a = A;
  uint8_t b = immval;
  uint8_t c = (state.C) ? 1 : 0;
  uint16_t result = a + b + c;
  char a_prop, b_prop, c_prop;
  a_prop = a & 0x80;
  b_prop = b & 0x80;
  c_prop = (result & 0x80);
  state.C = (result > 255);
  state.Z = (result == 0);
  state.V = ((c_prop ^ a_prop) & (c_prop ^ b_prop)) != 0;
  state.S = (result & 0x80) != 0; 
  A = (uint8_t)(result & 0xFF);
}

void SBC(){
  uint8_t a = A;
  uint8_t b = immval;
  uint8_t c = (state.C) ? 1 : 0;
  uint16_t result = a + (~b + 1) + 0xFF + c;
  char a_prop, b_prop, c_prop;
  a_prop = a & 0x80;
  b_prop = (~b + 1) & 0x80;
  c_prop = (result & 0x80);
  state.C = (result > 255);
  state.Z = (result == 0);
  state.V = ((c_prop ^ a_prop) & (c_prop ^ b_prop)) ? 1 : 0;
  state.S = (result & 0x80) != 0;
  A = (uint8_t)(result & 0xFF);
}
void CMP(){
  state.C = (A >= immval);
  state.Z = (A == immval);
  uint8_t calculation = (A - immval) & 0xFF;
  state.S = (calculation & 0x80) != 0;
}
void CPX(){
  state.C = (X >= immval);
  state.Z = (X == immval);
  uint8_t calculation = (X - immval) & 0xFF;
  state.S = (calculation & 0x80) != 0;
}
void CPY(){
  state.C = (Y >= immval);
  state.Z = (Y == immval);
  uint8_t calculation = (Y - immval) & 0xFF;
  state.S = (calculation & 0x80) != 0;
}

void INC(){
  immval++;
  state.Z = (immval == 0);
  state.S = (immval & 0x80) != 0;
  cpu_write(abs_addr,immval);
}
void INX(){
  X++;
  state.Z = (X == 0);
  state.S = (X & 0x80) != 0;
}
void INY(){
  Y++;
  state.Z = (Y == 0);
  state.S = (Y & 0x80) != 0;
}
void DEC(){
  immval--;
  state.Z = (immval == 0);
  state.S = (immval & 0x80) != 0;
  cpu_write(abs_addr,immval);
}
void DEX(){
  X--;
  state.Z = (X == 0);
  state.S = (X & 0x80) != 0;
}
void DEY(){
  Y--;
  state.Z = (Y == 0);
  state.S = (Y & 0x80) != 0;
}

void ASL(){
  uint8_t* ptr = (opcode == 0x0A) ? &A : &immval;
  state.C = (*ptr & 0x80) != 0;
  *ptr = *ptr << 1;
  state.S = (*ptr & 0x80) != 0;
  state.Z = (*ptr == 0);
  if (opcode != 0x0A) cpu_write(abs_addr,immval);

}

void LSR(){
  uint8_t* ptr = (opcode == 0x4A) ? &A : &immval;
  state.C = (*ptr & 0x1) != 0;
  *ptr = *ptr >> 1;
  state.Z = (*ptr == 0);
  state.S = (*ptr & 0x80) != 0;
  if(opcode != 0x4A) cpu_write(abs_addr,immval);
}

void ROL(){
  uint8_t* ptr = (opcode == 0x2A) ? &A : &immval;
  uint8_t temp = (state.C != 0);
  state.C = (*ptr & 0x80) ? 1 : 0;
  *ptr = (*ptr << 1) & 0xFE;
  *ptr = *ptr | temp;
  state.Z = (*ptr == 0);
  state.S = (*ptr & 0x80) != 0;
  if(opcode != 0x2A) cpu_write(abs_addr,immval); 
}

void ROR(){
  uint8_t* ptr = (opcode == 0x6A) ? &A : &immval;
  uint8_t temp = (state.C != 0) ? 0x80 : 0;
  state.C = (*ptr & 0x1) ? 1 : 0;
  *ptr = (*ptr >> 1) & 0x7F;
  *ptr = *ptr | temp;
  state.Z = (*ptr == 0);
  state.S = (*ptr & 0x80) != 0;
  if(opcode != 0x6A) cpu_write(abs_addr,immval); 
}

void JMP(){
    PC = abs_addr;
}
void JSR(){
  uint16_t temp = PC - 1;
  uint8_t lo_byte = temp & 0xFF;
  uint8_t hi_byte = (temp >> 8) & 0xFF;
  cpu_write(SP,hi_byte);
  SP--;
  cpu_write(SP,lo_byte);
  SP--;
  PC = abs_addr;
}
void RTS(){
  SP++;
  uint8_t lo;
  uint16_t hi;
  cpu_read(SP,&lo);
  SP++;
  cpu_read(SP,(uint8_t*)&hi);
  uint16_t temp = (hi << 8) | lo;
  PC = temp + 1;
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
  cpu_Read(0xFFFE,&lo);
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
  cpu_read(SP,&hi);
  PC = ((uint16_t)hi << 8) | lo;
}