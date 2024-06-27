#include "../lib/cpu.h"
#include "../lib/bus.h"
#include <iomanip> // For std::setw and std::setfill

CPU::CPU(BUS *bus) {
  this->bus = bus;
  RESET();
  lookup = {
      {"BRK", &CPU::IMP, &CPU::BRK, 7},  {"ORA", &CPU::IDX, &CPU::ORA, 6},
      {"XXX ", &CPU::IMP, &CPU::XXX, 0}, {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"XXX", &CPU::IMP, &CPU::XXX, 0},  {"ORA", &CPU::ZP0, &CPU::ORA, 3},
      {"ASL", &CPU::ZP0, &CPU::ASL, 5},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"PHP", &CPU::IMP, &CPU::PHP, 3},  {"ORA", &CPU::IMM, &CPU::ORA, 2},
      {"ASL", &CPU::ACC, &CPU::ASL, 2},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"XXX", &CPU::IMP, &CPU::XXX, 0},  {"ORA", &CPU::ABS, &CPU::ORA, 4},
      {"ASL", &CPU::ABS, &CPU::ASL, 6},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"BPL", &CPU::REL, &CPU::BPL, 2},  {"ORA", &CPU::IDY, &CPU::ORA, 5},
      {"XXX", &CPU::IMP, &CPU::XXX, 0},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"XXX", &CPU::IMP, &CPU::XXX, 0},  {"ORA", &CPU::ZPX, &CPU::ORA, 4},
      {"ASL", &CPU::ZPX, &CPU::ASL, 6},  {"XXX ", &CPU::IMP, &CPU::XXX, 0},
      {"CLC", &CPU::IMP, &CPU::CLC, 4},  {"ORA", &CPU::ABY, &CPU::ORA, 4},
      {"XXX", &CPU::IMP, &CPU::XXX, 0},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"XXX", &CPU::IMP, &CPU::XXX, 0},  {"ORA", &CPU::ABX, &CPU::ORA, 4},
      {"ASL", &CPU::ABX, &CPU::ASL, 7},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"JSR", &CPU::ABS, &CPU::JSR, 6},  {"AND", &CPU::IDX, &CPU::AND, 6},
      {"XXX", &CPU::IMP, &CPU::XXX, 0},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"BIT", &CPU::ZP0, &CPU::BIT, 3},  {"AND", &CPU::ZP0, &CPU::AND, 2},
      {"ROL", &CPU::ZP0, &CPU::ROL, 5},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"PLP", &CPU::IMP, &CPU::PLP, 4},  {"AND", &CPU::IMM, &CPU::AND, 2},
      {"ROL", &CPU::ACC, &CPU::ROL, 2},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"BIT", &CPU::ABS, &CPU::BIT, 4},  {"AND", &CPU::ABS, &CPU::AND, 4},
      {"ROL", &CPU::ABS, &CPU::ROL, 6},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"BMI", &CPU::REL, &CPU::BMI, 2},  {"AND", &CPU::IDY, &CPU::AND, 5},
      {"XXX", &CPU::IMP, &CPU::XXX, 0},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"XXX", &CPU::IMP, &CPU::XXX, 0},  {"AND", &CPU::ZPX, &CPU::AND, 3},
      {"ROL", &CPU::ZPX, &CPU::ROL, 6},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"SEC", &CPU::IMP, &CPU::SEC, 2},  {"AND", &CPU::ABY, &CPU::AND, 4},
      {"XXX", &CPU::IMP, &CPU::XXX, 0},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"XXX", &CPU::IMP, &CPU::XXX, 0},  {"AND", &CPU::ABX, &CPU::AND, 4},
      {"ROL", &CPU::ABX, &CPU::ROL, 7},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"RTI", &CPU::IMP, &CPU::RTI, 6},  {"EOR", &CPU::IDX, &CPU::EOR, 6},
      {"XXX", &CPU::IMP, &CPU::XXX, 0},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"XXX", &CPU::IMP, &CPU::XXX, 0},  {"EOR", &CPU::ZP0, &CPU::EOR, 3},
      {"LSR", &CPU::ZP0, &CPU::LSR, 5},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"PHA", &CPU::IMP, &CPU::PHA, 3},  {"EOR", &CPU::IMM, &CPU::EOR, 2},
      {"LSR", &CPU::ACC, &CPU::LSR, 2},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"JMP", &CPU::ABS, &CPU::JMP, 3},  {"EOR", &CPU::ABS, &CPU::EOR, 4},
      {"LSR", &CPU::ABS, &CPU::LSR, 6},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"BVC", &CPU::REL, &CPU::BVC, 2},  {"EOR", &CPU::IDY, &CPU::EOR, 5},
      {"XXX", &CPU::IMP, &CPU::XXX, 0},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"XXX", &CPU::IMP, &CPU::XXX, 0},  {"EOR", &CPU::ZPX, &CPU::EOR, 4},
      {"LSR", &CPU::ZPX, &CPU::LSR, 6},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"CLI", &CPU::IMP, &CPU::CLI, 2},  {"EOR", &CPU::ABY, &CPU::EOR, 4},
      {"XXX", &CPU::IMP, &CPU::XXX, 0},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"XXX", &CPU::IMP, &CPU::XXX, 0},  {"EOR", &CPU::ABX, &CPU::EOR, 4},
      {"LSR", &CPU::ABX, &CPU::LSR, 7},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"RTS", &CPU::IMP, &CPU::RTS, 6},  {"ADC", &CPU::IDX, &CPU::ADC, 6},
      {"XXX", &CPU::IMP, &CPU::XXX, 0},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"XXX", &CPU::IMP, &CPU::XXX, 0},  {"ADC", &CPU::ZP0, &CPU::ADC, 3},
      {"ROR", &CPU::ZP0, &CPU::ROR, 5},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"PLA", &CPU::IMP, &CPU::PLA, 4},  {"ADC", &CPU::IMM, &CPU::ADC, 2},
      {"ROR", &CPU::ACC, &CPU::ROR, 2},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"JMP", &CPU::IND, &CPU::JMP, 5},  {"ADC", &CPU::ABS, &CPU::ADC, 4},
      {"ROR", &CPU::ABS, &CPU::ROR, 6},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"BVS", &CPU::REL, &CPU::BVS, 2},  {"ADC", &CPU::IDY, &CPU::ADC, 5},
      {"XXX", &CPU::IMP, &CPU::XXX, 0},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"XXX", &CPU::IMP, &CPU::XXX, 0},  {"ADC", &CPU::ZPX, &CPU::ADC, 4},
      {"ROR", &CPU::ZPX, &CPU::ROR, 6},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"SEI", &CPU::IMP, &CPU::SEI, 2},  {"ADC", &CPU::ABY, &CPU::ADC, 4},
      {"XXX", &CPU::IMP, &CPU::XXX, 0},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"XXX", &CPU::IMP, &CPU::XXX, 0},  {"ADC", &CPU::ABX, &CPU::ADC, 4},
      {"ROR", &CPU::ABX, &CPU::ROR, 7},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"XXX", &CPU::IMP, &CPU::XXX, 0},  {"STA", &CPU::IDX, &CPU::STA, 6},
      {"XXX", &CPU::IMP, &CPU::XXX, 0},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"STY", &CPU::ZP0, &CPU::STY, 3},  {"STA", &CPU::ZP0, &CPU::STA, 3},
      {"STX", &CPU::ZP0, &CPU::STX, 3},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"DEY", &CPU::IMP, &CPU::DEY, 2},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"TXA", &CPU::IMP, &CPU::TXA, 2},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"STY", &CPU::ABS, &CPU::STY, 4},  {"STA", &CPU::ABS, &CPU::STA, 4},
      {"STX", &CPU::ABS, &CPU::STX, 4},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"BCC", &CPU::REL, &CPU::BCC, 2},  {"STA", &CPU::IDY, &CPU::STA, 6},
      {"XXX", &CPU::IMP, &CPU::XXX, 0},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"STY", &CPU::ZPX, &CPU::STY, 4},  {"STA", &CPU::ZPX, &CPU::STA, 4},
      {"STX", &CPU::ZPY, &CPU::STX, 4},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"TYA", &CPU::IMP, &CPU::TYA, 2},  {"STA", &CPU::ABY, &CPU::STA, 5},
      {"TXS", &CPU::IMP, &CPU::TXS, 2},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"XXX", &CPU::IMP, &CPU::XXX, 0},  {"STA", &CPU::ABX, &CPU::STA, 5},
      {"XXX", &CPU::IMP, &CPU::XXX, 0},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"LDY", &CPU::IMM, &CPU::LDY, 2},  {"LDA", &CPU::IDX, &CPU::LDA, 6},
      {"LDX", &CPU::IMM, &CPU::LDX, 2},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"LDY", &CPU::ZP0, &CPU::LDY, 3},  {"LDA", &CPU::ZP0, &CPU::LDA, 3},
      {"LDX", &CPU::ZP0, &CPU::LDX, 3},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"TAY", &CPU::IMP, &CPU::TAY, 2},  {"LDA", &CPU::IMM, &CPU::LDA, 2},
      {"TAX", &CPU::IMP, &CPU::TAX, 2},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"LDY", &CPU::ABS, &CPU::LDY, 4},  {"LDA", &CPU::ABS, &CPU::LDA, 4},
      {"LDX", &CPU::ABS, &CPU::LDX, 4},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"BCS", &CPU::REL, &CPU::BCS, 2},  {"LDA", &CPU::IDY, &CPU::LDA, 5},
      {"XXX", &CPU::IMP, &CPU::XXX, 0},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"LDY", &CPU::ZPX, &CPU::LDY, 4},  {"LDAX", &CPU::ZPX, &CPU::LDA, 4},
      {"LDX", &CPU::ZPY, &CPU::LDX, 4},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"CLV", &CPU::IMP, &CPU::CLV, 2},  {"LDA", &CPU::ABY, &CPU::LDA, 4},
      {"TSX", &CPU::IMP, &CPU::TSX, 2},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"LDY", &CPU::ABX, &CPU::LDY, 4},  {"LDA", &CPU::ABX, &CPU::LDA, 4},
      {"LDX", &CPU::ABY, &CPU::LDX, 4},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"CPY", &CPU::IMM, &CPU::CPY, 2},  {"CMP", &CPU::IDX, &CPU::CMP, 6},
      {"XXX", &CPU::IMP, &CPU::XXX, 0},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"CPY", &CPU::ZP0, &CPU::CPY, 3},  {"CMP", &CPU::ZP0, &CPU::CMP, 3},
      {"DEC", &CPU::ZP0, &CPU::DEC, 5},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"INY", &CPU::IMP, &CPU::INY, 2},  {"CMP", &CPU::IMM, &CPU::CMP, 2},
      {"DEX", &CPU::IMP, &CPU::DEX, 2},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"CPY", &CPU::ABS, &CPU::CPY, 4},  {"CMP", &CPU::ABS, &CPU::CMP, 4},
      {"DEC", &CPU::ABS, &CPU::DEC, 6},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"BNE", &CPU::REL, &CPU::BNE, 2},  {"CMP", &CPU::IDY, &CPU::CMP, 5},
      {"XXX", &CPU::IMP, &CPU::XXX, 0},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"XXX", &CPU::IMP, &CPU::XXX, 0},  {"CMP", &CPU::ZPX, &CPU::CMP, 4},
      {"DEC", &CPU::ZPX, &CPU::DEC, 6},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"CLD", &CPU::IMP, &CPU::CLD, 2},  {"CMP", &CPU::ABY, &CPU::CMP, 4},
      {"XXX", &CPU::IMP, &CPU::XXX, 0},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"XXX", &CPU::IMP, &CPU::XXX, 0},  {"CMP", &CPU::ABX, &CPU::CMP, 4},
      {"DEC", &CPU::ABX, &CPU::DEC, 7},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"CPX", &CPU::IMM, &CPU::CPX, 2},  {"SBC", &CPU::IDX, &CPU::SBC, 6},
      {"XXX", &CPU::IMP, &CPU::XXX, 0},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"CPX", &CPU::ZP0, &CPU::CPX, 3},  {"SBC", &CPU::ZP0, &CPU::SBC, 3},
      {"INC", &CPU::ZP0, &CPU::INC, 5},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"INX", &CPU::IMP, &CPU::INX, 2},  {"SBC", &CPU::IMM, &CPU::SBC, 2},
      {"NOP", &CPU::IMP, &CPU::NOP, 2},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"CPX", &CPU::ABS, &CPU::CPX, 4},  {"SBC", &CPU::ABS, &CPU::SBC, 4},
      {"INC", &CPU::ABS, &CPU::INC, 6},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"BEQ", &CPU::REL, &CPU::BEQ, 2},  {"SBC", &CPU::IDY, &CPU::SBC, 5},
      {"XXX", &CPU::IMP, &CPU::XXX, 0},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"XXX", &CPU::IMP, &CPU::XXX, 0},  {"SBC", &CPU::ZPX, &CPU::SBC, 4},
      {"INC", &CPU::ZPX, &CPU::INC, 6},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"SED", &CPU::IMP, &CPU::SED, 2},  {"SBC", &CPU::ABY, &CPU::SBC, 4},
      {"XXX", &CPU::IMP, &CPU::XXX, 0},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
      {"XXX", &CPU::IMP, &CPU::XXX, 0},  {"SBC", &CPU::ABX, &CPU::SBC, 4},
      {"INC", &CPU::ABX, &CPU::INC, 7},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
  };
}
CPU::~CPU() { std::cout << "CPU deallocated" << std::endl; }
void CPU::tick() {
  bool result = false;
  if (cycles == 0) {
    if (debug_enable) {
      result = debug();
    } else {
      log();
      if (total_cycles >= 26000)
        exit(0);
    }
    opcode = bus->cpuread(PC++);
    cycles += (this->*lookup[opcode].addr_mode)();
    (this->*lookup[opcode].instruction)();

    if (result) {
      exit(0);
    }
  }
  cycles--;
  total_cycles++;
}
uint8_t CPU::read(uint16_t address) { return bus->cpuread(address); }
void CPU::write(uint16_t address, uint8_t byte) {
  bus->cpuwrite(address, byte);
}

