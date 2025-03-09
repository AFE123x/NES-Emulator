use super::mapper::Mapper;

pub struct Mapper000{
    pub(crate) n_chr: u8,
    pub(crate) n_prg: u8,
}

impl Mapper for Mapper000{
    fn cpu_read(&self, address: &mut u16) -> bool {
        if *address >= 0x8000{
            *address &= if self.n_prg > 1 {0x7FFF} else {0x3FFF};
            return true;
        }
        false
    }

    fn cpu_write(&self, address: &mut u16) -> bool {
        if *address >= 0x8000{
            *address &= if self.n_prg > 1 {0x7FFF} else {0x3FFF};
            return true;
        }
        false
    }

    fn ppu_read(&self, address: &mut u16) -> bool {
        if *address <= 0x1FFF {
            *address = *address;
            return true;
        }
        false
    }

    fn ppu_write(&self, address: &mut u16) -> bool {
        if *address <= 0x1FFF {
            if self.n_chr == 0{
                *address = *address;
                return true;
            }
        }
        false
    }
}
