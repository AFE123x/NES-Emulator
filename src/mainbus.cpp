#include "./../lib/CPU.h"
#include "./../lib/mainbus.h"


        mainbus::mainbus(){
            ram = new uint8_t[65535];
        }
        mainbus::~mainbus(){
            delete[] ram;
        }
        uint8_t mainbus::read(uint16_t address){
            return ram[address];
        }
        uint8_t mainbus::write(uint16_t address, uint8_t data){
            ram[address] = data;
        }