bool CPU::get_flag(FLAGS tflag) { return (flag & tflag) != 0; }
void CPU::set_flag(FLAGS tflag, bool state) {
  if (state) {
    flag |= tflag;
  } else {
    flag &= ~tflag;
  }
}
//============interrupts=============

/**
 * @brief Maskable Interrupts, triggered by certain memory mappers, or by the
 * BRK instruction.
 * @details Will push Program Counter and status flags to the stack, then get
 * new PC value from trap table. Will not perform interrupt if Interrupt flag is
 * enabled.
 *
 * costs 7 cycles
 */
void CPU::IRQ() {
  SP &= 0xFF;
  // checks if interrupt flag is already enabled.
  if (get_flag(Interrupt)) {
    return;
  }
  // extract upper and lower byte of program counter.
  uint8_t lo = PC & 0xFF;
  uint8_t hi = PC >> 8;
  std::cout<<static_cast<int>(SP)<<std::endl;
  // store Program Counter and flags to stack.
  write(0x100 + (uint8_t)SP--, hi);
  write(0x100 + (uint8_t)SP--, lo);
  write(0x100 + (uint8_t)SP--, flag);
  // set interrupt flag
  set_flag(Interrupt, true);
  // read new PC location
  lo = read(0xFFFE);
  hi = read(0xFFFF);
  // Assign Program Counter to located specified by trap table.
  PC = ((uint16_t)hi << 8) | lo;
  cycles = 7;
}

