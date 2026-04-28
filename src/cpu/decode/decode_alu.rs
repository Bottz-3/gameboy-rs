use crate::cpu::{Cpu, Register};

impl Cpu {
    pub fn decode_alu(&mut self, opcode: u8) -> u32 {
        match opcode {
            0x80 => {
                self.add_register(Register::B);
                4
            }
            0x90 => {
                self.subtract_register(Register::B);
                4
            }
            0xA0 => {
                self.and_register(Register::B);
                4
            }
            0xB0 => {
                self.or_register(Register::B);
                4
            }
            0x81 => {
                self.add_register(Register::C);
                4
            }
            0x91 => {
                self.subtract_register(Register::C);
                4
            }
            0xA1 => {
                self.and_register(Register::C);
                4
            }
            0xB1 => {
                self.or_register(Register::C);
                4
            }
            0x82 => {
                self.add_register(Register::D);
                4
            }
            0x92 => {
                self.subtract_register(Register::D);
                4
            }
            0xA2 => {
                self.and_register(Register::D);
                4
            }
            0xB2 => {
                self.or_register(Register::D);
                4
            }
            0x83 => {
                self.add_register(Register::E);
                4
            }
            0x93 => {
                self.subtract_register(Register::E);
                4
            }
            0xA3 => {
                self.and_register(Register::E);
                4
            }
            0xB3 => {
                self.or_register(Register::E);
                4
            }
            0x84 => {
                self.add_register(Register::H);
                4
            }
            0x94 => {
                self.subtract_register(Register::H);
                4
            }
            0xA4 => {
                self.and_register(Register::H);
                4
            }
            0xB4 => {
                self.or_register(Register::H);
                4
            }
            0x85 => {
                self.add_register(Register::L);
                4
            }
            0x95 => {
                self.subtract_register(Register::L);
                4
            }
            0xA5 => {
                self.and_register(Register::L);
                4
            }
            0xB5 => {
                self.or_register(Register::L);
                4
            }
            0x86 => {
                let data = self.mmu.read(self.registers.get_hl());
                self.add_data(data);
                8
            }
            0x96 => {
                let data = self.mmu.read(self.registers.get_hl());
                self.subtract_data(data);
                8
            }
            0xA6 => {
                let data = self.mmu.read(self.registers.get_hl());
                self.and(data);
                8
            }
            0xB6 => {
                let data = self.mmu.read(self.registers.get_hl());
                self.or(data);
                8
            }
            0x87 => {
                self.add_register(Register::A);
                4
            }
            0x97 => {
                self.subtract_register(Register::A);
                4
            }
            0xA7 => {
                self.and_register(Register::A);
                4
            }
            0xB7 => {
                self.or_register(Register::A);
                4
            }
            0x88 => {
                self.add_carry_register(Register::B);
                4
            }
            0x98 => {
                self.subtract_carry_register(Register::B);
                4
            }
            0xA8 => {
                self.xor_register(Register::B);
                4
            }
            0xB8 => {
                self.compare_register(Register::B);
                4
            }
            0x89 => {
                self.add_carry_register(Register::C);
                4
            }
            0x99 => {
                self.subtract_carry_register(Register::C);
                4
            }
            0xA9 => {
                self.xor_register(Register::C);
                4
            }
            0xB9 => {
                self.compare_register(Register::C);
                4
            }
            0x8A => {
                self.add_carry_register(Register::D);
                4
            }
            0x9A => {
                self.subtract_carry_register(Register::D);
                4
            }
            0xAA => {
                self.xor_register(Register::D);
                4
            }
            0xBA => {
                self.compare_register(Register::D);
                4
            }
            0x8B => {
                self.add_carry_register(Register::E);
                4
            }
            0x9B => {
                self.subtract_carry_register(Register::E);
                4
            }
            0xAB => {
                self.xor_register(Register::E);
                4
            }
            0xBB => {
                self.compare_register(Register::E);
                4
            }
            0x8C => {
                self.add_carry_register(Register::H);
                4
            }
            0x9C => {
                self.subtract_carry_register(Register::H);
                4
            }
            0xAC => {
                self.xor_register(Register::H);
                4
            }
            0xBC => {
                self.compare_register(Register::H);
                4
            }
            0x8D => {
                self.add_carry_register(Register::L);
                4
            }
            0x9D => {
                self.subtract_carry_register(Register::L);
                4
            }
            0xAD => {
                self.xor_register(Register::L);
                4
            }
            0xBD => {
                self.compare_register(Register::L);
                4
            }
            0x8E => {
                let data = self.mmu.read(self.registers.get_hl());
                self.add_carry(data);
                8
            }
            0x9E => {
                let data = self.mmu.read(self.registers.get_hl());
                self.subtract_carry(data);
                8
            }
            0xAE => {
                let data = self.mmu.read(self.registers.get_hl());
                self.xor(data);
                8
            }
            0xBE => {
                let data = self.mmu.read(self.registers.get_hl());
                self.compare_data(data);
                8
            }
            0x8F => {
                self.add_carry_register(Register::A);
                4
            }
            0x9F => {
                self.subtract_carry_register(Register::A);
                4
            }
            0xAF => {
                self.xor_register(Register::A);
                4
            }
            0xBF => {
                self.compare_register(Register::A);
                4
            }

            _ => panic!("CATASTROPHIC ERROR! OPCODE NOT IN THIS RANGE SHOULD NOT RUN HERE AT ALL!"),
        }
    }
}
