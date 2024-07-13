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
  lookuptable[0x29] = {"AND {IMM}", &CPU::IMM, &CPU::AND, 2};
  lookuptable[0x25] = {"AND {ZP0}", &CPU::ZP0, &CPU::AND, 3};
  lookuptable[0x35] = {"AND {ZPX}", &CPU::ZPX, &CPU::AND, 4};
  lookuptable[0x2D] = {"AND {ABS}", &CPU::ABS, &CPU::AND, 4};
  lookuptable[0x3D] = {"AND {ABX}", &CPU::ABX, &CPU::AND, 4};
  lookuptable[0x39] = {"AND {ABY}", &CPU::ABY, &CPU::AND, 4};
  lookuptable[0x21] = {"AND {IDX}", &CPU::IDX, &CPU::AND, 6};
  lookuptable[0x31] = {"AND {IDY}", &CPU::IDY, &CPU::AND, 5};
  lookuptable[0x49] = {"EOR {IMM}", &CPU::IMM, &CPU::EOR, 2};
  lookuptable[0x45] = {"EOR {ZP0}", &CPU::ZP0, &CPU::EOR, 3};
  lookuptable[0x55] = {"EOR {ZPX}", &CPU::ZPX, &CPU::EOR, 4};
  lookuptable[0x4D] = {"EOR {ABS}", &CPU::ABS, &CPU::EOR, 4};
  lookuptable[0x5D] = {"EOR {ABX}", &CPU::ABX, &CPU::EOR, 4};
  lookuptable[0x59] = {"EOR {ABY}", &CPU::ABY, &CPU::EOR, 4};
  lookuptable[0x41] = {"EOR {IDX}", &CPU::IDX, &CPU::EOR, 6};
  lookuptable[0x51] = {"EOR {IDY}", &CPU::IDY, &CPU::EOR, 5};
  lookuptable[0x09] = {"ORA {IMM}", &CPU::IMM, &CPU::ORA, 2};
  lookuptable[0x05] = {"ORA {ZP0}", &CPU::ZP0, &CPU::ORA, 3};
  lookuptable[0x15] = {"ORA {ZPX}", &CPU::ZPX, &CPU::ORA, 4};

  populate1();
  populate2();
  illegalops();
  reset();
}
void CPU::illegalops(){
  lookuptable[0xEB] = {"ILL: SBC {IMM}", &CPU::IMM, &CPU::SBC, 2};
}
void CPU::populate2() {
  lookuptable[0x0A] = {"ASL {ACC}", &CPU::ACC, &CPU::ASL, 2};
  lookuptable[0x06] = {"ASL {ZP0}", &CPU::ZP0, &CPU::ASL, 5};
  lookuptable[0x16] = {"ASL {ZPX}", &CPU::ZPX, &CPU::ASL, 6};
  lookuptable[0x0E] = {"ASL {ABS}", &CPU::ABS, &CPU::ASL, 6};
  lookuptable[0x1E] = {"ASL {ABX}", &CPU::ABX, &CPU::ASL, 7};

  lookuptable[0x4A] = {"LSR {ACC}", &CPU::ACC, &CPU::LSR, 2};
  lookuptable[0x46] = {"LSR {ZP0}", &CPU::ZP0, &CPU::LSR, 5};
  lookuptable[0x56] = {"LSR {ZPX}", &CPU::ZPX, &CPU::LSR, 6};
  lookuptable[0x4E] = {"LSR {ABS}", &CPU::ABS, &CPU::LSR, 6};
  lookuptable[0x5E] = {"LSR {ABX}", &CPU::ABX, &CPU::LSR, 7};

  lookuptable[0x2A] = {"ROL {ACC}", &CPU::ACC, &CPU::ROL, 2};
  lookuptable[0x26] = {"ROL {ZP0}", &CPU::ZP0, &CPU::ROL, 5};
  lookuptable[0x36] = {"ROL {ZPX}", &CPU::ZPX, &CPU::ROL, 6};
  lookuptable[0x2E] = {"ROL {ABS}", &CPU::ABS, &CPU::ROL, 6};
  lookuptable[0x3E] = {"ROL {ABX}", &CPU::ABX, &CPU::ROL, 7};

  lookuptable[0x6A] = {"ROR {ACC}", &CPU::ACC, &CPU::ROR, 2};
  lookuptable[0x66] = {"ROR {ZP0}", &CPU::ZP0, &CPU::ROR, 5};
  lookuptable[0x76] = {"ROR {ZPX}", &CPU::ZPX, &CPU::ROR, 6};
  lookuptable[0x6E] = {"ROR {ABS}", &CPU::ABS, &CPU::ROR, 6};
  lookuptable[0x7E] = {"ROR {ABX}", &CPU::ABX, &CPU::ROR, 7};
  lookuptable[0x4C] = {"JMP {ABS}", &CPU::ABS, &CPU::JMP, 3};
  lookuptable[0x6C] = {"JMP {IND}", &CPU::IND, &CPU::JMP, 5};
  lookuptable[0x20] = {"JSR {ABS}", &CPU::ABS, &CPU::JSR, 6};
  lookuptable[0x60] = {"RTS {IMP}", &CPU::IMP, &CPU::RTS, 6};

  lookuptable[0x90] = {"BCC {REL}", &CPU::REL, &CPU::BCC, 2};
  lookuptable[0xB0] = {"BCS {REL}", &CPU::REL, &CPU::BCS, 2};
  lookuptable[0xF0] = {"BEQ {REL}", &CPU::REL, &CPU::BEQ, 2};
  lookuptable[0x30] = {"BMI {REL}", &CPU::REL, &CPU::BMI, 2};
  lookuptable[0xD0] = {"BNE {REL}", &CPU::REL, &CPU::BNE, 2};
  lookuptable[0x10] = {"BPL {REL}", &CPU::REL, &CPU::BPL, 2};
  lookuptable[0x50] = {"BVC {REL}", &CPU::REL, &CPU::BVC, 2};
  lookuptable[0x70] = {"BVS {REL}", &CPU::REL, &CPU::BVS, 2};
  lookuptable[0x18] = {"CLC {IMP}", &CPU::IMP, &CPU::CLC, 2};
  lookuptable[0xD8] = {"CLD {IMP}", &CPU::IMP, &CPU::CLD, 2};
  lookuptable[0x58] = {"CLI {IMP}", &CPU::IMP, &CPU::CLI, 2};
  lookuptable[0xB8] = {"CLV {IMP}", &CPU::IMP, &CPU::CLV, 2};
  lookuptable[0x38] = {"SEC {IMP}", &CPU::IMP, &CPU::SEC, 2};
  lookuptable[0xF8] = {"SED {IMP}", &CPU::IMP, &CPU::SED, 2};
  lookuptable[0x78] = {"SEI {IMP}", &CPU::IMP, &CPU::SEI, 2};
  lookuptable[0x00] = {"BRK {IMP}", &CPU::IMP, &CPU::SEI, 7};
  lookuptable[0xEA] = {"NOP {IMP}", &CPU::IMP, &CPU::NOP, 2};
  lookuptable[0x40] = {"RTI {IMP}", &CPU::IMP, &CPU::RTI, 6};
  
}
void CPU::populate1() {
  lookuptable[0x0D] = {"ORA {ABS}", &CPU::ABS, &CPU::ORA, 4};
  lookuptable[0x1D] = {"ORA {ABX}", &CPU::ABX, &CPU::ORA, 4};
  lookuptable[0x19] = {"ORA {ABY}", &CPU::ABY, &CPU::ORA, 4};
  lookuptable[0x01] = {"ORA {IDX}", &CPU::IDX, &CPU::ORA, 6};
  lookuptable[0x11] = {"ORA {IDY}", &CPU::IDY, &CPU::ORA, 5};
  lookuptable[0x24] = {"BIT {ZP0}", &CPU::ZP0, &CPU::BIT, 3};
  lookuptable[0x2C] = {"BIT {ABS}", &CPU::ABS, &CPU::BIT, 4};
  lookuptable[0x69] = {"ADC {IMM}", &CPU::IMM, &CPU::ADC, 2};
  lookuptable[0x65] = {"ADC {ZP0}", &CPU::ZP0, &CPU::ADC, 3};
  lookuptable[0x75] = {"ADC {ZPX}", &CPU::ZPX, &CPU::ADC, 4};
  lookuptable[0x6D] = {"ADC {ABS}", &CPU::ABS, &CPU::ADC, 4};
  lookuptable[0x7D] = {"ADC {ABX}", &CPU::ABX, &CPU::ADC, 4};
  lookuptable[0x79] = {"ADC {ABY}", &CPU::ABY, &CPU::ADC, 4};
  lookuptable[0x61] = {"ADC {IDX}", &CPU::IDX, &CPU::ADC, 6};
  lookuptable[0x71] = {"ADC {IDY}", &CPU::IDY, &CPU::ADC, 5};
  lookuptable[0xE9] = {"SBC {IMM}", &CPU::IMM, &CPU::SBC, 2};
  lookuptable[0xE5] = {"SBC {ZP0}", &CPU::ZP0, &CPU::SBC, 3};
  lookuptable[0xF5] = {"SBC {ZPX}", &CPU::ZPX, &CPU::SBC, 4};
  lookuptable[0xED] = {"SBC {ABS}", &CPU::ABS, &CPU::SBC, 4};
  lookuptable[0xFD] = {"SBC {ABX}", &CPU::ABX, &CPU::SBC, 4};
  lookuptable[0xF9] = {"SBC {ABY}", &CPU::ABY, &CPU::SBC, 4};
  lookuptable[0xE1] = {"SBC {IDX}", &CPU::IDX, &CPU::SBC, 6};
  lookuptable[0xF1] = {"SBC {IDY}", &CPU::IDY, &CPU::SBC, 5};

  lookuptable[0xC9] = {"CMP {IMM}", &CPU::IMM, &CPU::CMP, 2};
  lookuptable[0xC5] = {"CMP {ZP0}", &CPU::ZP0, &CPU::CMP, 3};
  lookuptable[0xD5] = {"CMP {ZPX}", &CPU::ZPX, &CPU::CMP, 4};
  lookuptable[0xCD] = {"CMP {ABS}", &CPU::ABS, &CPU::CMP, 4};
  lookuptable[0xDD] = {"CMP {ABX}", &CPU::ABX, &CPU::CMP, 4};
  lookuptable[0xD9] = {"CMP {ABY}", &CPU::ABY, &CPU::CMP, 4};
  lookuptable[0xC1] = {"CMP {IDX}", &CPU::IDX, &CPU::CMP, 6};
  lookuptable[0xD1] = {"CMP {IDY}", &CPU::IDY, &CPU::CMP, 5};

  lookuptable[0xE0] = {"CPX {IMM}", &CPU::IMM, &CPU::CPX, 2};
  lookuptable[0xE4] = {"CPX {ZP0}", &CPU::ZP0, &CPU::CPX, 3};
  lookuptable[0xEC] = {"CPX {ABS}", &CPU::ABS, &CPU::CPX, 4};

  lookuptable[0xC0] = {"CPY {IMM}", &CPU::IMM, &CPU::CPY, 2};
  lookuptable[0xC4] = {"CPY {ZP0}", &CPU::ZP0, &CPU::CPY, 3};
  lookuptable[0xCC] = {"CPY {ABS}", &CPU::ABS, &CPU::CPY, 4};

  lookuptable[0xE6] = {"INC {ZP0}", &CPU::ZP0, &CPU::INC, 5};
  lookuptable[0xF6] = {"INC {ZPX}", &CPU::ZPX, &CPU::INC, 6};
  lookuptable[0xEE] = {"INC {ABS}", &CPU::ABS, &CPU::INC, 6};
  lookuptable[0xFE] = {"INC {ABX}", &CPU::ABX, &CPU::INC, 7};

  lookuptable[0xE8] = {"INX {IMP}", &CPU::IMP, &CPU::INX, 2};
  lookuptable[0xC8] = {"INY {IMP}", &CPU::IMP, &CPU::INY, 2};

  lookuptable[0xC6] = {"DEC {ZP0}", &CPU::ZP0, &CPU::DEC, 5};
  lookuptable[0xD6] = {"DEC {ZPX}", &CPU::ZPX, &CPU::DEC, 6};
  lookuptable[0xCE] = {"DEC {ABS}", &CPU::ABS, &CPU::DEC, 6};
  lookuptable[0xDE] = {"DEC {ABX}", &CPU::ABX, &CPU::DEC, 7};

  lookuptable[0xCA] = {"DEX {IMP}", &CPU::IMP, &CPU::DEX, 2};
  lookuptable[0x88] = {"DEY {IMP}", &CPU::IMP, &CPU::DEY, 2};
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

  // PC = (hi << 8) | lo;
  PC = 0x8000;
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
    if(PC > 0xA905 && PC < 0xA90D){
      std::cout<<"we here"<<std::endl;
    }
    opcode = read(PC++);
    current_instruction = lookuptable[opcode].name;
    std::cout<<current_instruction<<std::endl;
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

uint8_t CPU::AND() {
  uint8_t M = read(addr_abs);
  A = A & M;
  flag_register.flag.zero = (A == 0) ? 1 : 0;
  flag_register.flag.negative = (A & 0x80) ? 1 : 0;
  return lookuptable[opcode].cycles;
}

uint8_t CPU::EOR() {
  uint8_t M = read(addr_abs);
  A = A ^ M;
  flag_register.flag.zero = (A == 0) ? 1 : 0;
  flag_register.flag.negative = (A & 0x80) ? 1 : 0;
  return lookuptable[opcode].cycles;
}

uint8_t CPU::ORA() {
  uint8_t M = read(addr_abs);
  A = A | M;
  flag_register.flag.zero = (A == 0) ? 1 : 0;
  flag_register.flag.negative = (A & 0x80) ? 1 : 0;
  return lookuptable[opcode].cycles;
}

uint8_t CPU::BIT() {
  uint8_t M = read(addr_abs);
  uint8_t temp = A & M;
  flag_register.flag.zero = (temp == 0) ? 1 : 0;
  flag_register.flag.overflow = (temp & 0x40) ? 1 : 0;
  flag_register.flag.negative = (temp & 0x80) ? 1 : 0;
  return lookuptable[opcode].cycles;
}

//==================Arithmetic instructions================

uint8_t CPU::ADC() {
  uint8_t M = read(addr_abs);
  uint8_t C = (flag_register.flag.carry) ? 1 : 0;
  uint16_t temp = A + M + C;
  flag_register.flag.carry = (temp > 255) ? 1 : 0;
  flag_register.flag.zero = (temp == 0) ? 1 : 0;
  flag_register.flag.negative = (temp & 0x80) ? 1 : 0;
  bool overflowa =
      ((A & 0x80) == 0) && ((M & 0x80) == 0) && ((temp & 0x80) != 0);
  bool overflowb =
      ((A & 0x80) != 0) && ((M & 0x80) != 0) && ((temp & 0x80) == 0);
  flag_register.flag.overflow = (overflowa && overflowb) ? 1 : 0;
  A = temp & 0x00FF;
  return lookuptable[opcode].cycles;
}

uint8_t CPU::SBC() {
  uint8_t M = read(addr_abs);
  M = ~M;
  uint8_t C = (flag_register.flag.carry) ? 1 : 0;
  uint16_t temp = A + M + C;
  flag_register.flag.carry = (temp > 255) ? 1 : 0;
  flag_register.flag.zero = (temp == 0) ? 1 : 0;
  flag_register.flag.negative = (temp & 0x80) ? 1 : 0;
  bool overflowa =
      ((A & 0x80) == 0) && ((M & 0x80) == 0) && ((temp & 0x80) != 0);
  bool overflowb =
      ((A & 0x80) != 0) && ((M & 0x80) != 0) && ((temp & 0x80) == 0);
  flag_register.flag.overflow = (overflowa && overflowb) ? 1 : 0;
  A = temp & 0x00FF;
  return lookuptable[opcode].cycles;
}

uint8_t CPU::CMP() {
  uint8_t M = read(addr_abs);
  uint8_t temp = A - M;
  flag_register.flag.carry = (A >= M) ? 1 : 0;
  flag_register.flag.zero = (temp == 0) ? 1 : 0;
  flag_register.flag.negative = (temp & 0x80) ? 1 : 0;
  return lookuptable[opcode].cycles;
}

uint8_t CPU::CPX() {
  uint8_t M = read(addr_abs);
  uint8_t temp = X - M;
  flag_register.flag.carry = (X >= M) ? 1 : 0;
  flag_register.flag.zero = (temp == 0) ? 1 : 0;
  flag_register.flag.negative = (temp & 0x80) ? 1 : 0;
  return lookuptable[opcode].cycles;
}

uint8_t CPU::CPY() {
  uint8_t M = read(addr_abs);
  uint8_t temp = Y - M;
  flag_register.flag.carry = (Y >= M) ? 1 : 0;
  flag_register.flag.zero = (temp == 0) ? 1 : 0;
  flag_register.flag.negative = (temp & 0x80) ? 1 : 0;
  return lookuptable[opcode].cycles;
}

// increments & decrements

uint8_t CPU::INC() {
  uint8_t M = read(addr_abs);
  M++;
  write(addr_abs, M);
  flag_register.flag.zero = (M == 0) ? 1 : 0;
  flag_register.flag.negative = (M & 0x80) ? 1 : 0;
  return lookuptable[opcode].cycles;
}
uint8_t CPU::INX() {
  X++;
  flag_register.flag.zero = (X == 0) ? 1 : 0;
  flag_register.flag.negative = (X & 0x80) ? 1 : 0;
  return lookuptable[opcode].cycles;
}
uint8_t CPU::INY() {
  Y++;
  flag_register.flag.zero = (Y == 0) ? 1 : 0;
  flag_register.flag.negative = (Y & 0x80) ? 1 : 0;
  return lookuptable[opcode].cycles;
}
uint8_t CPU::DEC() {
  uint8_t M = read(addr_abs);
  M--;
  write(addr_abs, M);
  flag_register.flag.zero = (M == 0) ? 1 : 0;
  flag_register.flag.negative = (M & 0x80) ? 1 : 0;
  return lookuptable[opcode].cycles;
}
uint8_t CPU::DEX() {
  X--;
  flag_register.flag.zero = (X == 0) ? 1 : 0;
  flag_register.flag.negative = (X & 0x80) ? 1 : 0;
  return lookuptable[opcode].cycles;
}
uint8_t CPU::DEY() {
  Y--;
  flag_register.flag.zero = (Y == 0) ? 1 : 0;
  flag_register.flag.negative = (Y & 0x80) ? 1 : 0;
  return lookuptable[opcode].cycles;
}

//===================shifts=========================

uint8_t CPU::ASL() {
  if (opcode == 0x0A) {
    flag_register.flag.carry = (A & 0x80) ? 1 : 0;
    A = A << 1;
    flag_register.flag.zero = (A == 0) ? 1 : 0;
    flag_register.flag.negative = (A & 0x80) ? 1 : 0;
  } else {
    uint8_t M = read(addr_abs);
    flag_register.flag.carry = (M & 0x80) ? 1 : 0;
    M = M << 1;
    flag_register.flag.zero = (M == 0) ? 1 : 0;
    flag_register.flag.negative = (M & 0x80) ? 1 : 0;
    write(addr_abs, M);
  }
  return lookuptable[opcode].cycles;
}

uint8_t CPU::LSR() {
  if (opcode == 0x4A) {
    flag_register.flag.carry = (A & 0x01) ? 1 : 0;
    A = A >> 1;
    flag_register.flag.zero = (A == 0) ? 1 : 0;
    flag_register.flag.negative = (A & 0x80) ? 1 : 0;
  } else {
    uint8_t M = read(addr_abs);
    flag_register.flag.carry = (M & 0x01) ? 1 : 0;
    M = M >> 1;
    flag_register.flag.zero = (M == 0) ? 1 : 0;
    flag_register.flag.negative = (M & 0x80) ? 1 : 0;
    write(addr_abs, M);
  }
  return lookuptable[opcode].cycles;
}

uint8_t CPU::ROL() {
  uint8_t temp = (flag_register.flag.carry) ? 1 : 0;
  if (opcode == 0x2A) {
    flag_register.flag.carry = (A & 0x80) ? 1 : 0;
    A = A << 1;
    A |= temp;
    flag_register.flag.zero = (A == 0) ? 1 : 0;
    flag_register.flag.negative = (A & 0x80) ? 1 : 0;
  } else {
    uint8_t M = read(addr_abs);
    flag_register.flag.carry = (M & 0x80) ? 1 : 0;
    M = M << 1;
    M |= temp;
    write(addr_abs, M);
    flag_register.flag.zero = (M == 0) ? 1 : 0;
    flag_register.flag.negative = (M & 0x80) ? 1 : 0;
  }
  return lookuptable[opcode].cycles;
}

uint8_t CPU::ROR() {
  uint8_t temp = (flag_register.flag.carry) ? 0x80 : 0;
  if (opcode == 0x2A) {
    flag_register.flag.carry = (A & 0x01) ? 1 : 0;
    A = A >> 1;
    A |= temp;
    flag_register.flag.zero = (A == 0) ? 1 : 0;
    flag_register.flag.negative = (A & 0x80) ? 1 : 0;
  } else {
    uint8_t M = read(addr_abs);
    flag_register.flag.carry = (M & 0x01) ? 1 : 0;
    M = M >> 1;
    M |= temp;
    write(addr_abs, M);
    flag_register.flag.zero = (M == 0) ? 1 : 0;
    flag_register.flag.negative = (M & 0x80) ? 1 : 0;
  }
  return lookuptable[opcode].cycles;
}

uint8_t CPU::JMP() {
  PC = addr_abs;
  return lookuptable[opcode].cycles;
}

uint8_t CPU::JSR() {
  uint16_t temp = PC - 1;
  write(0x100 + SP--, (temp >> 8));
  write(0x100 + SP--, (temp & 0xFF));
  PC = addr_abs;
  return lookuptable[opcode].cycles;
}

uint8_t CPU::RTS() {
  uint8_t lo = read(0x100 + ++SP);
  uint8_t hi = read(0x100 + ++SP);
  PC = ((uint16_t)hi << 8) | lo;
  PC++;
  return lookuptable[opcode].cycles;
}

//================branches==================

uint8_t CPU::BCC() {
  uint8_t toreturn = lookuptable[opcode].cycles;
  if (flag_register.flag.carry == 0) {
    toreturn++;
    int8_t displacement = (int8_t)addr_rel;
    uint16_t temp = PC + (int16_t)displacement;
    if ((temp & 0xFF00) != (PC & 0xFF00)) {
      toreturn += 2;
    }
    PC = temp;
  }
  return toreturn;
}

uint8_t CPU::BCS() {
  uint8_t toreturn = lookuptable[opcode].cycles;
  if (flag_register.flag.carry == 1) {
    toreturn++;
    int8_t displacement = (int8_t)addr_rel;
    uint16_t temp = PC + (int16_t)displacement;
    if ((temp & 0xFF00) != (PC & 0xFF00)) {
      toreturn += 2;
    }
    PC = temp;
  }
  return toreturn;
}

uint8_t CPU::BEQ() {
  uint8_t toreturn = lookuptable[opcode].cycles;
  if (flag_register.flag.zero == 1) {
    toreturn++;
    int8_t displacement = (int8_t)addr_rel;
    uint16_t temp = PC + (int16_t)displacement;
    if ((temp & 0xFF00) != (PC & 0xFF00)) {
      toreturn += 2;
    }
    PC = temp;
  }
  return toreturn;
}

uint8_t CPU::BMI() {
  uint8_t toreturn = lookuptable[opcode].cycles;
  if (flag_register.flag.negative == 1) {
    toreturn++;
    int8_t displacement = (int8_t)addr_rel;
    uint16_t temp = PC + (int16_t)displacement;
    if ((temp & 0xFF00) != (PC & 0xFF00)) {
      toreturn += 2;
    }
    PC = temp;
  }
  return toreturn;
}

uint8_t CPU::BNE() {
  uint8_t toreturn = lookuptable[opcode].cycles;
  if (flag_register.flag.zero == 0) {
    toreturn++;
    int8_t displacement = (int8_t)addr_rel;
    uint16_t temp = PC + (int16_t)displacement;
    if ((temp & 0xFF00) != (PC & 0xFF00)) {
      toreturn += 2;
    }
    PC = temp;
  }
  return toreturn;
}

uint8_t CPU::BPL() {
  uint8_t toreturn = lookuptable[opcode].cycles;
  if (flag_register.flag.negative == 0) {
    toreturn++;
    int8_t displacement = (int8_t)addr_rel;
    uint16_t temp = PC + (int16_t)displacement;
    if ((temp & 0xFF00) != (PC & 0xFF00)) {
      toreturn += 2;
    }
    PC = temp;
  }
  return toreturn;
}

uint8_t CPU::BVS() {
  uint8_t toreturn = lookuptable[opcode].cycles;
  if (flag_register.flag.overflow == 1) {
    toreturn++;
    int8_t displacement = (int8_t)addr_rel;
    uint16_t temp = PC + (int16_t)displacement;
    if ((temp & 0xFF00) != (PC & 0xFF00)) {
      toreturn += 2;
    }
    PC = temp;
  }
  return toreturn;
}

uint8_t CPU::BVC() {
  uint8_t toreturn = lookuptable[opcode].cycles;
  if (flag_register.flag.overflow == 0) {
    toreturn++;
    int8_t displacement = (int8_t)addr_rel;
    uint16_t temp = PC + (int16_t)displacement;
    if ((temp & 0xFF00) != (PC & 0xFF00)) {
      toreturn += 2;
    }
    PC = temp;
  }
  return toreturn;
}

//=======================status flag==================

uint8_t CPU::CLC(){
  flag_register.flag.carry = 0;
  return lookuptable[opcode].cycles;
}

uint8_t CPU::CLD(){
  flag_register.flag.decimal = 0;
  return lookuptable[opcode].cycles;
}

uint8_t CPU::CLI(){
  flag_register.flag.Interrupt_disable = 0;
  return lookuptable[opcode].cycles;
}

uint8_t CPU::CLV(){
  flag_register.flag.overflow = 0;
  return lookuptable[opcode].cycles;
}

uint8_t CPU::SEC(){
  flag_register.flag.carry = 1;
  return lookuptable[opcode].cycles;
}

uint8_t CPU::SED(){
  flag_register.flag.decimal = 1;
  return lookuptable[opcode].cycles;
}

uint8_t CPU::SEI(){
  flag_register.flag.Interrupt_disable = 1;
  return lookuptable[opcode].cycles;
}

uint8_t CPU::BRK(){
  irq();
  return 0;
}

uint8_t CPU::NOP(){
  return 2;
}

uint8_t CPU::RTI(){
  flag_register.data = read(0x100 + ++SP);
  uint8_t lo = read(0x100 + ++SP);
  uint8_t hi = read(0x100 + ++SP);
  PC = ((uint16_t)hi << 8) | lo;
  return lookuptable[opcode].cycles;
}