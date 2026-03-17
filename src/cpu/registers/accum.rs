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
                adj += 0x6;
            }
            if c == 1 {
                adj += 0x60;
                // set c flag
                f |= 1 << 4;
            }
            a = a.wrapping_sub(adj);
        } else {
            if h == 1 || (a & 0xf) > 0x9 {
                adj += 0x6;
            }
            if c == 1 || a > 0x99 {
                adj += 0x60;
            }
            a = a.wrapping_add(adj);
        }

        if a == 0 {
            f |= 1 << 7;
        }
        f &= !(1 << 5); // clearing h here

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

        self.set(Register::F, f);
        self.set(Register::A, a);
    }
}
