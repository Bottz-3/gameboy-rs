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
// Increment/decrement
impl Cpu {
    pub fn increment(&mut self, reg: Register) {
        let a = self.registers.get(reg);
        let res = a.wrapping_add(1);

        self.registers.set(reg, res);

        // set flags
        let mut f: u8 = 0;
        if res == 0 {
            f |= 1 << 7;
        }
        // h flag
        if (a & 0xF) == 0xF {
            f |= 1 << 5;
        }
        // c flag
        f |= self.registers.get(Register::F) & (1 << 4);
        self.registers.set(Register::F, f);
    }
    pub fn decrement(&mut self, reg: Register) {
        let a = self.registers.get(reg);
        let res = a.wrapping_sub(1);

        self.registers.set(reg, res);

        // set flags
        let mut f: u8 = 0;
        if res == 0 {
            f |= 1 << 7;
        }
        // n flag (always has to be 1)
        f |= 1 << 6;
        // h flag
        if (a & 0xF) == 0x0 {
            f |= 1 << 5;
        }
        // c flag
        f |= self.registers.get(Register::F) & (1 << 4);
        self.registers.set(Register::F, f);
    }
}
