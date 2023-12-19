
#pragma once
#include <cstdint>
#include <string>
#include <iostream>
#include <fstream>
class CPU {
public:
    //registers
    uint16_t PC = 0; //Program Counter: points to next instruction to be executed
    uint8_t SP = 0;  //Stack Pointer: stack pointer located between 0x0100 and 0x01FF. Holds lower 8 bits of the next free location
    uint8_t ACC = 0; //Accumulator: value in reg used in arithmetic and logical operations.
    uint8_t IRX = 0; //Index Register X: This holds the counters or offsets for accessing memory.
    uint8_t IRY = 0; //Index Register Y
    uint8_t Status = 0; //Processor Status

    enum status_flag {
        carry = 0x01,
        zero_flag = 0x02,
        interrupt_disable = 0x04,
        decimal_mode = 0x08,
        breaku = 0x10,
        unused = 0x20,
        overflow = 0x40,
        negative_result = 0x80,
    };

    struct Instruction {
        uint8_t i_len;           // Instruction length: number of bytes memory requires to store instruction
        uint8_t t_cnt;           // Timing Cycle Count: number of clock cycles required to execute the instruction
        std::string name;        // Name of instruction
        std::string addressing_mode;  // Memory addressing mode used by instruction.
    };

    CPU();
    ~CPU();
    uint8_t execute(std::string file);
private:
    uint16_t memory[64 * 1024];    
    int memlength = sizeof(memory) / sizeof(uint16_t);
};
