use super::registers::Registers;

// These are the 4 general purpose 16-bit registers (from high to low):
// BC, DE, HL, AF

// Getters:
// This shifts the bits left as a u16 so you get 8 zeros on the right
// Bitwise OR concats the low register to it
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
// Reverse of getter: put upper into high, put lower into low
// Masking with 0xFF is 1111111 and this goes into low.
impl Registers {
    pub fn set_bc(&mut self, val: u16) {
        self.b = (val >> 8) as u8;
        self.c = (val & 0xFF) as u8;
    }
    pub fn set_de(&mut self, val: u16) {
        self.d = (val >> 8) as u8;
        self.e = (val & 0xFF) as u8;
    }
    pub fn set_hl(&mut self, val: u16) {
        self.h = (val >> 8) as u8;
        self.l = (val & 0xFF) as u8;
    }
    pub fn set_af(&mut self, val: u16) {
        self.a = (val >> 8) as u8;
        self.f = (val & 0xFF) as u8;
    }
}
