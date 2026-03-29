use crate::cpu::Cpu;

impl Cpu {
    pub fn decode(&mut self, opcode: u8) {
        match opcode {
            0x00..=0x3F => self.decode_misc(opcode),
            0x40..=0x7F => self.decode_loads(opcode),
            0x80..=0xBF => self.decode_alu(opcode),
            0xC0..=0xFF => self.decode_control_flow(opcode),
        }
    }
}
