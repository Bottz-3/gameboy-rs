use crate::cpu::{Cpu, Register};

impl Cpu {
    pub fn gb_loop(&mut self) {
        let mut cycles = 4;

        if !self.halted {
            let opcode = self.fetch_u8();
            cycles = self.decode(opcode);
            //println!("op {:02X}, cycles {}", opcode, cycles)
        } else {
            if self.check_pending() != 0 {
                self.halted = false;
            }
        }

        if self.ime_delay > 0 {
            self.ime_delay -= 1;
            if self.ime_delay == 0 {
                self.ime = true;
            }
        }
        if self.mmu.timer.step(cycles) {
            let if_ = self.mmu.read(0xFF0F);
            self.mmu.write(0xFF0F, if_ | 0x04, self.pc);
            println!("Timer IF set, PC={:04X}", self.pc);
        }

        if self.mmu.ppu.step(cycles) {
            self.mmu.load_background();
            let if_ = self.mmu.read(0xFF0F);
            self.mmu.write(0xFF0F, if_ | 0x01, self.pc);
        }
        self.handle_interrupts();
    }
}
