use std::sync::Arc;
use glium::Display;
use glium::glutin::ContextBuilder;
use glium::glutin::dpi::LogicalSize;
use glium::glutin::event_loop::EventLoop;
use glium::glutin::window::WindowBuilder;

pub struct Window {
    width: u16,
    height: u16,
    pub event_loop: EventLoop<()>,
    display: Arc<Display>
}

impl Window {
    pub fn new(width: u16, height: u16) -> Window {
        let event_loop = EventLoop::new();
        let wb = WindowBuilder::new().with_inner_size(LogicalSize::new(width, height));
        let cb = ContextBuilder::new();
        let display = Display::new(wb, cb, &event_loop).unwrap();

        Window {
            width,
            height,
            event_loop,
            display: Arc::new(display)
        }
    }

    pub fn get_display(&self) -> Arc<Display> {
        self.display.clone()
    }
}

impl Default for Window {
    fn default() -> Self {
        Window::new(800, 600)
    }
}