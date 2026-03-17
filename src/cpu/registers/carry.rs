use super::{Register, Registers};
// Carry

impl Registers {
    // complement carry flag
    pub fn ccf(&mut self) {
        let mut f = self.get(Register::F);
        // n set to 0
        f &= !(1 << 6);
        // h set to 0
        f &= !(1 << 5);
        // flip c flag
        f ^= 1 << 4;
        self.set(Register::F, f);
    }
    // set carry flag
    pub fn scf(&mut self) {
        let mut f = self.get(Register::F);
        // n set to 0
        f &= !(1 << 6);
        // h set to 0
        f &= !(1 << 5);
        // set c to 1
        f |= 1 << 4;
        self.set(Register::F, f);
    }
}
