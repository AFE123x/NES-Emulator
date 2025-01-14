#include "../include/sdl_interface.h"
#include <SDL.h>
#include <stdio.h>

static SDL_Window *screen = NULL;
static SDL_Renderer *renderer = NULL;
static SDL_Texture *texture = NULL;

static uint8_t clear_renderer(SDL_Renderer* renderer) {
    const uint8_t red = 0;
    const uint8_t green = 0;
    const uint8_t blue = 0;
    
    // Set draw color
    if(SDL_SetRenderDrawColor(renderer, red, green, blue, SDL_ALPHA_OPAQUE)) {
        printf("ERROR: Renderer: %s\n", SDL_GetError());
        return 1;
    }
    
    // Clear the renderer
    if(SDL_RenderClear(renderer)) {
        printf("ERROR: Renderer: %s\n", SDL_GetError());
        return 1;
    }

    // Present the renderer
    SDL_RenderPresent(renderer);
    return 0;
}

// Public functions
uint8_t init_SDL() {
    if (SDL_Init(SDL_INIT_EVERYTHING) < 0) {
        printf("ERROR: SDL failed to init: %s\n", SDL_GetError());
        return 1;
    }

    // Create window
    screen = SDL_CreateWindow("NES Emulator",
                              SDL_WINDOWPOS_UNDEFINED,
                              SDL_WINDOWPOS_UNDEFINED,
                              640, 480,
                              SDL_WINDOW_OPENGL);
    if (screen == NULL) {
        printf("ERROR: Failed to create SDL WINDOW %s\n", SDL_GetError());
        return 1;
    }

    // Renderer creation
    renderer = SDL_CreateRenderer(screen, -1, SDL_RENDERER_ACCELERATED);
    if (renderer == NULL) {
        printf("ERROR: Failed to create renderer: %s\n", SDL_GetError());
        return 1;
    }

    // Clear renderer
    if (clear_renderer(renderer)) return 1;

    // Set render scale quality
    if (SDL_SetHint(SDL_HINT_RENDER_SCALE_QUALITY, "linear") != SDL_TRUE) {
        printf("ERROR: Failed to set scale quality hint: %s\n", SDL_GetError());
        return 1;
    }

    // Set logical size
    if (SDL_RenderSetLogicalSize(renderer, SCREEN_WIDTH, SCREEN_HEIGHT)) {
        printf("ERROR: Set Logical Size: %s\n", SDL_GetError());
        return 1;
    }

    // Create texture for rendering
    texture = SDL_CreateTexture(
        renderer,
        SDL_PIXELFORMAT_ARGB8888,
        SDL_TEXTUREACCESS_STREAMING,
        SCREEN_WIDTH, SCREEN_HEIGHT
    );
    if (texture == NULL) {
        printf("ERROR: Texture creation: %s\n", SDL_GetError());
        return 1;
    }

    return 0;
}

void render_frame(uint32_t* pixel_data) {
    SDL_UpdateTexture(texture, NULL, (void*)pixel_data, SCREEN_WIDTH * 4);
    SDL_RenderClear(renderer);
    SDL_RenderCopy(renderer, texture, NULL, NULL);
    SDL_RenderPresent(renderer);
}

void exit_SDL() {
    // Cleanup SDL resources
    SDL_DestroyTexture(texture);
    SDL_DestroyRenderer(renderer);
    SDL_DestroyWindow(screen);
    SDL_Quit();
}
