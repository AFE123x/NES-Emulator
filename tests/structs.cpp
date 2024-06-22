#include <iostream>
#include <cstdint>

int main() {
    struct FLAGS {
        char Carry: 1;
        char Zero: 1;
        char Interrupt: 1;
        char Decimal: 1;
        char Break: 1;
        char Unused: 1;
        char Overflow: 1;
        char Negative: 1;
    };

    FLAGS flag = {0, 0, 0, 0, 0, 0, 0, 0};  // Initialize all flags to 0

    // Set specific flags for demonstration
    flag.Negative = 1;
    flag.Overflow = 1;
    flag.Unused = 1;
    flag.Break = 1;
    flag.Decimal = 1;
    flag.Interrupt = 0;
    flag.Zero = 0;
    flag.Carry = 1;

    uint8_t flaga = (flag.Negative << 7) |
                    (flag.Overflow << 6) |
                    (flag.Unused << 5) |
                    (flag.Break << 4) |
                    (flag.Decimal << 3) |
                    (flag.Interrupt << 2) |
                    (flag.Zero << 1) |
                    (flag.Carry << 0);

    std::cout << static_cast<int>(flaga) << std::endl;

    return 0;
}
