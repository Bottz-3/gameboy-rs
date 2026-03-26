use super::registers::registers::Registers;
use crate::{cpu::registers::Register16, mmu::mmu::Mmu};

pub struct Cpu {
    pub registers: Registers,
    pub mmu: Mmu,
    pub pc: u16,
    pub sp: u16,
    pub ime: bool,
    pub halted: bool,
}

impl Cpu {
    pub fn fetch_u8(&mut self) -> u8 {
        let data = self.mmu.read(self.pc);
        self.pc += 1;
        data
    }
    pub fn fetch_u16(&mut self) -> u16 {
        let lsb = self.mmu.read(self.pc) as u16;
        self.pc += 1;
        let msb = self.mmu.read(self.pc) as u16;
        self.pc += 1;
        (msb << 8) | lsb
    }
    pub fn get16(&mut self, reg: Register16) -> u16 {
        match reg {
            Register16::AF => self.registers.get_af(),
            Register16::BC => self.registers.get_bc(),
            Register16::DE => self.registers.get_de(),
            Register16::HL => self.registers.get_hl(),
            Register16::SP => self.sp,
        }
    }
    pub fn set16(&mut self, reg: Register16, val: u16) {
        match reg {
            Register16::AF => self.registers.set_af(val),
            Register16::BC => self.registers.set_bc(val),
            Register16::DE => self.registers.set_de(val),
            Register16::HL => self.registers.set_hl(val),
            Register16::SP => self.sp = val,
        }
    }
}
