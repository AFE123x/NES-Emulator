mod bus;
mod cartridge;
mod cpu;
mod frame;
pub mod ppu;
use bus::cpubus::Cpubus;
use cartridge::cartridge::Cartridge;
use cpu::processor::Cpu;
use ppu::ppu::Ppu;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use std::{env, fs};
fn main() {
    let args: Vec<String> = env::args().collect();
    let args = ["aoeu".to_string(), args[1].to_string()];
    let mut cpu = Cpu::new();
    let buf = match fs::read(&args[1]) {
        Ok(str) => str,
        Err(_) => todo!(),
    };

    let mut cartridge = Cartridge::new(buf);
    let mut ppu = Ppu::new(&mut cartridge);
    let mut bus = Cpubus::new(&mut cpu, &mut cartridge, &mut ppu);
    let dream = ppu.make_pallet_table(0, 2);
    cpu.linkbus(&mut bus);
    cpu.reset();
    // Start the clock loop
    let sdl_context = sdl2::init().unwrap();
    let video_subsys = sdl_context.video().unwrap();
    let window = video_subsys
        .window("NES Emulator", 128 * 3, 128 * 3)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.set_scale(3.0, 3.0).unwrap();
    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGB24, 128, 128)
        .unwrap();

    texture.update(None, &dream.get_buf(), 128 * 3).expect("Failed to update texture");
    canvas.copy(&texture, None, None).unwrap();
    canvas.present();
    loop{
        
    }
    loop {
        bus.clock();
    }
}
