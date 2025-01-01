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

/**
 * @brief Get status function, which will retrieve the CPU state.
 * intended for debugging purposes.
 */
struct cpu_test get_status(){
    struct cpu_test result;
    result.PC = PC;
    result.A = A;
    result.X = X;
    result.Y = Y;
    result.SP = SP;
    cpu_read(0x2,&result.two_byte);
    cpu_read(0x3,&result.three_byte);
    return result;
}
uint8_t opcode; //global variable for opcode.

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

void LOGICAL_OPERATIONS(){
    /* exclusive or */
    addopcode(0x49,addr_immediate,EOR,2,"EOR {imm}");
    addopcode(0x45,addr_zero_page,EOR,3,"EOR {ZP0}");
    addopcode(0x55,addr_zero_page_x,EOR,4,"EOR {ZPX}");
    addopcode(0x4D,addr_absolute,EOR,4,"EOR {ABS}");
    addopcode(0x5D,addr_absolute_x,EOR,4,"EOR {ABX}");
    addopcode(0x59,addr_absolute_y,EOR,4,"EOR {ABY}");
    addopcode(0x41,addr_indexed_indirect,EOR,6,"EOR {IDX}");
    addopcode(0x51,addr_indirect_indexed,EOR,5,"EOR {IDY}");

    /* Logical AND */
    addopcode(0x29,addr_immediate,AND,2,"AND {imm}");
    addopcode(0x25,addr_zero_page,AND,3,"AND {ZP0}");
    addopcode(0x35,addr_zero_page_x,AND,4,"AND {ZPX}");
    addopcode(0x2D,addr_absolute,AND,4,"AND {ABS}");
    addopcode(0x3D,addr_absolute_x,AND,4,"AND {ABX}");
    addopcode(0x39,addr_absolute_y,AND,4,"AND {ABY}");
    addopcode(0x21,addr_indexed_indirect,AND,6,"AND {IDX}");
    addopcode(0x31,addr_indirect_indexed,AND,5,"AND {IDY}");

    /* logical inclusive or */
    addopcode(0x09,addr_immediate,ORA,2,"ORA {imm}");
    addopcode(0x05,addr_zero_page,ORA,3,"ORA {ZP0}");
    addopcode(0x15,addr_zero_page_x,ORA,4,"ORA {ZPX}");
    addopcode(0x0D,addr_absolute,ORA,4,"ORA {ABS}");
    addopcode(0x1D,addr_absolute_x,ORA,4,"ORA {ABX}");
    addopcode(0x19,addr_absolute_y,ORA,4,"ORA {ABY}");
    addopcode(0x01,addr_indexed_indirect,ORA,6,"ORA {IDX}");
    addopcode(0x11,addr_indirect_indexed,ORA,5,"ORA {IDY}");

    /* Bit Test */

    addopcode(0x24,addr_zero_page,BIT,3,"BIT {ZP}");
    addopcode(0x2C,addr_absolute,BIT,4,"BIT {ABS}");
}

