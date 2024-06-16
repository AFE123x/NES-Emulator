#include "../lib/CPU.h"
#include "../lib/BUS.h"
#include <iostream>
#include <thread>
CPU::CPU(BUS *bus) {
  this->bus = bus;
  RESET();
}
CPU::~CPU() { std::cout << "It's joever" << std::endl; }
void CPU::execute() { tick(); }

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
}

void CPU::RESET() {
  A = X = STATUS = Y = opcode = 0;
  SP = 0xFD;
  set_flag('I', true);
  uint8_t lo = this->bus->read(0xFFFC);
  uint8_t hi = this->bus->read(0xFFFD);
  PC = (hi << 8) | lo;
  cycles = 7;
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
  uint8_t lo = bus->read(temp_address) && 0xFF;
  uint8_t hi = bus->read(temp_address + 1) && 0xFF;
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
  uint8_t lo = bus->read(temp_address) && 0xFF;
  uint8_t hi = bus->read(temp_address + 1) && 0xFF;
  addr_abs = (hi << 8) | lo;
  addr_abs += Y;
  if ((addr_abs & 0xFF00) != (hi << 8)) {
    return 1;
  }
  return 0;
}