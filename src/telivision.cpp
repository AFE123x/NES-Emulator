#include "../lib/telivision.h"
#include <iostream>
television::television() {}
television::~television() {}
bool television::initialize(uint16_t width, uint8_t height) {
  if (SDL_Init(SDL_INIT_VIDEO) < 0) {
    std::cerr << "SDL could not initialize! SDL_Error: " << SDL_GetError()
              << std::endl;
    return false;
  }

  if (SDL_CreateWindowAndRenderer(width, height, 0, &window, &renderer) < 0) {
    std::cerr << "Window and renderer could not be created! SDL_Error: "
              << SDL_GetError() << std::endl;
    return false;
  }

  if (SDL_RenderSetScale(renderer, 4, 4) < 0) {
    std::cerr << "Renderer scaling could not be set! SDL_Error: "
              << SDL_GetError() << std::endl;
    return false;
  }

  if (SDL_SetRenderDrawColor(renderer, 0, 0, 0, 255) < 0) {
    std::cerr << "Render draw color could not be set! SDL_Error: "
              << SDL_GetError() << std::endl;
    return false;
  }

  if (SDL_RenderClear(renderer) < 0) {
    std::cerr << "Render could not be cleared! SDL_Error: " << SDL_GetError()
              << std::endl;
    return false;
  }
  return true;
}
void television::drawpoint(uint8_t r, uint8_t g, uint8_t b, uint16_t x,
                           uint16_t y) {
  SDL_SetRenderDrawColor(renderer, r, g, b, 255); // Set draw color to white
  SDL_RenderDrawPoint(renderer, x, y);
}
void television::refresh() { SDL_RenderPresent(renderer); }

void television::tvclose() {
  SDL_DestroyRenderer(renderer); // Clean up renderer
  SDL_DestroyWindow(window);     // Clean up window
  SDL_Quit();                    // Quit SDL
}