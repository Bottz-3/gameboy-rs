use super::registers::{Register, Registers};

// These are the 4 general purpose 16-bit registers (from high to low):
// BC, DE, HL, AF

// Getters
impl Registers {
    pub fn get_bc(&self) -> u16 {
        ((self.b as u16) << 8) | self.c as u16
    }
    pub fn get_de(&self) -> u16 {
        ((self.d) as u16) << 8 | self.e as u16
    }
    pub fn get_hl(&self) -> u16 {
        ((self.h) as u16) << 8 | self.l as u16
    }
    pub fn get_af(&self) -> u16 {
        ((self.a) as u16) << 8 | self.f as u16
    }
}

// Setters
impl Registers {}
