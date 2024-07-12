#include <iostream>
#include <typeinfo>

class test {
public:
  char aoeu() { return '\0'; }
  
  test() {
    using FuncPtrType = decltype(&test::aoeu);
    std::cout << "Type of &test::aoeu: " << typeid(FuncPtrType).name() << std::endl;
  }
};

int main() {
  test mytest;
  return 0;
}
