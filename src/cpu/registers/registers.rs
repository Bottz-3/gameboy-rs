#[derive(Copy, Clone)]
pub enum Register {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
}

pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
}
#[derive(Copy, Clone)]
pub enum Register16 {
    BC,
    DE,
    HL,
    AF,
    SP,
}

impl Registers {
    pub fn get(&self, reg: Register) -> u8 {
        match reg {
            Register::A => self.a,
            Register::B => self.b,
            Register::C => self.c,
            Register::D => self.d,
            Register::E => self.e,
            Register::F => self.f,
            Register::H => self.h,
            Register::L => self.l,
        }
    }
    pub fn set(&mut self, reg: Register, val: u8) {
        match reg {
            Register::A => self.a = val,
            Register::B => self.b = val,
            Register::C => self.c = val,
            Register::D => self.d = val,
            Register::E => self.e = val,
            Register::F => {
                if val & 0x0F != 0 {
                    println!("F being set with dirty lower nibble: {:02X}", val);
                }
                self.f = val & 0xF0
            }
            Register::H => self.h = val,
            Register::L => self.l = val,
        }
    }
}
