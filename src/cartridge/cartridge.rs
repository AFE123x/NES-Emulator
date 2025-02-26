pub mod mapper;

use crate::cartridge::cartridge::mapper::Map;

use super::mapper000::Mapper000;
pub enum Mirror{
    VERTICAL,
    HORIZONTAL,
}
pub struct Cartridge{
    mirror: Mirror,
    chr_rom: Vec<u8>,
    prg_rom: Vec<u8>,
    mapper: Box<dyn Map>,
}

impl Cartridge{
    pub fn new(buffer: Vec<u8>) -> Self{
        let mappernum = (&buffer[6] >> 4) | (&buffer[7] & 0xF0);
        let mapper: Box<dyn Map> = match mappernum{
            0 => {
                Box::new(Mapper000::new(buffer[4], buffer[5]))
            },
            _ => {
                panic!("Mapper {} not supported!",mappernum);
            }
        };

        let num_skip: usize = if buffer[6] & 4 != 0 { 512 + 16 } else { 16 };
        let mirror = if buffer[6] & 1 != 0 {Mirror::VERTICAL} else {Mirror::HORIZONTAL};
        let prg_size: usize = buffer[4] as usize * 16384;
        
        let prg_rom: Vec<u8> = buffer[num_skip..num_skip + prg_size].to_vec();
        let num_skip = num_skip + prg_size;
        let chr_size = buffer[5] as usize * 8192;
        let chr_rom: Vec<u8> = buffer[num_skip..num_skip + chr_size].to_vec();
        let temp = Cartridge{
            chr_rom: chr_rom,
            prg_rom: prg_rom,
            mapper: mapper,
            mirror: mirror,
        };
        return temp;
    }
    pub fn get_mirror(&self) -> &Mirror{
        &self.mirror
    }
    pub fn cpuread(&self,address: u16, readonly: bool) -> (u8,bool){
        let (mapped,success) = self.mapper.cpuread(address);
        if success {
            let data = self.prg_rom[mapped as usize];
            return (data,true);
        }
        else{
            return (0,false);
        }
    }

    pub fn cpu_write(&mut self, address: u16, data: u8) -> bool{
        let (mapped, sucess) = self.mapper.cpuwrite(address,0);
        if sucess{
            self.chr_rom[mapped as usize] = data;
            return true;
        }
        return false;

    }

    pub fn ppu_write(&mut self, addr: u16, data: u8) -> bool{
        let mut mapped_addr: u16 = 0;
        if self.mapper.ppu_map_write(addr, &mut mapped_addr){
            self.chr_rom[mapped_addr as usize] = data;
            return true;
        }
        return false;
    }

    pub fn ppu_read(&self, addr: u16, data: &mut u8) -> bool {
        let mut mapped_addr: u16 = 0;
        if self.mapper.ppu_map_read(addr, &mut mapped_addr){
            *data = self.chr_rom[mapped_addr as usize];
            return true;
        }
        return false;
    }
}