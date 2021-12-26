use crate::{
    renderer::Renderer,
    scene::{Scene, TestScene},
};
use glium::{
    glutin::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
        ContextBuilder,
    },
    Display,
};
use std::{
    rc::Rc,
    time::{Duration, Instant},
};

pub struct Almon {
    event_loop: EventLoop<()>,
    renderer: Rc<Renderer>,
    current_scene: Box<dyn Scene>,
}

impl Almon {
    pub fn new() -> Almon {
        let (event_loop, renderer) = Almon::init_window();
        let current_scene = Box::new(TestScene::new(renderer.clone()));

        Almon {
            event_loop,
            renderer,
            current_scene
        }
    }

    pub fn run(mut almon: Almon) {
        let tickrate = 128;
        let dt = Duration::from_millis(1000 / tickrate);
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

            almon.current_scene.render();

            match ev {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => {}
                },
                _ => {}
            }
        });
    }

    fn init_window() -> (EventLoop<()>, Rc<Renderer>) {
        let event_loop = EventLoop::new();
        let wb = WindowBuilder::new();
        let cb = ContextBuilder::new();
        let display = Display::new(wb, cb, &event_loop).unwrap();
        let renderer = Rc::new(Renderer::new(display));

        (event_loop, renderer)
    }
}
