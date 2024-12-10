#include<stdio.h>
#include<stdlib.h>
#include "../include/bus.h"
#include<assert.h>

#ifndef UNIT_TESTING
int main(int argc, char *argv[])
{
  // assert(argc >= 2);
  run_system(argv[1]);
  return EXIT_SUCCESS;
}
#endif
