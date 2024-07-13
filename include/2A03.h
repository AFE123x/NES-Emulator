#pragma once
#include <cstdint>
#include <memory>
#include <string>
#include <vector>
class NES;
class CPU {
public:
  CPU(NES *NESBUS);
  ~CPU();
  void tick();
  void skip();
  uint8_t read(uint16_t address);
  void write(uint16_t address, uint8_t byte);
  uint16_t PC;
  uint8_t SP;
  uint8_t A;
  uint8_t X;
  uint8_t Y;
  std::string current_instruction;
  bool memorychanged;

private:
  void reset();
  void nmi();
  void irq();
  NES *NESBUS;
  // Registers
public:
  /*
  7  bit  0
  ---- ----
  NV1B DIZC
  |||| ||||
  |||| |||+- Carry
  |||| ||+-- Zero
  |||| |+--- Interrupt Disable
  |||| +---- Decimal
  |||+------ (No CPU effect; see: the B flag)
  ||+------- (No CPU effect; always pushed as 1)
  |+-------- Overflow
  +--------- Negative
  */
  union status_register {
    struct status {
      uint8_t carry : 1;
      uint8_t zero : 1;
      uint8_t Interrupt_disable : 1;
      uint8_t decimal : 1;
      uint8_t break_command : 1;
      uint8_t unused : 1;
      uint8_t overflow : 1;
      uint8_t negative : 1;
    } flag;
    uint8_t data;
  };
  status_register flag_register;

private:
  void populate1();
  void populate2();
  void illegalops();
  // helper functions
  struct instructions_t {
    std::string name;
    uint8_t (CPU::*addr_mode)();
    uint8_t (CPU::*instruction)();
    uint8_t cycles;
  };
  std::unique_ptr<instructions_t[]> lookuptable;

  // important variables
public:
  uint8_t cycles = 0;
  uint64_t total_cycles = 0;

private:
  uint16_t addr_abs;
  uint16_t addr_rel;
  uint8_t opcode;

  // addressing modes
  uint8_t ZP0();
  uint8_t ZPX();
  uint8_t ZPY();
  uint8_t ABS();
  uint8_t ABX();
  uint8_t ABY();
  uint8_t IND();
  uint8_t IMP();
  uint8_t IMM();
  uint8_t REL();
  uint8_t IDX();
  uint8_t IDY();
  uint8_t ACC();

  // LOAD/STORE Instructions
  uint8_t LDA();
  uint8_t LDX();
  uint8_t LDY();
  uint8_t STA();
  uint8_t STX();
  uint8_t STY();

  // Register Transfers
  uint8_t TAX();
  uint8_t TAY();
  uint8_t TXA();
  uint8_t TYA();

  // stack operations
  uint8_t TSX();
  uint8_t TXS();
  uint8_t PHA();
  uint8_t PHP();
  uint8_t PLA();
  uint8_t PLP();

  // Logical
  uint8_t AND();
  uint8_t EOR();
  uint8_t ORA();
  uint8_t BIT();

  // Arithmetic
  uint8_t ADC();
  uint8_t SBC();
  uint8_t CMP();
  uint8_t CPX();
  uint8_t CPY();

  // Increments & Decrements
  uint8_t INC();
  uint8_t INX();
  uint8_t INY();
  uint8_t DEC();
  uint8_t DEX();
  uint8_t DEY();

  // Shifts
  uint8_t ASL();
  uint8_t LSR();
  uint8_t ROL();
  uint8_t ROR();

  // Jumps & Calls
  uint8_t JMP();
  uint8_t JSR();
  uint8_t RTS();

  // Branches
  uint8_t BCC();
  uint8_t BCS();
  uint8_t BEQ();
  uint8_t BMI();
  uint8_t BNE();
  uint8_t BPL();
  uint8_t BVC();
  uint8_t BVS();

  // Status Flag Changes
  uint8_t CLC();
  uint8_t CLD();
  uint8_t CLI();
  uint8_t CLV();
  uint8_t SEC();
  uint8_t SED();
  uint8_t SEI();

  // System Functions
  uint8_t BRK();
  uint8_t NOP();
  uint8_t RTI();
};
