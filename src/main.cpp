#include "../lib/bus.h"
#include "../lib/cpu.h"
#include <fcntl.h>
#include <unistd.h>

int main(int argc, char **argv) {
 BUS* bus = new BUS();
 while(1){
    bus->clock();
 }
 delete bus;
}