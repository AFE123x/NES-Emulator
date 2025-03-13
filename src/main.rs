use std::env;

mod render;
mod cartridge;
mod cpu;
mod bus;
mod ppu;
mod controller;
fn main(){
    let args: Vec<String> = env::args().collect();
    let status = render::gameloop(&args[1],3);
    match status{
        Ok(_) => {},
        Err(e) => panic!("error: {}",e.to_string()),
    }
}