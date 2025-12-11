//! # Cartridge
//! This module implements the NES cartridge system, including loading ROMs, handling different mappers,
//! memory mirroring modes, and interacting with CPU/PPU buses.
//!
//! It supports multiple mappers (000, 001, 002, 003, 004, 066), which are used to provide bank switching,
//! IRQ handling, and more advanced functionality for NES games.

mod mapper;
mod mapper000;
mod mapper001;
mod mapper002;
mod mapper003;
mod mapper004;
mod mapper066;

use mapper::Mapper;
use mapper000::Mapper000;
use mapper001::Mapper001;
use mapper002::Mapper002;
use mapper003::Mapper003;
use mapper004::Mapper004;
use mapper066::Mapper066;

use std::fs;

/// Represents the 16-byte iNES header from a NES ROM file.
#[derive(Debug)]
struct Header {
    _prg_rom_size: u8,
    _chr_rom_size: u8,
    _mapper: u8,
    _four_screen: bool,
}

/// Supported nametable mirroring configurations for PPU memory.
#[derive(Debug, Clone)]
pub enum MirrorMode {
    Horizontal,
    OneScreenHi,
    OneScreenLo,
    Vertical,
}

/// Represents an NES cartridge, encapsulating PRG/CHR ROM and a memory mapper.
/// Handles read/write operations from the CPU and PPU, mirroring, and mapper-specific IRQ behavior.
pub struct Cartridge {
    _header: Header,
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
    mapper: Box<dyn Mapper>,
}

impl Cartridge {
    /// Clears the mapper's IRQ flag. This is typically called by the CPU after an IRQ has been serviced.
    pub fn irq_clear(&mut self) {
        self.mapper.irq_clear();
    }

    /// Returns the nametable mirroring configuration for this cartridge (e.g., Horizontal, Vertical).
    pub fn get_nametable(&self) -> MirrorMode {
        self.mapper.get_mirror_mode().clone()
    }

    /// Constructs a new `Cartridge` from the provided file path.
    /// Loads PRG and CHR ROM data, parses the iNES header, and initializes the appropriate memory mapper.
    ///
    /// # Arguments
    /// * `file_name` - The path to the `.nes` ROM file.
    pub fn new(file_name: &str) -> Self {
        let buf = fs::read(file_name).expect("unable to open file!");
        let header = &buf[0..16];

        let prg_rom_size = header[4] as usize;
        let chr_rom_size = header[5] as usize;
        let prg_rom_len = prg_rom_size * 16 * 1024;
        let chr_rom_len = chr_rom_size * 8 * 1024;

        let training_data = if header[6] & 0x04 != 0 { 512 } else { 0 };

        let mut prg_rom: Vec<u8> = vec![0; prg_rom_len];
        for i in 0..prg_rom_len {
            prg_rom[i] = buf[16 + training_data + i]
        }

        let chr_rom: Vec<u8> = if chr_rom_size > 0 {
            let mut vec = vec![0; chr_rom_len];
            for i in 0..chr_rom_len {
                vec[i] = buf[16 + training_data + prg_rom_len + i];
            }
            vec
        } else {
            // Provide 8KB CHR RAM if no CHR ROM is present
            vec![0; 8192]
        };

        let mapper = (header[7] & 0xF0) | (header[6] >> 4);
        let four_screen = (header[6] & 0x08) != 0;
        let nametable_arrangement = match header[6] & 1 {
            0 => MirrorMode::Horizontal,
            1 => MirrorMode::Vertical,
            _ => unreachable!(),
        };

        let header = Header {
            _prg_rom_size: prg_rom_size as u8,
            _chr_rom_size: chr_rom_size as u8,
            _mapper: mapper,
            _four_screen: four_screen,
        };

        let mapper: Box<dyn Mapper> = match mapper {
            0 => Box::new(Mapper000 {
                n_chr: chr_rom_size as u8,
                n_prg: prg_rom_size as u8,
                nametable: nametable_arrangement,
            }),
            1 => Box::new(Mapper001::new(
                prg_rom_size as u8,
                chr_rom_size as u8,
                nametable_arrangement,
                None,
            )),
            2 => Box::new(Mapper002::new(
                prg_rom_size as u8,
                chr_rom_size as u8,
                nametable_arrangement,
            )),
            3 => Box::new(Mapper003::new(
                prg_rom_size as u8,
                chr_rom_size as u8,
                nametable_arrangement,
            )),
            4 => Box::new(Mapper004::new(
                prg_rom_size as u8,
                chr_rom_size as u8,
            )),
            66 => Box::new(Mapper066::new(
                prg_rom_size as u8,
                chr_rom_size as u8,
                nametable_arrangement,
            )),
            _ => panic!("mapper {} not supported", mapper),
        };

        println!("{:?}", header);

        Self {
            _header: header,
            prg_rom,
            chr_rom,
            mapper,
        }
    }

