#include <iostream>
#include <memory>
#include <string>
#include <cstdlib>
#include "../include/emulator.h"
int main(int argc, char *argv[]) {
  if (argc < 2) {
    std::cout << "./NES {arguments}, write -h/--help for help" << std::endl;
    return 1;
  }
  std::string rom;
  uint8_t scale = 1;
  for (int i = 1; i < argc; i++) {
    std::string mystring(argv[i]);
    if (mystring == "-h" || mystring == "--help") {
      std::cout << "This is a NES Emulator, Developed by AFE123x. Below are "
                   "some lovely"
                << std::endl
                << "instructions" << std::endl;
      std::cout << "-h/--help: prints out help screen (AKA, this)" << std::endl;
      std::cout << "-l/--load {rom file}: load rom" << std::endl;
      std::cout << "-c/--config {editor}: Will open the config file in editor."
                << std::endl;
      std::cout << "-s/--scale {scale factor}: Will scale your program by an "
                   "ammount between 1 and 255"
                << std::endl;
      std::cout << "-d/--debug: Enable debug mode" << std::endl;
      return 0;
    } else if (mystring == "-l" || mystring == "--load") {
      if (i + 1 == argc) {
        std::cout << "./NES -l {rom file}" << std::endl;
        break;
      } else {
        //std::cout << "We'll try to open it lol" << std::endl;
        i++;
        rom = argv[i];
      }
    } else if (mystring == "-c" || mystring == "--config") {
      std::cout << "open config" << std::endl;
    } else if (mystring == "-s" || mystring == "--scale") {
      if(i + 1 == argc){
        std::cerr<<"--scale {scale size between 1 and 255}"<<std::endl;
        return 1;
      }
      scale = atoi(argv[i + 1]);
      i++;
    } else if (mystring == "-d" || mystring == "--debug") {
      std::cout << "debug" << std::endl;
    } else {
      std::cout << "NESEmulator: Invalid argument" << std::endl;
      return 1;
    }
  }
  NES nes;
  nes.run(rom,scale);
  return 0;
}
