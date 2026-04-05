use crate::cpu::Cpu;

impl Cpu {
    pub fn check_pending(&mut self) -> u8 {
        let ie = self.mmu.read(0xFFFF);
        let if_ = self.mmu.read(0xFF0F);

        ie & if_
    }
}

impl Cpu {
    pub fn handle_interrupts(&mut self) {
        let pending = self.check_pending();
        if pending == 0 || !self.ime {
            if pending != 0 {
                self.halted = false;
            }
            return;
        }
        self.halted = false;
        self.ime = false;

        // 0xFF0F = IF

        if pending & 0x01 != 0 {
            self.mmu.write(0xFF0F, self.mmu.read(0xFF0F) & !0x01);
            // vblank interrupt
            self.call_interrupt(0x40);
        } else if pending & 0x02 != 0 {
            self.mmu.write(0xFF0F, self.mmu.read(0xFF0F) & !0x02);
            // lcd interrupt
            self.call_interrupt(0x48);
        } else if pending & 0x04 != 0 {
            self.mmu.write(0xFF0F, self.mmu.read(0xFF0F) & !0x04);
            // timer
            self.call_interrupt(0x50);
        } else if pending & 0x08 != 0 {
            self.mmu.write(0xFF0F, self.mmu.read(0xFF0F) & !0x08);
            // serial
            self.call_interrupt(0x58);
        } else if pending & 0x10 != 0 {
            self.mmu.write(0xFF0F, self.mmu.read(0xFF0F) & !0x10);
            // joypad
            self.call_interrupt(0x60);
        }
    }

    pub fn call_interrupt(&mut self, addr: u16) {
        self.restart(addr)
    }
}
