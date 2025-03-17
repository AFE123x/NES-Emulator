use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::{Color, PixelFormatEnum},
    rect::Rect,
};
use std::{
    error::Error,
    time::{Duration, Instant},
};

use crate::{
    bus::Bus,
    cartridge::Cartridge,
    controller::{self, Buttons},
    cpu::Cpu,
    ppu::{Frame::Frame, Ppu},
};

pub fn gameloop(rom_file: &str, scale: u32) -> Result<(), Box<dyn Error>> {
    /* initialize peripherals */
    let mut cartridge = Cartridge::new(rom_file);
    let mut cpu = Cpu::new();
    let mut game_frame = Frame::new(256, 240); //frame buffer for the actual game.
    let mut ppu = Ppu::new(&mut cartridge);
    let mut bus = Bus::new(&mut cpu, &mut cartridge, &mut ppu);
    cpu.linkbus(&mut bus);
    cpu.reset();
    let mut controller = controller::Controller::new();
    bus.link_controller(&mut controller);
    /* initialize sdl2 */
    let sdl_content = sdl2::init()?;
    let window_subsystem = sdl_content.video()?;
    let game_window = window_subsystem
        .window("NES Emulator", 512 * scale, 240 * scale)
        .position_centered()
        .build()?;
    let mut game_canvas = game_window.into_canvas().present_vsync().build()?;
    game_canvas.set_scale(scale as f32, scale as f32)?;
    let text_creator = game_canvas.texture_creator();
    let mut main_game_texture =
        text_creator.create_texture_target(PixelFormatEnum::RGB24, 256, 240)?;
    let mut palette_texture =
        text_creator.create_texture_target(PixelFormatEnum::RGB24, 256, 128)?;
    let mut pump = sdl_content.event_pump()?;
    let mut cont_game = true;

    /* initialize game frame! */
    let mut pattern_frame = Frame::new(256, 128); //frame buffer for the palette data.
    ppu.linkpattern(&mut game_frame);
    /* FPS Measurement */

    let mut fps_timer = Instant::now();

    /* Game loop */
    while cont_game {
        if fps_timer.elapsed() >= Duration::from_millis(17) { //17
            match pump.poll_event() {
                Some(event) => match event {
                    Event::Quit { .. } | Event::KeyDown {keycode: Some(Keycode::ESCAPE),..} => cont_game = false,
                    Event::KeyDown { keycode: Some(Keycode::P),.. } => ppu.set_bg_palette_num(),
                    Event::KeyDown { keycode,.. } => match keycode{
                        Some(kcode) => match kcode{
                            Keycode::UP => controller.set_button(Buttons::Up, true),
                            Keycode::DOWN => controller.set_button(Buttons::Down, true),
                            Keycode::LEFT => controller.set_button(Buttons::Left, true),
                            Keycode::RIGHT => controller.set_button(Buttons::Right, true),
                            Keycode::A => controller.set_button(Buttons::A, true),
                            Keycode::O => controller.set_button(Buttons::B, true),
                            Keycode::E => controller.set_button(Buttons::Select, true),
                            Keycode::U => controller.set_button(Buttons::Start, true),

                            _ => {}
                        },
                        None => {},
                    },
                    Event::KeyUp { keycode, .. } => match keycode{
                        Some(kcode) => match kcode{
                                Keycode::UP => controller.set_button(Buttons::Up, false),
                                Keycode::DOWN => controller.set_button(Buttons::Down, false),
                                Keycode::LEFT => controller.set_button(Buttons::Left, false),
                                Keycode::RIGHT => controller.set_button(Buttons::Right, false),
                                Keycode::A => controller.set_button(Buttons::A, false),
                                Keycode::O => controller.set_button(Buttons::B, false),
                                Keycode::E => controller.set_button(Buttons::Select, false),
                                Keycode::U => controller.set_button(Buttons::Start, false),
                                _ => {}
                        },
                        None => {},
                    }
                    _ => {}
                },
                None => {}
            }
            fps_timer = Instant::now();
        }

        bus.clock();

        if ppu.get_nmi() {
            cpu.nmi();
            ppu.set_name_table();
            ppu.get_pattern_table(&mut pattern_frame, 3);
            palette_texture.update(None, &pattern_frame.get_buf(), 256 * 3)?;
            main_game_texture.update(None, &game_frame.get_buf(), 256 * 3)?;
            game_canvas.clear();
            game_canvas.copy(&palette_texture, None, Rect::new(256, 0, 256, 128))?;
            game_canvas.copy(&main_game_texture, None, Rect::new(0,0,256,240))?;
            game_canvas.present();
        }
    }

    Ok(())
}
