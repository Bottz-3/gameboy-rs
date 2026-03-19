use crate::cpu::{Cpu, Register};

impl Cpu {
    pub fn rotate_left_circular(&mut self) {
        let a = self.registers.get(Register::A);
        let b7 = (a >> 7) & 1;

        let res = a.rotate_left(1);
        self.registers.set(Register::A, res);

        let mut f: u8 = 0;
        // just need to do c flag
        if b7 == 1 {
            f |= 1 << 4;
        }
        self.registers.set(Register::F, f);
    }
    pub fn rotate_right_circular(&mut self) {
        let a = self.registers.get(Register::A);
        let b7 = a & 1;

        let res = a.rotate_right(1);
        self.registers.set(Register::A, res);

        let mut f: u8 = 0;
        // just need to do c flag
        if b7 == 1 {
            f |= 1 << 4;
        }
        self.registers.set(Register::F, f);
    }
    pub fn rotate_left_accum(&mut self) {
        let a = self.registers.get(Register::A);
        let f = self.registers.get(Register::F);
        let b7 = (a >> 7) & 1;

        let res = (a << 1) | ((f >> 4) & 1);
        self.registers.set(Register::A, res);

        let mut f: u8 = 0;
        // just need to do c flag
        if b7 == 1 {
            f |= 1 << 4;
        }
        self.registers.set(Register::F, f);
    }
    pub fn rotate_right_accum(&mut self) {
        let a = self.registers.get(Register::A);
        let f = self.registers.get(Register::F);
        let b0 = (a >> 7) & 1;

        let res = (a >> 1) | (((f >> 4) & 1) << 7);
        self.registers.set(Register::A, res);

        let mut f: u8 = 0;
        // just need to do c flag
        if b0 == 1 {
            f |= 1 << 4;
        }
        self.registers.set(Register::F, f);
    }
}
// Rotations for register
impl Cpu {
    pub fn rotate_left_register(&mut self, reg: Register) {
        let reg_val = self.registers.get(reg);
        let b7 = (reg_val >> 7) & 1;

        let res = reg_val.rotate_left(1);
        self.registers.set(reg, res);

        let mut f: u8 = 0;

        // just doing z and c flags.
        if res == 0 {
            f |= 1 << 7;
        }

        if b7 == 1 {
            f |= 1 << 4;
        }
        self.registers.set(Register::F, f);
    }
    pub fn rotate_right_register(&mut self, reg: Register) {
        let reg_val = self.registers.get(reg);
        let b7 = reg_val & 1;

        let res = reg_val.rotate_right(1);
        self.registers.set(reg, res);

        let mut f: u8 = 0;

        // just doing z and c flags.
        if res == 0 {
            f |= 1 << 7;
        }

        if b7 == 1 {
            f |= 1 << 4;
        }
        self.registers.set(Register::F, f);
    }
}
// just rotate left and rotate right
impl Cpu {
    pub fn rotate_left(&mut self, reg: Register) {
        let reg_val = self.registers.get(reg);
        let f = self.registers.get(Register::F);
        let b7 = (reg_val >> 7) & 1;

        let res = (reg_val << 1) | ((f >> 4) & 1);
        self.registers.set(reg, res);

        let mut f: u8 = 0;

        // just doing z and c flags.
        if res == 0 {
            f |= 1 << 7;
        }

        if b7 == 1 {
            f |= 1 << 4;
        }
        self.registers.set(Register::F, f);
    }
    pub fn rotate_right(&mut self, reg: Register) {
        let reg_val = self.registers.get(reg);
        let f = self.registers.get(Register::F);
        let b0 = reg_val & 1;

        let res = (reg_val >> 1) | (((f >> 4) & 1) << 7);
        self.registers.set(reg, res);

        let mut f: u8 = 0;

        // just doing z and c flags.
        if res == 0 {
            f |= 1 << 7;
        }

        if b0 == 1 {
            f |= 1 << 4;
        }
        self.registers.set(Register::F, f);
    }
}
