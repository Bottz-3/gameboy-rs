use crate::cpu::{Cpu, Register};

// Doing it in groups of register
impl Cpu {
    pub fn decode_misc(&mut self, opcode: u8) {
        match opcode {
            0x00 => {} // NOP
            0x10 => {} // Not implemented for MVP
            0x20 => {
                let z = (self.registers.get(Register::F) >> 7) & 1;
                self.jump_relative_conditional(z != 0);
            }
            0x30 => {
                let c = (self.registers.get(Register::F) >> 4) & 1;
                self.jump_relative_conditional(c != 0);
            }
            // loads
            0x01 => self.registers.set_bc(self.fetch_u16()),
            0x11 => self.registers.set_de(self.fetch_u16()),
            0x21 => self.registers.set_hl(self.fetch_u16()),
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
            // increments
            0x03 => self
                .registers
                .set_bc(self.registers.get_bc().wrapping_add(1)),
            0x13 => self
                .registers
                .set_de(self.registers.get_de().wrapping_add(1)),
            0x23 => self
                .registers
                .set_hl(self.registers.get_hl().wrapping_add(1)),
            0x04 => self.increment(Register::B),
            0x05 => self.decrement(Register::B),
            0x06 => self.lo,
        }
    }
}
