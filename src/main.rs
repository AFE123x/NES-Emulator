use std::error::Error;
mod ppu;
use bus::Bus;
use cartridge::Cartridge;
use cpu::Cpu;
use ppu::{Frame::Frame, Ppu as p};
use sdl2::{
    keyboard::Keycode,
    pixels::{Color, PixelFormatEnum},
};
mod cartridge;

mod cpu;

mod bus;

fn main() -> Result<(), Box<dyn Error>> {
    let mut cpu = Cpu::new();
    let mut Cartridge = Cartridge::new("roms/nestest.nes".to_string());
    let mut Frame = Frame::new(256, 240);
    let mut ppu = p::new(&mut Cartridge, &mut Frame);
    let mut bus = Bus::new(&mut cpu, &mut Cartridge, &mut ppu);

    let sdl_context = sdl2::init()?;
    let video = sdl_context.video()?; //initialize the video subsystem.
    let window = video.window("Rust EMU", 256 * 4, 240 * 4).build()?;
    let mut canvas = window.into_canvas().build()?;
    canvas.set_scale(4.0, 4.0)?;
    let texture = canvas.texture_creator();
    let mut texture = texture.create_texture_target(PixelFormatEnum::RGB24, 128, 128)?;

    cpu.linkbus(&mut bus);
    let mut quit = true;
    let mut pump = sdl_context.event_pump()?;
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    cpu.reset();
    while quit {
        while let Some(i) = pump.poll_event() {
            match i {
                sdl2::event::Event::Quit { .. }
                | sdl2::event::Event::KeyDown {
                    keycode: Some(Keycode::ESCAPE),
                    ..
                } => quit = false,
                _ => {}
            }
        }

        bus.clock();
        
        if ppu.get_frame_com(){
            cpu.nmi();
            texture.update(None, Frame.get_buf(), 256 * 3)?;
            canvas.copy(&texture, None, None)?;
            ppu.reset_frame_com();
            canvas.present();
        }
    }
    Ok(())
}
