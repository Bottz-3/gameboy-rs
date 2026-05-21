use crate::cpu::Cpu;
use pixels::{Pixels, SurfaceTexture};
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::{ElementState, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

const WIDTH: u32 = 160;
const HEIGHT: u32 = 144;

pub struct App {
    pub window: Option<Arc<Window>>,
    pub pixels: Option<Pixels<'static>>,
    pub cpu: Cpu,
}
fn shade_to_rgba(shade: u8) -> [u8; 4] {
    match shade {
        0 => [0xFF, 0xFF, 0xFF, 0xFF], // white
        1 => [0xAA, 0xAA, 0xAA, 0xFF], // light gray
        2 => [0x55, 0x55, 0x55, 0xFF], // dark gray
        3 => [0x00, 0x00, 0x00, 0xFF], // black
        _ => [0xFF, 0xFF, 0xFF, 0xFF],
    }
}
impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes().with_title("gameboy-rs"))
                .unwrap(),
        );

        let size = window.inner_size();
        let surface = SurfaceTexture::new(size.width, size.height, window.clone());

        let pixels = Pixels::new(WIDTH, HEIGHT, surface).unwrap();
        let scale_factor = window.scale_factor();

        self.window = Some(window);
        self.pixels = Some(pixels);

        println!("Scale factor: {}", scale_factor);
    }
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                let pressed = event.state == ElementState::Pressed;
                match event.physical_key {
                    PhysicalKey::Code(KeyCode::KeyZ) => self.cpu.mmu.joypad.set_action(0, pressed),
                    PhysicalKey::Code(KeyCode::KeyX) => self.cpu.mmu.joypad.set_action(1, pressed),
                    PhysicalKey::Code(KeyCode::Enter) => self.cpu.mmu.joypad.set_action(3, pressed),
                    PhysicalKey::Code(KeyCode::ShiftRight) => {
                        self.cpu.mmu.joypad.set_action(2, pressed)
                    }
                    PhysicalKey::Code(KeyCode::ArrowRight) => {
                        self.cpu.mmu.joypad.set_direction(0, pressed)
                    }
                    PhysicalKey::Code(KeyCode::ArrowLeft) => {
                        self.cpu.mmu.joypad.set_direction(1, pressed)
                    }
                    PhysicalKey::Code(KeyCode::ArrowUp) => {
                        self.cpu.mmu.joypad.set_direction(2, pressed)
                    }
                    PhysicalKey::Code(KeyCode::ArrowDown) => {
                        self.cpu.mmu.joypad.set_direction(3, pressed)
                    }
                    _ => {}
                }
            }
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                if let Some(pixels) = &mut self.pixels {
                    let frame = pixels.frame_mut();

                    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                        let x = i % 160;
                        let y = i / 160;
                        let shade = self.cpu.mmu.ppu.framebuffer[y][x];
                        pixel.copy_from_slice(&shade_to_rgba(shade));
                    }
                    pixels.render().unwrap();
                }
            }
            _ => {}
        }
    }
    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        let mut cycles = 0;
        while cycles < 70224 {
            self.cpu.gb_loop();
            cycles += 4;
        }
        self.window
            .as_ref()
            .expect("PANIC: Window should exist")
            .request_redraw();
    }
}
