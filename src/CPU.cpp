#include "../lib/CPU.h"
#include "../lib/BUS.h"
#include <iostream>
#include <thread>

CPU::CPU(BUS *bus) {
  this->bus = bus;
  RESET();
  lookup = {{"BRK", &CPU::IMP, &CPU::BRK, 7}, {"ORA", &CPU::IDX, &CPU::ORA, 6},
            {"XXX", &CPU::IMP, &CPU::XXX, 0}, {"XXX", &CPU::IMP, &CPU::XXX, 0},
            {"XXX", &CPU::IMP, &CPU::XXX, 0}, {"ORA", &CPU::ZP, &CPU::ORA, 3},
            {"ASL", &CPU::ZP, &CPU::ASL, 5},  {"XXX", &CPU::IMP, &CPU::XXX, 0},
            {"PHP", &CPU::IMP, &CPU::PHP, 3}, {"ORA", &CPU::IMM, &CPU::ORA, 2},
            {"ASL", &CPU::ACC, &CPU::ASL, 2}, {"XXX", &CPU::IMP, &CPU::XXX, 0},
            {"XXX", &CPU::IMP, &CPU::XXX, 0}, {"ORA", &CPU::ABS, CPU::ORA, 4},
            {"ASL", &CPU::ABS, &CPU::ASL, 6}, {"XXX", &CPU::IMP, &CPU::XXX, 0},
            {"BPL", &CPU::REL, &CPU::BPL, 2}, {"ORA", &CPU::IDY, &CPU:ORA, 5},
            {"XXX",&CPU::IMP,&CPU::XXX,0}, {"XXX",&CPU::IMP,&CPU::XXX,0}, 
            {"XXX",&CPU::IMP,&CPU::XXX,0}, {"ORA",&CPU::ZPX,&CPU::ORA,4},
            {"ASL",&CPU::ZPX,&CPU::ASL,6}, {"XXX",&CPU::IMP,&CPU::XXX,0}, 
            {"CLC",&CPU::IMP, &CPU::ABY,4}, {"XXX",&CPU::IMP,&CPU::XXX,0},
            {"XXX",&CPU::IMP,&CPU::XXX,0}, {"XXX",&CPU::IMP,&CPU::XXX,0},
            {"ORA",&CPU::ABX,&CPU:ORA,4}, {"ASL",&CPU::ABX,&CPU::ASL,7},
            {"XXX",&CPU::IMP,&CPU::XXX,0}, };
}
CPU::~CPU() { std::cout << "It's joever" << std::endl; }
void CPU::execute() {

  // opcode = bus->read(PC++);
  // debug();
  // cycles += (this->*lookup[opcode].addressing_mode)();
  // (this->*lookup[opcode].instruction)();

  // tick();
  // opcode = bus->read(PC++);
  // debug();
  // cycles += (this->*lookup[opcode].addressing_mode)();
  // (this->*lookup[opcode].instruction)();
  // debug();
  while (1) {
    opcode = bus->read(PC++);
    bool decision = debug();
    if (!decision)
      return;
    // instructions_t aoeu = fetch();
    cycles += (this->*lookup[opcode].addressing_mode)();
    (this->*lookup[opcode].instruction)();
    tick();
  }
}

//===================clock cycle track thing====================
/**
 * @brief will stay in loop until clock cycles is complete;
 * expected to be executed after executing instruction.
 *
 **/
void CPU::tick() {
  using namespace std::chrono;
  while (cycles) {
    cycles--;
    std::this_thread::sleep_for(duration<double, std::nano>(558.65922));
  }
}
//===================STATUS FLAG FUNCTIONS======================

/**
 * @brief Gets the status of a particular flag register.
 *
 * This function retrieves the status of the specified flag register.
 * If the operation fails, the program will exit.
 *
 * @param flag The particular flag we want to check.
 * @return The status of the register.
 **/
