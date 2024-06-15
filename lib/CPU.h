#pragma once
#include <cstdint>

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
  void execute();

private:
  // our lovely registers
  uint16_t PC;    // program counter
  uint8_t SP;     // stack pointer
  uint8_t A;      // accumulator
  uint8_t X;      // X register
  uint8_t STATUS; // status register
  BUS *bus;

  // helper variables
  uint8_t cycles;
  uint8_t opcode;
  uint16_t addrmode;

  // status flag functions
  uint8_t get_flag(char flag);
  void set_flag(char flag, bool set);
  // instruction struct
  typedef struct instructions {
    uint8_t (*addressing_mode)(void);
    void (*instruction)(void);
    uint8_t clock_cycles;
    uint8_t bytes;
  };

  //interrupt stuff
  void interrupt();
  //clock stuff
  void tick();

};
