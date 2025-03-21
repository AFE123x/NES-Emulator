use std::env;

mod render;
mod cartridge;
mod cpu;
mod bus;
mod ppu;
mod controller;
fn main(){
    // let args: Vec<String> = env::args().collect();
    // if args.len() < 2{
    //     panic!("cargo run <rom-file> <scale>");
    // }
    // let scale: u8 = args[2].parse().unwrap();
    // let status = render::gameloop(&args[1],scale as u32);
    let status = render::gameloop("roms/nestest.nes", 1);
    match status{
        Ok(_) => {},
        Err(e) => panic!("error: {}",e.to_string()),
    }
}