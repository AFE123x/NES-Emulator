use args::Args;
use bus::Bus;
use cartridge::Cartridge;
use clap::Parser;
use cpu::Cpu;
use minifb::Scale;
use minifb::{Window, WindowOptions};
use ppu::{frame::Frame, Ppu};
use std::cell::RefCell;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::{rc::Rc, time::Instant};


mod args;
mod bus;
mod cartridge;
mod controller;
mod cpu;
mod ppu;

fn main() -> Result<(), Box<dyn Error>> {
    let goalframe = 60;
    let gamecont: Arc<Mutex<bool>> = Arc::new(Mutex::new(true));
    let vec = Args::parse();
    let byte = Arc::new(Mutex::new(0u8));
    /* Initialize peripherals */
    let cartridge = Rc::new(RefCell::new(Cartridge::new(&vec.rom)));
    let mut cpu = Cpu::new();
    let mut game_frame = Frame::new(255, 240);
    let ppu = Rc::new(RefCell::new(Ppu::new(Rc::clone(&cartridge))));
    let mut bus = Bus::new(Rc::clone(&ppu));

    let controller = Rc::new(RefCell::new(controller::Controller::new()));
    let controller2 = Rc::new(RefCell::new(controller::Controller::new()));
    bus.link_cartridge(Rc::clone(&cartridge));
    // Link the bus to the APU so DMC can read from CPU memory
    bus.link_controller1(Rc::clone(&controller));
    bus.link_controller2(Rc::clone(&controller2));

    // Connect APU to the bus

    cpu.linkbus(&mut bus);
    cpu.reset();

    let windowoption = WindowOptions {
        resize: false,
        scale: Scale::X4,
        ..Default::default()
    };
    let mut turbo_counter: u8 = 0;
    let mut last_time = Instant::now();
    let mut frame_count = 0;
    let saverom = Arc::new(Mutex::new(false));
    let mut window = Window::new("NES Emulator - FPS: ", 255, 240, windowoption)?;
    let loadgame = Arc::new(Mutex::new(false));
    let restart = Arc::new(Mutex::new(false));

    let turbob = Arc::new(Mutex::new(false));
    let mut delay_time = 11.0;
    while *gamecont.lock().unwrap() {
        *gamecont.lock().unwrap() = window.is_open();
        if *saverom.lock().unwrap() {
            cartridge.borrow_mut().savestate();
        }
        if *restart.lock().unwrap() {
            cpu.reset();
        }
        if *loadgame.lock().unwrap() {
            cartridge.borrow_mut().load();
        }
        // Clock components
        for _ in 0..3 {
            ppu.borrow_mut().clock(&mut game_frame);
        }
        let _cycles_left = cpu.clock();
        // Call dmc_clock for every CPU cycle
        controller
            .borrow_mut()
            ._set_reg_value(*byte.lock().unwrap());
        if cartridge.borrow_mut().irq() {
            cartridge.borrow_mut().irq_clear();
            cpu.irq();
        }
        if *turbob.lock().unwrap() {
            controller
                .borrow_mut()
                ._set_reg_value(if turbo_counter % 100 == 0 {
                    *byte.lock().unwrap() | 0b0000_0010
                } else {
                    *byte.lock().unwrap() & !0b0000_0010
                });
                turbo_counter += 1;
        }

        if ppu.borrow_mut().get_nmi() {
            cpu.nmi();
            // For slower turbo (15Hz)

            frame_count += 1;
            let elapsed = last_time.elapsed();
            if elapsed >= Duration::from_secs(1) {
                let fps = frame_count;
                window.set_title(&format!("NES Emulator - FPS: {}", fps));
                frame_count = 0;
                last_time = Instant::now();
                if fps > goalframe {
                    delay_time *= 1.01;
                } else if fps < 60 {
                    delay_time *= 0.99;
                }
                // println!("{}",delay_time);
            }
            window.update_with_buffer(game_frame.get_buf().as_slice(), 255, 240)?;
            thread::sleep(Duration::from_millis(delay_time as u64));
        }
    }
    Ok(())
}
