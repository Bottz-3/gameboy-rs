use crate::cpu::{Cpu, Register};

impl Cpu {
    pub fn compare_register(&mut self, reg: Register) {
        let val = self.registers.get(reg);
        self.compare_data(val);
    }
    pub fn compare_data(&mut self, val: u8) {
        let a = self.registers.get(Register::A);
        let res = a.wrapping_sub(val);

        // set flags
        let mut f: u8 = 0;
        if res == 0 {
            f |= 1 << 7;
        }
        // n flag
        f |= 1 << 6;
        // h flag
        if (a & 0xF) < (val & 0xF) {
            f |= 1 << 5;
        }
        // c flag
        if a < val {
            f |= 1 << 4;
        }
        self.registers.set(Register::F, f);
    }
}
// Swap
impl Cpu {
    pub fn swap_nibbles(&mut self, reg: Register) {
        let reg_val = self.registers.get(reg);
        let res = (reg_val << 4) | (reg_val >> 4);

        let mut f: u8 = 0;
        if res == 0 {
            f |= 1 << 7;
        }
        self.registers.set(Register::F, f);
        self.registers.set(reg, res);
    }
}
// Shift right logical
impl Cpu {
    pub fn shift_right_logical(&mut self, reg: Register) {
        let reg_val = self.registers.get(reg);
        let b0 = reg_val & 1;
        let b7 = (reg_val << 7) & 1;
        let res = reg_val >> 1;

        let mut f: u8 = 0;
    }
}
