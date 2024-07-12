/**
 * @file 2A03.cpp
 * @author Arun Felix (AFE123x on github)
 * @brief The 2A03 implementation.
 * @version 0.1
 * @date 2024-07-11
 *
 * @copyright Copyright (c) 2024
 *
 */

#include "../include/2A03.h"
#include "../include/emulator.h"
#include <iomanip>
#include <iostream>
/**
 * @brief Construct a new CPU::CPU object
 *
 * @param NESBUS, the NES bus where we can access memory.
 */
CPU::CPU(NES *NESBUS) {
  this->NESBUS = NESBUS;

  PC = addr_abs = addr_rel = 0;
  SP = A = X = Y = flag_register.data = cycles = 0;
  total_cycles = 0;
  opcode = 0;
  // lookuptable(std::make_unique<instructions_t[]>(256));
  lookuptable = std::make_unique<instructions_t[]>(256);

  // load/store instructions
  lookuptable[0xA9] = {"LDA {IMM}", &CPU::IMM, &CPU::LDA, 2};
  lookuptable[0xA5] = {"LDA {ZP}", &CPU::ZP0, &CPU::LDA, 3};
  lookuptable[0xB5] = {"LDA {ZPX}", &CPU::ZPX, &CPU::LDA, 4};
  lookuptable[0xAD] = {"LDA {ABS}", &CPU::ABS, &CPU::LDA, 4};
  lookuptable[0xBD] = {"LDA {ABX}", &CPU::ABX, &CPU::LDA, 4};
  lookuptable[0xB9] = {"LDA {ABY}", &CPU::ABY, &CPU::LDA, 4};
  lookuptable[0xA1] = {"LDA {IDX}", &CPU::IDX, &CPU::LDA, 6};
  lookuptable[0xB1] = {"LDA {IDY}", &CPU::IDY, &CPU::LDA, 5};
  lookuptable[0xA2] = {"LDX {IMM}", &CPU::IMM, &CPU::LDX, 2};
  lookuptable[0xA6] = {"LDX {ZP0}", &CPU::ZP0, &CPU::LDX, 3};
  lookuptable[0xB6] = {"LDX {ZPY}", &CPU::ZPY, &CPU::LDX, 4};
  lookuptable[0xAE] = {"LDX {ABS}", &CPU::ABS, &CPU::LDX, 4};
  lookuptable[0xBE] = {"LDX {ABY}", &CPU::ABY, &CPU::LDX, 4};
  lookuptable[0xA0] = {"LDY {IMM}", &CPU::IMM, &CPU::LDY, 2};
  lookuptable[0xA4] = {"LDY {ZP0}", &CPU::ZP0, &CPU::LDY, 3};
  lookuptable[0xB4] = {"LDY {ZPX}", &CPU::ZPX, &CPU::LDY, 4};
  lookuptable[0xAC] = {"LDY {ABS}", &CPU::ABS, &CPU::LDY, 4};
  lookuptable[0xBC] = {"LDY {ABX}", &CPU::ABX, &CPU::LDY, 4};
  lookuptable[0x85] = {"STA {ZP0}", &CPU::ZP0, &CPU::STA, 3};
  lookuptable[0x95] = {"STA {ZPX}", &CPU::ZPX, &CPU::STA, 4};
  lookuptable[0x8D] = {"STA {ABS}", &CPU::ABS, &CPU::STA, 4};
  lookuptable[0x9D] = {"STA {ABX}", &CPU::ABX, &CPU::STA, 5};
  lookuptable[0x99] = {"STA {ABY}", &CPU::ABY, &CPU::STA, 5};
  lookuptable[0x81] = {"STA {IDX}", &CPU::IDX, &CPU::STA, 6};
  lookuptable[0x91] = {"STA {IDY}", &CPU::IDY, &CPU::STA, 6};
  lookuptable[0x86] = {"STX {ZP0}", &CPU::ZP0, &CPU::STX, 3};
  lookuptable[0x96] = {"STX {ZPY}", &CPU::ZPY, &CPU::STX, 4};
  lookuptable[0x8E] = {"STX {ABS}", &CPU::ABS, &CPU::STX, 4};
  lookuptable[0x84] = {"STY {ZP0}", &CPU::ZP0, &CPU::STY, 3};
  lookuptable[0x94] = {"STY {ZPX}", &CPU::ZPX, &CPU::STY, 4};
  lookuptable[0x8C] = {"STY {ABS}", &CPU::ABS, &CPU::STY, 4};
  lookuptable[0xAA] = {"TAX {IMP}", &CPU::IMP, &CPU::TAX, 2};
  lookuptable[0xA8] = {"TAY {IMP}", &CPU::IMP, &CPU::TAY, 2};
  lookuptable[0x8A] = {"TXA {IMP}", &CPU::IMP, &CPU::TXA, 2};
  lookuptable[0x98] = {"TYA {IMP}", &CPU::IMP, &CPU::TYA, 2};
  lookuptable[0xBA] = {"TSX {IMP}", &CPU::IMP, &CPU::TSX, 2};
  lookuptable[0x9A] = {"TXS {IMP}", &CPU::IMP, &CPU::TXS, 2};
  lookuptable[0x48] = {"PHA {IMP}", &CPU::IMP, &CPU::PHA, 3};
  lookuptable[0x08] = {"PHP {IMP}", &CPU::IMP, &CPU::PHP, 3};
  lookuptable[0x68] = {"PLA {IMP}", &CPU::IMP, &CPU::PLA, 4};
  lookuptable[0x28] = {"PLP {IMP}", &CPU::IMP, &CPU::PLP, 4};
  reset();
}

