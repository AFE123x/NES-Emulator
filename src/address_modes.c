#include "../include/address_modes.h"
#include "../include/bus.h"
#ifdef UNIT_TESTING
#include "/opt/homebrew/include/criterion/criterion.h"
#endif

/* 
 * Addressing modes implementation for a 6502 CPU emulator.
 * These functions determine how memory addresses are calculated based on the instruction's addressing mode.
 */

void addr_immediate(void) {
    // Immediate addressing: Operand is the next byte in memory.
    cpu_read(PC++, &immval);
}

void addr_implied(void){
  return;
}

void addr_zero_page(void) {
    // Zero-page addressing: Address is a single byte (00 to FF).
    uint8_t temp;
    cpu_read(PC++, &temp);
    abs_addr = temp;
    cpu_read(temp, &immval);
}

void addr_zero_page_x(void) {
    // Zero-page X-indexed addressing: Address is (base + X) wrapped within zero-page.
    uint8_t temp;
    cpu_read(PC++, &temp);
    temp += X;
    temp = temp & 0xFF; // Ensure wrapping within zero-page (00 to FF)
    abs_addr = temp;
    cpu_read(temp, &immval);
}

void addr_zero_page_y(void) {
    // Zero-page Y-indexed addressing: Address is (base + Y) wrapped within zero-page.
    uint8_t temp;
    cpu_read(PC++, &temp);
    temp += Y;
    temp = temp & 0xFF; // Ensure wrapping within zero-page (00 to FF).
    abs_addr = temp;
    cpu_read(temp, &immval);
}

void addr_relative(void) {
    // Relative addressing: Used for branch instructions.
    uint8_t temp;
    cpu_read(PC++, &temp);
    rel_addr = (int8_t)temp; // Sign-extend for relative jump offset.
}

void addr_absolute(void) {
    // Absolute addressing: Address is specified by two bytes (low and high).
    uint8_t lo, hi;
    cpu_read(PC++, &lo);
    cpu_read(PC++, &hi);
    abs_addr = ((uint16_t)hi << 8) | lo; // Combine bytes to form full address.
    cpu_read(abs_addr, &immval);
}

void addr_absolute_x(void) {
    // Absolute X-indexed addressing: Address is base + X. Adds extra cycle if page boundary is crossed.
    uint8_t lo, hi;
    cpu_read(PC++, &lo);
    cpu_read(PC++, &hi);
    uint16_t temp = ((uint16_t)hi << 8) | lo;
    abs_addr = temp + X;
    if ((temp & 0xFF00) != (abs_addr & 0xFF00)) { // Check if page boundary is crossed.
        cycles += 1;
    }
    cpu_read(abs_addr, &immval);
}

void addr_absolute_y(void) {
    // Absolute Y-indexed addressing: Address is base + Y. Adds extra cycle if page boundary is crossed.
    uint8_t lo, hi;
    cpu_read(PC++, &lo);
    cpu_read(PC++, &hi);
    uint16_t temp = ((uint16_t)hi << 8) | lo;
    abs_addr = temp + Y;
    if ((temp & 0xFF00) != (abs_addr & 0xFF00)) { // Check if page boundary is crossed.
        cycles += 1;
    }
    cpu_read(abs_addr, &immval);
}

void addr_indirect(void) {
    // Indirect addressing: Address is fetched from pointer located at given address.
    uint8_t lo, hi;
    cpu_read(PC++, &lo);
    cpu_read(PC++, &hi);
    uint16_t temp_addr = ((uint16_t)hi << 8) | lo;
    cpu_read(temp_addr++, &lo);
    cpu_read(temp_addr++, &hi);
    abs_addr = ((uint16_t)hi << 8) | lo;
}

void addr_indexed_indirect(void) {
    // Indexed indirect addressing: Pointer (base + X) is dereferenced for the final address.
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
    // Indirect indexed addressing: Pointer is dereferenced, and Y is added to the final address.
    uint8_t temp;
    cpu_read(PC++, &temp);
    uint8_t lo, hi;
    cpu_read(temp++, &lo);
    cpu_read(temp++, &hi);
    uint16_t temp2 = ((uint16_t)hi << 8) | lo;
    abs_addr = temp2 + Y;
    if ((temp2 & 0xFF00) != (abs_addr & 0xFF00)) { // Check if page boundary is crossed.
        cycles += 1;
    }
    cpu_read(abs_addr, &immval);
}

