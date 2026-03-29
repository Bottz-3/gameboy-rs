use crate::cpu::registers::Register16;
use crate::cpu::{Cpu, Register};

impl Cpu {
    pub fn decode_control_flow(&mut self, opcode: u8) {
        match opcode {
            0xC0 => {
                let z = (self.registers.get(Register::F) >> 7) & 1;
                self.ret_conditional(z == 0);
            }
            0xD0 => {
                let c = (self.registers.get(Register::F) >> 4) & 1;
                self.ret_conditional(c == 0);
            }
            // LDH [a8], A
            0xE0 => {
                let addr: u16 = 0xFF00 + self.fetch_u8() as u16;
                self.mmu.write(addr, self.registers.get(Register::A))
            }
            0xF0 => {
                let addr: u16 = 0xFF00 + self.fetch_u8() as u16;
                let data = self.mmu.read(addr);
                self.registers.load_register_data(Register::A, data);
            }
            0xC1 => self.pop(Register16::BC),
            0xD1 => self.pop(Register16::DE),
            0xE1 => self.pop(Register16::HL),
            0xF1 => self.pop(Register16::AF),
            0xC2 => {
                let z = (self.registers.get(Register::F) >> 7) & 1;
                self.jump_conditional(z == 0);
            }
            0xD2 => {
                let c = (self.registers.get(Register::F) >> 4) & 1;
                self.jump_conditional(c == 0);
            }
            0xE2 => {
                let addr: u16 = 0xFF00 + self.registers.get(Register::C) as u16;
                self.mmu.write(addr, self.registers.get(Register::A));
            }
            0xF2 => {
                let addr: u16 = 0xFF00 + self.registers.get(Register::C) as u16;
                let data = self.mmu.read(addr);
                self.registers.load_register_data(Register::A, data);
            }
            0xC3 => self.jump(),
            0xF3 => self.ime = false,
            0xC4 => {
                let z = (self.registers.get(Register::F) >> 7) & 1;
                self.call_conditional(z == 0);
            }
            0xD4 => {
                let c = (self.registers.get(Register::F) >> 4) & 1;
                self.call_conditional(c == 0);
            }
            0xC5 => self.push(Register16::BC),
            0xD5 => self.push(Register16::DE),
            0xE5 => self.push(Register16::HL),
            0xF5 => self.push(Register16::AF),
            0xC6 => {
                let data = self.fetch_u8();
                self.add_data(data);
            }
            0xD6 => {
                let data = self.fetch_u8();
                self.subtract_data(data);
            }
            0xE6 => {
                let data = self.fetch_u8();
                self.and(data);
            }
            0xF6 => {
                let data = self.fetch_u8();
                self.or(data);
            }
            0xC7 => self.restart(0x00),
            0xD7 => self.restart(0x10),
            0xE7 => self.restart(0x20),
            0xF7 => self.restart(0x30),
            0xC8 => {
                let z = (self.registers.get(Register::F) >> 7) & 1;
                self.ret_conditional(z == 1);
            }
            0xD8 => {
                let c = (self.registers.get(Register::F) >> 4) & 1;
                self.ret_conditional(c == 1);
            }
            0xE8 => self.add_sp(self.mmu.read(self.pc) as i8),
            0xF8 => {
                let e = self.mmu.read(self.pc) as i8;
                self.set16(Register16::HL, self.sp + e as u16);
            }
            0xC9 => self.ret(),
            0xD9 => self.ret_from_interrupt(),
            0xE9 => {
                let hl = self.get16(Register16::HL);
                self.jump_conditional(hl == 1);
            }
            0xF9 => {
                let addr = self.fetch_u16();
                let data = self.mmu.read(addr);
                self.registers.load_register_data(Register::A, data);
            }
            0xCA => {
                let z = (self.registers.get(Register::F) >> 7) & 1;
                self.jump_conditional(z == 1);
            }
            0xDA => {
                let c = (self.registers.get(Register::F) >> 4) & 1;
                self.jump_conditional(c == 1);
            }
            0xEA => {
                let addr = self.fetch_u16();
                self.mmu.write(addr, self.registers.get(Register::A));
            }
            0xFA => {
                let addr = self.fetch_u16();
                let data = self.mmu.read(addr);
                self.registers.load_register_data(Register::A, data);
            }
            0xCB => self.decode_prefix(),
            0xFB => self.ime = true,
            0xCC => {
                let z = (self.registers.get(Register::F) >> 7) & 1;
                self.call_conditional(z == 1);
            }
            0xDC => {
                let c = (self.registers.get(Register::F) >> 4) & 1;
                self.call_conditional(c == 1);
            }
            0xCD => self.call(),
            0xCE => {
                let data = self.fetch_u8();
                self.add_carry(data);
            }
            0xDE => {
                let data = self.fetch_u8();
                self.subtract_carry(data)
            }
            0xEE => {
                let data = self.fetch_u8();
                self.xor(data);
            }
            0xFE => {
                let data = self.fetch_u8();
                self.compare_data(data);
            }
            0xCF => self.restart(0x08),
            0xDF => self.restart(0x18),
            0xEF => self.restart(0x28),
            0xFF => self.restart(0x38),

            _ => panic!("CATASTROPHIC ERROR! OPCODE NOT IN THIS RANGE SHOULD NOT RUN HERE AT ALL!"),
        }
    }
}
