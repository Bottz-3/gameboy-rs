use super::{Register, Registers};

// 8-bit load instructions
impl Registers {
    // Load register (pg 20)
    pub fn load_register(&mut self, reg1: Register, reg2: Register) {
        let reg_val = self.get(reg2);
        self.set(reg1, reg_val);
    }
    // This provides the use case for loading data to a register
    // Such as for pg 21 and 22 (indirect HL and immediate)
    // You handle the data at the callsite eg.
    // get_hl() -> load_register_data()
    pub fn load_register_data(&mut self, reg: Register, data: u8) {
        self.set(reg, data);
    }
}
