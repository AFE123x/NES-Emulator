#include<stdio.h>
#include<fcntl.h>
#include<unistd.h>

int main(int argc, char** argv){
    int rom = open(argv[1],O_RDONLY);
    char mychar;
    int readx = read(rom,&mychar,1);
    int address = 0;
    while(readx){
        printf("address %04x: %02x\n",address++ & 0xFFFF,mychar & 0xFF);
        readx = read(rom,&mychar,1);
    }
}