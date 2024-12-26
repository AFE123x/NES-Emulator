#include "../include/cpu.h"
#include "../include/bus.h"
#include "../include/address_modes.h"
#include "../include/instructions.h"
#include <stdio.h>
#include<stdlib.h>
#include<assert.h>
#include<string.h>

#ifdef UNIT_TESTING
#include<criterion/criterion.h>
#endif
/* Helper global variables */
// Temporary variables used during instruction decoding and execution.
uint8_t immval;   // Immediate value fetched from memory.
uint16_t abs_addr; // Absolute address calculated during decoding.
int8_t rel_addr;   // Relative address used for branching.

/* Special registers */
// Registers specific to the 6502 CPU architecture.
uint16_t PC; // Program Counter: Points to the next instruction to execute.
uint8_t SP;  // Stack Pointer: Points to the top of the stack in memory.

uint64_t total_cycles;
uint8_t cycles;

processor_state state; // CPU status flags instance.

/* General-purpose registers */
// Registers used during computation and addressing.
uint8_t A; // Accumulator: Used for arithmetic and logic operations.
uint8_t X; // X Register: Often used for indexing.
uint8_t Y; // Y Register: Often used for indexing.

/**
 * @brief Structure for representing a CPU instruction.
 * 
 * - `address_mode`: Function pointer to the addressing mode handler.
 * - `instruction`: Function pointer to the actual instruction handler.
 * - `cycles`: Number of clock cycles the instruction takes.
 * - `name`: Human-readable name of the instruction (for debugging/logging).
 */
typedef struct {
    void (*address_mode)(void); // Addressing mode handler.
    void (*instruction)(void); // Instruction handler.
    uint8_t cycles;            // Cycle count.
    char name[15];             // Instruction name.
} instructions_t;


instructions_t opcodetable[255];


/**
 * @brief Adds an opcode to the opcode table with its associated properties.
 * 
 * This function populates the opcode table entry for a specific opcode with its
 * addressing mode, instruction handler, execution cycle count, and name. It is used 
 * to define the behavior of the 6502 CPU for each opcode.
 * 
 * @param opcode The opcode value (0x00 to 0xFF) representing the instruction.
 * @param address_mode Pointer to the function that implements the addressing mode for this opcode.
 * @param instruction Pointer to the function that implements the instruction logic for this opcode.
 * @param cycles The number of clock cycles the instruction takes to execute.
 * @param name The mnemonic name of the instruction (e.g., "LDA", "STA").
 */
static void addopcode(uint8_t opcode, void (*address_mode)(void), void (*instruction)(void), uint8_t cycles, const char* name) {
    // Set the addressing mode function pointer for the opcode.
    opcodetable[opcode].address_mode = address_mode;

    // Set the cycle count for the opcode.
    opcodetable[opcode].cycles = cycles;

    // Set the instruction handler function pointer for the opcode.
    opcodetable[opcode].instruction = instruction;

    // Copy the mnemonic name into the opcode table entry.
    strcpy(opcodetable[opcode].name, name);
}


