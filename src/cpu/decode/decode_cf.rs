use crate::cpu::{Cpu, Register};

impl Cpu {
    pub fn decode_control_flow(&mut self, opcode: u8) {
        match opcode {
            _ => panic!("CATASTROPHIC ERROR! OPCODE NOT IN THIS RANGE SHOULD NOT RUN HERE AT ALL!"),
        }
    }
}
