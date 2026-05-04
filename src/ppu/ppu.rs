use crate::cpu::Cpu;
use crate::mmu::mmu::Mmu;

pub struct Ppu {
    pub framebuffer: [[u8; 160]; 144],
    pub oam: [u8; 160],
    pub lcdc: u8,
    pub scx: u8,
    pub scy: u8,
    pub wx: u8,
    pub wy: u8,
    pub bgp: u8,
    pub ly: u8,
    pub mode: u8,
    pub cycles: u32,
    vblank_triggered: bool,
}

impl Ppu {
    pub fn new() -> Self {
        Ppu {
            framebuffer: [[0; 160]; 144],
            oam: [0; 160],
            lcdc: 0,
            scx: 0,
            scy: 0,
            wx: 7,
            wy: 0,
            bgp: 0,
            ly: 0,
            mode: 0,
            cycles: 0,
            vblank_triggered: false,
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

impl Ppu {
    pub fn step(&mut self, cycles: u32) -> bool {
        self.cycles += cycles;
        let vblank = false;
        match self.mode {
            2 => {
                if self.cycles >= 80 {
                    self.cycles -= 80;
                    self.mode = 3;
                }
            }
            3 => {
                // hblank
                if self.cycles >= 172 {
                    self.cycles -= 172;
                    self.mode = 0;
                }
            }
            0 => {
                if self.cycles >= 204 {
                    self.cycles -= 204;
                    self.ly += 1;
                    if self.ly == 144 {
                        self.mode = 1; //vblank
                        if !self.vblank_triggered {
                            self.vblank_triggered = true;
                            return true;
                        }
                    } else {
                        self.mode = 2;
                    }
                }
            }
            1 => {
                if self.cycles >= 456 {
                    self.cycles -= 456;
                    self.ly += 1;
                    if self.ly > 153 {
                        self.ly = 0;
                        self.mode = 2;
                        self.vblank_triggered = false;
                    }
                }
            }
            _ => {}
        }
        vblank
    }
}

impl Mmu {
    pub fn render_window(&mut self) {
        let window_enabled = ((self.ppu.lcdc >> 5) & 1) == 1;
        let bg_enabled = (self.ppu.lcdc & 1) == 1;
        if !(window_enabled && bg_enabled) {
            return;
        }

        let map0 = &self.vram[0x1800..0x1C00]; // 0x9800-0x9BFF
        let map1 = &self.vram[0x1C00..0x2000]; // 0x9C00-0x9FFF
        let map = if ((self.ppu.lcdc >> 6) & 1) == 1 {
            map1
        } else {
            map0
        };
        let signed_mode = ((self.ppu.lcdc >> 4) & 1) == 0;

        let wx = self.ppu.wx.saturating_sub(7);

        for y in 0..144 {
            if y < self.ppu.wy {
                continue;
            }
            for x in 0..160 {
                if x + 7 < self.ppu.wx {
                    continue;
                }
                let win_x = x - wx;
                let win_y = y - self.ppu.wy;

                let tile_x = (win_x / 8) as usize; // FIX maybe: OOB access??
                let tile_y = (win_y / 8) as usize;

                let index = map[tile_y * 32 + tile_x];

                let tile_addr = if !signed_mode {
                    (index as usize) * 16
                } else {
                    let i = index as i8 as i16;
                    (0x1000_i16 + i * 16) as usize
                };

                let pixel_x = win_x % 8;
                let pixel_y = win_y % 8;

                let byte1 = self.vram[tile_addr + pixel_y as usize * 2];
                let byte2 = self.vram[tile_addr + pixel_y as usize * 2 + 1];

                let bit = 7 - pixel_x;
                let lo = (byte1 >> bit) & 1;
                let hi = (byte2 >> bit) & 1;

                let color_id = (hi << 1) | lo;

                // mapping to colour
                let shade = (self.ppu.bgp >> (color_id * 2)) & 0b11;

                self.ppu.framebuffer[y as usize][x as usize] = shade;
            }
        }
    }
}

impl Mmu {
    pub fn render_sprites(&mut self) {
        let sprite_size: u8 = if ((self.ppu.lcdc >> 2) & 1) == 1 {
            16
        } else {
            8
        };

        for i in 0..40 {
            let base = i * 4;
            let sy = self.ppu.oam[base].wrapping_sub(16);
            let sx = self.ppu.oam[base + 1].wrapping_sub(8);
            let tile_index = self.ppu.oam[base + 2];
            let flags = self.ppu.oam[base + 3];

            let priority = (flags >> 7) & 1 == 1;

            let y_flip = (flags >> 6) & 1 == 1;
            let x_flip = (flags >> 5) & 1 == 1;

            let palette = if (flags >> 4) & 1 == 1 {
                self.obp1
            } else {
                self.obp0
            };

            for row in 0..sprite_size {
                let py = sy.wrapping_add(row);
                if py >= 144 {
                    continue;
                }

                let tile_row = if y_flip { sprite_size - 1 - row } else { row };
                let byte1 = self.vram[(tile_index as usize) * 16 + tile_row as usize * 2];
                let byte2 = self.vram[(tile_index as usize) * 16 + tile_row as usize * 2 + 1];

                for col in 0..8_u8 {
                    let px = sx.wrapping_add(col);
                    if px >= 160 {
                        continue;
                    }
                    let bit = if x_flip { col } else { 7 - col };

                    let lo = (byte1 >> bit) & 1;
                    let hi = (byte2 >> bit) & 1;
                    // need to check if naming is consistent
                    let color_id = (hi << 1) | lo;

                    if color_id == 0 {
                        continue;
                    }
                    if priority && self.ppu.framebuffer[py as usize][px as usize] != 0 {
                        continue;
                    }
                    let shade = (palette >> (color_id * 2)) & 0b11;
                    self.ppu.framebuffer[py as usize][px as usize] = shade;
                }
            }
        }
    }
}