#ifdef UNIT_TESTING
// Unit tests for addressing modes using Criterion framework.
TestSuite(addressing);

Test(addressing, immediate) {
    // Test immediate addressing.
    cpu_write(0x0000, 69);
    PC = 0;
    addr_immediate();
    cr_assert_eq(immval, 69, "expected value == 69");
}

Test(addressing, zero_page) {
    // Test zero-page addressing.
    cpu_write(0x0000, 0x2A);
    cpu_write(0x002A, 99);
    PC = 0;
    addr_zero_page();
    cr_assert_eq(immval, 99, "expected value == 99");
}

Test(addressing, zero_page_x) {
    // Test zero-page X-indexed addressing.
    cpu_write(0x0000, 0xFE);
    X = 3;
    cpu_write(0x0001, 99);
    PC = 0;
    addr_zero_page_x();
    cr_assert_eq(immval, 99, "ZPX - Failed: expected");
}

Test(addressing, zero_page_y) {
    // Test zero-page Y-indexed addressing.
    cpu_write(0x0000, 0xFE);
    Y = 3;
    cpu_write(0x0001, 99);
    PC = 0;
    addr_zero_page_y();
    cr_assert_eq(immval, 99, "ZPY - Failed: expected");
}

Test(addressing, relative) {
    // Test relative addressing.
    cpu_write(0x0000, 0xFE); // -2 in two's complement.
    PC = 0;
    addr_relative();
    cr_assert_eq(rel_addr, -2, "relative addressing - FAILED");
}

Test(addressing, absolute) {
    // Test absolute addressing.
    cpu_write(0x0000, 0xEF);
    cpu_write(0x0001, 0xBE);
    PC = 0;
    addr_absolute();
    cr_assert_eq(abs_addr, 0xBEEF, "Absolute Addressing - FAILED");
}

Test(addressing, absolute_x) {
    // Test absolute X-indexed addressing.
    cpu_write(0x0000, 0xEF);
    cpu_write(0x0001, 0xBE);
    PC = 0;
    X = 1;
    addr_absolute_x();
    cr_assert_eq(abs_addr, (0xBEEF + 1), "Absolute Addressing X - FAILED");
}

Test(addressing, absolute_y) {
    // Test absolute Y-indexed addressing.
    cpu_write(0x0000, 0xEF);
    cpu_write(0x0001, 0xBE);
    PC = 0;
    Y = 1;
    addr_absolute_y();
    cr_assert_eq(abs_addr, (0xBEEF + 1), "Absolute Addressing Y - FAILED");
}

Test(addressing, indirect) {
    // Test indirect addressing.
    cpu_write(0xFFFC, 0xFC);
    cpu_write(0xFFFD, 0xBA);
    cpu_write(0xBAFC, 0xEF);
    cpu_write(0xBAFD, 0xBE);
    PC = 0xFFFC;
    addr_indirect();
    cr_assert_eq(abs_addr, 0xBEEF, "indirect addressing - FAILED! actual: %x\n", abs_addr);
}

Test(addressing, indexed_indirect) {
    // Test indexed indirect addressing.
    cpu_write(0, 0x43);
    X = 2;
    cpu_write(0x45, 0xAD);
    cpu_write(0x46, 0xDE);
    cpu_write(0xDEAD, 69);
    PC = 0;
    addr_indexed_indirect();
    cr_assert_eq(immval, 69, "indexed indirect addressing - FAILED! actual: %x\n", abs_addr);
}

Test(addressing, indirect_indexed) {
    // Test indirect indexed addressing.
    cpu_write(0, 0x45);
    cpu_write(0x45, 0xAD);
    cpu_write(0x46, 0xDE);
    Y = 2;
    cpu_write(0xDEAF, 69);
    PC = 0;
    addr_indirect_indexed();
    cr_assert_eq(immval, 69, "indirect indexed addressing - FAILED! actual: %x\n", abs_addr);
}
#endif
