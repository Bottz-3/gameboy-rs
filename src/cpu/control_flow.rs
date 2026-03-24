use crate::cpu::Cpu;

impl Cpu {
    pub fn jump(&mut self) {
        self.pc = self.fetch_u16();
    }
    pub fn jump_conditional(&mut self, cond: bool) {
        if cond {
            self.pc = self.fetch_u16();
        }
    }
    pub fn jump_relative(&mut self) {
        let e = self.mmu.read(self.pc) as i8;
        self.pc += 1;
        self.pc = self.pc.wrapping_add_signed(e as i16);
    }
    pub fn jump_relative_conditional(&mut self, cond: bool) {
        let e = self.mmu.read(self.pc) as i8;
        self.pc += 1;
        if cond {
            self.pc = self.pc.wrapping_add_signed(e as i16);
        }
    }
}
// Call and call cond
impl Cpu {
    pub fn call(&mut self) {
        let nn = self.fetch_u16();
        self.sp -= 1;

        let msb = (self.pc >> 8) as u8;
        let lsb = (self.pc & 0xFF) as u8;

        self.mmu.write(self.sp, msb);
        self.sp -= 1;
        self.mmu.write(self.sp, lsb);
        self.pc = nn;
    }

    pub fn call_conditional(&mut self, cond: bool) {
        let nn = self.fetch_u16();

        if cond {
            self.sp -= 1;
            let msb = (self.pc >> 8) as u8;
            let lsb = (self.pc & 0xFF) as u8;

            self.mmu.write(self.sp, msb);
            self.sp -= 1;
            self.mmu.write(self.sp, lsb);
            self.pc = nn;
        }
    }
}

// Return
impl Cpu {
    pub fn ret(&mut self) {
        let lsb = self.mmu.read(self.sp) as u16;
        self.sp += 1;
        let msb = self.mmu.read(self.sp) as u16;
        self.sp += 1;
        self.pc = (msb << 8) | lsb;
    }
    pub fn ret_conditional(&mut self, cond: bool) {
        if cond {
            let lsb = self.mmu.read(self.sp) as u16;
            self.sp += 1;
            let msb = self.mmu.read(self.sp) as u16;
            self.sp += 1;
            self.pc = (msb << 8) | lsb;
        }
    }
    pub fn ret_from_interrupt(&mut self) {
        self.ret();
        self.ime = true;
    }
}

// Restart
impl Cpu {
    pub fn restart(&mut self, addr: u16) {
        self.sp -= 1;
        let msb = (self.pc >> 8) as u8;
        let lsb = (self.pc & 0xFF) as u8;

        self.mmu.write(self.sp, msb);
        self.sp -= 1;
        self.mmu.write(self.sp, lsb);
        self.pc = addr;
    }
}
