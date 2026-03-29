use crate::cpu::Cpu;
use crate::cpu::registers::Register16;

impl Cpu {
    pub fn push(&mut self, reg: Register16) {
        let val = self.get16(reg);
        self.sp -= 1;
        self.mmu.write(self.sp, (val >> 8) as u8);
        self.sp -= 1;
        self.mmu.write(self.sp, (val & 0xFF) as u8);
    }
    pub fn pop(&mut self, reg: Register16) {
        let lsb = self.mmu.read(self.sp) as u16;
        self.sp += 1;
        let msb = self.mmu.read(self.sp) as u16;
        self.sp += 1;
        self.set16(reg, (msb << 8) | lsb);
    }
}
