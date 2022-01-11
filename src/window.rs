use glium::glutin::dpi::LogicalSize;
use glium::glutin::event_loop::EventLoop;
use glium::glutin::window::WindowBuilder;
use glium::glutin::ContextBuilder;
use glium::Display;
use std::rc::Rc;

pub struct Window {
    _width: u16,
    _height: u16,
    pub event_loop: Option<EventLoop<()>>,
    display: Rc<Display>,
}

impl Window {
    pub fn new(_width: u16, _height: u16) -> Window {
        let event_loop = EventLoop::new();
        let wb = WindowBuilder::new().with_inner_size(LogicalSize::new(_width, _height));
        let cb = ContextBuilder::new();
        let display = Display::new(wb, cb, &event_loop).unwrap();

        Window {
            _width,
            _height,
            event_loop: Some(event_loop),
            display: Rc::new(display),
        }
    }

    pub fn get_display(&self) -> Rc<Display> {
        self.display.clone()
    }
}

impl Default for Window {
    fn default() -> Self {
        Window::new(800, 600)
    }
}