/**
 * @brief Non-Maskable Interrupts, usually triggered by Picture Processing Unit
 * (PPU)
 * @details Will push Program Counter and status flags to the stack, then get
 * new PC value from trap table. Will perform regardless of Interrupt flag.
 *
 * costs 7 cycles
 */
void CPU::NMI() {
  // extract upper and lower byte of program counter.
  uint8_t lo = PC & 0xFF;
  uint8_t hi = PC >> 8;

  // store Program Counter and flags to stack.
  write(0x100 + SP--, hi);
  write(0x100 + SP--, lo);
  write(0x100 + SP--, flag);
  // set interrupt flag
  set_flag(Interrupt, true);
  // read new PC location
  lo = read(0xFFFA);
  hi = read(0xFFFB);
  // Assign Program Counter to located specified by trap table.
  PC = ((uint16_t)hi << 8) | lo;
  cycles = 7;
}

/**
 * @brief Reset Interrupt, Triggered by user
 * @details Will reset all register values to default, and retrieve PC value
 * from trap table.
 *
 * costs 7 cycles
 */
void CPU::RESET() {
  SP = 0xFD;            // set stack to default
  A = X = Y = flag = 0; // reset registers
  total_cycles = 0;
  cycles = 0;
  uint8_t lo = read(0xFFFC);
  uint8_t hi = read(0xFFFD);
  // Assign Program Counter to located specified by trap table.
  PC = ((uint16_t)hi << 8) | lo;
  cycles = 7;
}

//========================addressing modes=======================

/**
 * @brief Zero page addressing:
 * @details The Zero page specifies addresses 0x0000-0x00FF. We
 * simply read only one byte from the program memory.
 * @return uint8_t, representing additional clock cycles
 */
uint8_t CPU::ZP0() {
  addr_abs = read(PC++);
  return 0;
}
/**
 * @brief Zero page addressing with X offset:
 * @details The Zero page specifies addresses 0x0000-0x00FF. We
 * simply read only one byte from the program memory, and add X
 * @return uint8_t, representing additional clock cycles
 */
uint8_t CPU::ZPX() {
  addr_abs = (read(PC++) + X) & 0xFF;
  return 0;
}

/**
 * @brief Zero page addressing with Y offset:
 * @details The Zero page specifies addresses 0x0000-0x00FF. We
 * simply read only one byte from the program memory, and add Y
 * @return uint8_t, representing additional clock cycles
 */
uint8_t CPU::ZPY() {
  addr_abs = (read(PC++) + Y) & 0xFF;
  return 0;
}

/**
 * @brief Absolute addressing mode.
 * @details We get a 2 byte address, instead of 1 like with Zero
 * page addressing. We'll simply read two bytes from program memory.
 * @return uint8_t, representing additional clock cycles
 */
uint8_t CPU::ABS() {
  uint8_t lo = read(PC++);
  uint8_t hi = read(PC++);
  addr_abs = ((uint16_t)hi << 8) | lo;
  return 0;
}

/**
 * @brief Absolute addressing mode with X offset.
 * @details We get a 2 byte address, instead of 1 like with Zero
 * page addressing. We'll simply read two bytes from program memory,
 * then add X to it.
 *
 * @details If there's a page cross, there'll be an additional clock cycle.
 * @return uint8_t, representing additional clock cycles
 */
uint8_t CPU::ABX() {
  uint8_t lo = read(PC++);
  uint8_t hi = read(PC++);
  addr_abs = (((uint16_t)hi << 8) | lo) + X;
  if ((addr_abs >> 8) != hi) {
    return 1;
  }
  return 0;
}

