#include "../lib/bus.hpp"
#include<iostream>

Bus::Bus(){
    memory.resize(65535);
    cpu = std::make_unique<Cpu>(this);
}

Bus::~Bus(){

}

void Bus::cpu_read(uint16_t address, uint8_t& byte){
    byte = memory[address];
}

void Bus::cpu_write(uint16_t address, uint8_t byte){
    memory[address] = byte;
}

void Bus::clock(){
    cpu->clock();
}