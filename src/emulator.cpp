#include "../include/emulator.h"
#include "../include/2A03.h"
NES::NES() {}
NES::~NES() {
  // Clean up.
  SDL_DestroyRenderer(renderer);
  SDL_DestroyWindow(window);
  SDL_Quit();
}

/*
use this as a reference for the GUI: https://thenumb.at/cpp-course/sdl2/07/07.html
*/
bool NES::initialize(uint8_t scale) {
  // initialize SDL2 video
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
  SDL_DestroyRenderer(renderer);
  SDL_DestroyWindow(window);
  SDL_Quit();
  return true;
}
uint8_t NES::cpuread(uint16_t address) { return memory[address]; }
void NES::cpuwrite(uint16_t address, uint8_t byte) { memory[address] = byte; }
bool NES::run(const std::string &rom, uint8_t scale) {
  if (!initialize(scale)) {
    return false;
  }
  uint32_t array_size = 64 * 1024;
  memory = std::make_unique<uint8_t[]>(array_size);

  for (uint32_t i = 0; i < array_size; i++) {
    memory[i] = 0;
  }
  // a9 0a 85 00 a9 14 85 01 a5 00 85 02 a5 01 85 03
  memory[0x8000] = 0xA9;
  memory[0x8001] = 0xA;
  memory[0x8002] = 0x85;
  memory[0x8003] = 0x00;
  memory[0x8004] = 0xA9;
  memory[0x8005] = 0x14;
  memory[0x8006] = 0x85;
  memory[0x8007] = 0x01;
  memory[0x8008] = 0xA5;
  memory[0x8009] = 0x00;
  memory[0x800A] = 0x85;
  memory[0x800B] = 0x02;
  memory[0x800C] = 0xA5;
  memory[0x800D] = 0x01;
  memory[0x800E] = 0x85;
  memory[0x800F] = 0x03;
  memory[0xFFFC] = 0x00;
  memory[0xFFFD] = 0x80;
  cpu = std::make_shared<CPU>(this);

  while (1) {
    cpu->tick();
  }
  return true;
}
