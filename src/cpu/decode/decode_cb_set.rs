use crate::cpu::{Cpu, Register};

impl Cpu {
    pub fn decode_cb_set(&mut self, opcode: u8) -> u32 {
        match opcode {
            0xC0 => {
                self.set_bit(0, Register::B);
                8
            }
            0xD0 => {
                self.set_bit(2, Register::B);
                8
            }
            0xE0 => {
                self.set_bit(4, Register::B);
                8
            }
            0xF0 => {
                self.set_bit(6, Register::B);
                8
            }
            0xC1 => {
                self.set_bit(0, Register::C);
                8
            }
            0xD1 => {
                self.set_bit(2, Register::C);
                8
            }
            0xE1 => {
                self.set_bit(4, Register::C);
                8
            }
            0xF1 => {
                self.set_bit(6, Register::C);
                8
            }
            0xC2 => {
                self.set_bit(0, Register::D);
                8
            }
            0xD2 => {
                self.set_bit(2, Register::D);
                8
            }
            0xE2 => {
                self.set_bit(4, Register::D);
                8
            }
            0xF2 => {
                self.set_bit(6, Register::D);
                8
            }
            0xC3 => {
                self.set_bit(0, Register::E);
                8
            }
            0xD3 => {
                self.set_bit(2, Register::E);
                8
            }
            0xE3 => {
                self.set_bit(4, Register::E);
                8
            }
            0xF3 => {
                self.set_bit(6, Register::E);
                8
            }
            0xC4 => {
                self.set_bit(0, Register::H);
                8
            }
            0xD4 => {
                self.set_bit(2, Register::H);
                8
            }
            0xE4 => {
                self.set_bit(4, Register::H);
                8
            }
            0xF4 => {
                self.set_bit(6, Register::H);
                8
            }
            0xC5 => {
                self.set_bit(0, Register::L);
                8
            }
            0xD5 => {
                self.set_bit(2, Register::L);
                8
            }
            0xE5 => {
                self.set_bit(4, Register::L);
                8
            }
            0xF5 => {
                self.set_bit(6, Register::L);
                8
            }
            0xC6 => {
                let addr = self.registers.get_hl();
                let data = self.mmu.read(addr);
                self.set_bit_data(0, data);
                16
            }
            0xD6 => {
                let addr = self.registers.get_hl();
                let data = self.mmu.read(addr);
                self.set_bit_data(2, data);
                16
            }
            0xE6 => {
                let addr = self.registers.get_hl();
                let data = self.mmu.read(addr);
                self.set_bit_data(4, data);
                16
            }
            0xF6 => {
                let addr = self.registers.get_hl();
                let data = self.mmu.read(addr);
                self.set_bit_data(6, data);
                16
            }
            0xC7 => {
                self.set_bit(0, Register::A);
                8
            }
            0xD7 => {
                self.set_bit(2, Register::A);
                8
            }
            0xE7 => {
                self.set_bit(4, Register::A);
                8
            }
            0xF7 => {
                self.set_bit(6, Register::A);
                8
            }
            0xC8 => {
                self.set_bit(1, Register::B);
                8
            }
            0xD8 => {
                self.set_bit(3, Register::B);
                8
            }
            0xE8 => {
                self.set_bit(5, Register::B);
                8
            }
            0xF8 => {
                self.set_bit(7, Register::B);
                8
            }
            0xC9 => {
                self.set_bit(1, Register::C);
                8
            }
            0xD9 => {
                self.set_bit(3, Register::C);
                8
            }
            0xE9 => {
                self.set_bit(5, Register::C);
                8
            }
            0xF9 => {
                self.set_bit(7, Register::C);
                8
            }
            0xCA => {
                self.set_bit(1, Register::D);
                8
            }
            0xDA => {
                self.set_bit(3, Register::D);
                8
            }
            0xEA => {
                self.set_bit(5, Register::D);
                8
            }
            0xFA => {
                self.set_bit(7, Register::D);
                8
            }
            0xCB => {
                self.set_bit(1, Register::E);
                8
            }
            0xDB => {
                self.set_bit(3, Register::E);
                8
            }
            0xEB => {
                self.set_bit(5, Register::E);
                8
            }
            0xFB => {
                self.set_bit(7, Register::E);
                8
            }
            0xCC => {
                self.set_bit(1, Register::H);
                8
            }
            0xDC => {
                self.set_bit(3, Register::H);
                8
            }
            0xEC => {
                self.set_bit(5, Register::H);
                8
            }
            0xFC => {
                self.set_bit(7, Register::H);
                8
            }
            0xCD => {
                self.set_bit(1, Register::L);
                8
            }
            0xDD => {
                self.set_bit(3, Register::L);
                8
            }
            0xED => {
                self.set_bit(5, Register::L);
                8
            }
            0xFD => {
                self.set_bit(7, Register::L);
                8
            }
            0xCE => {
                let addr = self.registers.get_hl();
                let data = self.mmu.read(addr);
                self.set_bit_data(1, data);
                16
            }
            0xDE => {
                let addr = self.registers.get_hl();
                let data = self.mmu.read(addr);
                self.set_bit_data(3, data);
                16
            }
            0xEE => {
                let addr = self.registers.get_hl();
                let data = self.mmu.read(addr);
                self.set_bit_data(5, data);
                16
            }
            0xFE => {
                let addr = self.registers.get_hl();
                let data = self.mmu.read(addr);
                self.set_bit_data(7, data);
                16
            }
            0xCF => {
                self.set_bit(1, Register::A);
                8
            }
            0xDF => {
                self.set_bit(3, Register::A);
                8
            }
            0xEF => {
                self.set_bit(5, Register::A);
                8
            }
            0xFF => {
                self.set_bit(7, Register::A);
                8
            }
            _ => panic!("CATASTROPHIC ERROR! OPCODE NOT IN THIS RANGE SHOULD NOT RUN HERE AT ALL!"),
        }
    }
}
