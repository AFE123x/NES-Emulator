extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use rand::Rng;
use std::time::{Instant, Duration};

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-sdl2 static texture", 768, 720)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().present_vsync().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, 256, 240)
        .map_err(|e| e.to_string())?;
    
    let mut event_pump = sdl_context.event_pump()?;
    let mut last_time = Instant::now();
    let mut frame_count = 0;
    'tom: loop {
        let now = Instant::now();
        frame_count += 1;
        let elapsed = now.duration_since(last_time);
        if elapsed >= Duration::from_secs(1) {
            println!("FPS: {}", frame_count);
            frame_count = 0;
            last_time = now;
        }

        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            let mut rng = rand::thread_rng();
            for y in 0..240 {
                for x in 0..256 {
                    let offset = y * pitch + x * 3;
                    let value = if rng.gen_bool(0.5) { 255 } else { 0 };
                    buffer[offset] = value;
                    buffer[offset + 1] = value;
                    buffer[offset + 2] = value;
                }
            }
        })?;

        canvas.clear();
        canvas.copy(&texture, None, Some(Rect::new(0, 0, 768, 720)))?;
        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'tom,
                _ => {}
            }
        }
    }

    Ok(())
}
