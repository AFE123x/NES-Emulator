# NES Emulator

I'm developing an emulator for the Nintendo Entertainment System.

## About the NES

The Nintendo Entertainment System is an 8 bit game console released in 1983. It was intended to be a cheap and accessible console. The console had a huge influence on gaming as we know it today. Nintendo pioneered a new standard for licensing third-party developers to develop and distribute games. Although it's an outdated console, it still has a strong community of enthusiasts who wish to preserve the console/games, one way is through emulation.

## Architecture

### CPU

The NES uses the Ricoh 2A03 CPU, which is based on the 8 bit 6502 made by MOS technology. It runs at 1.79 MHz for ntsc. The 2A03 cpu lacked binary coded decimal mode, which was in the original 6502.

#### Memory

The Ricoh 2A03 and 6502 used an 8 bit data bus, and 16 bit address bus, but it didn't use all of it for memory. The NES had 2 KB of static ram, or "work ram" as called by nintendo. 

This memory was used for:
- variables
- stack
- buffer area


The other parts of the CPU are memory-mapped, which means we can access those parts with memory addresses.

## Cartridge

We obviously have a cartridge to play games. This is where your game is stored. The cartridge was designed so that we can access ~50 KB of cartridge data. Cartridge data consists of:
- Program ROM: The program itself.
- RAM chip, additional ram, like the N64 expansion pack.
- battery-packed ram chip to support save games.


What if we wanted to develop a game greater than 50 KB? We can using what is known as a **mapper** A mapper allows the programmer to choose a specific bank in the cartridge, or a specific segment of memory.

## Graphics

The Graphics are handled by a chip called the Picture Processing Unit.The NES uses the Ricoh 2C02 chip.This chip is responsible for rendering the 2D graphics, so the sprites and background that we see on the telivision. This brings up a question, how does the NES know what to draw on the screen? Well, the graphics data is obviously stored in the Cartridge. There's a dedicated chip known as character memory, which stores the 2D drawings (tiles), which are stored into a data structure known as a pattern table. There maybe CHR-RAM for some games. The PPU can address up to 8 KB of character memory, grouped into two, each group containing 4 KB. The cartridge may have metadata telling the PPU where and how to draw graphics. For instance, there's:

- A seperate 2 KB of SRAM on the motherboard, dedicated to graphics data. This is your VRAM. The vram will store two nametables.
- The PPU has 256B of DRAM to store object attribute memory (OAM).
- The PPU houses 4 bytes of memory defining the color palettes.



### How graphics are rendered

So, remember, these consoles were displayed on the CRT. The chip is designed with the CRT's behavior in mind. It won't buffer the image, instead writing the image on the fly. The PPU has a pixed dimension of 256x240 pixels. NTSC TVs will crop the top and bottom edges to accomodate overscan. Appoximately 224 scanlines are visible. The frame created by the PPU's output is composed of two different layers.

Remember we talked about pattern tables, they are a 16 x 16 matrix consisting of **tiles**, which are the foundation for producing the sprites and backgrounds.

#### Tiles

The tile is defined as an 8x8 pixel map, stored in character memory. each tile is represented by 16 bytes, and the Pattern table stores 256 tiles. On the nes, you'll have two patten tables. In a tile, each pixel is encoded using 2 bits, which is used to reference one of four colors from a palette table. The programmer can define up to eight of them (4 for background, 4 for sprites). The pallette table points to a master palette made up of 64 colors.


#### How to draw Something on the screen?

To actually draw something, the NES does the following:

- Game populates the pattern tables using data from the character memory.
- Each table is responsible for the background and foreground of the frame. The PPU reads from the table and composes the scanlines.


### Background Rendering

The background layer is a 512x480 pixel map containing static tiles (2x2 background, remember frame is 256 x 240). since the frame is smaller, the game chooses which part of the layer is displayed. The game moves the viewable area, and this is how scrolling is accomplished. To save memory, we combine four tiles into a 16x16 pixel map, which we call **blocks**, where all the blocks share the same color palette. The Nametable is what lets us draw the background. The **nametable** will specify which tiles to display in the background layer. The PPU will look for four 1 KB nametables for each quadrant of the layer. In reality, there are only two name tables (assuming there's no additional VRAM in the cartridge), and the remaining two are gonna be the same as the first two. This is a technique known as **mirroring**. This last 64 bytes of the nametable will house the attribute table.