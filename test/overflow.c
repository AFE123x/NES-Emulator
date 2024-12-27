#include<stdio.h>
#include<stdint.h>
#include<stdlib.h>
int main(int argc, char** argv){
    int8_t a,b;
    int16_t c;
    a = atoi(argv[1]);
    b = atoi(argv[2]);
    char a_prop = a & 0x80;
    char b_prop = b & 0x80;
    c = a + b;
    char c_prop = c & 0x80;
    if((c_prop ^ a_prop) & (c_prop ^ b_prop)){
        printf("Overflow!\n");
    }
    else{
        printf("No overflow!\n");
    }
}