/**
 * @brief Absolute addressing mode with Y offset.
 * @details We get a 2 byte address, instead of 1 like with Zero
 * page addressing. We'll simply read two bytes from program memory,
 * then add Y to it.
 *
 * @details If there's a page cross, there'll be an additional clock cycle.
 * @return uint8_t, representing additional clock cycles
 */
uint8_t CPU::ABY() {
  uint8_t lo = read(PC++);
  uint8_t hi = read(PC++);
  addr_abs = (((uint16_t)hi << 8) | lo) + Y;
  if ((addr_abs >> 8) != hi) {
    return 1;
  }
  return 0;
}

/**
 * @brief The Indirect addressing mode.
 *
 * Reads a 16-bit address from memory indirectly. First, reads a pointer from
 * program memory, then reads an address from the pointer, giving us the final
 * location.
 *
 * @return uint8_t, representing additional clock cycles.
 */
uint8_t CPU::IND() {
  uint8_t lo = read(PC++);
  uint8_t hi = read(PC++);
  uint16_t temp = (((uint16_t)hi << 8) | lo);
  lo = read(temp++);
  hi = read(temp++);
  addr_abs = (((uint16_t)hi << 8) | lo);
  return 0;
}

/**
 * @brief Implied addressing mode, no additional data needed
 *
 * @return uint8_t, representing additional clock cycles
 */
uint8_t CPU::IMP() { return 0; }

/**
 * @brief Accumulator addressing mode, no additional data needed
 *
 * @return uint8_t, representing additional clock cycles.
 */
uint8_t CPU::ACC() { return 0; }

/**
 * @brief Immediate addressing mode
 *
 * @details Will simply store the PC at current location.
 * @return uint8_t, representing additional clock cycles.
 */
uint8_t CPU::IMM() {
  addr_abs = PC++;
  return 0;
}

/**
 * @brief Relative addressing mode, most commonly found in branch instructions
 *
 * @return uint8_t, representing additional clock cycles.
 */
uint8_t CPU::REL() {
  addr_rel = read(PC++);
  return 0;
}
/**
 * @brief Indexed Indirect addressing mode.
 * @details This addressing mode will read an address from program memory, and
 * add X to it. Afterwards, it'll read the final address from the newly
 * calculated address
 *
 * @return uint8_t, representing additional clockcycles.
 */
uint8_t CPU::IDX() {
  uint8_t byte = read(PC++);
  byte += X;
  uint8_t lo = read(byte++);
  uint8_t hi = read(byte++);
  addr_abs = (((uint16_t)hi << 8) | lo);
  return 0;
}

/**
 * @brief Indirect Indexed addressing mode
 * @details This addressing mode will read an address from program memory,
 * read a 2 byte address from this address, then add Y to it
 * @return uint8_t, representing additional clock cycles, 1 if page cross
 * occurs.
 */
uint8_t CPU::IDY() {
  uint8_t byte = read(PC++);
  uint8_t lo = read(byte++);
  uint8_t hi = read(byte++);
  addr_abs = (((uint16_t)hi << 8) | lo) + Y;
  if ((addr_abs >> 8) != hi) {
    return 1;
  }
  return 0;
}

//================load/store operations===================

/**
 * @brief The LDA instruction -> Stores value into Accumulator
 * @details stores value to accumulator, can set negative and
 * zero flag
 */
void CPU::LDA() {
  A = read(addr_abs);
  set_flag(Negative, (A & 0x80) ? true : false);
  set_flag(Zero, (A == 0));
  cycles += lookup[opcode].cycles;
}

/**
 * @brief Will store the value from the accumulator to memory.
 */
void CPU::STA() {
  write(addr_abs, A);
  cycles += lookup[opcode].cycles;
}
/**
 * @brief The LDX instruction -> Stores value into X register
 * @details stores value to X register, can set negative and
 * zero flag
 */
void CPU::LDX() {
  X = read(addr_abs);
  set_flag(Negative, (X & 0x80) ? true : false);
  set_flag(Zero, (X == 0));
  cycles += lookup[opcode].cycles;
}

/**
 * @brief Will store the value from the X register to memory.
 */
void CPU::STX() {
  write(addr_abs, X);
  cycles += lookup[opcode].cycles;
}

/**
 * @brief The LDY instruction -> Stores value into Y register
 * @details stores value to Y register, can set negative and
 * zero flag
 */
void CPU::LDY() {
  Y = read(addr_abs);
  set_flag(Negative, (Y & 0x80) ? true : false);
  set_flag(Zero, (Y == 0));
  cycles += lookup[opcode].cycles;
}

/**
 * @brief Will store the value from the Y register to memory.
 */
void CPU::STY() {
  write(addr_abs, Y);
  cycles += lookup[opcode].cycles;
}

/**
 * @brief Transfers data from A register to X register
 * @details flags affected: Negative and Zero flag
 */
void CPU::TAX() {
  X = A;
  set_flag(Negative, (X & 0x80) ? true : false);
  set_flag(Zero, (X == 0));
  cycles += lookup[opcode].cycles;
}

/**
 * @brief Transfer data from X register to A register
 * @details: Flags affected: Negative and Zero flag.
 */
void CPU::TXA() {
  A = X;
  set_flag(Negative, (A & 0x80) ? true : false);
  set_flag(Zero, (A == 0));
  cycles += lookup[opcode].cycles;
}

/**
 * @brief Transfer data from A register to Y register
 * @details: Flags affected: Negative and Zero flag.
 */
void CPU::TAY() {
  Y = A;
  set_flag(Negative, (Y & 0x80) ? true : false);
  set_flag(Zero, (Y == 0));
  cycles += lookup[opcode].cycles;
}

/**
 * @brief Transfer data from Y register to A register
 * @details: Flags affected: Negative and Zero flag.
 */
void CPU::TYA() {
  A = Y;
  set_flag(Negative, (A & 0x80) ? true : false);
  set_flag(Zero, (A == 0));
  cycles += lookup[opcode].cycles;
}

//===============================Stack operations=========================
/**
 * @brief Transfer SP to X register
 * @details: Flags affected: Negative and Zero flag.
 */
void CPU::TSX() {
  X = SP;
  set_flag(Negative, (X & 0x80) ? true : false);
  set_flag(Zero, (X == 0));
  cycles += lookup[opcode].cycles;
}
/**
 * @brief Transfer X register to SP
 */
void CPU::TXS() {
  SP = X;
  cycles += lookup[opcode].cycles;
}

/**
 * @brief Pushes A onto stack.
 */
void CPU::PHA() {
  write(0x100 + SP, A);
  SP--;
  cycles += lookup[opcode].cycles;
}

