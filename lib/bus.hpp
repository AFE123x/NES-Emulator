#pragma once
#include<vector>
#include<cstdint>
#include<memory>
#include "./cpu.hpp"
class Bus{
    public:
        Bus();
        ~Bus();

        //functions for the cpu to read data
        void cpu_read(uint16_t address, uint8_t& byte);
        void cpu_write(uint16_t address, uint8_t byte);

        void clock();
    private:
        std::vector<uint8_t> memory;
        std::unique_ptr<Cpu> cpu;

};