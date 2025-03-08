use std::fs;

pub struct Cartridge {
    numchrrom: u8,
    numprgrom: u8,
    mapper_num: u8,
    chr_rom: Vec<u8>,
    prg_rom: Vec<u8>,
    nametable_orientation: Nametable,
}

pub enum Nametable{
    Vertical,
    Horizontal,
}

impl Cartridge {
    pub fn new(file: String) -> Self {
        let buffer = fs::read(file);
        let buffer = match buffer {
            Ok(buf) => buf,
            Err(e) => {
                panic!("Error - {}",e.to_string());
            },
        };
        let prg_rom_size = buffer[4];
        let mut prg_rom:Vec<u8> = vec![0;prg_rom_size as usize * 16384];
        let chr_rom_size = buffer[5];
        let mut chr_rom:Vec<u8> = vec![0;chr_rom_size as usize * 8192];
        let lo_byte_mapper = buffer[6];
        let hi_byte_mapper = buffer[7];
        let mapper = (hi_byte_mapper & 0xF0) | (lo_byte_mapper >> 4);
        if mapper != 0{
            panic!("At this moment, mapper 000 is the only supported option D:");
        }
        println!("prg rom size: {}, chr_rom size {}",prg_rom_size,chr_rom_size);
        let training_data = if buffer[6] & 0x4 != 0 {512} else {0};
        let length = prg_rom.len();
        for i in 0..length{
            prg_rom[i] = buffer[16 + training_data + i];
        }
        let clength = chr_rom.len();
        for i in 0..clength{
            chr_rom[i] = buffer[16 + training_data + length + i];
        }

        let orientation = if lo_byte_mapper & 0x1 != 0 {Nametable::Vertical} else {Nametable::Horizontal};
        println!("{}",mapper);
        Self {
            numchrrom: chr_rom_size,
            numprgrom: prg_rom_size,
            mapper_num: mapper,
            chr_rom: chr_rom,
            prg_rom: prg_rom,
            nametable_orientation: orientation,
        }
    }

    pub fn cpu_read(&self,address: u16) -> u8{
        let translated_addr = if self.numprgrom > 1 {address & 0x7FFF} else {address & 0x3FFF};
        let returnval: u8 = self.prg_rom[translated_addr as usize];
        returnval
    }

    pub fn cpu_write(&mut self,address: u16, byte: u8){
        let translated_addr = if self.numprgrom > 1 {address & 0x7FFF} else {address & 0x3FFF};
        self.prg_rom[translated_addr as usize] = byte;
    }

    pub fn ppu_read(&self, address: u16) -> u8{
        let returnval = self.chr_rom[address as usize & 0x1FFF];
        returnval
    }

    pub fn ppu_write(&mut self, address: u16, byte: u8){
        let x =  address + byte as u16;
    }
}
