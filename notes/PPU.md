# PPU Notes

## Drawing

- The PPU renders a 256x240 region.
    - There does exist a border oxtending 16 pixels left, 11 pixels right, and 2 pixels down, therefore the full size is 283 x 242 pixels.

- We get the picture region from doing memory fetches and filling shift registers.
    - A multiplexer will pick the pixel to draw to the screen.


- To go about rendering the backround in the picture region, the PPU will do memory fetches on dots 321 - 336 and 1-256 at scanline 0-239 and 261 (We'll make it -1).
    - Each of these memory fetches takes 2 dots.
        - First dot will place the address on the ppu bus, and the second dot gets the data.
    - On every eighth dot, we will read the pattern table, attribute, and store them into the shift registers.

- For every dot, we'll use the fine x register to select a bit from the shift register, then shift it.


## Circuit Timing

- The PPU Renders 262 Scanlines a frame.
    - Each scanline lasts 341 cycles.

## Pre render Scanline

- Nothing is rendered at this scanline, only purpose is to fill the shift register with data. 
- We do reads as usual, using the v register
- The scanline depends on whether we're on an even or odd frame.
    - On odd frames, we skip the cycle at the end of the scanline (we skip cycle 340).
- During pixels 280 through 304 of this scanline, the vertical scroll bits are reloaded if rendering is enabled.

## Visible Scanline (0-239)

- What we actually see.
- During this time, the PPU is fetching data, so the CPU shouldn't access the PPU memory, unless the rendering is disabled.

### Cycle 0

- Cycle 0 is skipped, it's an idle cycle.

### Cycle 1 to 256

- Here, we do our 4 memory accesses.
    - nametable, attribute, pattern hi and lo.
- We reload the registers every 8 cycles. (1,9,17,etc.)


### Cycle 257 - 320

- This is where we fetch the tile data for the sprites on the next scanline.
    - We read garbage nametable values, but we get the pattern table tile low and high bytes.

**come back to this**

### Cycle 321 - 336

- Here, we will retrieve the first two tiles of the next scanline. We load the nametable, attribute then the high and lo bytes. We store the result in the shift register.

### Cycle 337 - 340
- We don't really need this realistically, but we need to mimic the behavior of the odd frame.

## Post Render scanline (240)

- Here, we basically do nothing.

## VBlank scanlines (241 - 260)

- Here, we enable the vblank at cycle 1 at scanline 241.
- Besides that, the PPU does nothing.e