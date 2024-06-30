- The NES uses the 2C02 Picture Processing Unit.
	- The PPU handles the graphics stuff.
- The PPU has registers located in the CPU's memory from address $2000-$2007, and $4014.


## Memory map

- The PPU also has it's own memory, VRAM.
	- The PPU can also address 64kb of memory, but there's only 16kb of physical RAM. Below is the memory map for the PPU.

![text](./image.png)


## How do we control the PPU?

- To read/write to PPU memory is done with I/O registers $2006 and $2007 in CPU memory.
	- This R/W is usually done during V-Blank at the end of a frame, because you don't want to mess with data while it's rendering.
	- Another thing to be mindful of, the PPU has 16 bit addresses, but the register is only one byte. 
		- Because of this, we need to write to address $2006 twice.
	- $2007 has the actual byte itself, we can read and write to this address.
	- after doing a write, the address is incremented by 1 or 32 depending on the 2nd bit of $2000
	- The first read from $2007 isn't valid, but will be buffered and return on next read.
		- Exception to this are color palettes.

## PPU storage

- The PPU has a 256 byte area of memory, SPR-RAM(Sprite Ram), to store sprite attributes. The sprites are found in the pattern table.

## Control registers

- The PPU has control registers at locations $2000 and $2001
- $2000 has control register 1
- $2001 has control register 2.
- These registers are **write only**

### $2000 register
- only has write access
- Bit 7 is used to disable NMIs, which are interrupts generated when V-Blanks occur.
	- Remember, NMIs aren't affected by interrupt disable flag on CPU flag.
	- if the bit is 0, NMI will not happen, otherwise, it will happen.
- Bit 6 is a toggle between master/slave select for the PPU.
	- When bit 6 is 0, the PPU will get the palette index for the backdrop colors from the EXT pin. It's grounded on the NES, so I don't think we'll have to worry about it.
- Bit 5 will toggle the sprite size between 8x8 and 8x16.
	- 1 will switch the sprite to 8x16.
- Bit 4 specifies which pattern table to use (0: $0000, 1: 1$000)
- bit 3 specifies which sprite pattern table to use for 8x8 sprites
	- 0: $0000, 1: $1000
	- not used in 8x16 sprite size.
- After I/O occurs with the PPU address, the address is incremented by either 1 (horizontal), or 32 (vertical). bit 2 of $2000 will determine how te increment
	- If bit 2 is set, it'll be incremented by 32, otherwise 1.
- the 2 lsb represent the name table address.
	- 0 is $2000, 1 is $2400, 2 is $2800 and 3 is $2C00.
	- Another way to look at this:
		- when you set x to one, you add 256 to X position
		- when you set y to one, you add 240 to y position.

### $2001 register

- Clearing bit 3 clears the background, and the sprites can be hidden by clearing bit 4.


## PPU Status Registers

- The PPU status register is located at $2002, it's **read only**
- This register reports the status to the CPU.
	- bit 7 indicates if a V-blank is occuring.
	- Bit 4 indicates if the PPU is willing to accept write to VRAM.
		- if it's 0, then the write's ignored.
	- bit 6 and 7 will come later.
- When you read from address $2002, bit 7 is set to 0 with $2005 and $2006.


## Reading Large ammount of data between CPU and PPU
- It's not gonna be efficient to use the I/O on the PPU to read the bytes. 
- You can transfer data from CPU memory to sprite memory with the following steps:
	- Load required SPR-RAM address to CPU
	- write required SPR-RAM address to $2003
	- load byte into CPU
	- write byte to 2004.
- This approach would require you to perform the steps 256 times. Dynamic Memory Access is a more efficient way. 
	- Basically, the whole of sprite memory can be filled with one instruction, a write to $4014. 
		- Basically, you put the upper byte into this register, and it'll copy the data from, given you put XX in the register, XX00-XXFF to the internal PPU OAM.
	- This approach uses the memory bus, preventing the cpu from accessing additional instructions.
		- DMA takes appromixately 512 clock cycles.
		- Clock stealing is what prevents the cpu from accessing additional instructions.


## Color Palette:

- The NES has a color palette, which can store 52 colors.
	- Even though there's space for 64, we say 52 colors :-D
- THe NES uses two palettes, each with 16 entries.
	- Image palette: $3F00 - $3F0F
		- This palette shows colors available for background tiles.
	- Sprite palette $3F10 - $3F1F
		- THis shows colors available for sprites.
	- These palette don't have the color values, but an index of the color in the system palet.
- The first entry, 0x3F00, is the background color, used for transparency. Mirroring here is used, so every four bytes is a copy of $3F00


## Pattern Tables

- The NES has two pottern tables at addresses $0000 and $1000. These tables store 8x8 pixel tiles which can be drawn on the screen. The pattern tables are usually in CHR-ROM.
	- games without CHR-ROM will use ram for the pattern table, and fill em during execution.


The pattern will store the least significant bit in the first byte, and most significant in the second byte.

```

-----------lo bit------------
$0000  0  0  0  1  0  0  0  0
$0001  0  0  0  0  0  0  0  0
$0002  0  1  0  0  0  1  0  0
$0003  0  0  0  0  0  0  0  0
$0004  1  1  1  1  1  1  1  0
$0005  0  0  0  0  0  0  0  0
$0006  1  0  0  0  0  0  1  0
$0007  0  0  0  0  0  0  0  0

----------hi bit-------------
$0008  0  0  0  0  0  0  0  0
$0009  0  0  1  0  1  0  0  0
$000A  0  1  0  0  0  1  0  0
$000B  1  0  0  0  0  0  1  0
$000C  0  0  0  0  0  0  0  0
$000D  1  0  0  0  0  0  1  0
$000E  1  0  0  0  0  0  1  0
$000F  0  0  0  0  0  0  0  0

result

00  00  00  01  00  00  00  00
00  00  10  00  10  00  00  00
00  11  00  00  00  11  00  00
10  00  00  00  00  00  10  00
01  01  01  01  01  01  01  00
10  00  00  00  00  00  10  00
11  00  00  00  00  00  11  00
00  00  00  00  00  00  00  00
```

- As a reminder, the result numbers are an index of the pallet table, 0 - 3.


## Name Tables and Attribute tables

- Name tables are a matrix of tile numbers, which point to the tiles in the pattern table.
	- The name table in 32 x 30 tiles.
- Each name table has an associated attribute table.
	- The attribute table tells us which palette to use.