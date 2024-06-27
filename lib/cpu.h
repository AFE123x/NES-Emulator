#pragma once
#include <cstdint>
#include <iostream>
#include <string>
#include <vector>
class BUS;
class CPU {
public:
  CPU(BUS *bus);
  ~CPU();
  void tick();
  bool debug_enable;
  uint16_t jumptil;

private:
  bool debug();
  BUS *bus;
  // read function
  uint8_t read(uint16_t address);
  void write(uint16_t address, uint8_t byte);

  // registers
  uint16_t PC; // Program Counter
  uint8_t SP;  // Stack Pointer: Range 0x100 -> 0x1FF
  uint8_t A;   // accumulator
  uint8_t X;   // X Register
  uint8_t Y;   // Y register
               // struct to help with instructions
  struct INSTRUCTIONS {
    std::string name;
    uint8_t (CPU::*addr_mode)(void) = nullptr;
    void (CPU::*instruction)(void) = nullptr;
    uint8_t cycles;
  };
  std::vector<INSTRUCTIONS> lookup;

  // status flags
  uint8_t flag;
  enum FLAGS {
    Negative = 1 << 7,
    Overflow = 1 << 6,
    Unused = 1 << 5,
    Break = 1 << 4,
    Decimal = 1 << 3,
    Interrupt = 1 << 2,
    Zero = 1 << 1,
    Carry = 1 << 0
  };

  bool get_flag(FLAGS tflag);
  void set_flag(FLAGS tflag, bool state);
  // important variables
  uint32_t total_cycles;
  uint8_t cycles;
  uint16_t addr_abs;
  uint8_t addr_rel;
  uint8_t opcode;
  // interrupts
  void IRQ();
  void NMI();
  void RESET();

  // addressing modes
  uint8_t IMP();
  uint8_t ZPX();
  uint8_t ABY();
  uint8_t ACC();
  uint8_t ZPY();
  uint8_t IND();
  uint8_t IMM();
  uint8_t ABS();
  uint8_t IDX();
  uint8_t ZP0();
  uint8_t ABX();
  uint8_t IDY();
  uint8_t REL();

  // instructions

  // load/store operations
  void LDA();
  void STA();
  void LDX();
  void STX();
  void LDY();
  void STY();

  // register transfers
  void TAX();
  void TXA();
  void TAY();
  void TYA();

  // stack operations

  void TSX();
  void TXS();
  void PHA();
  void PHP();
  void PLA();
  void PLP();

  // logical operations
  void AND();
  void EOR();
  void ORA();
  void BIT();

  // arithmetic

  void ADC();
  void SBC();
  void CMP();
  void CPX();
  void CPY();

  // increments and decrements

  void INC();
  void INX();
  void INY();
  void DEC();
  void DEX();
  void DEY();

  // shifts
  void ASL();
  void LSR();
  void ROL();
  void ROR();

  // jumps & calls

  void JMP();
  void JSR();
  void RTS();

  // branches

  void BCC();
  void BCS();
  void BEQ();
  void BMI();
  void BNE();
  void BPL();
  void BVC();
  void BVS();

  // status flag changes

  void CLC();
  void CLD();
  void CLI();
  void CLV();
  void SEC();
  void SED();
  void SEI();

  // system functions

  void BRK();
  void NOP();
  void RTI();
  void XXX();
  // tools
  uint16_t BCD(uint8_t number);
  void log();

public:
  void dissasemble(uint16_t start, uint16_t end);
};