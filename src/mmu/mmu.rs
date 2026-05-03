use crate::ppu::ppu::Ppu;
use crate::timer::Timer;
pub struct Mmu {
    pub rom: Vec<u8>,
    pub vram: [u8; 8192],
    pub wram: [u8; 8192],
    pub hram: [u8; 128],
    pub ppu: Ppu,
    pub timer: Timer,
    ie: u8,
    pub if_: u8,
    sb: u8,
    sc: u8,
}

impl Mmu {
    pub fn new(rom: Vec<u8>) -> Self {
        Mmu {
            rom,
            vram: [0; 8192],
            wram: [0; 8192],
            hram: [0; 128],
            ppu: Ppu::new(),
            timer: Timer::new(),
            sb: 0,
            ie: 0,
            if_: 0,
            sc: 0,
        }
    }
    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7FFF => self.rom[addr as usize],
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize],
            0xC000..=0xFDFF => {
                let idx = (addr - 0xC000) & 0x1FFF;
                self.wram[idx as usize]
            }
            0xFE00..=0xFE9F => self.ppu.oam[(addr - 0xFE00) as usize],
            0xFF00..=0xFF7F => self.read_io(addr),
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize],
            0xFFFF => self.ie,
            _ => 0xFF,
        }
    }

    pub fn write(&mut self, addr: u16, val: u8, pc: u16) {
        match addr {
            0x0000..=0x7FFF => {}
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize] = val,
            0xC000..=0xFDFF => {
                // if addr >= 0xDF00 {
                //     println!("WRAM write: [{:04X}] = {:02X}", addr, val);
                // }
                let idx = (addr - 0xC000) & 0x1FFF;
                self.wram[idx as usize] = val;
            }
            0xFE00..=0xFE9F => self.ppu.oam[(addr - 0xFE00) as usize] = val,
            0xFF00..=0xFF7F => self.write_io(addr, val, pc),
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize] = val,
            0xFFFF => self.ie = val,
            _ => {}
        }
    }
}

impl Mmu {
    pub fn read_io(&self, addr: u16) -> u8 {
        match addr {
            0xFF00 => 0xFF,
            0xFF01 => self.sb,
            0xFF02 => self.sc,
            0xFF04 => self.timer.div,
            0xFF05 => self.timer.tima,
            0xFF06 => self.timer.tma,
            0xFF07 => self.timer.tac,
            0xFF40 => self.ppu.lcdc,
            0xFF41 => self.ppu.mode,
            0xFF42 => self.ppu.scy,
            0xFF43 => self.ppu.scx,
            0xFF44 => self.ppu.ly,
            0xFF47 => self.ppu.bgp,
            0xFF4B => self.ppu.wx,
            0xFF4A => self.ppu.wy,
            0xFF0F => self.if_,
            _ => 0xFF,
        }
    }
    pub fn write_io(&mut self, addr: u16, val: u8, pc: u16) {
        //println!("IO write: {:04X} = {:02X}", addr, val);
        match addr {
            0xFF01 => self.sb = val,
            0xFF02 => {
                self.sc = val;

                if val == 0x81 {
                    print!("{}", self.sb as char);
                    use std::io::Write;
                    std::io::stdout().flush().unwrap();
                }
            }
            0xFF04 => self.timer.div = 0,
            0xFF05 => self.timer.tima = val,
            0xFF06 => self.timer.tma = val,
            0xFF07 => self.timer.tac = val,
            0xFF40 => self.ppu.lcdc = val,
            0xFF42 => self.ppu.scy = val,
            0xFF43 => self.ppu.scx = val,
            0xFF44 => self.ppu.ly = 1,
            0xFF4B => self.ppu.wx = val,
            0xFF4A => self.ppu.wy = val,
            0xFF0F => {
                self.if_ = val;
            }
            0xFF47 => {
                self.ppu.bgp = val;
            }
            _ => {}
        }
    }
}
