use crate::cpu::{Cpu, Register};

impl Cpu {
    pub fn decode_alu(&mut self, opcode: u8) {
        match opcode {
            0x80 => self.add_register(Register::B),
            0x90 => self.subtract_register(Register::B),
            0xA0 => self.and_register(Register::B),
            0xB0 => self.or_register(Register::B),
            0x81 => self.add_register(Register::C),
            0x91 => self.subtract_register(Register::C),
            0xA1 => self.and_register(Register::C),
            0xB1 => self.or_register(Register::C),
            0x82 => self.add_register(Register::D),
            0x92 => self.subtract_register(Register::D),
            0xA2 => self.and_register(Register::D),
            0xB2 => self.or_register(Register::D),
            0x83 => self.add_register(Register::E),
            0x93 => self.subtract_register(Register::E),
            0xA3 => self.and_register(Register::E),
            0xB3 => self.or_register(Register::E),
            0x84 => self.add_register(Register::H),
            0x94 => self.subtract_register(Register::H),
            0xA4 => self.and_register(Register::H),
            0xB4 => self.or_register(Register::H),
            0x85 => self.add_register(Register::L),
            0x95 => self.subtract_register(Register::L),
            0xA5 => self.and_register(Register::L),
            0xB5 => self.or_register(Register::L),
            0x86 => {
                let data = self.mmu.read(self.registers.get_hl());
                self.add_data(data);
            }
            0x96 => {
                let data = self.mmu.read(self.registers.get_hl());
                self.subtract_data(data);
            }
            0xA6 => {
                let data = self.mmu.read(self.registers.get_hl());
                self.and(data);
            }
            0xB6 => {
                let data = self.mmu.read(self.registers.get_hl());
                self.or(data);
            }
            0x87 => self.add_register(Register::A),
            0x97 => self.subtract_register(Register::A),
            0xA7 => self.and_register(Register::A),
            0xB7 => self.or_register(Register::A),
            0x88 => self.add_carry_register(Register::B),
            0x98 => self.subtract_carry_register(Register::B),
            0xA8 => self.xor_register(Register::B),
            0xB8 => self.compare_register(Register::B),
            0x89 => self.add_carry_register(Register::C),
            0x99 => self.subtract_carry_register(Register::C),
            0xA9 => self.xor_register(Register::C),
            0xB9 => self.compare_register(Register::C),
            0x8A => self.add_carry_register(Register::D),
            0x9A => self.subtract_carry_register(Register::D),
            0xAA => self.xor_register(Register::D),
            0xBA => self.compare_register(Register::D),
            0x8B => self.add_carry_register(Register::E),
            0x9B => self.subtract_carry_register(Register::E),
            0xAB => self.xor_register(Register::E),
            0xBB => self.compare_register(Register::E),
            0x8C => self.add_carry_register(Register::H),
            0x9C => self.subtract_carry_register(Register::H),
            0xAC => self.xor_register(Register::H),
            0xBC => self.compare_register(Register::H),
            0x8D => self.add_carry_register(Register::L),
            0x9D => self.subtract_carry_register(Register::L),
            0xAD => self.xor_register(Register::L),
            0xBD => self.compare_register(Register::L),
            0x8E => {
                let data = self.mmu.read(self.registers.get_hl());
                self.add_carry(data);
            }
            0x9E => {
                let data = self.mmu.read(self.registers.get_hl());
                self.subtract_carry(data);
            }
            0xAE => {
                let data = self.mmu.read(self.registers.get_hl());
                self.xor(data);
            }
            0xBE => {
                let data = self.mmu.read(self.registers.get_hl());
                self.compare_data(data);
            }
            0x8F => self.add_carry_register(Register::A),
            0x9F => self.subtract_carry_register(Register::A),
            0xAF => self.xor_register(Register::A),
            0xBF => self.compare_register(Register::A),

            _ => panic!("CATASTROPHIC ERROR! OPCODE NOT IN THIS RANGE SHOULD NOT RUN HERE AT ALL!"),
        }
    }
}
