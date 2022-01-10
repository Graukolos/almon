use std::cell::RefCell;
use crate::renderer::{Renderer2D, SequentialRenderer};
use crate::scene::{MenuScene, Scene};
use glium::glutin::event::{Event, WindowEvent};
use glium::glutin::event_loop::ControlFlow;
use std::rc::Rc;
use std::time::{Duration, Instant};
use crate::window::Window;

pub struct Almon {
    window: Window,
    _renderer: Rc<RefCell<dyn Renderer2D>>,
    current_scene: Box<dyn Scene>
}

impl Almon {
    pub fn new() -> Almon {
        let window = Window::default();
        let _renderer = Rc::new(RefCell::new(SequentialRenderer::new(window.get_display())));
        let current_scene = Box::new(MenuScene::new(_renderer.clone()));

        Almon {
            window,
            _renderer,
            current_scene
        }
    }

    pub fn run(mut almon: Almon) {
        let tickrate = 128;
        let dt = Duration::from_millis(1000 / tickrate);
        let mut accumulator = Duration::from_millis(0);
        let mut frame_start = Instant::now();
        let event_loop = almon.window.event_loop.take().unwrap();

        event_loop.run(move |ev, _, control_flow| {
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
}
