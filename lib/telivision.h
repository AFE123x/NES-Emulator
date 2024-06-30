#pragma once
#include <SDL2/SDL.h>
class television {

public:
  television();
  ~television();
  bool initialize(uint16_t width, uint8_t height);
  void drawpoint(uint8_t r, uint8_t g, uint8_t b, uint16_t x, uint16_t y);
  void refresh();
  void tvclose();
private:
  SDL_Window *window = nullptr;
  SDL_Renderer *renderer = nullptr;
};