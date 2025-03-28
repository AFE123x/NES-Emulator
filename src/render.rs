use minifb::{Key, Scale, Window, WindowOptions};
use std::error::Error;

use crate::{
    bus::Bus,
    cartridge::Cartridge,
    controller::{self, Buttons},
    cpu::Cpu,
    ppu::{Frame::Frame, Ppu},
};

pub fn gameloop(rom_file: &str) -> Result<(), Box<dyn Error>> {
    /* Initialize peripherals */
    let mut cartridge = Cartridge::new(rom_file);
    let mut cpu = Cpu::new();
    let mut game_frame = Frame::new(256, 240);
    let mut ppu = Ppu::new(&mut cartridge);
    let mut bus = Bus::new(&mut cpu);
    let mut controller = controller::Controller::new();
    
    let mut pause = false;
    
    // Link components
    ppu.linkpattern(&mut game_frame);
    bus.link_cartridge(&mut cartridge);
    bus.link_ppu(&mut ppu);
    bus.link_controller(&mut controller);
    cpu.linkbus(&mut bus);
    cpu.reset();

    let windowoption = WindowOptions{
        resize: false,
        scale: Scale::X4,
        ..Default::default()
    };
    let mut window = Window::new("NES Emulator", 256, 240, windowoption )?;
    window.set_target_fps(80);
    while window.is_open() && !window.is_key_down(Key::Escape) {
        controller.set_button(Buttons::A, window.is_key_down(Key::A));
        controller.set_button(Buttons::B, window.is_key_down(Key::S));
        controller.set_button(Buttons::Select, window.is_key_down(Key::D));
        controller.set_button(Buttons::Start, window.is_key_down(Key::F));
        controller.set_button(Buttons::Up, window.is_key_down(Key::Up));
        controller.set_button(Buttons::Down, window.is_key_down(Key::Down));
        controller.set_button(Buttons::Left, window.is_key_down(Key::Left));
        controller.set_button(Buttons::Right, window.is_key_down(Key::Right));
        if !pause {
            bus.clock();
        }

        // Only update the window when a new frame is ready
        if ppu.get_nmi() {
            cpu.nmi();
            ppu.set_name_table();
            window.update_with_buffer(game_frame.get_buf().as_slice(), 256, 240)?;
        }
    }
    Ok(())
}
