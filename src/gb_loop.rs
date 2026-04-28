use crate::cpu::Cpu;
use crate::mmu::mmu::Mmu;

impl Cpu {
    pub fn gb_loop(&mut self) {
        self.handle_interrupts();
        if !self.halted {
            let opcode = self.fetch_u8();
            self.decode(opcode);
        }
        let cycles = 4;
        if self.mmu.ppu.step(cycles) {
            let if_ = self.mmu.read(0xFF0F);
            self.mmu.write(0xFF0F, if_ | 0x01);
        }
    }
}
