use crate::cpu::processor::Cpu;

use super::Flags;

impl Cpu {
    /// Sets or clears a specific condition flag.
    /// 
    /// # Arguments
    /// * `flag` - The flag to be modified.
    /// * `state` - `true` to set the flag, `false` to clear it.
    fn setflag(&mut self, flag: Flags, state: bool) {
        match flag {
            Flags::Negative => {
                self.flags = if state { self.flags | 0x80 } else { self.flags & !0x80 };
            }
            Flags::Overflow => {
                self.flags = if state { self.flags | 0x40 } else { self.flags & !0x40 };
            }
            Flags::Break => {
                self.flags = if state { self.flags | 0x10 } else { self.flags & !0x10 };
            }
            Flags::Decimal => {
                self.flags = if state { self.flags | 0x08 } else { self.flags & !0x08 };
            }
            Flags::Interrupt => {
                self.flags = if state { self.flags | 0x04 } else { self.flags & !0x04 };
            }
            Flags::Zeroflag => {
                self.flags = if state { self.flags | 0x02 } else { self.flags & !0x02 };
            }
            Flags::Carry => {
                self.flags = if state { self.flags | 0x01 } else { self.flags & !0x01 };
            }
        };
    }

    /// Loads an immediate value into the accumulator register (`A`).
    /// 
    /// Updates the Zero and Negative flags based on the value loaded.
    pub fn LDA(&mut self) {
        self.a = self.immval;
        self.setflag(Flags::Zeroflag, self.a == 0);
        self.setflag(Flags::Negative, (self.a & 0x80) != 0);
    }
}

#[cfg(test)]
mod tests {
    use crate::Cpu;
    use crate::bus::cpubus::Cpubus;
    
    #[test]
    /// Tests loading an immediate value (0x45) into the accumulator.
    fn cpu_test1() {
        let mut cpu = Cpu::new();
        let mut bus = Cpubus::new(&mut cpu);
        cpu.linkbus(&mut bus);
        bus.cpu_write(0x8000, 0xA9); // LDA immediate instruction
        bus.cpu_write(0x8001, 0x45); // Value to load
        bus.clock();
        
        assert_eq!(cpu.get_accumulator(), 69, "{} != 69", cpu.get_accumulator());
    }

    #[test]
    /// Tests loading a value (0xFF) from memory into the accumulator using LDA (zero page addressing).
    fn cpu_test2() {
        let mut cpu = Cpu::new();
        let mut bus = Cpubus::new(&mut cpu);
        cpu.linkbus(&mut bus);
        bus.cpu_write(0x8000, 0xA5); // LDA zero page
        bus.cpu_write(0x8001, 0x45); // Address 0x45
        bus.cpu_write(0x0045, 0xFF); // Store 0xFF at 0x45
        bus.clock();
        
        assert_eq!(cpu.get_accumulator(), 255, "{} != 255", cpu.get_accumulator());
    }
    
    #[test]
    /// Tests indexed zero page addressing mode with the X register.
    fn cpu_test3() {
        let mut cpu = Cpu::new();
        let mut bus = Cpubus::new(&mut cpu);
        cpu.linkbus(&mut bus);
        bus.cpu_write(0x8000, 0xB5); // LDA zero page, X indexed
        bus.cpu_write(0x8001, 0x44); // Base address 0x44
        cpu.set_x(1); // Set X to 1
        bus.cpu_write(0x0045, 0xFF); // Store 0xFF at 0x45 (0x44 + X)
        bus.clock();
        
        assert_eq!(cpu.get_accumulator(), 255, "{} != 255", cpu.get_accumulator());
    }

    #[test]
    /// Tests whether the zero flag is correctly set when loading a zero value into the accumulator.
    fn cpu_zero_flag() {
        let mut cpu = Cpu::new();
        let mut bus = Cpubus::new(&mut cpu);
        cpu.linkbus(&mut bus);
        bus.cpu_write(0x8000, 0xB5); // LDA zero page, X indexed
        bus.cpu_write(0x8001, 0x44); // Base address 0x44
        cpu.set_x(1);
        bus.cpu_write(0x0045, 0x00); // Store 0 at 0x45
        bus.clock();
        
        assert_eq!(cpu.get_flag() & 0x02, 0x02, "Zero flag not enabled!");
    }

    #[test]
    /// Tests whether the negative flag is correctly set when loading a negative value (0xFF) into the accumulator.
    fn cpu_negative_flag() {
        let mut cpu = Cpu::new();
        let mut bus = Cpubus::new(&mut cpu);
        cpu.linkbus(&mut bus);
        bus.cpu_write(0x8000, 0xB5); // LDA zero page, X indexed
        bus.cpu_write(0x8001, 0x44); // Base address 0x44
        cpu.set_x(1);
        bus.cpu_write(0x0045, 0xFF); // Store 0xFF at 0x45
        bus.clock();
        
        assert_eq!(cpu.get_flag() & 0x80, 0x80, "Negative flag not enabled!");
    }
}