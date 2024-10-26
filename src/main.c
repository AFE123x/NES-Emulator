#include<stdio.h>
#include<assert.h>
#include "../include/bus.h"
#include "../include/cpu.h"
int main(int argc, char** argv){
    assert(argc >= 2);
    initializebus();
    loadrom(argv[1]);
    while(1){
        clockbus();
    }
}