uint8_t CPU::get_flag(char flag) {
  switch (flag) {
  case 'N':
    return STATUS & NEGATIVE_FLAG;
  case 'V':
    return STATUS & OVERFLOW_FLAG;
  case 'U':
    return 0;
  case 'B':
    return STATUS & BREAK_FLAG;
  case 'D':
    return STATUS & DECIMAL_FLAG;
  case 'I':
    return STATUS & INTERRUPT_FLAG;
  case 'Z':
    return STATUS & ZERO_FLAG;
  case 'C':
    return STATUS & CARRY_FLAG;
  default:
    std::cerr << "Invalid status flag set" << std::endl;
    exit(-1);
  }
  return -1;
}

/**
 * @brief set the particular flag register
 * @param flag the flag we want to set
 * @param set whether we want to enable or disable it.
 *
 * If the operation fails, the program will exit with a
 * return code of -1.
 **/
void CPU::set_flag(char flag, bool set) {
  if (set) {
    switch (flag) {
    case 'N':
      STATUS |= NEGATIVE_FLAG;
      break;
    case 'V':
      STATUS |= OVERFLOW_FLAG;
      break;
    case 'U':
      break;
    case 'B':
      STATUS |= BREAK_FLAG;
      break;
    case 'D':
      STATUS |= DECIMAL_FLAG;
      break;
    case 'I':
      STATUS |= INTERRUPT_FLAG;
      break;
    case 'Z':
      STATUS |= ZERO_FLAG;
      break;
    case 'C':
      STATUS |= CARRY_FLAG;
      break;
    default:
      std::cerr << "Invalid status flag set" << std::endl;
      exit(-1);
    }
  } else {
    switch (flag) {
    case 'N':
      STATUS &= ~NEGATIVE_FLAG;
      break;
    case 'V':
      STATUS &= ~OVERFLOW_FLAG;
      break;
    case 'U':
      return;
    case 'B':
      STATUS &= ~BREAK_FLAG;
      break;
    case 'D':
      STATUS &= ~DECIMAL_FLAG;
      break;
    case 'I':
      STATUS &= ~INTERRUPT_FLAG;
      break;
    case 'Z':
      STATUS &= ~ZERO_FLAG;
      break;
    case 'C':
      STATUS &= ~CARRY_FLAG;
      break;
    default:
      std::cerr << "Invalid status flag set" << std::endl;
      exit(-1);
    }
  }
}

//========================== interrupt mode ======================

/**
 * @brief Implementation of Maskable interrupt.
 *
 * - interrupt will be ignored if interrupt disable is set.
 * - pushes PC and status register to stack, and store
 *   address of interrupt handling routine from address
 *   FFFE and FFFF into PC
 *
 * @note Interrupt disable flag is set here, unless it's already set
 *
 * @todo figure out the "triggering of a NMI can be prevented if bit
 * 7 of PPU Control Register 1 ($2000) is clear."
 **/
void CPU::IRQ() {
  if (get_flag('I')) {
    return;
  }
  uint8_t hi = (uint8_t)(PC >> 8);
  uint8_t lo = (uint8_t)PC;

  // stored in order to respect endianness
  this->bus->write(0x0100 + SP--, hi);
  this->bus->write(0x0100 + SP--, lo);
  this->bus->write(0x0100 + SP--, STATUS);
  set_flag('I', true);
  lo = this->bus->read(0xFFFE);
  hi = this->bus->read(0xFFFF);
  PC = (hi << 8) | lo;
  cycles = 7;
  tick();
}

/**
 * @brief Non-Maskable interrupts.
 *
 * -This is similar to maskable interrupts, but will
 *  still execute, even if the interrupt disable flag
 *  is still enabled.
 *
 **/
void CPU::NMI() {
  uint8_t hi = (uint8_t)(PC >> 8);
  uint8_t lo = (uint8_t)PC;

  // stored in order to respect endianness
  this->bus->write(0x0100 + SP--, hi);
  this->bus->write(0x0100 + SP--, lo);
  this->bus->write(0x0100 + SP--, STATUS);
  set_flag('I', true);
  lo = this->bus->read(0xFFFA);
  hi = this->bus->read(0xFFFB);
  PC = (hi << 8) | lo;
  cycles = 7;
  tick();
}

