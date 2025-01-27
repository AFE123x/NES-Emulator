#pragma once

#include "./common.hpp"
class Bus;

class Cpu{
    public:
        void clock();
        void reset();
        Cpu(Bus* bus);
        ~Cpu();
    private:
        //instructions types
        void initialize_loadstore_instructions();
        void populate_opcode(uint8_t opcode, std::string name, uint8_t cycles, void (Cpu::*instruction)(void), void (Cpu::*addressing_mode)(void));
        typedef struct {
            std::string name;
            uint8_t cycles;
            void (Cpu::*instruction)(void);
            void (Cpu::*addressing_mode)(void);
        }instructions_t;

        instructions_t opcode_table[255];


        //addressing mode types:
        uint8_t immval; 
        uint16_t addr_abs; 
        uint16_t addr_rel;

        //clock cycle variables
        uint8_t cycles_left;
        uint32_t total_cycles;

        //our bus
        Bus* bus;
        //special purpose registers
        uint16_t PC; //our program counter
        uint8_t SP; //our stack pointer

        //general purpose registers
        uint8_t A; //accumulator register
        uint8_t X; // Index Register X
        uint8_t Y; // Index Register Y

        typedef union{
            struct{
                uint8_t CF: 1;
                uint8_t ZF: 1;
                uint8_t I: 1;
                uint8_t D: 1;
                uint8_t BRK: 1;
                uint8_t OF: 1;
                uint8_t SF: 1;
            };
            uint8_t raw;
        } processor_status;

        processor_status status_reg; //our status register.

        // Load/Store Operations
        void LDA(); void LDX(); void LDY();
        void STA(); void STX(); void STY();

        // Register Transfers
        void TAX(); void TAY(); void TXA();
        void TYA();

        // Stack Operations
        void TSX(); void TXS(); void PHA();
        void PHP(); void PLA(); void PLP();

        // Logical
        void AND(); void EOR(); void ORA();
        void BIT();

        // Arithmetic
        void ADC(); void SBC(); void CMP();
        void CPX(); void CPY();

        // Increments & Decrements
        void INC(); void INX(); void INY();
        void DEC(); void DEX(); void DEY();

        // Shifts
        void ASL(); void LSR(); void ROL();
        void ROR();

        // Jumps & Calls
        void JMP(); void JSR(); void RTS();

        // Branches
        void BCC(); void BCS(); void BEQ();
        void BMI(); void BNE(); void BPL();
        void BVC(); void BVS();

        // Status Flag Changes
        void CLC(); void CLD(); void CLI();
        void CLV(); void SEC(); void SED();
        void SEI();

        // System Functions

        void BRK(); void NOP(); void RTI();


        // Addressing modes

        void IMP(); void ACC(); void IMM();
        void ZP0(); void ZPX(); void ZPY();
        void REL(); void ABS(); void ABX();
        void ABY(); void IND(); void IDX();
        void IDY();


};