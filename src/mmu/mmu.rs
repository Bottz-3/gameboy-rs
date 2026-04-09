pub struct Mmu {
    rom: Vec<u8>,
    vram: [u8; 8192],
    wram: [u8; 8192],
    hram: [u8; 256],
}

impl Mmu {
    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7FFF => self.rom[addr as usize],
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize],
            0xC000..=0xDFFF => self.wram[(addr - 0xC000) as usize],
            0xFF00..=0xFFFF => self.hram[(addr - 0xFF00) as usize],
            _ => 0xFF,
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000..=0x7FFF => {}
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize] = val,
            0xC000..=0xDFFF => self.wram[(addr - 0xC000) as usize] = val,
            0xFF00..=0xFFFF => self.hram[(addr - 0xFF00) as usize] = val,
            _ => {}
        }
    }
}