void ARITHMETIC_INSTRUCTIONS(){
    //ADC
    addopcode(0x69,addr_immediate,ADC,2,"ADC {imm}");
    addopcode(0x65,addr_zero_page,ADC,3,"ADC {ZP0}");
    addopcode(0x75,addr_zero_page_x,ADC,4,"ADC {ZPX}");
    addopcode(0x6D,addr_absolute,ADC,4,"ADC {ABS}");
    addopcode(0x7D,addr_absolute_x,ADC,4,"ADC_ABX");
    addopcode(0x79,addr_absolute_y,ADC,4,"ADC {ABY}");
    addopcode(0x61,addr_indexed_indirect,ADC,6,"ADC {IDX}");
    addopcode(0x71,addr_indirect_indexed,ADC,5,"ADC {IDY}");
    //SBC
    addopcode(0xE9,addr_immediate,SBC,2,"SBC {imm}");
    addopcode(0xE5,addr_zero_page,SBC,3,"SBC {ZP0}");
    addopcode(0xF5,addr_zero_page_x,SBC,4,"SBC {ZPX}");
    addopcode(0xED,addr_absolute,SBC,4,"SBC {ABS}");
    addopcode(0xFD,addr_absolute_x,SBC,4,"SBC_ABX");
    addopcode(0xF9,addr_absolute_y,SBC,4,"SBC {ABY}");
    addopcode(0xE1,addr_indexed_indirect,SBC,6,"SBC {IDX}");
    addopcode(0xF1,addr_indirect_indexed,SBC,5,"SBC {IDY}");
    //CMP
    addopcode(0xC9,addr_immediate,CMP,2,"CMP {imm}");
    addopcode(0xC5,addr_zero_page,CMP,3,"CMP {ZP0}");
    addopcode(0xD5,addr_zero_page_x,CMP,4,"CMP {ZPX}");
    addopcode(0xCD,addr_absolute,CMP,4,"CMP {ABS}");
    addopcode(0xDD,addr_absolute_x,CMP,4,"CMP_ABX");
    addopcode(0xD9,addr_absolute_y,CMP,4,"CMP {ABY}");
    addopcode(0xC1,addr_indexed_indirect,CMP,6,"CMP {IDX}");
    addopcode(0xD1,addr_indirect_indexed,CMP,5,"CMP {IDY}");
    //CPX
    addopcode(0xE0,addr_immediate,CPX,2,"CPX {imm}");
    addopcode(0xE4,addr_zero_page,CPX,3,"CPX {ZP0}");
    addopcode(0xEC,addr_absolute,CPX,4,"CPX {ABS}");
    //CPY
    addopcode(0xC0,addr_immediate,CPY,2,"CPY {imm}");
    addopcode(0xC4,addr_zero_page,CPY,3,"CPY {ZP0}");
    addopcode(0xCC,addr_absolute,CPY,4,"CPY {ABS}");
}

void INCREMENT_DECREMENT_INSTRUCTIONS(){
    /* INC instruction*/
    addopcode(0xE6,addr_zero_page,INC,5,"INC {ZP0}");
    addopcode(0xF6,addr_zero_page_x,INC,6,"INC {ZPX}");
    addopcode(0xEE,addr_absolute,INC,6,"INC {ABS}");
    addopcode(0xFE,addr_absolute_x,INC,7,"INC {ABX}");

    /* INX instruction*/
    addopcode(0xE8,addr_implied,INX,2,"INX {IMP}");
    
    /* INY instruction*/
    addopcode(0xC8,addr_implied,INY,2,"INY {IMP}");

    /* DEC instruction */
    addopcode(0xC6,addr_zero_page,DEC,5,"DEC {ZP0}");
    addopcode(0xD6,addr_zero_page_x,DEC,6,"DEC {ZPX}");
    addopcode(0xCE,addr_absolute,DEC,6,"DEC {ABS}");
    addopcode(0xDE,addr_absolute_x,DEC,7,"DEC {ABX}");

    /* DEX instruction*/
    addopcode(0xCA,addr_implied,DEX,2,"DEX {IMP}");
    
    /* DEY instruction*/
    addopcode(0x88,addr_implied,DEY,2,"DEY {IMP}");
}

