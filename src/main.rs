use apu::Apu;
use args::Args;
use bus::Bus;
use cartridge::Cartridge;
use clap::Parser;
use cpu::Cpu;
use device_query::Keycode;
use device_query::{DeviceQuery, DeviceState};
use minifb::Scale;
use minifb::{Window, WindowOptions};
use ppu::{frame::Frame, Ppu};
use std::cell::RefCell;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::{rc::Rc, time::Instant};

mod apu;
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
    let apu: Rc<RefCell<Apu>> = Rc::new(RefCell::new(Apu::new()));
    bus.link_cartridge(Rc::clone(&cartridge));
    bus.link_apu(Rc::clone(&apu));
    // Link the bus to the APU so DMC can read from CPU memory
    apu.borrow_mut().link_bus(&mut bus);
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
    let button_state = Arc::clone(&byte);
    let game_running = Arc::clone(&gamecont);
    let clonesave = saverom.clone();
    let loadgame = Arc::new(Mutex::new(false));
    let restart = Arc::new(Mutex::new(false));
    let restartclone = restart.clone();
    let mute = Arc::new(Mutex::new(false));
    let muteclone = mute.clone();
    let turboa = Arc::new(Mutex::new(false));
    let turboaclone = turboa.clone();

    let turbob = Arc::new(Mutex::new(false));

    let loadgameclone = loadgame.clone();
    let turbobclone = turbob.clone();

    let thread = thread::spawn(move || {
        let device_state = DeviceState::new();
        while *game_running.lock().unwrap() {
            let keys = device_state.get_keys();
            let mut output = 0u8;

            if keys.contains(&Keycode::A) {
                output |= 0b0000_0001;
            } // A
            if keys.contains(&Keycode::S) {
                output |= 0b0000_0010;
            } // B
            if keys.contains(&Keycode::D) {
                output |= 0b0000_0100;
            } // Select
            if keys.contains(&Keycode::F) {
                output |= 0b0000_1000;
            } // Start
            if keys.contains(&Keycode::Up) {
                output |= 0b0001_0000;
            } else if keys.contains(&Keycode::Down) {
                output |= 0b0010_0000;
            }
            if keys.contains(&Keycode::X) {
                *turbobclone.lock().unwrap() = true;
            }
            if keys.contains(&Keycode::Left) {
                output |= 0b0100_0000;
            } else if keys.contains(&Keycode::Right) {
                output |= 0b1000_0000;
            }
            *loadgameclone.lock().unwrap() = keys.contains(&Keycode::P);
            *turboaclone.lock().unwrap() = keys.contains(&Keycode::Z);
            *turbobclone.lock().unwrap() = keys.contains(&Keycode::X);
            *muteclone.lock().unwrap() = keys.contains(&Keycode::M);
            *restartclone.lock().unwrap() =
                keys.contains(&Keycode::Command) && keys.contains(&Keycode::O);
            *clonesave.lock().unwrap() = keys.contains(&Keycode::Semicolon);
            *button_state.lock().unwrap() = output;
        }
    });
    let mut delay_time = 11.0;
    while *gamecont.lock().unwrap() {
        *gamecont.lock().unwrap() = window.is_open();
        if *saverom.lock().unwrap() {
            cartridge.borrow_mut().savestate();
        }
        if *restart.lock().unwrap() {
            cpu.reset();
        }
        if *mute.lock().unwrap() {
            apu.borrow_mut().toggle_sound();
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
        apu.borrow_mut().dmc_clock();
        bus.cpu_write(0xAA, 0x13);
        //bus.cpu_write(0x32, 0xff);
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
    thread.join().unwrap();
    Ok(())
}
