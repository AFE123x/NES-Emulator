#ifndef MAINBUS_H
#define MAINBUS_H
#include<cstdint>
#include "./CPU.h"
    class mainbus{
        public:
        mainbus();
        ~mainbus();
        uint8_t read(uint16_t address);
        uint8_t write(uint16_t address, uint8_t data);
        private:
            uint8_t* ram;
            CPU* cpu;

    };
#endif