void CPU::RESET() {
  A = X = STATUS = Y = opcode = 0;
  SP = 0xFD;
  set_flag('I', true);
  uint8_t lo = this->bus->read(0xFFFC);
  uint8_t hi = this->bus->read(0xFFFD);
  PC = (hi << 8) | lo;
  cycles = 7;
  tick();
}

//====================addressing modes=======================

/**
 * @brief - Zero Page Addressing mode.
 *
 * This addressing mode will read one byte from the
 * program and go to the address specified. possible
 * range: 0x0000-0x00FF
 * @return 0, for number of additional clock cycles.
 *
 */
uint8_t CPU::ZP() {
  addr_abs = bus->read(PC++);
  addr_abs &= 0x00FF;
  return 0;
}

/**
 * @brief - Zero Page Addressing with X offset
 *
 * This will read address from program, but will
 * add the value from the X register to it.
 * @return additional clock cycles
 *
 **/
uint8_t CPU::ZPX() {
  addr_abs = bus->read(PC++);
  addr_abs += X;
  addr_abs &= 0x00FF;
  return 0;
}

/**
 * @brief The Zero Page addressing with Y offset
 * This will read address from program, and add
 * the value stored in the Y register.
 * @return 0, which indicates how many additional clock cycles.
 *
 **/
uint8_t CPU::ZPY() {
  addr_abs = bus->read(PC++);
  addr_abs += Y;
  addr_abs &= 0x00FF;
  return 0;
}

/**
 * @brief Absolute addressing mode
 * This will read two bytes from your program.
 * and read from that address.
 * @return additional clock cycles, always 0.
 *
 **/
uint8_t CPU::ABS() {
  uint8_t lo = bus->read(PC++);
  uint8_t hi = bus->read(PC++);
  addr_abs = (hi << 8) | lo;
  return 0;
}

/**
 * @brief Absolute addressing with X offset
 *
 * This will read two bytes from your program,
 * then add X to the absolute address.
 * @return additional clock cycles, if page's
 * crossed.
 *
 **/
uint8_t CPU::ABX() {
  uint8_t lo = bus->read(PC++);
  uint8_t hi = bus->read(PC++);
  addr_abs = (hi << 8) | lo;
  addr_abs += X;
  if ((addr_abs & 0xFF00) != (hi << 8)) {
    return 1;
  }
  return 0;
}

/**
 * @brief Absolute Adressing with Y offset
 *
 * This will read two bytes from your program,
 * then add Y to the absolute address
 *
 * @return additional clock cycles, 1 if page
 * crossed.
 *
 **/
uint8_t CPU::ABY() {
  uint8_t lo = bus->read(PC++);
  uint8_t hi = bus->read(PC++);
  addr_abs = (hi << 8) | lo;
  addr_abs += Y;
  if ((addr_abs & 0xFF00) != (hi << 8)) {
    return 1;
  }
  return 0;
}

/**
 * @brief Indirect addressing mode
 *
 * This will read the address, temp, from the program.
 * It will then read the address, addr_abs, from temp
 * address.
 * @return additional clock cycles, always 0.
 *
 **/
uint8_t CPU::IND() {
  uint8_t lo = bus->read(PC++);
  uint8_t hi = bus->read(PC++);
  uint16_t tmp_adr = (hi << 8) | lo;
  lo = bus->read(tmp_adr);
  hi = bus->read(tmp_adr + 1);
  addr_abs = (hi << 8) | lo;
  return 0;
}

/**
 * @brief implied addressing mode
 *
 * Nothing has to be done here, no need for addressing
 * @return additional clock cycles, 0.
 *
 **/
uint8_t CPU::IMP() { return 0; }

