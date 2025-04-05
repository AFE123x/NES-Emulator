//! # Cartridge
//! - This is the implementation of the cartridge on the NES.

mod mapper;
mod mapper000;
mod mapper001;
mod mapper002;
use mapper::Mapper;
use mapper000::Mapper000;
use mapper002::Mapper002;
use mapper001::Mapper001;

use std::fs;
#[derive(Debug)]
struct Header {
    prg_rom_size: u8,
    chr_rom_size: u8,
    _mapper: u8,
    name_table_arrangement: Nametable,
}
pub struct Cartridge {
    header: Header,
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
    mapper: Box<dyn Mapper>,
}

#[derive(Debug)]
pub enum Nametable {
    Vertical,
    Horizontal,
}

impl Cartridge {
    pub fn get_nametable(&self) -> Nametable {
        let nameboi = match self.header.name_table_arrangement {
            Nametable::Vertical => Nametable::Vertical,
            Nametable::Horizontal => Nametable::Horizontal,
        };
        nameboi
    }
    ///# `new` - Constructs a new cartridge
    pub fn new(file_name: &str) -> Self {
        let buf = fs::read(file_name).expect("unable to open file!");
        let header = &buf[0..16];
        let prg_rom_size = header[4] as usize;
        let chr_rom_size = header[5] as usize;
        let prg_rom_len = prg_rom_size * 16 * 1024;
        let chr_rom_len = chr_rom_size * 8 * 1024;
        let mut prg_rom: Vec<u8> = vec![0; prg_rom_len];

        let training_data = if header[6] & 0x04 != 0 {512} else {0};
        for i in 0..prg_rom_len{
            prg_rom[i] = buf[16 + training_data + i]
        }
        let chr_rom:Vec<u8> =  if chr_rom_size > 0{
            let mut vec: Vec<u8> = vec![0; chr_rom_len];
            for i in 0..chr_rom_len{
                vec[i] = buf[16 + training_data + prg_rom_len + i];
            }
            vec
        }
        else{
            vec![0; 8192]
        };
        let nametable_arrangement = match header[6] & 1 {
            0 => Nametable::Horizontal,
            1 => Nametable::Vertical,
            _ => panic!("impossible"),
        };
        let mapper = (header[7] & 0xF0) | (header[6] >> 4);
        let header = Header {
            prg_rom_size: prg_rom_size as u8,
            chr_rom_size: chr_rom_size as u8,
            _mapper: mapper,
            name_table_arrangement: nametable_arrangement,
        };
        let mapper: Box<dyn Mapper> = match mapper{
            0 => Box::new(Mapper000 { n_chr: chr_rom_size as u8, n_prg: prg_rom_size as u8 }),
            2 => Box::new(Mapper002::new(prg_rom_size as u8, chr_rom_size as u8)),
            1 => Box::new(Mapper001::new(prg_rom_size as u8, chr_rom_size as u8)),
            _ => panic!("mapper {} not supported",mapper),
        };
        println!("{:?}",header);
        Self {
            header: header,
            prg_rom: prg_rom,
            chr_rom: chr_rom,
            mapper: mapper,
        }
    }

    pub fn cpu_read(&self, address: u16, byte: &mut u8) {
        let mut mapped_addr = address as u32;
        let res = self.mapper.cpu_read(address,&mut mapped_addr,*byte);
        if res {
            *byte = self.prg_rom[mapped_addr as usize];
        }
    }

    pub fn cpu_write(&mut self, address: u16, byte: u8) {
        let mut mapped_address = address as u32;
        let res = self.mapper.cpu_write(address,&mut mapped_address,byte);
        if res {
            self.prg_rom[mapped_address as usize] = byte;
        }
    }

    pub fn ppu_read(&self, address: u16, byte: &mut u8) {
        let mut mapped_addr = address as u32;
        let res = self.mapper.ppu_read(address,&mut mapped_addr,*byte);
        if res {
            
            *byte = self.chr_rom[mapped_addr as usize];
        }
    }

    pub fn ppu_write(&mut self, address: u16, byte: u8) {
        let mut mapped_address = address as u32;
        let res = self.mapper.ppu_write(address,&mut mapped_address,byte);
        if res {
            self.chr_rom[mapped_address as usize] = byte;
        }
    }
}
