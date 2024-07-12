#include "../include/emulator.h"
#include "../include/2A03.h"
NES::NES() {
}
NES::~NES() {
  // Clean up.
  SDL_DestroyRenderer(renderer);
  SDL_DestroyWindow(window);
  SDL_Quit();
}
bool NES::initialize(uint8_t scale) {
  //initialize SDL2 video
  if (SDL_Init(SDL_INIT_VIDEO) != 0) {
    SDL_Log("Unable to initialize SDL: %s", SDL_GetError());
    return false;
  }

  // Create the SDL window and renderer.
  window =
      SDL_CreateWindow("NES Emulator", SDL_WINDOWPOS_CENTERED,
                       SDL_WINDOWPOS_CENTERED, 256 * scale, 240 * scale, 0);
  if (!window) {
    SDL_Log("Could not create window: %s", SDL_GetError());
    SDL_Quit();
    return false;
  }

  renderer = SDL_CreateRenderer(window, -1, SDL_RENDERER_ACCELERATED);
  if (!renderer) {
    SDL_Log("Could not create renderer: %s", SDL_GetError());
    SDL_DestroyWindow(window);
    SDL_Quit();
    return false;
  }

  // Set the renderer scale.
  if (SDL_RenderSetScale(renderer, scale, scale) != 0) {
    SDL_Log("Could not set renderer scale: %s", SDL_GetError());
    SDL_DestroyRenderer(renderer);
    SDL_DestroyWindow(window);
    SDL_Quit();
    return false;
  }

  // Set the default color to blue and clear the renderer.
  SDL_SetRenderDrawColor(renderer, 0, 0, 255, 255);
  SDL_RenderClear(renderer);

  // Set the draw color to white and draw a point at the center.
  SDL_SetRenderDrawColor(renderer, 255, 255, 255, 255);

  for (int i = 0; i < 240; i++) {
    SDL_RenderDrawPoint(renderer, i, i);
    SDL_RenderPresent(renderer);
    SDL_Delay(10);
  }
  // Delay for 10 seconds.
  SDL_Delay(2000);

  return true;
}
uint8_t NES::cpuread(uint16_t address){
    return memory[address];
}
void NES::cpuwrite(uint16_t address, uint8_t byte){
    memory[address] = byte;
}
bool NES::run(const std::string &rom, uint8_t scale) {
  if (!initialize(scale)) {
    return false;
  }
  cpu = std::make_shared<CPU>(this);
  uint32_t array_size = 64 * 1024;
  memory = std::make_unique<uint8_t[]>(array_size);
  for(int i = 0; i < array_size; i++){
    memory[i] = 0;
  }
  return true;
}
