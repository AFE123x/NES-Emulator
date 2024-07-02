#include "../lib/telivision.h"
#include <iostream>
#include <sstream>
television::television() {}
television::~television() {}
bool television::initialize(uint16_t width, uint16_t height) {
  if (SDL_Init(SDL_INIT_VIDEO) < 0) {
    std::cerr << "SDL could not initialize! SDL_Error: " << SDL_GetError()
              << std::endl;
    return false;
  }

  if (SDL_CreateWindowAndRenderer(width * 2, height * 2, 0, &window, &renderer) < 0) {
    std::cerr << "Window and renderer could not be created! SDL_Error: "
              << SDL_GetError() << std::endl;
    goto SDLQUIT;
  }

  if (SDL_RenderSetScale(renderer, 2, 2) < 0) {
    std::cerr << "Renderer scaling could not be set! SDL_Error: "
              << SDL_GetError() << std::endl;
    goto DESTROYWINDOW;
  }
  if (TTF_Init() == -1) {
    std::cerr << "TTF_Init: " << SDL_GetError() << std::endl;
    goto DESTROYWINDOW;
  }
  font = TTF_OpenFont(
      "/usr/share/fonts/noto/NotoSerifDisplay-CondensedBlackItalic.ttf", 10);
  if (!font) {
    std::cerr << "Unable to open font: " << TTF_GetError() << std::endl;
    goto TTF_QUIT;
  }
  return true;
TTF_QUIT:
  TTF_Quit();
DESTROYWINDOW:
  SDL_DestroyWindow(window);
  SDL_DestroyRenderer(renderer);

SDLQUIT:
  SDL_Quit();
  return false;
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
  TTF_CloseFont(font);
  TTF_Quit();
}

void television::renderText(SDL_Renderer *renderer, const std::string &text,
                            int x, int y, TTF_Font *font, SDL_Color color) {
  std::cout << text << std::endl;
  std::istringstream stream(text);
  std::string line;
  int yOffset = 0;

  while (std::getline(stream, line)) {
    SDL_Surface *surface = TTF_RenderText_Solid(font, line.c_str(), color);
    SDL_Texture *texture = SDL_CreateTextureFromSurface(renderer, surface);

    SDL_Rect dstRect = {x, y + yOffset, surface->w, surface->h};
    SDL_RenderCopy(renderer, texture, nullptr, &dstRect);

    yOffset += surface->h;

    SDL_FreeSurface(surface);
    SDL_DestroyTexture(texture);
  }
}

bool television::drawdisassembly(std::string &dissasembly) {
  renderText(renderer, dissasembly, 510,17 , font, textColor);
  return true;
}