/**
 * @brief Destroy the CPU::CPU object
 *
 */
CPU::~CPU() { std::cout << "CPU Deallocated" << std::endl; }

void CPU::reset() {
  current_instruction = "reset";
  uint16_t lo = read(0xFFFC);
  uint16_t hi = read(0xFFFD);
  memorychanged = true;

  PC = (hi << 8) | lo;

  A = 0;
  X = 0;
  Y = 0;
  SP = 0xFD;
  flag_register.data = 0;

  addr_rel = 0;
  addr_abs = 0;
  cycles = 8;
}

void CPU::irq() {
  if (!flag_register.flag.Interrupt_disable) {
    current_instruction = "irq";
    write(0x100 + SP--, (PC >> 8) & 0xFF);
    write(0x100 + SP--, (PC & 0xFF));
    flag_register.flag.Interrupt_disable = 1;
    flag_register.flag.unused = 1;
    flag_register.flag.break_command = 1;
    write(0x100 + SP--, flag_register.data);
    uint8_t lo = read(0xFFFE);
    uint8_t hi = read(0xFFFF);
    PC = ((uint16_t)hi << 8) | lo;
    cycles = 7;
  }
}

void CPU::nmi() {
  current_instruction = "nmi";
  write(0x100 + SP--, (PC >> 8) & 0xFF);
  write(0x100 + SP--, (PC & 0xFF));
  flag_register.flag.Interrupt_disable = 1;
  flag_register.flag.unused = 1;
  flag_register.flag.break_command = 0;
  write(0x100 + SP--, flag_register.data);
  uint8_t lo = read(0xFFFA);
  uint8_t hi = read(0xFFFB);
  PC = ((uint16_t)hi << 8) | lo;
  cycles = 8;
}
/**
 * @brief Performs one system tick.
 *
 */
void CPU::tick() {
  if (cycles == 0) {
    // NESBUS->updateregisters();
    // printState();
    opcode = read(PC++);
    current_instruction = lookuptable[opcode].name;

    uint8_t cycles1 = (this->*(lookuptable[opcode].addr_mode))();
    uint8_t cycles2 = (this->*(lookuptable[opcode].instruction))();
    cycles = cycles1 + cycles2;
  }
  cycles--;
  total_cycles++;
}

void CPU::skip() {
  while (cycles > 0) {
    tick();
  }
}

