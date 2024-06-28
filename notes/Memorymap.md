# Memory map

- Even though we can address 16 bits, not all of it is ram. instead, only 2k of it is ram. below is the memory map of this

|address range | size | device | 
| --- | --- | --- |
| 0x0000 - 0x07FF | 0x800 | 2kb internal RAM |
| 0x0800 - 0x1FFF | 0x17FF | mirror of the 2kb |
| 0x2000 - 0x3FFF | 0x1FFF | mirrors of PPU registers|
| 0x4000 - 0x4017 | 0x0018 | NES APU and I/O registers |
| 0x4018 - 0x401F | 0x0008 | APU and I/O functionality |
|0x4020 - 0xFFFF | 0xBFE0 | Cartridge stuff |

- address 4018 - 401F wasn't actually implemented on production units, so we'll ignore it.       