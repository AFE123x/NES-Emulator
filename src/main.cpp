#include "../lib/BUS.h"
#include <iostream>
#include <fcntl.h>
#include <unistd.h>

int main(int argc, char *argv[]) {
    int fd = open("../tests/assembly/6502_functional_test.bin",O_RDONLY);
    BUS bus;
    char buf = 0;
    read(fd,&buf,1);
    for(int i = 0; i <= 0xFFFF; i++){
        std::cout<<"address: "<<i<<std::endl;
        bus.cpuwrite(i,buf);
        read(fd,&buf,1);
    }
    bus.execute();
    return 0;
}