/**
 * @brief reads from the CPU bus.
 *
 * @param address we want to read
 * @return uint8_t, the byte located at the address.
 */
uint8_t CPU::read(uint16_t address) { return NESBUS->cpuread(address); }

/**
 * @brief writes to an address on the CPU bus.
 *
 * @param address
 * @param byte
 */
void CPU::write(uint16_t address, uint8_t byte) {
  if (address <= 0xFF) {
    memorychanged = true;
  }
  NESBUS->cpuwrite(address, byte);
}

//=========================addressing modes===================================

/**
 * @brief Zero Page addressing mode.
 *
 * @return uint8_t, additional clock cycles
 */
uint8_t CPU::ZP0() {
  addr_abs = read(PC++) & 0xFF;
  return 0;
}

/**
 * @brief Zero Page X addressing
 *
 * @return uint8_t, additional clock cycles
 */
uint8_t CPU::ZPX() {
  addr_abs = (read(PC++) + X) & 0xFF;
  return 0;
}
/**
 * @brief Zero Page Y addressing
 *
 * @return uint8_t uint8_t, additional clock cycles
 */
uint8_t CPU::ZPY() {
  addr_abs = (read(PC++) + Y) & 0xFF;
  return 0;
}

/**
 * @brief Absolute addressing
 *
 * @return uint8_t, additional clock cycles
 */
uint8_t CPU::ABS() {
  uint8_t lo = read(PC++);
  uint8_t hi = read(PC++);
  addr_abs = ((uint16_t)hi << 8) | lo;
  return 0;
}

/**
 * @brief Absolute addressing X
 *
 * @return uint8_t, additional clock cycles
 */
uint8_t CPU::ABX() {
  uint8_t lo = read(PC++);
  uint8_t hi = read(PC++);
  uint16_t temp = ((uint16_t)hi << 8) | lo;
  addr_abs = temp + X;
  if ((addr_abs & 0xFF00) != (temp & 0xFF00)) {
    return 1;
  }
  return 0;
}

/**
 * @brief Absolute addressing Y
 *
 * @return uint8_t, additional clock cycles
 */
uint8_t CPU::ABY() {
  uint8_t lo = read(PC++);
  uint8_t hi = read(PC++);
  uint16_t temp = ((uint16_t)hi << 8) | lo;
  addr_abs = temp + Y;
  if ((addr_abs & 0xFF00) != (temp & 0xFF00)) {
    return 1;
  }
  return 0;
}

/**
 * @brief Indirect addressing
 *
 * @return uint8_t, additional clock cycles
 */
uint8_t CPU::IND() {
  uint8_t lo = read(PC++);
  uint8_t hi = read(PC++);
  uint16_t temp = ((uint16_t)hi << 8) | lo;
  lo = read(temp++);
  hi = read(temp++);
  addr_abs = ((uint16_t)hi << 8) | lo;
  ;
  return 0;
}

/**
 * @brief Implied addressing mode
 *
 * @return uint8_t, additional clock cycles
 */
uint8_t CPU::IMP() { return 0; }

/**
 * @brief Immediate addressing
 *
 * @return uint8_t, additional clock cycles
 */
uint8_t CPU::IMM() {
  addr_abs = PC++;
  return 0;
}

/**
 * @brief Relative addressing mode
 *
 * @return uint8_t, additional clock cycles
 */
uint8_t CPU::REL() {
  addr_rel = read(PC++);
  return 0;
}

/**
 * @brief Indexed Indirect
 *
 * @return uint8_t, additional clock cycles
 */
uint8_t CPU::IDX() {
  uint8_t lo = read(PC++);
  lo += X;
  uint16_t temp = lo;
  lo = read(temp++);
  uint8_t hi = read(temp++);
  temp = ((uint16_t)hi << 8) | lo;
  addr_abs = temp;
  return 0;
}

/**
 * @brief Indirect Indexed Addressing
 *
 * @return uint8_t, additional clock cycles
 */
