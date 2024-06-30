#include <SDL2/SDL.h>
#include <iostream>
#include <random>

SDL_Window *window = nullptr;
SDL_Renderer *renderer = nullptr;

// Random number generator setup
std::random_device rd;
std::mt19937 gen(rd());
std::uniform_int_distribution<> dis(0, 1);

void initialize() {
  SDL_Init(SDL_INIT_VIDEO);
  SDL_CreateWindowAndRenderer(1280, 720, 0, &window, &renderer);
  SDL_RenderSetScale(renderer, 4, 4); // Scaling factor for the renderer
  SDL_SetRenderDrawColor(renderer, 0, 0, 0, 255); // Set draw color to black
  SDL_RenderClear(renderer);   // Clear the screen with the black color
  SDL_RenderPresent(renderer); // Present the cleared screen
}

void drawpoint(uint8_t r, uint8_t g, uint8_t b, uint16_t x, uint16_t y) {
  SDL_SetRenderDrawColor(renderer, r, g, b, 255); // Set draw color to white
  SDL_RenderDrawPoint(renderer, x, y);            // Draw the point
}

void drawstatic() {
  for (int i = 0; i < 1280; i++) { // Adjusted for scaling factor
    for (int j = 0; j < 720; j++) {
      if (dis(gen)) {
        drawpoint(249, 237, 204, i, j);
      } else {
        drawpoint(97, 33, 15, i, j);
      }
    }
  }
}
void refresh() { SDL_RenderPresent(renderer); }

void close() {
  SDL_DestroyRenderer(renderer); // Clean up renderer
  SDL_DestroyWindow(window);     // Clean up window
  SDL_Quit();                    // Quit SDL
}

void run() {
  // use as reference
  bool running = true;
  SDL_Event event;
  while (running) {

    while (SDL_PollEvent(&event)) {
      if (event.type == SDL_QUIT) {
        running = false;
      }
    }
    // Draw static
    drawstatic();
    refresh();

    SDL_Delay(50);
  }
}

int main() {
  initialize();
  run();
  close();

  return 0;
}
