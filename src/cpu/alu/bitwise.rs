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
