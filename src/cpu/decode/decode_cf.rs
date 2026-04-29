use crate::cpu::registers::Register16;
use crate::cpu::{Cpu, Register};

impl Cpu {
    pub fn decode_control_flow(&mut self, opcode: u8) -> u32 {
        match opcode {
            0xC0 => {
                let z = (self.registers.get(Register::F) >> 7) & 1;
                self.ret_conditional(z == 0)
            }
            0xD0 => {
                let c = (self.registers.get(Register::F) >> 4) & 1;
                self.ret_conditional(c == 0)
            }
            // LDH [a8], A
            0xE0 => {
                let addr: u16 = 0xFF00 + self.fetch_u8() as u16;
                self.mmu.write(addr, self.registers.get(Register::A));
                12
            }
            0xF0 => {
                let addr: u16 = 0xFF00 + self.fetch_u8() as u16;
                let data = self.mmu.read(addr);
                self.registers.load_register_data(Register::A, data);
                12
            }
            0xC1 => {
                self.pop(Register16::BC);
                12
            }
            0xD1 => {
                self.pop(Register16::DE);
                12
            }
            0xE1 => {
                self.pop(Register16::HL);
                12
            }
            0xF1 => {
                self.pop(Register16::AF);
                12
            }
            0xC2 => {
                let z = (self.registers.get(Register::F) >> 7) & 1;
                self.jump_conditional(z == 0)
            }
            0xD2 => {
                let c = (self.registers.get(Register::F) >> 4) & 1;
                self.jump_conditional(c == 0)
            }
            0xE2 => {
                let addr: u16 = 0xFF00 + self.registers.get(Register::C) as u16;
                self.mmu.write(addr, self.registers.get(Register::A));
                8
            }
            0xF2 => {
                let addr: u16 = 0xFF00 + self.registers.get(Register::C) as u16;
                let data = self.mmu.read(addr);
                self.registers.load_register_data(Register::A, data);
                8
            }
            0xC3 => {
                self.jump();
                16
            }
            0xF3 => {
                self.ime = false;
                4
            }
            0xC4 => {
                let z = (self.registers.get(Register::F) >> 7) & 1;
                self.call_conditional(z == 0)
            }
            0xD4 => {
                let c = (self.registers.get(Register::F) >> 4) & 1;
                self.call_conditional(c == 0)
            }
            0xC5 => {
                self.push(Register16::BC);
                16
            }
            0xD5 => {
                self.push(Register16::DE);
                16
            }
            0xE5 => {
                self.push(Register16::HL);
                16
            }
            0xF5 => {
                self.push(Register16::AF);
                16
            }
            0xC6 => {
                let data = self.fetch_u8();
                self.add_data(data);
                8
            }
            0xD6 => {
                let data = self.fetch_u8();
                self.subtract_data(data);
                8
            }
            0xE6 => {
                let data = self.fetch_u8();
                self.and(data);
                8
            }
            0xF6 => {
                let data = self.fetch_u8();
                self.or(data);
                8
            }
            0xC7 => {
                self.restart(0x00);
                16
            }
            0xD7 => {
                self.restart(0x10);
                16
            }
            0xE7 => {
                self.restart(0x20);
                16
            }
            0xF7 => {
                self.restart(0x30);
                16
            }
            0xC8 => {
                let z = (self.registers.get(Register::F) >> 7) & 1;
                self.ret_conditional(z == 1)
            }
            0xD8 => {
                let c = (self.registers.get(Register::F) >> 4) & 1;
                self.ret_conditional(c == 1)
            }
            0xE8 => {
                let data = self.fetch_u8();
                self.add_sp(data as i8);
                16
            }
            0xF8 => {
                let e = self.fetch_u8() as i8;
                let val = self.sp.wrapping_add_signed(e as i16);
                self.set16(Register16::HL, val);
                12
            }
            0xC9 => {
                self.ret();
                16
            }
            0xD9 => {
                self.ret_from_interrupt();
                16
            }
            0xE9 => {
                self.pc = self.registers.get_hl();
                4
            }
            0xF9 => {
                self.sp = self.get16(Register16::HL);
                8
            }
            0xCA => {
                let z = (self.registers.get(Register::F) >> 7) & 1;
                self.jump_conditional(z == 1)
            }
            0xDA => {
                let c = (self.registers.get(Register::F) >> 4) & 1;
                self.jump_conditional(c == 1)
            }
            0xEA => {
                let addr = self.fetch_u16();
                self.mmu.write(addr, self.registers.get(Register::A));
                16
            }
            0xFA => {
                let addr = self.fetch_u16();
                let data = self.mmu.read(addr);
                self.registers.load_register_data(Register::A, data);
                16
            }
            0xCB => {
                let next_op = self.fetch_u8();
                self.decode_cb(next_op);
                4
            }
            0xFB => {
                self.ime = true;
                4
            }
            0xCC => {
                let z = (self.registers.get(Register::F) >> 7) & 1;
                self.call_conditional(z == 1)
            }
            0xDC => {
                let c = (self.registers.get(Register::F) >> 4) & 1;
                self.call_conditional(c == 1)
            }
            0xCD => {
                self.call();
                24
            }
            0xCE => {
                let data = self.fetch_u8();
                self.add_carry(data);
                8
            }
            0xDE => {
                let data = self.fetch_u8();
                self.subtract_carry(data);
                8
            }
            0xEE => {
                let data = self.fetch_u8();
                self.xor(data);
                8
            }
            0xFE => {
                let data = self.fetch_u8();
                self.compare_data(data);
                8
            }
            0xCF => {
                self.restart(0x08);
                16
            }
            0xDF => {
                self.restart(0x18);
                16
            }
            0xEF => {
                self.restart(0x28);
                16
            }
            0xFF => {
                self.restart(0x38);
                16
            }

            _ => panic!("CATASTROPHIC ERROR! OPCODE NOT IN THIS RANGE SHOULD NOT RUN HERE AT ALL!"),
        }
    }
}