/**
 * @brief Pushes status registers to stack.
 */
void CPU::PHP() {
  write(0x100 + SP, flag);
  SP--;
  cycles += lookup[opcode].cycles;
}

/**
 * @brief Pops top value on stack and stores value in A.
 * @details Flags affected: Zero and Negative flag.
 */
void CPU::PLA() {
  SP++;
  A = read(0x100 + SP);
  set_flag(Negative, (A & 0x80) ? true : false);
  set_flag(Zero, (A == 0));
  cycles += lookup[opcode].cycles;
}

/**
 * @brief Pops top value on stack and stores value in status register.
 */
void CPU::PLP() {
  SP++;
  flag = read(0x100 + SP);
  cycles += lookup[opcode].cycles;
}

//=========================Logical operations==========================

/**
 * @brief performs a bitwise AND with the A register and memory.
 * @details flags affected: Zero flag and Negative flag.
 */
void CPU::AND() {
  uint8_t byte = read(addr_abs);
  A = A & byte;
  set_flag(Negative, (A & 0x80) ? true : false);
  set_flag(Zero, (A == 0));
  cycles += lookup[opcode].cycles;
}

/**
 * @brief Perform bitwise xor with A register and memory.
 * @details flags affected: Zero flag and Negative flag.
 */
void CPU::EOR() {
  uint8_t byte = read(addr_abs);
  A = A ^ byte;
  set_flag(Negative, (A & 0x80) ? true : false);
  set_flag(Zero, (A == 0));
  cycles += lookup[opcode].cycles;
}

/**
 * @brief Perform bitwise or with A register and memory.
 * @details flags affected: Zero flag and Negative flag.
 */
void CPU::ORA() {
  uint8_t byte = read(addr_abs);
  A = A ^ byte;
  set_flag(Negative, (A & 0x80) ? true : false);
  set_flag(Zero, (A == 0));
  cycles += lookup[opcode].cycles;
}

/**
 * @brief Performs a bitwise AND operation, meant to test value.
 * @details flags affected: Negative, Overflow and Zero flag.
 */
void CPU::BIT() {
  uint8_t M = read(addr_abs);
  uint8_t temp = A & M;
  set_flag(Negative, (temp & 0x80) ? true : false);
  set_flag(Overflow, (temp & 0x40) ? true : false);
  set_flag(Zero, (temp == 0));
  cycles += lookup[opcode].cycles;
}

//=================================arithmetic
// operations===========================

/**
 * @brief Converts given number to binary coded decimal equivalent
 *
 * @param number
 * @return uint16_t
 */
uint16_t CPU::BCD(uint8_t number) {
  // Extract the hundreds, tens, and ones digits
  uint8_t hundreds = (number / 100) % 10;
  uint8_t tens = (number / 10) % 10;
  uint8_t ones = number % 10;
  uint16_t bcd = (hundreds << 8) | (tens << 4) | ones;

  return bcd;
}

/**
 * @brief Add with Carry instruction
 * @details This instruction will add the accumulator + memory + carry_flag.
 * @details Flags affected: Overflow flag, Zero flag, carry flag, negative flag.
 */
void CPU::ADC() {
  uint8_t carry_bit = get_flag(Carry) ? 1 : 0;
  uint8_t byte = read(addr_abs);
  uint16_t temp = A + byte + carry_bit; // a + b  = c
  set_flag(Overflow, ((((temp & 0x80) & ~(A & 0x80) & ~(byte & 0x80)) ||
                       (~(temp & 0x80)) & (A & 0x80) & (byte & 0x80))));
  set_flag(Negative, temp & 0x80);
  set_flag(Zero, (temp & 0xFF) == 0);
  if (get_flag(Decimal)) {
    temp = BCD(A) + BCD(byte) + carry_bit;
    set_flag(Carry, temp > 99);
  } else {
    set_flag(Carry, temp > 255);
  }
  A = temp & 0xFF;
  cycles += lookup[opcode].cycles;
}

/**
 * @brief Substract with Carry
 * @details Will perform A - byte - carry_bit.
 * @details affected flags: Overflow, Negative and Zero flag.
 * @todo Implement decimal mode.
 */
void CPU::SBC() {
  uint8_t carry_bit = get_flag(Carry) ? 1 : 0;
  uint8_t byte = read(addr_abs);
  byte = ~byte;
  // uint16_t temp;
  uint16_t temp = A + byte + carry_bit; // a + b  = c
  set_flag(Overflow, ((((temp & 0x80) & ~(A & 0x80) & ~(byte & 0x80)) ||
                       (~(temp & 0x80)) & (A & 0x80) & (byte & 0x80))));
  set_flag(Negative, temp & 0x80);
  set_flag(Zero, (temp & 0xFF) == 0);
  A = temp & 0xFF;
  cycles += lookup[opcode].cycles;
}

/**
 * @brief Compare Accumulator and src
 * @details Will perform substraction, A - byte, and set the Negative, Zero and
 * Carry flag
 */
void CPU::CMP() {
  uint8_t byte = read(addr_abs);
  uint16_t temp = A - byte;
  set_flag(Negative, ((temp & 0x80) != 0));
  set_flag(Zero, temp == 0);
  set_flag(Carry, (A >= byte));
  cycles += lookup[opcode].cycles;
}

/**
 * @brief Compare X register and source.
 * @details will peform X - src, and set the Zero, negative and Carry flag based
 * on result
 */
void CPU::CPX() {
  uint8_t byte = read(addr_abs);
  uint16_t temp = X - byte;
  set_flag(Negative, ((temp & 0x80) != 0));
  set_flag(Zero, temp == 0);
  set_flag(Carry, X >= byte);
  cycles += lookup[opcode].cycles;
}

/**
 * @brief Compare Y register and source.
 * @details will peform X - src, and set the Zero, negative and Carry flag based
 * on result
 */
void CPU::CPY() {
  uint8_t byte = read(addr_abs);
  uint16_t temp = Y - byte;
  set_flag(Negative, ((temp & 0x80) != 0));
  set_flag(Zero, temp == 0);
  set_flag(Carry, Y >= byte);
  cycles += lookup[opcode].cycles;
}

/**
 * @brief Will increment byte in memory by one.
 * @details Will increment the byte by one, setting Negative, Zero flag when
 * appropriate.
 *
 */
void CPU::INC() {
  uint8_t byte = read(addr_abs);
  byte = (byte + 1) & 0xFF;
  set_flag(Negative, (byte & 0x80) ? true : false);
  set_flag(Zero, byte == 0);
  write(addr_abs, byte);
  cycles += lookup[opcode].cycles;
}

