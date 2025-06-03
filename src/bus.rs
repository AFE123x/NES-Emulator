use std::{cell::RefCell, rc::Rc};

use crate::{apu::Apu, cartridge::Cartridge, controller::Controller, ppu::Ppu};

/// The `Bus` struct acts as the central communication layer connecting the CPU
/// to the various subsystems in the NES emulator, including RAM, the cartridge,
/// PPU, APU, and controllers.
pub struct Bus {
    /// Main system RAM (2KB), mirrored every 2KB through $0000-$1FFF.
    memory: Vec<u8>,

    /// The inserted cartridge, which handles memory mapping for PRG-ROM, CHR-ROM, etc.
    cartridge: Option<Rc<RefCell<Cartridge>>>,

    /// Pointer to the PPU (Picture Processing Unit), used for accessing registers and DMA.
    ppu: Option<*mut Ppu>,

    /// First controller, typically for player 1.
    controller1: Option<Rc<RefCell<Controller>>>,

    /// Second controller, typically for player 2.
    controller2: Option<Rc<RefCell<Controller>>>,

    /// The APU (Audio Processing Unit), handles sound and related I/O registers.
    apu: Option<Rc<RefCell<Apu>>>,
}

impl Bus {
    /// Constructs a new `Bus` with all components unlinked and RAM initialized to zero.
    pub fn new() -> Self {
        Self {
            memory: vec![0; 2048],
            cartridge: None,
            controller1: None,
            controller2: None,
            ppu: None,
            apu: None,
        }
    }

    /// Links a cartridge to the bus, allowing CPU access to PRG-ROM and other mapper-controlled behavior.
    pub fn link_cartridge(&mut self, cart: Rc<RefCell<Cartridge>>){
        self.cartridge = Some(cart);
    }

    /// Links the PPU to the bus, enabling register and DMA interaction.
    pub fn link_ppu(&mut self, ppu: &mut Ppu){
        self.ppu = Some(ppu);
    }

    /// Links the APU to the bus for handling audio register access.
    pub fn link_apu(&mut self, apu: Rc<RefCell<Apu>>){
        self.apu = Some(apu);
    }

    /// Links controller 1 to the bus.
    pub fn link_controller1(&mut self, controller: Rc<RefCell<Controller>>) {
        self.controller1 = Some(controller);
    }

    /// Links controller 2 to the bus.
    pub fn link_controller2(&mut self, controller: Rc<RefCell<Controller>>) {
        self.controller2 = Some(controller);
    }

 

    /// Reads a byte from the specified CPU address space.
    ///
    /// # Arguments
    /// * `address` - The 16-bit memory address from which to read.
    /// * `rdonly` - If true, indicates a read-only access (for PPU reads).
    ///
    /// # Returns
    /// * `u8` - The byte read from the given address.
    pub fn cpu_read(&self, address: u16, rdonly: bool) -> u8 {
        let mut data = 0;

        if address <= 0x1FFF {
            data = self.memory[(address & 0x7FF) as usize];
        } else if address <= 0x3FFF {
            data = unsafe { (*self.ppu.unwrap()).cpu_read(address, rdonly) };
        } else if address <= 0x4017 {
            match address {
                0x4000..=0x4013 | 0x4015 => {
                    if let Some(apu) = &self.apu {
                        data = apu.borrow_mut().cpu_read(address);
                    } else {
                        panic!("APU Error");
                    }
                },
                0x4016 => {
                    if let Some(controller) = &self.controller1 {
                        data = controller.borrow_mut().cpu_read();
                    } else {
                        panic!("ERROR - Controller 1 not initialized");
                    }
                },
                0x4017 => {
                    if let Some(controller) = &self.controller2 {
                        data = controller.borrow_mut().cpu_read();
                    } else {
                        data = 0;
                    }
                },
                _ => {}
            }
        } else if address <= 0x401F {
            data = 0;
        } else {
            if let Some(cart) = &self.cartridge {
                cart.borrow_mut().cpu_read(address, &mut data);
            } else {
                panic!("Cartridge Error - Attempted read with no cartridge loaded");
            }
        }

        data
    }

    /// Writes a byte to the specified CPU address space.
    ///
    /// # Arguments
    /// * `address` - The 16-bit memory address to write to.
    /// * `byte` - The value to write.
    pub fn cpu_write(&mut self, address: u16, byte: u8) {
        if address <= 0x1FFF {
            self.memory[(address & 0x7FF) as usize] = byte;
        } else if address <= 0x3FFF {
            unsafe {
                (*self.ppu.unwrap()).cpu_write(address, byte);
            }
        } else if address <= 0x4017 {
            match address {
                0x4000..=0x4013 | 0x4015 | 0x4017 => {
                    if let Some(apu) = &self.apu {
                        apu.borrow_mut().cpu_write(address, byte);
                    } else {
                        panic!("APU Error");
                    }
                },
                0x4014 => {
                    // Perform OAM DMA transfer from page in memory to PPU OAM
                    let base = (byte as usize) << 8;
                    for i in 0..=0xFF {
                        let data = self.memory[(base + i) % 2048];
                        unsafe {
                            if let Some(ppu_ptr) = self.ppu {
                                (*ppu_ptr).oam_dma_write(i as u8, data);
                            } else {
                                panic!("PPU pointer is null during DMA transfer");
                            }
                        }
                    }
                    // TODO: Add CPU stall logic (~513-514 cycles)
                },
                0x4016 => {
                    if let Some(controller) = &self.controller1 {
                        controller.borrow_mut().cpu_write(byte);
                    }
                    if let Some(controller) = &self.controller2 {
                        controller.borrow_mut().cpu_write(byte);
                    }
                },
                _ => {
                    // Unimplemented APU/IO registers
                }
            }
        } else if address <= 0x401F {
            // Typically disabled APU/IO registers
        } else {
            if let Some(cartridge) = &self.cartridge {
                cartridge.borrow_mut().cpu_write(address, byte);
            } else {
                panic!("Cartridge Error - Attempted write with no cartridge loaded");
            }
        }
    }
}
