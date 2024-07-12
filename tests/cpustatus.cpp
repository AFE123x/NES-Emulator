#include <bitset>
#include <cstdint>
#include <iostream>
union status_register {
  struct status {
    uint8_t carry : 1;
    uint8_t zero : 1;
    uint8_t interrupt_disable : 1;
    uint8_t decimal : 1;
    uint8_t break_command : 1;
    uint8_t unused : 1;
    uint8_t overflow : 1;
    uint8_t negative : 1;
  } flags;
  uint8_t data;
};

void print_status(const status_register &sr) {
  std::cout << "Status Register Data: " << std::bitset<8>(sr.data) << std::endl;
  std::cout << "Carry: " << static_cast<int>(sr.flags.carry) << std::endl;
  std::cout << "Zero: " << static_cast<int>(sr.flags.zero) << std::endl;
  std::cout << "Interrupt Disable: "
            << static_cast<int>(sr.flags.interrupt_disable) << std::endl;
  std::cout << "Decimal: " << static_cast<int>(sr.flags.decimal) << std::endl;
  std::cout << "Break Command: " << static_cast<int>(sr.flags.break_command)
            << std::endl;
  std::cout << "Unused: " << static_cast<int>(sr.flags.unused) << std::endl;
  std::cout << "Overflow: " << static_cast<int>(sr.flags.overflow) << std::endl;
  std::cout << "Negative: " << static_cast<int>(sr.flags.negative) << std::endl;
}

int main() {
  status_register sr;
  sr.data = 0b11001010; // Setting some test data

  std::cout << "Initial Status Register Values:" << std::endl<<std::endl;
  print_status(sr);
  std::cout<<std::endl<<"testing bitfield to data"<<std::endl;
  sr.flags.carry = 1;
  sr.flags.zero = 1;
  sr.flags.interrupt_disable = 1;
  sr.flags.decimal = 1;
  sr.flags.break_command = 1;
  sr.flags.unused = 1;
  sr.flags.overflow = 1;
  sr.flags.negative = 0;
  print_status(sr);
  std::cout << static_cast<int>(sr.data) << std::endl; //Should print out 127

  return 0;
}
