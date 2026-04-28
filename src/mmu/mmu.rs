use crate::ppu::ppu::Ppu;
pub struct Mmu {
    rom: Vec<u8>,
    pub vram: [u8; 8192],
    pub wram: [u8; 8192],
    pub hram: [u8; 256],
    pub ppu: Ppu,
}

impl Mmu {
    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7FFF => self.rom[addr as usize],
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize],
            0xC000..=0xDFFF => self.wram[(addr - 0xC000) as usize],
            0xFF00..=0xFF7F => self.read_io(addr),
            0xFF80..=0xFFFF => self.hram[(addr - 0xFF80) as usize],
            _ => 0xFF,
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000..=0x7FFF => {}
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize] = val,
            0xC000..=0xDFFF => self.wram[(addr - 0xC000) as usize] = val,
            0xFF00..=0xFF7F => self.write_io(addr, val),
            0xFF80..=0xFFFF => self.hram[(addr - 0xFF80) as usize] = val,
            _ => {}
        }
    }
}

impl Mmu {
    pub fn read_io(&self, addr: u16) -> u8 {
        match addr {
            0xFF40 => self.ppu.lcdc,
            0xFF42 => self.ppu.scy,
            0xFF43 => self.ppu.scx,
            0xFF47 => self.ppu.bgp,
            _ => 0xFF,
        }
    }
    pub fn write_io(&mut self, addr: u16, val: u8) {
        match addr {
            0xFF40 => self.ppu.lcdc = val,
            0xFF42 => self.ppu.scy = val,
            0xFF43 => self.ppu.scx = val,
            0xFF47 => self.ppu.bgp = val,
            _ => {}
        }
    }
}
