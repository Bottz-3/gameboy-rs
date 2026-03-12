use super::registers::registers::Registers;
use crate::mmu::mmu::Mmu;

pub struct Cpu {
    pub registers: Registers,
    mmu: Mmu,
    pc: u16,
    sp: u16,
}

impl Cpu {
    pub fn fetch_u16(&mut self) -> u16 {
        let lsb = self.mmu.read(self.pc) as u16;
        self.pc += 1;
        let msb = self.mmu.read(self.pc) as u16;
        self.pc += 1;
        (msb << 8) | lsb
    }
}
