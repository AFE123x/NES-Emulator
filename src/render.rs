use minifb::{Key, Scale, Window, WindowOptions};
use std::{cell::RefCell, error::Error, rc::Rc};

use crate::{
    bus::Bus,
    cartridge::Cartridge,
    controller::{self, Buttons},
    cpu::Cpu,
    ppu::{Frame::Frame, Ppu},
};

pub fn gameloop(rom_file: &str) -> Result<(), Box<dyn Error>> {
    /* Initialize peripherals */
    let cartridge = Rc::new(RefCell::new(Cartridge::new(rom_file)));
    let mut cpu = Cpu::new();
    let mut game_frame = Frame::new(512, 240);
    let mut ppu = Ppu::new(Rc::clone(&cartridge));
    let mut bus = Bus::new();
    let controller = Rc::new(RefCell::new(controller::Controller::new()));

    // Link components
    ppu.linkpattern(&mut game_frame);
    bus.link_cartridge(Rc::clone(&cartridge));
    bus.link_ppu(&mut ppu);
    bus.link_controller(Rc::clone(&controller));
    cpu.linkbus(&mut bus);
    cpu.reset();

    let windowoption = WindowOptions {
        resize: false,
        scale: Scale::X2,
        ..Default::default()
    };
    let mut window = Window::new("NES Emulator", 512, 240, windowoption)?;
    // let mut pattern_frame: Frame = Frame::new(256, 128);
    // window.set_target_fps(60);
    while window.is_open() && !window.is_key_down(Key::Escape) {
        controller.borrow_mut().set_button(Buttons::A, window.is_key_down(Key::A));
        controller.borrow_mut().set_button(Buttons::B, window.is_key_down(Key::S));
        controller.borrow_mut().set_button(Buttons::Select, window.is_key_down(Key::D));
        controller.borrow_mut().set_button(Buttons::Start, window.is_key_down(Key::F));
        controller.borrow_mut().set_button(Buttons::Up, window.is_key_down(Key::Up));
        controller.borrow_mut().set_button(Buttons::Down, window.is_key_down(Key::Down));
        controller.borrow_mut().set_button(Buttons::Left, window.is_key_down(Key::Left));
        controller.borrow_mut().set_button(Buttons::Right, window.is_key_down(Key::Right));
        if window.is_key_pressed(Key::P, minifb::KeyRepeat::No){
            ppu.set_bg_palette_num();
        }
        if window.is_key_pressed(Key::Q, minifb::KeyRepeat::No){
            cpu.reset()
        }
        for _ in 0..3{
            ppu.clock();
        }
        cpu.clock();
        if ppu.get_nmi() {
            cpu.nmi();
            
            ppu.set_name_table();
            ppu.get_pattern_table(&mut game_frame);
            window.update_with_buffer(game_frame.get_buf().as_slice(), 512, 240)?;
            // pattern_window.update_with_buffer(pattern_frame.get_buf().as_slice(), 256, 128)?;
        }
        
    }
    Ok(())
}
