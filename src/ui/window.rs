use crate::ui::config::VideoMode;
use crate::ui::Config;
use glium::glutin::dpi::LogicalSize;
use glium::glutin::event_loop::EventLoop;
use glium::glutin::window::{Fullscreen, WindowBuilder};
use glium::glutin::ContextBuilder;
use glium::Display;
use std::rc::Rc;

pub struct Window {
    pub event_loop: Option<EventLoop<()>>,
    display: Rc<Display>,
}

impl Window {
    pub fn new(config: Config) -> Self {
        let event_loop = EventLoop::new();
        let (width, height) = config.resolution;
        let wb = match config.video_mode {
            VideoMode::Windowed => {
                WindowBuilder::new().with_inner_size(LogicalSize::new(width, height))
            }
            VideoMode::Fullscreen => WindowBuilder::new().with_fullscreen(Some(
                Fullscreen::Borderless(event_loop.available_monitors().next()),
            )),
        };
        let cb = ContextBuilder::new();
        let display = Display::new(wb, cb, &event_loop).unwrap();

        Self {
            event_loop: Some(event_loop),
            display: Rc::new(display),
        }
    }

    pub fn get_display(&self) -> Rc<Display> {
        self.display.clone()
    }
}