/**
 * @brief Increment the X register by one
 * @details flags affected: Zero and Negative flag.
 */
void CPU::INX() {
  X++;
  set_flag(Zero, X == 0);
  set_flag(Negative, (X & 0x80) ? true : false);
  cycles += lookup[opcode].cycles;
}

/**
 * @brief Increment the Y register by one
 * @details flags affected: Zero and Negative flag.
 */
void CPU::INY() {
  Y++;
  set_flag(Zero, Y == 0);
  set_flag(Negative, (Y & 0x80) ? true : false);
  cycles += lookup[opcode].cycles;
}
/**
 * @brief Dencrement a byte from memory by one
 * @details flags affected: Zero and Negative flag.
 */
void CPU::DEC() {
  uint8_t byte = read(addr_abs);
  byte = (byte - 1) & 0xFF;
  set_flag(Negative, (byte & 0x80) ? true : false);
  set_flag(Zero, byte == 0);
  write(addr_abs, byte);
  cycles += lookup[opcode].cycles;
}

/**
 * @brief Dencrement X register by one
 * @details flags affected: Zero and Negative flag.
 */
void CPU::DEX() {
  X--;
  set_flag(Zero, X == 0);
  set_flag(Negative, (X & 0x80) ? true : false);
  cycles += lookup[opcode].cycles;
}

/**
 * @brief Dencrement Y register by one
 * @details flags affected: Zero and Negative flag.
 */
void CPU::DEY() {
  Y--;
  set_flag(Zero, Y == 0);
  set_flag(Negative, (Y & 0x80) ? true : false);
  cycles += lookup[opcode].cycles;
}
/**
 * @brief Perform arithmetic left shift
 * @details Flags affected: Carry, Negative and Zero flag.
 */
void CPU::ASL() {
  if (opcode == 0x0A) {
    uint8_t byte = A;
    set_flag(Carry, (byte & 0x8) ? true : false);
    byte = (byte << 1) & 0xFE;
    set_flag(Negative, (byte & 0x8) ? true : false);
    set_flag(Zero, byte == 0);
    A = byte;
  } else {
    uint8_t byte = read(addr_abs);
    set_flag(Carry, (byte & 0x8) ? true : false);
    byte = (byte << 1) & 0xFE;
    set_flag(Negative, (byte & 0x8) ? true : false);
    set_flag(Zero, byte == 0);
    write(addr_abs, byte);
  }
  cycles += lookup[opcode].cycles;
}

/**
 * @brief Perform arithmetic left shift
 * @details Flags affected: Carry, Negative and Zero flag.
 */
void CPU::LSR() {
  set_flag(Negative, false);
  if (opcode == 0x4A) {
    uint8_t byte = A;
    set_flag(Carry, (byte & 0x1) ? true : false);
    byte = (byte >> 1) & 0x7F;
    set_flag(Zero, byte == 0);
    A = byte;
  } else {
    uint8_t byte = read(addr_abs);
    set_flag(Carry, (byte & 0x1) ? true : false);
    byte = (byte >> 1) & 0x7F;
    set_flag(Zero, byte == 0);
    write(addr_abs, byte);
  }
  cycles += lookup[opcode].cycles;
}

/**
 * @brief Rotates bits to left by 1, most significant bit is stored in Carry
 * flag.
 * @details Flags affected: Carry, Zero and Negative
 */
void CPU::ROL() {
  if (opcode == 0x2A) {
    uint8_t byte = A;
    uint8_t temp = A & 0x80;
    byte = (byte << 1) & 0xFE;
    byte = byte | get_flag(Carry) ? 1 : 0;
    set_flag(Carry, temp ? true : false);
    set_flag(Zero, byte == 0);
    set_flag(Negative, (byte & 0x80) ? true : false);
    A = byte;
  } else {
    uint8_t byte = read(addr_abs);
    uint8_t temp = A & 0x80;
    byte = (byte << 1) & 0xFE;
    byte = byte | get_flag(Carry) ? 1 : 0;
    set_flag(Carry, temp ? true : false);
    set_flag(Zero, byte == 0);
    set_flag(Negative, (byte & 0x80) ? true : false);
    write(addr_abs, byte);
  }
  cycles += lookup[opcode].cycles;
}

/**
 * @brief Rotates bits to right by 1, least significant bit is stored in Carry
 * flag.
 * @details Flags affected: Carry, Zero and Negative
 */
void CPU::ROR() {
  if (opcode == 0x6A) {
    uint8_t byte = A;
    uint8_t temp = A & 0x01;
    byte = (byte >> 1) & 0x7F;
    byte = byte | get_flag(Carry) ? 0x80 : 0;
    set_flag(Carry, temp ? true : false);
    set_flag(Zero, byte == 0);
    set_flag(Negative, (byte & 0x80) ? true : false);
    A = byte;
  } else {
    uint8_t byte = read(addr_abs);
    uint8_t temp = A & 0x01;
    byte = (byte << 1) & 0x7F;
    byte = byte | get_flag(Carry) ? 0x80 : 0;
    set_flag(Carry, temp ? true : false);
    set_flag(Zero, byte == 0);
    set_flag(Negative, (byte & 0x80) ? true : false);
    write(addr_abs, byte);
  }
  cycles += lookup[opcode].cycles;
}

//========================Jumps and calls===========================

/**
 * @brief Jump instruction
 * @details Will change value on PC to a different address.
 */
void CPU::JMP() {
  PC = addr_abs;
  cycles += lookup[opcode].cycles;
}

/**
 * @brief Jump to Subroutine call.
 * @details Similar to a call instruction in x86 assembly. The program
 * counter is pushed onto the stack.
 */
void CPU::JSR() {
  uint16_t temp = PC - 1;
  uint8_t hi = temp >> 8;
  uint8_t lo = temp & 0xFF;
  write(0x100 + SP--, hi);
  write(0x100 + SP--, lo);
  PC = addr_abs;
  cycles += lookup[opcode].cycles;
}

/**
 * @brief Return from subroutine
 * @details Similar to ret in x86 assembly. The previous program counter
 * is pulled from stack, and incremented by one.
 */
void CPU::RTS() {
  SP++;
  uint8_t lo = read(0x100 + SP);
  SP++;
  uint16_t hi = (read(0x100 + SP) << 8);
  PC = (hi | lo) + 1;
  cycles += lookup[opcode].cycles;
}

/**
 * @brief Branch if Carry flag is clear
 */
