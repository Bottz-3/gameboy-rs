use crate::cpu::Cpu;

impl Cpu {
    pub fn gb_loop(&mut self) {
        self.handle_interrupts();
        if !self.halted {
            let opcode = self.fetch_u8();
            self.decode(opcode);
        }
    }
}
