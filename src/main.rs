use std::{error::Error, thread::sleep_ms};
mod ppu;
use bus::Bus;
use cartridge::Cartridge;
use cpu::Cpu;
use ppu::{frame::Frame, Ppu};
use sdl2::{keyboard::Keycode, pixels::{Color, PixelFormatEnum}};
mod cartridge;
mod renderer;
mod cpu;

mod bus;

fn main() -> Result<(),Box<dyn Error>> {
    let mut cpu = Cpu::new();
    let mut cartridge = Cartridge::new("roms/mario.nes");
    let mut main_frame = Frame::new(256, 240);
    let mut ppu = Ppu::new(&mut cartridge,&mut main_frame);
    let mut bus = Bus::new(&mut cpu,&mut cartridge,&mut ppu);
    let sdl_context = sdl2::init()?;
    let video = sdl_context.video()?; //initialize the video subsystem.
    let window = video.window("Rust EMU", 256 * 4, 240 * 4).build()?;
    let mut canvas = window.into_canvas().build()?;
    canvas.set_scale(4.0, 4.0)?;
    let texture = canvas.texture_creator();
    let mut texture = texture.create_texture_target(PixelFormatEnum::RGB24, 128, 128)?;
    let _frfr = ppu.create_palette_table();
    
    cpu.linkbus(&mut bus);
    let mut quit = true;
    let mut pump = sdl_context.event_pump()?;
    cpu.reset();
    while quit {
        while let Some(i) = pump.poll_event(){
            match i{ 
                sdl2::event::Event::Quit { .. } | sdl2::event::Event::KeyDown { keycode: Some(Keycode::ESCAPE), .. } => quit = false,
                _ => {}
            }
        }
        bus.clock();
        if ppu.get_enable_interrupt(){
            cpu.nmi();
        }
        if ppu.get_frame_comp(){
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            texture.update(None, &main_frame.get_buf(), 256 * 3)?;
            canvas.clear();
            canvas.copy(&texture,None,None)?;
            canvas.present();
        }
    }
    Ok(())
}
