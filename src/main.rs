use std::error::Error;
use winit::event_loop::EventLoop;
mod rendering;
use rendering::App;

use crate::{cpu::Cpu, mmu::mmu::Mmu};
mod cpu;
mod mmu;

mod gb_loop;
mod joypad;
mod ppu;
mod timer;

fn main() -> Result<(), Box<dyn Error>> {
    let rom = std::fs::read("game.gb").unwrap();
    let mmu = Mmu::new(rom);
    let cpu = Cpu::new(mmu);
    let event_loop = EventLoop::new()?;

    let mut app = App {
        window: None,
        pixels: None,
        cpu,
    };
    event_loop.run_app(&mut app)?;
    Ok(())
}
