use super::mapper::Mapper;

pub struct Mapper002 {
    bank_select: u8,
    prg_rom: u8,
    chr_rom: u8,
    last_bank_address: u32,
}

impl Mapper002 {
    pub fn new(prg_rom: u8, chr_rom: u8) -> Self {
        println!("prg_rom {}",prg_rom);
        let last_bank_address: u32 = ((prg_rom - 1) as u32) * 0x4000;
        Self {
            bank_select: 0,
            prg_rom: prg_rom,
            chr_rom: chr_rom,
            last_bank_address: last_bank_address,
        }
    }
}
impl Mapper for Mapper002 {
    /*
    CPU $8000-$BFFF: 16 KB switchable PRG ROM bank
    CPU $C000-$FFFF: 16 KB PRG ROM bank, fixed to the last bank
     */
    fn cpu_read(&self, address: u16,mapped_addr: &mut u32, data: u8) -> bool {
        if *mapped_addr >= 0x8000 && *mapped_addr <= 0xBFFF {
            *mapped_addr = ((self.bank_select as u32) * 0x4000) + (*mapped_addr & 0x3FFF);
            return true;
        } else if *mapped_addr >= 0xC000{
            *mapped_addr = self.last_bank_address + (*mapped_addr & 0x3FFF);
            return true;
        }
        return false;
    }

    fn cpu_write(&mut self, address: u16,mapped_addr: &mut u32, data: u8) -> bool {
        if address >= 0x8000{
            let data = data & 0xF;
            self.bank_select = data;
            return true;
        }
        return false;

    }

    fn ppu_read(&self, address: u16,mapped_addr: &mut u32, data: u8) -> bool {
        if *mapped_addr <= 0x1FFF {
            if self.prg_rom > 0{
                *mapped_addr = *mapped_addr;
                return true;
            }
        }
        false
    }

    fn ppu_write(&self, address: u16,mapped_addr: &mut u32, data: u8) -> bool {
        if *mapped_addr <= 0x1FFF {
            if self.chr_rom == 0{
                *mapped_addr = *mapped_addr;
                return true;
            }
        }
        false
    }
}
