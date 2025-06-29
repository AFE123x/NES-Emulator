
use std::error::Error;
use std::io::stdin;
use std::time::Duration;
use std::{rc::Rc, time::Instant};
use std::cell::RefCell;
use apu::Apu;
use args::Args;
use bus::Bus;
use cartridge::Cartridge;
use clap::Parser;
use controller::Buttons;
use cpu::Cpu;
use minifb::{Key, Window, WindowOptions};
use ppu::{frame::Frame, Ppu};
use minifb::Scale;

mod apu;
mod bus;
mod cartridge;
mod controller;
mod cpu;
mod ppu;
mod args;



fn main() -> Result<(), Box<dyn Error>> {
    let vec = Args::parse();
    // println!("hello {}",vec.rom);
    // println!("id is {}",vec.id);
    let debugmode = !vec.debug;
    
    /* Initialize peripherals */
    let cartridge = Rc::new(RefCell::new(Cartridge::new(&vec.rom)));
    let mut cpu = Cpu::new();
    let mut game_frame = if debugmode{
        Frame::new(512, 240)
    }
    else{
        Frame::new(256, 240)
    };
    let ppu = Rc::new(RefCell::new( Ppu::new(Rc::clone(&cartridge))));
    let mut bus = Bus::new(Rc::clone(&ppu));
    let controller = Rc::new(RefCell::new(controller::Controller::new()));
    let controller2 = Rc::new(RefCell::new(controller::Controller::new()));
    let mut turn = true;
    let apu: Rc<RefCell<Apu>> = Rc::new(RefCell::new(Apu::new()));
    bus.link_cartridge(Rc::clone(&cartridge));
    bus.link_apu(Rc::clone(&apu));
    bus.link_controller1(Rc::clone(&controller));
    bus.link_controller2(Rc::clone(&controller2));

    // Connect APU to the bus

    cpu.linkbus(&mut bus);
    cpu.reset();
    let mut opened = false;

    let windowoption = if debugmode{
        WindowOptions {
            resize: false,
            scale: Scale::X2,
            ..Default::default()
        }
    }
    else{
        WindowOptions {
            resize: true,
            scale: Scale::X4,
            ..Default::default()
        }
    };
    // 
    let mut last_time = Instant::now();
    let mut frame_count = 0;
    let mut ismuted = false;
    let mut window = if debugmode{
        Window::new("NES Emulator - FPS: ", 512, 240, windowoption)
    }
    else{
        Window::new("NES Emulator - FPS: ", 256, 240, windowoption)
    }?;
    window.set_target_fps(59);
    let mut audiotog = false;
    let mut savetog = false;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_pressed(Key::Semicolon, minifb::KeyRepeat::No) {
            if !savetog{
                cartridge.borrow_mut().savestate();
                savetog = true;
            }
        }

        if window.is_key_released(Key::Semicolon){
            savetog = false;
        }
        if window.is_key_pressed(Key::P, minifb::KeyRepeat::No) {
            if !opened {
                cartridge.borrow_mut().load();
            }
            opened = true;
        }
        if window.is_key_released(Key::P){
            opened = false;
        }

        if window.is_key_pressed(Key::H, minifb::KeyRepeat::No) {
            if !opened {
                // cartridge.borrow_mut().write_to_prgram();
                print!("Select a memory address");
                let mut str = String::new();
                stdin().read_line(&mut str)?;
                let address: u16 = str.trim().parse().unwrap();
                print!("Select data to write");
                let mut str = String::new();
                stdin().read_line(&mut str)?;
                let data: u8 = str.trim().parse().unwrap();
                bus.cpu_write(address, data);
            }
            opened = true;
        }
        if window.is_key_released(Key::H){
            opened = false;
        }

        turn = !turn;
        if window.is_key_pressed(Key::R, minifb::KeyRepeat::No) {
            ppu.borrow_mut().set_bg_palette_num();
        }
        if window.is_key_pressed(Key::Q, minifb::KeyRepeat::No) {
            cpu.reset();
        }
        if window.is_key_pressed(Key::M, minifb::KeyRepeat::No){
            if !audiotog{
                apu.borrow_mut().toggle_sound();
                audiotog = true;
                ismuted = !ismuted;
            }
        }
        if window.is_key_released(Key::M){
            audiotog = false;
        }
        // Clock components
        for _ in 0..3 {
            ppu.borrow_mut().clock(&mut game_frame);
        }
        let _cycles_left = cpu.clock();

        if cartridge.borrow_mut().irq() {
            cartridge.borrow_mut().irq_clear();
            cpu.irq();
        }
        if ppu.borrow_mut().get_nmi() {

            let up = window.is_key_down(Key::Up);
            let down = window.is_key_down(Key::Down);
            let left = window.is_key_down(Key::Left);
            let right = window.is_key_down(Key::Right);
            if turn {
                controller
                    .borrow_mut()
                    .set_button(Buttons::A, window.is_key_down(Key::A));
                controller
                    .borrow_mut()
                    .set_button(Buttons::B, window.is_key_down(Key::S));
                controller
                    .borrow_mut()
                    .set_button(Buttons::Select, window.is_key_down(Key::D));
                controller
                    .borrow_mut()
                    .set_button(Buttons::Start, window.is_key_down(Key::F));
                controller.borrow_mut().set_button(Buttons::Up, up && !down);
                controller
                    .borrow_mut()
                    .set_button(Buttons::Down, down && !up);
                controller
                    .borrow_mut()
                    .set_button(Buttons::Left, left && !right);
                controller
                    .borrow_mut()
                    .set_button(Buttons::Right, !left && right);
            } else {
                controller2
                    .borrow_mut()
                    .set_button(Buttons::A, window.is_key_down(Key::A));
                controller2
                    .borrow_mut()
                    .set_button(Buttons::B, window.is_key_down(Key::S));
                controller2
                    .borrow_mut()
                    .set_button(Buttons::Select, window.is_key_down(Key::D));
                controller2
                    .borrow_mut()
                    .set_button(Buttons::Start, window.is_key_down(Key::F));
                controller2
                    .borrow_mut()
                    .set_button(Buttons::Up, up && !down);
                controller2
                    .borrow_mut()
                    .set_button(Buttons::Down, down && !up);
                controller2
                    .borrow_mut()
                    .set_button(Buttons::Left, left && !right);
                controller2
                    .borrow_mut()
                    .set_button(Buttons::Right, !left && right);
            }
            cpu.nmi();

            frame_count += 1;
            let elapsed = last_time.elapsed();
            if elapsed >= Duration::from_secs(1) {
                let fps = frame_count;
                window.set_title(&format!("NES Emulator - FPS: {}", fps));
                frame_count = 0;
                last_time = Instant::now();
            }
            for i in 0..8{
                for j in 0..8{
                    if !ismuted{
                        game_frame.drawpixel(i, j, (255,0,0));
                    }
                    else{
                        game_frame.drawpixel(i, j, (0,0,255));
                    }
                }
            }
            if debugmode{
                ppu.borrow_mut().get_pattern_table(&mut game_frame);
                window.update_with_buffer(game_frame.get_buf().as_slice(), 512, 240)?;
            }
            else{
                window.update_with_buffer(game_frame.get_buf().as_slice(), 256, 240)?;
            }
            
        }
  


    }
    Ok(())
}
