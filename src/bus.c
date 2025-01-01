#include "../include/bus.h"
#include"../include/cpu.h"
#include"../include/cartridge.h"
#include <stdlib.h>
#include <assert.h>
#include<stdio.h>
#include<string.h>
// Static pointer to represent the system's memory.
char memory[2 * 1024];

/**
 * @brief Initializes the bus memory.
 * 
 * Allocates 64KB of memory (0x10000 bytes) and ensures successful allocation.
 * This memory serves as the bus for the system's components to interact with.
 */
static void initialize_bus() {
   memset(memory,0,sizeof(memory));
}

/**
 * @brief Reads a byte from the specified memory address.
 * 
 * @param address The memory address to read from.
 * @param byte Pointer to a variable where the read byte will be stored.
 * 
 * @note The function assumes the memory has already been initialized.
 */
void cpu_read(uint16_t address, uint8_t* byte) {
    if(address >= 0x0000 && address <= 0x1FFF){
        *byte = memory[address & 0x7FF];
    }
    else if(address >= 0x2000 && address <= 0x3FFF){
        //PPU Stuff
    }
    else if(address >= 0x4000 && address <= 0x4017){
        //NES APU and I/O registers
    }
    else if(address >= 0x4018 && address <= 0x401F){
        //APU and I/O functionality that is normally disabled. See CPU Test Mode. 
    }
    else if(address >= 0x4020 && address <= 0xFFFF){
        rom_cpu_read(address,byte);
    }
}

/**
 * @brief Handles CPU writes to memory.
 * 
 * @param address The memory address to write to.
 * @param byte The byte to write to the specified memory address.
 * 
 * @note The function assumes the memory has already been initialized.
 */
void cpu_write(uint16_t address, uint8_t byte) {
    /*
    	if (cart->cpuWrite(addr, data))
	{
	}
	else if (addr >= 0x0000 && addr <= 0x1FFF)
	{
		cpuRam[addr & 0x07FF] = data;

	}
	else if (addr >= 0x2000 && addr <= 0x3FFF)
	{
		ppu.cpuWrite(addr & 0x0007, data);
	}	
	else if (addr >= 0x4016 && addr <= 0x4017)
	{
		controller_state[addr & 0x0001] = controller[addr & 0x0001];
	}
    */
   if(address >= 0x0000 && address <= 0x1FFF){
        memory[address & 0x07FF] = byte;
   }
   else if(address >= 0x2000 && address <= 0x3FFF){
        //ppu stuff
   }
   else if(address >= 0x4016 && address <= 0x4017){
        //controller stuff
   }
   else if(address >= 0x4020 && address <= 0xFFFF){
    rom_cpu_write(address,byte);
   }
    
}

/**
 * @brief Initializes the system and prepares for gameplay.
 * 
 * @details This function initializes the bus and other components,
 *          using the provided ROM file for setup. It ensures proper cleanup
 *          by freeing the allocated memory before exiting.
 * 
 * @param rom Path to the ROM file to be loaded into the system.
 */
void run_system(char* rom) {
    initialize_bus();
    loadrom(rom);
    cpu_init();
    while(1){
        clock_cpu();
        struct cpu_test test = get_status();
        printf("PC: %x, A: %x X: %x Y: %x SP: %x 2: %x, 3: %x\n",test.PC & 0xFFFF,test.A & 0xFF,test.X & 0xFF,test.Y & 0xFF,test.SP & 0xFF,test.two_byte & 0xFF,test.three_byte & 0xFF);
        if(test.two_byte != 0 || test.three_byte != 0) break;
    }
    freerom();
}
