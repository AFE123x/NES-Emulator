#include <fcntl.h>
#include <stdint.h>
#include <stdio.h>
#include<unistd.h>
int main(int argc, char **argv) {

    //load/store instructions
  uint8_t array[] = {0xa9, 0x0a, 0x85, 0x00, 0xa9, 0x14, 0x85, 0x01,
                     0xa5, 0x00, 0x85, 0x02, 0xa5, 0x01, 0x85, 0x03};
    int fd = open("./testrom.nes",O_CREAT,O_RW);
    uint8_t program[0xFFFF];
    
}