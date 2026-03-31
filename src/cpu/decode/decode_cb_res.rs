use crate::cpu::{Cpu, Register};

impl Cpu {
    pub fn decode_cb_res(&mut self, opcode: u8) {
        match opcode {
            0x80 => self.reset_bit(0, Register::B),
            0x90 => self.reset_bit(2, Register::B),
            0xA0 => self.reset_bit(4, Register::B),
            0xB0 => self.reset_bit(6, Register::B),
            0x81 => self.reset_bit(0, Register::C),
            0x91 => self.reset_bit(2, Register::C),
            0xA1 => self.reset_bit(4, Register::C),
            0xB1 => self.reset_bit(6, Register::C),
            0x82 => self.reset_bit(0, Register::D),
            0x92 => self.reset_bit(2, Register::D),
            0xA2 => self.reset_bit(4, Register::D),
            0xB2 => self.reset_bit(6, Register::D),
            0x83 => self.reset_bit(0, Register::E),
            0x93 => self.reset_bit(2, Register::E),
            0xA3 => self.reset_bit(4, Register::E),
            0xB3 => self.reset_bit(6, Register::E),
            0x84 => self.reset_bit(0, Register::H),
            0x94 => self.reset_bit(2, Register::H),
            0xA4 => self.reset_bit(4, Register::H),
            0xB4 => self.reset_bit(6, Register::H),
            0x85 => self.reset_bit(0, Register::L),
            0x95 => self.reset_bit(2, Register::L),
            0xA5 => self.reset_bit(4, Register::L),
            0xB5 => self.reset_bit(6, Register::L),
            0x86 => {
                let addr = self.registers.get_hl();
                let data = self.mmu.read(addr);
                self.reset_bit_data(0, data);
            }
            0x96 => {
                let addr = self.registers.get_hl();
                let data = self.mmu.read(addr);
                self.reset_bit_data(2, data);
            }
            0xA6 => {
                let addr = self.registers.get_hl();
                let data = self.mmu.read(addr);
                self.reset_bit_data(4, data);
            }
            0xB6 => {
                let addr = self.registers.get_hl();
                let data = self.mmu.read(addr);
                self.reset_bit_data(6, data);
            }
            0x87 => self.reset_bit(0, Register::A),
            0x97 => self.reset_bit(2, Register::A),
            0xA7 => self.reset_bit(4, Register::A),
            0xB7 => self.reset_bit(6, Register::A),
            0x88 => self.reset_bit(1, Register::B),
            0x98 => self.reset_bit(3, Register::B),
            0xA8 => self.reset_bit(5, Register::B),
            0xB8 => self.reset_bit(7, Register::B),
            0x89 => self.reset_bit(1, Register::C),
            0x99 => self.reset_bit(3, Register::C),
            0xA9 => self.reset_bit(5, Register::C),
            0xB9 => self.reset_bit(7, Register::C),
            0x8A => self.reset_bit(1, Register::D),
            0x9A => self.reset_bit(3, Register::D),
            0xAA => self.reset_bit(5, Register::D),
            0xBA => self.reset_bit(7, Register::D),
            0x8B => self.reset_bit(1, Register::E),
            0x9B => self.reset_bit(3, Register::E),
            0xAB => self.reset_bit(5, Register::E),
            0xBB => self.reset_bit(7, Register::E),
            0x8C => self.reset_bit(1, Register::H),
            0x9C => self.reset_bit(3, Register::H),
            0xAC => self.reset_bit(5, Register::H),
            0xBC => self.reset_bit(7, Register::H),
            0x8D => self.reset_bit(1, Register::L),
            0x9D => self.reset_bit(3, Register::L),
            0xAD => self.reset_bit(5, Register::L),
            0xBD => self.reset_bit(7, Register::L),
            0x8E => {
                let addr = self.registers.get_hl();
                let data = self.mmu.read(addr);
                self.reset_bit_data(1, data);
            }
            0x9E => {
                let addr = self.registers.get_hl();
                let data = self.mmu.read(addr);
                self.reset_bit_data(3, data);
            }
            0xAE => {
                let addr = self.registers.get_hl();
                let data = self.mmu.read(addr);
                self.reset_bit_data(5, data);
            }
            0xBE => {
                let addr = self.registers.get_hl();
                let data = self.mmu.read(addr);
                self.reset_bit_data(7, data);
            }
            0x8F => self.reset_bit(1, Register::A),
            0x9F => self.reset_bit(3, Register::A),
            0xAF => self.reset_bit(5, Register::A),
            0xBF => self.reset_bit(7, Register::A),
            _ => panic!("CATASTROPHIC ERROR! OPCODE NOT IN THIS RANGE SHOULD NOT RUN HERE AT ALL!"),
        }
    }
}
