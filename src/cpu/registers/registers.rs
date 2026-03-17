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
    pub(super) a: u8,
    pub(super) b: u8,
    pub(super) c: u8,
    pub(super) d: u8,
    pub(super) e: u8,
    pub(super) f: u8,
    pub(super) h: u8,
    pub(super) l: u8,
}
#[derive(Copy, Clone)]
pub enum Register16 {
    BC,
    DE,
    HL,
    AF,
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
            Register::F => self.f = val,
            Register::H => self.h = val,
            Register::L => self.l = val,
        }
    }
}
