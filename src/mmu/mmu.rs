pub struct Mmu {
    memory: [u8; 65536],
}

impl Mmu {
    pub fn read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        self.memory[addr as usize] = val;
    }
}
