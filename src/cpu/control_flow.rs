use crate::cpu::Cpu;

impl Cpu {
    pub fn jump(&mut self) {
        let nn = self.fetch_u16();
        self.pc = nn;
    }
    pub fn jump_conditional(&mut self, cond: bool) -> u32 {
        let nn = self.fetch_u16();
        if cond {
            self.pc = nn;
            return 16;
        }
        12
    }
    pub fn jump_relative(&mut self) {
        let e = self.fetch_u8() as i8;
        self.pc = self.pc.wrapping_add_signed(e as i16);
    }
    pub fn jump_relative_conditional(&mut self, cond: bool) -> u32 {
        let e = self.mmu.read(self.pc) as i8;
        self.pc = self.pc.wrapping_add(1);
        if cond {
            self.pc = self.pc.wrapping_add_signed(e as i16);
            return 12;
        }
        8
    }
}
// Call and call cond
impl Cpu {
    pub fn call(&mut self) {
        let nn = self.fetch_u16();
        let ret_addr = self.pc;
        self.push_u16(ret_addr);
        self.pc = nn;
    }

    pub fn call_conditional(&mut self, cond: bool) -> u32 {
        let nn = self.fetch_u16();

        if cond {
            self.push_u16(self.pc);
            self.pc = nn;
            return 24;
        }
        12
    }
}

// Return
impl Cpu {
    pub fn ret(&mut self) {
        let popped = self.pop_u16();

        self.pc = popped;
    }
    pub fn ret_conditional(&mut self, cond: bool) -> u32 {
        if cond {
            self.pc = self.pop_u16();
            return 20;
        }
        8
    }
    pub fn ret_from_interrupt(&mut self) {
        self.ret();
        self.ime = true;
    }
}

// Restart
impl Cpu {
    pub fn restart(&mut self, addr: u16) {
        self.push_u16(self.pc);
        self.pc = addr;
    }
}
