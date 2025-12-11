pub struct Bus{
    ram: Vec<u8>,
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            ram: vec![0; 0x10000],
        }
    }

    pub fn read(&self, addr: usize) -> u8 {
        self.ram[addr]
    }

    pub fn write(&mut self, addr: usize, data: u8) {
        self.ram[addr] = data;
    }
}