    /// Resets the mapper to its initial state. Called when the console is reset.
    pub fn _reset(&mut self) {
        self.mapper.reset();
    }

    /// Advances the internal scanline counter for mappers that support scanline-based IRQs.
    pub fn scanline(&mut self) {
        self.mapper.scanline();
    }

    /// Returns true if the mapper has an active IRQ (interrupt request) pending.
    pub fn irq(&mut self) -> bool {
        self.mapper.hasirq()
    }

    /// Reads a byte from CPU-visible memory mapped to the cartridge.
    ///
    /// # Arguments
    /// * `address` - The 16-bit CPU address to read from.
    /// * `byte` - A mutable reference where the read value will be stored.
    pub fn cpu_read(&self, address: u16, byte: &mut u8) {
        let mut mapped_addr = address as u32;
        let res = self.mapper.cpu_read(address, &mut mapped_addr, byte);
        if res && mapped_addr != 0xFFFFFFFF {
            let mapped_addr = (mapped_addr as usize) % self.prg_rom.len();
            *byte = self.prg_rom[mapped_addr];
        }
    }

    /// Writes a byte to CPU-visible memory mapped to the cartridge.
    ///
    /// # Arguments
    /// * `address` - The 16-bit CPU address to write to.
    /// * `byte` - The byte value to be written.
    pub fn cpu_write(&mut self, address: u16, byte: u8) {
        let mut mapped_address = address as u32;
        let res = self.mapper.cpu_write(address, &mut mapped_address, byte);
        if res && mapped_address != 0xFFFFFFFF {
            self.prg_rom[mapped_address as usize] = byte;
        }
    }

    /// Reads a byte from PPU-visible memory mapped to the cartridge.
    ///
    /// # Arguments
    /// * `address` - The 14-bit PPU address to read from.
    /// * `byte` - A mutable reference where the read value will be stored.
    pub fn ppu_read(&mut self, address: u16, byte: &mut u8) {
        let mut mapped_addr = address as u32;
        let res = self.mapper.ppu_read(address, &mut mapped_addr, *byte);
        if res {
            let mapped_addr = mapped_addr % (self.chr_rom.len() as u32);
            *byte = self.chr_rom[mapped_addr as usize];
        }
    }

    /// Writes a byte to PPU-visible memory mapped to the cartridge.
    ///
    /// # Arguments
    /// * `address` - The 14-bit PPU address to write to.
    /// * `byte` - The byte value to write.
    pub fn ppu_write(&mut self, address: u16, byte: u8) {
        let mut mapped_address = address as u32;
        let res = self.mapper.ppu_write(address, &mut mapped_address, byte);
        if res {
            self.chr_rom[mapped_address as usize] = byte;
        }
    }

    /// Saves the internal state of the mapper (useful for emulator save states).
    pub fn savestate(&mut self) {
        self.mapper.savestate();
    }
}
