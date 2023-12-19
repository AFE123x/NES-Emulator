/**
* This is the main file where the program starts. We input a text file
* into the CPU and execute the instructions. Depending on the return code
* of the execute function, it'll inform user of issues. 
* @author Arun Felix
*/
#include "AFE6502.hpp"
void printerrer(uint8_t code);

/**
* Our main function. We input a .bin file into the execute function
* and execute the instructions. 
*/
int main(int argc, char** argv) {
	CPU* cpu = new CPU();
	uint8_t code = cpu->execute("test.bin");
	printerrer(code);
	delete cpu;
	return code;
}
/**
* Will explain issues or status based on execute return code. 
*/
void printerrer(uint8_t code){
	switch (code) {
	case 2:
		std::cout << "Error Code 2: unable to open file. Please check file location and name is correct"<<std::endl;
	}
}