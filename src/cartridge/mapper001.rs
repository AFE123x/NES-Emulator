use super::mapper::Mapper;

pub struct Mapper001;

impl Mapper for Mapper001{
    fn cpu_read(&self, address: &mut u16) -> bool {
        todo!()
    }

    fn cpu_write(&self, address: &mut u16) -> bool {
        todo!()
    }

    fn ppu_read(&self, address: &mut u16) -> bool {
        todo!()
    }

    fn ppu_write(&self, address: &mut u16) -> bool {
        todo!()
    }
}