/**
 * @brief Accumulator addressing mode
 * will simply act directly on the accumulator
 * @todo something, maybe?
 * @return additional clock cycles, 0.
 *
 **/
uint8_t CPU::ACC() { return 0; }
/**
 * @brief immediate addressing mode
 * We'll just read the byte from the program.
 * @return additional clock cycles, 0.
 *
 */
uint8_t CPU::IMM() {
  addr_abs = PC++;
  return 0;
}

/**
 * @brief Relative addressing mode
 * Provide a relative address, mainly
 * for jmp instructions.
 * @return additional clockcycles, which will always be 0.
 *
 **/
uint8_t CPU::REL() {
  addr_rel = bus->read(PC++);
  if (addr_rel & 0x80) {
    addr_rel |= 0xFF00;
  }
  return 0;
}

/**
 * @brief Indirect addressing mode with X offset
 * Will get the address from Program, add X to this
 * address, then get the address from program + x.
 * @return addional clock cycles, which will be 0
 *
 */
uint8_t CPU::IDX() {
  uint8_t temp_address = bus->read(PC++);
  temp_address += X;
  uint8_t lo = bus->read(temp_address) & 0xFF;
  uint8_t hi = bus->read(temp_address + 1) & 0xFF;
  addr_abs = (hi << 8) | lo;
  return 0;
}

/**
 * @brief Indirect Addressing Mode with Y offset.
 *
 * Will get the address from Program, add Y, get
 * address from newly computated address.
 * @return additional clock cycles, 1 if page cross.
 *
 **/
uint8_t CPU::IDY() {
  uint8_t temp_address = bus->read(PC++);
  uint8_t lo = bus->read(temp_address) & 0xFF;
  uint8_t hi = bus->read(temp_address + 1) & 0xFF;
  addr_abs = (hi << 8) | lo;
  addr_abs += Y;
  if ((addr_abs & 0xFF00) != (hi << 8)) {
    return 1;
  }
  return 0;
}

//================instructions===================

void CPU::ADC() {
  uint8_t temp_byte = bus->read(addr_abs);
  uint8_t carry_val = get_flag('C') ? 1 : 0;
  uint16_t temp = A + temp_byte + carry_val;
  // set overflow flag
  set_flag('V', ((A & 0x80) != (temp & 0x80)));
  // set negative flag
  set_flag('N', (A & 0x80));
  // set zero flag
  set_flag('Z', (temp == 0));

  // set carry flag
  if (get_flag('D')) {
    temp = BCD(A) + BCD(temp_byte) + carry_val;
    set_flag('C', (temp > 99));
  } else {
    set_flag('C', (temp > 255));
  }
  A = temp & 0xFF;
  cycles += lookup[opcode].clock_cycles;
}
void CPU::AND() {
  uint8_t byte = bus->read(addr_abs);
  A = A & byte;
  if (A & 0x80) {
    set_flag('N', true);
  } else {
    set_flag('N', false);
  }
  if (A == 0) {
    set_flag('Z', true);
  } else {
    set_flag('Z', false);
  }
  cycles += lookup[opcode].clock_cycles;
}

void CPU::ASL() {
  if (addr_abs & 0x80) {
    set_flag('C', true);
  } else {
    set_flag('C', false);
  }
  uint8_t temp = (addr_abs << 1) & 0xFE;
  if (temp & 0x80) {
    set_flag('N', true);
  } else {
    set_flag('N', false);
  }
  if (temp == 0) {
    set_flag('Z', true);
  } else {
    set_flag('Z', false);
  }
  cycles += lookup[opcode].clock_cycles;
  bus->write(addr_abs, temp);
}

void CPU::BCC() {
  uint16_t temp = PC + (int8_t)addr_rel;
  if (!get_flag('C')) {
    cycles++;
    if ((temp & 0xFF00) != (PC & 0xFF00)) {
      cycles++;
    }
    PC = temp;
  }
  cycles += lookup[opcode].clock_cycles;
}

