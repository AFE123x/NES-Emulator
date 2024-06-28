#include <stdio.h>
#include <unistd.h>
#include <fcntl.h>
#include <stdint.h>
#include <string.h>

typedef struct {
    char magic[4];   // Should be "NES\x1A"
    uint8_t PRG_ROM_size;
    uint8_t CHR_ROM_size;
    uint8_t flags_6;
    uint8_t flags_7;
    uint8_t flags_8;
    uint8_t flags_9;
    uint8_t flags_10;
    char unused[5];   // Unused bytes (should be zero-filled)
} ines_t;

int main(int argc, char** argv) {
    if (argc < 2) {
        write(STDERR_FILENO, "./ines {file}\n", strlen("./ines {file}\n"));
        return -1;
    }

    int fd = open(argv[1], O_RDONLY);
    if (fd == -1) {
        perror("Error opening file");
        return -1;
    }

    ines_t header;
    ssize_t bytes_read = read(fd, &header, sizeof(header));
    if (bytes_read != sizeof(header)) {
        perror("Error reading file");
        close(fd);
        return -1;
    }

    close(fd);

    // Print the contents of the header struct
    printf("Magic: %c%c%c%c\n", header.magic[0], header.magic[1], header.magic[2], header.magic[3]);
    printf("PRG ROM Size: %u * 16 KB\n", header.PRG_ROM_size);
    printf("CHR ROM Size: %u * 8 KB\n", header.CHR_ROM_size);
    printf("Flags 6: %02X\n", header.flags_6);
    printf("Flags 7: %02X\n", header.flags_7);
    printf("Flags 8: %02X\n", header.flags_8);
    printf("Flags 9: %02X\n", header.flags_9);
    printf("Flags 10: %02X\n", header.flags_10);
    printf("Mapper: %d\n",((header.flags_7 & 0xF0) | (header.flags_6 >> 4)));

    return 0;
}
