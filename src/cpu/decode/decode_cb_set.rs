use crate::cpu::{Cpu, Register};

impl Cpu {
    pub fn decode_cb_set(&mut self, opcode: u8) {
        match opcode {
            0xC0 => self.set_bit(0, Register::B),
            0xD0 => self.set_bit(2, Register::B),
            0xE0 => self.set_bit(4, Register::B),
            0xF0 => self.set_bit(6, Register::B),
            0xC1 => self.set_bit(0, Register::C),
            0xD1 => self.set_bit(2, Register::C),
            0xE1 => self.set_bit(4, Register::C),
            0xF1 => self.set_bit(6, Register::C),
            0xC2 => self.set_bit(0, Register::D),
            0xD2 => self.set_bit(2, Register::D),
            0xE2 => self.set_bit(4, Register::D),
            0xF2 => self.set_bit(6, Register::D),
            0xC3 => self.set_bit(0, Register::E),
            0xD3 => self.set_bit(2, Register::E),
            0xE3 => self.set_bit(4, Register::E),
            0xF3 => self.set_bit(6, Register::E),
            0xC4 => self.set_bit(0, Register::H),
            0xD4 => self.set_bit(2, Register::H),
            0xE4 => self.set_bit(4, Register::H),
            0xF4 => self.set_bit(6, Register::H),
            0xC5 => self.set_bit(0, Register::L),
            0xD5 => self.set_bit(2, Register::L),
            0xE5 => self.set_bit(4, Register::L),
            0xF5 => self.set_bit(6, Register::L),
            0xC6 => {
                let addr = self.registers.get_hl();
                let data = self.mmu.read(addr);
                self.set_bit_data(0, data);
            }
            0xD6 => {
                let addr = self.registers.get_hl();
                let data = self.mmu.read(addr);
                self.set_bit_data(2, data);
            }
            0xE6 => {
                let addr = self.registers.get_hl();
                let data = self.mmu.read(addr);
                self.set_bit_data(4, data);
            }
            0xF6 => {
                let addr = self.registers.get_hl();
                let data = self.mmu.read(addr);
                self.set_bit_data(6, data);
            }
            0xC7 => self.set_bit(0, Register::A),
            0xD7 => self.set_bit(2, Register::A),
            0xE7 => self.set_bit(4, Register::A),
            0xF7 => self.set_bit(6, Register::A),
            0xC8 => self.set_bit(1, Register::B),
            0xD8 => self.set_bit(3, Register::B),
            0xE8 => self.set_bit(5, Register::B),
            0xF8 => self.set_bit(7, Register::B),
            0xC9 => self.set_bit(1, Register::C),
            0xD9 => self.set_bit(3, Register::C),
            0xE9 => self.set_bit(5, Register::C),
            0xF9 => self.set_bit(7, Register::C),
            0xCA => self.set_bit(1, Register::D),
            0xDA => self.set_bit(3, Register::D),
            0xEA => self.set_bit(5, Register::D),
            0xFA => self.set_bit(7, Register::D),
            0xCB => self.set_bit(1, Register::E),
            0xDB => self.set_bit(3, Register::E),
            0xEB => self.set_bit(5, Register::E),
            0xFB => self.set_bit(7, Register::E),
            0xCC => self.set_bit(1, Register::H),
            0xDC => self.set_bit(3, Register::H),
            0xEC => self.set_bit(5, Register::H),
            0xFC => self.set_bit(7, Register::H),
            0xCD => self.set_bit(1, Register::L),
            0xDD => self.set_bit(3, Register::L),
            0xED => self.set_bit(5, Register::L),
            0xFD => self.set_bit(7, Register::L),
            0xCE => {
                let addr = self.registers.get_hl();
                let data = self.mmu.read(addr);
                self.set_bit_data(1, data);
            }
            0xDE => {
                let addr = self.registers.get_hl();
                let data = self.mmu.read(addr);
                self.set_bit_data(3, data);
            }
            0xEE => {
                let addr = self.registers.get_hl();
                let data = self.mmu.read(addr);
                self.set_bit_data(5, data);
            }
            0xFE => {
                let addr = self.registers.get_hl();
                let data = self.mmu.read(addr);
                self.set_bit_data(7, data);
            }
            0xCF => self.set_bit(1, Register::A),
            0xDF => self.set_bit(3, Register::A),
            0xEF => self.set_bit(5, Register::A),
            0xFF => self.set_bit(7, Register::A),
            _ => panic!("CATASTROPHIC ERROR! OPCODE NOT IN THIS RANGE SHOULD NOT RUN HERE AT ALL!"),
        }
    }
}
