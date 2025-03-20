# NES Emulator

## Introduction

This is an NES Emulator Fully written in Rust. This has been a project I started in December of 2023, but I only gained enough will power and courage to complete it in March. This Emulator currently only supports Mapper 000, but I intend to add more later, which I will discuss later.

## Installation

By the off chance that you are planning on using this, here is how you do it:

### Step One
- Install [rustup!](https://www.rust-lang.org)

### Step Two

- clone repository

```bash
git clone https://github.com/AFE123x/NES-Emulator
```

### Step Three
- Run your program

```bash
cargo run <path-of-game> <scale>
```
- `path-of-game` denotes the path where your **legally backed up** rom is located.
- `scale` denotes how big you want the window to be.

- You may encounter issues with sdl2. In this case, you can modify the `cargo.toml` file as follows:

```toml
sdl2 = {version = "0.37.0" features=["bundled"]}
```
- This will compile sdl2 from source (it will take a minute).


### Controls
- I hardcoded the controls at the moment (they seem convoluted, Dvorak moment ;p)

| Key | Button |
| --- | --- |
| Up | Up Button |
| Down | Down Button |
| Left | Left Button |
| Right | Right Button |
| A | A Button |
| O | B Button |
| E | Select Button |
| U | Start Button |

## Plans

Ideally, this is not the end of the journey for my Emulator. Below are my plans. If you would like to contribute, feel free to make a pull request (there is much work to be done).

- Optimize the PPU Background Rendering:
    - Optimize PPU Scrolling by utilizing the loopy register mechanism.
    - Troubleshoot nametable rendering (Super Mario Bros nametable does not seem to compile correctly (may also be a flag issue)).
- Check CPU Instructions for correctness
    - The nestest seems to fail a flag test.
- Implement Other Mappers
    - I hope to implement mapper 001, which should broaden the horizon for game compatability.
- Audio
    - This is a WHOLE can of worms.
- Debugger
    - Implement a debugger to view the assembly, palette table, foam attributes, etc.
- Second controller support:
    - This is self-explanatory
- Network support
    - Support wireless multiplayer (never done before).
- AWS (I do not know; people are lowkey hyped about it.)

## Contributions

I love contributions; You are welcome to contribute to my lovely project.