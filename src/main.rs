use std::env;

mod render;
mod cartridge;
mod cpu;
mod bus;
mod ppu;
fn main(){
    // let args: Vec<String> = env::args().collect();
    let status = render::gameloop("roms/nestest.nes",3);
    match status{
        Ok(_) => {},
        Err(e) => panic!("error: {}",e.to_string()),
    }
}