static void LOADSTORE_INSTRUCTIONS(){
    addopcode(0xA9,addr_immediate,LDA,2,"LDA {IMM}");
    addopcode(0xA5,addr_zero_page,LDA,3,"LDA {ZP}");
    addopcode(0xB5,addr_zero_page_x,LDA,4,"LDA {ZPX}");
    addopcode(0xAD,addr_absolute,LDA,4,"LDA {ABS}");
    addopcode(0xBD,addr_absolute_x,LDA,4,"LDA {ABSX}");
    addopcode(0xB9,addr_absolute_y,LDA,4,"LDA {ABSY}");
    addopcode(0xA1,addr_indexed_indirect,LDA,6,"LDA {INDX}");
    addopcode(0xB1,addr_indirect_indexed,LDA,5,"LDA {INDY}");

    addopcode(0xA2,addr_immediate,LDX,2,"LDX {IMM}");
    addopcode(0xA6,addr_zero_page,LDX,3,"LDX {ZP}");
    addopcode(0xB6,addr_zero_page_y,LDX,4,"LDX {ZPY}");
    addopcode(0xAE,addr_absolute,LDX,4,"LDX {ABS}");
    addopcode(0xBE,addr_absolute_y,LDX,4,"LDX {ABSY}");

    addopcode(0xA0,addr_immediate,LDY,2,"LDY {IMM}");
    addopcode(0xA4,addr_zero_page,LDY,3,"LDY {ZP}");
    addopcode(0xB4,addr_zero_page_x,LDY,4,"LDY {ZPX}");
    addopcode(0xAC,addr_absolute,LDY,4,"LDY {ABS}");
    addopcode(0xBC,addr_absolute_x,LDY,4,"LDY {ABSX}");

    addopcode(0x85,addr_zero_page,STA,3,"STA {ZP}");
    addopcode(0x95,addr_zero_page_x,STA,4,"STA {ZPX}");
    addopcode(0x8D,addr_absolute,STA,4,"STA {ABS}");
    addopcode(0x9D,addr_absolute_x,STA,5,"STA {ABSX}");
    addopcode(0x99,addr_absolute_y,STA,5,"STA {ABSY}");
    addopcode(0x81,addr_indexed_indirect,STA,6,"STA {INDX}");
    addopcode(0x91,addr_indirect_indexed,STA,6,"STA {INDY}");

    addopcode(0x86,addr_zero_page,STX,3,"STX {ZP}");
    addopcode(0x96,addr_zero_page_y,STX,4,"STX {ZPY}");
    addopcode(0x8E,addr_absolute,STX,4,"STX {ABS}");

    addopcode(0x84,addr_zero_page,STY,3,"STY {ZP}");
    addopcode(0x94,addr_zero_page_x,STY,4,"STY {ZPX}");
    addopcode(0x8C,addr_absolute,STY,4,"STY {ABS}");
}

void REGISTERTRANSFER_INSTRUCTIONS(){
  addopcode(0xAA,addr_implied,TAX,2,"TAX {IMP}");
  addopcode(0xA8,addr_implied,TAY,2,"TAY {IMP}");
  addopcode(0x8A,addr_implied,TXA,2,"TXA {IMP}");
  addopcode(0x98,addr_implied,TYA,2,"TYA {IMP}");
}

void STACK_OPERATIONS_INSTRUCTIONS(){
    addopcode(0x9A,addr_implied,TXS,2,"TXS {IMP}");
    addopcode(0xBA,addr_implied,TSX,2,"TSX {IMP}");
    addopcode(0x48,addr_implied,PHA,3,"PHA {IMP}");
    addopcode(0x08,addr_implied,PHP,3,"PHP {IMP}");
    addopcode(0x68,addr_implied,PLA,4,"PLA {IMP}");
    addopcode(0x28,addr_implied,PLP,4,"PLP {IMP}");
}

/* CPU Initialization */
/**
 * @brief Initializes the CPU.
 */
void cpu_init() {
    LOADSTORE_INSTRUCTIONS();
    REGISTERTRANSFER_INSTRUCTIONS();
    STACK_OPERATIONS_INSTRUCTIONS();
    cpu_write(0,0x68);
    cpu_write(0,0x08);
    cpu_write(1,0x68);
    PC = 0;
    state.raw = 0x45;
    SP = 0xFF;
}

/* CPU Clock Cycle */
/**
 * @brief Executes a single CPU clock cycle.
 */
void clock_cpu() {
    
    if(cycles == 0){
        uint8_t opcode;
        cpu_read(PC++,&opcode);
        instructions_t decode = opcodetable[opcode];
        cycles = decode.cycles;
        /* addressing mode*/
        decode.address_mode();
        /* instruction*/
        decode.instruction();

        /* print out instruction*/
        printf("%x: %s\n",opcode,decode.name);
    }
    cycles--;
}


#ifdef UNIT_TESTING

TestSuite(Instructions);

Test(Instructions,LDA){
    cpu_init();
    cpu_write(0,0xA9);
    cpu_write(1,0xFF);
    PC = 0;
    clock_cpu();
    cr_assert_eq(A,255,"LDA test - FAILED!");
}

Test(Instructions,LDX){
    cpu_init();
    cpu_write(0,0xA2);
    cpu_write(1,0xFF);
    PC = 0;
    clock_cpu();
    cr_assert_eq(X,255,"LDX test - FAILED!");
}

Test(Instructions,LDY){
    cpu_init();
    cpu_write(0,0xA0);
    cpu_write(1,0xFF);
    PC = 0;
    clock_cpu();
    cr_assert_eq(Y,255,"LDY test - FAILED!");
}

