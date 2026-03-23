use crate::cpu::{Cpu, Register};

impl Cpu {
    pub fn shift_left_arith(&mut self, reg: Register) {
        let reg_val = self.registers.get(reg);
        let b7 = (reg_val >> 7) & 1;
        let res = reg_val << 1;

        // just need to flag z and c
        let mut f: u8 = 0;

        if res == 0 {
            f |= 1 << 7;
        }
        if b7 == 1 {
            f |= 1 << 4;
        }
        self.registers.set(reg, res);
        self.registers.set(Register::F, f);
    }
    pub fn shift_right_arith(&mut self, reg: Register) {
        let reg_val = self.registers.get(reg);
        // 0x80 is 10000000
        let b7 = reg_val & 0x80;
        let b0 = reg_val & 1;

        let res = (reg_val >> 1) | b7;

        // just need to flag z and c
        let mut f: u8 = 0;

        if res == 0 {
            f |= 1 << 7;
        }
        if b0 == 1 {
            f |= 1 << 4;
        }
        self.registers.set(reg, res);
        self.registers.set(Register::F, f);
    }
}
impl Cpu {
    pub fn shift_right_logical(&mut self, reg: Register) {
        let reg_val = self.registers.get(reg);
        let b0 = reg_val & 1;
        let res = reg_val >> 1;

        let mut f: u8 = 0;

        if res == 0 {
            f |= 1 << 7;
        }
        if b0 == 1 {
            f |= 1 << 4;
        }
        self.registers.set(reg, res);
        self.registers.set(Register::F, f);
    }
}
