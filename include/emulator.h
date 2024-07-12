#pragma once
#include<string>
#include<memory>
#include<cstdint>
#include<SDL2/SDL.h>
class CPU;
class NES{
    public:
    NES();
    ~NES();
      bool run(const std::string& rom, uint8_t scale);
      uint8_t cpuread(uint16_t address);
      void cpuwrite(uint16_t address, uint8_t byte);
    private:
      // std::unique_ptr<uint8_t[]> memory;
      std::shared_ptr<CPU> cpu;
      bool initialize(uint8_t scale);
      std::unique_ptr<uint8_t[]> memory;
      SDL_Window* window = nullptr;
      SDL_Renderer* renderer = nullptr;


};