uint8_t CPU::IDY() {
  uint8_t lo = read(PC++);
  uint16_t temp = lo & 0xFF;
  lo = read(temp++);
  uint8_t hi = read(temp++);
  temp = ((uint16_t)hi << 8) | lo;
  addr_abs = temp + Y;
  if ((addr_abs & 0xFF00) != (temp & 0xFF00)) {
    return 1;
  }
  return 0;
}

/**
 * @brief Accumulator addressing
 *
 * @return uint8_t, additional clock cycles
 */
uint8_t CPU::ACC() { return 0; }

//=======================load and store instructions====================

uint8_t CPU::LDA() {
  A = read(addr_abs);
  flag_register.flag.negative = (A & 0x80) ? 1 : 0;
  flag_register.flag.zero = (A == 0) ? 1 : 0;
  return lookuptable[opcode].cycles;
}

uint8_t CPU::LDX() {
  X = read(addr_abs);
  flag_register.flag.negative = (X & 0x80) ? 1 : 0;
  flag_register.flag.zero = (X == 0) ? 1 : 0;
  return lookuptable[opcode].cycles;
}

uint8_t CPU::LDY() {
  Y = read(addr_abs);
  flag_register.flag.negative = (Y & 0x80) ? 1 : 0;
  flag_register.flag.zero = (Y == 0) ? 1 : 0;
  return lookuptable[opcode].cycles;
}

uint8_t CPU::STA() {
  write(addr_abs, A);
  return lookuptable[opcode].cycles;
}

uint8_t CPU::STX() {
  write(addr_abs, X);
  return lookuptable[opcode].cycles;
}

uint8_t CPU::STY() {
  write(addr_abs, Y);
  return lookuptable[opcode].cycles;
}

//===============================Register Transfers===========================

uint8_t CPU::TAX() {
  X = A;
  flag_register.flag.zero = (X == 0) ? 1 : 0;
  flag_register.flag.negative = (X & 0x80) ? 1 : 0;
  return lookuptable[opcode].cycles;
}

uint8_t CPU::TAY() {
  Y = A;
  flag_register.flag.zero = (Y == 0) ? 1 : 0;
  flag_register.flag.negative = (Y & 0x80) ? 1 : 0;
  return lookuptable[opcode].cycles;
}

uint8_t CPU::TXA() {
  A = X;
  flag_register.flag.zero = (A == 0) ? 1 : 0;
  flag_register.flag.negative = (A & 0x80) ? 1 : 0;
  return lookuptable[opcode].cycles;
}

uint8_t CPU::TYA() {
  A = Y;
  flag_register.flag.zero = (A == 0) ? 1 : 0;
  flag_register.flag.negative = (A & 0x80) ? 1 : 0;
  return lookuptable[opcode].cycles;
}

//====================stack operations=================

uint8_t CPU::TSX() {
  X = SP;
  flag_register.flag.zero = (X == 0) ? 1 : 0;
  flag_register.flag.negative = (X & 0x80) ? 1 : 0;
  return lookuptable[opcode].cycles;
}
uint8_t CPU::TXS() {
  SP = X;
  return lookuptable[opcode].cycles;
}
uint8_t CPU::PHA() {
  write(0x100 + SP--, A);
  return lookuptable[opcode].cycles;
}
uint8_t CPU::PHP() {
  write(0x100 + SP--, flag_register.data);
  return lookuptable[opcode].cycles;
}
uint8_t CPU::PLA() {
  A = read(0x100 + ++SP);
  flag_register.flag.zero = (A == 0) ? 1 : 0;
  flag_register.flag.negative = (A & 0x80) ? 1 : 0;
  return lookuptable[opcode].cycles;
}
uint8_t CPU::PLP() {

  flag_register.data = read(0x100 + ++SP);
  return lookuptable[opcode].cycles;
}

//==================logical operations===================

uint8_t CPU::AND() { return 0; }

uint8_t CPU::EOR() { return 0; }

uint8_t CPU::ORA() { return 0; }

uint8_t CPU::BIT() { return 0; }