void CPU::BCS() {
  uint16_t temp = PC + (int8_t)addr_rel;

  if (get_flag('C')) {
    cycles++;
    if ((temp & 0xFF00) != (PC & 0xFF00)) {
      cycles++;
    }
    PC = temp;
  }
  cycles += lookup[opcode].clock_cycles;
}

void CPU::BEQ() {

  uint16_t temp = PC + (int8_t)addr_rel;
  if (get_flag('Z')) {
    cycles++;
    if ((temp & 0xFF00) != (PC & 0xFF00)) {
      cycles++;
    }
    PC = temp;
  }
  cycles += lookup[opcode].clock_cycles;
}

void CPU::BIT() {
  uint8_t M = bus->read(addr_abs);
  uint8_t temp = A & M;
  if (temp & 0x80) {
    set_flag('N', true);
  } else {
    set_flag('N', false);
  }
  if (temp & 0x40) {
    set_flag('V', true);
  } else {
    set_flag('V', true);
  }
  if (temp == 0) {
    set_flag('Z', true);
  } else {
    set_flag('Z', false);
  }
  cycles += lookup[opcode].clock_cycles;
}

void CPU::BMI() {
  uint16_t temp = PC + (int8_t)addr_rel;
  if (get_flag('N')) {
    cycles++;
    if ((temp & 0xFF00) != (PC & 0xFF00)) {
      cycles++;
    }
    PC = temp;
  }
  cycles += lookup[opcode].clock_cycles;
}

void CPU::BNE() {
  uint16_t temp = PC + (int8_t)addr_rel;
  if (!get_flag('Z')) {
    cycles++;
    if ((temp & 0xFF00) != (PC & 0xFF00)) {
      cycles++;
    }
    PC = temp;
  }
  cycles += lookup[opcode].clock_cycles;
}

void CPU::BPL() {
  uint16_t temp = PC + (int8_t)addr_rel;
  if (!get_flag('N')) {
    cycles++;
    if ((temp & 0xFF00) != (PC & 0xFF00)) {
      cycles++;
    }
    PC = temp;
  }
  cycles += lookup[opcode].clock_cycles;
}

void CPU::BRK() {
  uint8_t hi = PC >> 8;
  uint8_t lo = PC & 0xFF;
  // push current PC and STATUS register on stack.
  bus->write(0x0100 + SP--, hi);
  bus->write(0x0100 + SP--, lo);
  bus->write(0x0100 + SP--, STATUS);

  // get new address.
  lo = bus->read(0xFFFE);
  hi = bus->read(0xFFFF);
  PC = (hi << 8) | lo;
  cycles += lookup[opcode].clock_cycles;
}

void CPU::BVC() {
  uint16_t temp = PC + (int8_t)addr_rel;
  if (!get_flag('V')) {
    cycles++;
    if ((temp & 0xFF00) != (PC & 0xFF00)) {
      cycles++;
    }
    PC = temp;
  }
  cycles += lookup[opcode].clock_cycles;
}

void CPU::BVS() {
  uint16_t temp = PC + (int8_t)addr_rel;
  if (get_flag('V')) {
    cycles++;
    if ((temp & 0xFF00) != (PC & 0xFF00)) {
      cycles++;
    }
    PC = temp;
  }
  cycles += lookup[opcode].clock_cycles;
}

void CPU::CLC() { set_flag('C', false); }

void CPU::CLD() { set_flag('D', false); }

void CPU::CLI() { set_flag('I', false); }

void CPU::CLV() { set_flag('C', false); }

void CPU::CMP() {
  uint8_t byte = bus->read(addr_abs);
  uint8_t temp = A - byte;
  if (temp & 0x80) {
    set_flag('N', true);
  } else {
    set_flag('N', false);
  }
  if (A >= byte) {
    set_flag('C', true);
  } else {
    set_flag('C', false);
  }

  if (temp == 0) {
    set_flag('Z', true);
  } else {
    set_flag('Z', false);
  }
  cycles += lookup[opcode].clock_cycles;
}

