#pragma once
#include <SDL2/SDL.h>
#include <SDL2/SDL_ttf.h>
#include <string>
class television {

public:
  television();
  ~television();
  bool initialize(uint16_t width, uint16_t height);
  void drawpoint(uint8_t r, uint8_t g, uint8_t b, uint16_t x, uint16_t y);
  bool drawdisassembly(std::string& dissasembly);
  void refresh();
  void tvclose();
  void renderText(SDL_Renderer *renderer, const std::string &text,
                             int x, int y, TTF_Font *font, SDL_Color color);

      private : SDL_Window *window = nullptr;
  SDL_Renderer *renderer = nullptr;
  TTF_Font *font = nullptr;
  SDL_Color textColor = {255, 255, 255, 255};
};