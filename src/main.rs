use std::error::Error;
use winit::event_loop::EventLoop;
mod rendering;
use rendering::App;
mod cpu;

mod gb_loop;
mod mmu;
mod ppu;

fn main() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new()?;

    let mut app = App::default();
    event_loop.run_app(&mut app)?;
    Ok(())
}
