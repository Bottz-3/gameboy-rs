use pixels::{Pixels, SurfaceTexture};
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow},
    window::Window,
};

const WIDTH: u32 = 160;
const HEIGHT: u32 = 144;

#[derive(Default)]
pub struct App {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
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
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                if let Some(pixels) = &mut self.pixels {
                    let frame = pixels.frame_mut();

                    for spot in frame.chunks_exact_mut(4) {
                        spot[0] = 0x20;
                        spot[1] = 0x40;
                        spot[2] = 0xFF;
                        spot[3] = 0xFF;
                    }
                    pixels.render().unwrap();
                }
            }
            _ => {}
        }
    }
    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        self.window
            .as_ref()
            .expect("PANIC: Window should exist")
            .request_redraw();
    }
}