Test(Instructions,STA){
    cpu_init();
    cpu_write(0,0x85);  
    cpu_write(1,0x95);
    PC = 0;
    A = 69;
    
    clock_cpu();
    uint8_t byte;
    cpu_read(0x95,&byte);
    cr_assert_eq(byte,69,"STA test - FAILED! value: %d",byte);
}

Test(Instructions,STX){
    cpu_init();
    cpu_write(0,0x86);  
    cpu_write(1,0x95);
    PC = 0;
    X = 69;
    
    clock_cpu();
    uint8_t byte;
    cpu_read(0x95,&byte);
    cr_assert_eq(byte,69,"STX test - FAILED! value: %d",byte);
}

Test(Instructions,STY){
    cpu_init();
    cpu_write(0,0x84);  
    cpu_write(1,0x95);
    PC = 0;
    Y = 69;
    
    clock_cpu();
    uint8_t byte;
    cpu_read(0x95,&byte);
    cr_assert_eq(byte,69,"STY test - FAILED! value: %d",byte);
}


Test(Instructions,TAX){
  cpu_init();
  cpu_write(0,0xAA);
  PC = 0;
  A = 10;
  clock_cpu();
  cr_assert_eq(X,10,"TAX test - FAILED! value: %d",X);
}

Test(Instructions,TAY){
  cpu_init();
  cpu_write(0,0xA8);
  PC = 0;
  A = 10;
  clock_cpu();
  cr_assert_eq(Y,10,"TAY test - FAILED! value: %d",Y);

}

Test(Instructions,TXA){
  cpu_init();
  cpu_write(0,0x8A);
  PC = 0;
  X = 20;
  clock_cpu();
  cr_assert_eq(A,20,"TXA test - FAILED! value: %d",A);
}

Test(Instructions,TYA){
  cpu_init();
  cpu_write(0,0x98);
  PC = 0;
  Y = 90;
  clock_cpu();
  cr_assert_eq(A,90,"TYA test - FAILED! value: %d",A);
}
/*
    addopcode(0x9A,addr_implied,TXS,2,"TXS {IMP}");
    addopcode(0xBA,addr_implied,TSX,2,"TSX {IMP}");
    addopcode(0x48,addr_implied,PHA,3,"PHA {IMP}");
    addopcode(0x08,addr_implied,PHP,3,"PHP {IMP}");
    addopcode(0x68,addr_implied,PLA,4,"PLA {IMP}");
    addopcode(0x28,addr_implied,PLP,4,"PLP {IMP}");
*/
Test(Instructions,TXS){
    cpu_init();
    cpu_write(0,0x9A);
    PC = 0;
    X = 0xFF;
    SP = 0x5;
    clock_cpu();
    cr_assert_eq(SP,0xFF,"TXS test - FAILED! value: %d",SP);
}
Test(Instructions,TSX){
    cpu_init();
    cpu_write(0,0xBA);
    PC = 0;
    X = 0xFF;
    SP = 0x5;
    clock_cpu();
    cr_assert_eq(X,0x5,"TSX test - FAILED! value: %d",X);
}
Test(Instructions,PHA){
    cpu_init();
    cpu_write(0,0x48);
    PC = 0;
    A = 0x45;
    SP = 0xFF;
    clock_cpu();
    uint8_t byte;
    cpu_read(0x1FF,&byte);
    cr_assert_eq(byte,0x45,"TYA test - FAILED! value: %d",byte);
}
Test(Instructions,PHP){
    cpu_init();
    cpu_write(0,0x08);
    PC = 0;
    state.raw = 0x45;
    SP = 0xFF;
    clock_cpu();
    uint8_t byte;
    cpu_read(0x1FF,&byte);
    cr_assert_eq(byte,0x45,"TYA test - FAILED! value: %d",byte);
}

Test(Instructions,PLA){
    cpu_init();
    cpu_write(0,0x08);
    cpu_write(1,0x68);
    PC = 0;
    state.raw = 0x45;
    SP = 0xFF;
    clock_cpu();
    cycles = 0;
    clock_cpu();
    cr_assert_eq(A,0x45,"TYA test - FAILED! value: %d",A);
}

Test(Instructions,PLP){
    cpu_init();
    cpu_write(0,0x08);
    cpu_write(1,0x28);
    PC = 0;
    state.raw = 0xFF;
    SP = 0xFF;
    clock_cpu();
    cycles = 0;
    clock_cpu();
    cr_assert_eq(state.raw,0xFF,"TYA test - FAILED! value: %d",A);
}

#endif
