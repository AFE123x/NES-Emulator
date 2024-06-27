#include "../lib/bus.h"
#include "../lib/cpu.h"
#include <fcntl.h>
#include <unistd.h>

int main(int argc, char **argv) {
  // char* mystuff[] = {"./NES","/home/afe123x/Documents/projects/NES-Emulator/tests/assembly/LDA.nes","0"};
  // argv = mystuff;
  // argc = 3;
  if (argc < 3) {
    std::cerr << "./NES {program} {debugmode 1/0}" << std::endl;
    return -1;
  }
  int condition = atoi(argv[2]);

  int fd = open(argv[1], O_RDONLY);
  if (fd == -1) {
    std::cerr << "failed to open file" << std::endl;
    return -1;
  }
  BUS *mybus = new BUS();
  mybus->cpuwrite(0xFFFC, 0x00);
  mybus->cpuwrite(0xFFFD, 0x40);
  CPU *cpu = new CPU(mybus);


  char buf;
  for (int i = 0x0000; i <= 0xFFFF; i++) {
    long bytesread = read(fd, &buf, 1);
    if (bytesread) {
      mybus->cpuwrite(i, buf);
    } else {
      mybus->cpuwrite(i, 0x00);
    }
  }
  mybus->cpuwrite(0xFFFC, 0x00);
  mybus->cpuwrite(0xFFFD, 0x40);
  if (condition == 1) {
    cpu->debug_enable = true;
  } else if (condition == 2) {
    cpu->debug_enable = true;
    cpu->dissasemble(0x0000, 0xFFFE);
    delete mybus;
    delete cpu;
    close(fd);
    return -1;
  } else {
    cpu->debug_enable = false;
  }

  while (1) {

    cpu->tick();
  }
  delete mybus;
  delete cpu;
  close(fd);
  return 0;
}