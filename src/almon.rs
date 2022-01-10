use std::cell::RefCell;
use crate::renderer::{Renderer2D, SequentialRenderer};
use crate::scene::{MenuScene, Scene, TestScene};
use glium::Display;
use glium::glutin::ContextBuilder;
use glium::glutin::event::{Event, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;
use std::rc::Rc;
use std::time::{Duration, Instant};

pub struct Almon {
    event_loop: EventLoop<()>,
    renderer: Rc<RefCell<dyn Renderer2D>>,
    current_scene: Box<dyn Scene>
}

impl Almon {
    pub fn new() -> Almon {
        let (event_loop, renderer) = Almon::init_window();
        let current_scene = Box::new(MenuScene::new(renderer.clone()));

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
                match almon.current_scene.update(&dt) {
                    None => {}
                    Some(scene) => {almon.current_scene = scene}
                }
                accumulator -= dt;
            }

            match ev {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => {}
                },
                Event::MainEventsCleared => {
                    almon.current_scene.render();
                }
                _ => {}
            }
        });
    }

    fn init_window() -> (EventLoop<()>, Rc<RefCell<dyn Renderer2D>>) {
        let event_loop = EventLoop::new();
        let wb = WindowBuilder::new();
        let cb = ContextBuilder::new();
        let display = Display::new(wb, cb, &event_loop).unwrap();
        let renderer = Rc::new(RefCell::new(SequentialRenderer::new(display)));

        (event_loop, renderer)
    }
}
