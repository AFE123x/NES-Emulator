#include "../lib/bus.hpp"

Cpu::Cpu(Bus* bus){
    this->bus = bus;
    reset();
    initialize_loadstore_instructions();
}

Cpu::~Cpu(){

}

void Cpu::populate_opcode(uint8_t opcode, std::string name, uint8_t cycles, void (Cpu::*instruction)(void), void (Cpu::*addressing_mode)(void)){
    opcode_table[opcode].addressing_mode = addressing_mode;
    opcode_table[opcode].cycles = cycles;
    opcode_table[opcode].name = name;
    opcode_table[opcode].instruction = instruction;
}
void Cpu::initialize_loadstore_instructions(){
    populate_opcode(0xA9,"LDA {IMM}",2,&Cpu::LDA, &Cpu::IMM);
    populate_opcode(0xA5,"LDA {ZP0}",3,&Cpu::LDA, &Cpu::ZP0);
    populate_opcode(0xB5,"LDA {ZPX}",4,&Cpu::LDA, &Cpu::ZPX);
    populate_opcode(0xAD,"LDA {ABS}",4,&Cpu::LDA, &Cpu::ABS);
    populate_opcode(0xBD,"LDA {ABX}",4,&Cpu::LDA, &Cpu::ABX);
    populate_opcode(0xB9,"LDA {ABY}",4,&Cpu::LDA, &Cpu::ABY);
    populate_opcode(0xA1,"LDA {IDX}",6,&Cpu::LDA, &Cpu::IDX);
    populate_opcode(0xB1,"LDA {IDY}",5,&Cpu::LDA, &Cpu::IDY);

    populate_opcode(0xA2,"LDX {IMM}",2,&Cpu::LDX, &Cpu::IMM);
    populate_opcode(0xA6,"LDX {ZP0}",3,&Cpu::LDX, &Cpu::ZP0);
    populate_opcode(0xB6,"LDX {ZPY}",4,&Cpu::LDX, &Cpu::ZPY);
    populate_opcode(0xAE,"LDX {ABS}",4,&Cpu::LDX, &Cpu::ABS);
    populate_opcode(0xBE,"LDX {ABY}",4,&Cpu::LDX, &Cpu::ABY);

    populate_opcode(0xA0,"LDY {IMM}",2,&Cpu::LDY, &Cpu::IMM);
    populate_opcode(0xA4,"LDY {ZP0}",3,&Cpu::LDY, &Cpu::ZP0);
    populate_opcode(0xB4,"LDY {ZPX}",4,&Cpu::LDY, &Cpu::ZPX);
    populate_opcode(0xAC,"LDY {ABS}",4,&Cpu::LDY, &Cpu::ABS);
    populate_opcode(0xBC,"LDY {ABX}",4,&Cpu::LDY, &Cpu::ABX);

    populate_opcode(0x85,"STA {ZP0}",3,&Cpu::STA, &Cpu::ZP0);
    populate_opcode(0x95,"STA {ZPX}",4,&Cpu::STA, &Cpu::ZPX);
    populate_opcode(0x8D,"STA {ABS}",4,&Cpu::STA, &Cpu::ABS);
    populate_opcode(0x9D,"STA {ABX}",5,&Cpu::STA, &Cpu::ABX);
    populate_opcode(0x99,"STA {ABY}",5,&Cpu::STA, &Cpu::ABY);
    populate_opcode(0x81,"STA {IDX}",6,&Cpu::STA, &Cpu::IDX);
    populate_opcode(0x91,"STA {IDY}",6,&Cpu::STA, &Cpu::IDY);

    populate_opcode(0x86,"STX {ZP0}", 3, &Cpu::STX, &Cpu::ZP0);
    populate_opcode(0x96,"STX {ZPY}", 4, &Cpu::STX, &Cpu::ZPY);
    populate_opcode(0x8E,"STX {ABS}", 4, &Cpu::STX, &Cpu::ABS);

    populate_opcode(0x84,"STY {ZP0}", 3, &Cpu::STX, &Cpu::ZP0);
    populate_opcode(0x94,"STY {ZPX}", 4, &Cpu::STX, &Cpu::ZPX);
    populate_opcode(0x8C,"STY {ABS}", 4, &Cpu::STX, &Cpu::ABS);

}

void Cpu::clock(){
    if(cycles_left == 0){
        uint8_t opcode;
        bus->cpu_read(PC++,opcode);

        instructions_t decoded = opcode_table[opcode];
        (this->*decoded.addressing_mode)();
        (this->*decoded.instruction)();

        cycles_left = decoded.cycles;

    }
    total_cycles++;
    cycles_left--;
}

void Cpu::reset(){
    A = X = Y = 0;
    SP = 0xFD;
    PC = 0x8000;
    cycles_left = 0;
    total_cycles = 0;
    status_reg.raw = 0;
}