void CPU::BCC() {
  if (!get_flag(Carry)) {
    cycles++;
    int8_t difference = (int8_t)addr_rel;
    uint16_t newPC = PC + difference;
    if ((PC & 0xFF00) != (newPC & 0xFF00)) {
      cycles++;
    }
    PC = newPC;
  }
  cycles += lookup[opcode].cycles;
}

/**
 * @brief Branch if Carry Flag is Set.
 */
void CPU::BCS() {
  if (get_flag(Carry)) {
    cycles++;
    int8_t difference = (int8_t)addr_rel;
    uint16_t newPC = PC + difference;
    if ((PC & 0xFF00) != (newPC & 0xFF00)) {
      cycles++;
    }
    PC = newPC;
  }
  cycles += lookup[opcode].cycles;
}

/**
 * @brief Branch if Zero flag is set
 *
 */
void CPU::BEQ() {
  if (get_flag(Zero)) {
    cycles++;
    int8_t difference = (int8_t)addr_rel;
    uint16_t newPC = PC + difference;
    if ((PC & 0xFF00) != (newPC & 0xFF00)) {
      cycles++;
    }
    PC = newPC;
  }
  cycles += lookup[opcode].cycles;
}

/**
 * @brief Branch if Negative flag is set
 *
 */
void CPU::BMI() {
  if (get_flag(Negative)) {
    cycles++;
    int8_t difference = (int8_t)addr_rel;
    uint16_t newPC = PC + difference;
    if ((PC & 0xFF00) != (newPC & 0xFF00)) {
      cycles++;
    }
    PC = newPC;
  }
  cycles += lookup[opcode].cycles;
}

/**
 * @brief Branch if Zero Flag is not set
 *
 */
void CPU::BNE() {
  if (!get_flag(Zero)) {
    cycles++;
    int8_t difference = (int8_t)addr_rel;
    uint16_t newPC = PC + difference;
    if ((PC & 0xFF00) != (newPC & 0xFF00)) {
      cycles++;
    }
    PC = newPC;
  }
  cycles += lookup[opcode].cycles;
}

/**
 * @brief Branch if Negative flag is not set
 *
 */
void CPU::BPL() {
  if (!get_flag(Negative)) {
    cycles++;
    int8_t difference = (int8_t)addr_rel;
    uint16_t newPC = PC + difference;
    if ((PC & 0xFF00) != (newPC & 0xFF00)) {
      cycles++;
    }
    PC = newPC;
  }
  cycles += lookup[opcode].cycles;
}

/**
 * @brief Branch if overflow flag is not set
 *
 */
void CPU::BVC() {
  if (!get_flag(Overflow)) {
    cycles++;
    int8_t difference = (int8_t)addr_rel;
    uint16_t newPC = PC + difference;
    if ((PC & 0xFF00) != (newPC & 0xFF00)) {
      cycles++;
    }
    PC = newPC;
  }
  cycles += lookup[opcode].cycles;
}

/**
 * @brief Branch if Overflow flag is set
 *
 */
void CPU::BVS() {
  if (get_flag(Overflow)) {
    cycles++;
    int8_t difference = (int8_t)addr_rel;
    uint16_t newPC = PC + difference;
    if ((PC & 0xFF00) != (newPC & 0xFF00)) {
      cycles++;
    }
    PC = newPC;
  }
  cycles += lookup[opcode].cycles;
}

//===========status flag instructions==============

/**
 * @brief clear carry flag
 *
 */
void CPU::CLC() {
  set_flag(Carry, false);
  cycles += lookup[opcode].cycles;
}

/**
 * @brief clear decimal flag
 *
 */
void CPU::CLD() {
  set_flag(Decimal, false);
  cycles += lookup[opcode].cycles;
}

/**
 * @brief clean interrupt flag
 *
 */
void CPU::CLI() {
  set_flag(Interrupt, false);
  cycles += lookup[opcode].cycles;
}

/**
 * @brief clean overflow flag
 *
 */
void CPU::CLV() {
  set_flag(Overflow, false);
  cycles += lookup[opcode].cycles;
}

/**
 * @brief set carry flag
 *
 */
void CPU::SEC() {
  set_flag(Carry, true);
  cycles += lookup[opcode].cycles;
}

/**
 * @brief set decimal flag
 *
 */
void CPU::SED() {
  set_flag(Decimal, true);
  cycles += lookup[opcode].cycles;
}

/**
 * @brief set interrupt flag
 *
 */
void CPU::SEI() {
  set_flag(Interrupt, true);
  cycles += lookup[opcode].cycles;
}

//================system instructions==============

void CPU::BRK() {
  IRQ();
  cycles += lookup[opcode].cycles;
}
void CPU::NOP() { cycles += lookup[opcode].cycles; }
void CPU::RTI() {
  flag = read(0x100 + (++SP));
  uint8_t lo = read(0x100 + (++SP));
  uint8_t hi = read(0x100 + (++SP));
  PC = (hi << 8) | lo;
  cycles += lookup[opcode].cycles;
}

void CPU::XXX() {}

//==============dissasembler=============

/**
 * @brief This is a dissasembler, which will convert machine code into human
 * readable assembly
 *
 * @param start, specifying the address where to start
 * @param end , specifying address where to end.
 */
