use crate::cpu::{Cpu, Register};

impl Cpu {
    pub fn decode_loads(&mut self, opcode: u8) {
        match opcode {
            0x40 => self.registers.load_register(Register::B, Register::B),
            0x50 => self.registers.load_register(Register::D, Register::B),
            0x60 => self.registers.load_register(Register::H, Register::B),
            0x70 => {
                let addr = self.registers.get_hl();
                self.mmu.write(addr, self.registers.get(Register::B))
            }
            0x41 => self.registers.load_register(Register::B, Register::C),
            0x51 => self.registers.load_register(Register::D, Register::C),
            0x61 => self.registers.load_register(Register::H, Register::C),
            0x71 => {
                let addr = self.registers.get_hl();
                self.mmu.write(addr, self.registers.get(Register::C));
            }
            0x42 => self.registers.load_register(Register::B, Register::D),
            0x52 => self.registers.load_register(Register::D, Register::D),
            0x62 => self.registers.load_register(Register::H, Register::D),
            0x72 => {
                let addr = self.registers.get_hl();
                self.mmu.write(addr, self.registers.get(Register::D));
            }
            0x43 => self.registers.load_register(Register::B, Register::E),
            0x53 => self.registers.load_register(Register::D, Register::E),
            0x63 => self.registers.load_register(Register::H, Register::E),
            0x73 => {
                let addr = self.registers.get_hl();
                self.mmu.write(addr, self.registers.get(Register::E));
            }
            0x44 => self.registers.load_register(Register::B, Register::H),
            0x54 => self.registers.load_register(Register::D, Register::H),
            0x64 => self.registers.load_register(Register::H, Register::H),
            0x74 => {
                let addr = self.registers.get_hl();
                self.mmu.write(addr, self.registers.get(Register::H));
            }
            0x45 => self.registers.load_register(Register::B, Register::L),
            0x55 => self.registers.load_register(Register::D, Register::L),
            0x65 => self.registers.load_register(Register::H, Register::L),
            0x75 => {
                let addr = self.registers.get_hl();
                self.mmu.write(addr, self.registers.get(Register::L));
            }
            0x46 => {
                let data = self.mmu.read(self.registers.get_hl());
                self.registers.load_register_data(Register::B, data);
            }
            0x56 => {
                let data = self.mmu.read(self.registers.get_hl());
                self.registers.load_register_data(Register::D, data);
            }
            0x66 => {
                let data = self.mmu.read(self.registers.get_hl());
                self.registers.load_register_data(Register::H, data);
            }
            0x76 => {
                self.halted = true;
                // Should I do the actual halt stuff here?
            }
            0x47 => self.registers.load_register(Register::B, Register::A),
            0x57 => self.registers.load_register(Register::D, Register::A),
            0x67 => self.registers.load_register(Register::H, Register::A),
            0x77 => {
                let data = self.registers.get(Register::A);
                self.mmu.write(self.registers.get_hl(), data)
            }
            0x48 => self.registers.load_register(Register::C, Register::B),
            0x58 => self.registers.load_register(Register::E, Register::B),
            0x68 => self.registers.load_register(Register::L, Register::B),
            0x78 => self.registers.load_register(Register::A, Register::B),
            0x49 => self.registers.load_register(Register::C, Register::C),
            0x59 => self.registers.load_register(Register::E, Register::C),
            0x69 => self.registers.load_register(Register::L, Register::C),
            0x79 => self.registers.load_register(Register::A, Register::C),
            0x4A => self.registers.load_register(Register::C, Register::D),
            0x5A => self.registers.load_register(Register::E, Register::D),
            0x6A => self.registers.load_register(Register::L, Register::D),
            0x7A => self.registers.load_register(Register::A, Register::D),
            0x4B => self.registers.load_register(Register::C, Register::E),
            0x5B => self.registers.load_register(Register::E, Register::E),
            0x6B => self.registers.load_register(Register::L, Register::E),
            0x7B => self.registers.load_register(Register::A, Register::E),
            0x4C => self.registers.load_register(Register::C, Register::H),
            0x5C => self.registers.load_register(Register::E, Register::H),
            0x6C => self.registers.load_register(Register::L, Register::H),
            0x7C => self.registers.load_register(Register::A, Register::H),
            0x4D => self.registers.load_register(Register::C, Register::L),
            0x5D => self.registers.load_register(Register::E, Register::L),
            0x6D => self.registers.load_register(Register::L, Register::L),
            0x7D => self.registers.load_register(Register::A, Register::L),
            0x4E => {
                let data = self.mmu.read(self.registers.get_hl());
                self.registers.load_register_data(Register::C, data);
            }
            0x5E => {
                let data = self.mmu.read(self.registers.get_hl());
                self.registers.load_register_data(Register::E, data);
            }
            0x6E => {
                let data = self.mmu.read(self.registers.get_hl());
                self.registers.load_register_data(Register::L, data);
            }
            0x7E => {
                let data = self.mmu.read(self.registers.get_hl());
                self.registers.load_register_data(Register::A, data);
            }
            0x4F => self.registers.load_register(Register::C, Register::A),
            0x5F => self.registers.load_register(Register::E, Register::A),
            0x6F => self.registers.load_register(Register::L, Register::A),
            0x7F => self.registers.load_register(Register::A, Register::A),

            _ => panic!("CATASTROPHIC ERROR! OPCODE NOT IN THIS RANGE SHOULD NOT RUN HERE AT ALL!"),
        }
    }
}
