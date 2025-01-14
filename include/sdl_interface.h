
#ifndef SDL_INTERFACE_H
#define SDL_INTERFACE_H


#define SCREEN_WIDTH 128
#define SCREEN_HEIGHT 256

#include<stdint.h>
uint8_t init_SDL();
void render_frame(uint32_t*);
void exit_SDL();

#endif // SDL_INTERFACE_H