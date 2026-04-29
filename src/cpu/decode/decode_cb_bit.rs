use std::f64::consts::LN_10;

use crate::cpu::{Cpu, Register};

impl Cpu {
    pub fn decode_cb_bit(&mut self, opcode: u8) -> u32 {
        match opcode {
            0x40 => {
                self.test_bit(0, Register::B);
                8
            }
            0x50 => {
                self.test_bit(2, Register::B);
                8
            }
            0x60 => {
                self.test_bit(4, Register::B);
                8
            }
            0x70 => {
                self.test_bit(6, Register::B);
                8
            }
            0x41 => {
                self.test_bit(0, Register::C);
                8
            }
            0x51 => {
                self.test_bit(2, Register::C);
                8
            }
            0x61 => {
                self.test_bit(4, Register::C);
                8
            }
            0x71 => {
                self.test_bit(6, Register::C);
                8
            }
            0x42 => {
                self.test_bit(0, Register::D);
                8
            }
            0x52 => {
                self.test_bit(2, Register::D);
                8
            }
            0x62 => {
                self.test_bit(4, Register::D);
                8
            }
            0x72 => {
                self.test_bit(6, Register::D);
                8
            }
            0x43 => {
                self.test_bit(0, Register::E);
                8
            }
            0x53 => {
                self.test_bit(2, Register::E);
                8
            }
            0x63 => {
                self.test_bit(4, Register::E);
                8
            }
            0x73 => {
                self.test_bit(6, Register::E);
                8
            }
            0x44 => {
                self.test_bit(0, Register::H);
                8
            }
            0x54 => {
                self.test_bit(2, Register::H);
                8
            }
            0x64 => {
                self.test_bit(4, Register::H);
                8
            }
            0x74 => {
                self.test_bit(6, Register::H);
                8
            }
            0x45 => {
                self.test_bit(0, Register::L);
                8
            }
            0x55 => {
                self.test_bit(2, Register::L);
                8
            }
            0x65 => {
                self.test_bit(4, Register::L);
                8
            }
            0x75 => {
                self.test_bit(6, Register::L);
                8
            }
            0x46 => {
                let addr = self.registers.get_hl();
                let data = self.mmu.read(addr);
                self.test_bit_data(0, data);
                12
            }
            0x56 => {
                let addr = self.registers.get_hl();
                let data = self.mmu.read(addr);
                self.test_bit_data(2, data);
                12
            }
            0x66 => {
                let addr = self.registers.get_hl();
                let data = self.mmu.read(addr);
                self.test_bit_data(4, data);
                12
            }
            0x76 => {
                let addr = self.registers.get_hl();
                let data = self.mmu.read(addr);
                self.test_bit_data(6, data);
                12
            }
            0x47 => {
                self.test_bit(0, Register::A);
                8
            }
            0x57 => {
                self.test_bit(2, Register::A);
                8
            }
            0x67 => {
                self.test_bit(4, Register::A);
                8
            }
            0x77 => {
                self.test_bit(6, Register::A);
                8
            }
            0x48 => {
                self.test_bit(1, Register::B);
                8
            }
            0x58 => {
                self.test_bit(3, Register::B);
                8
            }
            0x68 => {
                self.test_bit(5, Register::B);
                8
            }
            0x78 => {
                self.test_bit(7, Register::B);
                8
            }
            0x49 => {
                self.test_bit(1, Register::C);
                8
            }
            0x59 => {
                self.test_bit(3, Register::C);
                8
            }
            0x69 => {
                self.test_bit(5, Register::C);
                8
            }
            0x79 => {
                self.test_bit(7, Register::C);
                8
            }
            0x4A => {
                self.test_bit(1, Register::D);
                8
            }
            0x5A => {
                self.test_bit(3, Register::D);
                8
            }
            0x6A => {
                self.test_bit(5, Register::D);
                8
            }
            0x7A => {
                self.test_bit(7, Register::D);
                8
            }
            0x4B => {
                self.test_bit(1, Register::E);
                8
            }
            0x5B => {
                self.test_bit(3, Register::E);
                8
            }
            0x6B => {
                self.test_bit(5, Register::E);
                8
            }
            0x7B => {
                self.test_bit(7, Register::E);
                8
            }
            0x4C => {
                self.test_bit(1, Register::H);
                8
            }
            0x5C => {
                self.test_bit(3, Register::H);
                8
            }
            0x6C => {
                self.test_bit(5, Register::H);
                8
            }
            0x7C => {
                self.test_bit(7, Register::H);
                8
            }
            0x4D => {
                self.test_bit(1, Register::L);
                8
            }
            0x5D => {
                self.test_bit(3, Register::L);
                8
            }
            0x6D => {
                self.test_bit(5, Register::L);
                8
            }
            0x7D => {
                self.test_bit(7, Register::L);
                8
            }
            0x4E => {
                let addr = self.registers.get_hl();
                let data = self.mmu.read(addr);
                self.test_bit_data(1, data);
                12
            }
            0x5E => {
                let addr = self.registers.get_hl();
                let data = self.mmu.read(addr);
                self.test_bit_data(3, data);
                12
            }
            0x6E => {
                let addr = self.registers.get_hl();
                let data = self.mmu.read(addr);
                self.test_bit_data(5, data);
                12
            }
            0x7E => {
                let addr = self.registers.get_hl();
                let data = self.mmu.read(addr);
                self.test_bit_data(7, data);
                12
            }
            0x4F => {
                self.test_bit(1, Register::A);
                8
            }
            0x5F => {
                self.test_bit(3, Register::A);
                8
            }
            0x6F => {
                self.test_bit(5, Register::A);
                8
            }
            0x7F => {
                self.test_bit(7, Register::A);
                8
            }
            _ => panic!("CATASTROPHIC ERROR! OPCODE NOT IN THIS RANGE SHOULD NOT RUN HERE AT ALL!"),
        }
    }
}
