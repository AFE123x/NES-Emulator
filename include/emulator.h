#pragma once
#include <SDL2/SDL.h>
#include <SDL2/SDL_ttf.h>
#include <cstdint>
#include <memory>
#include <string>
class CPU;
class NES {
public:
  NES();
  ~NES();
  bool run(const std::string &rom, uint8_t scale);
  uint8_t cpuread(uint16_t address);
  void cpuwrite(uint16_t address, uint8_t byte);
  void updateregisters();

private:
  bool PrintText(char *buf, SDL_Color color, int X, int Y);
  void setstatusregister();
  TTF_Font *font;
  std::shared_ptr<CPU> cpu;
  bool initialize(uint8_t scale);
  std::unique_ptr<uint8_t[]> memory;
  SDL_Window *window = nullptr;
  SDL_Renderer *renderer = nullptr;
  uint8_t cache[16][16];
};