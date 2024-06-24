#include<iostream>
#include<cstdint>

/*
Truth table, with most significant bit:

+---+---+---+---+
| A | M | R | V |
+---+---+---+---+
| 0 | 0 | 0 | 0 |
+---+---+---+---+
| 0 | 0 | 1 | 1 |
+---+---+---+---+
| 0 | 1 | 0 | 0 |
+---+---+---+---+
| 0 | 1 | 1 | 0 |
+---+---+---+---+
| 1 | 0 | 0 | 0 |
+---+---+---+---+
| 1 | 0 | 1 | 0 |
+---+---+---+---+
| 1 | 1 | 0 | 1 |
+---+---+---+---+
| 1 | 1 | 1 | 0 |
+---+---+---+---+
*/
int main(int argc, char** argv){
    uint8_t a, b, c;

    //test no overflow:
    a = 0b10000000;
    b = 0b10000000;
    c = a + b;
    ((c & 0x80) & ~(a & 0x80) & ~(b & 0x80)) || (~(c & 0x80)) & (a & 0x80) & (b & 0x80)
    std::cout<<static_cast<int>()<<std::endl;
    return 0;
}