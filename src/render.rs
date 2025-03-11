use std::error::Error;

use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::{Color, PixelFormatEnum},
    rect::Rect,
    sys::{KeyCode, SDL_QuitEvent},
};

use crate::{
    bus::Bus,
    cartridge::Cartridge,
    cpu::Cpu,
    ppu::{frame::Frame, Ppu},
};

pub fn gameloop(rom_file: &str, scale: u32) -> Result<(), Box<dyn Error>> {
    /* initialize peripherals */
    let mut cartridge = Cartridge::new(rom_file);
    let mut cpu = Cpu::new();
    let mut ppu = Ppu::new(&mut cartridge);
    let mut bus = Bus::new(&mut cpu, &mut cartridge, &mut ppu);
    cpu.linkbus(&mut bus);
    cpu.reset();

    /* initialize sdl2 */
    let sdl_content = sdl2::init()?;
    let window_subsystem = sdl_content.video()?;
    let game_window = window_subsystem
        .window("NES Emulator", 512 * scale, 240 * scale)
        .build()?;
    let mut game_canvas = game_window.into_canvas().build()?;
    game_canvas.set_scale(scale as f32, scale as f32)?;
    let text_creator = game_canvas.texture_creator();
    let mut main_game_texture =
        text_creator.create_texture_target(PixelFormatEnum::RGB24, 256, 240)?;
    let mut palette_texture =
        text_creator.create_texture_target(PixelFormatEnum::RGB24, 256, 128)?;
    let mut pump = sdl_content.event_pump()?;
    let mut cont_game = true;
    /* initialize game frame! */
    let mut palette_frame = Frame::new(256, 128); //frame buffer for the palette data.
    let mut game_frame = Frame::new(256, 240); //frame buffer for the actual game.

    game_canvas.set_draw_color(Color::RGB(0, 0, 255));

    /* Game loop */
    while cont_game {
        for i in pump.poll_iter() {
            match i {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::ESCAPE),
                    ..
                } => {
                    cont_game = false;
                }
                _ => {}
            }
        }
        bus.clock();
        
        if ppu.get_nmi(){
            cpu.nmi();
            ppu.get_palette_table(&mut palette_frame);
            palette_texture.update(None, &palette_frame.get_buf(), 256 * 3)?;
            main_game_texture.update(None, &game_frame.get_buf(), 256 * 3)?;
            game_canvas.clear();
            game_canvas.copy(&palette_texture, None, Rect::new(256, 0, 256, 128))?;
            game_canvas.copy(&main_game_texture, None, Rect::new(0, 0, 256, 240))?;
            game_canvas.present();
        }
    }

    Ok(())
}
