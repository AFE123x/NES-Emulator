#include "../include/mapper0.h"

/**
 * @brief Maps a CPU read address for Mapper 0 (NROM).
 * 
 * @param address The CPU address being accessed (0x8000 - 0xFFFF).
 * @param mapped_address Pointer to store the corresponding ROM address.
 * 
 * @details 
 * - If the address is within the PRG ROM range, it is mapped to the appropriate 
 *   PRG ROM bank based on the size of the PRG ROM.
 * - Uses a mask of 0x7FFF if NPRG_ROM > 1 (multiple banks), or 0x3FFF otherwise.
 */
void mapper_0_cpu_read(uint16_t address, uint32_t* mapped_address) {
    if (address >= 0x8000 && address <= 0xFFFF) {
        *mapped_address = address & (NPRG_ROM > 1 ? 0x7FFF : 0x3FFF);
    }
}

/**
 * @brief Maps a CPU write address for Mapper 0 (NROM).
 * 
 * @param address The CPU address being written to (0x8000 - 0xFFFF).
 * @param mapped_address Pointer to store the corresponding ROM address.
 * 
 * @details
 * - Same behavior as CPU read: Maps the address based on the PRG ROM size.
 * - The mapped address is calculated using the same masking logic.
 */
void mapper_0_cpu_write(uint16_t address, uint32_t* mapped_address) {
    if (address >= 0x8000 && address <= 0xFFFF) {
        *mapped_address = address & (NPRG_ROM > 1 ? 0x7FFF : 0x3FFF);
    }
}

/**
 * @brief Maps a PPU write address for Mapper 0 (NROM).
 * 
 * @param address The PPU address being written to (0x0000 - 0x1FFF).
 * @param mapped_address Pointer to store the corresponding ROM/VRAM address.
 * 
 * @details
 * - Maps PPU addresses directly to the corresponding address in the pattern table (0x0000 - 0x1FFF).
 */
void mapper_0_ppu_write(uint16_t address, uint32_t* mapped_address) {
    if (address >= 0x0000 && address <= 0x1FFF) {
        *mapped_address = address;
    }
}

/**
 * @brief Maps a PPU read address for Mapper 0 (NROM).
 * 
 * @param address The PPU address being read (0x0000 - 0x1FFF).
 * @param mapped_address Pointer to store the corresponding ROM/VRAM address.
 * 
 * @details
 * - If NCHR_ROM == 0 (indicating CHR RAM is being used), the address is mapped directly.
 * - Otherwise, this function performs no mapping for CHR ROM.
 */
void mapper_0_ppu_read(uint16_t address, uint32_t* mapped_address) {
    if (address >= 0x0000 && address <= 0x1FFF) {
        if (NCHR_ROM == 0) {
            *mapped_address = address;
        }
    }
}
