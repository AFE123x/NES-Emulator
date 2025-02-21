use crate::cpu::processor::Cpu;
impl Cpu{

    fn fetch(&mut self) -> u8{
        let byte = unsafe { 
            (*self.cpubus.unwrap()).cpu_read(self.pc, false) 
        };
        self.pc = self.pc.wrapping_add(1);
        byte
    }

    fn read(&mut self, address: u16) -> u8{
        unsafe{
            (*self.cpubus.unwrap()).cpu_read(address, false)
        }
    }
    pub fn immediate_addressing(&mut self){
        self.immval = self.fetch();
        self.extra_cycles = 0;
    }

    pub fn zeropage_addressing(&mut self){
        self.abs_addr = self.fetch() as u16;
        self.immval = unsafe{
            (*self.cpubus.unwrap()).cpu_read(self.abs_addr,false)
        };
        self.extra_cycles = 0;
    }

    pub fn zeropagex_addressing(&mut self){
        self.abs_addr = self.fetch() as u16;

        self.abs_addr = self.abs_addr.wrapping_add(self.x as u16);
        self.abs_addr = self.abs_addr & 0x00FF;
        self.immval = unsafe{
            (*self.cpubus.unwrap()).cpu_read(self.abs_addr,false)
        };
        self.extra_cycles = 0;
    }

    pub fn zeropagey_addressing(&mut self){
        self.abs_addr = self.fetch() as u16;
        self.abs_addr = self.abs_addr.wrapping_add(self.y as u16);
        self.abs_addr = self.abs_addr & 0x00FF;
        self.immval = unsafe{
            (*self.cpubus.unwrap()).cpu_read(self.abs_addr,false)
        };
        self.extra_cycles = 0;
    }

    pub fn relative_addressing(&mut self){
        self.relval = self.fetch() as u16;
        if self.relval & 0x0080 != 0 {
            self.relval = self.relval | 0xFF00;
        }
        self.extra_cycles = 0;
    }


    pub fn absolute_addressing(&mut self){
        let lo = self.fetch() as u16;
        let hi = self.fetch() as u16;
        self.abs_addr = (hi << 8) | lo;
        self.extra_cycles = 0;
    }


    pub fn absolutex_addressing(&mut self){
        let lo = self.fetch() as u16;
        let hi = self.fetch() as u16;
        self.abs_addr = (hi << 8) | lo;
        if (self.abs_addr & 0xFF00) != (hi << 8){
            self.extra_cycles = 1;
        }
        else{
            self.extra_cycles = 0;
        }
    }


    pub fn absolutey_addressing(&mut self){
        let lo = self.fetch() as u16;
        let hi = self.fetch() as u16;
        self.abs_addr = (hi << 8) | lo;
        self.abs_addr = self.abs_addr.wrapping_add(self.y as u16);
        if (self.abs_addr & 0xFF00) != (hi << 8){
            self.extra_cycles = 1;
        }
        else{
            self.extra_cycles = 0;
        }

    }

    pub fn indirect_addressing(&mut self){
        let lo = self.fetch() as u16;
        let hi = self.fetch() as u16;
        let ptr = (hi << 8) | lo;
        if lo == 0x00FF{
            let hi = ((self.read((ptr as u16) & 0xFF00) as u16) << 8)as u16;
            let lo = self.read(ptr + 0) as u16;
            self.abs_addr = hi | lo;
        }
        else{
            let hi = ((self.read(ptr + 1) as u16) << 8) as u16;
            let lo = self.read(ptr + 0) as u16;
            self.abs_addr = hi | lo;
        }
    }

    pub fn indexedindirect_addressing(&mut self){
        let temp = self.fetch() as u16;
        let lo = self.read(temp.wrapping_add(self.x as u16)) as u16;
        let hi = self.read(temp.wrapping_add(self.x as u16).wrapping_add(1)) as u16;
        self.abs_addr = (hi << 8) | lo;
    }

    /*
uint8_t olc6502::IZY()
{
	uint16_t t = read(pc);
	pc++;

	uint16_t lo = read(t & 0x00FF);
	uint16_t hi = read((t + 1) & 0x00FF);

	addr_abs = (hi << 8) | lo;
	addr_abs += y;
	
	if ((addr_abs & 0xFF00) != (hi << 8))
		return 1;
	else
		return 0;
}

     */
    pub fn indirect_indexed(&mut self){
        let temp = self.fetch() as u16;
        let lo = self.read(temp & 0x00FF) as u16;
        let hi = self.read(temp.wrapping_add(1) & 0x00FF) as u16;
        self.abs_addr = (hi << 8) | lo;
        self.abs_addr += self.y as u16;

        if (self.abs_addr & 0xFF00) != (hi << 8){
            self.extra_cycles = 1;
        }
        else{
            self.extra_cycles = 0;
        }
    }

    pub fn implied_addressing(&mut self){
        let x = 0;
    }

    pub fn accumulator_addressing(&mut self){
        let x = 0;
    }
}
