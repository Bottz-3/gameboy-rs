use crate::cpu::{Cpu, Register};

impl Cpu {
    pub fn and_register(&mut self, reg: Register) {
        let val = self.registers.get(reg);
        self.and(val);
    }
    pub fn and(&mut self, val: u8) {
        let a = self.registers.get(Register::A);
        let res = a & val;
        self.registers.set(Register::A, res);

        let mut f: u8 = 0;

        if res == 0 {
            f |= 1 << 7;
        }
        // set n to 0
        f |= 1 << 5;
        // set c to 0
        self.registers.set(Register::F, f);
    }
    pub fn or_register(&mut self, reg: Register) {
        let val = self.registers.get(reg);
        self.or(val);
    }
    pub fn or(&mut self, val: u8) {
        let a = self.registers.get(Register::A);
        let res = a | val;
        self.registers.set(Register::A, res);

        let mut f: u8 = 0;

        if res == 0 {
            f |= 1 << 7;
        }
        // set n to 0
        // set h to 0
        // set c to 0
        self.registers.set(Register::F, f);
    }
    pub fn xor_register(&mut self, reg: Register) {
        let val = self.registers.get(reg);
        self.xor(val);
    }
    pub fn xor(&mut self, val: u8) {
        let a = self.registers.get(Register::A);
        let res = a ^ val;
        self.registers.set(Register::A, res);

        let mut f: u8 = 0;

        if res == 0 {
            f |= 1 << 7;
        }
        // set n to 0
        // set h to 0
        // set c to 0
        self.registers.set(Register::F, f);
    }
}
// Test, reset and set bit
impl Cpu {
    pub fn test_bit(&mut self, bit: u8, reg: Register) {
        let reg_val = self.registers.get(reg);
        let mut f = self.registers.get(Register::F);

        if (reg_val >> bit) & 1 == 0 {
            f |= 1 << 7;
        } else {
            f &= !(1 << 7);
        }
        f |= 1 << 5;
        f &= !(1 << 6);
        self.registers.set(Register::F, f);
    }
    pub fn reset_bit(&mut self, bit: u8, reg: Register) {
        let reg_val = self.registers.get(reg);

        let res = reg_val & !(1 << bit);

        self.registers.set(reg, res);
    }
    pub fn set_bit(&mut self, bit: u8, reg: Register) {
        let reg_val = self.registers.get(reg);
        let res = reg_val | (1 << bit);
        self.registers.set(reg, res);
    }
}