void CPU::CPX() {
  uint8_t byte = bus->read(addr_abs);
  uint8_t temp = X - byte;
  if (temp & 0x80) {
    set_flag('N', true);
  } else {
    set_flag('N', false);
  }
  if (X >= byte) {
    set_flag('C', true);
  } else {
    set_flag('C', false);
  }

  if (temp == 0) {
    set_flag('Z', true);
  } else {
    set_flag('Z', false);
  }
  cycles += lookup[opcode].clock_cycles;
}

void CPU::CPY() {
  uint8_t byte = bus->read(addr_abs);
  uint8_t temp = Y - byte;
  if (temp & 0x80) {
    set_flag('N', true);
  } else {
    set_flag('N', false);
  }
  if (Y >= byte) {
    set_flag('C', true);
  } else {
    set_flag('C', false);
  }

  if (temp == 0) {
    set_flag('Z', true);
  } else {
    set_flag('Z', false);
  }
  cycles += lookup[opcode].clock_cycles;
}

void CPU::DEC() {
  uint8_t temp = bus->read(addr_abs) & 0xFF;
  temp--;
  if (temp & 0x80) {
    set_flag('N', true);
  } else {
    set_flag('N', false);
  }
  if (temp == 0) {
    set_flag('Z', true);
  } else {
    set_flag('Z', false);
  }
  bus->write(addr_abs, (temp & 0xFF));
  cycles += lookup[opcode].clock_cycles;
}

void CPU::DEX() {
  X--;
  if (X & 0x80) {
    set_flag('N', true);
  } else {
    set_flag('N', false);
  }
  if (X == 0) {
    set_flag('Z', true);
  } else {
    set_flag('Z', false);
  }
  cycles += lookup[opcode].clock_cycles;
}

void CPU::DEY() {
  Y--;
  if (Y & 0x80) {
    set_flag('N', true);
  } else {
    set_flag('N', false);
  }
  if (Y == 0) {
    set_flag('Z', true);
  } else {
    set_flag('Z', false);
  }
  cycles += lookup[opcode].clock_cycles;
}

void CPU::EOR() {
  uint8_t byte = bus->read(addr_abs) & 0xFF;
  A = A ^ byte;
  if (A & 0x80) {
    set_flag('N', true);
  } else {
    set_flag('N', false);
  }
  if (A == 0) {
    set_flag('Z', true);
  } else {
    set_flag('Z', false);
  }
  cycles += lookup[opcode].clock_cycles;
}

void CPU::INC() {
  uint8_t byte = bus->read(addr_abs) & 0xFF;
  byte++;
  if (byte & 0x80) {
    set_flag('N', true);
  } else {
    set_flag('N', false);
  }
  if (byte == 0) {
    set_flag('Z', true);
  } else {
    set_flag('Z', false);
  }
  bus->write(addr_abs, byte & 0xFF);
  cycles += lookup[opcode].clock_cycles;
}

void CPU::INX() {
  X++;
  if (X & 0x80) {
    set_flag('N', true);
  } else {
    set_flag('N', false);
  }
  if (X == 0) {
    set_flag('Z', true);
  } else {
    set_flag('Z', false);
  }
  cycles += lookup[opcode].clock_cycles;
}

void CPU::INY() {
  Y++;
  if (Y & 0x80) {
    set_flag('N', true);
  } else {
    set_flag('N', false);
  }
  if (Y == 0) {
    set_flag('Z', true);
  } else {
    set_flag('Z', false);
  }
  cycles += lookup[opcode].clock_cycles;
}

void CPU::JMP() {
  PC = addr_abs;
  cycles += lookup[opcode].clock_cycles;
}

void CPU::JSR() {
  PC--;
  bus->write(0x0100 + SP--, (PC >> 8) & 0xFF);
  bus->write(0x1000 + SP--, PC & 0xFF);
  PC = addr_abs;
  cycles += lookup[opcode].clock_cycles;
}
void CPU::LDA() {
  uint8_t byte = bus->read(addr_abs);
  A = byte;
  if (A & 0x80) {
    set_flag('N', true);
  } else {
    set_flag('N', false);
  }

  if (A == 0) {
    set_flag('Z', true);
  } else {
    set_flag('Z', false);
  }
  cycles += lookup[opcode].clock_cycles;
}