void SHIFT_INSTRUCTIONS(){
    /* Arithmetic Shift Left */
    addopcode(0x0A,addr_implied,ASL,2,"ASL {ACC}");
    addopcode(0x06,addr_zero_page,ASL,5,"ASL {ZP0}");
    addopcode(0x16,addr_zero_page_x,ASL,6,"ASL {ZPX}");
    addopcode(0x0E,addr_absolute,ASL,6,"ASL {ABS}");
    addopcode(0x1E,addr_absolute_x,ASL,7,"ASL {ABX}");

    /* Logical shift right */
    addopcode(0x4A,addr_implied,LSR,2,"LSR {ACC}");
    addopcode(0x46,addr_zero_page,LSR,5,"LSR {ZP0}");
    addopcode(0x56,addr_zero_page_x,LSR,6,"LSR {ZPX}");
    addopcode(0x4E,addr_absolute,LSR,6,"LSR {ABS}");
    addopcode(0x5E,addr_absolute_x,LSR,7,"LSR {ABX}");
    
    /* Rotate Left*/
    addopcode(0x2A,addr_implied,ROL,2,"ROL {ACC}");
    addopcode(0x26,addr_zero_page,ROL,5,"ROL {ZP0}");
    addopcode(0x36,addr_zero_page_x,ROL,6,"ROL {ZPX}");
    addopcode(0x2E,addr_absolute,ROL,6,"ROL {ABS}");
    addopcode(0x3E,addr_absolute_x,ROL,7,"ROL {ABX}");

    /* Rotate Right*/
    addopcode(0x6A,addr_implied,ROR,2,"ROR {ACC}");
    addopcode(0x66,addr_zero_page,ROR,5,"ROR {ZP0}");
    addopcode(0x76,addr_zero_page_x,ROR,6,"ROR {ZPX}");
    addopcode(0x6E,addr_absolute,ROR,6,"ROR {ABS}");
    addopcode(0x7E,addr_absolute_x,ROR,7,"ROR {ABX}");
}

void JUMP_CALLS_INSTRUCTIONS(){
    /* JMP Instructions*/
    addopcode(0x4C,addr_absolute,JMP,3,"JMP {ABS}");
    addopcode(0x6C,addr_indirect,JMP,5,"JMP {IND}");

    /* JSR Instructions */
    addopcode(0x20,addr_absolute,JSR,6,"JSR {ABS}");

    /* RTS Instructions */
    addopcode(0x60, addr_implied,RTS,6,"RTS {IMP}");
}

void BRANCHES_INSTRUCTIONS(){
    addopcode(0x90,addr_relative,BCC,2,"BCC {REL}");
    addopcode(0xB0,addr_relative,BCS,2,"BCS {REL}");
    addopcode(0xF0,addr_relative,BEQ,2,"BEQ {REL}");
    addopcode(0x30,addr_relative,BMI,2,"BMI {REL}");
    addopcode(0xD0,addr_relative,BNE,2,"BNE {REL}");
    addopcode(0x10,addr_relative,BPL,2,"BPL {REL}");
    addopcode(0x50,addr_relative,BVC,2,"BVC {REL}");
    addopcode(0x70,addr_relative,BVS,2,"BVS {REL}");
}

void STATUS_FLAG_INSTRUCTIONS(){

    addopcode(0x18,addr_implied,CLC,2,"CLC {IMP}");
    addopcode(0xD8,addr_implied,CLD,2,"CLD {IMP}");
    addopcode(0x58,addr_implied,CLI,2,"CLI {IMP}");
    addopcode(0xB8,addr_implied,CLV,2,"CLV {IMP}");
    addopcode(0x38,addr_implied,SEC,2,"SEC {IMP}");
    addopcode(0xF8,addr_implied,SED,2,"SED {IMP}");
    addopcode(0x78,addr_implied,SEI,2,"SEI {IMP}");
}

void SYSTEM_INSTRUCTIONS(){
    addopcode(0x00,addr_implied,BRK,7,"BRK {IMP}");
    addopcode(0xEA,addr_implied,NOP,2,"NOP {IMP}");
    addopcode(0x40,addr_implied,RTI,6,"RTI {IMP}");

}

void ILLEGAL_OPCODES(){
    addopcode(0x1A,addr_implied,NOP,2,"NOP {IMP}");
}
/* CPU Initialization */
/**
 * @brief Initializes the CPU.
 */
