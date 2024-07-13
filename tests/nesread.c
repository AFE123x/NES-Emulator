#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <fcntl.h>
#include <unistd.h>
#include <string.h>

#define HEADER_SIZE 16
#define TRAINER_SIZE 512

// Function to print a binary string representation of a byte
void print_binary(uint8_t byte) {
    for (int i = 7; i >= 0; --i) {
        printf("%d", (byte >> i) & 1);
    }
}

// Main function to read the .nes file and print information
int main(int argc, char *argv[]) {
    if (argc != 2) {
        fprintf(stderr, "Usage: %s <filename.nes>\n", argv[0]);
        return 1;
    }

    const char *filename = argv[1];
    int fd = open(filename, O_RDONLY);
    if (fd < 0) {
        perror("Failed to open file");
        return 1;
    }

    uint8_t header[HEADER_SIZE];
    if (read(fd, header, HEADER_SIZE) != HEADER_SIZE) {
        perror("Failed to read the header");
        close(fd);
        return 1;
    }

    // Check the NES file signature
    if (header[0] != 0x4E || header[1] != 0x45 || header[2] != 0x53 || header[3] != 0x1A) {
        fprintf(stderr, "Not a valid iNES file.\n");
        close(fd);
        return 1;
    }

    // Extract and print the header information
    uint8_t prg_rom_size = header[4];
    uint8_t chr_rom_size = header[5];
    uint8_t flags6 = header[6];
    uint8_t flags7 = header[7];
    uint8_t flags8 = header[8];
    uint8_t flags9 = header[9];
    uint8_t flags10 = header[10];

    printf("iNES Header Information:\n");
    printf("PRG ROM Size: %u x 16KB\n", prg_rom_size);
    printf("CHR ROM Size: %u x 8KB\n", chr_rom_size);
    printf("Mapper Number: %u\n", ((flags7 >> 4) | (flags6 & 0xF0)));
    printf("Mirroring: %s\n", (flags6 & 0x01) ? "Horizontal" : "Vertical");
    printf("Battery-backed PRG RAM: %s\n", (flags6 & 0x02) ? "Yes" : "No");
    printf("Trainer Present: %s\n", (flags6 & 0x04) ? "Yes" : "No");
    printf("Four-screen VRAM: %s\n", (flags6 & 0x08) ? "Yes" : "No");
    printf("VS System: %s\n", (flags7 & 0x01) ? "Yes" : "No");
    printf("PlayChoice-10: %s\n", (flags7 & 0x02) ? "Yes" : "No");
    printf("NES 2.0 Format: %s\n", (flags7 & 0x0C) == 0x08 ? "Yes" : "No");
    printf("PRG RAM Size: %u KB\n", (flags8 & 0x0F) * 8);
    printf("TV System: %s\n", (flags9 & 0x01) ? "PAL" : "NTSC");
    printf("Bus Conflicts: %s\n", (flags10 & 0x01) ? "Yes" : "No");

    // Read and print the trainer if present
    if (flags6 & 0x04) {
        uint8_t trainer[TRAINER_SIZE];
        if (read(fd, trainer, TRAINER_SIZE) != TRAINER_SIZE) {
            perror("Failed to read the trainer");
            close(fd);
            return 1;
        }
        printf("Trainer Data:\n");
        for (int i = 0; i < TRAINER_SIZE; ++i) {
            printf("%02X ", trainer[i]);
            if ((i + 1) % 16 == 0) printf("\n");
        }
    }

    // Skip over PRG ROM data
    off_t prg_rom_size_bytes = prg_rom_size * 16384;
    lseek(fd, prg_rom_size_bytes, SEEK_CUR);

    // Skip over CHR ROM data
    off_t chr_rom_size_bytes = chr_rom_size * 8192;
    lseek(fd, chr_rom_size_bytes, SEEK_CUR);

    // Check for PlayChoice INST-ROM data
    if (flags7 & 0x02) {
        uint8_t inst_rom[8192];
        if (read(fd, inst_rom, 8192) != 8192) {
            perror("Failed to read the PlayChoice INST-ROM data");
            close(fd);
            return 1;
        }
        printf("PlayChoice INST-ROM Data:\n");
        for (int i = 0; i < 8192; ++i) {
            printf("%02X ", inst_rom[i]);
            if ((i + 1) % 16 == 0) printf("\n");
        }
    }

    // Check for PlayChoice PROM data
    if (flags7 & 0x01) {
        uint8_t prom[32];
        if (read(fd, prom, 32) != 32) {
            perror("Failed to read the PlayChoice PROM data");
            close(fd);
            return 1;
        }
        printf("PlayChoice PROM Data:\n");
        printf("Data: ");
        for (int i = 0; i < 16; ++i) {
            printf("%02X ", prom[i]);
        }
        printf("\nCounterOut: ");
        for (int i = 16; i < 32; ++i) {
            printf("%02X ", prom[i]);
        }
        printf("\n");
    }

    // Check for additional title data
    off_t remaining_size = lseek(fd, 0, SEEK_END) - lseek(fd, 0, SEEK_CUR);
    if (remaining_size > 0) {
        uint8_t title_data[128];
        if (read(fd, title_data, remaining_size) != remaining_size) {
            perror("Failed to read the title data");
            close(fd);
            return 1;
        }
        printf("Additional Title Data (%ld bytes):\n", remaining_size);
        for (int i = 0; i < remaining_size; ++i) {
            printf("%02X ", title_data[i]);
            if ((i + 1) % 16 == 0) printf("\n");
        }
        printf("\n");
    }

    close(fd);
    return 0;
}
