use super::mapper::Mapper;

pub struct Mapper001;

impl Mapper for Mapper001{
    fn cpu_read(&self, address: u16,mapped_addr: &mut u32, data: u8) -> bool {
        println!("{:4x}",address);
        todo!()
    }

    fn cpu_write(&mut self, address: u16,mapped_addr: &mut u32, data: u8) -> bool {
        println!("{:4x}",address);
        todo!()
    }

    fn ppu_read(&self, address: u16,mapped_addr: &mut u32, data: u8) -> bool {
        println!("{:4x}",address);
        todo!()
    }

    fn ppu_write(&self, address: u16,mapped_addr: &mut u32, data: u8) -> bool {
        println!("{:4x}",address);
        todo!()
    }
}
