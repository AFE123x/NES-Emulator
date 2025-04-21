use minifb::{Key, Scale, Window, WindowOptions};
use std::{cell::RefCell, error::Error, rc::Rc, time::{Duration, Instant}};

use crate::{
    apu::Apu, bus::Bus, cartridge::Cartridge, controller::{self, Buttons}, cpu::Cpu, ppu::{frame::Frame, Ppu}
};

pub fn gameloop(rom_file: &str) -> Result<(), Box<dyn Error>> {
    /* Initialize peripherals */
    let cartridge = Rc::new(RefCell::new(Cartridge::new(rom_file)));
    let mut cpu = Cpu::new();
    let mut game_frame = Frame::new(384, 240);
    let mut ppu = Ppu::new(Rc::clone(&cartridge));
    let mut bus = Bus::new();
    let controller = Rc::new(RefCell::new(controller::Controller::new()));
    let controller2 = Rc::new(RefCell::new(controller::Controller::new()));
    let mut turn = true;
    let apu: Rc<RefCell<Apu>> = Rc::new(RefCell::new(Apu::new()));
    let mut prevamount= apu.borrow_mut().get_pulse1_frequency();
    // Link components
    ppu.linkpattern(&mut game_frame);
    bus.link_cartridge(Rc::clone(&cartridge));
    bus.link_ppu(&mut ppu);
    bus.link_apu(Rc::clone(&apu));
    bus.link_controller1(Rc::clone(&controller));
    bus.link_controller2(Rc::clone(&controller2));
    
    // Connect APU to the bus

    cpu.linkbus(&mut bus);
    cpu.reset();
    let mut opened = false;

    let windowoption = WindowOptions {
        resize: true,
        scale: Scale::X4,
        ..Default::default()
    };
    
    let mut last_time = Instant::now();
    let mut frame_count = 0;
    let mut window = Window::new("NES Emulator - FPS: ", 384, 240, windowoption)?;
    window.set_target_fps(60);
    
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let curramount = apu.borrow().get_pulse1_frequency();
        if curramount != prevamount {
            // apu.borrow().play_sound_threaded();
            prevamount = curramount;
        }

        if window.is_key_pressed(Key::Semicolon, minifb::KeyRepeat::No){
            cartridge.borrow_mut().savestate();
        }
        if window.is_key_pressed(Key::P, minifb::KeyRepeat::No){
            if !opened{
                cartridge.borrow_mut().load();
            }
            opened = true;
        }
        turn = !turn;
        if window.is_key_pressed(Key::R, minifb::KeyRepeat::No){
            ppu.set_bg_palette_num();
        }
        if window.is_key_pressed(Key::Q, minifb::KeyRepeat::No){
            cpu.reset()
        }
        
        // Clock components
        for _ in 0..3{
            ppu.clock();
        }
        cpu.clock();
        // apu.clock();  // This is now properly integrated
        
        if ppu.get_nmi() {
            if turn{
                controller.borrow_mut().set_button(Buttons::A, window.is_key_down(Key::A));
                controller.borrow_mut().set_button(Buttons::B, window.is_key_down(Key::S));
                controller.borrow_mut().set_button(Buttons::Select, window.is_key_down(Key::D));
                controller.borrow_mut().set_button(Buttons::Start, window.is_key_down(Key::F));
                controller.borrow_mut().set_button(Buttons::Up, window.is_key_down(Key::Up));
                controller.borrow_mut().set_button(Buttons::Down, window.is_key_down(Key::Down));
                controller.borrow_mut().set_button(Buttons::Left, window.is_key_down(Key::Left));
                controller.borrow_mut().set_button(Buttons::Right, window.is_key_down(Key::Right));
            }
            else{
                controller2.borrow_mut().set_button(Buttons::A, window.is_key_down(Key::A));
                controller2.borrow_mut().set_button(Buttons::B, window.is_key_down(Key::S));
                controller2.borrow_mut().set_button(Buttons::Select, window.is_key_down(Key::D));
                controller2.borrow_mut().set_button(Buttons::Start, window.is_key_down(Key::F));
                controller2.borrow_mut().set_button(Buttons::Up, window.is_key_down(Key::Up));
                controller2.borrow_mut().set_button(Buttons::Down, window.is_key_down(Key::Down));
                controller2.borrow_mut().set_button(Buttons::Left, window.is_key_down(Key::Left));
                controller2.borrow_mut().set_button(Buttons::Right, window.is_key_down(Key::Right));
            }
            cpu.nmi();
            
            ppu.set_name_table();
            frame_count += 1;
            let elapsed = last_time.elapsed();
            if elapsed >= Duration::from_secs(1) {
                let fps = frame_count;
                window.set_title(&format!("NES Emulator - FPS: {}", fps));
                frame_count = 0;
                last_time = Instant::now();
            }
            ppu.get_pattern_table(&mut game_frame);
            window.update_with_buffer(game_frame.get_buf().as_slice(), 384, 240)?;
        }
    }
    Ok(())
}