void CPU::LDX() {
  uint8_t byte = bus->read(addr_abs);
  X = byte;
  if (X & 0x80) {
    set_flag('N', true);
  } else {
    set_flag('N', false);
  }

  if (X == 0) {
    set_flag('Z', true);
  } else {
    set_flag('Z', false);
  }
  cycles += lookup[opcode].clock_cycles;
}

void CPU::LDY() {
  uint8_t byte = bus->read(addr_abs);
  Y = byte;
  if (Y & 0x80) {
    set_flag('N', true);
  } else {
    set_flag('N', false);
  }

  if (Y == 0) {
    set_flag('Z', true);
  } else {
    set_flag('Z', false);
  }
  cycles += lookup[opcode].clock_cycles;
}

void CPU::LSR() {
  set_flag('N', false);
  uint8_t byte;
  if (opcode == 0x4A) {
    byte = A;
  } else {
    byte = bus->read(addr_abs) & 0xFF;
  }
  if (byte & 0x1) {
    set_flag('C', true);
  } else {
    set_flag('C', false);
  }
  byte = (byte >> 1) & 0xFF;
  if (byte == 0) {
    set_flag('Z', true);
  } else {
    set_flag('Z', false);
  }
  if (opcode == 0x4A) {
    A = byte;
  } else {
    bus->write(addr_abs, byte & 0xFF);
  }
  cycles += lookup[opcode].clock_cycles;
}
void CPU::NOP() { cycles += lookup[opcode].clock_cycles; }

void CPU::ORA() {
  uint8_t byte = bus->read(addr_abs);
  A = A | byte;
  if (A & 0x80) {
    set_flag('N', true);
  } else {
    set_flag('N', false);
  }
  if (A == 0) {
    set_flag('Z', true);
  } else {
    set_flag('Z', false);
  }
  cycles += lookup[opcode].clock_cycles;
}

void CPU::PHA() {
  bus->write(0x0100 + SP--, A);
  cycles += lookup[opcode].clock_cycles;
}
void CPU::PHP() {
  bus->write(0x0100 + SP--, STATUS);
  cycles += lookup[opcode].clock_cycles;
}

void CPU::PLA() {
  SP++;
  A = bus->read(0x0100 + SP);
  set_flag('N', (A & 0x80) ? true : false);
  set_flag('Z', (A == 0) ? true : false);
  cycles += lookup[opcode].clock_cycles;
}
void CPU::PLP() {
  SP++;
  STATUS = bus->read(0x0100 + SP);
  cycles += lookup[opcode].clock_cycles;
}

void CPU::ROL() {
  uint8_t byte = bus->read(addr_abs) & 0xFF;
  uint8_t temp = byte & 0x80;
  byte = (byte << 1) & 0xFE;
  byte = byte | (get_flag('C')) ? 1 : 0;
  set_flag('C', (temp != 0));
  set_flag('Z', (byte == 0));
  set_flag('N', (byte & 0x80) ? true : false);
  cycles += lookup[opcode].clock_cycles;
}

void CPU::ROR() {
  uint8_t byte = bus->read(addr_abs);
  uint8_t temp = byte & 0x1;
  byte = (byte >> 1) & 0x3F;
  byte = byte | (get_flag('C')) ? 0x80 : 0;
  set_flag('C', (temp != 0));
  set_flag('Z', (byte == 0));
  set_flag('N', (byte & 0x80) ? true : false);
  cycles += lookup[opcode].clock_cycles;
}

