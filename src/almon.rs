use glium::{
    Display,
    glutin::{
        ContextBuilder,
        event_loop::EventLoop,
        window::WindowBuilder
    }
};
use glium::glutin::event_loop::ControlFlow;

pub struct Almon {
    event_loop: EventLoop<()>,
    display: Display
}

impl Almon {
    pub fn new() -> Almon {
        let (event_loop, display) = Almon::init_window();

        Almon {
            event_loop,
            display
        }
    }

    pub fn run(almon: Almon) {
        almon.event_loop.run(move |_, _, control_flow| {
            *control_flow = ControlFlow::Exit;
        });
    }

    fn init_window() -> (EventLoop<()>, Display) {
        let event_loop = EventLoop::new();
        let wb = WindowBuilder::new();
        let cb = ContextBuilder::new();
        let display = Display::new(wb, cb, &event_loop).unwrap();

        (event_loop, display)
    }
}