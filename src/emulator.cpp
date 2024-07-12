#include "../include/emulator.h"
#include "../include/2A03.h"
#include <cstdio>
#include <iostream>
NES::NES() {}
NES::~NES() {
  // Clean up.
  SDL_DestroyRenderer(renderer);
  SDL_DestroyWindow(window);
  SDL_Quit();
}

/*
use this as a reference for the GUI:
https://thenumb.at/cpp-course/sdl2/07/07.html
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
                       SDL_WINDOWPOS_CENTERED, 640 * scale, 360 * scale, 0);
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

  // initialize text
  if (TTF_Init() < 0) {
    SDL_Log("Error initializing SDL_ttf: %s", TTF_GetError());
    SDL_DestroyRenderer(renderer);
    SDL_DestroyWindow(window);
    SDL_Quit();
  }

  font = TTF_OpenFont("/home/afe123x/Documents/projects/NES-Emulator/src/fonts/"
                      "Retro Gaming.ttf",
                      12);
  if (!font) {
    SDL_Log("Failed to load font: %s", TTF_GetError());
    SDL_DestroyRenderer(renderer);
    SDL_DestroyWindow(window);
    SDL_Quit();
    return false;
  }
  return true;
}
bool NES::PrintText(char *buf, SDL_Color color, int X, int Y) {

  // Set color to red

  // Render text to surface
  SDL_Surface *text = TTF_RenderText_Solid(font, buf, color);
  if (!text) {
    std::cerr << "Failed to render text: " << TTF_GetError() << std::endl;
    return false;
  }

  // Create texture from the rendered surface
  SDL_Texture *text_texture = SDL_CreateTextureFromSurface(renderer, text);
  if (!text_texture) {
    std::cerr << "Failed to create texture: " << SDL_GetError() << std::endl;
    SDL_FreeSurface(text); // Clean up the surface
    return false;
  }

  // Set the destination rectangle
  SDL_Rect dest = {X, Y, text->w, text->h};

  // Copy texture to renderer
  SDL_RenderCopy(renderer, text_texture, nullptr, &dest);
  SDL_RenderPresent(renderer); // Present the renderer (if necessary)

  // Clean up resources
  SDL_DestroyTexture(text_texture);
  SDL_FreeSurface(text);
  SDL_Delay(5);
  return true; // Return true if everything succeeds
}
void NES::setstatusregister() {
  char buf[3];
  uint8_t status = cpu->flag_register.data;
  SDL_Color color;
  // Negative flag (N)
  color = (status & 0x80) ? SDL_Color{255, 0, 0, 255}
                          : SDL_Color{255, 255, 255, 255};
  sprintf(buf, "N");
  PrintText(buf, color, 510, 40);

  // Overflow flag (V)
  color = (status & 0x40) ? SDL_Color{255, 0, 0, 255}
                          : SDL_Color{255, 255, 255, 255};
  sprintf(buf, "V");
  PrintText(buf, color, 525, 40);

  // Unused flag (-)
  color = (status & 0x20) ? SDL_Color{255, 0, 0, 255}
                          : SDL_Color{255, 255, 255, 255};
  sprintf(buf, "-");
  PrintText(buf, color, 540, 40);

  // Break flag (B)
  color = (status & 0x10) ? SDL_Color{255, 0, 0, 255}
                          : SDL_Color{255, 255, 255, 255};
  sprintf(buf, "B");
  PrintText(buf, color, 555, 40);

  // Decimal mode flag (D)
  color = (status & 0x8) ? SDL_Color{255, 0, 0, 255}
                         : SDL_Color{255, 255, 255, 255};
  sprintf(buf, "D");
  PrintText(buf, color, 570, 40);

  // IRQ disable flag (I)
  color = (status & 0x4) ? SDL_Color{255, 0, 0, 255}
                         : SDL_Color{255, 255, 255, 255};
  sprintf(buf, "I");
  PrintText(buf, color, 585, 40);

  // Zero flag (Z)
  color = (status & 0x2) ? SDL_Color{255, 0, 0, 255}
                         : SDL_Color{255, 255, 255, 255};
  sprintf(buf, "Z");
  PrintText(buf, color, 600, 40);

  // Carry flag (C)
  color = (status & 0x1) ? SDL_Color{255, 0, 0, 255}
                         : SDL_Color{255, 255, 255, 255};
  sprintf(buf, "C");
  PrintText(buf, color, 615, 40);
}
void NES::updateregisters() {
  // Clear the renderer
  SDL_SetRenderDrawColor(renderer, 0, 0, 255, 255);
  SDL_RenderClear(renderer);

  // Set color for text rendering
  SDL_Color color = {255, 255, 255, 255};

  // Render PC and SP
  char buf[100];
  sprintf(buf, "PC: %x  SP: %x", cpu->PC, cpu->SP);
  PrintText(buf, color, 510, 10);

  // Render A, X, Y registers
  sprintf(buf, "A: %x  X: %X  Y: %x", cpu->A, cpu->X, cpu->Y);
  PrintText(buf, color, 510, 25);

  // Render status flags individually

  setstatusregister(); // set the status registers
  strcpy(buf, cpu->current_instruction.c_str());
  // std::cout<<buf<<std::endl;
  if (cpu->current_instruction == "") {
    sprintf(buf, "NOP");
  }
  PrintText(buf, color, 510, 55);

  // print out cycles

  sprintf(buf, "cycles: %d  total: %ld", cpu->cycles, cpu->total_cycles);
  PrintText(buf, color, 490, 70);

  if (cpu->memorychanged) {
    for (int j = 0; j < 16; j++) {
      for (int i = 0; i < 16; i++) {
        cache[i][j] = memory[(i << 4) + j];
        snprintf(buf, 3, "%X", cache[i][j]);
        PrintText(buf, color, 10 + j * 20, 10 + i * 20);
      }
    }
    cpu->memorychanged = false;
  } else {
    for (int j = 0; j < 16; j++) {
      for (int i = 0; i < 16; i++) {
        snprintf(buf, 3, "%X", cache[i][j]);
        PrintText(buf, color, 10 + j * 20, 10 + i * 20);
      }
    }
  }

  SDL_RenderPresent(renderer);
}

uint8_t NES::cpuread(uint16_t address) { return memory[address]; }
void NES::cpuwrite(uint16_t address, uint8_t byte) { memory[address] = byte; }
bool NES::run(const std::string &rom, uint8_t scale) {
  std::cout << rom << std::endl;
  if (!initialize(scale)) {
    return false;
  }

  uint32_t array_size = 64 * 1024;
  memory = std::make_unique<uint8_t[]>(array_size);

  // Initialize memory
  for (uint32_t i = 0; i < array_size; i++) {
    memory[i] = 0;
  }

  // Set initial memory values
  memory[0x8000] = 0xA9;
  memory[0x8001] = 0x00;
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

  // Create CPU instance
  cpu = std::make_shared<CPU>(this);

  bool quit = false;
  SDL_Event e;
  updateregisters();
  SDL_Delay(50);
  SDL_RenderPresent(renderer);
  while (!quit) {
    while (SDL_PollEvent(&e) != 0) {
      if (e.type == SDL_QUIT) {
        quit = true;
      } else if (e.type == SDL_KEYDOWN) {
        // Check which key was pressed
        SDL_Keycode keyPressed = e.key.keysym.sym;
        if (keyPressed == SDLK_t) { //perform one clock tick.
          cpu->tick();
          updateregisters();
        }
        else if(keyPressed == SDLK_g){ //goto the next instruction
          cpu->skip();
          cpu->tick();
          updateregisters();
        }
      }
    }

    SDL_Delay(50);
  }

  // Cleanup resources
  SDL_DestroyRenderer(renderer);
  SDL_DestroyWindow(window);
  TTF_CloseFont(font);
  TTF_Quit();
  SDL_Quit();

  return true;
}
