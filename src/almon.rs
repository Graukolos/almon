use crate::renderer::Renderer2D;
use crate::scene::{MenuScene, Scene};
use crate::ui::{Config, Window};
use glium::glutin::event::{ElementState, Event, WindowEvent};
use glium::glutin::event_loop::ControlFlow;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

pub struct Almon {
    window: Window,
    current_scene: Box<dyn Scene>,
    config: Config,
}

impl Almon {
    pub fn new() -> Almon {
        let config = Config::load(String::from("config.yml"));
        let window = Window::new(config);
        let renderer = Rc::new(RefCell::new(Renderer2D::new(window.get_display())));
        let current_scene = Box::new(MenuScene::new(renderer));

        Almon {
            window,
            current_scene,
            config,
        }
    }

    pub fn run(mut self) {
        let tickrate = 128;
        let dt = Duration::from_millis(1000 / tickrate);
        let mut accumulator = Duration::from_millis(0);
        let mut frame_start = Instant::now();
        let event_loop = self.window.event_loop.take().unwrap();

        event_loop.run(move |ev, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match ev {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        self.config.save(String::from("config.yml"));
                        *control_flow = ControlFlow::Exit;
                    }
                    WindowEvent::Resized(physical_size) => {
                        self
                            .current_scene
                            .handle(crate::ui::Event::WindowResizedEvent(
                                physical_size.width as u16,
                                physical_size.height as u16,
                            ));
                    }
                    WindowEvent::KeyboardInput { input, .. } => {
                        self
                            .current_scene
                            .handle(crate::ui::Event::KeyPressedEvent(
                                input.scancode as u16,
                                input.state == ElementState::Pressed,
                            ))
                    }
                    _ => {}
                },
                Event::MainEventsCleared => {
                    accumulator += frame_start.elapsed();
                    frame_start = Instant::now();

                    if accumulator > Duration::from_millis(200) {
                        accumulator = Duration::from_millis(200);
                    }

                    while accumulator > dt {
                        if let Some(scene) = self.current_scene.update(&dt) {
                            self.current_scene = scene;
                        }
                        accumulator -= dt;
                    }

                    self.current_scene.render();
                }
                _ => {}
            }
        });
    }
}
