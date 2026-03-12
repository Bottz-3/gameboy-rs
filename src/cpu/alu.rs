use super::cpu::Cpu;
use super::registers::registers::{Register, Registers};

// Addition
impl Cpu {
    pub fn add_register(&mut self, reg: Register) {
        let val = self.registers.get(reg);
        self.add_data(val);
    }
    pub fn add_data(&mut self, val: u8) {
        let a = self.registers.get(Register::A);
        let res = a.wrapping_add(val);

        self.registers.set(Register::A, res);

        // set flags
        let mut f: u8 = 0;
        if res == 0 {
            f |= 1 << 7;
        }
        // h flag
        if (a & 0xF) + (val & 0xF) > 0xF {
            f |= 1 << 5;
        }
        // c flag
        if (a as u16) + (val as u16) > 0xFF {
            f |= 1 << 4;
        }
        self.registers.set(Register::F, f);
    }
    pub fn add_carry_register(&mut self, reg: Register) {
        let val = self.registers.get(reg);
        self.add_carry(val);
    }
    pub fn add_carry(&mut self, val: u8) {
        let a = self.registers.get(Register::A);
        let c = (self.registers.get(Register::F) >> 4) & 1;
        let res = a.wrapping_add(val).wrapping_add(c);

        self.registers.set(Register::A, res);

        // set flags
        let mut f: u8 = 0;
        if res == 0 {
            f |= 1 << 7;
        }
        // h flag
        if (a & 0xF) + (val & 0xF) + c > 0xF {
            f |= 1 << 5;
        }
        // c flag
        if (a as u16) + (val as u16) + (c as u16) > 0xFF {
            f |= 1 << 4;
        }
        self.registers.set(Register::F, f);
    }
}

// Subtraction
impl Cpu {
    pub fn subtract_register(&mut self, reg: Register) {
        let val = self.registers.get(reg);
        self.subtract_data(val);
    }
    pub fn subtract_data(&mut self, val: u8) {
        let a = self.registers.get(Register::A);
        let res = a.wrapping_sub(val);

        self.registers.set(Register::A, res);

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
    pub fn subtract_carry_register(&mut self, reg: Register) {
        let val = self.registers.get(reg);
        self.subtract_carry(val);
    }
    pub fn subtract_carry(&mut self, val: u8) {
        let a = self.registers.get(Register::A);
        let c = (self.registers.get(Register::F) >> 4) & 1;
        let res = a.wrapping_sub(val).wrapping_sub(c);

        self.registers.set(Register::A, res);

        // set flags
        let mut f: u8 = 0;
        if res == 0 {
            f |= 1 << 7;
        }
        // n flag
        f |= 1 << 6;
        // h flag
        if (a & 0xF) < (val & 0xF) + c {
            f |= 1 << 5;
        }
        // c flag
        if (a as u16) < (val as u16) + c as u16 {
            f |= 1 << 4;
        }
        self.registers.set(Register::F, f);
    }
}
