#include "../lib/BUS.h"
#include "../lib/CPU.h"
#include <iostream>
int main(int argc, char *argv[]) {
  BUS *mybus = new BUS();
  CPU myCPU(mybus);
  myCPU.execute();
  delete mybus;
}
