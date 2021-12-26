use std::time::{
    Duration,
    Instant
};
use glium::{
    Display,
    glutin::{
        ContextBuilder,
        event::{
            Event,
            WindowEvent
        },
        event_loop::{
            ControlFlow,
            EventLoop
        },
        window::WindowBuilder
    }
};
use crate::scene::{Scene, TestScene};

pub struct Almon {
    event_loop: EventLoop<()>,
    display: Display,
    current_scene: Box<dyn Scene>
}

impl Almon {
    pub fn new() -> Almon {
        let (event_loop, display) = Almon::init_window();
        let current_scene = Box::new(TestScene::new());

        Almon {
            event_loop,
            display,
            current_scene
        }
    }

    pub fn run(mut almon: Almon) {
        let fps = 100;
        let dt = Duration::from_millis(1000 / fps);
        let mut accumulator = Duration::from_millis(0);
        let mut frame_start = Instant::now();

        almon.event_loop.run(move |ev, _, control_flow| {
            *control_flow = ControlFlow::Poll;
            accumulator += frame_start.elapsed();
            frame_start = Instant::now();

            if accumulator > Duration::from_millis(200) {
                accumulator = Duration::from_millis(200);
            }

            while accumulator > dt {
                almon.current_scene.update(&dt);
                accumulator -= dt;
            }

            // TODO: render game

            match ev {
                Event::WindowEvent { event,.. } => {
                    match event {
                        WindowEvent::CloseRequested => {
                            *control_flow = ControlFlow::Exit;
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
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