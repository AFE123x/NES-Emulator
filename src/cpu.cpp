#include "../lib/cpu.h"
#include "../lib/bus.h"

BUS::CPU(BUS *bus) {
  this->bus = bus;
  RESET();
}
BUS::~CPU() { std::cout << "CPU deallocated" << std::endl; }
void BUS::tick() {
  if (cycles == 0) {
  }
  cycles--;
  total_cycles++;
}
uint8_t BUS::read(uint16_t address) { return bus->cpuread(address); }
void BUS::write(uint16_t address, uint8_t byte) { bus->cpuwrite(address); }
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
  // checks if interrupt flag is already enabled.
  if (get_flag(Interrupt)) {
    return;
  }
  // extract upper and lower byte of program counter.
  uint8_t lo = PC & 0xFF;
  uint8_t hi = PC >> 8;

  // store Program Counter and flags to stack.
  write(0x100 + SP--, hi);
  write(0x100 + SP--, lo);
  write(0x100 + SP--, flag);
  // set interrupt flag
  set_flag(Interrupt);
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
  set_flag(Interrupt);
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
  SP = 0xFF;            // set stack to default
  A = X = Y = flag = 0; // reset registers
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