#include <iostream>
#include <fstream>
#include <cstdint>
int main() {
    // Open a file in binary mode for writing
    std::ofstream binaryFile("test.bin", std::ios::binary);

    if (!binaryFile.is_open()) {
        std::cerr << "Error opening file for writing.\n";
        return 1;
    }

    // Example data to write to the binary file
    uint8_t data[] = { 0x00, 0x01, 0x02, 0x03 };

    // Write the data to the binary file
    binaryFile.write(reinterpret_cast<const char*>(data), sizeof(data));

    // Close the file
    binaryFile.close();

    std::cout << "Binary file created successfully.\n";

    return 0;
}
