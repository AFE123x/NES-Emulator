#include "../lib/BUS.h"
#include "../lib/CPU.h"
#include <iostream>
#include <fcntl.h>
#include <unistd.h>
int main(int argc, char *argv[]) {
  BUS *mybus = new BUS();
  mybus->write(0xFFFC,0x10); 
  mybus->write(0xFFFD,0x00);
  int rom = open(argv[1],O_RDONLY);
  if(rom == -1){
    perror("file opening");
    delete mybus;
    return -1;
  }
  int i = 0;
  char buf;
  while(read(rom,&buf,1) != 0){
    mybus->write(i++,buf);
  }
  CPU myCPU(mybus);
  myCPU.execute();
  delete mybus;
  close(rom);
}
