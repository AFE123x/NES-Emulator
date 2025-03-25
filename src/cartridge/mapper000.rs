use super::mapper::Mapper;

pub struct Mapper000{
    pub(crate) n_chr: u8,
    pub(crate) n_prg: u8,
}

impl Mapper for Mapper000{
    fn cpu_read(&self, address: u16,mapped_addr: &mut u32, data: u8) -> bool {
        if *mapped_addr >= 0x8000{
            *mapped_addr &= if self.n_prg > 1 {0x7FFF} else {0x3FFF};
            return true;
        }
        false
    }

    fn cpu_write(&mut self, address: u16,mapped_addr: &mut u32, data: u8) -> bool {
        if *mapped_addr >= 0x8000{
            *mapped_addr &= if self.n_prg > 1 {0x7FFF} else {0x3FFF};
            return true;
        }
        false
    }

    fn ppu_read(&self, address: u16,mapped_addr: &mut u32, data: u8) -> bool {
        if *mapped_addr <= 0x1FFF {
            if self.n_prg > 0{
                *mapped_addr = *mapped_addr;
                return true;
            }
        }
        false
    }

    fn ppu_write(&self, address: u16,mapped_addr: &mut u32, data: u8) -> bool {
        if *mapped_addr <= 0x1FFF {
            if self.n_chr == 0{
                *mapped_addr = *mapped_addr;
                return true;
            }
        }
        false
    }
}
