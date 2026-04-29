use crate::cpu::Cpu;

impl Cpu {
    // 0xCB opcode
    pub fn decode_cb(&mut self, opcode: u8) -> u32 {
        match opcode {
            0x00..=0x3F => self.decode_cb_misc(opcode),
            0x40..=0x7F => self.decode_cb_bit(opcode),
            0x80..=0xBF => self.decode_cb_res(opcode),
            0xC0..=0xFF => self.decode_cb_set(opcode),
        }
    }
}
