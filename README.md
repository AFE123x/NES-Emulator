# NES Emulator

## Introduction

- Ever since December 2023, I've always wanted to create an NES Emulator. It's a cool project that requires in-depth knowledge of computer architecture and the ability to read the documentation. This is also my first time implementing a GUI and using Rust (for a personal project)

## Why Rust?

- There were two factors:
    - My friend kinda talked me into it
    - My dog was holding a toy crab; it was very cute.

- I read through all the chapters of The Rust book, even creating an Anki deck to know everything the language offers.

## The CPU

- The NES uses the 6502, a popular CPU created by MOS Technologies and copied by Ricoh. 

## Implementation of the CPU

- I'm very accustomed to C/C++, which has the luxury of function pointers (I now know Rust has function pointers.)
    - Instead, I took the cliche approach of using a lot of enums and match statements.
    - I took inspiration from OneLoneCoder, specifically with his CPU clock implementation. Besides that, I was on my own.

## PPU

- The next part of the project is implementing the Picture Processing Unit (PPU)
    - I plan to use minifb, a rust library where you can draw pixels from a u32 buffer.
    - I ultimately went with this since SDL2 sucks with Rust (linking is very convoluted.)