use crate::cpu::Cpu;

impl Cpu {
    pub fn check_pending(&self) -> u8 {
        let ie = self.mmu.read(0xFFFF);
        let if_ = self.mmu.read(0xFF0F);

        ie & if_
    }
}

impl Cpu {
    pub fn handle_interrupts(&mut self) {
        let pending = self.check_pending();
        if pending == 0 {
            return;
        }

        if self.halted {
            self.halted = false;
        }

        if !self.ime {
            return;
        }

        self.halted = false;
        self.ime = false;

        let if_ = self.mmu.read(0xFF0F);

        if pending & 0x01 != 0 {
            self.mmu.write(0xFF0F, if_ & !0x01, self.pc);
            self.call_interrupt(0x40);
        } else if pending & 0x02 != 0 {
            self.mmu.write(0xFF0F, if_ & !0x02, self.pc);
            self.call_interrupt(0x48);
        } else if pending & 0x04 != 0 {
            self.mmu.write(0xFF0F, if_ & !0x04, self.pc);
            self.call_interrupt(0x50);
        } else if pending & 0x08 != 0 {
            self.mmu.write(0xFF0F, if_ & !0x08, self.pc);
            self.call_interrupt(0x58);
        } else if pending & 0x10 != 0 {
            self.mmu.write(0xFF0F, if_ & !0x10, self.pc);
            self.call_interrupt(0x60);
        }
    }

    pub fn call_interrupt(&mut self, addr: u16) {
        println!("call_interrupt: {:04X} PC: {:04X}", addr, self.pc);

        self.restart(addr)
    }
}
