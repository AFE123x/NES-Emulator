#include "AFE6502.hpp"
/**
* Our constructor for CPU object. Sets all values in array
* to zero. 
*/
CPU::CPU() {
	for (int i = 0; i < memlength; i++) {
		memory[i] = 0;
	}
}
/**
* Our destructor function. This isn't used at the moment, as
* we don't have anything on the heap. 
*/
CPU::~CPU() {

}
/**
* This function will handle reading the bin file. With this, 
* We can execute instructions and do stuff.
* @param std::string file which is the bin file. 
* @return unsigned 8 bit integer based on status of emulator
*/
uint8_t CPU::execute(std::string file) {
	std::ifstream inputfile(file, std::ios::binary);
	if (!inputfile.is_open()) {
		return 2;
	}
	char byte;
	
	while (inputfile.get(byte)) {
		// Process the byte as needed
		std::cout << "Read byte: " << static_cast<int>(byte) << std::endl;
	}
	return 0;
}
