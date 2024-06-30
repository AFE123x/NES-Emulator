#include "../lib/bus.h"
#include "../lib/cpu.h"
#include <fcntl.h>
#include <unistd.h>

int main(int argc, char **argv) {
//  argc = 2;
//  argv = {"./NES", "./tests/nestest.nes"};
 BUS* bus = new BUS();
 bus->clock();
 delete bus;
}