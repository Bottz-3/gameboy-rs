use crate::cpu::Cpu;
use crate::mmu::mmu::Mmu;

impl Cpu {
    pub fn gb_loop(&mut self) {
        self.handle_interrupts();
        let mut cycles = 4;
        if !self.halted {
            let opcode = self.fetch_u8();
            cycles = self.decode(opcode);
        }
        if self.mmu.ppu.step(cycles) {
            let if_ = self.mmu.read(0xFF0F);
            self.mmu.write(0xFF0F, if_ | 0x01);
        }
    }
}
