use crate::cpu::registers::Register16;
use crate::cpu::{Cpu, Register};

impl Cpu {
    pub fn push(&mut self, reg: Register16) {
        let val = self.get16(reg);
        if val >= 0xFF00 {
            //println!("Suspicious push: {:04X} at PC {:04X}", val, self.pc);
        }
        self.sp = self.sp.wrapping_sub(1);
        self.mmu.write(self.sp, (val >> 8) as u8);
        self.sp = self.sp.wrapping_sub(1);
        self.mmu.write(self.sp, (val & 0xFF) as u8);
    }
    pub fn pop(&mut self, reg: Register16) {
        let lsb = self.mmu.read(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        let msb = self.mmu.read(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        let val = (msb << 8) | lsb;

        match reg {
            Register16::AF => {
                let a = (val >> 8) as u8;
                let f = (val as u8) & 0xF0;

                self.registers.load_register_data(Register::A, a);
                self.registers.load_register_data(Register::F, f);
            }
            _ => {
                self.set16(reg, val);
            }
        }
    }
    pub fn push_u16(&mut self, val: u16) {
        let hi = (val >> 8) as u8;
        let lo = (val & 0xFF) as u8;

        self.sp = self.sp.wrapping_sub(1);
        self.mmu.write(self.sp, hi);

        self.sp = self.sp.wrapping_sub(1);
        self.mmu.write(self.sp, lo);
    }

    pub fn pop_u16(&mut self) -> u16 {
        let lo = self.mmu.read(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        let hi = self.mmu.read(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        (hi << 8) | lo
    }
}
