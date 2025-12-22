pub struct Bus{
    ram: Vec<u8>,
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            ram: vec![0; 0x10000],
        }
    }

    pub fn cpu_read(&self, addr: u16) -> Option<u8> {
        let val = self.ram[addr as usize];
        Some(val)
    }

    pub fn cpu_write(&mut self, addr: u16, data: u8) -> bool {
        self.ram[addr as usize] = data;
        true
    }
}
