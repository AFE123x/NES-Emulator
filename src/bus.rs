

pub struct Bus {
    memory: Vec<u8>,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            memory: vec![0; 65536],
        }
    }

    pub fn cpu_read(&self, address: u16, rdonly: bool) -> u8 {
        self.memory[address as usize]
    }
    
    pub fn cpu_write(&mut self, address: u16, byte: u8) {
        self.memory[address as usize] = byte;
    }
}


