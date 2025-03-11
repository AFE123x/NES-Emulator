mod mapper;
mod mapper000;
mod mapper001;
use mapper::Mapper;
use mapper000::Mapper000;
use mapper001::Mapper001;
use std::fs;
struct Header {
    nes_constants: [u8; 4],
    prg_rom_size: u8,
    chr_rom_size: u8,
    mapper: u8,
    name_table_arrangement: Nametable,
}
pub struct Cartridge {
    header: Header,
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
    mapper: Box<dyn Mapper>,
}

pub enum Nametable {
    Vertical,
    Horizontal,
}

impl Cartridge {

    pub fn get_nametable(&self) -> Nametable{
        let nameboi = match self.header.name_table_arrangement{
            Nametable::Vertical => {
                Nametable::Vertical
            },
            Nametable::Horizontal => {
                Nametable::Horizontal
            },
        };
        nameboi
    }
    pub fn new(file_name: &str) -> Self {
        let buf = fs::read(file_name).expect("unable to open file!");
        let lobyte = buf[6] as u16;
        let hibyte = buf[7] as u16;
        let flag = (hibyte << 8) | lobyte;
        let name_table_arrangement = if flag & 1 != 0 {
            Nametable::Horizontal
        } else {
            Nametable::Vertical
        };
        let mapper_num = (flag >> 4) & 0xFF;
        let mapper: Box<dyn Mapper> = match mapper_num {
            0 => Box::new(Mapper000{
                n_chr: buf[5],
                n_prg: buf[4],
            }),
            _ => {
                panic!("Mapper {} not supported!", mapper_num);
            }
        };
        let training = if lobyte & 0x04 != 0 { 512 } else { 0 };
        let header = Header {
            nes_constants: [buf[0], buf[1], buf[2], buf[3]],
            prg_rom_size: buf[4],
            chr_rom_size: buf[5],
            mapper: mapper_num as u8,
            name_table_arrangement: name_table_arrangement,
        };
        let chr_length = header.chr_rom_size as usize * 8192;
        let prg_length = header.prg_rom_size as usize * 16384;
        let mut chr_rom: Vec<u8> = vec![0; chr_length];
        for i in 0..chr_length {
            chr_rom[i] = buf[16 + training + prg_length + i];
        }
        let mut prg_rom: Vec<u8> = vec![0; prg_length];

        for i in 0..prg_length {
            prg_rom[i] = buf[16 + training + i];
        }
        Self {
            header: header,
            prg_rom: prg_rom,
            chr_rom: chr_rom,
            mapper: mapper,
        }
    }

    pub fn cpu_read(&self, address: u16, byte: &mut u8) {
        let mut mapped_addr = address;
        self.mapper.cpu_read(&mut mapped_addr);
        *byte = self.prg_rom[mapped_addr as usize];
    }

    pub fn cpu_write(&mut self, address: u16, byte: u8) {
        let mut mapped_address = address;
        self.mapper.cpu_write(&mut mapped_address);
        self.prg_rom[mapped_address as usize] = byte;
    }

    pub fn ppu_read(&self, address: u16, byte: &mut u8){
        let mut mapped_addr = address;
        self.mapper.ppu_read(&mut mapped_addr);
        *byte = self.chr_rom[mapped_addr as usize];
    }

    pub fn ppu_write(&mut self, address: u16, byte: u8) {
        let mut mapped_address = address;
        self.mapper.ppu_write(&mut mapped_address);
        self.chr_rom[mapped_address as usize] = byte;
    }
}
