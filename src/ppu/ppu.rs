use crate::cpu::Cpu;
use crate::mmu::mmu::Mmu;

pub struct Ppu {
    pub framebuffer: [[u8; 160]; 144],
    pub oam: [u8; 160],
    pub lcdc: u8,
    pub scx: u8,
    pub scy: u8,
    pub bgp: u8,
    pub ly: u8,
    pub mode: u8,
    pub cycles: u32,
}

impl Ppu {
    pub fn new() -> Self {
        Ppu {
            framebuffer: [[0; 160]; 144],
            oam: [0; 160],
            lcdc: 0,
            scx: 0,
            scy: 0,
            bgp: 0,
            ly: 0,
            mode: 0,
            cycles: 0,
        }
    }
}
// let scx = self.mmu.read(0xFF42);
// let scy = self.mmu.read(0xFF43);
impl Mmu {
    // Handling background work...
    pub fn load_background(&mut self) {
        let map0 = &self.vram[0x1800..0x1C00]; // 0x9800-0x9BFF
        let map1 = &self.vram[0x1C00..0x2000]; // 0x9C00-0x9FFF

        // determine if signed or unsigned
        let signed_mode = ((self.ppu.lcdc >> 4) & 1) == 0;

        // Get bit 3 of lcdc and use that to get the selected map
        let map = if ((self.ppu.lcdc >> 3) & 1) == 1 {
            map1
        } else {
            map0
        };

        for y in 0..144 {
            for x in 0..160 {
                let bg_x = ((x as u16 + self.ppu.scx as u16) & 0xFF) as usize;
                let bg_y = ((y as u16 + self.ppu.scy as u16) & 0xFF) as usize;

                let tile_x = (bg_x / 8) as usize;
                let tile_y = (bg_y / 8) as usize;

                let index = map[tile_y * 32 + tile_x];

                let tile_addr = if !signed_mode {
                    (index as usize) * 16
                } else {
                    let i = index as i8 as i16;
                    (0x1000_i16 + i * 16) as usize
                };

                let pixel_x = bg_x % 8;
                let pixel_y = bg_y % 8;

                let byte1 = self.vram[tile_addr + pixel_y as usize * 2];
                let byte2 = self.vram[tile_addr + pixel_y as usize * 2 + 1];

                let bit = 7 - pixel_x;
                let lo = (byte1 >> bit) & 1;
                let hi = (byte2 >> bit) & 1;

                let color_id = (hi << 1) | lo;

                // mapping to colour
                let shade = (self.ppu.bgp >> (color_id * 2)) & 0b11;

                self.ppu.framebuffer[y][x] = shade;
            }
        }
    }
}
