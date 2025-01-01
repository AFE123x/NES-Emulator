#include "../include/cartridge.h"
#include "../include/mapper0.h"
#include <stdlib.h>
#include <string.h>
#include <assert.h>

// Static variables for ROM data
static uint8_t* program_rom;
static uint8_t* character_rom;

// NES Header structure
NES_Header header;

// Function pointers for mapper operations
void (*mapper_cpu_read)(uint16_t, uint32_t*);
void (*mapper_cpu_write)(uint16_t, uint32_t*);
void (*mapper_ppu_read)(uint16_t, uint32_t*);
void (*mapper_ppu_write)(uint16_t, uint32_t*);

// File pointer for the ROM
FILE* rom;

/**
 * @brief Loads a ROM file into memory and initializes the mapper.
 * 
 * @param romfile Path to the ROM file.
 * 
 * @details 
 * - Reads the NES header and extracts metadata, such as PRG ROM size and CHR ROM size.
 * - Allocates memory for PRG ROM and CHR ROM data.
 * - Detects and sets the appropriate mapper functions based on the header flags.
 * - Handles optional trainer data if present.
 */
void loadrom(char *romfile) {
    memset(&header, 0, sizeof(header));
    rom = fopen(romfile, "r");
    assert(rom);

    // Read the NES header
    fread(&header, 1, sizeof(header), rom);
    header.Constants[3] = '\0';
    printf("%s\n", header.Constants);

    // Extract PRG and CHR ROM sizes
    NPRG_ROM = header.PRG_ROM;
    NCHR_ROM = header.CHR_ROM;
    uint32_t program_size = 16384 * header.PRG_ROM;
    uint32_t character_size = 8192 * header.CHR_ROM;

    // Allocate memory for ROM data
    program_rom = (uint8_t*)malloc(program_size);
    assert(program_rom);
    character_rom = (uint8_t*)malloc(character_size);
    assert(character_rom);

    // Detect mapper number
    mapper_num = header.flag6 >> 4;
    mapper_num = (header.flag7 & 0xF0) | mapper_num;

    // Set mapper functions for Mapper 0 (NROM)
    if (mapper_num == 0) {
        mapper_cpu_read = mapper_0_cpu_read;
        mapper_cpu_write = mapper_0_cpu_write;
        mapper_ppu_read = mapper_0_ppu_read;
        mapper_ppu_write = mapper_0_ppu_write;
    }

    // Handle trainer data if present
    if (header.flag6 & 0x4) {
        fread(program_rom, 1, 512, rom); // Remove trainer data
    }

    // Load PRG and CHR ROM data
    fread(program_rom, 1, program_size, rom);
    fread(character_rom, 1, character_size, rom);

    fclose(rom);
}

/**
 * @brief Frees the allocated memory for the ROM data.
 */
void freerom() {
    free(program_rom);
    free(character_rom);
}

/**
 * @brief Reads a byte from the CPU memory map.
 * 
 * @param address The CPU address to read from.
 * @param byte Pointer to store the retrieved byte.
 * 
 * @details
 * - Maps the address using the mapper's CPU read function.
 * - Retrieves the corresponding byte from PRG ROM.
 */
void rom_cpu_read(uint16_t address, uint8_t* byte) {
    uint32_t mapped_address = 0;
    mapper_cpu_read(address, &mapped_address);
    if (mapped_address == 0) return;
    *byte = program_rom[mapped_address];
}

/**
 * @brief Reads a byte from the PPU memory map.
 * 
 * @param address The PPU address to read from.
 * @param byte Pointer to store the retrieved byte.
 * 
 * @details
 * - Maps the address using the mapper's PPU read function.
 * - Retrieves the corresponding byte from CHR ROM.
 */
void rom_ppu_read(uint16_t address, uint8_t* byte) {
    uint32_t mapped_address = 0x2000;
    mapper_ppu_read(address, &mapped_address);
    if (mapped_address == 0x2000) return;
    *byte = character_rom[mapped_address];
}

/**
 * @brief Writes a byte to the CPU memory map.
 * 
 * @param address The CPU address to write to.
 * @param byte The byte value to write.
 * 
 * @details
 * - Maps the address using the mapper's CPU write function.
 * - Writes the byte to the corresponding address in PRG ROM.
 */
void rom_cpu_write(uint16_t address, uint8_t byte) {
    uint32_t mapped_address = 0x7FFF;
    mapper_cpu_write(address, &mapped_address);
    if (mapped_address == 0x7FFF) return;
    program_rom[mapped_address] = byte;
}

/**
 * @brief Writes a byte to the PPU memory map.
 * 
 * @param address The PPU address to write to.
 * @param byte The byte value to write.
 * 
 * @details
 * - Maps the address using the mapper's PPU write function.
 * - Writes the byte to the corresponding address in CHR ROM.
 */
void rom_ppu_write(uint16_t address, uint8_t byte) {
    uint32_t mapped_address = 0x2000;
    mapper_ppu_write(address, &mapped_address);
    if (mapped_address == 0x2000) return;
    character_rom[mapped_address] = byte;
}