void cpu_init() {
    LOADSTORE_INSTRUCTIONS();
    REGISTERTRANSFER_INSTRUCTIONS();
    STACK_OPERATIONS_INSTRUCTIONS();
    LOGICAL_OPERATIONS();
    ARITHMETIC_INSTRUCTIONS();
    INCREMENT_DECREMENT_INSTRUCTIONS();
    SHIFT_INSTRUCTIONS();
    JUMP_CALLS_INSTRUCTIONS();
    BRANCHES_INSTRUCTIONS();
    SYSTEM_INSTRUCTIONS();
    STATUS_FLAG_INSTRUCTIONS();
    // ILLEGAL_OPCODES();
    SP = 0xFF;
    PC = 0x8000;
}

/* CPU Clock Cycle */
/**
 * @brief Executes a single CPU clock cycle.
 */
void clock_cpu() {
    
    if(cycles == 0){
        cpu_read(PC++,&opcode);
        printf("PC %x - opcode: %x: ",PC,opcode);
        instructions_t decode = opcodetable[opcode];
        cycles = decode.cycles;
        /* addressing mode*/
        decode.address_mode();
        /* instruction*/
        decode.instruction();

        /* print out instruction*/
        printf("%s\n",decode.name);
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

Test(Instructions,EOR){
    cpu_init();
    cpu_write(0,0x49);
    cpu_write(1,0x45);
    PC = 0;
    A = 0x45;
    clock_cpu();
    cr_assert_eq(A,0,"TYA test flag - FAILED!");
}

Test(Instructions,AND){
    cpu_init();
    cpu_write(0,0x29);
    cpu_write(1,0x7);
    PC = 0;
    A = 0x1F;
    clock_cpu();
    cr_assert_eq(A,7,"AND - failed!");
}

Test(Instructions,ORA){
    cpu_init();
    cpu_write(0,0x09);
    cpu_write(1,0x1);
    PC = 0;
    A = 14;
    clock_cpu();
    cr_assert_eq(A,15,"ORA - failed!");
}

Test(Instructions,BIT){
    cpu_init();
    cpu_write(0,0x24);
    cpu_write(0x25,0x40);
    cpu_write(1,0x25);
    PC = 0;
    A = 0xFF;
    clock_cpu();
    cr_assert_eq(state.V,1,"TEST - failed! %d",state.V);
}

Test(Instructions,BIT1){
    cpu_init();
    cpu_write(0,0x24);
    cpu_write(0x25,0x80);
    cpu_write(1,0x25);
    PC = 0;
    A = 0xFF;
    clock_cpu();
    cr_assert_eq(state.S,1,"TEST - failed! %d",state.V);
}

Test(Instructions,ADC){
    cpu_init();
    cpu_write(0,0x69);
    cpu_write(1,0x1);
    PC = 0;
    A = 0x7F;
    clock_cpu();
    cr_assert_eq(state.S,1,"ADC signed flag - failed! %d",state.V);
}

Test(Instructions,ADC1){
    cpu_init();
    cpu_write(0,0x69);
    cpu_write(1,0x1);
    PC = 0;
    A = 0x7F;
    clock_cpu();
    cr_assert_eq(state.V,1,"TEST - failed! %d",state.V);
}

Test(Instructions,ADC2){
    cpu_init();
    cpu_write(0,0x69);
    cpu_write(1,0x1);
    PC = 0;
    A = 0xFF;
    clock_cpu();
    cr_assert_eq(state.C,1,"TEST - failed! %d",state.V);
}

Test(Instructions,ADC3){
    cpu_init();
    cpu_write(0,0x69);
    cpu_write(1,0x1);
    PC = 0;
    A = 0x45;
    state.C = 1;
    clock_cpu();
    cr_assert_eq(A,0x47,"TEST - failed! %d",state.V);
}


Test(Instructions,SBC){
    cpu_init();
    cpu_write(0,0xE9);
    cpu_write(1,0x1);
    PC = 0;
    A = 0x0;
    clock_cpu();
    cr_assert_eq(state.S,1,"SBC: Signed flag test failed!");
}

Test(Instructions,SBC1){
    cpu_init();
    cpu_write(0,0xE9);
    cpu_write(1,0x1);
    PC = 0;
    A = 0x80;
    clock_cpu();
    cr_assert_eq(state.V,1,"SBC: Overflow flag test failed!");
}

Test(Instructions,SBC2){
    cpu_init();
    cpu_write(0,0xE9);
    cpu_write(1,0x1);
    PC = 0;
    A = 0x45;
    state.C = 1;
    clock_cpu();
    cr_assert_eq(A,0x44,"SBC: subtraction failed!");
}

Test(Instruction,CMP){
    cpu_init();
    cpu_write(0,0xC9);
    cpu_write(1,0x45);
    A = 0x45;
    PC = 0;
    clock_cpu();
    cr_assert_eq(state.Z,1,"CMP ZF - Failed!");
}

Test(Instruction,CMP1){
    cpu_init();
    cpu_write(0,0xC9);
    cpu_write(1,0x45);
    A = 0x46;
    PC = 0;
    clock_cpu();
    cr_assert_eq(state.C,1,"CMP CF - Failed!");
}

Test(Instruction,CMP2){
    cpu_init();
    cpu_write(0,0xC9);
    cpu_write(1,0x45);
    A = 0x44;
    PC = 0;
    clock_cpu();
    cr_assert_eq(state.S,1,"CMP SF - Failed!");
}

Test(Instruction,CPX){
    cpu_init();
    cpu_write(0,0xE0);
    cpu_write(1,0x45);
    X = 0x45;
    PC = 0;
    clock_cpu();
    cr_assert_eq(state.Z,1,"CPX ZF - Failed!");
}

Test(Instruction,CPX1){
    cpu_init();
    cpu_write(0,0xE0);
    cpu_write(1,0x45);
    X = 0x46;
    PC = 0;
    clock_cpu();
    cr_assert_eq(state.C,1,"CPX CF - Failed!");
}

Test(Instruction,CPX2){
    cpu_init();
    cpu_write(0,0xE0);
    cpu_write(1,0x45);
    X = 0x44;
    PC = 0;
    clock_cpu();
    cr_assert_eq(state.S,1,"CPX SF - Failed!");
}

Test(Instruction,CPY){
    cpu_init();
    cpu_write(0,0xC0);
    cpu_write(1,0x45);
    Y = 0x45;
    PC = 0;
    clock_cpu();
    cr_assert_eq(state.Z,1,"CPX ZF - Failed!");
}

Test(Instruction,CPY1){
    cpu_init();
    cpu_write(0,0xC0);
    cpu_write(1,0x45);
    Y = 0x46;
    PC = 0;
    clock_cpu();
    cr_assert_eq(state.C,1,"CPX CF - Failed!");
}

Test(Instruction,CPY2){
    cpu_init();
    cpu_write(0,0xC0);
    cpu_write(1,0x45);
    Y = 0x44;
    PC = 0;
    clock_cpu();
    cr_assert_eq(state.S,1,"CPX SF - Failed!");
}

Test(Instruction, INC){
    cpu_init();
    cpu_write(0,0xE6);
    cpu_write(0x45,0x1);
    cpu_write(1,0x45);
    PC = 0x00;
    clock_cpu();
    uint8_t byte;
    cpu_read(0x45,&byte);
    cr_assert_eq(byte,0x2,"INC Instruction - Failed!");
}

Test(Instruction,INX){
    cpu_init();
    cpu_write(0x00,0xE8);
    PC = 0;
    X = 0x45;
    clock_cpu();
    cr_assert_eq(X,0x46,"INX Instruction - Failed!");
}

Test(Instruction,INY){
    cpu_init();
    cpu_write(0x00,0xC8);
    PC = 0;
    Y = 0x45;
    clock_cpu();
    cr_assert_eq(Y,0x46,"INX Instruction - Failed!");
}

Test(Instruction, DEC){
    cpu_init();
    cpu_write(0,0xC6);
    cpu_write(0x45,0x45);
    cpu_write(1,0x45);
    PC = 0x00;
    clock_cpu();
    uint8_t byte;
    cpu_read(0x45,&byte);
    cr_assert_eq(byte,0x44,"DEC Instruction - Failed! Actual: %x, expected: 0x44",byte);
}

Test(Instruction,DEX){
    cpu_init();
    cpu_write(0x00,0xCA);
    PC = 0;
    X = 0x45;
    clock_cpu();
    cr_assert_eq(X,0x44,"DEX  Instruction - Failed!");
}

Test(Instruction,DEY){
    cpu_init();
    cpu_write(0x00,0x88);
    PC = 0;
    Y = 0x45;
    clock_cpu();
    cr_assert_eq(Y,0x44,"DEY  Instruction - Failed!");
}

Test(Instruction,ASL){
    cpu_init();
    cpu_write(0,0x0A);
    PC = 0;
    A = 0x7;
    clock_cpu();
    cr_assert_eq(A,14,"ASL Instruction - FAILED!");
}

Test(Instruction,LSR){
    cpu_init();
    cpu_write(0,0x4A);
    PC = 0;
    A = 0xFF;
    clock_cpu();
    cr_assert_eq(A,0x7F,"LSR Instruction - FAILED!");
}

Test(Instruction,ROL){
    cpu_init();
    cpu_write(0,0x2A);
    PC = 0;
    A = 0xFF;
    state.C = 1;
    clock_cpu();
    cr_assert_eq(A,0xFF,"ROL Instruction - FAILED!");
}

Test(Instruction,ROL1){
    cpu_init();
    cpu_write(0,0x2A);
    PC = 0;
    A = 0xFF;
    state.C = 0;
    clock_cpu();
    cr_assert_eq(A,0xFE,"ROL Instruction - FAILED!");
}

Test(Instruction,ROR){
    cpu_init();
    cpu_write(0,0x6A);
    PC = 0;
    A = 0xFF;
    state.C = 1;
    clock_cpu();
    cr_assert_eq(A,0xFF,"ROL Instruction - FAILED!");
}

Test(Instruction,ROR1){
    cpu_init();
    cpu_write(0,0x6A);
    PC = 0;
    A = 0xFF;
    state.C = 0;
    clock_cpu();
    cr_assert_eq(A,0x7F,"ROL Instruction - FAILED!");
}

Test(Instruction,JMP){
    cpu_init();
    cpu_write(0,0x4C);
    cpu_write(1,0xAD);
    cpu_write(2,0xDE);
    PC = 0;
    clock_cpu();
    cr_assert_eq(PC,0xDEAD,"JMP{ABS} Instruction - FAILED!");
}

Test(Instruction,JMP1){
    cpu_init();
    cpu_write(0,0x6C);
    cpu_write(1,0xAD);
    cpu_write(2,0xDE);
    cpu_write(0xDEAD,0xEF);
    cpu_write(0xDEAD + 1,0xBE);
    PC = 0;
    clock_cpu();
    cr_assert_eq(PC,0xBEEF,"JMP{IND} Instruction - FAILED! Expected: 0xBEEF, Actual: %x",PC);
}

Test(Instruction,JSRRTS){
    cpu_init();
    cpu_write(0,0x20);
    cpu_write(1,0xEF);
    cpu_write(2,0xBE);
    cpu_write(0xBEEF,0x60);
    for(int i = 0; i < 5; i++){
        clock_cpu();
    }
    char success1 = PC == 0xBEEF;
    for(int i = 0; i < 5; i++){
        clock_cpu();
    }
    char success = ((PC == 0x3) && (SP == 0xFF));
    cr_assert_eq(success && success1,1,"JSR - RTS Instruction - FAILED!");
}

Test(Instruction,BCC){
    PC = 0;
    rel_addr = 4;
    BCC();
    cr_assert_eq(PC,4,"BCC - FAILED!");
}

Test(Instruction,BCC1){
    PC = 100;
    rel_addr = -4;
    BCC();
    cr_assert_eq(PC,96,"BCC - FAILED!");
}
#endif
