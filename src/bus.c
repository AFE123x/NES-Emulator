#include "../include/bus.h"
#include"../include/cpu.h"
#include <stdlib.h>
#include <assert.h>
#include<string.h>
// Static pointer to represent the system's memory.
char memory[0x10000];

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
        // assert(memory); / / Ensure memory is initialized.
    *byte = memory[address]; // Read the byte from the specified address.
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
    // assert(memory); // Ensure memory is initialized.
    memory[address] = byte; // Write the byte to the specified address.
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
    initialize_bus(); // Initialize the bus memory.
    cpu_init();
    // free(memory);
}
