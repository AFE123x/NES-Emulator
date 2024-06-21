#pragma once
#include <cstdint>
#include <string>
#include <vector>
/*
condition flags
+-- N: Negative
|+-- V: Overflow
||+-- U: Unused
|||+-- B: The B Flag
||||
00000000
    ||||
    |||+-- C: Carry
    ||+-- Z: Zero
    |+-- I: Interrupt Disable
    +-- D: Decimal
*/
#define NEGATIVE_FLAG 0b10000000
#define OVERFLOW_FLAG 0b01000000
#define BREAK_FLAG 0b00010000
#define DECIMAL_FLAG 0b00001000
#define INTERRUPT_FLAG 0b00000100
#define ZERO_FLAG 0b00000010
#define CARRY_FLAG 0b00000001

class BUS;
class CPU {
public:
  CPU(BUS *bus);
  ~CPU();
  void tick();
private:
  std::string get_addressing_mode();
  // our lovely registers
  uint16_t PC;    // program counter
  uint8_t SP;     // stack pointer
  uint8_t A;      // accumulator
  uint8_t X;      // X register
  uint8_t Y;      // Y register
  uint8_t STATUS; // status register
  BUS *bus;

  // helper variables
  uint8_t cycles;
  uint32_t total_cycles;
  uint8_t opcode;
  uint16_t addr_abs;
  uint16_t addr_rel;

  // helper functions
  uint8_t BCD(uint8_t data);
  void initialize();
  bool debug();
  // status flag functions
  uint8_t get_flag(char flag);
  void set_flag(char flag, bool set);
  // instruction struct
  struct instructions_t {
    std::string name;
    uint8_t (CPU::*addressing_mode)(void) = nullptr;
    void (CPU::*instruction)(void) = nullptr;
    uint8_t clock_cycles;
  };
  std::vector<instructions_t> lookup;

  // instruction table
  // interrupt stuff
  void RESET();
  void NMI();
  void IRQ();
  // clock stuff


  // Addressing modes - will return the number of clock cycles
  uint8_t ZP();  // Zero Page addressing
  uint8_t ZPX(); // Indexed Zero Page addressing X register
  uint8_t ZPY();
  uint8_t ABS(); // Absolute addressing
  uint8_t ABX(); // Indexed Absolute addressing
  uint8_t ABY();
  uint8_t IND(); // Indirect addressing
  uint8_t IMP(); // Implied addressing
  uint8_t ACC(); // Accumulator
  uint8_t IMM(); // Immediate
  uint8_t REL(); // Relative
  uint8_t IDX(); // Indexed Indirect
  uint8_t IDY(); // Indirect Indexed

  // our lovely instructions
  void ADC();
  void AND();
  void ASL();
  void BCC();
  void BCS();
  void BEQ();
  void BIT();
  void BMI();
  void BNE();
  void BPL();
  void BRK();
  void BVC();
  void BVS();
  void CLC();
  void CLD();
  void CLI();
  void CLV();
  void CMP();
  void CPX();
  void CPY();
  void DEC();
  void DEX();
  void DEY();
  void EOR();
  void INC();
  void INX();
  void INY();
  void JMP();
  void JSR();
  void LDA();
  void LDX();
  void LDY();
  void LSR();
  void NOP();
  void ORA();
  void PHA();
  void PHP();
  void PLA();
  void PLP();
  void ROL();
  void ROR();
  void RTI();
  void RTS();
  void SBC();
  void SEC();
  void SED();
  void SEI();
  void STA();
  void STX();
  void STY();
  void TAX();
  void TAY();
  void TSX();
  void TXA();
  void TXS();
  void TYA();
  void XXX(); // for all illegal opcodes;
};