void CPU::RTI() {
  SP++;
  STATUS = bus->read(0x0100 + SP) & 0xFF;
  SP++;
  uint8_t lo = bus->read(0x0100 + SP) & 0xFF;
  SP++;
  uint16_t hi = bus->read(0x1000 + SP) & 0xFF;
  PC = (hi << 8) | lo;
  cycles += lookup[opcode].clock_cycles;
}

void CPU::RTS() {
  SP++;
  uint8_t lo = bus->read(0x0100 + SP) & 0xFF;
  SP++;
  uint16_t hi = bus->read(0x0100 + SP) & 0xFF;
  PC = (hi << 8) | lo;
  PC++;
  cycles += lookup[opcode].clock_cycles;
}

void CPU::SBC() {
  uint16_t temp;
  uint8_t byte = bus->read(addr_abs) & 0xFF;
  uint8_t flag = (get_flag('C')) ? 0 : 1;
  if (get_flag('D')) {
    temp = (uint16_t)BCD(A) - (uint16_t)BCD(byte) - flag;
    set_flag('V', (temp > 99 || temp < 0) ? true : false);
  } else {
    temp = (uint16_t)A - (uint16_t)byte - flag;
    set_flag('V', (temp > 127 || temp < -128) ? true : false);
  }
  set_flag('C', (temp >= 0));
  set_flag('N', (temp & 0x80) ? true : false);
  set_flag('Z', (temp == 0));
  A = temp & 0xFF;
  cycles += lookup[opcode].clock_cycles;
}

void CPU::SEC() {
  set_flag('C', true);
  cycles += lookup[opcode].clock_cycles;
}

void CPU::SED() {
  set_flag('D', true);
  cycles += lookup[opcode].clock_cycles;
}

void CPU::SEI() {
  set_flag('I', true);
  cycles += lookup[opcode].clock_cycles;
}
void CPU::STA() {
  bus->write(addr_abs, A);
  cycles += lookup[opcode].clock_cycles;
}

void CPU::STX() {
  bus->write(addr_abs, X);
  cycles += lookup[opcode].clock_cycles;
}

void CPU::STY() {
  bus->write(addr_abs, Y);
  cycles += lookup[opcode].clock_cycles;
}

void CPU::TAX() {
  X = A;
  set_flag('N', (X & 0x80) ? true : false);
  set_flag('Z', (X == 0));
  cycles += lookup[opcode].clock_cycles;
}

void CPU::TAY() {
  Y = A;
  set_flag('N', (Y & 0x80) ? true : false);
  set_flag('Z', (Y == 0));
  cycles += lookup[opcode].clock_cycles;
}

void CPU::TSX() {
  X = SP;
  set_flag('N', (Y & 0x80) ? true : false);
  set_flag('Z', (Y == 0));
  cycles += lookup[opcode].clock_cycles;
}

void CPU::TXA() {
  A = X;
  set_flag('N', (A & 0x80) ? true : false);
  set_flag('Z', (A == 0));
  cycles += lookup[opcode].clock_cycles;
}

void CPU::TXS() {
  SP = X;
  cycles += lookup[opcode].clock_cycles;
}

void CPU::TYA() {
  A = A;
  set_flag('N', (A & 0x80) ? true : false);
  set_flag('Z', (A == 0));
  cycles += lookup[opcode].clock_cycles;
}

void CPU::XXX() { std::cout << "illegal opcode" << std::endl; }
//====================misc helper functions========================
uint8_t CPU::BCD(uint8_t data) {
  uint8_t lo = data % 10;
  data = data / 10;
  uint8_t hi = data << 4;
  return hi | lo;
}

bool CPU::debug() {
  std::cout << "===========debug info===========" << std::endl;
  std::cout << "opcode: " << static_cast<int>(opcode) << std::endl;
  std::cout << "PC = " << static_cast<int>(PC)
            << ", A = " << static_cast<int>(A)
            << ", X = " << static_cast<int>(X)
            << ", Y = " << static_cast<int>(Y)
            << ", SP = " << static_cast<int>(SP) << std::endl;
  char decision;
  std::cin >> decision;
  return decision != 'q';
}