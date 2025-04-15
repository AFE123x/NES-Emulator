<h1 align="center">NES Emulator</h1>


An NES emulator is fully written in Rust.

<p align="center">
  <img src="./images/zelda.png" width="300"/>
  <img src="./images/megaman.png" width="300"/>
  <img src="./images/mario.png" width="300"/>
  <img src="./images/contra.png" width="300"/>
  <img src="./images/zelda2.png" width="300"/>
</p>

<h2 align="center">Introduction</h2>

This project is a Nintendo Entertainment System emulator written entirely in Rust. I began working on it in December 2023 but only found the motivation to push through and complete it in March 2024. Currently, the emulator supports Mappers 000, 001, and 002, with plans to add more.

<h2 align="center">Installation</h2>


If you are curious or brave enough to try it out, here is how you can get started:

### 1. Install Rust

If you have not already, install Rust using [rustup](https://www.rust-lang.org).

### 2. Clone the Repository

```bash
git clone https://github.com/AFE123x/NES-Emulator
cd NES-Emulator
```

### 3. Run the Emulator

```bash
cargo run --release
```

You will be prompted to select a ROM file (make sure it is one you legally own and backed up!).

<h2 align="center">Controls</h2>

Controls are currently hardcoded and may feel unconventional:

| Key         | NES Button         |
|-------------|--------------------|
| Up          | D-Pad Up           |
| Down        | D-Pad Down         |
| Left        | D-Pad Left         |
| Right       | D-Pad Right        |
| A           | A Button           |
| O           | B Button           |
| E           | Select             |
| U           | Start              |
| `;` or `S` | Save PRG RAM (Mapper 001 only) |
| `L` or `P` | Load PRG RAM (Mapper 001 only) |

<h2>Roadmap</h2>

This emulator is a work in progress, and there is still a lot I want to explore and implement:

- ✅ Replace SDL2 with `minifb`
- ✅ Optimize PPU background rendering
  - ✅ Implement Loopy register scrolling
  - ✅ Fix name table rendering issues in Super Mario Bros
- 🔲 Validate all CPU instructions (some flags still need verification)
- 🔲 Implement Mappers
  - ✅ Implement Mapper 001
  - ✅ Implement Mapper 001
  - ✅ Implement Mapper 002
  - 🔲 Implement Mapper 004
  - 🔲 Implement the other Mappers
- 🔲 Add additional mapper support
- 🔲 Audio emulation (coming... eventually)
- ✅ Implement a basic debugger
- 🔲 Second controller support (partially implemented)
- 🔲 Network multiplayer support (netplay)
- 🔲 Cloud save / AWS integration (just an idea... maybe?)

## Contributing

Contributions are more than welcome! Feel free to open a pull request, whether it is fixing a bug, adding a feature, or improving documentation. There is plenty of work to be done, and I would love the help.