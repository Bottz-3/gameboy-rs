use crate::cpu::registers::Register16;
use crate::cpu::{Cpu, Register};

// Doing it in groups of register
impl Cpu {
    pub fn decode_misc(&mut self, opcode: u8) {
        match opcode {
            0x00 => {} // NOP
            0x10 => {} // Not implemented for MVP (STOP instruc.)
            0x20 => {
                let z = (self.registers.get(Register::F) >> 7) & 1;
                self.jump_relative_conditional(z == 0);
            }
            0x30 => {
                let c = (self.registers.get(Register::F) >> 4) & 1;
                self.jump_relative_conditional(c == 0);
            }
            // loads
            0x01 => {
                let data = self.fetch_u16();
                self.registers.set_bc(data)
            }
            0x11 => {
                let data = self.fetch_u16();
                self.registers.set_de(data)
            }
            0x21 => {
                let data = self.fetch_u16();
                self.registers.set_hl(data)
            }
            0x31 => self.sp = self.fetch_u16(),
            // indirect(?) loads
            0x02 => {
                let addr = self.registers.get_bc();
                self.mmu.write(addr, self.registers.get(Register::A));
            }
            0x12 => {
                let addr = self.registers.get_de();
                self.mmu.write(addr, self.registers.get(Register::A));
            }
            0x22 => {
                let addr = self.registers.get_hl();
                self.mmu.write(addr, self.registers.get(Register::A));
                self.registers.set_hl(addr.wrapping_add(1));
            }
            0x32 => {
                let addr = self.registers.get_hl();
                self.mmu.write(addr, self.registers.get(Register::A));
                self.registers.set_hl(addr.wrapping_sub(1));
            }
            // 16-bit increments
            0x03 => self
                .registers
                .set_bc(self.registers.get_bc().wrapping_add(1)),
            0x13 => self
                .registers
                .set_de(self.registers.get_de().wrapping_add(1)),
            0x23 => self
                .registers
                .set_hl(self.registers.get_hl().wrapping_add(1)),
            0x33 => self.sp = self.sp.wrapping_add(1),
            // 8-bit increments
            0x04 => {
                let reg_val = self.registers.get(Register::B);
                let increment = self.increment(reg_val);
                self.registers.set(Register::B, increment);
            }
            0x14 => {
                let reg_val = self.registers.get(Register::D);
                let increment = self.increment(reg_val);
                self.registers.set(Register::D, increment);
            }
            0x24 => {
                let reg_val = self.registers.get(Register::H);
                let increment = self.increment(reg_val);
                self.registers.set(Register::H, increment);
            }
            0x34 => {
                let addr = self.registers.get_hl();
                let reg_val = self.mmu.read(addr);
                let increment = self.increment(reg_val);
                self.mmu.write(addr, increment);
            }
            // 8-bit decrements
            0x05 => {
                let reg_val = self.registers.get(Register::B);
                let decrement = self.decrement(reg_val);
                self.registers.set(Register::B, decrement);
            }
            0x15 => {
                let reg_val = self.registers.get(Register::D);
                let decrement = self.decrement(reg_val);
                self.registers.set(Register::D, decrement);
            }
            0x25 => {
                let reg_val = self.registers.get(Register::H);
                let decrement = self.decrement(reg_val);
                self.registers.set(Register::H, decrement);
            }
            0x35 => {
                let addr = self.registers.get_hl();
                let reg_val = self.mmu.read(addr);
                let decrement = self.decrement(reg_val);
                self.mmu.write(addr, decrement);
            }
            0x06 => {
                let data = self.fetch_u8();
                self.registers.load_register_data(Register::B, data)
            }
            0x16 => {
                let data = self.fetch_u8();
                self.registers.load_register_data(Register::D, data)
            }
            0x26 => {
                let data = self.fetch_u8();
                self.registers.load_register_data(Register::H, data)
            }
            0x36 => {
                let addr = self.registers.get_hl();
                let data = self.fetch_u8();
                self.mmu.write(addr, data);
            }
            0x07 => self.rotate_left_circular(),
            0x17 => self.rotate_left_accum(),
            0x27 => self.registers.daa(),
            0x37 => self.registers.scf(),
            0x08 => {
                let addr = self.fetch_u16();
                self.mmu.write(addr, (self.sp & 0xFF) as u8);
                self.mmu.write(addr + 1, (self.sp >> 8) as u8);
            }
            0x18 => self.jump_relative(),
            0x28 => {
                let z = (self.registers.get(Register::F) >> 7) & 1;
                self.jump_relative_conditional(z != 0)
            }
            0x38 => {
                let c = (self.registers.get(Register::F) >> 4) & 1;
                self.jump_relative_conditional(c != 0)
            }
            0x09 => self.add_16(Register16::BC),
            0x19 => self.add_16(Register16::DE),
            0x29 => self.add_16(Register16::HL),
            0x39 => self.add_16(Register16::SP),
            0x0A => {
                let data = self.mmu.read(self.registers.get_bc());
                self.registers.load_register_data(Register::A, data);
            }
            0x1A => {
                let data = self.mmu.read(self.registers.get_de());
                self.registers.load_register_data(Register::A, data);
            }
            0x2A => {
                let data = self.mmu.read(self.registers.get_hl());
                self.registers.load_register_data(Register::A, data);
                self.registers
                    .set_hl(self.registers.get_hl().wrapping_add(1))
            }
            0x3A => {
                let data = self.mmu.read(self.registers.get_hl());
                self.registers.load_register_data(Register::A, data);
                self.registers
                    .set_hl(self.registers.get_hl().wrapping_sub(1))
            }
            0x0B => self
                .registers
                .set_bc(self.registers.get_bc().wrapping_sub(1)),
            0x1B => self
                .registers
                .set_de(self.registers.get_de().wrapping_sub(1)),
            0x2B => self
                .registers
                .set_hl(self.registers.get_hl().wrapping_sub(1)),
            0x3B => self.sp = self.sp.wrapping_sub(1),
            0x0C => {
                let reg = self.registers.get(Register::C);
                let increment = self.increment(reg);
                self.registers.set(Register::C, increment);
            }
            0x1C => {
                let reg = self.registers.get(Register::E);
                let increment = self.increment(reg);
                self.registers.set(Register::E, increment);
            }
            0x2C => {
                let reg = self.registers.get(Register::L);
                let increment = self.increment(reg);
                self.registers.set(Register::L, increment);
            }
            0x3C => {
                let reg = self.registers.get(Register::A);
                let increment = self.increment(reg);
                self.registers.set(Register::A, increment);
            }
            0x0D => {
                let reg = self.registers.get(Register::C);
                let decrement = self.decrement(reg);
                self.registers.set(Register::C, decrement);
            }
            0x1D => {
                let reg = self.registers.get(Register::E);
                let decrement = self.decrement(reg);
                self.registers.set(Register::E, decrement);
            }
            0x2D => {
                let reg = self.registers.get(Register::L);
                let decrement = self.decrement(reg);
                self.registers.set(Register::L, decrement);
            }
            0x3D => {
                let reg = self.registers.get(Register::A);
                let decrement = self.decrement(reg);
                self.registers.set(Register::A, decrement);
            }
            0x0E => {
                let data = self.fetch_u8();
                self.registers.load_register_data(Register::C, data)
            }
            0x1E => {
                let data = self.fetch_u8();
                self.registers.load_register_data(Register::E, data)
            }
            0x2E => {
                let data = self.fetch_u8();
                self.registers.load_register_data(Register::L, data)
            }
            0x3E => {
                let data = self.fetch_u8();
                self.registers.load_register_data(Register::A, data)
            }
            0x0F => self.rotate_right_circular(),
            0x1F => self.rotate_right_accum(),
            0x2F => self.registers.cpl(),
            0x3F => self.registers.ccf(),
            _ => panic!("CATASTROPHIC ERROR! OPCODE NOT IN THIS RANGE SHOULD NOT RUN HERE AT ALL!"),
        }
    }
}
