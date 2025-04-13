# Mapper 001

- Mapper 001

## Capacities

- The PRG ROM can store up to 256 KiB of program rom.
- The ROM can be representad as:
    - 16 Kib changeable window and 16 Kb fixed section
    - 32 KiB window
- There's 32 KiB of program ram, accessible in 8 kb windows.
- There is 128 KiB of character memory
    - It can either be a switchable 4 Kib + 4 Kib or full 8 Kib.

## Nametable mirroring

- The nametable here, unlike other mappers, can be switched in this mapper.
    - It can either use Horizontal Mirroring, Vertical mirroring, or fixed at bank 0 or 1. 


## Mapper 001 banks

- From address 0x6000 to 0x7FFF, we write to the 8 KB PRG-RAM bank.
- from 0x8000-0xBFFF, we have a 16 kb PRG-ROM bank, which is either switchable or fixed to the first bank
- from address 0xC000 to 0xFFFF, we have another 16 kb rom bank, which is either pixed to the last bank, or switchable.

- You can write to the MMC1 control register to swap the fixed and switchable prg-rom, or setup the 32 kb prg bank switch.

## Programming Interface

- MMC1 is configured through a serial port.
    - A Serial port, will only write one bit at a time.

- CPU writes to address 0x8000 - 0xFFFF is connected to a 5 bit shift register.
    - If the msb of the data written is set, the shift register is reset to it's initial state.

### Writing to shift register

- The CPU will write to the address 5 times, writing to the shift register.
- On the 5th write, the shift register is reset to 10000

## Mapper 1 registers


### Load Register

The first register is the load register

```
7  bit  0
---- ----
Rxxx xxxD
|       |
|       +- Data bit to be shifted into shift register, LSB first
+--------- A write with bit set will reset shift register
            and write Control with (Control OR $0C), 
            locking PRG-ROM at $C000-$FFFF to the last bank.
```

- We reset the shift register if the msb is one. We also do a bitwise or with 0x0C with the control register value.


### Control Register

- This controls the mapper (obviously)

```
4bit0
-----
CPPMM
|||||
|||++- Nametable arrangement: (0: one-screen, lower bank; 1: one-screen, upper bank;
|||               2: horizontal arrangement ("vertical mirroring", PPU A10); 
|||               3: vertical arrangement ("horizontal mirroring", PPU A11) )
|++--- PRG-ROM bank mode (0, 1: switch 32 KB at $8000, ignoring low bit of bank number;
|                         2: fix first bank at $8000 and switch 16 KB bank at $C000;
|                         3: fix last bank at $C000 and switch 16 KB bank at $8000)
+----- CHR-ROM bank mode (0: switch 8 KB at a time; 1: switch two separate 4 KB banks)
```

- We select bank modes, and the nametable arangement here.


### CHR Bank 0 and 1

```
CHR BANK 0
4bit0
-----
CCCCC
|||||
+++++- Select 4 KB or 8 KB CHR bank at PPU $0000 (low bit ignored in 8 KB mode)

CHR BANK 1
4bit0
-----
CCCCC
|||||
+++++- Select 4 KB CHR bank at PPU $1000 (ignored in 8 KB mode)
```

- CHR Bank 0 and 1 selects the specific region of memory to access. 
    - In the 8 kb mode, we "ignore the low bit"

### PRG BANK

```
4bit0
-----
RPPPP
|||||
|++++- Select 16 KB PRG-ROM bank (low bit ignored in 32 KB mode)
+----- MMC1B and later: PRG-RAM chip enable (0: enabled; 1: disabled; ignored on MMC1A)
       MMC1A: Bit 3 bypasses fixed bank logic in 16K mode (0: fixed bank affects A17-A14;
       1: fixed bank affects A16-A14 and bit 3 directly controls A17)
```
- We only have one switchable program rom bank. the second half is fixed to the last bank.
    - we ignore the low bit in 32 bit mode.
    - The MSB selects whether the PRG-RAM chip is enabled (i dunno man)