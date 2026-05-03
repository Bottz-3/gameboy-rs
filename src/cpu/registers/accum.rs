use super::{Register, Registers};
// decimal adjust accumulator
//
impl Registers {
    pub fn daa(&mut self) {
        let mut adj: u8 = 0;
        let mut a = self.get(Register::A);
        let mut f = self.get(Register::F);
        let n = (self.get(Register::F) >> 6) & 1;
        let h = (self.get(Register::F) >> 5) & 1;
        let c = (self.get(Register::F) >> 4) & 1;
        if n == 1 {
            if h == 1 {
                adj += 0x06;
            }
            if c == 1 {
                adj += 0x60;
            }
            a = a.wrapping_sub(adj);
        } else {
            if h == 1 || (a & 0x0F) > 0x09 {
                adj += 0x06;
            }
            if c == 1 || a > 0x99 {
                adj += 0x60;
                f |= 1 << 4;
            }
            a = a.wrapping_add(adj);
        }
        // z and h clear
        f &= !(1 << 7);
        f &= !(1 << 5); // clearing h here
        if a == 0 {
            f |= 1 << 7;
        }

        self.set(Register::A, a);
        self.set(Register::F, f);
    }
    // complement accumulator
    pub fn cpl(&mut self) {
        let mut a = self.get(Register::A);
        a = !a;

        let mut f = self.get(Register::F);

        // setting n and h to 1.
        f |= 1 << 6;
        f |= 1 << 5;

        self.set(Register::F, f & 0xF0);
        self.set(Register::A, a);
    }
}
