use std::error::Error;
mod ppu;
use bus::Bus;
use cartridge::Cartridge;
use cpu::Cpu;
use ppu::{Ppu as p, Frame::Frame};
use sdl2::{keyboard::Keycode, pixels::{Color, PixelFormatEnum}};
mod cartridge;

mod cpu;

mod bus;

fn main() -> Result<(),Box<dyn Error>> {
    let mut cpu = Cpu::new();
    let mut Cartridge = Cartridge::new("roms/mario.nes".to_string());
    let mut ppu = p::new(&mut Cartridge);
    let mut bus = Bus::new(&mut cpu,&mut Cartridge,&mut ppu);

    let sdl_context = sdl2::init()?;
    let video = sdl_context.video()?; //initialize the video subsystem.
    let window = video.window("Rust EMU", 128 * 4, 128 * 2 * 4).build()?;
    let mut canvas = window.into_canvas().build()?;
    canvas.set_scale(4.0, 4.0)?;
    let texture = canvas.texture_creator();
    let mut texture = texture.create_texture_target(PixelFormatEnum::RGB24, 128, 128)?;
    let frfr = ppu.create_palette_table();
    
    cpu.linkbus(&mut bus);
    let mut quit = true;
    let mut pump = sdl_context.event_pump()?;
    while quit {
        while let Some(i) = pump.poll_event(){
            match i{ 
                sdl2::event::Event::Quit { .. } | sdl2::event::Event::KeyDown { keycode: Some(Keycode::ESCAPE), .. } => quit = false,
                _ => {}
            }
        }
        
        bus.clock();
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        texture.update(None, &frfr.get_buf(), 128 * 3)?;
        canvas.clear();
        canvas.copy(&texture,None,None)?;
        canvas.present();
    }
    Ok(())
}
