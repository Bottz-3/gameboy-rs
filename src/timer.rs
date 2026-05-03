pub struct Timer {
    pub div: u8,
    pub tima: u8,
    pub tma: u8,
    pub tac: u8,
    div_cycles: u32,
    timer_cycles: u32,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,
            div_cycles: 0,
            timer_cycles: 0,
        }
    }
    pub fn is_enabled(&self) -> bool {
        let val = (self.tac >> 2) & 1;
        val == 1
    }
    pub fn threshold(&self) -> u32 {
        let select = self.tac & 0b11;

        match select {
            0 => 1024,
            1 => 16,
            2 => 64,
            3 => 256,
            _ => 1024,
        }
    }
}

impl Timer {
    pub fn step(&mut self, cycles: u32) -> bool {
        self.div_cycles += cycles;
        if self.div_cycles >= 256 {
            self.div_cycles -= 256;
            self.div = self.div.wrapping_add(1);
        }

        if !self.is_enabled() {
            return false;
        }

        let thres = self.threshold();

        self.timer_cycles += cycles;
        if self.timer_cycles >= thres {
            self.timer_cycles -= thres;
            self.tima = self.tima.wrapping_add(1);
            if self.tima == 0 {
                self.tima = self.tma;
                return true;
            }
        }
        false
    }
}
