#include<stdio.h>
#include<stdint.h>
#include<stdlib.h>

uint8_t A;
uint8_t immval;
uint8_t C, Z, V;
void ADC(){
  uint8_t a = A;
  uint8_t b = immval;
  uint8_t c = (C) ? 1 : 0;
  uint16_t result = a + b + c;
  char a_prop, b_prop, c_prop;
  a_prop = a & 0x80;
  b_prop = b & 0x80;
  c_prop = (result & 0x80);
  C = (result > 255);
  Z = (result == 0);
  V = ((c_prop ^ a_prop) & (c_prop ^ b_prop)) != 0;
  A = (uint8_t)(result & 0xFF);
}


int main(int argc, char const *argv[])
{
    // A = 127;
    int8_t aoeu = -128;
    int8_t result = aoeu - 1;
    printf("%d\n",result);
    return 0;
}