void CPU::dissasemble(uint16_t start, uint16_t end) {
  uint8_t opcode;
  while (start <= end) {
    opcode = read(start++);
    std::cout << std::hex << std::setw(4) << std::setfill('0')
              << static_cast<int>(start - 1) << ": " << std::hex << std::setw(2)
              << std::setfill('0') << static_cast<int>(opcode) << "  ";
    uint8_t item1;
    uint16_t item2;
    if (lookup[opcode].addr_mode == &CPU::IMM) {
      item1 = read(start++);
      std::cout << std::hex << std::setw(2) << std::setfill('0')
                << static_cast<int>(item1) << "\t\t" << lookup[opcode].name
                << " #$" << std::hex << std::setw(2) << std::setfill('0')
                << static_cast<int>(item1);
    } else if (lookup[opcode].addr_mode == &CPU::IMP) {

      std::cout << "\t\t" << lookup[opcode].name;
    } else if (lookup[opcode].addr_mode == &CPU::ZP0) {
      item1 = read(start++);
      std::cout << std::hex << std::setw(2) << std::setfill('0')
                << static_cast<int>(item1) << "\t\t" << lookup[opcode].name
                << " $" << std::hex << std::setw(2) << std::setfill('0')
                << static_cast<int>(item1);
    } else if (lookup[opcode].addr_mode == &CPU::ACC) {
      item1 = read(start++);
      std::cout << "\t\t" << lookup[opcode].name << " A";
    } else if (lookup[opcode].addr_mode == &CPU::ZPX) {
      item1 = read(start++);
      std::cout << std::hex << std::setw(2) << std::setfill('0')
                << static_cast<int>(item1) << "\t\t" << lookup[opcode].name
                << " $" << std::hex << std::setw(2) << std::setfill('0')
                << static_cast<int>(item1) << ",X";
    } else if (lookup[opcode].addr_mode == &CPU::ZPY) {
      item1 = read(start++);
      std::cout << std::hex << std::setw(2) << std::setfill('0')
                << static_cast<int>(item1) << "\t\t" << lookup[opcode].name
                << " $" << std::hex << std::setw(2) << std::setfill('0')
                << static_cast<int>(item1) << ",Y";
    } else if (lookup[opcode].addr_mode == &CPU::REL) {
      item1 = read(start++);
      std::cout << std::hex << std::setw(2) << std::setfill('0')
                << static_cast<int>(item1) << "\t\t" << lookup[opcode].name
                << " *" << static_cast<int>(item1);
    } else if (lookup[opcode].addr_mode == &CPU::ABS) {

      uint8_t lo = read(start++);
      uint8_t hi = read(start++);
      uint16_t address = ((uint16_t)hi << 8) | lo;
      std::cout << std::hex << std::setw(2) << std::setfill('0')
                << static_cast<int>(lo) << "  " << std::hex << std::setw(2)
                << std::setfill('0') << static_cast<int>(hi) << "\t"
                << lookup[opcode].name << "  $" << std::hex << std::setw(4)
                << std::setfill('0') << static_cast<int>(address);
    } else if (lookup[opcode].addr_mode == &CPU::ABX) {
      uint8_t lo = read(start++);
      uint8_t hi = read(start++);
      uint16_t address = ((uint16_t)hi << 8) | lo;
      std::cout << std::hex << std::setw(2) << std::setfill('0')
                << static_cast<int>(lo) << "  " << std::hex << std::setw(2)
                << std::setfill('0') << static_cast<int>(hi) << "\t"
                << lookup[opcode].name << "  $" << std::hex << std::setw(4)
                << std::setfill('0') << static_cast<int>(address) << ",X";
    } else if (lookup[opcode].addr_mode == &CPU::ABY) {
      uint8_t lo = read(start++);
      uint8_t hi = read(start++);
      uint16_t address = ((uint16_t)hi << 8) | lo;
      std::cout << std::hex << std::setw(2) << std::setfill('0')
                << static_cast<int>(lo) << "  " << std::hex << std::setw(2)
                << std::setfill('0') << static_cast<int>(hi) << "\t"
                << lookup[opcode].name << "  $" << std::hex << std::setw(4)
                << std::setfill('0') << static_cast<int>(address) << ",Y";
    } else if (lookup[opcode].addr_mode == &CPU::IND) {
      uint8_t lo = read(start++);
      uint8_t hi = read(start++);
      uint16_t address = ((uint16_t)hi << 8) | lo;
      std::cout << std::hex << std::setw(2) << std::setfill('0')
                << static_cast<int>(lo) << "  " << std::hex << std::setw(2)
                << std::setfill('0') << static_cast<int>(hi) << "\t\t"
                << lookup[opcode].name << "  (" << std::hex << std::setw(4)
                << std::setfill('0') << static_cast<int>(address) << ")";
    } else if (lookup[opcode].addr_mode == &CPU::IDX) {
      item1 = read(start++);
      std::cout << std::hex << std::setw(2) << std::setfill('0')
                << static_cast<int>(item1) << "\t\t" << lookup[opcode].name
                << " ($" << static_cast<int>(item1) << ",X)";
    } else if (lookup[opcode].addr_mode == &CPU::IDY) {
      item1 = read(start++);
      //($40),Y
      std::cout << std::hex << std::setw(2) << std::setfill('0')
                << static_cast<int>(item1) << "\t\t" << lookup[opcode].name
                << " ($" << static_cast<int>(item1) << "),Y";
    }
    if (debug_enable) {
      std::cout << std::endl;
    }
    else{
      std::cout<<std::endl;
    }
  }
}

void CPU::log() {
  std::cout << "A:" << std::hex << std::setw(2) << std::setfill('0')
            << static_cast<int>(A) << " X:" << static_cast<int>(X)
            << " Y:" << static_cast<int>(Y) << " SP:" << static_cast<int>(SP)
            << std::endl;
}

bool CPU::debug() {
  std::cout << "========================DEBUG=================================="
               "========"
            << std::endl;
  std::cout << "instruction: ";
  dissasemble(PC, PC);
  std::cout << "      cycles: " << static_cast<int>(total_cycles)
            << "     Cycles left: " << static_cast<int>(cycles)
            << "       FLAGS: " << (get_flag(Negative) ? "N" : "-")
            << (get_flag(Overflow) ? "V" : "-")
            << (get_flag(Unused) ? "U" : "-") << (get_flag(Break) ? "B" : "-")
            << (get_flag(Decimal) ? "D" : "-")
            << (get_flag(Interrupt) ? "I" : "-") << (get_flag(Zero) ? "Z" : "-")
            << (get_flag(Carry) ? "C" : "-") << std::endl
            << std::endl;

  std::cout << "             GENERAL PURPOSE REGISTERS" << std::endl;
  std::cout << "             A: " << static_cast<int>(A) << std::endl;
  std::cout << "             X: " << static_cast<int>(X) << std::endl;
  std::cout << "             Y: " << static_cast<int>(Y) << std::endl;
  std::cout << "                SPECIAL REGISTERS" << std::endl;
  std::cout << "            PC: " << static_cast<int>(PC) << std::endl;
  std::cout << "            SP: " << static_cast<int>(SP) << std::endl;
  std::cout << "addr 0210: " << static_cast<int>(read(0x0210)) << std::endl;
  if (debug_enable) {
    std::string decision;
  decision:
    std::cin >> decision;
    if (decision == "x/b") {
      std::cin >> decision;
      uint16_t address = strtol(decision.c_str(), NULL, 16);
      std::cout << static_cast<int>(read(address)) << std::endl;
      goto decision;
    } else if (decision == "jtil") {
      std::cin >> decision;
      jumptil = strtol(decision.c_str(), NULL, 16);
    }
    return decision == "q";
  }
  return false;
}