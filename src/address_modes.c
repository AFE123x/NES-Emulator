#include "../include/address_modes.h"
#include "../include/bus.h"

/* Addressing modes implementation. */

void addr_immediate(void) {
    cpu_read(PC++, &immval);
}

void addr_zero_page(void) {
    uint8_t temp;
    cpu_read(PC++, &temp);
    cpu_read(temp, &immval);
}

void addr_zero_page_x(void) {
    uint8_t temp;
    cpu_read(PC++, &temp);
    temp += X;
    temp = temp & 0xFF; // Ensure wrapping within zero-page.
    cpu_read(temp, &immval);
}

void addr_zero_page_y(void) {
    uint8_t temp;
    cpu_read(PC++, &temp);
    temp += Y;
    temp = temp & 0xFF; // Ensure wrapping within zero-page.
    cpu_read(temp, &immval);
}

void addr_relative(void) {
    uint8_t temp;
    cpu_read(PC++, &temp);
    rel_addr = (int8_t)temp;
}

void addr_absolute(void) {
    uint8_t lo, hi;
    cpu_read(PC++, &lo);
    cpu_read(PC++, &hi);
    abs_addr = ((uint16_t)hi << 8) | lo;
    cpu_read(abs_addr, &immval);
}

void addr_absolute_x(void) {
    uint8_t lo, hi;
    cpu_read(PC++, &lo);
    cpu_read(PC++, &hi);
    abs_addr = ((uint16_t)hi << 8) | lo;
    abs_addr += X;
    cpu_read(abs_addr, &immval);
}

void addr_absolute_y(void) {
    uint8_t lo, hi;
    cpu_read(PC++, &lo);
    cpu_read(PC++, &hi);
    abs_addr = ((uint16_t)hi << 8) | lo;
    abs_addr += Y;
    cpu_read(abs_addr, &immval);
}

void addr_indirect(void) {
    uint8_t lo, hi;
    cpu_read(PC++, &lo);
    cpu_read(PC++, &hi);
    uint16_t temp_addr = ((uint16_t)hi << 8) | lo;
    cpu_read(temp_addr++, &lo);
    cpu_read(temp_addr++, &hi);
    abs_addr = ((uint16_t)hi << 8) | lo;
}

void addr_indexed_indirect(void) {
    uint8_t temp;
    cpu_read(PC++, &temp);
    temp += X;
    uint8_t lo, hi;
    cpu_read(temp++, &lo);
    cpu_read(temp++, &hi);
    abs_addr = ((uint16_t)hi << 8) | lo;
    cpu_read(abs_addr, &immval);
}

void addr_indirect_indexed(void) {
    uint8_t temp;
    cpu_read(PC++, &temp);
    uint8_t lo, hi;
    cpu_read(temp++, &lo);
    cpu_read(temp++, &hi);
    abs_addr = ((uint16_t)hi << 8) | lo;
    abs_addr += Y;
    cpu_read(abs_addr, &immval);
}
