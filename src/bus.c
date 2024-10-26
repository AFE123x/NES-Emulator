#include "../include/bus.h"
#include "../include/cpu.h"
//includes
#include<stdlib.h>
#include<assert.h>
#include<fcntl.h>
#include<unistd.h>
static uint8_t* memory;
static int romfd;

void initializebus(){
    memory = malloc(sizeof(uint8_t) * 0x10000);
    assert(memory);
    initializecpu();
}
void clockbus(){
    clock();
}
void freebus(){
    free(memory);
    close(romfd);
}
void loadrom(const char* path){
    romfd = open(path,O_RDONLY);
    assert(romfd != -1);
    int i = 0;
    while(read(romfd,&memory[i++],1));
}
void cpuread(uint16_t address, uint8_t* byte){
    *byte = memory[address];
}

void cpuwrite(uint16_t address, uint8_t byte){
    memory[address] = byte;
}