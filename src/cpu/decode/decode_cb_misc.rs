use crate::cpu::{Cpu, Register};

impl Cpu {
    pub fn decode_cb_misc(&mut self, opcode: u8) {
        match opcode {
            0x00 => self.rotate_left_register(Register::B),
            0x10 => self.rotate_left(Register::B),
            0x20 => self.shift_left_arith(Register::B),
            0x30 => self.swap_nibbles(Register::B),
            0x01 => self.rotate_left_register(Register::C),
            0x11 => self.rotate_left(Register::C),
            0x21 => self.shift_left_arith(Register::C),
            0x31 => self.swap_nibbles(Register::C),
            0x02 => self.rotate_left_register(Register::D),
            0x12 => self.rotate_left(Register::D),
            0x22 => self.shift_left_arith(Register::D),
            0x32 => self.swap_nibbles(Register::D),
            0x03 => self.rotate_left_register(Register::E),
            0x13 => self.rotate_left(Register::E),
            0x23 => self.shift_left_arith(Register::E),
            0x33 => self.swap_nibbles(Register::E),
            0x04 => self.rotate_left_register(Register::H),
            0x14 => self.rotate_left(Register::H),
            0x24 => self.shift_left_arith(Register::H),
            0x34 => self.swap_nibbles(Register::H),
            0x05 => self.rotate_left_register(Register::L),
            0x15 => self.rotate_left(Register::L),
            0x25 => self.shift_left_arith(Register::L),
            0x35 => self.swap_nibbles(Register::L),
            0x06 => {
                let addr = self.registers.get_hl();
                let data = self.mmu.read(addr);
                let rot = self.rotate_left_circular_data(data);
                self.mmu.write(addr, rot);
            }
            0x16 => {
                let addr = self.registers.get_hl();
                let data = self.mmu.read(addr);
                let rot = self.rotate_left_data(data);
                self.mmu.write(addr, rot);
            }
            0x26 => {
                let addr = self.registers.get_hl();
                let data = self.mmu.read(addr);
                let shift = self.shift_left_arith_data(data);
                self.mmu.write(addr, shift);
            }
            0x36 => {
                let addr = self.registers.get_hl();
                let data = self.mmu.read(addr);
                let swap = self.swap_nibbles_data(data);
                self.mmu.write(addr, swap);
            }
            0x07 => self.rotate_left_register(Register::A),
            0x17 => self.rotate_left(Register::A),
            0x27 => self.shift_left_arith(Register::A),
            0x37 => self.swap_nibbles(Register::A),
            0x08 => self.rotate_right_register(Register::B),
            0x18 => self.rotate_right(Register::B),
            0x28 => self.shift_right_arith(Register::B),
            0x38 => self.shift_right_logical(Register::B),
            0x09 => self.rotate_right_register(Register::C),
            0x19 => self.rotate_right(Register::C),
            0x29 => self.shift_right_arith(Register::C),
            0x39 => self.shift_right_logical(Register::C),
            0x0A => self.rotate_right_register(Register::D),
            0x1A => self.rotate_right(Register::D),
            0x2A => self.shift_right_arith(Register::D),
            0x3A => self.shift_right_logical(Register::D),
            0x0B => self.rotate_right_register(Register::E),
            0x1B => self.rotate_right(Register::E),
            0x2B => self.shift_right_arith(Register::E),
            0x3B => self.shift_right_logical(Register::E),
            0x0C => self.rotate_right_register(Register::H),
            0x1C => self.rotate_right(Register::H),
            0x2C => self.shift_right_arith(Register::H),
            0x3C => self.shift_right_logical(Register::H),
            0x0D => self.rotate_right_register(Register::L),
            0x1D => self.rotate_right(Register::L),
            0x2D => self.shift_right_arith(Register::L),
            0x3D => self.shift_right_logical(Register::L),
            0x0E => {
                let addr = self.registers.get_hl();
                let data = self.mmu.read(addr);
                let rot = self.rotate_right_circular_data(data);
                self.mmu.write(addr, rot);
            }
            0x1E => {
                let addr = self.registers.get_hl();
                let data = self.mmu.read(addr);
                let rot = self.rotate_right_data(data);
                self.mmu.write(addr, rot);
            }
            0x2E => {
                let addr = self.registers.get_hl();
                let data = self.mmu.read(addr);
                let shift = self.shift_right_arith_data(data);
                self.mmu.write(addr, shift);
            }
            0x3E => {
                let addr = self.registers.get_hl();
                let data = self.mmu.read(addr);
                let shift = self.shift_right_logical_data(data);
                self.mmu.write(addr, shift);
            }
            0x0F => self.rotate_right_register(Register::A),
            0x1F => self.rotate_right(Register::A),
            0x2F => self.shift_right_arith(Register::A),
            0x3F => self.shift_right_logical(Register::A),

            _ => panic!("CATASTROPHIC ERROR! OPCODE NOT IN THIS RANGE SHOULD NOT RUN HERE AT ALL!"),
        }
    }
}
